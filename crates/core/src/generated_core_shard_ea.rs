//! core shard EA — 100 core stubs EA-sorted, next uncovered after DZ 0x897b78 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x897b78.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>::pair(std::string const&,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>> const&)")]
// 0x89e550 — __ZNSt4pairISsS_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEC2ERKSsRKS6_
// was: std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>::pair(std::string const&,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>> const&)
pub fn stub_89e550() -> ! {
    todo!("0x89e550 __ZNSt4pairISsS_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEC2ERKSsRKS6_")
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>> const&)")]
// 0x89e628 — __ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEESaIS8_EE14_M_create_nodeERKS8_
// was: std::list<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>> const&)
pub fn stub_89e628() -> ! {
    todo!("0x89e628 __ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEESaIS8_EE14_M_create_nodeERKS8_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0x89e744 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)
pub fn stub_89e744() -> ! {
    todo!("0x89e744 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "RBX::ControlledLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::ControlledLRUCache(unsigned long,RBX::CacheSizeEnforceMethod)")]
// 0x89e77c — __ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEEC2EmNS_22CacheSizeEnforceMethodE
// was: RBX::ControlledLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::ControlledLRUCache(unsigned long,RBX::CacheSizeEnforceMethod)
pub fn stub_89e77c() -> ! {
    todo!("0x89e77c __ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEEC2EmNS_22CacheSizeEnforceMethodE")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::LRUCache(void)")]
// 0x89e984 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEEC2Ev
// was: RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::LRUCache(void)
pub fn stub_89e984() -> ! {
    todo!("0x89e984 __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEEC2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>> const&)")]
// 0x89ea64 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSI_RKSK_RKSaINS1_8ptr_nodeISF_EEE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>> const&)
pub fn stub_89ea64() -> ! {
    todo!("0x89ea64 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSI_RKSK_RKSaINS1_8ptr_nodeISF_EEE")
}

#[doc(alias = "RBX::MemEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::resize(unsigned long)")]
// 0x89ead0 — __ZN3RBX19MemEnforcedLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6resizeEm
// was: RBX::MemEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::resize(unsigned long)
pub fn stub_89ead0() -> ! {
    todo!("0x89ead0 __ZN3RBX19MemEnforcedLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6resizeEm")
}

#[doc(alias = "RBX::MemEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::insert(std::string const&,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)")]
// 0x89eb34 — __ZN3RBX19MemEnforcedLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6insertERKSsRKS5_m
// was: RBX::MemEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::insert(std::string const&,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)
pub fn stub_89eb34() -> ! {
    todo!("0x89eb34 __ZN3RBX19MemEnforcedLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6insertERKSsRKS5_m")
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::resize(unsigned long)")]
// 0x89eb9c — __ZN3RBX20SizeEnforcedLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6resizeEm
// was: RBX::SizeEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::resize(unsigned long)
pub fn stub_89eb9c() -> ! {
    todo!("0x89eb9c __ZN3RBX20SizeEnforcedLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6resizeEm")
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::insert(std::string const&,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)")]
// 0x89ec14 — __ZN3RBX20SizeEnforcedLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6insertERKSsRKS5_m
// was: RBX::SizeEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::insert(std::string const&,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)
pub fn stub_89ec14() -> ! {
    todo!("0x89ec14 __ZN3RBX20SizeEnforcedLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6insertERKSsRKS5_m")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0x89ec88 — __ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_89ec88() -> ! {
    todo!("0x89ec88 __ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0x89ecc0 — __ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_89ecc0() -> ! {
    todo!("0x89ecc0 __ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::function2<RBX::TaskScheduler::StepResult,std::string,rbx_core::SharedPtr<std::string const>>::clear(void)")]
// 0x89eee4 — __ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE5clearEv
// was: boost::function2<RBX::TaskScheduler::StepResult,std::string,rbx_core::SharedPtr<std::string const>>::clear(void)
pub fn stub_89eee4() -> ! {
    todo!("0x89eee4 __ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE5clearEv")
}

#[doc(alias = "RBX::BindableFunction::processQueue(void)")]
// 0x89f3e8 — __ZN3RBX16BindableFunction12processQueueEv
pub fn stub_89f3e8() -> ! {
    todo!("0x89f3e8 __ZN3RBX16BindableFunction12processQueueEv")
}

#[doc(alias = "RBX::BindableFunction::~BindableFunction()")]
// 0x89fbb0 — __ZN3RBX16BindableFunctionD1Ev
pub fn stub_89fbb0() -> ! {
    todo!("0x89fbb0 __ZN3RBX16BindableFunctionD1Ev")
}

#[doc(alias = "RBX::BindableFunction::~BindableFunction()")]
// 0x89fcb8 — __ZN3RBX16BindableFunctionD0Ev
pub fn stub_89fcb8() -> ! {
    todo!("0x89fcb8 __ZN3RBX16BindableFunctionD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BindableFunction::~BindableFunction()")]
// 0x89fdd0 — __ZThn32_N3RBX16BindableFunctionD1Ev
pub fn stub_89fdd0() -> ! {
    todo!("0x89fdd0 __ZThn32_N3RBX16BindableFunctionD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BindableFunction::~BindableFunction()")]
// 0x89fed8 — __ZThn32_N3RBX16BindableFunctionD0Ev
pub fn stub_89fed8() -> ! {
    todo!("0x89fed8 __ZThn32_N3RBX16BindableFunctionD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BindableFunction::~BindableFunction()")]
// 0x89fff4 — __ZThn36_N3RBX16BindableFunctionD1Ev
pub fn stub_89fff4() -> ! {
    todo!("0x89fff4 __ZThn36_N3RBX16BindableFunctionD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BindableFunction::~BindableFunction()")]
// 0x8a00f8 — __ZThn36_N3RBX16BindableFunctionD0Ev
pub fn stub_8a00f8() -> ! {
    todo!("0x8a00f8 __ZThn36_N3RBX16BindableFunctionD0Ev")
}

#[doc(alias = "RBX::BindableEvent::~BindableEvent()")]
// 0x8a0210 — __ZN3RBX13BindableEventD1Ev
pub fn stub_8a0210() -> ! {
    todo!("0x8a0210 __ZN3RBX13BindableEventD1Ev")
}

#[doc(alias = "RBX::BindableEvent::~BindableEvent()")]
// 0x8a0320 — __ZN3RBX13BindableEventD0Ev
pub fn stub_8a0320() -> ! {
    todo!("0x8a0320 __ZN3RBX13BindableEventD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BindableEvent::~BindableEvent()")]
// 0x8a0444 — __ZThn32_N3RBX13BindableEventD1Ev
pub fn stub_8a0444() -> ! {
    todo!("0x8a0444 __ZThn32_N3RBX13BindableEventD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BindableEvent::~BindableEvent()")]
// 0x8a0554 — __ZThn32_N3RBX13BindableEventD0Ev
pub fn stub_8a0554() -> ! {
    todo!("0x8a0554 __ZThn32_N3RBX13BindableEventD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BindableEvent::~BindableEvent()")]
// 0x8a067c — __ZThn36_N3RBX13BindableEventD1Ev
pub fn stub_8a067c() -> ! {
    todo!("0x8a067c __ZThn36_N3RBX13BindableEventD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BindableEvent::~BindableEvent()")]
// 0x8a078c — __ZThn36_N3RBX13BindableEventD0Ev
pub fn stub_8a078c() -> ! {
    todo!("0x8a078c __ZThn36_N3RBX13BindableEventD0Ev")
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::push_back(RBX::BindableFunction::Invocation const&)")]
// 0x8a08b4 — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE9push_backERKS2_
pub fn stub_8a08b4() -> ! {
    todo!("0x8a08b4 __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_push_back_aux(RBX::BindableFunction::Invocation const&)")]
// 0x8a09ec — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE16_M_push_back_auxERKS2_
pub fn stub_8a09ec() -> ! {
    todo!("0x8a09ec __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE16_M_push_back_auxERKS2_")
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_reserve_map_at_back(unsigned long)")]
// 0x8a0c88 — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE22_M_reserve_map_at_backEm
pub fn stub_8a0c88() -> ! {
    todo!("0x8a0c88 __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE22_M_reserve_map_at_backEm")
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_reallocate_map(unsigned long,bool)")]
// 0x8a0ca4 — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE17_M_reallocate_mapEmb
pub fn stub_8a0ca4() -> ! {
    todo!("0x8a0ca4 __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::pop_front(void)")]
// 0x8a0d7c — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE9pop_frontEv
pub fn stub_8a0d7c() -> ! {
    todo!("0x8a0d7c __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE9pop_frontEv")
}

#[doc(alias = "std::deque<RBX::BindableFunction::Invocation,std::allocator<RBX::BindableFunction::Invocation>>::_M_pop_front_aux(void)")]
// 0x8a0e84 — __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE16_M_pop_front_auxEv
pub fn stub_8a0e84() -> ! {
    todo!("0x8a0e84 __ZNSt5dequeIN3RBX16BindableFunction10InvocationESaIS2_EE16_M_pop_front_auxEv")
}

#[doc(alias = "BuildGenericApiUrl(std::string,std::string,std::string,std::string)")]
// 0x8a2254 — __ZL18BuildGenericApiUrlSsSsSsSs
pub fn stub_8a2254() -> ! {
    todo!("0x8a2254 __ZL18BuildGenericApiUrlSsSsSsSs")
}

#[doc(alias = "GetSettingsUrl(std::string,std::string,std::string)")]
// 0x8a29d4 — __Z14GetSettingsUrlSsSsSs
pub fn stub_8a29d4() -> ! {
    todo!("0x8a29d4 __Z14GetSettingsUrlSsSsSs")
}

#[doc(alias = "RBX::BuoyancyContact::removeAllConnectorsFromKernel(void)")]
// 0x8a2c98 — __ZN3RBX15BuoyancyContact29removeAllConnectorsFromKernelEv
pub fn stub_8a2c98() -> ! {
    todo!("0x8a2c98 __ZN3RBX15BuoyancyContact29removeAllConnectorsFromKernelEv")
}

#[doc(alias = "RBX::BuoyancyContact::putAllConnectorsInKernel(void)")]
// 0x8a2d04 — __ZN3RBX15BuoyancyContact24putAllConnectorsInKernelEv
pub fn stub_8a2d04() -> ! {
    todo!("0x8a2d04 __ZN3RBX15BuoyancyContact24putAllConnectorsInKernelEv")
}

#[doc(alias = "RBX::BuoyancyContact::deleteConnectors(void)")]
// 0x8a2d6c — __ZN3RBX15BuoyancyContact16deleteConnectorsEv
pub fn stub_8a2d6c() -> ! {
    todo!("0x8a2d6c __ZN3RBX15BuoyancyContact16deleteConnectorsEv")
}

#[doc(alias = "RBX::BuoyancyContact::deleteAllConnectors(void)")]
// 0x8a2e38 — __ZN3RBX15BuoyancyContact19deleteAllConnectorsEv
pub fn stub_8a2e38() -> ! {
    todo!("0x8a2e38 __ZN3RBX15BuoyancyContact19deleteAllConnectorsEv")
}

#[doc(alias = "RBX::BuoyancyContact::determineGeometricType(RBX::Primitive *)")]
// 0x8a2e3c — __ZN3RBX15BuoyancyContact22determineGeometricTypeEPNS_9PrimitiveE
pub fn stub_8a2e3c() -> ! {
    todo!("0x8a2e3c __ZN3RBX15BuoyancyContact22determineGeometricTypeEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::BuoyancyContact::BuoyancyContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x8a2e84 — __ZN3RBX15BuoyancyContactC2EPNS_9PrimitiveES2_
pub fn stub_8a2e84() -> ! {
    todo!("0x8a2e84 __ZN3RBX15BuoyancyContactC2EPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::BuoyancyContact::~BuoyancyContact()")]
// 0x8a2fe0 — __ZN3RBX15BuoyancyContactD0Ev
pub fn stub_8a2fe0() -> ! {
    todo!("0x8a2fe0 __ZN3RBX15BuoyancyContactD0Ev")
}

#[doc(alias = "RBX::BuoyancyContact::~BuoyancyContact()")]
// 0x8a3080 — __ZN3RBX15BuoyancyContactD1Ev
pub fn stub_8a3080() -> ! {
    todo!("0x8a3080 __ZN3RBX15BuoyancyContactD1Ev")
}

#[doc(alias = "RBX::BuoyancyContact::~BuoyancyContact()")]
// 0x8a3084 — __ZN3RBX15BuoyancyContactD2Ev
pub fn stub_8a3084() -> ! {
    todo!("0x8a3084 __ZN3RBX15BuoyancyContactD2Ev")
}

#[doc(alias = "RBX::BuoyancyContact::computeExtentsWaterBand(RBX::Extents const&,float &,float &)")]
// 0x8a31c0 — __ZN3RBX15BuoyancyContact23computeExtentsWaterBandERKNS_7ExtentsERfS4_
pub fn stub_8a31c0() -> ! {
    todo!("0x8a31c0 __ZN3RBX15BuoyancyContact23computeExtentsWaterBandERKNS_7ExtentsERfS4_")
}

#[doc(alias = "RBX::BuoyancyContact::updateSubmergeRatio(void)")]
// 0x8a3930 — __ZN3RBX15BuoyancyContact19updateSubmergeRatioEv
pub fn stub_8a3930() -> ! {
    todo!("0x8a3930 __ZN3RBX15BuoyancyContact19updateSubmergeRatioEv")
}

#[doc(alias = "RBX::BuoyancyContact::getWaterVelocity(int)")]
// 0x8a39c0 — __ZN3RBX15BuoyancyContact16getWaterVelocityEi
pub fn stub_8a39c0() -> ! {
    todo!("0x8a39c0 __ZN3RBX15BuoyancyContact16getWaterVelocityEi")
}

#[doc(alias = "RBX::BuoyancyContact::updateConnectors(void)")]
// 0x8a3b28 — __ZN3RBX15BuoyancyContact16updateConnectorsEv
pub fn stub_8a3b28() -> ! {
    todo!("0x8a3b28 __ZN3RBX15BuoyancyContact16updateConnectorsEv")
}

#[doc(alias = "RBX::BuoyancyContact::stepContact(void)")]
// 0x8a3de8 — __ZN3RBX15BuoyancyContact11stepContactEv
pub fn stub_8a3de8() -> ! {
    todo!("0x8a3de8 __ZN3RBX15BuoyancyContact11stepContactEv")
}

#[doc(alias = "RBX::BuoyancyContact::isTouchingWater(RBX::Primitive *)")]
// 0x8a3e90 — __ZN3RBX15BuoyancyContact15isTouchingWaterEPNS_9PrimitiveE
pub fn stub_8a3e90() -> ! {
    todo!("0x8a3e90 __ZN3RBX15BuoyancyContact15isTouchingWaterEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::BuoyancyContact::computeIsColliding(float)")]
// 0x8a3f2c — __ZN3RBX15BuoyancyContact18computeIsCollidingEf
pub fn stub_8a3f2c() -> ! {
    todo!("0x8a3f2c __ZN3RBX15BuoyancyContact18computeIsCollidingEf")
}

#[doc(alias = "RBX::BuoyancyContact::computeIsCollidingUi(float)")]
// 0x8a3f38 — __ZN3RBX15BuoyancyContact20computeIsCollidingUiEf
pub fn stub_8a3f38() -> ! {
    todo!("0x8a3f38 __ZN3RBX15BuoyancyContact20computeIsCollidingUiEf")
}

#[doc(alias = "RBX::BuoyancyContact::create(RBX::Primitive *,RBX::Primitive *)")]
// 0x8a3f50 — __ZN3RBX15BuoyancyContact6createEPNS_9PrimitiveES2_
pub fn stub_8a3f50() -> ! {
    todo!("0x8a3f50 __ZN3RBX15BuoyancyContact6createEPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::BuoyancyBallContact::initializeCrossSections(void)")]
// 0x8a417c — __ZN3RBX19BuoyancyBallContact23initializeCrossSectionsEv
pub fn stub_8a417c() -> ! {
    todo!("0x8a417c __ZN3RBX19BuoyancyBallContact23initializeCrossSectionsEv")
}

#[doc(alias = "RBX::BuoyancyBallContact::computeIsColliding(float)")]
// 0x8a41f8 — __ZN3RBX19BuoyancyBallContact18computeIsCollidingEf
pub fn stub_8a41f8() -> ! {
    todo!("0x8a41f8 __ZN3RBX19BuoyancyBallContact18computeIsCollidingEf")
}

#[doc(alias = "RBX::BuoyancyBallContact::createConnectors(void)")]
// 0x8a4304 — __ZN3RBX19BuoyancyBallContact16createConnectorsEv
pub fn stub_8a4304() -> ! {
    todo!("0x8a4304 __ZN3RBX19BuoyancyBallContact16createConnectorsEv")
}

#[doc(alias = "RBX::BuoyancyBallContact::updateWaterBand(void)")]
// 0x8a43e8 — __ZN3RBX19BuoyancyBallContact15updateWaterBandEv
pub fn stub_8a43e8() -> ! {
    todo!("0x8a43e8 __ZN3RBX19BuoyancyBallContact15updateWaterBandEv")
}

#[doc(alias = "RBX::BuoyancyBallContact::updateSubmergeRatio(void)")]
// 0x8a47c8 — __ZN3RBX19BuoyancyBallContact19updateSubmergeRatioEv
pub fn stub_8a47c8() -> ! {
    todo!("0x8a47c8 __ZN3RBX19BuoyancyBallContact19updateSubmergeRatioEv")
}

#[doc(alias = "RBX::BuoyancyBallContact::getWaterVelocity(int)")]
// 0x8a48a4 — __ZN3RBX19BuoyancyBallContact16getWaterVelocityEi
pub fn stub_8a48a4() -> ! {
    todo!("0x8a48a4 __ZN3RBX19BuoyancyBallContact16getWaterVelocityEi")
}

#[doc(alias = "RBX::BuoyancyBoxContact::initializeCrossSections(void)")]
// 0x8a4a30 — __ZN3RBX18BuoyancyBoxContact23initializeCrossSectionsEv
pub fn stub_8a4a30() -> ! {
    todo!("0x8a4a30 __ZN3RBX18BuoyancyBoxContact23initializeCrossSectionsEv")
}

#[doc(alias = "RBX::BuoyancyBoxContact::createConnectors(void)")]
// 0x8a4d24 — __ZN3RBX18BuoyancyBoxContact16createConnectorsEv
pub fn stub_8a4d24() -> ! {
    todo!("0x8a4d24 __ZN3RBX18BuoyancyBoxContact16createConnectorsEv")
}

#[doc(alias = "RBX::BuoyancyBoxContact::updateWaterBand(void)")]
// 0x8a4e9c — __ZN3RBX18BuoyancyBoxContact15updateWaterBandEv
pub fn stub_8a4e9c() -> ! {
    todo!("0x8a4e9c __ZN3RBX18BuoyancyBoxContact15updateWaterBandEv")
}

#[doc(alias = "RBX::BuoyancyBoxContact::BuoyancyBoxContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x8a4fcc — __ZN3RBX18BuoyancyBoxContactC2EPNS_9PrimitiveES2_
pub fn stub_8a4fcc() -> ! {
    todo!("0x8a4fcc __ZN3RBX18BuoyancyBoxContactC2EPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::BuoyancyCylinderContact::initializeCrossSections(void)")]
// 0x8a50d4 — __ZN3RBX23BuoyancyCylinderContact23initializeCrossSectionsEv
pub fn stub_8a50d4() -> ! {
    todo!("0x8a50d4 __ZN3RBX23BuoyancyCylinderContact23initializeCrossSectionsEv")
}

#[doc(alias = "RBX::BuoyancyCylinderContact::updateSubmergeRatio(void)")]
// 0x8a5110 — __ZN3RBX23BuoyancyCylinderContact19updateSubmergeRatioEv
pub fn stub_8a5110() -> ! {
    todo!("0x8a5110 __ZN3RBX23BuoyancyCylinderContact19updateSubmergeRatioEv")
}

#[doc(alias = "RBX::BuoyancyWedgeContact::initializeCrossSections(void)")]
// 0x8a53e0 — __ZN3RBX20BuoyancyWedgeContact23initializeCrossSectionsEv
pub fn stub_8a53e0() -> ! {
    todo!("0x8a53e0 __ZN3RBX20BuoyancyWedgeContact23initializeCrossSectionsEv")
}

#[doc(alias = "RBX::BuoyancyWedgeContact::updateSubmergeRatio(void)")]
// 0x8a5464 — __ZN3RBX20BuoyancyWedgeContact19updateSubmergeRatioEv
pub fn stub_8a5464() -> ! {
    todo!("0x8a5464 __ZN3RBX20BuoyancyWedgeContact19updateSubmergeRatioEv")
}

#[doc(alias = "RBX::BuoyancyCornerWedgeContact::initializeCrossSections(void)")]
// 0x8a5520 — __ZN3RBX26BuoyancyCornerWedgeContact23initializeCrossSectionsEv
pub fn stub_8a5520() -> ! {
    todo!("0x8a5520 __ZN3RBX26BuoyancyCornerWedgeContact23initializeCrossSectionsEv")
}

#[doc(alias = "RBX::BuoyancyCornerWedgeContact::updateSubmergeRatio(void)")]
// 0x8a55bc — __ZN3RBX26BuoyancyCornerWedgeContact19updateSubmergeRatioEv
pub fn stub_8a55bc() -> ! {
    todo!("0x8a55bc __ZN3RBX26BuoyancyCornerWedgeContact19updateSubmergeRatioEv")
}

#[doc(alias = "RBX::FixedArray<RBX::BuoyancyConnector *,8ul>::operator[](unsigned long)")]
// 0x8a5678 — __ZN3RBX10FixedArrayIPNS_17BuoyancyConnectorELm8EEixEm
pub fn stub_8a5678() -> ! {
    todo!("0x8a5678 __ZN3RBX10FixedArrayIPNS_17BuoyancyConnectorELm8EEixEm")
}

#[doc(alias = "RBX::Constants::getKmsGravity(void)")]
// 0x8a56d8 — __ZN3RBX9Constants13getKmsGravityEv
pub fn stub_8a56d8() -> ! {
    todo!("0x8a56d8 __ZN3RBX9Constants13getKmsGravityEv")
}

#[doc(alias = "RBX::FixedArray<RBX::BuoyancyConnector *,8ul>::push_back(RBX::BuoyancyConnector * const&)")]
// 0x8a5728 — __ZN3RBX10FixedArrayIPNS_17BuoyancyConnectorELm8EE9push_backERKS2_
pub fn stub_8a5728() -> ! {
    todo!("0x8a5728 __ZN3RBX10FixedArrayIPNS_17BuoyancyConnectorELm8EE9push_backERKS2_")
}

#[doc(alias = "RBX::BuoyancyContact::numConnectors(void)const")]
// 0x8a5790 — __ZNK3RBX15BuoyancyContact13numConnectorsEv
pub fn stub_8a5790() -> ! {
    todo!("0x8a5790 __ZNK3RBX15BuoyancyContact13numConnectorsEv")
}

#[doc(alias = "RBX::BuoyancyContact::getConnector(int)")]
// 0x8a5794 — __ZN3RBX15BuoyancyContact12getConnectorEi
pub fn stub_8a5794() -> ! {
    todo!("0x8a5794 __ZN3RBX15BuoyancyContact12getConnectorEi")
}

#[doc(alias = "RBX::BuoyancyBallContact::~BuoyancyBallContact()")]
// 0x8a579c — __ZN3RBX19BuoyancyBallContactD1Ev
pub fn stub_8a579c() -> ! {
    todo!("0x8a579c __ZN3RBX19BuoyancyBallContactD1Ev")
}

#[doc(alias = "RBX::BuoyancyBallContact::~BuoyancyBallContact()")]
// 0x8a57a0 — __ZN3RBX19BuoyancyBallContactD0Ev
pub fn stub_8a57a0() -> ! {
    todo!("0x8a57a0 __ZN3RBX19BuoyancyBallContactD0Ev")
}

#[doc(alias = "RBX::BuoyancyBallContact::getType(void)")]
// 0x8a5840 — __ZN3RBX19BuoyancyBallContact7getTypeEv
pub fn stub_8a5840() -> ! {
    todo!("0x8a5840 __ZN3RBX19BuoyancyBallContact7getTypeEv")
}

#[doc(alias = "RBX::BuoyancyBoxContact::~BuoyancyBoxContact()")]
// 0x8a5844 — __ZN3RBX18BuoyancyBoxContactD1Ev
pub fn stub_8a5844() -> ! {
    todo!("0x8a5844 __ZN3RBX18BuoyancyBoxContactD1Ev")
}

#[doc(alias = "RBX::BuoyancyBoxContact::~BuoyancyBoxContact()")]
// 0x8a5848 — __ZN3RBX18BuoyancyBoxContactD0Ev
pub fn stub_8a5848() -> ! {
    todo!("0x8a5848 __ZN3RBX18BuoyancyBoxContactD0Ev")
}

#[doc(alias = "RBX::BuoyancyBoxContact::getType(void)")]
// 0x8a58e8 — __ZN3RBX18BuoyancyBoxContact7getTypeEv
pub fn stub_8a58e8() -> ! {
    todo!("0x8a58e8 __ZN3RBX18BuoyancyBoxContact7getTypeEv")
}

#[doc(alias = "RBX::BuoyancyCylinderContact::~BuoyancyCylinderContact()")]
// 0x8a58ec — __ZN3RBX23BuoyancyCylinderContactD1Ev
pub fn stub_8a58ec() -> ! {
    todo!("0x8a58ec __ZN3RBX23BuoyancyCylinderContactD1Ev")
}

#[doc(alias = "RBX::BuoyancyCylinderContact::~BuoyancyCylinderContact()")]
// 0x8a58f0 — __ZN3RBX23BuoyancyCylinderContactD0Ev
pub fn stub_8a58f0() -> ! {
    todo!("0x8a58f0 __ZN3RBX23BuoyancyCylinderContactD0Ev")
}

#[doc(alias = "RBX::BuoyancyCylinderContact::getType(void)")]
// 0x8a5990 — __ZN3RBX23BuoyancyCylinderContact7getTypeEv
pub fn stub_8a5990() -> ! {
    todo!("0x8a5990 __ZN3RBX23BuoyancyCylinderContact7getTypeEv")
}

#[doc(alias = "RBX::BuoyancyWedgeContact::~BuoyancyWedgeContact()")]
// 0x8a5994 — __ZN3RBX20BuoyancyWedgeContactD1Ev
pub fn stub_8a5994() -> ! {
    todo!("0x8a5994 __ZN3RBX20BuoyancyWedgeContactD1Ev")
}

#[doc(alias = "RBX::BuoyancyWedgeContact::~BuoyancyWedgeContact()")]
// 0x8a5998 — __ZN3RBX20BuoyancyWedgeContactD0Ev
pub fn stub_8a5998() -> ! {
    todo!("0x8a5998 __ZN3RBX20BuoyancyWedgeContactD0Ev")
}

#[doc(alias = "RBX::BuoyancyWedgeContact::getType(void)")]
// 0x8a5a38 — __ZN3RBX20BuoyancyWedgeContact7getTypeEv
pub fn stub_8a5a38() -> ! {
    todo!("0x8a5a38 __ZN3RBX20BuoyancyWedgeContact7getTypeEv")
}

#[doc(alias = "RBX::BuoyancyCornerWedgeContact::~BuoyancyCornerWedgeContact()")]
// 0x8a5a3c — __ZN3RBX26BuoyancyCornerWedgeContactD1Ev
pub fn stub_8a5a3c() -> ! {
    todo!("0x8a5a3c __ZN3RBX26BuoyancyCornerWedgeContactD1Ev")
}

#[doc(alias = "RBX::BuoyancyCornerWedgeContact::~BuoyancyCornerWedgeContact()")]
// 0x8a5a40 — __ZN3RBX26BuoyancyCornerWedgeContactD0Ev
pub fn stub_8a5a40() -> ! {
    todo!("0x8a5a40 __ZN3RBX26BuoyancyCornerWedgeContactD0Ev")
}

#[doc(alias = "RBX::BuoyancyCornerWedgeContact::getType(void)")]
// 0x8a5ae0 — __ZN3RBX26BuoyancyCornerWedgeContact7getTypeEv
pub fn stub_8a5ae0() -> ! {
    todo!("0x8a5ae0 __ZN3RBX26BuoyancyCornerWedgeContact7getTypeEv")
}

#[doc(alias = "RBX::HUMAN::Ragdoll::Ragdoll(RBX::Humanoid *,RBX::HUMAN::StateType)")]
// 0x8a5f7c — __ZN3RBX5HUMAN7RagdollC1EPNS_8HumanoidENS0_9StateTypeE
pub fn stub_8a5f7c() -> ! {
    todo!("0x8a5f7c __ZN3RBX5HUMAN7RagdollC1EPNS_8HumanoidENS0_9StateTypeE")
}

#[doc(alias = "RBX::HUMAN::Ragdoll::~Ragdoll()")]
// 0x8a5fa4 — __ZN3RBX5HUMAN7RagdollD0Ev
pub fn stub_8a5fa4() -> ! {
    todo!("0x8a5fa4 __ZN3RBX5HUMAN7RagdollD0Ev")
}

#[doc(alias = "RBX::HUMAN::Ragdoll::~Ragdoll()")]
// 0x8a6044 — __ZN3RBX5HUMAN7RagdollD1Ev
pub fn stub_8a6044() -> ! {
    todo!("0x8a6044 __ZN3RBX5HUMAN7RagdollD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::HUMAN::Ragdoll::~Ragdoll()")]
// 0x8a6048 — __ZThn4_N3RBX5HUMAN7RagdollD0Ev
pub fn stub_8a6048() -> ! {
    todo!("0x8a6048 __ZThn4_N3RBX5HUMAN7RagdollD0Ev")
}

#[doc(alias = "RBX::HUMAN::Ragdoll::~Ragdoll()")]
// 0x8a6050 — __ZN3RBX5HUMAN7RagdollD2Ev
pub fn stub_8a6050() -> ! {
    todo!("0x8a6050 __ZN3RBX5HUMAN7RagdollD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::HUMAN::Ragdoll::~Ragdoll()")]
// 0x8a6128 — __ZThn4_N3RBX5HUMAN7RagdollD1Ev
pub fn stub_8a6128() -> ! {
    todo!("0x8a6128 __ZThn4_N3RBX5HUMAN7RagdollD1Ev")
}

#[doc(alias = "RBX::HUMAN::Ragdoll::onStepImpl(void)")]
// 0x8a6130 — __ZN3RBX5HUMAN7Ragdoll10onStepImplEv
pub fn stub_8a6130() -> ! {
    todo!("0x8a6130 __ZN3RBX5HUMAN7Ragdoll10onStepImplEv")
}

#[doc(alias = "RBX::HUMAN::Ragdoll::onComputeForceImpl(void)")]
// 0x8a61f4 — __ZN3RBX5HUMAN7Ragdoll18onComputeForceImplEv
pub fn stub_8a61f4() -> ! {
    todo!("0x8a61f4 __ZN3RBX5HUMAN7Ragdoll18onComputeForceImplEv")
}

#[doc(alias = "RBX::HUMAN::Ragdoll::getStateType(void)const")]
// 0x8a61f8 — __ZNK3RBX5HUMAN7Ragdoll12getStateTypeEv
pub fn stub_8a61f8() -> ! {
    todo!("0x8a61f8 __ZNK3RBX5HUMAN7Ragdoll12getStateTypeEv")
}

#[doc(alias = "RBX::BuoyancyConnector::getWorldPosition(void)")]
// 0x8a6584 — __ZN3RBX17BuoyancyConnector16getWorldPositionEv
pub fn stub_8a6584() -> ! {
    todo!("0x8a6584 __ZN3RBX17BuoyancyConnector16getWorldPositionEv")
}

#[doc(alias = "RBX::BuoyancyConnector::computeForce(bool)")]
// 0x8a6630 — __ZN3RBX17BuoyancyConnector12computeForceEb
pub fn stub_8a6630() -> ! {
    todo!("0x8a6630 __ZN3RBX17BuoyancyConnector12computeForceEb")
}

#[doc(alias = "RBX::BuoyancyConnector::updateContactPoint(void)")]
// 0x8a67d8 — __ZN3RBX17BuoyancyConnector18updateContactPointEv
pub fn stub_8a67d8() -> ! {
    todo!("0x8a67d8 __ZN3RBX17BuoyancyConnector18updateContactPointEv")
}
