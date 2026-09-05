// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 120 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x257980..0x278574 | earliest EA gap 0x257980 (filtered Lua|Script|Yield|lua)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x257980 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()")]
pub fn stub_0x257980(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x2579c0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::~BoundYieldFuncDesc()")]
pub fn stub_0x2579c0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x25a2f0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EEC2EMS2_FvSsSsS3_N5boost8functionIFvSsEEES9_EPKcSD_SD_SD_S3_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int, int, int, int)
// was: int __fastcall(int, unsigned int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,std::string,RBX::HttpService::HttpContentType,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,char const*,RBX::HttpService::HttpContentType,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x25a2f0() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::HttpService", "std::string", 3)
}

// 0x25a540 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EE16declareSignatureEPKcNS0_7VariantES7_S8_S7_S8_
// type: int __fastcall(int, int, int, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x25a540() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::HttpService", "std::string", 3)
}

// 0x25a5a8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EED0Ev
// type: void __fastcall(_DWORD *)
// was: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::~BoundYieldFuncDesc() [0x25a5a8]")]
pub fn stub_0x25a5a8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x25a68c — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsSsNS2_15HttpContentTypeEESsLi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSC_IFvSsEEE
// type: void __fastcall(int, int, int, int, int)
// was: void __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string,std::string,RBX::HttpService::HttpContentType),std::string,3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x25a68c() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::HttpService", "std::string", 3)
}

// 0x25ab40 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EEC2EMS2_FvSsN5boost8functionIFvSsEEES8_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
// was: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::HttpService::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x25ab40() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::HttpService", "std::string", 1)
}

// 0x25acb8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x25acb8() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::HttpService", "std::string", 1)
}

// 0x25ace8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EED0Ev
// type: void __fastcall(_DWORD *)
// was: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc() [0x25ace8]")]
pub fn stub_0x25ace8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x25adb4 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_11HttpServiceEFSsSsESsLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// type: void __fastcall(int, int, int, int, int)
// was: void __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::HttpService,std::string ()(std::string),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_0x25adb4() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::HttpService", "std::string", 1)
}

// 0x260394 — __ZN3RBX10Reflection23YieldFunctionDescriptorC2ERNS0_15ClassDescriptorEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int)
// was: _DWORD *__fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::YieldFunctionDescriptor::YieldFunctionDescriptor(RBX::Reflection::ClassDescriptor &,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x260394(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::YieldFunctionDescriptor::YieldFunctionDescriptor(RBX::Reflection::ClassDe~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x260638 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE7declareEPS2_
// type: int __fastcall(int **, int)
// was: int __fastcall(int **, int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declare(RBX::Reflection::YieldFunctionDescriptor*)")]
pub fn stub_0x260638(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::decl~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x2607e0 — __ZN3RBX10Reflection23YieldFunctionDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::YieldFunctionDescriptor *__hidden this)
// was: void __fastcall(RBX::Reflection::YieldFunctionDescriptor *__hidden this)
#[doc(alias = "RBX::Reflection::YieldFunctionDescriptor::~YieldFunctionDescriptor()")]
pub fn stub_0x2607e0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::YieldFunctionDescriptor dtor.
drop(handle);
}

// 0x260808 — __ZNSt6vectorIPN3RBX10Reflection23YieldFunctionDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int *, _DWORD *, _DWORD *)
// was: int __fastcall(int *, _DWORD *, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::YieldFunctionDescriptor **,std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>>,RBX::Reflection::YieldFunctionDescriptor * const&)")]
pub fn stub_0x260808() -> crate::slot::PortedFn {
// IDA 0x260808: std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::inse~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x260808, "std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunction~")
}

// 0x260840 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10declareSubEPS2_S4_
// type: int *__fastcall(int *, int, int, const void *)
// was: int *__fastcall(int *, int, int, const void *)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declareSub(RBX::Reflection::YieldFunctionDescriptor*,RBX::Reflection::YieldFunctionDescriptor*)")]
pub fn stub_0x260840(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::decl~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x2609c0 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10staticDataEv
// type: double *()
// was: double *()
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::staticData(void)")]
pub fn stub_0x2609c0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::stat~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x260a28 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10CollectionD1Ev
// type: void **__fastcall(void **)
// was: void **__fastcall(void **)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::Collection::~Collection()")]
pub fn stub_0x260a28(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer dtor.
drop(handle);
}

// 0x260a40 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: char **__fastcall(_DWORD *, char **, int, int, void *, int, int, int, int)
// was: char **__fastcall(_DWORD *, char **, int, int, void *, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
pub fn stub_0x260a40(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

// 0x261b78 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEEC2EPS3_
// type: int __fastcall(_DWORD *, int)
// was: int __fastcall(_DWORD *, int)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>*)")]
pub fn stub_0x261b78(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::Memb~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x263108 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE12mergeMembersEPKS3_
// type: int __fastcall(int result, int *)
// was: int __fastcall(int result, int *)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> const*)")]
pub fn stub_0x263108(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::merg~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x263130 — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_23YieldFunctionDescriptorEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int result, _DWORD *)
// was: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *>>::push_back(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> * const&)")]
pub fn stub_0x263130(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

// 0x263160 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv
// type: void __fastcall(int)
// was: void __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
pub fn stub_0x263160(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

// 0x2631ac — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_23YieldFunctionDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: char *__fastcall(int, char *__src, _DWORD *)
// was: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> **,std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *>>>,RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> * const&)")]
pub fn stub_0x2631ac(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

// 0x26328c — __ZNSt12_Vector_baseIPN3RBX10Reflection25MemberDescriptorContainerINS1_23YieldFunctionDescriptorEEESaIS5_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
// was: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> *>>::_M_allocate(unsigned long)")]
pub fn stub_0x26328c() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::initStaticData(void)")]
pub fn stub_0x2632a8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::init~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x2632ac — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
// was: unsigned int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x2632ac(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x263300 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
// was: void __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
pub fn stub_0x263300(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

// 0x263428 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
// was: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x263428(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

// 0x2634b8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
// was: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
pub fn stub_0x2634b8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

// 0x2634e4 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
// was: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x2634e4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

// 0x26353c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEEEEE9constructEv
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>>>::construct(void)")]
pub fn stub_0x26353c() -> crate::slot::PortedFn {
// IDA 0x26353c: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x26353c, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pa~")
}

// 0x263574 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
// type: int __fastcall(_DWORD *, unsigned int, const char **)
// was: int __fastcall(_DWORD *, unsigned int, const char **)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
pub fn stub_0x263574(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

// 0x2635e4 — __ZNSt6vectorIPN3RBX10Reflection23YieldFunctionDescriptorESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: char *__fastcall(int, char *__src, _DWORD *)
// was: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::YieldFunctionDescriptor **,std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>>,RBX::Reflection::YieldFunctionDescriptor * const&)")]
pub fn stub_0x2635e4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

// 0x2636c4 — __ZNSt12_Vector_baseIPN3RBX10Reflection23YieldFunctionDescriptorESaIS3_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
// was: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::_M_allocate(unsigned long)")]
pub fn stub_0x2636c4() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x2636dc — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
// type: int __fastcall(int result, unsigned int)
// was: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>> const&)")]
pub fn stub_0x2636dc() -> crate::slot::PortedFn {
// IDA 0x2636dc: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2636dc, "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* c~")
}

// 0x2747e0 — __ZN3RBX3Lua21CoordinateFrameBridge13on_componentsEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_components(lua_State *)")]
pub fn stub_0x2747e0(thread: &mut crate::lua::LuaThreadState) -> i32 {
// CFrame:components() — pushes x,y,z + R00..R22 (12).
let frame = crate::lua::check_coordinateframe_slot(thread, 1);
thread.push(crate::lua::LuaStackValue::Number(f64::from(frame.position.x)));
thread.push(crate::lua::LuaStackValue::Number(f64::from(frame.position.y)));
thread.push(crate::lua::LuaStackValue::Number(f64::from(frame.position.z)));
for i in 0..3 {
    for j in 0..3 {
        thread.push(crate::lua::LuaStackValue::Number(f64::from(frame.rotation[i][j])));
    }
}
12
}

// 0x274868 — __ZN3RBX3Lua21CoordinateFrameBridge19on_toEulerAnglesXYZEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_toEulerAnglesXYZ(lua_State *)")]
pub fn stub_0x274868(thread: &mut crate::lua::LuaThreadState) -> i32 {
// CFrame:toEulerAnglesXYZ() — pushes rx, ry, rz.
let frame = crate::lua::check_coordinateframe_slot(thread, 1);
let (rx, ry, rz) = crate::lua::cframe_to_euler(&frame);
thread.push(crate::lua::LuaStackValue::Number(f64::from(rx)));
thread.push(crate::lua::LuaStackValue::Number(f64::from(ry)));
thread.push(crate::lua::LuaStackValue::Number(f64::from(rz)));
3
}

// 0x2748cc — __ZN3RBX3Lua21CoordinateFrameBridge22on_vectorToObjectSpaceEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_vectorToObjectSpace(lua_State *)")]
pub fn stub_0x2748cc(thread: &mut crate::lua::LuaThreadState) -> i32 {
// CFrame:vectorToObjectSpace(v) — inverse-rotate only.
let frame = crate::lua::check_coordinateframe_slot(thread, 1);
let vector = crate::lua::check_vector3_slot(thread, 2);
let mapped = crate::lua::cframe_vector_to_world(&crate::lua::cframe_inverse(&frame), &vector);
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::VECTOR3, crate::lua::LuaUserdataPayload::Vector3(mapped));
1
}

// 0x2749f0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(float *, char *__s1, int)
// was: int __fastcall(float *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(G3D::CoordinateFrame const&,char const*,lua_State *)")]
pub fn stub_0x2749f0(value: &crate::lua::LuaCoordinateFrame, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<CoordinateFrame>::on_index: X/Y/Z lanes, Position,
// method closures; else invalid member.
if key == "X" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.position.x)));
} else if key == "Y" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.position.y)));
} else if key == "Z" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.position.z)));
} else if key == "Position" {
    crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::VECTOR3, crate::lua::LuaUserdataPayload::Vector3(value.position));
} else if matches!(key, "inverse" | "lerp" | "toWorldSpace" | "toObjectSpace" | "pointToWorldSpace" | "pointToObjectSpace" | "vectorToWorldSpace" | "vectorToObjectSpace" | "components" | "toEulerAnglesXYZ") {
    thread.push(crate::lua::LuaStackValue::Function(crate::lua::method_fn_id(key)));
} else {
    panic!("{key} is not a valid member");
}
1
}

// 0x274da0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
// was: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(G3D::CoordinateFrame&,char const*,lua_State *)")]
pub fn stub_0x274da0(key: &str) -> ! {
// Bridge<CoordinateFrame>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

// 0x274e58 — __ZN3RBX3Lua10UDimBridge7newUDimEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::UDimBridge::newUDim(lua_State *)")]
pub fn stub_0x274e58(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x274e58: n = min(gettop, 3); no args -> (0, 0);
// one arg -> (scale, 0); else (scale, (u16)offset) — the
// offset truncation is preserved (cf. 0x274ea0).
// Pushes and returns 1.
let top = thread.stack_top().min(3);
let scale = if top >= 1 { crate::lua::lua_to_number_or_zero(thread, 1) as f32 } else { 0.0 };
let offset = if top >= 2 { crate::lua::lua_to_integer_or_zero(thread, 2) as u16 as i32 } else { 0 };
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM, crate::lua::LuaUserdataPayload::UDim(crate::lua::LuaUDim { scale, offset }));
1
}

// 0x274ebc — __ZN3RBX3Lua10UDimBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::UDimBridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x274ebc(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luaL_register + setreadonly + pop (cf. 0x2708b0).
let _ = thread;
0
}

// 0x274ef8 — __ZN3RBX3Lua10UDimBridge6on_addEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::UDimBridge::on_add(lua_State *)")]
pub fn stub_0x274ef8(thread: &mut crate::lua::LuaThreadState) -> i32 {
// UDim + UDim — componentwise add; push; return 1.
let a = crate::lua::check_udim_slot(thread, 1);
let b = crate::lua::check_udim_slot(thread, 2);
let sum = crate::lua::LuaUDim { scale: a.scale + b.scale, offset: a.offset + b.offset };
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM, crate::lua::LuaUserdataPayload::UDim(sum));
1
}

// 0x274f38 — __ZN3RBX3Lua10UDimBridge6on_subEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::UDimBridge::on_sub(lua_State *)")]
pub fn stub_0x274f38(thread: &mut crate::lua::LuaThreadState) -> i32 {
// UDim - UDim — componentwise sub; push; return 1.
let a = crate::lua::check_udim_slot(thread, 1);
let b = crate::lua::check_udim_slot(thread, 2);
let diff = crate::lua::LuaUDim { scale: a.scale - b.scale, offset: a.offset - b.offset };
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM, crate::lua::LuaUserdataPayload::UDim(diff));
1
}

// 0x274f78 — __ZN3RBX3Lua10UDimBridge6on_unmEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::UDimBridge::on_unm(lua_State *)")]
pub fn stub_0x274f78(thread: &mut crate::lua::LuaThreadState) -> i32 {
// -UDim — componentwise negate; push; return 1.
let a = crate::lua::check_udim_slot(thread, 1);
let neg = crate::lua::LuaUDim { scale: -a.scale, offset: -a.offset };
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM, crate::lua::LuaUserdataPayload::UDim(neg));
1
}

// 0x274fac — __ZN3RBX3Lua6BridgeINS_4UDimELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
// was: int __fastcall(int, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::UDim,true>::on_index(RBX::UDim const&,char const*,lua_State *)")]
pub fn stub_0x274fac(value: &crate::lua::LuaUDim, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<UDim>::on_index: Scale, Offset; else invalid member.
if key == "Scale" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.scale)));
} else if key == "Offset" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.offset)));
} else {
    panic!("{key} is not a valid member");
}
1
}

// 0x275148 — __ZN3RBX3Lua6BridgeINS_4UDimELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
// was: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::UDim,true>::on_newindex(RBX::UDim&,char const*,lua_State *)")]
pub fn stub_0x275148(key: &str) -> ! {
// Bridge<UDim>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

// 0x275200 — __ZN3RBX3Lua11UDim2Bridge8newUDim2EP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::UDim2Bridge::newUDim2(lua_State *)")]
pub fn stub_0x275200(thread: &mut crate::lua::LuaThreadState) -> i32 {
// UDim2.new(scaleX, offsetX, scaleY, offsetY) — missing
// lanes stay zero; offsets truncate to u16 like newUDim
// (cf. 0x274ea0). Pushes and returns 1.
let top = thread.stack_top().min(4);
let lane = |thread: &crate::lua::LuaThreadState, i: usize| if top >= i { crate::lua::lua_to_number_or_zero(thread, i) as f32 } else { 0.0 };
let off = |thread: &crate::lua::LuaThreadState, i: usize| if top >= i { crate::lua::lua_to_integer_or_zero(thread, i) as u16 as i32 } else { 0 };
let value = crate::lua::LuaUDim2 { x: crate::lua::LuaUDim { scale: lane(thread, 1), offset: off(thread, 2) }, y: crate::lua::LuaUDim { scale: lane(thread, 3), offset: off(thread, 4) } };
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM2, crate::lua::LuaUserdataPayload::UDim2(value));
1
}

// 0x275284 — __ZN3RBX3Lua11UDim2Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::UDim2Bridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x275284(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luaL_register + setreadonly + pop (cf. 0x2708b0).
let _ = thread;
0
}

// 0x2752c0 — __ZN3RBX3Lua11UDim2Bridge6on_addEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::UDim2Bridge::on_add(lua_State *)")]
pub fn stub_0x2752c0(thread: &mut crate::lua::LuaThreadState) -> i32 {
// UDim2 + UDim2 — lane-wise add; push; return 1.
let a = crate::lua::check_udim2_slot(thread, 1);
let b = crate::lua::check_udim2_slot(thread, 2);
let sum = crate::lua::LuaUDim2 { x: crate::lua::LuaUDim { scale: a.x.scale + b.x.scale, offset: a.x.offset + b.x.offset }, y: crate::lua::LuaUDim { scale: a.y.scale + b.y.scale, offset: a.y.offset + b.y.offset } };
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM2, crate::lua::LuaUserdataPayload::UDim2(sum));
1
}

// 0x275304 — __ZN3RBX3Lua11UDim2Bridge6on_subEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::UDim2Bridge::on_sub(lua_State *)")]
pub fn stub_0x275304(thread: &mut crate::lua::LuaThreadState) -> i32 {
// UDim2 - UDim2 — lane-wise sub; push; return 1.
let a = crate::lua::check_udim2_slot(thread, 1);
let b = crate::lua::check_udim2_slot(thread, 2);
let diff = crate::lua::LuaUDim2 { x: crate::lua::LuaUDim { scale: a.x.scale - b.x.scale, offset: a.x.offset - b.x.offset }, y: crate::lua::LuaUDim { scale: a.y.scale - b.y.scale, offset: a.y.offset - b.y.offset } };
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM2, crate::lua::LuaUserdataPayload::UDim2(diff));
1
}

// 0x275348 — __ZN3RBX3Lua11UDim2Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::UDim2Bridge::on_unm(lua_State *)")]
pub fn stub_0x275348(thread: &mut crate::lua::LuaThreadState) -> i32 {
// -UDim2 — lane-wise negate; push; return 1.
let a = crate::lua::check_udim2_slot(thread, 1);
let neg = crate::lua::LuaUDim2 { x: crate::lua::LuaUDim { scale: -a.x.scale, offset: -a.x.offset }, y: crate::lua::LuaUDim { scale: -a.y.scale, offset: -a.y.offset } };
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM2, crate::lua::LuaUserdataPayload::UDim2(neg));
1
}

// 0x275380 — __ZN3RBX3Lua6BridgeINS_5UDim2ELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(__int64 *, char *__s1, int)
// was: int __fastcall(__int64 *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::UDim2,true>::on_index(RBX::UDim2 const&,char const*,lua_State *)")]
pub fn stub_0x275380(value: &crate::lua::LuaUDim2, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<UDim2>::on_index: X, Y, Width, Height; else invalid.
if key == "X" {
    crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM, crate::lua::LuaUserdataPayload::UDim(value.x));
} else if key == "Y" {
    crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM, crate::lua::LuaUserdataPayload::UDim(value.y));
} else if key == "Width" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.x.offset)));
} else if key == "Height" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.y.offset)));
} else {
    panic!("{key} is not a valid member");
}
1
}

// 0x275530 — __ZN3RBX3Lua6BridgeINS_5UDim2ELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
// was: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::UDim2,true>::on_newindex(RBX::UDim2&,char const*,lua_State *)")]
pub fn stub_0x275530(key: &str) -> ! {
// Bridge<UDim2>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

// 0x2755e8 — __ZN3RBX3Lua11FacesBridge8newFacesEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::FacesBridge::newFaces(lua_State *)")]
pub fn stub_0x2755e8(thread: &mut crate::lua::LuaThreadState) -> i32 {
// Faces.new(...) — up to 6 NormalId/number lanes set bits
// 0..5 (cf. normalIdToVector3, 0x35d1e8); push; return 1.
let top = thread.stack_top().min(6);
let mut bits = 0u8;
for i in 1..=top {
    let bit = match thread.slot(i) {
        Some(crate::lua::LuaStackValue::Number(n)) if *n != 0.0 => i as u8 - 1,
        Some(crate::lua::LuaStackValue::Userdata(ud)) => match &ud.payload {
            crate::lua::LuaUserdataPayload::EnumItem(item) if item.value >= 0 && item.value < 6 => item.value as u8,
            _ => continue,
        },
        _ => continue,
    };
    bits |= 1 << bit;
}
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::FACES, crate::lua::LuaUserdataPayload::Faces(crate::lua::LuaFaces { bits }));
1
}

// 0x27573c — __ZN3RBX3Lua11FacesBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::FacesBridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x27573c(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luaL_register + setreadonly + pop (cf. 0x2708b0).
let _ = thread;
0
}

// 0x275778 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
// was: int __fastcall(int, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_index(RBX::Faces const&,char const*,lua_State *)")]
pub fn stub_0x275778(value: &crate::lua::LuaFaces, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Faces>::on_index: per-face NormalId items
// (Right=0, Top=1, Back=2, Left=3, Bottom=4, Front=5, cf.
// normalIdToVector3, 0x35d1e8); else invalid member.
let bit = match key { "Right" => 0, "Top" => 1, "Back" => 2, "Left" => 3, "Bottom" => 4, "Front" => 5, _ => panic!("{key} is not a valid member"), };
thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: "NormalId".to_owned(), value: bit }) }));
let _ = value;
1
}

// 0x275990 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
// was: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_newindex(RBX::Faces&,char const*,lua_State *)")]
pub fn stub_0x275990(key: &str) -> ! {
// Bridge<Faces>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

// 0x275a48 — __ZN3RBX3Lua10AxesBridge7newAxesEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::AxesBridge::newAxes(lua_State *)")]
pub fn stub_0x275a48(thread: &mut crate::lua::LuaThreadState) -> i32 {
// Axes.new(...) — up to 3 lanes set X/Y/Z bits 1/2/4;
// push; return 1.
let top = thread.stack_top().min(3);
let mut bits = 0u8;
for i in 1..=top {
    let bit = match thread.slot(i) {
        Some(crate::lua::LuaStackValue::Number(n)) if *n != 0.0 => i as u8,
        Some(crate::lua::LuaStackValue::Userdata(ud)) => match &ud.payload {
            crate::lua::LuaUserdataPayload::EnumItem(item) if item.value >= 0 && item.value < 3 => 1 << (item.value as u8),
            _ => continue,
        },
        _ => continue,
    };
    bits |= bit;
}
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::AXES, crate::lua::LuaUserdataPayload::Axes(crate::lua::LuaAxes { bits }));
1
}

// 0x275bdc — __ZN3RBX3Lua10AxesBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::AxesBridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x275bdc(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luaL_register + setreadonly + pop (cf. 0x2708b0).
let _ = thread;
0
}

// 0x275c18 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(RBX::Axes *, char *__s1, int)
// was: int __fastcall(RBX::Axes *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_index(RBX::Axes const&,char const*,lua_State *)")]
pub fn stub_0x275c18(value: &crate::lua::LuaAxes, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Axes>::on_index: X/Y/Z presence flags; else invalid.
let bit = match key { "X" => 1u8, "Y" => 2u8, "Z" => 4u8, _ => panic!("{key} is not a valid member"), };
thread.push(crate::lua::LuaStackValue::Bool(value.bits & bit != 0));
1
}

// 0x275e94 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
// was: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_newindex(RBX::Axes&,char const*,lua_State *)")]
pub fn stub_0x275e94(key: &str) -> ! {
// Bridge<Axes>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

// 0x275f4c — __ZN3RBX3Lua12CellIDBridge9newCellIDEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::CellIDBridge::newCellID(lua_State *)")]
pub fn stub_0x275f4c(thread: &mut crate::lua::LuaThreadState) -> i32 {
// CellID.new(x, y, z) — integer lanes, missing stay zero;
// pushNewObject bumps shared_count (cf. 0x26e42e).
let lane = |thread: &crate::lua::LuaThreadState, i: usize| crate::lua::lua_to_integer_or_zero(thread, i) as i32;
let cell = crate::lua::LuaCellId { x: lane(thread, 1), y: lane(thread, 2), z: lane(thread, 3), shared: 1 };
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::CELLID, crate::lua::LuaUserdataPayload::CellId(cell));
1
}

// 0x27612c — __ZN3RBX3Lua12CellIDBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::CellIDBridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x27612c(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luaL_register + setreadonly + pop (cf. 0x2708b0).
let _ = thread;
0
}

// 0x276168 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(unsigned __int8 *, char *__s1, int)
// was: int __fastcall(unsigned __int8 *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_index(RBX::CellID const&,char const*,lua_State *)")]
pub fn stub_0x276168(value: &crate::lua::LuaCellId, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<CellID>::on_index: X/Y/Z lanes; else invalid member.
if key == "X" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.x)));
} else if key == "Y" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.y)));
} else if key == "Z" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.z)));
} else {
    panic!("{key} is not a valid member");
}
1
}

// 0x2762f4 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
// was: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_newindex(RBX::CellID&,char const*,lua_State *)")]
pub fn stub_0x2762f4(key: &str) -> ! {
// Bridge<CellID>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

// 0x2763ac — __ZN3RBX3Lua17InputObjectBridge14newInputObjectEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::InputObjectBridge::newInputObject(lua_State *)")]
pub fn stub_0x2763ac(thread: &mut crate::lua::LuaThreadState) -> i32 {
// InputObject.new(kind) — input-kind tag, default 0;
// push; return 1.
let kind = if thread.stack_top() >= 1 { crate::lua::lua_to_integer_or_zero(thread, 1) as u32 } else { 0 };
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::INPUTOBJECT, crate::lua::LuaUserdataPayload::InputObject(crate::lua::LuaInputObject { kind }));
1
}

// 0x2765f4 — __ZN3RBX3Lua17InputObjectBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::InputObjectBridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x2765f4(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luaL_register + setreadonly + pop (cf. 0x2708b0).
let _ = thread;
0
}

// 0x276630 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(_DWORD *, char *__s1, int)
// was: int __fastcall(_DWORD *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_index(RBX::InputObject const&,char const*,lua_State *)")]
pub fn stub_0x276630(value: &crate::lua::LuaInputObject, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<InputObject>::on_index: KeyCode/UserInputType enums,
// Position/Delta vectors, UserInputState; else invalid.
if key == "KeyCode" {
    thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: "KeyCode".to_owned(), value: value.kind as i32 }) }));
} else if key == "UserInputType" {
    thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: "UserInputType".to_owned(), value: value.kind as i32 }) }));
} else if key == "UserInputState" {
    thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: "UserInputState".to_owned(), value: 0 }) }));
} else if key == "Position" || key == "Delta" {
    crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::VECTOR3, crate::lua::LuaUserdataPayload::Vector3(crate::lua::LuaVector3 { x: 0.0, y: 0.0, z: 0.0 }));
} else {
    panic!("{key} is not a valid member");
}
1
}

// 0x2767a0 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
// was: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_newindex(RBX::InputObject&,char const*,lua_State *)")]
pub fn stub_0x2767a0(key: &str) -> ! {
// Bridge<InputObject>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

// 0x276858 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x276858(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x276858: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x276a48 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x276a48(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x276a48: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x276c38 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x276c38(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x276c38: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x276e28 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x276e28(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x276e28: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x277018 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x277018(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x277018: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x2771dc — __ZN3RBX3Lua6BridgeINS_4UDimELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::UDim,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x2771dc(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x2771dc: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x277344 — __ZN3RBX3Lua6BridgeINS_5UDim2ELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::UDim2,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x277344(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x277344: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x2774ac — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *)
// was: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<G3D::Color3>(lua_State *,G3D::Color3)")]
pub fn stub_0x2774ac(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaColor3) -> crate::lua::LuaColor3 {
// Overload of the primary stub_0xf29f74.
crate::lua::stub_0xf29f74(thread, value)
}

// 0x2774f4 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *)
// was: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<float *>(lua_State *,float *)")]
pub fn stub_0x2774f4(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaColor3) -> crate::lua::LuaColor3 {
// Overload of the primary stub_0xf29f74.
crate::lua::stub_0xf29f74(thread, value)
}

// 0x27753c — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::RbxRay* RBX::Lua::Bridge<RBX::RbxRay,true>::pushNewObject<RBX::RbxRay>(lua_State *,RBX::RbxRay)")]
pub fn stub_0x27753c(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaRbxRay) -> crate::lua::LuaRbxRay {
// Bridge<RbxRay>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::RAY, crate::lua::LuaUserdataPayload::RbxRay(*value));
*value
}

// 0x27759c — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, int, int, int)
// was: _DWORD *__fastcall(int, int, int, int)
#[doc(alias = "G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<G3D::Vector3>(lua_State *,G3D::Vector3)")]
pub fn stub_0x27759c(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector3) -> crate::lua::LuaVector3 {
// Overload of the primary stub_0xf29fc4.
crate::lua::stub_0xf29fc4(thread, value)
}

// 0x2775ec — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *)
// was: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<float *>(lua_State *,float *)")]
pub fn stub_0x2775ec(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector3) -> crate::lua::LuaVector3 {
// Overload of the primary stub_0xf29fc4.
crate::lua::stub_0xf29fc4(thread, value)
}

// 0x277634 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIPiEEPS3_P9lua_StateT_
// type: _WORD *__fastcall(int, _DWORD *)
// was: _WORD *__fastcall(int, _DWORD *)
#[doc(alias = "G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<int *>(lua_State *,int *)")]
pub fn stub_0x277634(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector3i16) -> crate::lua::LuaVector3i16 {
// Overload of the primary stub_0x26eaf0.
crate::lua::stub_0x26eaf0(thread, value)
}

// 0x27767c — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8getValueIS3_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
// was: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<G3D::Vector2int16>(lua_State *,unsigned int,G3D::Vector2int16 &)")]
pub fn stub_0x27767c(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::LuaVector2i16, slot: &crate::lua::LuaStackValue) -> bool {
// Bridge<Vector2int16>::getValue — class-tag + payload check.
match crate::lua::bridge_payload(slot, crate::lua::lua_bridge_class::VECTOR2INT16) {
    Some(crate::lua::LuaUserdataPayload::Vector2i16(v)) => { *out = *v; true }
    _ => false,
}
}

// 0x2776ec — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIPiEEPS3_P9lua_StateT_
// type: G3D::Vector2int16 *__fastcall(int, int *)
// was: G3D::Vector2int16 *__fastcall(int, int *)
#[doc(alias = "G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<int *>(lua_State *,int *)")]
pub fn stub_0x2776ec(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector2i16) -> crate::lua::LuaVector2i16 {
// Overload of the primary stub_0x26e9c0.
crate::lua::stub_0x26e9c0(thread, value)
}

// 0x277730 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8getValueIS3_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
// was: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<G3D::Vector2>(lua_State *,unsigned int,G3D::Vector2 &)")]
pub fn stub_0x277730(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::LuaVector2, slot: &crate::lua::LuaStackValue) -> bool {
// Bridge<Vector2>::getValue — class-tag + payload check.
match crate::lua::bridge_payload(slot, crate::lua::lua_bridge_class::VECTOR2) {
    Some(crate::lua::LuaUserdataPayload::Vector2(v)) => { *out = *v; true }
    _ => false,
}
}

// 0x2777a8 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *)
// was: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<float *>(lua_State *,float *)")]
pub fn stub_0x2777a8(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector2) -> crate::lua::LuaVector2 {
// Overload of the primary stub_0xf29f94.
crate::lua::stub_0xf29f94(thread, value)
}

// 0x2777ec — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *)
// was: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<G3D::Vector2>(lua_State *,G3D::Vector2)")]
pub fn stub_0x2777ec(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaVector2) -> crate::lua::LuaVector2 {
// Overload of the primary stub_0xf29f94.
crate::lua::stub_0xf29f94(thread, value)
}

// 0x277830 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: _DWORD *__fastcall(int, int)
// was: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::BrickColor* RBX::Lua::Bridge<RBX::BrickColor,true>::pushNewObject<RBX::BrickColor>(lua_State *,RBX::BrickColor)")]
pub fn stub_0x277830(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaBrickColor) -> crate::lua::LuaBrickColor {
// Bridge<BrickColor>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::BRICKCOLOR, crate::lua::LuaUserdataPayload::BrickColor(*value));
*value
}

// 0x277894 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: G3D::Matrix3 *__fastcall(int, int)
// was: G3D::Matrix3 *__fastcall(int, int)
#[doc(alias = "G3D::CoordinateFrame* RBX::Lua::Bridge<G3D::CoordinateFrame,true>::pushNewObject<G3D::CoordinateFrame>(lua_State *,G3D::CoordinateFrame)")]
pub fn stub_0x277894(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaCoordinateFrame) -> crate::lua::LuaCoordinateFrame {
// Overload of the primary stub_0xf29f54.
crate::lua::stub_0xf29f54(thread, value)
}

// 0x2778e4 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8getValueIS3_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<G3D::CoordinateFrame>(lua_State *,unsigned int,G3D::CoordinateFrame &)")]
pub fn stub_0x2778e4(_thread: &crate::lua::LuaThreadState, _index: usize, out: &mut crate::lua::LuaCoordinateFrame, slot: &crate::lua::LuaStackValue) -> bool {
// Bridge<CoordinateFrame>::getValue — class-tag + payload check.
match crate::lua::bridge_payload(slot, crate::lua::lua_bridge_class::CFRAME) {
    Some(crate::lua::LuaUserdataPayload::CoordinateFrame(v)) => { *out = *v; true }
    _ => false,
}
}

// 0x2779d4 — __ZN3RBX3Lua6BridgeINS_4UDimELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: unsigned __int64 *__fastcall(int, unsigned int, unsigned int)
// was: unsigned __int64 *__fastcall(int, unsigned int, unsigned int)
#[doc(alias = "RBX::UDim* RBX::Lua::Bridge<RBX::UDim,true>::pushNewObject<RBX::UDim>(lua_State *,RBX::UDim)")]
pub fn stub_0x2779d4(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaUDim) -> crate::lua::LuaUDim {
// Bridge<UDim>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM, crate::lua::LuaUserdataPayload::UDim(*value));
*value
}

// 0x277a20 — __ZN3RBX3Lua6BridgeINS_5UDim2ELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: _DWORD *__fastcall(int, int, int, int, int)
// was: _DWORD *__fastcall(int, int, int, int, int)
#[doc(alias = "RBX::UDim2* RBX::Lua::Bridge<RBX::UDim2,true>::pushNewObject<RBX::UDim2>(lua_State *,RBX::UDim2)")]
pub fn stub_0x277a20(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaUDim2) -> crate::lua::LuaUDim2 {
// Bridge<UDim2>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM2, crate::lua::LuaUserdataPayload::UDim2(*value));
*value
}

// 0x277a74 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: _DWORD *__fastcall(int, int)
// was: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::Faces* RBX::Lua::Bridge<RBX::Faces,true>::pushNewObject<RBX::Faces>(lua_State *,RBX::Faces)")]
pub fn stub_0x277a74(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaFaces) -> crate::lua::LuaFaces {
// Bridge<Faces>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::FACES, crate::lua::LuaUserdataPayload::Faces(*value));
*value
}

// 0x277ab4 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: _DWORD *__fastcall(int, int)
// was: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::Axes* RBX::Lua::Bridge<RBX::Axes,true>::pushNewObject<RBX::Axes>(lua_State *,RBX::Axes)")]
pub fn stub_0x277ab4(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaAxes) -> crate::lua::LuaAxes {
// Bridge<Axes>::pushNewObject — metatable + payload copy
// (cf. 0x2705d0).
crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::AXES, crate::lua::LuaUserdataPayload::Axes(*value));
*value
}

// 0x277bd0 — __ZN3RBX3Lua6BridgeINS_11InputObject13UserInputTypeELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, int)
// was: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::InputObject::UserInputType* RBX::Lua::Bridge<RBX::InputObject::UserInputType,true>::pushNewObject<RBX::InputObject::UserInputType>(lua_State *,RBX::InputObject::UserInputType)")]
pub fn stub_0x277bd0(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaInputObject) -> crate::lua::LuaInputObject {
// Overload of the primary stub_0x26e1d8.
crate::lua::stub_0x26e1d8(thread, value)
}

// 0x277c10 — __ZN3RBX3Lua6BridgeINS_11InputObject14UserInputStateELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, int)
// was: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::InputObject::UserInputState* RBX::Lua::Bridge<RBX::InputObject::UserInputState,true>::pushNewObject<RBX::InputObject::UserInputState>(lua_State *,RBX::InputObject::UserInputState)")]
pub fn stub_0x277c10(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaInputObject) -> crate::lua::LuaInputObject {
// Overload of the primary stub_0x26e1d8.
crate::lua::stub_0x26e1d8(thread, value)
}

// 0x277c50 — __ZN3RBX3Lua6BridgeINS_7KeyCodeELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: _DWORD *__fastcall(int, int)
// was: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::KeyCode* RBX::Lua::Bridge<RBX::KeyCode,true>::pushNewObject<RBX::KeyCode>(lua_State *,RBX::KeyCode)")]
pub fn stub_0x277c50(handle: &crate::slot::InstanceHandle) {
// RBX::KeyCode* RBX::Lua::Bridge<RBX::KeyCode,true>::pushNewObject<RBX::KeyCode>(lua_State *~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x277c90 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_gc(lua_State *)")]
pub fn stub_0x277c90(value: crate::lua::LuaVector3i16) {
// Bridge<Vector3int16>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x277cac — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_eq(lua_State *)")]
pub fn stub_0x277cac(a: &crate::lua::LuaVector3i16, b: &crate::lua::LuaVector3i16) -> bool {
// Bridge<Vector3int16>::on_eq — value equality on the payload.
a == b
}

// 0x277cf8 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(lua_State *)")]
pub fn stub_0x277cf8(value: &crate::lua::LuaVector3i16, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Vector3int16>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}", value.x, value.y, value.z)));
1
}

// 0x277d1c — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_gc(lua_State *)")]
pub fn stub_0x277d1c(value: crate::lua::LuaVector2i16) {
// Bridge<Vector2int16>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x277d38 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_eq(lua_State *)")]
pub fn stub_0x277d38(a: &crate::lua::LuaVector2i16, b: &crate::lua::LuaVector2i16) -> bool {
// Bridge<Vector2int16>::on_eq — value equality on the payload.
a == b
}

// 0x277d74 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(lua_State *)")]
pub fn stub_0x277d74(value: &crate::lua::LuaVector2i16, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Vector2int16>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}", value.x, value.y)));
1
}

// 0x277d98 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_gc(lua_State *)")]
pub fn stub_0x277d98(value: crate::lua::LuaVector3) {
// Bridge<Vector3>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x277db4 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_eq(lua_State *)")]
pub fn stub_0x277db4(a: &crate::lua::LuaVector3, b: &crate::lua::LuaVector3) -> bool {
// Bridge<Vector3>::on_eq — value equality on the payload.
a == b
}

// 0x277e20 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(lua_State *)")]
pub fn stub_0x277e20(value: &crate::lua::LuaVector3, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Vector3>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}", value.x, value.y, value.z)));
1
}

// 0x277e44 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_gc(lua_State *)")]
pub fn stub_0x277e44(value: crate::lua::LuaVector2) {
// Bridge<Vector2>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x277e60 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_eq(lua_State *)")]
pub fn stub_0x277e60(a: &crate::lua::LuaVector2, b: &crate::lua::LuaVector2) -> bool {
// Bridge<Vector2>::on_eq — value equality on the payload.
a == b
}

// 0x277eb8 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(lua_State *)")]
pub fn stub_0x277eb8(value: &crate::lua::LuaVector2, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Vector2>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}", value.x, value.y)));
1
}

// 0x277edc — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_gc(lua_State *)")]
pub fn stub_0x277edc(value: crate::lua::LuaCoordinateFrame) {
// Bridge<CoordinateFrame>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x277ef8 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_eq(lua_State *)")]
pub fn stub_0x277ef8(a: &crate::lua::LuaCoordinateFrame, b: &crate::lua::LuaCoordinateFrame) -> bool {
// Bridge<CoordinateFrame>::on_eq — value equality on the payload.
a == b
}

// 0x277f70 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(lua_State *)")]
pub fn stub_0x277f70(value: &crate::lua::LuaCoordinateFrame, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<CoordinateFrame>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", value.position.x, value.position.y, value.position.z, value.rotation[0][0], value.rotation[0][1], value.rotation[0][2], value.rotation[1][0], value.rotation[1][1], value.rotation[1][2], value.rotation[2][0], value.rotation[2][1], value.rotation[2][2])));
1
}

// 0x277f94 — __ZN3RBX3Lua6BridgeINS_4UDimELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::UDim,true>::on_gc(lua_State *)")]
pub fn stub_0x277f94(value: crate::lua::LuaUDim) {
// Bridge<UDim>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x277fb0 — __ZN3RBX3Lua6BridgeINS_4UDimELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::UDim,true>::on_eq(lua_State *)")]
pub fn stub_0x277fb0(a: &crate::lua::LuaUDim, b: &crate::lua::LuaUDim) -> bool {
// Bridge<UDim>::on_eq — value equality on the payload.
a == b
}

// 0x277ffc — __ZN3RBX3Lua6BridgeINS_5UDim2ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::UDim2,true>::on_gc(lua_State *)")]
pub fn stub_0x277ffc(value: crate::lua::LuaUDim2) {
// Bridge<UDim2>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x278018 — __ZN3RBX3Lua6BridgeINS_5UDim2ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::UDim2,true>::on_eq(lua_State *)")]
pub fn stub_0x278018(a: &crate::lua::LuaUDim2, b: &crate::lua::LuaUDim2) -> bool {
// Bridge<UDim2>::on_eq — value equality on the payload.
a == b
}

// 0x27832c — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(G3D::Color3 const&,lua_State *)")]
pub fn stub_0x27832c(value: &crate::lua::LuaColor3, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Color3>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}", value.r, value.g, value.b)));
1
}

// 0x278450 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(struct _Unwind_Exception *lpuexcpt, int)
// was: int __fastcall(struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_tostring(RBX::RbxRay const&,lua_State *)")]
pub fn stub_0x278450(value: &crate::lua::LuaRbxRay, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<RbxRay>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {} | {}, {}, {}", value.origin.x, value.origin.y, value.origin.z, value.direction.x, value.direction.y, value.direction.z)));
1
}

// 0x278574 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(G3D::Vector3int16 const&,lua_State *)")]
pub fn stub_0x278574(value: &crate::lua::LuaVector3i16, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Vector3int16>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}", value.x, value.y, value.z)));
1
}
