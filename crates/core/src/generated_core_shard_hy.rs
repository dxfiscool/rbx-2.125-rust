//! core shard HY — 100 core stubs EA-sorted, continuation after HX 0x3164c8 (EA-sorted ascending, next 100 uncovered).
//! Source: ida/export.json EA-sorted ascending, next 100 after 0x3164c8 not yet in rbx_core.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "RBX::Extents::closestFace(G3D::Vector3 const&)")]
// 0x31e9f4 — __ZN3RBX7Extents11closestFaceERKN3G3D7Vector3E
pub fn stub_31e9f4() {
    // IDA 0x31e9f4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Extents::getFaceCorners(RBX::NormalId,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)const")]
// 0x31ebfc — __ZNK3RBX7Extents14getFaceCornersENS_8NormalIdERN3G3D7Vector3ES4_S4_S4_
pub fn stub_31ebfc() {
    // IDA 0x31ebfc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Extents::express(G3D::CoordinateFrame const&,G3D::CoordinateFrame const&)const")]
// 0x31ee8c — __ZNK3RBX7Extents7expressERKN3G3D15CoordinateFrameES4_
pub fn stub_31ee8c() {
    // IDA 0x31ee8c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Extents::toWorldSpace(G3D::CoordinateFrame const&)")]
// 0x31f1b4 — __ZN3RBX7Extents12toWorldSpaceERKN3G3D15CoordinateFrameE
pub fn stub_31f1b4() {
    // IDA 0x31f1b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Extents::computeClosestSqDistanceToPoint(G3D::Vector3 const&)const")]
// 0x31f4d0 — __ZNK3RBX7Extents31computeClosestSqDistanceToPointERKN3G3D7Vector3E
pub fn stub_31f4d0() {
    // IDA 0x31f4d0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Extents::vv(G3D::Vector3 const&,G3D::Vector3 const&)")]
// 0x31f68c — __ZN3RBX7Extents2vvERKN3G3D7Vector3ES4_
pub fn stub_31f68c() {
    // IDA 0x31f68c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Face::fuzzyContainsInExtrusion(G3D::Vector3 const&,float)const")]
// 0x31fcd4 — __ZNK3RBX4Face24fuzzyContainsInExtrusionERKN3G3D7Vector3Ef
pub fn stub_31fcd4() {
    // IDA 0x31fcd4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Face::minMax(G3D::Vector3 const&,G3D::Vector3 const&,float &,float &)const")]
// 0x31fdc4 — __ZNK3RBX4Face6minMaxERKN3G3D7Vector3ES4_RfS5_
pub fn stub_31fdc4() {
    // IDA 0x31fdc4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Face::toWorldSpace(G3D::CoordinateFrame const&)const")]
// 0x320024 — __ZNK3RBX4Face12toWorldSpaceERKN3G3D15CoordinateFrameE
pub fn stub_320024() {
    // IDA 0x320024: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Face::toObjectSpace(G3D::CoordinateFrame const&)const")]
// 0x32010c — __ZNK3RBX4Face13toObjectSpaceERKN3G3D15CoordinateFrameE
pub fn stub_32010c() {
    // IDA 0x32010c: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::InstanceHandle::InstanceHandle(RBX::Reflection::DescribedBase *)")]
// 0x322ec8 — __ZN3RBX14InstanceHandleC1EPNS_10Reflection13DescribedBaseE
pub fn stub_322ec8() {
    // IDA 0x322ec8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::InstanceHandle::operatorLess(RBX::InstanceHandle const&)const")]
// 0x322ed8 — __ZNK3RBX14InstanceHandle12operatorLessERKS0_
pub fn stub_322ed8() {
    // IDA 0x322ed8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::InstanceHandle::empty(void)const")]
// 0x322ee8 — __ZNK3RBX14InstanceHandle5emptyEv
pub fn stub_322ee8() {
    // IDA 0x322ee8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::HeartbeatInstance::onServiceProviderHeartbeatInstance(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x32305c — __ZN3RBX17HeartbeatInstance34onServiceProviderHeartbeatInstanceEPNS_15ServiceProviderES2_
pub fn stub_32305c() {
    // IDA 0x32305c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::HandleHitTest::hitTestHandleWorld(RBX::Extents const&,RBX::HandleType,RBX::RbxRay const&,G3D::Vector3 &,RBX::NormalId &,int)")]
// 0x32366c — __ZN3RBX13HandleHitTest18hitTestHandleWorldERKNS_7ExtentsENS_10HandleTypeERKNS_6RbxRayERN3G3D7Vector3ERNS_8NormalIdEi
pub fn stub_32366c() {
    // IDA 0x32366c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::HandleHitTest::hitTestHandleLocal(RBX::Extents const&,G3D::CoordinateFrame const&,RBX::HandleType,RBX::RbxRay const&,G3D::Vector3 &,RBX::NormalId &,int)")]
// 0x323768 — __ZN3RBX13HandleHitTest18hitTestHandleLocalERKNS_7ExtentsERKN3G3D15CoordinateFrameENS_10HandleTypeERKNS_6RbxRayERNS4_7Vector3ERNS_8NormalIdEi
pub fn stub_323768() {
    // IDA 0x323768: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::HandleHitTest::hitTestMoveHandleWorld(RBX::Extents const&,RBX::RbxRay const&,G3D::Vector3 &,RBX::NormalId &,int)")]
// 0x3238e8 — __ZN3RBX13HandleHitTest22hitTestMoveHandleWorldERKNS_7ExtentsERKNS_6RbxRayERN3G3D7Vector3ERNS_8NormalIdEi
pub fn stub_3238e8() {
    // IDA 0x3238e8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IndexBox::IndexBox(G3D::Vector3 const&,G3D::Vector3 const&)")]
// 0x323de8 — __ZN3RBX8IndexBoxC1ERKN3G3D7Vector3ES4_
pub fn stub_323de8() {
    // IDA 0x323de8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IndexBox::IndexBox(G3D::Vector3 const&,G3D::Vector3 const&)")]
// 0x323dec — __ZN3RBX8IndexBoxC2ERKN3G3D7Vector3ES4_
pub fn stub_323dec() {
    // IDA 0x323dec: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IndexBox::getFaceCorners(int,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)const")]
// 0x323efc — __ZNK3RBX8IndexBox14getFaceCornersEiRN3G3D7Vector3ES3_S3_S3_
pub fn stub_323efc() {
    // IDA 0x323efc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::IndexedTree *,10,32ul>::append(RBX::IndexedTree * const&)")]
// 0x324cec — __ZN3G3D5ArrayIPN3RBX11IndexedTreeELi10ELm32EE6appendERKS3_
pub fn stub_324cec() {
    // IDA 0x324cec: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::IndexedTree *,10,32ul>::resize(int,bool)")]
// 0x324d48 — __ZN3G3D5ArrayIPN3RBX11IndexedTreeELi10ELm32EE6resizeEib
pub fn stub_324d48() {
    // IDA 0x324d48: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::IndexedTree *,10,32ul>::realloc(int)")]
// 0x324e00 — __ZN3G3D5ArrayIPN3RBX11IndexedTreeELi10ELm32EE7reallocEi
pub fn stub_324e00() {
    // IDA 0x324e00: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::IndexedTree *,10,32ul>::~Array()")]
// 0x324fe8 — __ZN3G3D5ArrayIPN3RBX11IndexedTreeELi10ELm32EED2Ev
pub fn stub_324fe8() {
    // IDA 0x324fe8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::IndexedTree *,10,32ul>::Array(void)")]
// 0x3250bc — __ZN3G3D5ArrayIPN3RBX11IndexedTreeELi10ELm32EEC2Ev
pub fn stub_3250bc() {
    // IDA 0x3250bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::InterpolatedCFrame::setValue(RBX::PartInstance *,G3D::CoordinateFrame const&,RBX::RemoteTime const&)")]
// 0x3252f8 — __ZN3RBX18InterpolatedCFrame8setValueEPNS_12PartInstanceERKN3G3D15CoordinateFrameERKNS_10RemoteTimeE
pub fn stub_3252f8() {
    // IDA 0x3252f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::InterpolatedCFrame::computeValue(RBX::PartInstance *)")]
// 0x325998 — __ZN3RBX18InterpolatedCFrame12computeValueEPNS_12PartInstanceE
pub fn stub_325998() {
    // IDA 0x325998: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::InterpolatedCFrame::setRenderedFrame(G3D::CoordinateFrame const&)")]
// 0x325b08 — __ZN3RBX18InterpolatedCFrame16setRenderedFrameERKN3G3D15CoordinateFrameE
pub fn stub_325b08() {
    // IDA 0x325b08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::InterpolatedCFrame::setRenderedFrame(G3D::CoordinateFrame const&,RBX::RemoteTime const&)")]
// 0x325b4c — __ZN3RBX18InterpolatedCFrame16setRenderedFrameERKN3G3D15CoordinateFrameERKNS_10RemoteTimeE
pub fn stub_325b4c() {
    // IDA 0x325b4c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::EnumDesc(void)")]
// 0x326378 — __ZN3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEEC1Ev
pub fn stub_326378() {
    // IDA 0x326378: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::EnumDesc(void)")]
// 0x32637c — __ZN3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEEC2Ev
pub fn stub_32637c() {
    // IDA 0x32637c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::addPair(RBX::KeywordFilterType,char const*)")]
// 0x32653c — __ZN3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEE7addPairES2_PKc
pub fn stub_32653c() {
    // IDA 0x32653c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_125")]
// 0x326d8c — __GLOBAL__I_a_125
pub fn stub_326d8c() {
    // IDA 0x326d8c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "sub_3378C6")]
// 0x3378c6 — sub_3378C6
pub fn stub_3378c6() {
    // IDA 0x3378c6: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "RBX::LuaWebService::LuaWebService(void)")]
// 0x345ed0 — __ZN3RBX13LuaWebServiceC1Ev
pub fn stub_345ed0() {
    // IDA 0x345ed0: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "RBX::LuaWebService::LuaWebService(void)")]
// 0x345ed4 — __ZN3RBX13LuaWebServiceC2Ev
pub fn stub_345ed4() {
    // IDA 0x345ed4: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "RBX::LuaWebService::~LuaWebService()")]
// 0x34a4dc — __ZN3RBX13LuaWebServiceD1Ev
pub fn stub_34a4dc() {
    // IDA 0x34a4dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LuaWebService::~LuaWebService()")]
// 0x34a5ec — __ZN3RBX13LuaWebServiceD0Ev
pub fn stub_34a5ec() {
    // IDA 0x34a5ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE12getClassNameEv")]
// 0x34a714 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE12getClassNameEv
pub fn stub_34a714() {
    // IDA 0x34a714: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::LuaWebService::~LuaWebService()")]
// 0x34a740 — __ZThn32_N3RBX13LuaWebServiceD1Ev
pub fn stub_34a740() {
    // IDA 0x34a740: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::LuaWebService::~LuaWebService()")]
// 0x34a84c — __ZThn32_N3RBX13LuaWebServiceD0Ev
pub fn stub_34a84c() {
    // IDA 0x34a84c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE12getClassNameEv")]
// 0x34a970 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE12getClassNameEv
pub fn stub_34a970() {
    // IDA 0x34a970: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::LuaWebService::~LuaWebService()")]
// 0x34a998 — __ZThn36_N3RBX13LuaWebServiceD1Ev
pub fn stub_34a998() {
    // IDA 0x34a998: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::LuaWebService::~LuaWebService()")]
// 0x34aaa4 — __ZThn36_N3RBX13LuaWebServiceD0Ev
pub fn stub_34aaa4() {
    // IDA 0x34aaa4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sLuaWebServiceEEEEvv")]
// 0x34abc8 — __ZN3RBX4Name13callDoDeclareILZNS_14sLuaWebServiceEEEEvv
pub fn stub_34abc8() {
    // IDA 0x34abc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::findCacheItem(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo*)")]
// 0x34c404 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EE13findCacheItemERKSsPS2_
pub fn stub_34c404() {
    // IDA 0x34c404: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::findCacheItem(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo*)")]
// 0x3529bc — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EE13findCacheItemERKSsPS2_
pub fn stub_3529bc() {
    // IDA 0x3529bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::~AsyncHttpCache()")]
// 0x353088 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EED1Ev
pub fn stub_353088() {
    // IDA 0x353088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::~AsyncHttpCache()")]
// 0x353190 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EED0Ev
pub fn stub_353190() {
    // IDA 0x353190: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long)")]
// 0x353554 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6insertERKSsRKS2_m
pub fn stub_353554() {
    // IDA 0x353554: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long)")]
// 0x353588 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6insertERKSsRKS2_m
pub fn stub_353588() {
    // IDA 0x353588: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::removeLeastRecentlyUsed(void)")]
// 0x353b10 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE23removeLeastRecentlyUsedEv
pub fn stub_353b10() {
    // IDA 0x353b10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::remove(std::string const&)")]
// 0x353b68 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6removeERKSs
pub fn stub_353b68() {
    // IDA 0x353b68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>::destroy(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>*)")]
// 0x353c84 — __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEE7destroyEPS6_
pub fn stub_353c84() {
    // IDA 0x353c84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo> const&)")]
// 0x3541f8 — __ZNSt4pairISsS_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEC2ERKSsRKS3_
pub fn stub_3541f8() {
    // IDA 0x3541f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>> const&)")]
// 0x3542c4 — __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_3542c4() {
    // IDA 0x3542c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::~LRUCache()")]
// 0x3543d4 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEED2Ev
pub fn stub_3543d4() {
    // IDA 0x3543d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::resize(unsigned long)")]
// 0x3544e8 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6resizeEm
pub fn stub_3544e8() {
    // IDA 0x3544e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_clear(void)")]
// 0x354520 — __ZNSt10_List_baseISt4pairISsS0_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEESaIS5_EE8_M_clearEv
pub fn stub_354520() {
    // IDA 0x354520: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::LRUCache(void)")]
// 0x3545b4 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEEC2Ev
pub fn stub_3545b4() {
    // IDA 0x3545b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::resize(unsigned long)")]
// 0x354694 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6resizeEm
pub fn stub_354694() {
    // IDA 0x354694: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::~AsyncHttpCache()")]
// 0x354bb8 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EED1Ev
pub fn stub_354bb8() {
    // IDA 0x354bb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::~AsyncHttpCache()")]
// 0x354cc0 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EED0Ev
pub fn stub_354cc0() {
    // IDA 0x354cc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long)")]
// 0x355034 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6insertERKSsRKS2_m
pub fn stub_355034() {
    // IDA 0x355034: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long)")]
// 0x3550a8 — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6insertERKSsRKS2_m
pub fn stub_3550a8() {
    // IDA 0x3550a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>)")]
// 0x3556e4 — __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE8_M_eraseESt14_List_iteratorIS5_E
pub fn stub_3556e4() {
    // IDA 0x3556e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo> const&)")]
// 0x355c80 — __ZNSt4pairISsS_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEC2ERKSsRKS3_
pub fn stub_355c80() {
    // IDA 0x355c80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>> const&)")]
// 0x355d60 — __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_355d60() {
    // IDA 0x355d60: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::~LRUCache()")]
// 0x355e88 — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEED2Ev
pub fn stub_355e88() {
    // IDA 0x355e88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::resize(unsigned long)")]
// 0x355f9c — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6resizeEm
pub fn stub_355f9c() {
    // IDA 0x355f9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_clear(void)")]
// 0x356010 — __ZNSt10_List_baseISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE8_M_clearEv
pub fn stub_356010() {
    // IDA 0x356010: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::LRUCache(void)")]
// 0x356164 — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEEC2Ev
pub fn stub_356164() {
    // IDA 0x356164: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::resize(unsigned long)")]
// 0x356244 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6resizeEm
pub fn stub_356244() {
    // IDA 0x356244: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x356388 — __ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_356388() {
    // IDA 0x356388: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x35638c — __ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_35638c() {
    // IDA 0x35638c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x35642c — __ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_35642c() {
    // IDA 0x35642c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x356434 — __ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_356434() {
    // IDA 0x356434: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3564d8 — __ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3564d8() {
    // IDA 0x3564d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3564e0 — __ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3564e0() {
    // IDA 0x3564e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "global constructor keyed to_a_126")]
// 0x3565e4 — __GLOBAL__I_a_126
pub fn stub_3565e4() {
    // IDA 0x3565e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Math::sumDeltaAxis(G3D::Matrix3 const&,G3D::Matrix3 const&)")]
// 0x35677c — __ZN3RBX4Math12sumDeltaAxisERKN3G3D7Matrix3ES4_
pub fn stub_35677c() {
    // IDA 0x35677c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Math::mulMatrixDiagVector(G3D::Matrix3 const&,G3D::Vector3 const&,G3D::Matrix3&)")]
// 0x3567e0 — __ZN3RBX4Math19mulMatrixDiagVectorERKN3G3D7Matrix3ERKNS1_7Vector3ERS2_
pub fn stub_3567e0() {
    // IDA 0x3567e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Math::mulMatrixMatrixTranspose(G3D::Matrix3 const&,G3D::Matrix3 const&,G3D::Matrix3&)")]
// 0x356878 — __ZN3RBX4Math24mulMatrixMatrixTransposeERKN3G3D7Matrix3ES4_RS2_
pub fn stub_356878() {
    // IDA 0x356878: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Math::getFocusSpace(G3D::CoordinateFrame const&)")]
// 0x356ae0 — __ZN3RBX4Math13getFocusSpaceERKN3G3D15CoordinateFrameE
pub fn stub_356ae0() {
    // IDA 0x356ae0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Math::getHeadingElevation(G3D::CoordinateFrame const&,float &,float &)")]
// 0x356b18 — __ZN3RBX4Math19getHeadingElevationERKN3G3D15CoordinateFrameERfS5_
pub fn stub_356b18() {
    // IDA 0x356b18: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::setHeadingElevation(G3D::CoordinateFrame &,float,float)")]
// 0x356b84 — __ZN3RBX4Math19setHeadingElevationERN3G3D15CoordinateFrameEff
pub fn stub_356b84() {
    // IDA 0x356b84: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::lessThan(G3D::Vector3 const&,G3D::Vector3 const&)")]
// 0x356c3c — __ZN3RBX4Math8lessThanERKN3G3D7Vector3ES4_
pub fn stub_356c3c() {
    // IDA 0x356c3c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::isNanInfVector3(G3D::Vector3 const&)")]
// 0x356cc8 — __ZN3RBX4Math15isNanInfVector3ERKN3G3D7Vector3E
pub fn stub_356cc8() {
    // IDA 0x356cc8: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::isNanInfDenormVector3(G3D::Vector3 const&)")]
// 0x356d38 — __ZN3RBX4Math21isNanInfDenormVector3ERKN3G3D7Vector3E
pub fn stub_356d38() {
    // IDA 0x356d38: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::hasNanOrInf(G3D::CoordinateFrame const&)")]
// 0x356d70 — __ZN3RBX4Math11hasNanOrInfERKN3G3D15CoordinateFrameE
pub fn stub_356d70() {
    // IDA 0x356d70: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::fixDenorm(G3D::Vector3 &)")]
// 0x356df4 — __ZN3RBX4Math9fixDenormERN3G3D7Vector3E
pub fn stub_356df4() {
    // IDA 0x356df4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::getIWorldAtPoint(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Matrix3 const&,float)")]
// 0x35711c — __ZN3RBX4Math16getIWorldAtPointERKN3G3D7Vector3ES4_RKNS1_7Matrix3Ef
pub fn stub_35711c() {
    // IDA 0x35711c: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::getIBodyAtPoint(G3D::Vector3 const&,G3D::Matrix3 const&,float)")]
// 0x3571c0 — __ZN3RBX4Math15getIBodyAtPointERKN3G3D7Vector3ERKNS1_7Matrix3Ef
pub fn stub_3571c0() {
    // IDA 0x3571c0: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::momentToObjectSpace(G3D::Matrix3 const&,G3D::Matrix3 const&)")]
// 0x357250 — __ZN3RBX4Math19momentToObjectSpaceERKN3G3D7Matrix3ES4_
pub fn stub_357250() {
    // IDA 0x357250: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::momentToWorldSpace(G3D::Matrix3 const&,G3D::Matrix3 const&)")]
// 0x35728c — __ZN3RBX4Math18momentToWorldSpaceERKN3G3D7Matrix3ES4_
pub fn stub_35728c() {
    // IDA 0x35728c: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::toDiagonal(G3D::Matrix3 const&)")]
// 0x3572c4 — __ZN3RBX4Math10toDiagonalERKN3G3D7Matrix3E
pub fn stub_3572c4() {
    // IDA 0x3572c4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::fromVectorToVectorRotation(G3D::Vector3 const&,G3D::Vector3 const&)")]
// 0x3572e4 — __ZN3RBX4Math26fromVectorToVectorRotationERKN3G3D7Vector3ES4_
pub fn stub_3572e4() {
    // IDA 0x3572e4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::fromRotationAxisAndAngle(G3D::Vector3 const&,float const&)")]
// 0x357450 — __ZN3RBX4Math24fromRotationAxisAndAngleERKN3G3D7Vector3ERKf
pub fn stub_357450() {
    // IDA 0x357450: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::orthonormalizeIfNecessary(G3D::Matrix3 &)")]
// 0x3575bc — __ZN3RBX4Math25orthonormalizeIfNecessaryERN3G3D7Matrix3E
pub fn stub_3575bc() {
    // IDA 0x3575bc: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Math::fromDirectionCosines(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&)")]
// 0x3575dc — __ZN3RBX4Math20fromDirectionCosinesERKN3G3D7Vector3ES4_S4_S4_S4_S4_
pub fn stub_3575dc() {
    // IDA 0x3575dc: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}
// ============================================================
// IDA-grounded implementations — RBX::Math matrix/vector/frame
// helpers (0x35677c..0x3575dc, 20 fns). The carrier stubs above
// are unchanged; the items below are the live Rust ports.
// Conventions: G3D::Vector3 is [f32; 3]; G3D::Matrix3 is row-major
// [[f32; 3]; 3] (flat words m[i * 3 + j], IDA offsets +0..+32);
// G3D::CoordinateFrame is rotation + translation at +36..+47
// (disasm 0x356b84/0x356ae0: tx at +0x24, ty +0x28, tz +0x2c).
// All float arithmetic stays f32 (sinf/cosf/sqrtf/atan2f per the
// BLX targets in 0x356b84); double precision is used only where
// the decompile shows double sin/cos/asin (0x357450, 0x3572e4,
// 0x356b18), narrowed back to float on store.
// ============================================================

/// was: `RBX::Math` matrix/vector/frame helpers
/// (`Client/App/util/Math.cpp`, G3D `Matrix3`/`CoordinateFrame`).
pub mod vector_matrix {
    /// was: `G3D::Vector3` (three floats, IDA words +0/+4/+8).
    pub type Vector3 = [f32; 3];
    /// was: `G3D::Matrix3` row-major block (IDA words +0..+32).
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Matrix3(pub [[f32; 3]; 3]);
    /// was: `G3D::CoordinateFrame` (rotation + translation at +36).
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct CoordinateFrame {
        pub rotation: Matrix3,
        pub translation: Vector3,
    }

    impl Matrix3 {
        pub const fn from_rows(rows: [[f32; 3]; 3]) -> Self {
            Self(rows)
        }
        pub fn identity() -> Self {
            Self([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        }
        pub fn column(&self, index: usize) -> Vector3 {
            [self.0[0][index], self.0[1][index], self.0[2][index]]
        }
        pub fn transpose(&self) -> Self {
            Self([
                [self.0[0][0], self.0[1][0], self.0[2][0]],
                [self.0[0][1], self.0[1][1], self.0[2][1]],
                [self.0[0][2], self.0[1][2], self.0[2][2]],
            ])
        }
        pub fn mul(&self, other: &Self) -> Self {
            let mut out = [[0.0f32; 3]; 3];
            let mut r = 0;
            while r < 3 {
                let mut c = 0;
                while c < 3 {
                    out[r][c] =
                        self.0[r][0] * other.0[0][c] + self.0[r][1] * other.0[1][c] + self.0[r][2] * other.0[2][c];
                    c += 1;
                }
                r += 1;
            }
            Self(out)
        }
    }

    fn dot3(a: Vector3, b: Vector3) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    fn cross3(a: Vector3, b: Vector3) -> Vector3 {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    fn sub3(a: Vector3, b: Vector3) -> Vector3 {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }

    fn scale3(v: Vector3, s: f32) -> Vector3 {
        [v[0] * s, v[1] * s, v[2] * s]
    }

    fn add_scaled(a: Vector3, b: Vector3, s: f32) -> Vector3 {
        [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s]
    }

    /// G3D `Vector3::direction` as used by 0xc3ccdc: normalize with
    /// `1.0 / sqrtf`, no zero guard (IEEE inf/nan preserved).
    fn normalize(v: Vector3) -> Vector3 {
        scale3(v, 1.0 / dot3(v, v).sqrt())
    }

    /// G3D fuzzy zero from 0xc3f068: `x == 0` fast path, else
    /// `|x| <= (|x| + 1) * 1e-5`, with the inf guard (IDA 0xc3f0ea:
    /// when `|x| + 1 == inf` the bound stays `1e-5` so inf reads false).
    fn fuzzy_zero(value: f32) -> bool {
        if value == 0.0 {
            return true;
        }
        let magnitude = value.abs();
        let bound = if magnitude + 1.0 == f32::INFINITY {
            0.00001
        } else {
            (magnitude + 1.0) * 0.00001
        };
        magnitude <= bound
    }

    /// G3D fuzzy one from 0xc3f068 (`|x * x - 1|` variant, IDA 0xc3f228).
    fn fuzzy_unit(square: f32) -> bool {
        if square == 1.0 {
            return true;
        }
        let magnitude = square.abs();
        let bound = if magnitude + 1.0 == f32::INFINITY {
            0.00001
        } else {
            (magnitude + 1.0) * 0.00001
        };
        (square - 1.0).abs() <= bound
    }

    /// G3D::Matrix3::isOrthonormal (IDA 0xc3f068): pairwise column
    /// dots are fuzzy-zero and each squared column norm is fuzzy-one,
    /// short-circuiting in that order.
    fn is_orthonormal(m: &Matrix3) -> bool {
        let c0 = m.column(0);
        let c1 = m.column(1);
        let c2 = m.column(2);
        fuzzy_zero(dot3(c0, c1)) && fuzzy_zero(dot3(c1, c2)) && fuzzy_zero(dot3(c0, c2))
            && fuzzy_unit(dot3(c0, c0))
            && fuzzy_unit(dot3(c1, c1))
            && fuzzy_unit(dot3(c2, c2))
    }

    /// G3D::Matrix3::orthonormalize (IDA 0xc3fd1c): modified
    /// Gram-Schmidt over columns, `1.0 / sqrt` with no zero guard.
    fn orthonormalize(m: &mut Matrix3) {
        let mut c0 = m.column(0);
        c0 = normalize(c0);
        let mut c1 = m.column(1);
        let d = dot3(c0, c1);
        c1 = normalize(sub3(c1, scale3(c0, d)));
        let c2 = m.column(2);
        let e0 = dot3(c0, c2);
        let e1 = dot3(c1, c2);
        let c2 = normalize(sub3(c2, add_scaled(scale3(c0, e0), c1, e1)));
        m.0 = [[c0[0], c1[0], c2[0]], [c0[1], c1[1], c2[1]], [c0[2], c1[2], c2[2]]];
    }

    /// G3D::CoordinateFrame::lookAt(target, up) (IDA 0xc3ccdc, via the
    /// 2-arg 0xc3ccb8 which passes `unitY`): `z = (target - eye)` unit,
    /// unitX/unitY fallback past `|up . z| > 0.99`, `x = up - z(up . z)`
    /// unit, then columns `(x, zc x x, zc)` with `zc = -z` so `-Z`
    /// faces the target. The column signs are forced: the decompile
    /// negates `z` into column 2 (0xc3ce66-0xc3ce84), and only
    /// `col2 = -z` keeps a proper rotation (det +1) and keeps
    /// get/setHeadingElevation round-tripping (0x356b18/0x356b84).
    fn look_at(frame: &mut CoordinateFrame, target: Vector3) {
        let mut up: Vector3 = [0.0, 1.0, 0.0];
        let z = normalize(sub3(target, frame.translation));
        if dot3(up, z).abs() > 0.99 {
            up = [1.0, 0.0, 0.0];
            if dot3(up, z).abs() > 0.99 {
                up = [0.0, 1.0, 0.0];
            }
        }
        let x = normalize(sub3(up, scale3(z, dot3(up, z))));
        let zc = scale3(z, -1.0);
        let y = cross3(zc, x);
        let x = cross3(y, zc);
        frame.rotation = Matrix3([[x[0], y[0], zc[0]], [x[1], y[1], zc[1]], [x[2], y[2], zc[2]]]);
    }

    #[doc(alias = "RBX::Math::sumDeltaAxis")]
    // 0x35677c — __ZN3RBX4Math12sumDeltaAxisERKN3G3D7Matrix3ES4_
    // IDA 0x35677c: three passes over columns 0..2 accumulating
    // |a - b| per element: the L1 (elementwise abs) distance of the
    // two 3x3 blocks. Returns float.
    pub fn sum_delta_axis(a: &Matrix3, b: &Matrix3) -> f32 {
        let mut total = 0.0f32;
        let mut c = 0;
        while c < 3 {
            let mut r = 0;
            while r < 3 {
                total += (a.0[r][c] - b.0[r][c]).abs();
                r += 1;
            }
            c += 1;
        }
        total
    }

    #[doc(alias = "RBX::Math::mulMatrixDiagVector")]
    // 0x3567e0 — __ZN3RBX4Math19mulMatrixDiagVectorERKN3G3D7Matrix3ERKNS1_7Vector3ERS2_
    // IDA 0x3567e0: out[i] = mat[i] * vec[i % 3], i in 0..9, i.e.
    // out = mat * diag(vec) (column scaling). The disasm returns the
    // input matrix pointer (by-value hidden-pointer artifact); the
    // live port returns the fresh product.
    pub fn mul_matrix_diag_vector(mat: &Matrix3, vec: Vector3) -> Matrix3 {
        let mut out = [[0.0f32; 3]; 3];
        let mut r = 0;
        while r < 3 {
            let mut c = 0;
            while c < 3 {
                out[r][c] = mat.0[r][c] * vec[c];
                c += 1;
            }
            r += 1;
        }
        Matrix3(out)
    }

    #[doc(alias = "RBX::Math::mulMatrixMatrixTranspose")]
    // 0x356878 — __ZN3RBX4Math24mulMatrixMatrixTransposeERKN3G3D7Matrix3ES4_RS2_
    // IDA 0x356878: out[i][j] = dot(row i of a, row j of b), i.e.
    // out = a * transpose(b).
    pub fn mul_matrix_matrix_transpose(a: &Matrix3, b: &Matrix3) -> Matrix3 {
        let mut out = [[0.0f32; 3]; 3];
        let mut r = 0;
        while r < 3 {
            let mut c = 0;
            while c < 3 {
                out[r][c] = a.0[r][0] * b.0[c][0] + a.0[r][1] * b.0[c][1] + a.0[r][2] * b.0[c][2];
                c += 1;
            }
            r += 1;
        }
        Matrix3(out)
    }

    #[doc(alias = "RBX::Math::getHeadingElevation")]
    // 0x356b18 — __ZN3RBX4Math19getHeadingElevationERKN3G3D15CoordinateFrameERfS5_
    // IDA 0x356b18 (disasm: column index literal 2): col = rotation
    // column 2; heading = atan2(col.x, col.z); elevation = asin(-col.y)
    // with the asin in double, narrowed to float on store. The C++
    // double return is the same elevation value.
    pub fn get_heading_elevation(frame: &CoordinateFrame) -> (f32, f32) {
        let col = frame.rotation.column(2);
        let heading = col[0].atan2(col[2]);
        let elevation = (-col[1] as f64).asin() as f32;
        (heading, elevation)
    }

    #[doc(alias = "RBX::Math::setHeadingElevation")]
    // 0x356b84 — __ZN3RBX4Math19setHeadingElevationERN3G3D15CoordinateFrameEff
    // IDA 0x356b84 (disasm: R1/R2 are the two float params, BLX sinf/
    // cosf): se = sin(elev); ce = sqrt(1 - se*se) (note: always >= 0,
    // unlike cos, for |elev| > pi/2); sh/ch = sin/cos(heading);
    // s = 1/sqrt(se*se + (ce*sh)^2 + (ce*ch)^2) (~1, kept for exact
    // float behavior); target = translation + (-ce*sh*s, se*s,
    // -ce*ch*s); lookAt(target). Translation is preserved.
    pub fn set_heading_elevation(frame: &mut CoordinateFrame, heading: f32, elevation: f32) {
        let se = elevation.sin();
        let ce = (1.0 - se * se).sqrt();
        let sh = heading.sin();
        let ch = heading.cos();
        let x = ce * sh;
        let z = ce * ch;
        let s = 1.0 / (se * se + x * x + z * z).sqrt();
        let target = [
            frame.translation[0] - x * s,
            frame.translation[1] + se * s,
            frame.translation[2] - z * s,
        ];
        look_at(frame, target);
    }

    #[doc(alias = "RBX::Math::getFocusSpace")]
    // 0x356ae0 — __ZN3RBX4Math13getFocusSpaceERKN3G3D15CoordinateFrameE
    // IDA 0x356ae0 (disasm: copies rotation via the Matrix3 copy ctor
    // and translation words +0x24/+0x2c, then setHeadingElevation with
    // the heading temp and a 0.0f literal): out = frame with elevation
    // zeroed, heading preserved.
    pub fn get_focus_space(frame: &CoordinateFrame) -> CoordinateFrame {
        let (heading, _) = get_heading_elevation(frame);
        let mut out = *frame;
        set_heading_elevation(&mut out, heading, 0.0);
        out
    }

    #[doc(alias = "RBX::Math::lessThan")]
    // 0x356c3c — __ZN3RBX4Math8lessThanERKN3G3D7Vector3ES4_
    // IDA 0x356c3c: componentwise strict less on all three lanes.
    pub fn less_than(a: Vector3, b: Vector3) -> bool {
        a[0] < b[0] && a[1] < b[1] && a[2] < b[2]
    }

    #[doc(alias = "RBX::Math::isNanInfVector3")]
    // 0x356cc8 — __ZN3RBX4Math15isNanInfVector3ERKN3G3D7Vector3E
    // IDA 0x356cc8: true if any |lane| == INFINITY, short-circuit x,y,z.
    // BUG: original at 0x356cc8 — the name claims NaN too, but
    // fabsf(NaN) != INFINITY, so NaN lanes read false; preserved here.
    pub fn is_nan_inf_vector3(v: Vector3) -> bool {
        v[0].abs() == f32::INFINITY || v[1].abs() == f32::INFINITY || v[2].abs() == f32::INFINITY
    }

    #[doc(alias = "RBX::Math::isNanInfDenormVector3")]
    // 0x356d38 — __ZN3RBX4Math21isNanInfDenormVector3ERKN3G3D7Vector3E
    // IDA 0x356d38: per lane `__fpclassifyf(x) - 3 <= 1` (unsigned)
    // continues, else true; false only when every lane classifies to
    // 3 or 4. Scalar isDenormal (0x356c80) shows class 5 = subnormal,
    // so {3, 4} are exactly the two clean classes (normal, zero) and
    // any NaN/Inf/subnormal lane reads true, matching the name.
    pub fn is_nan_inf_denorm_vector3(v: Vector3) -> bool {
        (v[0].is_nan() || v[0].is_infinite() || v[0].is_subnormal())
            || (v[1].is_nan() || v[1].is_infinite() || v[1].is_subnormal())
            || (v[2].is_nan() || v[2].is_infinite() || v[2].is_subnormal())
    }

    #[doc(alias = "RBX::Math::hasNanOrInf")]
    // 0x356d70 — __ZN3RBX4Math11hasNanOrInfERKN3G3D15CoordinateFrameE
    // IDA 0x356d70: scans translation (words +9..+11) then all nine
    // rotation words for |x| == INFINITY.
    // BUG: original at 0x356d70 — NaN never matches (fabsf(NaN) !=
    // INFINITY), despite the name; preserved here.
    pub fn has_nan_or_inf(frame: &CoordinateFrame) -> bool {
        frame.translation.iter().any(|c| c.abs() == f32::INFINITY)
            || frame.rotation.0.iter().flatten().any(|c| c.abs() == f32::INFINITY)
    }

    #[doc(alias = "RBX::Math::fixDenorm")]
    // 0x356df4 — __ZN3RBX4Math9fixDenormERN3G3D7Vector3E
    // IDA 0x356df4: zeroes each subnormal lane (`__fpclassifyf == 5`,
    // cf. 0x356c80); returns whether any lane was fixed.
    pub fn fix_denorm(v: &mut Vector3) -> bool {
        let mut fixed = false;
        for lane in v.iter_mut() {
            if lane.is_subnormal() {
                *lane = 0.0;
                fixed = true;
            }
        }
        fixed
    }

    /// Shared point-mass inertia summand of 0x35711c/0x3571c0:
    /// `|d|^2 * I - d * dT` for offset `d`.
    fn point_inertia(d: Vector3) -> Matrix3 {
        Matrix3([
            [d[1] * d[1] + d[2] * d[2], -(d[0] * d[1]), -(d[0] * d[2])],
            [-(d[0] * d[1]), d[0] * d[0] + d[2] * d[2], -(d[1] * d[2])],
            [-(d[0] * d[2]), -(d[1] * d[2]), d[0] * d[0] + d[1] * d[1]],
        ])
    }

    fn add_scaled_matrix(a: &Matrix3, b: &Matrix3, s: f32) -> Matrix3 {
        let mut out = [[0.0f32; 3]; 3];
        let mut r = 0;
        while r < 3 {
            let mut c = 0;
            while c < 3 {
                out[r][c] = a.0[r][c] + b.0[r][c] * s;
                c += 1;
            }
            r += 1;
        }
        Matrix3(out)
    }

    #[doc(alias = "RBX::Math::getIWorldAtPoint")]
    // 0x35711c — __ZN3RBX4Math16getIWorldAtPointERKN3G3D7Vector3ES4_RKNS1_7Matrix3Ef
    // IDA 0x35711c: d = p2 - p1; returns inertia + mass * point_inertia(d)
    // (parallel-axis shift of the world inertia to the point).
    pub fn get_i_world_at_point(p1: Vector3, p2: Vector3, inertia: &Matrix3, mass: f32) -> Matrix3 {
        let d = sub3(p2, p1);
        add_scaled_matrix(inertia, &point_inertia(d), mass)
    }

    #[doc(alias = "RBX::Math::getIBodyAtPoint")]
    // 0x3571c0 — __ZN3RBX4Math15getIBodyAtPointERKN3G3D7Vector3ERKNS1_7Matrix3Ef
    // IDA 0x3571c0: same summand as 0x35711c but d = p itself;
    // returns inertia + mass * point_inertia(p).
    pub fn get_i_body_at_point(p: Vector3, inertia: &Matrix3, mass: f32) -> Matrix3 {
        add_scaled_matrix(inertia, &point_inertia(p), mass)
    }

    #[doc(alias = "RBX::Math::momentToObjectSpace")]
    // 0x357250 — __ZN3RBX4Math19momentToObjectSpaceERKN3G3D7Matrix3ES4_
    // IDA 0x357250 (disasm: the transpose BL takes the SECOND matrix,
    // then out = transposed * first, out = that * second): with
    // (inertia, rotation) argument order this is R^T * I * R.
    pub fn moment_to_object_space(inertia: &Matrix3, rotation: &Matrix3) -> Matrix3 {
        rotation.transpose().mul(inertia).mul(rotation)
    }

    #[doc(alias = "RBX::Math::momentToWorldSpace")]
    // 0x35728c — __ZN3RBX4Math18momentToWorldSpaceERKN3G3D7Matrix3ES4_
    // IDA 0x35728c (disasm mirrors 0x357250: tmp = second * first,
    // out = tmp * transpose(second)): R * I * R^T.
    pub fn moment_to_world_space(inertia: &Matrix3, rotation: &Matrix3) -> Matrix3 {
        rotation.mul(inertia).mul(&rotation.transpose())
    }

    #[doc(alias = "RBX::Math::toDiagonal")]
    // 0x3572c4 — __ZN3RBX4Math10toDiagonalERKN3G3D7Matrix3E
    // IDA 0x3572c4: returns words 0, 4, 8 (the matrix diagonal).
    pub fn to_diagonal(m: &Matrix3) -> Vector3 {
        [m.0[0][0], m.0[1][1], m.0[2][2]]
    }

    #[doc(alias = "RBX::Math::fromRotationAxisAndAngle")]
    // 0x357450 — __ZN3RBX4Math24fromRotationAxisAndAngleERKN3G3D7Vector3ERKf
    // IDA 0x357450: identity when |axis| <= 0.001 (float sqrtf);
    // else Rodrigues with double sin/cos, C = 1 - cos, narrowed to
    // float on Matrix3::set, then orthonormalizeIfNecessary. The axis
    // is NOT normalized (callers pass unit axes); the C++ Matrix3
    // return travels by hidden pointer while the trailing
    // orthonormalize call value is a disasm artifact.
    pub fn from_rotation_axis_and_angle(axis: Vector3, angle: f32) -> Matrix3 {
        let mut out = Matrix3::identity();
        let length = dot3(axis, axis).sqrt();
        if length > 0.001 {
            let (x, y, z) = (axis[0] as f64, axis[1] as f64, axis[2] as f64);
            let c = (angle as f64).cos();
            let s = (angle as f64).sin();
            let cc = 1.0 - c;
            out = Matrix3([
                [(x * x * cc + c) as f32, (x * y * cc - z * s) as f32, (x * z * cc + y * s) as f32],
                [(y * x * cc + z * s) as f32, (y * y * cc + c) as f32, (y * z * cc - x * s) as f32],
                [(z * x * cc - y * s) as f32, (z * y * cc + x * s) as f32, (z * z * cc + c) as f32],
            ]);
        }
        orthonormalize_if_necessary(&mut out);
        out
    }

    #[doc(alias = "RBX::Math::fromVectorToVectorRotation")]
    // 0x3572e4 — __ZN3RBX4Math26fromVectorToVectorRotationERKN3G3D7Vector3ES4_
    // IDA 0x3572e4: identity default; axis = from x to (|axis| float
    // sqrtf); when |axis| > 0.001, angle = acos(dot / (|from||to|))
    // (double acos, narrowed) about the normalized axis via
    // fromRotationAxisAndAngle.
    // BUG: original at 0x3572e4 — antiparallel inputs (180 degrees)
    // have a ~zero cross product, so the function returns identity
    // instead of a half-turn; preserved here.
    pub fn from_vector_to_vector_rotation(from: Vector3, to: Vector3) -> Matrix3 {
        let axis = cross3(from, to);
        let axis_length = dot3(axis, axis).sqrt();
        if axis_length > 0.001 {
            let cos_angle = dot3(from, to) / (dot3(from, from).sqrt() * dot3(to, to).sqrt());
            let angle = (cos_angle as f64).acos() as f32;
            return from_rotation_axis_and_angle(scale3(axis, 1.0 / axis_length), angle);
        }
        Matrix3::identity()
    }

    #[doc(alias = "RBX::Math::orthonormalizeIfNecessary")]
    // 0x3575bc — __ZN3RBX4Math25orthonormalizeIfNecessaryERN3G3D7Matrix3E
    // IDA 0x3575bc (disasm: early return 0 when isOrthonormal, else
    // orthonormalize + return 1): in-place fixup, reports whether it ran.
    pub fn orthonormalize_if_necessary(m: &mut Matrix3) -> bool {
        if is_orthonormal(m) {
            return false;
        }
        orthonormalize(m);
        true
    }

    #[doc(alias = "RBX::Math::fromDirectionCosines")]
    // 0x3575dc — __ZN3RBX4Math20fromDirectionCosinesERKN3G3D7Vector3ES4_S4_S4_S4_S4_
    // IDA 0x3575dc: change-of-basis rotation with rows from the second
    // triple and columns from the first: out[i][j] = to_i . from_j,
    // then orthonormalizeIfNecessary (Matrix3 return by hidden pointer).
    pub fn from_direction_cosines(
        from_x: Vector3,
        from_y: Vector3,
        from_z: Vector3,
        to_x: Vector3,
        to_y: Vector3,
        to_z: Vector3,
    ) -> Matrix3 {
        let from = [from_x, from_y, from_z];
        let to = [to_x, to_y, to_z];
        let mut out = [[0.0f32; 3]; 3];
        let mut r = 0;
        while r < 3 {
            let mut c = 0;
            while c < 3 {
                out[r][c] = dot3(to[r], from[c]);
                c += 1;
            }
            r += 1;
        }
        let mut out = Matrix3(out);
        orthonormalize_if_necessary(&mut out);
        out
    }
}
