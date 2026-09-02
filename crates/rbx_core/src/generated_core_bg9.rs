//! core bg9 — 100 core stubs EA-sorted asc distinct not yet in rbx_core nor global set.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua) global distinct not yet in crates/rbx_core/src nor /tmp/global_eas.txt — next 100 uncovered after 0xb46d24 (prior max 0xb44e58) -> 0xb46d24..0xb623b0.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::Bucket::splice(std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>,RBX::Network::ErrorCompPhysicsSender2::Bucket*,std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>)")]
#[doc(alias = "__ZN3RBX7Network23ErrorCompPhysicsSender26Bucket6spliceESt14_List_iteratorINS1_6NuggetEEPS2_S5_")]
// 0xb46d24 — __ZN3RBX7Network23ErrorCompPhysicsSender26Bucket6spliceESt14_List_iteratorINS1_6NuggetEEPS2_S5_
// type: std::_List_node_base *__fastcall(std::_List_node_base **, std::_List_node_base *this, std::_List_node_base **, std::_List_node_base *)
pub fn stub_0xb46d24() -> ! {
    todo!("0xb46d24 __ZN3RBX7Network23ErrorCompPhysicsSender26Bucket6spliceESt14_List_iteratorINS1_6NuggetEEPS2_S5_")
}

#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::calculateSendCount(void)")]
#[doc(alias = "__ZN3RBX7Network23ErrorCompPhysicsSender218calculateSendCountEv")]
// 0xb46d9c — __ZN3RBX7Network23ErrorCompPhysicsSender218calculateSendCountEv
// type: int __fastcall(RBX::Network::ErrorCompPhysicsSender2 *this)
pub fn stub_0xb46d9c() -> ! {
    todo!("0xb46d9c __ZN3RBX7Network23ErrorCompPhysicsSender218calculateSendCountEv")
}

#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::sendPacket(int,PacketPriority,RBX::Network::ReplicatorStats::PhysicsSenderStats *)")]
#[doc(alias = "__ZN3RBX7Network23ErrorCompPhysicsSender210sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE")]
// 0xb46e90 — __ZN3RBX7Network23ErrorCompPhysicsSender210sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE
// type: int __fastcall(int, int, int, int)
pub fn stub_0xb46e90() -> ! {
    todo!("0xb46e90 __ZN3RBX7Network23ErrorCompPhysicsSender210sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE")
}

#[doc(alias = "std::list<RBX::Network::ErrorCompPhysicsSender2::Nugget,std::allocator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>::_M_insert(std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>,RBX::Network::ErrorCompPhysicsSender2::Nugget const&)")]
#[doc(alias = "__ZNSt4listIN3RBX7Network23ErrorCompPhysicsSender26NuggetESaIS3_EE9_M_insertESt14_List_iteratorIS3_ERKS3_")]
// 0xb4884c — __ZNSt4listIN3RBX7Network23ErrorCompPhysicsSender26NuggetESaIS3_EE9_M_insertESt14_List_iteratorIS3_ERKS3_
// type: void __fastcall(int, std::_List_node_base *, int *, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
pub fn stub_0xb4884c() -> ! {
    todo!("0xb4884c __ZNSt4listIN3RBX7Network23ErrorCompPhysicsSender26NuggetESaIS3_EE9_M_insertESt14_List_iteratorIS3_ERKS3_")
}

#[doc(alias = "std::vector<RBX::Network::ErrorCompPhysicsSender2::Bucket,std::allocator<RBX::Network::ErrorCompPhysicsSender2::Bucket>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::ErrorCompPhysicsSender2::Bucket*,std::vector<RBX::Network::ErrorCompPhysicsSender2::Bucket,std::allocator<RBX::Network::ErrorCompPhysicsSender2::Bucket>>>,RBX::Network::ErrorCompPhysicsSender2::Bucket const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX7Network23ErrorCompPhysicsSender26BucketESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
// 0xb49c30 — __ZNSt6vectorIN3RBX7Network23ErrorCompPhysicsSender26BucketESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: void __fastcall(struct _Unwind_Exception *, _DWORD *, int, int, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int, int, int, int, void *, int, int, int, int, int, void *, int)
pub fn stub_0xb49c30() -> ! {
    todo!("0xb49c30 __ZNSt6vectorIN3RBX7Network23ErrorCompPhysicsSender26BucketESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::Bucket * std::__uninitialized_copy_aux<RBX::Network::ErrorCompPhysicsSender2::Bucket *,RBX::Network::ErrorCompPhysicsSender2::Bucket *>(RBX::Network::ErrorCompPhysicsSender2::Bucket *,RBX::Network::ErrorCompPhysicsSender2::Bucket *,RBX::Network::ErrorCompPhysicsSender2::Bucket *,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxIPN3RBX7Network23ErrorCompPhysicsSender26BucketES4_ET0_T_S6_S5_St12__false_type")]
// 0xb4a0e8 — __ZSt24__uninitialized_copy_auxIPN3RBX7Network23ErrorCompPhysicsSender26BucketES4_ET0_T_S6_S5_St12__false_type
// type: int __fastcall(char *, char *, __int64, void *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int)
pub fn stub_0xb4a0e8() -> ! {
    todo!("0xb4a0e8 __ZSt24__uninitialized_copy_auxIPN3RBX7Network23ErrorCompPhysicsSender26BucketES4_ET0_T_S6_S5_St12__false_type")
}

#[doc(alias = "std::list<RBX::Network::ErrorCompPhysicsSender2::Nugget,std::allocator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>::operator=(std::list<RBX::Network::ErrorCompPhysicsSender2::Nugget,std::allocator<RBX::Network::ErrorCompPhysicsSender2::Nugget>> const&)")]
#[doc(alias = "__ZNSt4listIN3RBX7Network23ErrorCompPhysicsSender26NuggetESaIS3_EEaSERKS5_")]
// 0xb4a298 — __ZNSt4listIN3RBX7Network23ErrorCompPhysicsSender26NuggetESaIS3_EEaSERKS5_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0xb4a298() -> ! {
    todo!("0xb4a298 __ZNSt4listIN3RBX7Network23ErrorCompPhysicsSender26NuggetESaIS3_EEaSERKS5_")
}

#[doc(alias = "void std::list<RBX::Network::ErrorCompPhysicsSender2::Nugget,std::allocator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>::insert<std::_List_const_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>(std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>,std::_List_const_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>,std::_List_const_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>)")]
#[doc(alias = "__ZNSt4listIN3RBX7Network23ErrorCompPhysicsSender26NuggetESaIS3_EE6insertISt20_List_const_iteratorIS3_EEEvSt14_List_iteratorIS3_ET_SB_")]
// 0xb4a410 — __ZNSt4listIN3RBX7Network23ErrorCompPhysicsSender26NuggetESaIS3_EE6insertISt20_List_const_iteratorIS3_EEEvSt14_List_iteratorIS3_ET_SB_
// type: void __fastcall(int, std::_List_node_base *, void *, void *, int, int, int, int, int, int)
pub fn stub_0xb4a410() -> ! {
    todo!("0xb4a410 __ZNSt4listIN3RBX7Network23ErrorCompPhysicsSender26NuggetESaIS3_EE6insertISt20_List_const_iteratorIS3_EEEvSt14_List_iteratorIS3_ET_SB_")
}

#[doc(alias = "RBX::Network::ClusterUpdateBuffer::ClusterUpdateBuffer(void)")]
#[doc(alias = "__ZN3RBX7Network19ClusterUpdateBufferC1Ev")]
// 0xb4d654 — __ZN3RBX7Network19ClusterUpdateBufferC1Ev
// type: RBX::Network::ClusterUpdateBuffer *__fastcall(RBX::Network::ClusterUpdateBuffer *this)
pub fn stub_0xb4d654() -> ! {
    todo!("0xb4d654 __ZN3RBX7Network19ClusterUpdateBufferC1Ev")
}

#[doc(alias = "RBX::Network::ClusterUpdateBuffer::size(void)const")]
#[doc(alias = "__ZNK3RBX7Network19ClusterUpdateBuffer4sizeEv")]
// 0xb4d718 — __ZNK3RBX7Network19ClusterUpdateBuffer4sizeEv
// type: int __fastcall(RBX::Network::ClusterUpdateBuffer *this)
pub fn stub_0xb4d718() -> ! {
    todo!("0xb4d718 __ZNK3RBX7Network19ClusterUpdateBuffer4sizeEv")
}

#[doc(alias = "std::vector<RBX::UintSet,std::allocator<RBX::UintSet>>::vector(unsigned long,RBX::UintSet const&,std::allocator<RBX::UintSet> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX7UintSetESaIS1_EEC2EmRKS1_RKS2_")]
// 0xb4d8dc — __ZNSt6vectorIN3RBX7UintSetESaIS1_EEC2EmRKS1_RKS2_
// type: int *__fastcall(int *, unsigned int, __int64 *)
pub fn stub_0xb4d8dc() -> ! {
    todo!("0xb4d8dc __ZNSt6vectorIN3RBX7UintSetESaIS1_EEC2EmRKS1_RKS2_")
}

#[doc(alias = "RBX::Network::ClusterPacketCache::ClusterPacketCache(void)")]
#[doc(alias = "__ZN3RBX7Network18ClusterPacketCacheC1Ev")]
// 0xb4e12c — __ZN3RBX7Network18ClusterPacketCacheC1Ev
// type: int __fastcall(RBX::Network::ClusterPacketCache *this)
pub fn stub_0xb4e12c() -> ! {
    todo!("0xb4e12c __ZN3RBX7Network18ClusterPacketCacheC1Ev")
}

#[doc(alias = "RBX::Network::ClusterPacketCache::ClusterPacketCache(void)")]
#[doc(alias = "__ZN3RBX7Network18ClusterPacketCacheC2Ev")]
// 0xb4e138 — __ZN3RBX7Network18ClusterPacketCacheC2Ev
// type: RBX::Instance *__fastcall(RBX::Network::ClusterPacketCache *this)
pub fn stub_0xb4e138() -> ! {
    todo!("0xb4e138 __ZN3RBX7Network18ClusterPacketCacheC2Ev")
}

#[doc(alias = "RBX::Network::ClusterPacketCache::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
#[doc(alias = "__ZN3RBX7Network18ClusterPacketCache18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")]
// 0xb4e948 — __ZN3RBX7Network18ClusterPacketCache18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: int __fastcall(int, int)
pub fn stub_0xb4e948() -> ! {
    todo!("0xb4e948 __ZN3RBX7Network18ClusterPacketCache18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ClusterPacketCache::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
#[doc(alias = "__ZThn96_N3RBX7Network18ClusterPacketCache18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")]
// 0xb4e998 — __ZThn96_N3RBX7Network18ClusterPacketCache18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: int __fastcall(int, int)
pub fn stub_0xb4e998() -> ! {
    todo!("0xb4e998 __ZThn96_N3RBX7Network18ClusterPacketCache18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "RBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
#[doc(alias = "__ZN3RBX7Network18ClusterPacketCacheD1Ev")]
// 0xb4f1ec — __ZN3RBX7Network18ClusterPacketCacheD1Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
pub fn stub_0xb4f1ec() -> ! {
    todo!("0xb4f1ec __ZN3RBX7Network18ClusterPacketCacheD1Ev")
}

#[doc(alias = "RBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
#[doc(alias = "__ZN3RBX7Network18ClusterPacketCacheD0Ev")]
// 0xb4f1f8 — __ZN3RBX7Network18ClusterPacketCacheD0Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
pub fn stub_0xb4f1f8() -> ! {
    todo!("0xb4f1f8 __ZN3RBX7Network18ClusterPacketCacheD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
#[doc(alias = "__ZThn32_N3RBX7Network18ClusterPacketCacheD1Ev")]
// 0xb4f398 — __ZThn32_N3RBX7Network18ClusterPacketCacheD1Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
pub fn stub_0xb4f398() -> ! {
    todo!("0xb4f398 __ZThn32_N3RBX7Network18ClusterPacketCacheD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
#[doc(alias = "__ZThn32_N3RBX7Network18ClusterPacketCacheD0Ev")]
// 0xb4f3a4 — __ZThn32_N3RBX7Network18ClusterPacketCacheD0Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
pub fn stub_0xb4f3a4() -> ! {
    todo!("0xb4f3a4 __ZThn32_N3RBX7Network18ClusterPacketCacheD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
#[doc(alias = "__ZThn36_N3RBX7Network18ClusterPacketCacheD1Ev")]
// 0xb4f544 — __ZThn36_N3RBX7Network18ClusterPacketCacheD1Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
pub fn stub_0xb4f544() -> ! {
    todo!("0xb4f544 __ZThn36_N3RBX7Network18ClusterPacketCacheD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
#[doc(alias = "__ZThn36_N3RBX7Network18ClusterPacketCacheD0Ev")]
// 0xb4f550 — __ZThn36_N3RBX7Network18ClusterPacketCacheD0Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
pub fn stub_0xb4f550() -> ! {
    todo!("0xb4f550 __ZThn36_N3RBX7Network18ClusterPacketCacheD0Ev")
}

#[doc(alias = "boost::shared_mutex::release_waiters(void)")]
#[doc(alias = "__ZN5boost12shared_mutex15release_waitersEv")]
// 0xb4f5f8 — __ZN5boost12shared_mutex15release_waitersEv
// type: void __fastcall(boost::shared_mutex *this)
pub fn stub_0xb4f5f8() -> ! {
    todo!("0xb4f5f8 __ZN5boost12shared_mutex15release_waitersEv")
}

#[doc(alias = "boost::shared_mutex::lock(void)")]
#[doc(alias = "__ZN5boost12shared_mutex4lockEv")]
// 0xb4f6f8 — __ZN5boost12shared_mutex4lockEv
// type: void __fastcall(boost::shared_mutex *this, int, int, int)
pub fn stub_0xb4f6f8() -> ! {
    todo!("0xb4f6f8 __ZN5boost12shared_mutex4lockEv")
}

#[doc(alias = "boost::condition_variable::wait(boost::unique_lock<boost::mutex> &)")]
#[doc(alias = "__ZN5boost18condition_variable4waitERNS_11unique_lockINS_5mutexEEE")]
// 0xb4f818 — __ZN5boost18condition_variable4waitERNS_11unique_lockINS_5mutexEEE
// type: void __fastcall(int, int)
pub fn stub_0xb4f818() -> ! {
    todo!("0xb4f818 __ZN5boost18condition_variable4waitERNS_11unique_lockINS_5mutexEEE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>> const&)")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEEC1ERKS5_")]
// 0xb4fb08 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEEC1ERKS5_
// type: int __fastcall(int, int, int, int, char, std::exception *, int, int, int, int)
pub fn stub_0xb4fb08() -> ! {
    todo!("0xb4fb08 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEEC1ERKS5_")
}

#[doc(alias = "boost::detail::interruption_checker::interruption_checker(_opaque_pthread_mutex_t *,_opaque_pthread_cond_t *)")]
#[doc(alias = "__ZN5boost6detail20interruption_checkerC2EP23_opaque_pthread_mutex_tP22_opaque_pthread_cond_t")]
// 0xb4fcf0 — __ZN5boost6detail20interruption_checkerC2EP23_opaque_pthread_mutex_tP22_opaque_pthread_cond_t
// type: pthread_mutex_t **__fastcall(pthread_mutex_t **this, _opaque_pthread_mutex_t *, _opaque_pthread_cond_t *)
pub fn stub_0xb4fcf0() -> ! {
    todo!("0xb4fcf0 __ZN5boost6detail20interruption_checkerC2EP23_opaque_pthread_mutex_tP22_opaque_pthread_cond_t")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StreamRegion::Id const,RBX::Network::ClusterPacketCache::CachedBitStream>>,RBX::StreamRegion::Id,RBX::Network::ClusterPacketCache::CachedBitStream,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::operator[](RBX::StreamRegion::Id const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network18ClusterPacketCache15CachedBitStreamEEES7_SB_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEEixERS8_")]
// 0xb4fe28 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network18ClusterPacketCache15CachedBitStreamEEES7_SB_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEEixERS8_
// type: _QWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0xb4fe28() -> ! {
    todo!("0xb4fe28 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network18ClusterPacketCache15CachedBitStreamEEES7_SB_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEEixERS8_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StreamRegion::Id const,RBX::Network::ClusterPacketCache::CachedBitStream>>,RBX::StreamRegion::Id,RBX::Network::ClusterPacketCache::CachedBitStream,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network18ClusterPacketCache15CachedBitStreamEEES7_SB_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEE18reserve_for_insertEm")]
// 0xb50078 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network18ClusterPacketCache15CachedBitStreamEEES7_SB_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
pub fn stub_0xb50078() -> ! {
    todo!("0xb50078 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network18ClusterPacketCache15CachedBitStreamEEES7_SB_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StreamRegion::Id const,RBX::Network::ClusterPacketCache::CachedBitStream>>,RBX::StreamRegion::Id,RBX::Network::ClusterPacketCache::CachedBitStream,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network18ClusterPacketCache15CachedBitStreamEEES7_SB_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEE14create_bucketsEm")]
// 0xb50220 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network18ClusterPacketCache15CachedBitStreamEEES7_SB_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
pub fn stub_0xb50220() -> ! {
    todo!("0xb50220 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network18ClusterPacketCache15CachedBitStreamEEES7_SB_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEE14create_bucketsEm")
}

#[doc(alias = "boost::shared_mutex::lock_shared(void)")]
#[doc(alias = "__ZN5boost12shared_mutex11lock_sharedEv")]
// 0xb504e0 — __ZN5boost12shared_mutex11lock_sharedEv
// type: void __fastcall(boost::shared_mutex *this, int, int, int)
pub fn stub_0xb504e0() -> ! {
    todo!("0xb504e0 __ZN5boost12shared_mutex11lock_sharedEv")
}

#[doc(alias = "RBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
#[doc(alias = "__ZN3RBX7Network18ClusterPacketCacheD2Ev")]
// 0xb50604 — __ZN3RBX7Network18ClusterPacketCacheD2Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
pub fn stub_0xb50604() -> ! {
    todo!("0xb50604 __ZN3RBX7Network18ClusterPacketCacheD2Ev")
}

#[doc(alias = "boost::shared_mutex::shared_mutex(void)")]
#[doc(alias = "__ZN5boost12shared_mutexC2Ev")]
// 0xb507e8 — __ZN5boost12shared_mutexC2Ev
// type: boost::shared_mutex *__fastcall(boost::shared_mutex *this)
pub fn stub_0xb507e8() -> ! {
    todo!("0xb507e8 __ZN5boost12shared_mutexC2Ev")
}

#[doc(alias = "RBX::Network::ReplicatorStats::PhysicsSenderStats::PhysicsSenderStats(void)")]
#[doc(alias = "__ZN3RBX7Network15ReplicatorStats18PhysicsSenderStatsC2Ev")]
// 0xb50f54 — __ZN3RBX7Network15ReplicatorStats18PhysicsSenderStatsC2Ev
// type: RBX::Network::ReplicatorStats::PhysicsSenderStats *__fastcall(RBX::Network::ReplicatorStats::PhysicsSenderStats *this)
pub fn stub_0xb50f54() -> ! {
    todo!("0xb50f54 __ZN3RBX7Network15ReplicatorStats18PhysicsSenderStatsC2Ev")
}

#[doc(alias = "RBX::Network::ReplicatorStats::ReplicatorStats(void)")]
#[doc(alias = "__ZN3RBX7Network15ReplicatorStatsC1Ev")]
// 0xb51274 — __ZN3RBX7Network15ReplicatorStatsC1Ev
// type: RBX::Network::ReplicatorStats *__fastcall(RBX::Network::ReplicatorStats *this)
pub fn stub_0xb51274() -> ! {
    todo!("0xb51274 __ZN3RBX7Network15ReplicatorStatsC1Ev")
}

#[doc(alias = "RBX::Network::ReplicatorStats::ReplicatorStats(void)")]
#[doc(alias = "__ZN3RBX7Network15ReplicatorStatsC2Ev")]
// 0xb51280 — __ZN3RBX7Network15ReplicatorStatsC2Ev
// type: RBX::Network::ReplicatorStats *__fastcall(RBX::Network::ReplicatorStats *this)
pub fn stub_0xb51280() -> ! {
    todo!("0xb51280 __ZN3RBX7Network15ReplicatorStatsC2Ev")
}

#[doc(alias = "RBX::Network::ReplicatorStats::incrementPacketsSent(RBX::Network::ReplicatorStats::PacketType)")]
#[doc(alias = "__ZN3RBX7Network15ReplicatorStats20incrementPacketsSentENS1_10PacketTypeE")]
// 0xb51a44 — __ZN3RBX7Network15ReplicatorStats20incrementPacketsSentENS1_10PacketTypeE
// type: int __fastcall(int, int)
pub fn stub_0xb51a44() -> ! {
    todo!("0xb51a44 __ZN3RBX7Network15ReplicatorStats20incrementPacketsSentENS1_10PacketTypeE")
}

#[doc(alias = "RBX::Network::ReplicatorStats::incrementPacketsSent(std::string const&)")]
#[doc(alias = "__ZN3RBX7Network15ReplicatorStats20incrementPacketsSentERKSs")]
// 0xb51aac — __ZN3RBX7Network15ReplicatorStats20incrementPacketsSentERKSs
// type: int __fastcall(int this, const std::string *)
pub fn stub_0xb51aac() -> ! {
    todo!("0xb51aac __ZN3RBX7Network15ReplicatorStats20incrementPacketsSentERKSs")
}

#[doc(alias = "RBX::Network::ReplicatorStats::incrementPacketsReceived(RBX::Network::ReplicatorStats::PacketType)")]
#[doc(alias = "__ZN3RBX7Network15ReplicatorStats24incrementPacketsReceivedENS1_10PacketTypeE")]
// 0xb51b34 — __ZN3RBX7Network15ReplicatorStats24incrementPacketsReceivedENS1_10PacketTypeE
// type: int __fastcall(int, int)
pub fn stub_0xb51b34() -> ! {
    todo!("0xb51b34 __ZN3RBX7Network15ReplicatorStats24incrementPacketsReceivedENS1_10PacketTypeE")
}

#[doc(alias = "RBX::Network::ReplicatorStats::incrementPacketsReceived(std::string const&)")]
#[doc(alias = "__ZN3RBX7Network15ReplicatorStats24incrementPacketsReceivedERKSs")]
// 0xb51bac — __ZN3RBX7Network15ReplicatorStats24incrementPacketsReceivedERKSs
// type: int __fastcall(int this, const std::string *)
pub fn stub_0xb51bac() -> ! {
    todo!("0xb51bac __ZN3RBX7Network15ReplicatorStats24incrementPacketsReceivedERKSs")
}

#[doc(alias = "RBX::Network::Replicator::ChangePropertyItem::~ChangePropertyItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator18ChangePropertyItemD1Ev")]
// 0xb52380 — __ZN3RBX7Network10Replicator18ChangePropertyItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::ChangePropertyItem *__hidden this)
pub fn stub_0xb52380() -> ! {
    todo!("0xb52380 __ZN3RBX7Network10Replicator18ChangePropertyItemD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::ChangePropertyItem::~ChangePropertyItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator18ChangePropertyItemD0Ev")]
// 0xb523a4 — __ZN3RBX7Network10Replicator18ChangePropertyItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::ChangePropertyItem *__hidden this)
pub fn stub_0xb523a4() -> ! {
    todo!("0xb523a4 __ZN3RBX7Network10Replicator18ChangePropertyItemD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::EventInvocationItem::~EventInvocationItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator19EventInvocationItemD1Ev")]
// 0xb54e88 — __ZN3RBX7Network10Replicator19EventInvocationItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::EventInvocationItem *__hidden this)
pub fn stub_0xb54e88() -> ! {
    todo!("0xb54e88 __ZN3RBX7Network10Replicator19EventInvocationItemD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::EventInvocationItem::~EventInvocationItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator19EventInvocationItemD0Ev")]
// 0xb54f98 — __ZN3RBX7Network10Replicator19EventInvocationItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::EventInvocationItem *__hidden this)
pub fn stub_0xb54f98() -> ! {
    todo!("0xb54f98 __ZN3RBX7Network10Replicator19EventInvocationItemD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::MarkerItem::MarkerItem(RBX::Network::Replicator*,long)")]
#[doc(alias = "__ZN3RBX7Network10Replicator10MarkerItemC1EPS1_l")]
// 0xb557ec — __ZN3RBX7Network10Replicator10MarkerItemC1EPS1_l
// type: _DWORD *__fastcall(_DWORD *this, RBX::Network::Replicator *, int)
pub fn stub_0xb557ec() -> ! {
    todo!("0xb557ec __ZN3RBX7Network10Replicator10MarkerItemC1EPS1_l")
}

#[doc(alias = "RBX::Network::Replicator::MarkerItem::~MarkerItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator10MarkerItemD1Ev")]
// 0xb55b70 — __ZN3RBX7Network10Replicator10MarkerItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::MarkerItem *__hidden this)
pub fn stub_0xb55b70() -> ! {
    todo!("0xb55b70 __ZN3RBX7Network10Replicator10MarkerItemD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::MarkerItem::~MarkerItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator10MarkerItemD0Ev")]
// 0xb55b74 — __ZN3RBX7Network10Replicator10MarkerItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::MarkerItem *__hidden this)
pub fn stub_0xb55b74() -> ! {
    todo!("0xb55b74 __ZN3RBX7Network10Replicator10MarkerItemD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::PingBackItem::PingBackItem(RBX::Network::Replicator*,unsigned long long)")]
#[doc(alias = "__ZN3RBX7Network10Replicator12PingBackItemC1EPS1_y")]
// 0xb5621c — __ZN3RBX7Network10Replicator12PingBackItemC1EPS1_y
// type: int __fastcall(int this, RBX::Network::Replicator *, unsigned __int64)
pub fn stub_0xb5621c() -> ! {
    todo!("0xb5621c __ZN3RBX7Network10Replicator12PingBackItemC1EPS1_y")
}

#[doc(alias = "RBX::Network::Replicator::PingBackItem::~PingBackItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator12PingBackItemD1Ev")]
// 0xb562ac — __ZN3RBX7Network10Replicator12PingBackItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::PingBackItem *__hidden this)
pub fn stub_0xb562ac() -> ! {
    todo!("0xb562ac __ZN3RBX7Network10Replicator12PingBackItemD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::PingBackItem::~PingBackItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator12PingBackItemD0Ev")]
// 0xb562b0 — __ZN3RBX7Network10Replicator12PingBackItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::PingBackItem *__hidden this)
pub fn stub_0xb562b0() -> ! {
    todo!("0xb562b0 __ZN3RBX7Network10Replicator12PingBackItemD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::PingItem::PingItem(RBX::Network::Replicator*,unsigned long long)")]
#[doc(alias = "__ZN3RBX7Network10Replicator8PingItemC1EPS1_y")]
// 0xb56954 — __ZN3RBX7Network10Replicator8PingItemC1EPS1_y
// type: int __fastcall(int this, RBX::Network::Replicator *, unsigned __int64)
pub fn stub_0xb56954() -> ! {
    todo!("0xb56954 __ZN3RBX7Network10Replicator8PingItemC1EPS1_y")
}

#[doc(alias = "RBX::Network::Replicator::PingItem::~PingItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator8PingItemD1Ev")]
// 0xb569e4 — __ZN3RBX7Network10Replicator8PingItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::PingItem *__hidden this)
pub fn stub_0xb569e4() -> ! {
    todo!("0xb569e4 __ZN3RBX7Network10Replicator8PingItemD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::PingItem::~PingItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator8PingItemD0Ev")]
// 0xb569e8 — __ZN3RBX7Network10Replicator8PingItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::PingItem *__hidden this)
pub fn stub_0xb569e8() -> ! {
    todo!("0xb569e8 __ZN3RBX7Network10Replicator8PingItemD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::ItemSender::openPacket(void)")]
#[doc(alias = "__ZN3RBX7Network10Replicator10ItemSender10openPacketEv")]
// 0xb57cc0 — __ZN3RBX7Network10Replicator10ItemSender10openPacketEv
// type: void __fastcall(RBX::Network::Replicator::ItemSender *this)
pub fn stub_0xb57cc0() -> ! {
    todo!("0xb57cc0 __ZN3RBX7Network10Replicator10ItemSender10openPacketEv")
}

#[doc(alias = "RBX::Network::Replicator::ItemSender::closePacket(void)")]
#[doc(alias = "__ZN3RBX7Network10Replicator10ItemSender11closePacketEv")]
// 0xb57f2c — __ZN3RBX7Network10Replicator10ItemSender11closePacketEv
// type: void __fastcall(RBX::Network::Replicator::ItemSender *this)
pub fn stub_0xb57f2c() -> ! {
    todo!("0xb57f2c __ZN3RBX7Network10Replicator10ItemSender11closePacketEv")
}

#[doc(alias = "RBX::Network::Replicator::ItemSender::ItemSender(RBX::Network::Replicator&,RBX::Network::ConcurrentRakPeer *)")]
#[doc(alias = "__ZN3RBX7Network10Replicator10ItemSenderC1ERS1_PNS0_17ConcurrentRakPeerE")]
// 0xb58268 — __ZN3RBX7Network10Replicator10ItemSenderC1ERS1_PNS0_17ConcurrentRakPeerE
// type: RBX::Network::Replicator::ItemSender *__fastcall(RBX::Network::Replicator::ItemSender *this, RBX::Network::Replicator *, RBX::Network::ConcurrentRakPeer *)
pub fn stub_0xb58268() -> ! {
    todo!("0xb58268 __ZN3RBX7Network10Replicator10ItemSenderC1ERS1_PNS0_17ConcurrentRakPeerE")
}

#[doc(alias = "RBX::Network::Replicator::ItemSender::~ItemSender()")]
#[doc(alias = "__ZN3RBX7Network10Replicator10ItemSenderD1Ev")]
// 0xb58340 — __ZN3RBX7Network10Replicator10ItemSenderD1Ev
// type: void __fastcall(RBX::Network::Replicator::ItemSender *__hidden this)
pub fn stub_0xb58340() -> ! {
    todo!("0xb58340 __ZN3RBX7Network10Replicator10ItemSenderD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::ItemSender::send(RBX::Network::Item &)")]
#[doc(alias = "__ZN3RBX7Network10Replicator10ItemSender4sendERNS0_4ItemE")]
// 0xb58408 — __ZN3RBX7Network10Replicator10ItemSender4sendERNS0_4ItemE
// type: int __fastcall(RBX::Network::Replicator::ItemSender *this, int)
pub fn stub_0xb58408() -> ! {
    todo!("0xb58408 __ZN3RBX7Network10Replicator10ItemSender4sendERNS0_4ItemE")
}

#[doc(alias = "RBX::Network::Replicator::ItemSender::getNumberOfBytesUsed(void)const")]
#[doc(alias = "__ZNK3RBX7Network10Replicator10ItemSender20getNumberOfBytesUsedEv")]
// 0xb58444 — __ZNK3RBX7Network10Replicator10ItemSender20getNumberOfBytesUsedEv
// type: unsigned int __fastcall(RBX::Network::Replicator::ItemSender *this)
pub fn stub_0xb58444() -> ! {
    todo!("0xb58444 __ZNK3RBX7Network10Replicator10ItemSender20getNumberOfBytesUsedEv")
}

#[doc(alias = "RBX::Network::Replicator::ReferencePropertyChangedItem::~ReferencePropertyChangedItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator28ReferencePropertyChangedItemD1Ev")]
// 0xb58ea8 — __ZN3RBX7Network10Replicator28ReferencePropertyChangedItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::ReferencePropertyChangedItem *__hidden this)
pub fn stub_0xb58ea8() -> ! {
    todo!("0xb58ea8 __ZN3RBX7Network10Replicator28ReferencePropertyChangedItemD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::ReferencePropertyChangedItem::~ReferencePropertyChangedItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator28ReferencePropertyChangedItemD0Ev")]
// 0xb58ecc — __ZN3RBX7Network10Replicator28ReferencePropertyChangedItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::ReferencePropertyChangedItem *__hidden this)
pub fn stub_0xb58ecc() -> ! {
    todo!("0xb58ecc __ZN3RBX7Network10Replicator28ReferencePropertyChangedItemD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamRegionIterator::sortNextNRegions(unsigned int)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator16sortNextNRegionsEj")]
// 0xb599c8 — __ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator16sortNextNRegionsEj
// type: int __fastcall(RBX::Network::Replicator::StreamJob::StreamRegionIterator *this, unsigned int)
pub fn stub_0xb599c8() -> ! {
    todo!("0xb599c8 __ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator16sortNextNRegionsEj")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamRegionIterator::updateWorldExtents(void)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator18updateWorldExtentsEv")]
// 0xb59af0 — __ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator18updateWorldExtentsEv
// type: int __fastcall(RBX::Network::Replicator::StreamJob::StreamRegionIterator *this, int, int)
pub fn stub_0xb59af0() -> ! {
    todo!("0xb59af0 __ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator18updateWorldExtentsEv")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamRegionIterator::getNextRegion(RBX::StreamRegion::Id &)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator13getNextRegionERNS_12StreamRegion2IdE")]
// 0xb59db4 — __ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator13getNextRegionERNS_12StreamRegion2IdE
// type: int __fastcall(_DWORD *, _DWORD *)
pub fn stub_0xb59db4() -> ! {
    todo!("0xb59db4 __ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator13getNextRegionERNS_12StreamRegion2IdE")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamJob(RBX::Network::Replicator&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJobC1ERS1_")]
// 0xb59f98 — __ZN3RBX7Network10Replicator9StreamJobC1ERS1_
// type: int __fastcall(RBX::Network::Replicator::StreamJob *this, RBX::Network::Replicator *)
pub fn stub_0xb59f98() -> ! {
    todo!("0xb59f98 __ZN3RBX7Network10Replicator9StreamJobC1ERS1_")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamJob(RBX::Network::Replicator&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJobC2ERS1_")]
// 0xb59fa4 — __ZN3RBX7Network10Replicator9StreamJobC2ERS1_
// type: RBX::Network::Replicator::StreamJob *__fastcall(RBX::Network::Replicator::StreamJob *this, RBX::Network::Replicator *)
pub fn stub_0xb59fa4() -> ! {
    todo!("0xb59fa4 __ZN3RBX7Network10Replicator9StreamJobC2ERS1_")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::~StreamJob()")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJobD0Ev")]
// 0xb5a4b8 — __ZN3RBX7Network10Replicator9StreamJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::StreamJob *__hidden this)
pub fn stub_0xb5a4b8() -> ! {
    todo!("0xb5a4b8 __ZN3RBX7Network10Replicator9StreamJobD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::~StreamJob()")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJobD1Ev")]
// 0xb5a558 — __ZN3RBX7Network10Replicator9StreamJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::StreamJob *__hidden this)
pub fn stub_0xb5a558() -> ! {
    todo!("0xb5a558 __ZN3RBX7Network10Replicator9StreamJobD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::~StreamJob()")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJobD2Ev")]
// 0xb5a564 — __ZN3RBX7Network10Replicator9StreamJobD2Ev
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, int, int)
pub fn stub_0xb5a564() -> ! {
    todo!("0xb5a564 __ZN3RBX7Network10Replicator9StreamJobD2Ev")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::clearPendingItems(void)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob17clearPendingItemsEv")]
// 0xb5a950 — __ZN3RBX7Network10Replicator9StreamJob17clearPendingItemsEv
// type: int *__fastcall(RBX::Network::Replicator::StreamJob *this)
pub fn stub_0xb5a950() -> ! {
    todo!("0xb5a950 __ZN3RBX7Network10Replicator9StreamJob17clearPendingItemsEv")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::updateClientQuota(int,short)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob17updateClientQuotaEis")]
// 0xb5aa14 — __ZN3RBX7Network10Replicator9StreamJob17updateClientQuotaEis
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, int, const void *)
pub fn stub_0xb5aa14() -> ! {
    todo!("0xb5aa14 __ZN3RBX7Network10Replicator9StreamJob17updateClientQuotaEis")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::isInitialDataCollected(void)const")]
#[doc(alias = "__ZNK3RBX7Network10Replicator9StreamJob22isInitialDataCollectedEv")]
// 0xb5ad04 — __ZNK3RBX7Network10Replicator9StreamJob22isInitialDataCollectedEv
// type: int __fastcall(RBX::Network::Replicator::StreamJob *this)
pub fn stub_0xb5ad04() -> ! {
    todo!("0xb5ad04 __ZNK3RBX7Network10Replicator9StreamJob22isInitialDataCollectedEv")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::isRegionInPendingStreamItemQueue(RBX::StreamRegion::Id const&)const")]
#[doc(alias = "__ZNK3RBX7Network10Replicator9StreamJob32isRegionInPendingStreamItemQueueERKNS_12StreamRegion2IdE")]
// 0xb5adec — __ZNK3RBX7Network10Replicator9StreamJob32isRegionInPendingStreamItemQueueERKNS_12StreamRegion2IdE
// type: int __fastcall(_DWORD *, _DWORD *)
pub fn stub_0xb5adec() -> ! {
    todo!("0xb5adec __ZNK3RBX7Network10Replicator9StreamJob32isRegionInPendingStreamItemQueueERKNS_12StreamRegion2IdE")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::isInStreamedRegions(RBX::Extents const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob19isInStreamedRegionsERKNS_7ExtentsE")]
// 0xb5ae38 — __ZN3RBX7Network10Replicator9StreamJob19isInStreamedRegionsERKNS_7ExtentsE
// type: int __fastcall(RBX::Network::Replicator::StreamJob *this, const RBX::Extents *)
pub fn stub_0xb5ae38() -> ! {
    todo!("0xb5ae38 __ZN3RBX7Network10Replicator9StreamJob19isInStreamedRegionsERKNS_7ExtentsE")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::setupListeners(RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob14setupListenersEPNS0_6PlayerE")]
// 0xb5b71c — __ZN3RBX7Network10Replicator9StreamJob14setupListenersEPNS0_6PlayerE
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, RBX::Network::Player *)
pub fn stub_0xb5b71c() -> ! {
    todo!("0xb5b71c __ZN3RBX7Network10Replicator9StreamJob14setupListenersEPNS0_6PlayerE")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::adjustSimulationOwnershipRange(RBX::Region2::WeightedPoint *)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob30adjustSimulationOwnershipRangeEPNS_7Region213WeightedPointE")]
// 0xb5bf08 — __ZN3RBX7Network10Replicator9StreamJob30adjustSimulationOwnershipRangeEPNS_7Region213WeightedPointE
// type: unsigned __int32 __fastcall(int, int)
pub fn stub_0xb5bf08() -> ! {
    todo!("0xb5bf08 __ZN3RBX7Network10Replicator9StreamJob30adjustSimulationOwnershipRangeEPNS_7Region213WeightedPointE")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob5errorERKNS_13TaskScheduler3Job5StatsE")]
// 0xb5bf90 — __ZN3RBX7Network10Replicator9StreamJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
pub fn stub_0xb5bf90() -> ! {
    todo!("0xb5bf90 __ZN3RBX7Network10Replicator9StreamJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::sendPackets(int)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob11sendPacketsEi")]
// 0xb5d1a4 — __ZN3RBX7Network10Replicator9StreamJob11sendPacketsEi
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, int)
pub fn stub_0xb5d1a4() -> ! {
    todo!("0xb5d1a4 __ZN3RBX7Network10Replicator9StreamJob11sendPacketsEi")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::coarsePrimitiveMovement(RBX::Primitive *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback::UpdateInfo const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE")]
// 0xb5e218 — __ZN3RBX7Network10Replicator9StreamJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE
// type: void __fastcall(int, int, _DWORD *, int, pthread_mutex_t *, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, void *, int, int, int, int, int)
pub fn stub_0xb5e218() -> ! {
    todo!("0xb5e218 __ZN3RBX7Network10Replicator9StreamJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::StreamJob::coarsePrimitiveMovement(RBX::Primitive *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback::UpdateInfo const&)")]
#[doc(alias = "__ZThn488_N3RBX7Network10Replicator9StreamJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE")]
// 0xb5eb30 — __ZThn488_N3RBX7Network10Replicator9StreamJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE
// type: 
pub fn stub_0xb5eb30() -> ! {
    todo!("0xb5eb30 __ZThn488_N3RBX7Network10Replicator9StreamJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::unregisterCoarseMovementCallback(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback *)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE32unregisterCoarseMovementCallbackEPNS4_22CoarseMovementCallbackE")]
// 0xb5eb40 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE32unregisterCoarseMovementCallbackEPNS4_22CoarseMovementCallbackE
// type: int __fastcall(int, int)
pub fn stub_0xb5eb40() -> ! {
    todo!("0xb5eb40 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE32unregisterCoarseMovementCallbackEPNS4_22CoarseMovementCallbackE")
}

#[doc(alias = "bool RBX::StreamRegion::IdExtents::intersectsContainer<boost::unordered::unordered_set<RBX::StreamRegion::Id,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>,std::allocator<RBX::StreamRegion::Id>>>(boost::unordered::unordered_set<RBX::StreamRegion::Id,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>,std::allocator<RBX::StreamRegion::Id>> const&,RBX::StreamRegion::Id*)const")]
#[doc(alias = "__ZNK3RBX12StreamRegion9IdExtents19intersectsContainerIN5boost9unordered13unordered_setINS0_2IdENS6_27boost_compatible_hash_valueESt8equal_toIS6_ESaIS6_EEEEEbRKT_PS6_")]
// 0xb5ec24 — __ZNK3RBX12StreamRegion9IdExtents19intersectsContainerIN5boost9unordered13unordered_setINS0_2IdENS6_27boost_compatible_hash_valueESt8equal_toIS6_ESaIS6_EEEEEbRKT_PS6_
// type: int __fastcall(int *, int, _DWORD *)
pub fn stub_0xb5ec24() -> ! {
    todo!("0xb5ec24 __ZNK3RBX12StreamRegion9IdExtents19intersectsContainerIN5boost9unordered13unordered_setINS0_2IdENS6_27boost_compatible_hash_valueESt8equal_toIS6_ESaIS6_EEEEEbRKT_PS6_")
}

#[doc(alias = "std::deque<RBX::Network::Replicator::StreamJob::StreamDataItem *,std::allocator<RBX::Network::Replicator::StreamJob::StreamDataItem *>>::erase(std::_Deque_iterator<RBX::Network::Replicator::StreamJob::StreamDataItem *,RBX::Network::Replicator::StreamJob::StreamDataItem *&,RBX::Network::Replicator::StreamJob::StreamDataItem **>)")]
#[doc(alias = "__ZNSt5dequeIPN3RBX7Network10Replicator9StreamJob14StreamDataItemESaIS5_EE5eraseESt15_Deque_iteratorIS5_RS5_PS5_E")]
// 0xb5ed30 — __ZNSt5dequeIPN3RBX7Network10Replicator9StreamJob14StreamDataItemESaIS5_EE5eraseESt15_Deque_iteratorIS5_RS5_PS5_E
// type: _DWORD *__fastcall(_DWORD *, int, int **)
pub fn stub_0xb5ed30() -> ! {
    todo!("0xb5ed30 __ZNSt5dequeIPN3RBX7Network10Replicator9StreamJob14StreamDataItemESaIS5_EE5eraseESt15_Deque_iteratorIS5_RS5_PS5_E")
}

#[doc(alias = "void RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getPrimitivesOverlapping<RBX::DenseHashSet<RBX::Primitive*,boost::hash<RBX::Primitive*>,std::allocator<RBX::Primitive*>>>(RBX::Extents const&,RBX::DenseHashSet<RBX::Primitive*,boost::hash<RBX::Primitive*>,std::allocator<RBX::Primitive*>> &)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE24getPrimitivesOverlappingINS_12DenseHashSetIPS1_N5boost4hashIS7_EESaIS7_EEEEEvRKNS_7ExtentsERT_")]
// 0xb5f1b8 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE24getPrimitivesOverlappingINS_12DenseHashSetIPS1_N5boost4hashIS7_EESaIS7_EEEEEvRKNS_7ExtentsERT_
// type: RBX::SpatialHashStatic *__fastcall(int, int, int)
pub fn stub_0xb5f1b8() -> ! {
    todo!("0xb5f1b8 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE24getPrimitivesOverlappingINS_12DenseHashSetIPS1_N5boost4hashIS7_EESaIS7_EEEEEvRKNS_7ExtentsERT_")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
// 0xb5f4dc — __ZN3RBX7Network10Replicator9StreamJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_0xb5f4dc() -> ! {
    todo!("0xb5f4dc __ZN3RBX7Network10Replicator9StreamJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "non-virtual thunk toRBX::ObjectValue::~ObjectValue()")]
#[doc(alias = "__ZThn32_N3RBX11ObjectValueD1Ev")]
// 0xb5f510 — __ZThn32_N3RBX11ObjectValueD1Ev
// type: void __fastcall(RBX::ObjectValue *__hidden this)
pub fn stub_0xb5f510() -> ! {
    todo!("0xb5f510 __ZThn32_N3RBX11ObjectValueD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ObjectValue::~ObjectValue()")]
#[doc(alias = "__ZThn36_N3RBX11ObjectValueD0Ev")]
// 0xb5f590 — __ZThn36_N3RBX11ObjectValueD0Ev
// type: void __fastcall(RBX::ObjectValue *__hidden this)
pub fn stub_0xb5f590() -> ! {
    todo!("0xb5f590 __ZThn36_N3RBX11ObjectValueD0Ev")
}

#[doc(alias = "RBX::StringValue::~StringValue()")]
#[doc(alias = "__ZN3RBX11StringValueD1Ev")]
// 0xb5fc30 — __ZN3RBX11StringValueD1Ev
// type: void __fastcall(RBX::StringValue *__hidden this)
pub fn stub_0xb5fc30() -> ! {
    todo!("0xb5fc30 __ZN3RBX11StringValueD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StringValue::~StringValue()")]
#[doc(alias = "__ZThn32_N3RBX11StringValueD1Ev")]
// 0xb5fc48 — __ZThn32_N3RBX11StringValueD1Ev
// type: void __fastcall(RBX::StringValue *__hidden this)
pub fn stub_0xb5fc48() -> ! {
    todo!("0xb5fc48 __ZThn32_N3RBX11StringValueD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StringValue::~StringValue()")]
#[doc(alias = "__ZThn36_N3RBX11StringValueD0Ev")]
// 0xb5fcc8 — __ZThn36_N3RBX11StringValueD0Ev
// type: void __fastcall(RBX::StringValue *__hidden this)
pub fn stub_0xb5fcc8() -> ! {
    todo!("0xb5fcc8 __ZThn36_N3RBX11StringValueD0Ev")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::StreamRegion::Id>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::StreamRegion::Id>,RBX::StreamRegion::Id,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::StreamRegion::Id>>(RBX::StreamRegion::Id const&,boost::unordered::detail::emplace_args1<RBX::StreamRegion::Id> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_")]
// 0xb5ff10 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
// type: int __fastcall(_DWORD *, _DWORD *, _DWORD *, __int64 **)
pub fn stub_0xb5ff10() -> ! {
    todo!("0xb5ff10 __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::StreamRegion::Id>,RBX::StreamRegion::Id,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE18reserve_for_insertEm")]
// 0xb60130 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
pub fn stub_0xb60130() -> ! {
    todo!("0xb60130 __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::StreamRegion::Id>,RBX::StreamRegion::Id,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14create_bucketsEm")]
// 0xb602d8 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
pub fn stub_0xb602d8() -> ! {
    todo!("0xb602d8 __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14create_bucketsEm")
}

#[doc(alias = "std::deque<RBX::Network::Replicator::StreamJob::StreamDataItem *,std::allocator<RBX::Network::Replicator::StreamJob::StreamDataItem *>>::_M_reallocate_map(unsigned long,bool)")]
#[doc(alias = "__ZNSt5dequeIPN3RBX7Network10Replicator9StreamJob14StreamDataItemESaIS5_EE17_M_reallocate_mapEmb")]
// 0xb60388 — __ZNSt5dequeIPN3RBX7Network10Replicator9StreamJob14StreamDataItemESaIS5_EE17_M_reallocate_mapEmb
// type: char *__fastcall(void **, unsigned int, int)
pub fn stub_0xb60388() -> ! {
    todo!("0xb60388 __ZNSt5dequeIPN3RBX7Network10Replicator9StreamJob14StreamDataItemESaIS5_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::insert(RBX::Primitive * const&)")]
#[doc(alias = "__ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE6insertERKS2_")]
// 0xb60460 — __ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE6insertERKS2_
// type: 
pub fn stub_0xb60460() -> ! {
    todo!("0xb60460 __ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE6insertERKS2_")
}

#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::rehash(void)")]
#[doc(alias = "__ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE6rehashEv")]
// 0xb60570 — __ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE6rehashEv
// type: void __fastcall(__int64 *, int, int, int, void *, int, int, int, int, int, int, int, int, int)
pub fn stub_0xb60570() -> ! {
    todo!("0xb60570 __ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE6rehashEv")
}

#[doc(alias = "RBX::SpatialHashStatic::safeExtents(RBX::Extents const&)")]
#[doc(alias = "__ZN3RBX17SpatialHashStatic11safeExtentsERKNS_7ExtentsE")]
// 0xb606e8 — __ZN3RBX17SpatialHashStatic11safeExtentsERKNS_7ExtentsE
// type: void __fastcall(RBX::SpatialHashStatic *this, const RBX::Extents *)
pub fn stub_0xb606e8() -> ! {
    todo!("0xb606e8 __ZN3RBX17SpatialHashStatic11safeExtentsERKNS_7ExtentsE")
}

#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::DenseHashSet(RBX::Primitive * const&,unsigned long,boost::hash<RBX::Primitive *> const&)")]
#[doc(alias = "__ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EEC2ERKS2_mRKS5_")]
// 0xb60908 — __ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EEC2ERKS2_mRKS5_
// type: int *__fastcall(int *, int *, unsigned int)
pub fn stub_0xb60908() -> ! {
    todo!("0xb60908 __ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EEC2ERKS2_mRKS5_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::StreamRegion::Id>,RBX::StreamRegion::Id,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::erase_key(RBX::StreamRegion::Id const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE9erase_keyERKS6_")]
// 0xb621bc — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE9erase_keyERKS6_
// type: int __fastcall(_DWORD *, _DWORD *)
pub fn stub_0xb621bc() -> ! {
    todo!("0xb621bc __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX12StreamRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE9erase_keyERKS6_")
}

#[doc(alias = "std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback *,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback **,std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback *,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback *>>>,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE22CoarseMovementCallbackESaIS7_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_S9_EERKS7_")]
// 0xb622b8 — __ZNSt6vectorIPN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE22CoarseMovementCallbackESaIS7_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_S9_EERKS7_
// type: void *__fastcall(int, char *__src, _DWORD *)
pub fn stub_0xb622b8() -> ! {
    todo!("0xb622b8 __ZNSt6vectorIPN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE22CoarseMovementCallbackESaIS7_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_S9_EERKS7_")
}

#[doc(alias = "std::_Deque_base<RBX::Network::Replicator::StreamJob::StreamDataItem *,std::allocator<RBX::Network::Replicator::StreamJob::StreamDataItem *>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN3RBX7Network10Replicator9StreamJob14StreamDataItemESaIS5_EE17_M_initialize_mapEm")]
// 0xb623b0 — __ZNSt11_Deque_baseIPN3RBX7Network10Replicator9StreamJob14StreamDataItemESaIS5_EE17_M_initialize_mapEm
// type: void __fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, void *, int)
pub fn stub_0xb623b0() -> ! {
    todo!("0xb623b0 __ZNSt11_Deque_baseIPN3RBX7Network10Replicator9StreamJob14StreamDataItemESaIS5_EE17_M_initialize_mapEm")
}
