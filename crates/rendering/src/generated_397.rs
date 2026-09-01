//! rendering shard 397 — 100 stubs 0x5e60ec..0x5ea74c EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 42910->43010 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x5e60ec..0x5ea74c (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5e60ec — __ZN3RBX11shared_fromINS_14FWPartInstanceEEEN5boost10shared_ptrIT_EEPS4_
// type: int(void)
#[doc(alias = "__ZN3RBX11shared_fromINS_14FWPartInstanceEEEN5boost10shared_ptrIT_EEPS4_")]
#[doc(alias = "boost::shared_ptr<RBX::FWPartInstance> RBX::shared_from<RBX::FWPartInstance>(RBX::FWPartInstance*)")]
// was: __ZN3RBX11shared_fromINS_14FWPartInstanceEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_5e60ec() -> ! {
    todo!("0x5e60ec boost::shared_ptr<RBX::FWPartInstance> RBX::shared_from<RBX::FWPartInstance>(RBX::FWPartInstance*)")
}


// 0x5e6254 — __ZN5boost10shared_ptrIN3RBX9IFWHolderEEaSERKS3_
// type: int(void)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9IFWHolderEEaSERKS3_")]
#[doc(alias = "boost::shared_ptr<RBX::IFWHolder>::operator=(boost::shared_ptr<RBX::IFWHolder> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX9IFWHolderEEaSERKS3_
pub fn stub_5e6254() -> ! {
    todo!("0x5e6254 boost::shared_ptr<RBX::IFWHolder>::operator=(boost::shared_ptr<RBX::IFWHolder> const&)")
}


// 0x5e6290 — __ZN5boost12bad_weak_ptrD1Ev
// type: void __fastcall(boost::bad_weak_ptr *__hidden this)
#[doc(alias = "__ZN5boost12bad_weak_ptrD1Ev")]
#[doc(alias = "boost::bad_weak_ptr::~bad_weak_ptr()")]
// was: __ZN5boost12bad_weak_ptrD1Ev
pub fn stub_5e6290() -> ! {
    todo!("0x5e6290 boost::bad_weak_ptr::~bad_weak_ptr()")
}


// 0x5e6298 — __ZNK5boost12bad_weak_ptr4whatEv
// type: _DWORD __fastcall(boost::bad_weak_ptr *__hidden this)
#[doc(alias = "__ZNK5boost12bad_weak_ptr4whatEv")]
#[doc(alias = "boost::bad_weak_ptr::what(void)const")]
// was: __ZNK5boost12bad_weak_ptr4whatEv
pub fn stub_5e6298() -> ! {
    todo!("0x5e6298 boost::bad_weak_ptr::what(void)const")
}


// 0x5e62a8 — __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED2Ev")]
#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED2Ev
pub fn stub_5e62a8() -> ! {
    todo!("0x5e62a8 boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")
}


// 0x5e6360 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev")]
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
// was: __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev
pub fn stub_5e6360() -> ! {
    todo!("0x5e6360 `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")
}


// 0x5e6370 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE5cloneEv
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE5cloneEv")]
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone(void)const")]
// was: __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE5cloneEv
pub fn stub_5e6370() -> ! {
    todo!("0x5e6370 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone(void)const")
}


// 0x5e6430 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEEC1ERKS4_
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEEC1ERKS4_")]
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_impl(boost::exception_detail::error_info_injector<boost::bad_weak_ptr> const&)")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEEC1ERKS4_
pub fn stub_5e6430() -> ! {
    todo!("0x5e6430 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_impl(boost::exception_detail::error_info_injector<boost::bad_weak_ptr> const&)")
}


// 0x5e6568 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_")]
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartInstance>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::DumbPtr<RBX::FWPartInstance>>>(RBX::DumbPtr<RBX::FWPartInstance> const&,boost::unordered::detail::emplace_args1<RBX::DumbPtr<RBX::FWPartInstance>> const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
pub fn stub_5e6568() -> ! {
    todo!("0x5e6568 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartInstance>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::DumbPtr<RBX::FWPartInstance>>>(RBX::DumbPtr<RBX::FWPartInstance> const&,boost::unordered::detail::emplace_args1<RBX::DumbPtr<RBX::FWPartInstance>> const&)")
}


// 0x5e6718 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm")]
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
pub fn stub_5e6718() -> ! {
    todo!("0x5e6718 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::create_buckets(unsigned long)")
}


// 0x5e6840 — __ZNK5boost9unordered6detail5tableINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
// type: int(void)
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm")]
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
pub fn stub_5e6840() -> ! {
    todo!("0x5e6840 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::min_buckets_for_size(unsigned long)const")
}


// 0x5e68d0 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
// type: int(void)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm")]
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
pub fn stub_5e68d0() -> ! {
    todo!("0x5e68d0 boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::rehash_impl(unsigned long)")
}


// 0x5e68fc — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE
// type: int(void)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE")]
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE
pub fn stub_5e68fc() -> ! {
    todo!("0x5e68fc boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>> &,boost::unordered::detail::ptr_bucket *)")
}


// 0x5e6950 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIN3RBX7DumbPtrINS4_14FWPartInstanceEEEEEEE9constructEv
// type: int(void)
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIN3RBX7DumbPtrINS4_14FWPartInstanceEEEEEEE9constructEv")]
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartInstance>>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIN3RBX7DumbPtrINS4_14FWPartInstanceEEEEEEE9constructEv
pub fn stub_5e6950() -> ! {
    todo!("0x5e6950 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartInstance>>>>::construct(void)")
}


// 0x5e6988 — __ZNK5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_
// type: int(void)
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_")]
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartInstance>>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::find_node_impl<RBX::DumbPtr<RBX::FWPartInstance>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>(unsigned long,RBX::DumbPtr<RBX::FWPartInstance> const&,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>> const&)const")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3setISaIN3RBX7DumbPtrINS4_14FWPartInstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_
pub fn stub_5e6988() -> ! {
    todo!("0x5e6988 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::DumbPtr<RBX::FWPartInstance>>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::DumbPtr<RBX::FWPartInstance>>,RBX::DumbPtr<RBX::FWPartInstance>,boost::hash<RBX::DumbPtr<RBX::FWPartInstance>>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>>::find_node_impl<RBX::DumbPtr<RBX::FWPartInstance>,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>>>(unsigned long,RBX::DumbPtr<RBX::FWPartInstance> const&,std::equal_to<RBX::DumbPtr<RBX::FWPartInstance>> const&)const")
}


// 0x5e6a00 — __ZNK3RBX15ServiceProvider4findINS_9FWServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_9FWServiceEEEPT_v")]
#[doc(alias = "RBX::FWService * RBX::ServiceProvider::find<RBX::FWService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_9FWServiceEEEPT_v
pub fn stub_5e6a00() -> ! {
    todo!("0x5e6a00 RBX::FWService * RBX::ServiceProvider::find<RBX::FWService>(void)const")
}


// 0x5e6b78 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEE15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEE15isNullClassNameEv
pub fn stub_5e6b78() -> ! {
    todo!("0x5e6b78 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEE15isNullClassNameEv")
}


// 0x5e6c18 — __ZN3RBX4Name7declareILZNS_10sFWServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sFWServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_10sFWServiceEEEERKS0_v
pub fn stub_5e6c18() -> ! {
    todo!("0x5e6c18 __ZN3RBX4Name7declareILZNS_10sFWServiceEEEERKS0_v")
}


// 0x5e6c60 — __ZN3RBX4Name9doDeclareILZNS_10sFWServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sFWServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sFWServiceEEEERKS0_v
pub fn stub_5e6c60() -> ! {
    todo!("0x5e6c60 __ZN3RBX4Name9doDeclareILZNS_10sFWServiceEEEERKS0_v")
}


// 0x5e6d44 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_9FWServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_9FWServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FWService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_9FWServiceEEEvv
pub fn stub_5e6d44() -> ! {
    todo!("0x5e6d44 void RBX::ServiceProvider::callDoGetClassIndex<RBX::FWService>(void)")
}


// 0x5e6d48 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_9FWServiceEEEmv
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_9FWServiceEEEmv")]
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FWService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_9FWServiceEEEmv
pub fn stub_5e6d48() -> ! {
    todo!("0x5e6d48 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FWService>(void)")
}


// 0x5e6e20 — __ZN3RBX11ComputePropINS_4PartENS_12PartInstance20OnDemandPartInstanceEE8getValueEv
// type: int(void)
#[doc(alias = "__ZN3RBX11ComputePropINS_4PartENS_12PartInstance20OnDemandPartInstanceEE8getValueEv")]
#[doc(alias = "RBX::ComputeProp<RBX::Part,RBX::PartInstance::OnDemandPartInstance>::getValue(void)")]
// was: __ZN3RBX11ComputePropINS_4PartENS_12PartInstance20OnDemandPartInstanceEE8getValueEv
pub fn stub_5e6e20() -> ! {
    todo!("0x5e6e20 RBX::ComputeProp<RBX::Part,RBX::PartInstance::OnDemandPartInstance>::getValue(void)")
}


// 0x5e6f20 — __ZN3rbx7signals6signalIFvbEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(bool)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvbEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_5e6f20() -> ! {
    todo!("0x5e6f20 rbx::signals::signal<void ()(bool)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot> &)")
}


// 0x5e7080 — __ZN3rbx7signals6signalIFvbEE8on_errorERSt9exception
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(bool)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvbEE8on_errorERSt9exception
pub fn stub_5e7080() -> ! {
    todo!("0x5e7080 rbx::signals::signal<void ()(bool)>::on_error(std::exception &)")
}


// 0x5e70a8 — __ZN5boost21intrusive_ptr_releaseIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost21intrusive_ptr_releaseIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE")]
#[doc(alias = "void boost::intrusive_ptr_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
// was: __ZN5boost21intrusive_ptr_releaseIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
pub fn stub_5e70a8() -> ! {
    todo!("0x5e70a8 void boost::intrusive_ptr_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")
}


// 0x5e70e8 — __ZNK5boost9function1IvRSt9exceptionEclES2_
// type: int(void)
#[doc(alias = "__ZNK5boost9function1IvRSt9exceptionEclES2_")]
#[doc(alias = "boost::function1<void,std::exception &>::operator()(std::exception &)const")]
// was: __ZNK5boost9function1IvRSt9exceptionEclES2_
pub fn stub_5e70e8() -> ! {
    todo!("0x5e70e8 boost::function1<void,std::exception &>::operator()(std::exception &)const")
}


// 0x5e71b0 — __ZN5boost17bad_function_callD0Ev
// type: void __fastcall(boost::bad_function_call *__hidden this)
#[doc(alias = "__ZN5boost17bad_function_callD0Ev")]
#[doc(alias = "boost::bad_function_call::~bad_function_call()")]
// was: __ZN5boost17bad_function_callD0Ev
pub fn stub_5e71b0() -> ! {
    todo!("0x5e71b0 boost::bad_function_call::~bad_function_call()")
}


// 0x5e71c8 — __ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED2Ev")]
#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED2Ev
pub fn stub_5e71c8() -> ! {
    todo!("0x5e71c8 boost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")
}


// 0x5e7280 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_17bad_function_callEED1Ev
#[doc(alias = "__ZThn8_N5boost16exception_detail19error_info_injectorINS_17bad_function_callEED1Ev")]
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")]
// was: __ZThn8_N5boost16exception_detail19error_info_injectorINS_17bad_function_callEED1Ev
pub fn stub_5e7280() -> ! {
    todo!("0x5e7280 `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")
}


// 0x5e7288 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED1Ev
#[doc(alias = "__ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED1Ev")]
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()")]
// was: __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED1Ev
pub fn stub_5e7288() -> ! {
    todo!("0x5e7288 `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()")
}


// 0x5e7290 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED1Ev
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED1Ev")]
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()")]
// was: __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEED1Ev
pub fn stub_5e7290() -> ! {
    todo!("0x5e7290 `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()")
}


// 0x5e72a0 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE5cloneEv
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE5cloneEv")]
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone(void)const")]
// was: __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE5cloneEv
pub fn stub_5e72a0() -> ! {
    todo!("0x5e72a0 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone(void)const")
}


// 0x5e7360 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE5cloneEv
#[doc(alias = "__ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE5cloneEv")]
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone(void)const")]
// was: __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE5cloneEv
pub fn stub_5e7360() -> ! {
    todo!("0x5e7360 `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone(void)const")
}


// 0x5e7370 — __ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED0Ev
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED0Ev")]
#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED0Ev
pub fn stub_5e7370() -> ! {
    todo!("0x5e7370 boost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")
}


// 0x5e7388 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbEE4slotEEaSERKS7_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbEE4slotEEaSERKS7_")]
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbEE4slotEEaSERKS7_
pub fn stub_5e7388() -> ! {
    todo!("0x5e7388 boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool)>::slot> const&)")
}


// 0x5e73b0 — __ZN3rbx7signals6signalIFvbEE24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(bool)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvbEE24safe_static_do_get_mutexEv
pub fn stub_5e73b0() -> ! {
    todo!("0x5e73b0 rbx::signals::signal<void ()(bool)>::safe_static_do_get_mutex(void)")
}


// 0x5e74a8 — __ZN5boost11unique_lockINS_5mutexEE4lockEv
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost11unique_lockINS_5mutexEE4lockEv")]
#[doc(alias = "boost::unique_lock<boost::mutex>::lock(void)")]
// was: __ZN5boost11unique_lockINS_5mutexEE4lockEv
pub fn stub_5e74a8() -> ! {
    todo!("0x5e74a8 boost::unique_lock<boost::mutex>::lock(void)")
}


// 0x5e75dc — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_")]
#[doc(alias = "std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::weak_ptr<RBX::PartInstance>*,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>>,boost::weak_ptr<RBX::PartInstance> const&)")]
// was: __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
pub fn stub_5e75dc() -> ! {
    todo!("0x5e75dc std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::weak_ptr<RBX::PartInstance>*,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>>,boost::weak_ptr<RBX::PartInstance> const&)")
}


// 0x5e7b24 — __ZNSt12_Vector_baseIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "__ZNSt12_Vector_baseIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE11_M_allocateEm")]
#[doc(alias = "std::_Vector_base<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE11_M_allocateEm
pub fn stub_5e7b24() -> ! {
    todo!("0x5e7b24 std::_Vector_base<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::_M_allocate(unsigned long)")
}


// 0x5e7b40 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost8weak_ptrIN3RBX12PartInstanceEEES8_EET0_T_SA_S9_
// type: int(void)
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost8weak_ptrIN3RBX12PartInstanceEEES8_EET0_T_SA_S9_")]
#[doc(alias = "boost::weak_ptr<RBX::PartInstance> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *>(boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost8weak_ptrIN3RBX12PartInstanceEEES8_EET0_T_SA_S9_
pub fn stub_5e7b40() -> ! {
    todo!("0x5e7b40 boost::weak_ptr<RBX::PartInstance> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *>(boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *,boost::weak_ptr<RBX::PartInstance> *)")
}


// 0x5e7b98 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_")]
#[doc(alias = "std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::PartInstance>*,std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>>,boost::shared_ptr<RBX::PartInstance> const&)")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
pub fn stub_5e7b98() -> ! {
    todo!("0x5e7b98 std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::PartInstance>*,std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>>,boost::shared_ptr<RBX::PartInstance> const&)")
}


// 0x5e7f64 — __ZN3RBX10Reflection9DescribedINS_16TouchTransmitterELZNS_17sTouchTransmitterEENS_14FactoryProductIS2_NS_8InstanceELZNS_17sTouchTransmitterEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_16TouchTransmitterELZNS_17sTouchTransmitterEENS_14FactoryProductIS2_NS_8InstanceELZNS_17sTouchTransmitterEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_16TouchTransmitterELZNS_17sTouchTransmitterEENS_14FactoryProductIS2_NS_8InstanceELZNS_17sTouchTransmitterEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_5e7f64() -> ! {
    todo!("0x5e7f64 __ZN3RBX10Reflection9DescribedINS_16TouchTransmitterELZNS_17sTouchTransmitterEENS_14FactoryProductIS2_NS_8InstanceELZNS_17sTouchTransmitterEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}


// 0x5e8084 — __ZThn32_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5e8084() -> ! {
    todo!("0x5e8084 __ZThn32_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x5e8098 — __ZThn32_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5e8098() -> ! {
    todo!("0x5e8098 __ZThn32_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x5e814c — __ZThn36_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5e814c() -> ! {
    todo!("0x5e814c __ZThn36_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x5e8160 — __ZThn36_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5e8160() -> ! {
    todo!("0x5e8160 __ZThn36_N3RBX21DescribedNonCreatableINS_12PartInstanceENS_10PVInstanceELZNS_5sPartEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x5e8214 — __ZThn32_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5e8214() -> ! {
    todo!("0x5e8214 __ZThn32_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x5e8228 — __ZThn32_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5e8228() -> ! {
    todo!("0x5e8228 __ZThn32_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x5e82dc — __ZThn36_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5e82dc() -> ! {
    todo!("0x5e82dc __ZThn36_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x5e82f0 — __ZThn36_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5e82f0() -> ! {
    todo!("0x5e82f0 __ZThn36_N3RBX10Reflection9DescribedINS_12PartInstanceELZNS_5sPartEENS_17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x5e83a4 — __ZThn32_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED1Ev
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED1Ev")]
// was: __ZThn32_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED1Ev
pub fn stub_5e83a4() -> ! {
    todo!("0x5e83a4 __ZThn32_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED1Ev")
}


// 0x5e83b8 — __ZThn32_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED0Ev
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED0Ev")]
// was: __ZThn32_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED0Ev
pub fn stub_5e83b8() -> ! {
    todo!("0x5e83b8 __ZThn32_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED0Ev")
}


// 0x5e8468 — __ZThn36_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED1Ev
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED1Ev")]
// was: __ZThn36_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED1Ev
pub fn stub_5e8468() -> ! {
    todo!("0x5e8468 __ZThn36_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED1Ev")
}


// 0x5e847c — __ZThn36_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED0Ev
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED0Ev")]
// was: __ZThn36_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED0Ev
pub fn stub_5e847c() -> ! {
    todo!("0x5e847c __ZThn36_N3RBX17NonFactoryProductINS_10PVInstanceELZNS_5sPartEEED0Ev")
}


// 0x5e852c — __ZN3RBX7FWFinalINS_14FWPartInstanceEED1Ev
#[doc(alias = "__ZN3RBX7FWFinalINS_14FWPartInstanceEED1Ev")]
#[doc(alias = "RBX::FWFinal<RBX::FWPartInstance>::~FWFinal()")]
// was: __ZN3RBX7FWFinalINS_14FWPartInstanceEED1Ev
pub fn stub_5e852c() -> ! {
    todo!("0x5e852c RBX::FWFinal<RBX::FWPartInstance>::~FWFinal()")
}


// 0x5e8530 — __ZN3RBX7FWFinalINS_14FWPartInstanceEED0Ev
#[doc(alias = "__ZN3RBX7FWFinalINS_14FWPartInstanceEED0Ev")]
#[doc(alias = "RBX::FWFinal<RBX::FWPartInstance>::~FWFinal()")]
// was: __ZN3RBX7FWFinalINS_14FWPartInstanceEED0Ev
pub fn stub_5e8530() -> ! {
    todo!("0x5e8530 RBX::FWFinal<RBX::FWPartInstance>::~FWFinal()")
}


// 0x5e85e4 — __ZN3RBX7FWFinalINS_14FWPartInstanceEED2Ev
#[doc(alias = "__ZN3RBX7FWFinalINS_14FWPartInstanceEED2Ev")]
#[doc(alias = "RBX::FWFinal<RBX::FWPartInstance>::~FWFinal()")]
// was: __ZN3RBX7FWFinalINS_14FWPartInstanceEED2Ev
pub fn stub_5e85e4() -> ! {
    todo!("0x5e85e4 RBX::FWFinal<RBX::FWPartInstance>::~FWFinal()")
}


// 0x5e86b8 — __ZN3RBX9AllocatorINS_14FWPartInstanceEEnwEm
// type: int(void)
#[doc(alias = "__ZN3RBX9AllocatorINS_14FWPartInstanceEEnwEm")]
#[doc(alias = "RBX::Allocator<RBX::FWPartInstance>::operator new(unsigned long)")]
// was: __ZN3RBX9AllocatorINS_14FWPartInstanceEEnwEm
pub fn stub_5e86b8() -> ! {
    todo!("0x5e86b8 RBX::Allocator<RBX::FWPartInstance>::operator new(unsigned long)")
}


// 0x5e8728 — __ZN5boost14singleton_poolIN3RBX14FWPartInstanceELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: int(void)
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX14FWPartInstanceELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
#[doc(alias = "boost::singleton_pool<RBX::FWPartInstance,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// was: __ZN5boost14singleton_poolIN3RBX14FWPartInstanceELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_5e8728() -> ! {
    todo!("0x5e8728 boost::singleton_pool<RBX::FWPartInstance,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}


// 0x5e8760 — __ZN5boost4poolINS_34default_user_allocator_malloc_freeEE18malloc_need_resizeEv
// type: int(void)
#[doc(alias = "__ZN5boost4poolINS_34default_user_allocator_malloc_freeEE18malloc_need_resizeEv")]
#[doc(alias = "boost::pool<boost::default_user_allocator_malloc_free>::malloc_need_resize(void)")]
// was: __ZN5boost4poolINS_34default_user_allocator_malloc_freeEE18malloc_need_resizeEv
pub fn stub_5e8760() -> ! {
    todo!("0x5e8760 boost::pool<boost::default_user_allocator_malloc_free>::malloc_need_resize(void)")
}


// 0x5e8840 — __ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEE13releaseMemoryEv
#[doc(alias = "__ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEE13releaseMemoryEv")]
#[doc(alias = "RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::releaseMemory(void)")]
// was: __ZN3RBX9AllocatorINS_12PartInstance20OnDemandPartInstanceEE13releaseMemoryEv
pub fn stub_5e8840() -> ! {
    todo!("0x5e8840 RBX::Allocator<RBX::PartInstance::OnDemandPartInstance>::releaseMemory(void)")
}


// 0x5e885c — __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int(void)
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
#[doc(alias = "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// was: __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_5e885c() -> ! {
    todo!("0x5e885c boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}


// 0x5e8890 — __ZN5boost10shared_ptrIN3RBX16TouchTransmitterEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX16TouchTransmitterEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "boost::shared_ptr<RBX::TouchTransmitter>::shared_ptr<RBX::TouchTransmitter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX16TouchTransmitterEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_5e8890() -> ! {
    todo!("0x5e8890 boost::shared_ptr<RBX::TouchTransmitter>::shared_ptr<RBX::TouchTransmitter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x5e8958 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16TouchTransmitterES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16TouchTransmitterES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TouchTransmitter,RBX::TouchTransmitter>(boost::shared_ptr<RBX::TouchTransmitter> const*,RBX::TouchTransmitter *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16TouchTransmitterES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_5e8958() -> ! {
    todo!("0x5e8958 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TouchTransmitter,RBX::TouchTransmitter>(boost::shared_ptr<RBX::TouchTransmitter> const*,RBX::TouchTransmitter *)const")
}


// 0x5e8a40 — __ZN5boost6detail12shared_countC2IPN3RBX16TouchTransmitterENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX16TouchTransmitterENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX16TouchTransmitterENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_5e8a40() -> ! {
    todo!("0x5e8a40 boost::detail::shared_count::shared_count<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x5e8b48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5e8b48() -> ! {
    todo!("0x5e8b48 boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x5e8b4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5e8b4c() -> ! {
    todo!("0x5e8b4c boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x5e8b50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5e8b50() -> ! {
    todo!("0x5e8b50 boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}


// 0x5e8b70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5e8b70() -> ! {
    todo!("0x5e8b70 boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}


// 0x5e8b88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16TouchTransmitterENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5e8b88() -> ! {
    todo!("0x5e8b88 boost::detail::sp_counted_impl_pd<RBX::TouchTransmitter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}


// 0x5e8b8c — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEED0Ev
pub fn stub_5e8b8c() -> ! {
    todo!("0x5e8b8c RBX::Reflection::EventDesc<RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::~EventDesc()")
}


// 0x5e8c40 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_5e8c40() -> ! {
    todo!("0x5e8c40 RBX::Reflection::EventDescImpl<0,RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}


// 0x5e8e64 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
pub fn stub_5e8e64() -> ! {
    todo!("0x5e8e64 RBX::Reflection::EventDescImpl<0,RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}


// 0x5e8edc — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE13disconnectAllEPNS0_11EventSourceE
pub fn stub_5e8edc() -> ! {
    todo!("0x5e8edc RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")
}


// 0x5e8f14 — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE9getSignalEPS2_
// type: int __fastcall(int, RBX::Instance *this)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE9getSignalEPS2_")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvvEN3rbx6signalIS3_EEMS2_FRS6_vEE9getSignalEPS2_
pub fn stub_5e8f14() -> ! {
    todo!("0x5e8f14 RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)>& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const")
}


// 0x5e8f88 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS2_7VariantESaIS5_EENS_10shared_ptrIS3_EES7_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS2_7VariantESaIS5_EENS_10shared_ptrIS3_EES7_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS2_7VariantESaIS5_EENS_10shared_ptrIS3_EES7_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
pub fn stub_5e8f88() -> ! {
    todo!("0x5e8f88 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>)")
}


// 0x5e9268 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS2_ISt6vectorINS5_7VariantESaISA_EEEEEC2ES8_SD_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS2_ISt6vectorINS5_7VariantESaISA_EEEEEC2ES8_SD_")]
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS2_ISt6vectorINS5_7VariantESaISA_EEEEEC2ES8_SD_
pub fn stub_5e9268() -> ! {
    todo!("0x5e9268 boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)")
}


// 0x5e9368 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS2_ISt6vectorINS5_7VariantESaISA_EEEEEC2ES8_SD_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS2_ISt6vectorINS5_7VariantESaISA_EEEEEC2ES8_SD_")]
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS2_ISt6vectorINS5_7VariantESaISA_EEEEEC2ES8_SD_
pub fn stub_5e9368() -> ! {
    todo!("0x5e9368 boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)")
}


// 0x5e9470 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS9_7VariantESaISC_EEEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENSJ_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS9_7VariantESaISC_EEEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENSJ_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS9_7VariantESaISC_EEEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENSJ_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
pub fn stub_5e9470() -> ! {
    todo!("0x5e9470 __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS9_7VariantESaISC_EEEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENSJ_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}


// 0x5e95d0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
pub fn stub_5e95d0() -> ! {
    todo!("0x5e95d0 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}


// 0x5e9738 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
pub fn stub_5e9738() -> ! {
    todo!("0x5e9738 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}


// 0x5e9758 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINSA_7VariantESaISD_EEEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENSK_ISF_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINSA_7VariantESaISD_EEEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENSK_ISF_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINSA_7VariantESaISD_EEEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENSK_ISF_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_5e9758() -> ! {
    todo!("0x5e9758 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}


// 0x5e98b8 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINSA_7VariantESaISD_EEEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENSK_ISF_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINSA_7VariantESaISD_EEEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENSK_ISF_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINSA_7VariantESaISD_EEEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENSK_ISF_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_5e98b8() -> ! {
    todo!("0x5e98b8 void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}


// 0x5e99c8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS5_7VariantESaIS8_EEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENSF_ISA_EEEEEclEv
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS5_7VariantESaIS8_EEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENSF_ISA_EEEEEclEv")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS5_7VariantESaIS8_EEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENSF_ISA_EEEEEclEv
pub fn stub_5e99c8() -> ! {
    todo!("0x5e99c8 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>::operator()(void)")
}


// 0x5e99e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKSt6vectorINS8_7VariantESaISB_EEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENSI_ISD_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_5e99e0() -> ! {
    todo!("0x5e99e0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}


// 0x5e9b98 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::function<void ()(void)>>(boost::function<void ()(void)> const&)")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_5e9b98() -> ! {
    todo!("0x5e9b98 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::function<void ()(void)>>(boost::function<void ()(void)> const&)")
}


// 0x5e9c90 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost8functionIS2_EEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost8functionIS2_EEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::function<void ()(void)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_5e9c90() -> ! {
    todo!("0x5e9c90 rbx::signals::signal<void ()(void)>::callable_slot<boost::function<void ()(void)>>::~callable_slot()")
}


// 0x5e9da0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost8functionIS2_EEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost8functionIS2_EEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::function<void ()(void)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_5e9da0() -> ! {
    todo!("0x5e9da0 rbx::signals::signal<void ()(void)>::callable_slot<boost::function<void ()(void)>>::~callable_slot()")
}


// 0x5e9ed0 — __ZN3rbx7signals6signalIFvvEE4slot10disconnectEv
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE4slot10disconnectEv")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvvEE4slot10disconnectEv
pub fn stub_5e9ed0() -> ! {
    todo!("0x5e9ed0 rbx::signals::signal<void ()(void)>::slot::disconnect(void)")
}


// 0x5e9fe0 — __ZNK3rbx7signals6signalIFvvEE4slot9connectedEv
#[doc(alias = "__ZNK3rbx7signals6signalIFvvEE4slot9connectedEv")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvvEE4slot9connectedEv
pub fn stub_5e9fe0() -> ! {
    todo!("0x5e9fe0 rbx::signals::signal<void ()(void)>::slot::connected(void)const")
}


// 0x5e9ff0 — __ZN3rbx7signals6signalIFvvEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE4slot24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvvEE4slot24safe_static_do_get_mutexEv
pub fn stub_5e9ff0() -> ! {
    todo!("0x5e9ff0 rbx::signals::signal<void ()(void)>::slot::safe_static_do_get_mutex(void)")
}


// 0x5ea0e0 — __ZN5boost5mutexD1Ev
// type: void __fastcall(pthread_mutex_t *this)
#[doc(alias = "__ZN5boost5mutexD1Ev")]
#[doc(alias = "boost::mutex::~mutex()")]
// was: __ZN5boost5mutexD1Ev
pub fn stub_5ea0e0() -> ! {
    todo!("0x5ea0e0 boost::mutex::~mutex()")
}


// 0x5ea0f8 — __ZN3rbx7signals6signalIFvvEE4slotD0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE4slotD0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvvEE4slotD0Ev
pub fn stub_5ea0f8() -> ! {
    todo!("0x5ea0f8 rbx::signals::signal<void ()(void)>::slot::~slot()")
}


// 0x5ea1cc — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS0_10Descriptor10AttributesE
pub fn stub_5ea1cc() -> ! {
    todo!("0x5ea1cc RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)")
}


// 0x5ea35c — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEED0Ev
pub fn stub_5ea35c() -> ! {
    todo!("0x5ea35c RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::~EventDesc()")
}


// 0x5ea410 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
pub fn stub_5ea410() -> ! {
    todo!("0x5ea410 RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}


// 0x5ea5b0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
pub fn stub_5ea5b0() -> ! {
    todo!("0x5ea5b0 RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}


// 0x5ea70c — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE13disconnectAllEPNS0_11EventSourceE
pub fn stub_5ea70c() -> ! {
    todo!("0x5ea70c RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")
}


// 0x5ea748 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE22safe_static_init_mutexEv
pub fn stub_5ea748() -> ! {
    todo!("0x5ea748 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::safe_static_init_mutex(void)")
}


// 0x5ea74c — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE9getSignalEPS2_
// type: int __fastcall(int, RBX::Instance *this)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE9getSignalEPS2_")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE9getSignalEPS2_
pub fn stub_5ea74c() -> ! {
    todo!("0x5ea74c RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const")
}

