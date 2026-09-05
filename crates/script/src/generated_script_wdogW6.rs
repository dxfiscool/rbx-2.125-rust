// Auto-generated skeletons for rbx-script — wdogW6 (crate script)
// Filter: Script|Lua|LuaBridge|Yield|ProtectedString (case-sensitive) — 4921 filtered, 42 remaining not yet in crates/script/src, gap_filler EA-sorted asc distinct
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs EA-sorted asc | range 0x5b3b28..0xf60f74 | distinct not yet in crates/script/src (remaining 42 -> +78 gap filler global EA asc, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; boost stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x5b3b28 — __ZN3RBX16KeyframeSequence14removeKeyframeEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequence::removeKeyframe(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX16KeyframeSequence14removeKeyframeEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x5b3b28() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b3b7c — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEEC1Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEEC1Ev")]
pub fn stub_0x5b3b7c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::EnumDesc() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b3b80 — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::EnumDesc(void) [0x5b3b80]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEEC2Ev")]
pub fn stub_0x5b3b80(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::EnumDesc() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b4174 — __ZN3RBXL9CopyChildEN5boost10shared_ptrINS_8InstanceEEEPS2_
#[doc(alias = "RBX::CopyChild(boost::shared_ptr<RBX::Instance>,RBX::Instance*)")]
#[doc(alias = "__ZN3RBXL9CopyChildEN5boost10shared_ptrINS_8InstanceEEEPS2_")]
pub fn stub_0x5b4174() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b496c — __ZN3RBX16KeyframeSequence12onChildAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::onChildAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX16KeyframeSequence12onChildAddedEPNS_8InstanceE")]
pub fn stub_0x5b496c(handle: &crate::slot::InstanceHandle) {
// RBX::KeyframeSequence::onChildAdded(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b497c — __ZN3RBX16KeyframeSequence14onChildRemovedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::onChildRemoved(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX16KeyframeSequence14onChildRemovedEPNS_8InstanceE")]
pub fn stub_0x5b497c(handle: &crate::slot::InstanceHandle) {
// RBX::KeyframeSequence::onChildRemoved(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b4984 — __ZNK3RBX16KeyframeSequence15AppendPosePass0ERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequence::AppendPosePass0(boost::shared_ptr<RBX::Instance> const&)const")]
#[doc(alias = "__ZNK3RBX16KeyframeSequence15AppendPosePass0ERKN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x5b4984() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b4bf8 — __ZNK3RBX16KeyframeSequence15AppendPosePass1ERKN5boost10shared_ptrINS_8InstanceEEEPSt6vectorIPNS_10CachedPoseESaIS9_EE
#[doc(alias = "RBX::KeyframeSequence::AppendPosePass1(boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *)const")]
#[doc(alias = "__ZNK3RBX16KeyframeSequence15AppendPosePass1ERKN5boost10shared_ptrINS_8InstanceEEEPSt6vectorIPNS_10CachedPoseESaIS9_EE")]
pub fn stub_0x5b4bf8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b4ec8 — __ZN3RBX10CachedPose9setCFrameERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::CachedPose *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::CachedPose::setCFrame(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX10CachedPose9setCFrameERKN3G3D15CoordinateFrameE")]
pub fn stub_0x5b4ec8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::CachedPose setter.
cell.set(value)
}

// 0x5b50a4 — __ZNK3RBX16KeyframeSequence18cacheKeyframePass0ERKN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::KeyframeSequence::cacheKeyframePass0(boost::shared_ptr<RBX::Instance> const&)const")]
#[doc(alias = "__ZNK3RBX16KeyframeSequence18cacheKeyframePass0ERKN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x5b50a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b520c — __ZNK3RBX16KeyframeSequence18cacheKeyframePass1ERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequence::cacheKeyframePass1(boost::shared_ptr<RBX::Instance> const&)const")]
#[doc(alias = "__ZNK3RBX16KeyframeSequence18cacheKeyframePass1ERKN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x5b520c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b5350 — __ZN3RBX13lerpAxisAngleERKN3G3D7Vector3ES3_ff
// type: _DWORD __fastcall(RBX *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *, float, float)
#[doc(alias = "RBX::lerpAxisAngle(G3D::Vector3 const&,G3D::Vector3 const&,float,float)")]
#[doc(alias = "__ZN3RBX13lerpAxisAngleERKN3G3D7Vector3ES3_ff")]
pub fn stub_0x5b5350() -> crate::slot::PortedFn {
// IDA 0x5b5350: RBX::lerpAxisAngle(G3D::Vector3 const&, G3D::Vector3 const&, float, float).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5b5350, "RBX::lerpAxisAngle(G3D::Vector3 const&, G3D::Vector3 const&, float, float)")
}

// 0x5b560c — __ZNK3RBX16KeyframeSequence17verifySetAncestorEPKNS_8InstanceES3_
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, const RBX::Instance *, const RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::verifySetAncestor(RBX::Instance const*,RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX16KeyframeSequence17verifySetAncestorEPKNS_8InstanceES3_")]
pub fn stub_0x5b560c(handle: &crate::slot::InstanceHandle) {
// RBX::KeyframeSequence::verifySetAncestor(RBX::Instance const*, RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b56fc — __ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev")]
pub fn stub_0x5b56fc(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x5b5720 — __ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev")]
pub fn stub_0x5b5720(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x5b5834 — __ZN3RBX10Reflection14PropDescriptorINS_16KeyframeSequenceEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::KeyframeSequence,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_16KeyframeSequenceEbED1Ev")]
pub fn stub_0x5b5834(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5b5860 — __ZN3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::KeyframeSequence,RBX::KeyframeSequence::Priority>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEED1Ev")]
pub fn stub_0x5b5860(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5b5884 — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::addPair(RBX::KeyframeSequence::Priority,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE7addPairES3_PKc")]
pub fn stub_0x5b5884(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::addPair(RBX::KeyframeSequence:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b5c28 — __ZN3RBX9findOrAddISt4pairImmEEEmRSt6vectorIT_SaIS4_EERKS4_
// type: int(void)
#[doc(alias = "unsigned long RBX::findOrAdd<std::pair<unsigned long,unsigned long>>(std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>> &,std::pair<unsigned long,unsigned long> const&)")]
#[doc(alias = "__ZN3RBX9findOrAddISt4pairImmEEEmRSt6vectorIT_SaIS4_EERKS4_")]
pub fn stub_0x5b5c28() -> crate::slot::PortedFn {
// IDA 0x5b5c28: unsigned long RBX::findOrAdd<std::pair<unsigned long, unsigned long>>(std::vector<std::pair<unsigned long, unsigned long~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5b5c28, "unsigned long RBX::findOrAdd<std::pair<unsigned long, unsigned long>>(std::vector<std::pair<unsigned~")
}

// 0x5b5c7c — __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE9push_backERKS1_
// type: int(void)
#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::push_back(RBX::CachedPose const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE9push_backERKS1_")]
pub fn stub_0x5b5c7c() -> crate::slot::PortedFn {
// IDA 0x5b5c7c: std::vector<RBX::CachedPose, std::allocator<RBX::CachedPose>>::push_back(RBX::CachedPose const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5b5c7c, "std::vector<RBX::CachedPose, std::allocator<RBX::CachedPose>>::push_back(RBX::CachedPose const&)")
}

// 0x5b5cb8 — __ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::resize(unsigned long,RBX::CachedPose *)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE6resizeEmS2_")]
pub fn stub_0x5b5cb8(map: &crate::slot::TreeMapModel) -> usize {
// map size.
map.len()
}

// 0x5b5cec — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf2IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEPSt6vectorIPNS_10CachedPoseESaISE_EEEENS3_5list3INS3_5valueIPKS7_EENS2_3argILi1EEENSK_ISH_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>> const&)const")]
#[doc(alias = "__ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf2IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEPSt6vectorIPNS_10CachedPoseESaISE_EEEENS3_5list3INS3_5valueIPKS7_EENS2_3argILi1EEENSK_ISH_EEEEEEEEvRKT_")]
pub fn stub_0x5b5cec() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

// 0x5b5df4 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf1IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPKS7_EENS2_3argILi1EEEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>> const&)const")]
#[doc(alias = "__ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf1IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPKS7_EENS2_3argILi1EEEEEEEEEvRKT_")]
pub fn stub_0x5b5df4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

// 0x5b5ef0 — __ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::push_back(RBX::KeyframeSequence::CachedKeyframe const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE9push_backERKS2_")]
pub fn stub_0x5b5ef0() -> crate::slot::PortedFn {
// IDA 0x5b5ef0: std::vector<RBX::KeyframeSequence::CachedKeyframe, std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::push_back(RBX~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5b5ef0, "std::vector<RBX::KeyframeSequence::CachedKeyframe, std::allocator<RBX::KeyframeSequence::CachedKeyfr~")
}

// 0x5b5f40 — __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE7reserveEm
// type: int(void)
#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE7reserveEm")]
pub fn stub_0x5b5f40(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

// 0x5b5fcc — __ZN3RBX16KeyframeSequenceD1Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::~KeyframeSequence()")]
#[doc(alias = "__ZN3RBX16KeyframeSequenceD1Ev")]
pub fn stub_0x5b5fcc(handle: crate::slot::InstanceHandle) {
// RBX::KeyframeSequence dtor.
drop(handle);
}

// 0x5b6104 — __ZN3RBX16KeyframeSequenceD0Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::~KeyframeSequence() [0x5b6104]")]
#[doc(alias = "__ZN3RBX16KeyframeSequenceD0Ev")]
pub fn stub_0x5b6104(handle: crate::slot::InstanceHandle) {
// RBX::KeyframeSequence dtor.
drop(handle);
}

// 0x5b61a4 — __ZNK3RBX16KeyframeSequence11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX16KeyframeSequence11askAddChildEPKNS_8InstanceE")]
pub fn stub_0x5b61a4(handle: &crate::slot::InstanceHandle) {
// RBX::KeyframeSequence::askAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b61e0 — __ZNK3RBX16KeyframeSequence12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX16KeyframeSequence12askSetParentEPKNS_8InstanceE")]
pub fn stub_0x5b61e0(handle: &crate::slot::InstanceHandle) {
// RBX::KeyframeSequence::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b61f4 — __ZThn32_N3RBX16KeyframeSequenceD1Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::KeyframeSequence::~KeyframeSequence()")]
#[doc(alias = "__ZThn32_N3RBX16KeyframeSequenceD1Ev")]
pub fn stub_0x5b61f4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5b61fc — __ZThn32_N3RBX16KeyframeSequenceD0Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::KeyframeSequence::~KeyframeSequence() [0x5b61fc]")]
#[doc(alias = "__ZThn32_N3RBX16KeyframeSequenceD0Ev")]
pub fn stub_0x5b61fc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5b62b0 — __ZThn36_N3RBX16KeyframeSequenceD1Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::KeyframeSequence::~KeyframeSequence() [0x5b62b0]")]
#[doc(alias = "__ZThn36_N3RBX16KeyframeSequenceD1Ev")]
pub fn stub_0x5b62b0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5b62b8 — __ZThn36_N3RBX16KeyframeSequenceD0Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::KeyframeSequence::~KeyframeSequence() [0x5b62b8]")]
#[doc(alias = "__ZThn36_N3RBX16KeyframeSequenceD0Ev")]
pub fn stub_0x5b62b8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5b6964 — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiEvT_SA_T0_
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int)")]
#[doc(alias = "__ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiEvT_SA_T0_")]
pub fn stub_0x5b6964() -> crate::slot::PortedFn {
// IDA 0x5b6964: void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::vector<RBX::Keyfram~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5b6964, "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std:~")
}

// 0x5b6aec — __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
#[doc(alias = "__ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")]
pub fn stub_0x5b6aec(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x5b6bf4 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, void *, int, int, int, int, int, int)
#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
#[doc(alias = "__ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")]
pub fn stub_0x5b6bf4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x5b6d58 — __ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEES4_EvT_T0_
// type: int(void)
#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe)")]
#[doc(alias = "__ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEES4_EvT_T0_")]
pub fn stub_0x5b6d58(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x5b6d9c — __ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEC2ERKS4_
// type: int(void)
#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::vector(std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEC2ERKS4_")]
pub fn stub_0x5b6d9c() -> crate::slot::PortedFn {
// IDA 0x5b6d9c: std::vector<RBX::CachedPose*, std::allocator<RBX::CachedPose*>>::vector(std::vector<RBX::CachedPose*, std::allocator<RBX~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5b6d9c, "std::vector<RBX::CachedPose*, std::allocator<RBX::CachedPose*>>::vector(std::vector<RBX::CachedPose*~")
}

// 0x5b6dd4 — __ZNSt12_Vector_baseIPN3RBX10CachedPoseESaIS2_EEC2EmRKS3_
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::_Vector_base(unsigned long,std::allocator<RBX::CachedPose *> const&)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX10CachedPoseESaIS2_EEC2EmRKS3_")]
pub fn stub_0x5b6dd4() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x5b6e04 — __ZNSt12_Vector_baseIPN3RBX10CachedPoseESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX10CachedPoseESaIS2_EE11_M_allocateEm")]
pub fn stub_0x5b6e04() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x5b6e1c — __ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEaSERKS4_
// type: int(void)
#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::operator=(std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEaSERKS4_")]
pub fn stub_0x5b6e1c() -> crate::slot::PortedFn {
// IDA 0x5b6e1c: std::vector<RBX::CachedPose*, std::allocator<RBX::CachedPose*>>::operator=(std::vector<RBX::CachedPose*, std::allocator<~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5b6e1c, "std::vector<RBX::CachedPose*, std::allocator<RBX::CachedPose*>>::operator=(std::vector<RBX::CachedPo~")
}

// 0x5b6eb4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16KeyframeSequence14CachedKeyframeES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::KeyframeSequence::CachedKeyframe * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::KeyframeSequence::CachedKeyframe *,RBX::KeyframeSequence::CachedKeyframe *>(RBX::KeyframeSequence::CachedKeyframe *,RBX::KeyframeSequence::CachedKeyframe *,RBX::KeyframeSequence::CachedKeyframe *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16KeyframeSequence14CachedKeyframeES6_EET0_T_S8_S7_")]
pub fn stub_0x5b6eb4(handle: &crate::slot::InstanceHandle) {
// RBX::KeyframeSequence::CachedKeyframe* std::__copy_backward<false, std::random_access_iter~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b6f10 — __ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_T0_
// type: float *__fastcall(float *, float *, float *)
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>> std::__unguarded_partition<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe)")]
#[doc(alias = "__ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_T0_")]
pub fn stub_0x5b6f10() -> crate::slot::PortedFn {
// IDA 0x5b6f10: __gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::vector<RBX::KeyframeSequence::CachedKeyframe, ~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x5b6f10, "__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::vector<RBX::KeyframeSequen~")
}

// 0x5b7040 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_SA_
// type: int __fastcall(int, int, void *, int, int, int, int, void *, int, int, int, void *, int, int, int, int, int, int)
#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
#[doc(alias = "__ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_SA_")]
pub fn stub_0x5b7040() -> crate::slot::PortedFn {
// IDA 0x5b7040: void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::vector<RBX::KeyframeSe~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5b7040, "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::ve~")
}

// 0x5b718c — __ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
// type: int(void)
#[doc(alias = "void std::sort_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
#[doc(alias = "__ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")]
pub fn stub_0x5b718c() -> crate::slot::PortedFn {
// IDA 0x5b718c: void std::sort_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::vector<RBX::KeyframeSequen~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5b718c, "void std::sort_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::vector~")
}

// 0x5b71b4 — __ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
#[doc(alias = "__ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")]
pub fn stub_0x5b71b4() -> crate::slot::PortedFn {
// IDA 0x5b71b4: void std::pop_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::vector<RBX::KeyframeSequenc~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5b71b4, "void std::pop_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::vector<~")
}

// 0x5b72c8 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int,RBX::KeyframeSequence::CachedKeyframe>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int,int,RBX::KeyframeSequence::CachedKeyframe)")]
#[doc(alias = "__ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_")]
pub fn stub_0x5b72c8() -> crate::slot::PortedFn {
// IDA 0x5b72c8: void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::vector<RBX::KeyframeSe~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5b72c8, "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::ve~")
}

// 0x5b7424 — __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
// type: int(void)
#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int,RBX::KeyframeSequence::CachedKeyframe>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,int,int,RBX::KeyframeSequence::CachedKeyframe)")]
#[doc(alias = "__ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_")]
pub fn stub_0x5b7424(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x5b748c — __ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "void std::make_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>>(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe *,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>)")]
#[doc(alias = "__ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX16KeyframeSequence14CachedKeyframeESt6vectorIS4_SaIS4_EEEEEvT_SA_")]
pub fn stub_0x5b748c() -> crate::slot::PortedFn {
// IDA 0x5b748c: void std::make_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::vector<RBX::KeyframeSequen~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5b748c, "void std::make_heap<__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*, std::vector~")
}

// 0x5b7588 — __ZNSt12_Vector_baseIN3RBX10CachedPoseESaIS1_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CachedPose,std::allocator<RBX::CachedPose>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX10CachedPoseESaIS1_EE11_M_allocateEm")]
pub fn stub_0x5b7588() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x5b75a0 — __ZN5boost3_bi6bind_tIvNS_4_mfi4cmf1IvN3RBX16KeyframeSequenceERKNS_10shared_ptrINS4_8InstanceEEEEENS0_5list2INS0_5valueIPKS5_EENS_3argILi1EEEEEEclIS8_EEvRKT_
// type: int(void)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>::operator()<boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi4cmf1IvN3RBX16KeyframeSequenceERKNS_10shared_ptrINS4_8InstanceEEEEENS0_5list2INS0_5valueIPKS5_EENS_3argILi1EEEEEEclIS8_EEvRKT_")]
pub fn stub_0x5b75a0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

// 0x5b75b8 — __ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE15_M_erase_at_endEPS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::_M_erase_at_end(RBX::KeyframeSequence::CachedKeyframe*)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE15_M_erase_at_endEPS2_")]
pub fn stub_0x5b75b8(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x5b75e8 — __ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::CachedKeyframe*,std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>>,RBX::KeyframeSequence::CachedKeyframe const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x5b75e8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x5b797c — __ZNSt12_Vector_baseIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE11_M_allocateEm")]
pub fn stub_0x5b797c() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x5b7994 — __ZN5boost3_bi5list3INS0_5valueIPKN3RBX16KeyframeSequenceEEENS_3argILi1EEENS2_IPSt6vectorIPNS3_10CachedPoseESaISC_EEEEEclINS_4_mfi4cmf2IvS4_RKNS_10shared_ptrINS3_8InstanceEEESF_EENS0_5list1ISP_EEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>::operator()<boost::_mfi::cmf2<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::cmf2<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPKN3RBX16KeyframeSequenceEEENS_3argILi1EEENS2_IPSt6vectorIPNS3_10CachedPoseESaISC_EEEEEclINS_4_mfi4cmf2IvS4_RKNS_10shared_ptrINS3_8InstanceEEESF_EENS0_5list1ISP_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x5b7994(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x5b7994: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

// 0x5b79c0 — __ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CachedPose **,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>>,unsigned long,RBX::CachedPose * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x5b79c0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x5b7b28 — __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int(void)
#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CachedPose*,std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>>,RBX::CachedPose const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
pub fn stub_0x5b7b28(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x5b7cc4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10CachedPoseES5_EET0_T_S7_S6_
// type: int(void)
#[doc(alias = "RBX::CachedPose * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CachedPose *,RBX::CachedPose *>(RBX::CachedPose *,RBX::CachedPose *,RBX::CachedPose *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10CachedPoseES5_EET0_T_S7_S6_")]
pub fn stub_0x5b7cc4(handle: &crate::slot::InstanceHandle) {
// RBX::CachedPose* std::__copy_backward<false, std::random_access_iterator_tag>::__copy_b<RB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b7d40 — __ZNSt6vectorISt4pairImmESaIS1_EE9push_backERKS1_
// type: int(void)
#[doc(alias = "std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>>::push_back(std::pair<unsigned long,unsigned long> const&)")]
#[doc(alias = "__ZNSt6vectorISt4pairImmESaIS1_EE9push_backERKS1_")]
pub fn stub_0x5b7d40(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

// 0x5b7d70 — __ZNSt6vectorISt4pairImmESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int(void)
#[doc(alias = "std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<unsigned long,unsigned long>*,std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>>>,std::pair<unsigned long,unsigned long> const&)")]
#[doc(alias = "__ZNSt6vectorISt4pairImmESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
pub fn stub_0x5b7d70(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

// 0x5b7e68 — __ZNSt12_Vector_baseISt4pairImmESaIS1_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseISt4pairImmESaIS1_EE11_M_allocateEm")]
pub fn stub_0x5b7e68() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x5b7e80 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairImmES5_EET0_T_S7_S6_
// type: int(void)
#[doc(alias = "std::pair<unsigned long,unsigned long> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<unsigned long,unsigned long> *,std::pair<unsigned long,unsigned long> *>(std::pair<unsigned long,unsigned long> *,std::pair<unsigned long,unsigned long> *,std::pair<unsigned long,unsigned long> *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairImmES5_EET0_T_S7_S6_")]
pub fn stub_0x5b7e80(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

// 0x5b7ec4 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPN3RBX16KeyframeSequenceEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEEPSC_ENS0_5list1IRKSD_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::KeyframeSequence *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::Instance*),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Instance*) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPN3RBX16KeyframeSequenceEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEEPSC_ENS0_5list1IRKSD_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x5b7ec4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x5b7ec4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

// 0x5b8194 — __ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::resize(unsigned long,RBX::KeyframeSequence::Priority)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE6resizeEmS2_")]
pub fn stub_0x5b8194(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

// 0x5b81c8 — __ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::push_back(RBX::KeyframeSequence::Priority const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE9push_backERKS2_")]
pub fn stub_0x5b81c8(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

// 0x5b81f0 — __ZNSt3mapIPKN3RBX4NameENS0_16KeyframeSequence8PriorityESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::KeyframeSequence::Priority,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_16KeyframeSequence8PriorityESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0x5b81f0(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

// 0x5b8248 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0x5b8248(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x5b82fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x5b82fc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x5b8354 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0x5b8354(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x5b83bc — __ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::Priority*,std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>>,RBX::KeyframeSequence::Priority const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x5b83bc(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

// 0x5b84a0 — __ZNSt12_Vector_baseIN3RBX16KeyframeSequence8PriorityESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX16KeyframeSequence8PriorityESaIS2_EE11_M_allocateEm")]
pub fn stub_0x5b84a0() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x5b84b8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16KeyframeSequence8PriorityES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::KeyframeSequence::Priority * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::KeyframeSequence::Priority *,RBX::KeyframeSequence::Priority *>(RBX::KeyframeSequence::Priority *,RBX::KeyframeSequence::Priority *,RBX::KeyframeSequence::Priority *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16KeyframeSequence8PriorityES6_EET0_T_S8_S7_")]
pub fn stub_0x5b84b8(handle: &crate::slot::InstanceHandle) {
// RBX::KeyframeSequence::Priority* std::__copy_backward<false, std::random_access_iterator_t~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b84f4 — __ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::KeyframeSequence::Priority*,std::vector<RBX::KeyframeSequence::Priority,std::allocator<RBX::KeyframeSequence::Priority>>>,unsigned long,RBX::KeyframeSequence::Priority const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x5b84f4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

// 0x5b8684 — __ZN3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::KeyframeSequence,RBX::KeyframeSequence::Priority>::EnumPropDescriptor<RBX::KeyframeSequence::Priority (RBX::KeyframeSequence::*)(void)const,void (RBX::KeyframeSequence::*)(RBX::KeyframeSequence::Priority)>(char const*,char const*,RBX::KeyframeSequence::Priority (RBX::KeyframeSequence::*)(void)const,void (RBX::KeyframeSequence::*)(RBX::KeyframeSequence::Priority),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x5b8684(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5b8838 — __ZN3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::KeyframeSequence,RBX::KeyframeSequence::Priority>::~EnumPropDescriptor() [0x5b8838]")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEED0Ev")]
pub fn stub_0x5b8838(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5b8864 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::KeyframeSequence,RBX::KeyframeSequence::Priority>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEE10isReadOnlyEv")]
pub fn stub_0x5b8864(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5b8874 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::KeyframeSequence,RBX::KeyframeSequence::Priority>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEE11isWriteOnlyEv")]
pub fn stub_0x5b8874(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5b8884 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::KeyframeSequence,RBX::KeyframeSequence::Priority>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_0x5b8884(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x603d58 — __ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EEC2ERKNS_6tuples4consINSI_5tupleImNS0_8identityISA_EENS_4hashIS7_EESt8equal_toIS7_ENSI_9null_typeESR_SR_SR_SR_SR_EESR_EERKSG_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::multi_index_container(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type> const&,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&)")]
#[doc(alias = "__ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EEC2ERKNS_6tuples4consINSI_5tupleImNS0_8identityISA_EENS_4hashIS7_EESt8equal_toIS7_ENSI_9null_typeESR_SR_SR_SR_SR_EESR_EERKSG_")]
pub fn stub_0x603d58() -> crate::slot::PortedFn {
// IDA 0x603d58: boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x603d58, "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweig~")
}

// 0x818d7c — __ZN3RBX14LibraryService17contentReadyLocalERKSsN5boost10flyweights9flyweightINS_15ProtectedStringENS3_9parameter5void_ES8_S8_S8_S8_EE
// type: int __fastcall(int, void *, int)
#[doc(alias = "RBX::LibraryService::contentReadyLocal(std::string const&,boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>)")]
#[doc(alias = "__ZN3RBX14LibraryService17contentReadyLocalERKSsN5boost10flyweights9flyweightINS_15ProtectedStringENS3_9parameter5void_ES8_S8_S8_S8_EE")]
pub fn stub_0x818d7c(handle: &crate::slot::InstanceHandle) {
// RBX::LibraryService::contentReadyLocal(std::string const&, boost::flyweights::flyweight<RB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x81f510 — __ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE12insert_valueERKS5_
#[doc(alias = "boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::insert_value(RBX::ProtectedString const&)")]
#[doc(alias = "__ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE12insert_valueERKS5_")]
pub fn stub_0x81f510(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x81f7a0 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE5eraseENS1_21hashed_index_iteratorINS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEENS1_12bucket_arrayISO_EEEE
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::erase(boost::multi_index::detail::hashed_index_iterator<boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::index_node_base<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>>,boost::multi_index::detail::bucket_array<std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>>)")]
#[doc(alias = "__ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE5eraseENS1_21hashed_index_iteratorINS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEENS1_12bucket_arrayISO_EEEE")]
pub fn stub_0x81f7a0(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x81f808 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE6erase_EPNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::erase_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::index_node_base<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>> *)")]
#[doc(alias = "__ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE6erase_EPNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE")]
pub fn stub_0x81f808(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x81f848 — __ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EE7insert_ERKSA_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::insert_(boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const&)")]
#[doc(alias = "__ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EE7insert_ERKSA_")]
pub fn stub_0x81f848(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x81f948 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7insert_ERKSC_PNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::insert_(boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const&,boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::index_node_base<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>> *)")]
#[doc(alias = "__ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7insert_ERKSC_PNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE")]
pub fn stub_0x81f948(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x81f9c8 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7reserveEm
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::reserve(unsigned long)")]
#[doc(alias = "__ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7reserveEm")]
pub fn stub_0x81f9c8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

// 0x81fa10 — __ZNK5boost4hashIN3RBX15ProtectedStringEEclERKS2_
// type: int __fastcall(int, RBX::ProtectedString *this)
#[doc(alias = "boost::hash<RBX::ProtectedString>::operator()(RBX::ProtectedString const&)const")]
#[doc(alias = "__ZNK5boost4hashIN3RBX15ProtectedStringEEclERKS2_")]
pub fn stub_0x81fa10() -> crate::slot::PortedFn {
// IDA 0x81fa10: boost::hash<RBX::ProtectedString>::operator()(RBX::ProtectedString const&) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x81fa10, "boost::hash<RBX::ProtectedString>::operator()(RBX::ProtectedString const&) const")
}

// 0x81fa48 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE16unchecked_rehashEm
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::unchecked_rehash(unsigned long)")]
#[doc(alias = "__ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE16unchecked_rehashEm")]
pub fn stub_0x81fa48(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

// 0x822500 — __ZN5boost10flyweights6detail30flyweight_core_tracking_helperINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE5eraseIPFbRKNS1_17refcounted_handleIPKNS1_16refcounted_valueINS6_8rep_typeES5_EESE_EEEEEvSO_T_
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "void boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::erase<bool (*)(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&)>(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&,bool (*)(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&))")]
#[doc(alias = "__ZN5boost10flyweights6detail30flyweight_core_tracking_helperINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE5eraseIPFbRKNS1_17refcounted_handleIPKNS1_16refcounted_valueINS6_8rep_typeES5_EESE_EEEEEvSO_T_")]
pub fn stub_0x822500(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x95c014 — __ZN3RBX7Network16ServerReplicator24isProtectedStringEnabledEv
// type: _DWORD __fastcall(RBX::Network::ServerReplicator *__hidden this)
#[doc(alias = "RBX::Network::ServerReplicator::isProtectedStringEnabled(void)")]
#[doc(alias = "__ZN3RBX7Network16ServerReplicator24isProtectedStringEnabledEv")]
pub fn stub_0x95c014(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Network::ServerReplicator getter.
cell.get()
}

// 0x9813ec — __ZN3RBX7Network16ClientReplicator24isProtectedStringEnabledEv
// type: int __fastcall(RBX::Network::ClientReplicator *this)
#[doc(alias = "RBX::Network::ClientReplicator::isProtectedStringEnabled(void)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator24isProtectedStringEnabledEv")]
pub fn stub_0x9813ec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Network::ClientReplicator getter.
cell.get()
}

// 0xa222e0 — __ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE4initEv
// type: int()
#[doc(alias = "boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::init(void)")]
#[doc(alias = "__ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE4initEv")]
pub fn stub_0xa222e0(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "__ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE16unchecked_rehashEm$shim")]
pub fn stub_0xf24b68() -> crate::slot::PortedFn {
// IDA 0xf24b68: boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost:~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf24b68, "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::ref~")
}

// 0xf299d4 — j___ZN5boost11multi_index6detail10auto_spaceImSaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES8_EEEEC2ERKSC_m
#[doc(alias = "boost::multi_index::detail::auto_space<unsigned long,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::auto_space(std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&,unsigned long) [0xf299d4]")]
#[doc(alias = "j___ZN5boost11multi_index6detail10auto_spaceImSaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES8_EEEEC2ERKSC_m")]
pub fn stub_0xf299d4() -> crate::slot::PortedFn {
// IDA 0xf299d4: j___ZN5boost11multi_index6detail10auto_spaceImSaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3R~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf299d4, "j___ZN5boost11multi_index6detail10auto_spaceImSaINS_10flyweights6detail16refcounted_valueINS4_20defa~")
}

// 0xf299e4 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE10link_pointERKSC_RPNS1_22hashed_index_node_implISaIcEEEST_
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::link_point(boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const&,boost::multi_index::detail::hashed_index_node_impl<std::allocator<char>> *&,boost::multi_index::detail::hashed_unique_tag) [0xf299e4]")]
#[doc(alias = "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE10link_pointERKSC_RPNS1_22hashed_index_node_implISaIcEEEST_")]
pub fn stub_0xf299e4() -> crate::slot::PortedFn {
// IDA 0xf299e4: j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_valu~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf299e4, "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valu~")
}

// 0xf29dd4 — j___ZN3rbx8any_castIRKN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::ProtectedString const& rbx::any_cast<RBX::ProtectedString const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0xf29dd4]")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf29dd4() -> crate::slot::PortedFn {
// IDA 0xf29dd4: j___ZN3rbx8any_castIRKN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf29dd4, "j___ZN3rbx8any_castIRKN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

// 0xf2a8c4 — j___ZN3RBX10Reflection23TypedPropertyDescriptorINS_15ProtectedStringEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions) [0xf2a8c4]")]
#[doc(alias = "j___ZN3RBX10Reflection23TypedPropertyDescriptorINS_15ProtectedStringEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf2a8c4() -> crate::slot::PortedFn {
// IDA 0xf2a8c4: j___ZN3RBX10Reflection23TypedPropertyDescriptorINS_15ProtectedStringEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6G~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2a8c4, "j___ZN3RBX10Reflection23TypedPropertyDescriptorINS_15ProtectedStringEEC2ERNS0_15ClassDescriptorEPKcS~")
}

// 0xf2aa04 — j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EC2Ev
#[doc(alias = "boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::flyweight(void) [0xf2aa04]")]
#[doc(alias = "j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EC2Ev")]
pub fn stub_0xf2aa04() -> crate::slot::PortedFn {
// IDA 0xf2aa04: j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EC2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2aa04, "j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EC2Ev")
}

// 0xf2aa14 — j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS3_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int)
#[doc(alias = "boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::operator=(RBX::ProtectedString const&) [0xf2aa14]")]
#[doc(alias = "j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS3_")]
pub fn stub_0xf2aa14() -> crate::slot::PortedFn {
// IDA 0xf2aa14: j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS3_.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2aa14, "j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS3_")
}

// 0xf2aa24 — j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS6_
#[doc(alias = "boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::operator=(boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_> const&) [0xf2aa24]")]
#[doc(alias = "j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS6_")]
pub fn stub_0xf2aa24() -> crate::slot::PortedFn {
// IDA 0xf2aa24: j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS6_.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2aa24, "j___ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS6_")
}

// 0xf2f644 — j___ZN3RBX10Reflection4TypeC2INS_15ProtectedStringEEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<RBX::ProtectedString>(char const*,RBX::ProtectedString *) [0xf2f644]")]
#[doc(alias = "j___ZN3RBX10Reflection4TypeC2INS_15ProtectedStringEEEPKcPT_")]
pub fn stub_0xf2f644() -> crate::slot::PortedFn {
// IDA 0xf2f644: j___ZN3RBX10Reflection4TypeC2INS_15ProtectedStringEEEPKcPT_.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2f644, "j___ZN3RBX10Reflection4TypeC2INS_15ProtectedStringEEEPKcPT_")
}

// 0xf2f654 — j___ZN3RBX10Reflection7Variant14genericConvertINS_15ProtectedStringEEERT_v
#[doc(alias = "RBX::ProtectedString & RBX::Reflection::Variant::genericConvert<RBX::ProtectedString>(void) [0xf2f654]")]
#[doc(alias = "j___ZN3RBX10Reflection7Variant14genericConvertINS_15ProtectedStringEEERT_v")]
pub fn stub_0xf2f654() -> crate::slot::PortedFn {
// IDA 0xf2f654: j___ZN3RBX10Reflection7Variant14genericConvertINS_15ProtectedStringEEERT_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2f654, "j___ZN3RBX10Reflection7Variant14genericConvertINS_15ProtectedStringEEERT_v")
}

// 0xf2f664 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15ProtectedStringEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ProtectedString>(RBX::ProtectedString const&) [0xf2f664]")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15ProtectedStringEEERS3_RKT_")]
pub fn stub_0xf2f664() -> crate::slot::PortedFn {
// IDA 0xf2f664: j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15ProtectedStringEEERS3_RKT_.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2f664, "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15ProtectedStringEEERS3_RKT_")
}

// 0xf2f674 — j___ZN3rbx14implementation12typed_holderIN3RBX15ProtectedStringEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::ProtectedString>::singleton(void) [0xf2f674]")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX15ProtectedStringEE9singletonEv")]
pub fn stub_0xf2f674() -> crate::slot::PortedFn {
// IDA 0xf2f674: j___ZN3rbx14implementation12typed_holderIN3RBX15ProtectedStringEE9singletonEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2f674, "j___ZN3rbx14implementation12typed_holderIN3RBX15ProtectedStringEE9singletonEv")
}

// 0xf2f684 — j___ZN3rbx8any_castIN3RBX15ProtectedStringENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::ProtectedString * rbx::any_cast<RBX::ProtectedString,RBX::Region3>(rbx::placement_any<RBX::Region3> *) [0xf2f684]")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX15ProtectedStringENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_0xf2f684() -> crate::slot::PortedFn {
// IDA 0xf2f684: j___ZN3rbx8any_castIN3RBX15ProtectedStringENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2f684, "j___ZN3rbx8any_castIN3RBX15ProtectedStringENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

// 0xf2f694 — j___ZN3rbx8any_castIRN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::ProtectedString & rbx::any_cast<RBX::ProtectedString &,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0xf2f694]")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf2f694() -> crate::slot::PortedFn {
// IDA 0xf2f694: j___ZN3rbx8any_castIRN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2f694, "j___ZN3rbx8any_castIRN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

// 0xf33264 — j___ZN5boost10flyweights19static_holder_classINS0_6detail14flyweight_coreINS2_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS9_S9_S9_Li0EEENS0_14simple_lockingENS0_13static_holderEE10holder_argEE3getEv
#[doc(alias = "boost::flyweights::static_holder_class<boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::holder_arg>::get(void) [0xf33264]")]
#[doc(alias = "j___ZN5boost10flyweights19static_holder_classINS0_6detail14flyweight_coreINS2_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS9_S9_S9_Li0EEENS0_14simple_lockingENS0_13static_holderEE10holder_argEE3getEv")]
pub fn stub_0xf33264() -> crate::slot::PortedFn {
// IDA 0xf33264: j___ZN5boost10flyweights19static_holder_classINS0_6detail14flyweight_coreINS2_20default_value_policyIN3RBX15ProtectedStr~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf33264, "j___ZN5boost10flyweights19static_holder_classINS0_6detail14flyweight_coreINS2_20default_value_policy~")
}

// 0xf332a4 — j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EED2Ev
#[doc(alias = "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::~multi_index_container() [0xf332a4]")]
#[doc(alias = "j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EED2Ev")]
pub fn stub_0xf332a4() -> crate::slot::PortedFn {
// IDA 0xf332a4: j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3R~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf332a4, "j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20defa~")
}

// 0xf332b4 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE17delete_all_nodes_Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::delete_all_nodes_(void) [0xf332b4]")]
#[doc(alias = "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE17delete_all_nodes_Ev")]
pub fn stub_0xf332b4() -> crate::slot::PortedFn {
// IDA 0xf332b4: j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_valu~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf332b4, "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valu~")
}

// 0xf46124 — j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EEC2ERKNS_6tuples4consINSI_5tupleImNS0_8identityISA_EENS_4hashIS7_EESt8equal_toIS7_ENSI_9null_typeESR_SR_SR_SR_SR_EESR_EERKSG_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::multi_index_container(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type> const&,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&) [0xf46124]")]
#[doc(alias = "j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EEC2ERKNS_6tuples4consINSI_5tupleImNS0_8identityISA_EENS_4hashIS7_EESt8equal_toIS7_ENSI_9null_typeESR_SR_SR_SR_SR_EESR_EERKSG_")]
pub fn stub_0xf46124() -> crate::slot::PortedFn {
// IDA 0xf46124: j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3R~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf46124, "j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20defa~")
}

// 0xf55504 — j___ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE12insert_valueERKS5_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::insert_value(RBX::ProtectedString const&) [0xf55504]")]
#[doc(alias = "j___ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE12insert_valueERKS5_")]
pub fn stub_0xf55504() -> crate::slot::PortedFn {
// IDA 0xf55504: j___ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refc~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55504, "j___ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEE~")
}

// 0xf55514 — j___ZN5boost10flyweights6detail30flyweight_core_tracking_helperINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE5eraseIPFbRKNS1_17refcounted_handleIPKNS1_16refcounted_valueINS6_8rep_typeES5_EESE_EEEEEvSO_T_
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "void boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::erase<bool (*)(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&)>(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&,bool (*)(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&)) [0xf55514]")]
#[doc(alias = "j___ZN5boost10flyweights6detail30flyweight_core_tracking_helperINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE5eraseIPFbRKNS1_17refcounted_handleIPKNS1_16refcounted_valueINS6_8rep_typeES5_EESE_EEEEEvSO_T_")]
pub fn stub_0xf55514() -> crate::slot::PortedFn {
// IDA 0xf55514: j___ZN5boost10flyweights6detail30flyweight_core_tracking_helperINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mp~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55514, "j___ZN5boost10flyweights6detail30flyweight_core_tracking_helperINS1_20default_value_policyIN3RBX15Pr~")
}

// 0xf55544 — j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EE7insert_ERKSA_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::insert_(boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const&) [0xf55544]")]
#[doc(alias = "j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EE7insert_ERKSA_")]
pub fn stub_0xf55544() -> crate::slot::PortedFn {
// IDA 0xf55544: j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3R~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55544, "j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20defa~")
}

// 0xf55554 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE16unchecked_rehashEm
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::unchecked_rehash(unsigned long) [0xf55554]")]
#[doc(alias = "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE16unchecked_rehashEm")]
pub fn stub_0xf55554() -> crate::slot::PortedFn {
// IDA 0xf55554: j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_valu~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55554, "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valu~")
}

// 0xf55564 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE5eraseENS1_21hashed_index_iteratorINS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEENS1_12bucket_arrayISO_EEEE
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::erase(boost::multi_index::detail::hashed_index_iterator<boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::index_node_base<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>>,boost::multi_index::detail::bucket_array<std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>>) [0xf55564]")]
#[doc(alias = "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE5eraseENS1_21hashed_index_iteratorINS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEENS1_12bucket_arrayISO_EEEE")]
pub fn stub_0xf55564() -> crate::slot::PortedFn {
// IDA 0xf55564: j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_valu~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55564, "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valu~")
}

// 0xf55574 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE6erase_EPNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::erase_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::index_node_base<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>> *) [0xf55574]")]
#[doc(alias = "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE6erase_EPNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE")]
pub fn stub_0xf55574() -> crate::slot::PortedFn {
// IDA 0xf55574: j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_valu~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55574, "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valu~")
}

// 0xf55584 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7insert_ERKSC_PNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::insert_(boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const&,boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::index_node_base<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>> *) [0xf55584]")]
#[doc(alias = "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7insert_ERKSC_PNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE")]
pub fn stub_0xf55584() -> crate::slot::PortedFn {
// IDA 0xf55584: j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_valu~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55584, "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valu~")
}

// 0xf55594 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7reserveEm
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::reserve(unsigned long) [0xf55594]")]
#[doc(alias = "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7reserveEm")]
pub fn stub_0xf55594() -> crate::slot::PortedFn {
// IDA 0xf55594: j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_valu~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55594, "j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valu~")
}

// 0xf55894 — j___ZNK5boost4hashIN3RBX15ProtectedStringEEclERKS2_
// type: int __fastcall(int, RBX::ProtectedString *this)
#[doc(alias = "boost::hash<RBX::ProtectedString>::operator()(RBX::ProtectedString const&)const [0xf55894]")]
#[doc(alias = "j___ZNK5boost4hashIN3RBX15ProtectedStringEEclERKS2_")]
pub fn stub_0xf55894() -> crate::slot::PortedFn {
// IDA 0xf55894: j___ZNK5boost4hashIN3RBX15ProtectedStringEEclERKS2_.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55894, "j___ZNK5boost4hashIN3RBX15ProtectedStringEEclERKS2_")
}

// 0xf60f74 — j___ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE4initEv
// type: int(void)
#[doc(alias = "boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::init(void) [0xf60f74]")]
#[doc(alias = "j___ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE4initEv")]
pub fn stub_0xf60f74() -> crate::slot::PortedFn {
// IDA 0xf60f74: j___ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refc~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf60f74, "j___ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEE~")
}
