//! core shard ou — 100 core stubs EA-sorted, 0x93f220..0xa2bb50 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::SceneUpdater::updateAllInvalidAttachements(bool)")]
// 0x93f220 — __ZN3RBX12SceneUpdater28updateAllInvalidAttachementsEb
// type: _DWORD __fastcall(RBX::SceneUpdater *__hidden this, bool)
pub fn stub_0x93f220() -> ! {
    todo!("0x93f220 __ZN3RBX12SceneUpdater28updateAllInvalidAttachementsEb")
}

#[doc(alias = "RBX::SceneUpdater::createAllAttachements(void)")]
// 0x93f818 — __ZN3RBX12SceneUpdater21createAllAttachementsEv
// type: _DWORD __fastcall(RBX::SceneUpdater *__hidden this)
pub fn stub_0x93f818() -> ! {
    todo!("0x93f818 __ZN3RBX12SceneUpdater21createAllAttachementsEv")
}

#[doc(alias = "RBX::SceneUpdater::removeMegaClusters(void)")]
// 0x93ff94 — __ZN3RBX12SceneUpdater18removeMegaClustersEv
// type: _DWORD __fastcall(RBX::SceneUpdater *__hidden this)
pub fn stub_0x93ff94() -> ! {
    todo!("0x93ff94 __ZN3RBX12SceneUpdater18removeMegaClustersEv")
}

#[doc(alias = "RBX::SceneUpdater::updateMegaClusters(void)")]
// 0x940d50 — __ZN3RBX12SceneUpdater18updateMegaClustersEv
// type: _DWORD __fastcall(RBX::SceneUpdater *__hidden this)
pub fn stub_0x940d50() -> ! {
    todo!("0x940d50 __ZN3RBX12SceneUpdater18updateMegaClustersEv")
}

#[doc(alias = "RBX::SceneUpdater::updateInvalidatedFastClusters(bool)")]
// 0x941290 — __ZN3RBX12SceneUpdater29updateInvalidatedFastClustersEb
// type: _DWORD __fastcall(RBX::SceneUpdater *__hidden this, bool)
pub fn stub_0x941290() -> ! {
    todo!("0x941290 __ZN3RBX12SceneUpdater29updateInvalidatedFastClustersEb")
}

#[doc(alias = "RBX::SceneUpdater::updateDynamicAttachements(void)")]
// 0x941cbc — __ZN3RBX12SceneUpdater25updateDynamicAttachementsEv
// type: _DWORD __fastcall(RBX::SceneUpdater *__hidden this)
pub fn stub_0x941cbc() -> ! {
    todo!("0x941cbc __ZN3RBX12SceneUpdater25updateDynamicAttachementsEv")
}

#[doc(alias = "RBX::SceneUpdater::processPendingMegaClusters(void)")]
// 0x941e98 — __ZN3RBX12SceneUpdater26processPendingMegaClustersEv
// type: _DWORD __fastcall(RBX::SceneUpdater *__hidden this)
pub fn stub_0x941e98() -> ! {
    todo!("0x941e98 __ZN3RBX12SceneUpdater26processPendingMegaClustersEv")
}

#[doc(alias = "RBX::SceneUpdater::update(unsigned long,RBX::Frustum const&)")]
// 0x9424cc — __ZN3RBX12SceneUpdater6updateEmRKNS_7FrustumE
// type: _DWORD __fastcall(RBX::SceneUpdater *__hidden this, unsigned int, const RBX::Frustum *)
pub fn stub_0x9424cc() -> ! {
    todo!("0x9424cc __ZN3RBX12SceneUpdater6updateEmRKNS_7FrustumE")
}

#[doc(alias = "RBX::SpatialRegion::centerOfRegionInGlobalCoordStuds(RBX::SpatialRegion::Id const&)")]
// 0x942af8 — __ZN3RBX13SpatialRegion32centerOfRegionInGlobalCoordStudsERKNS0_2IdE
// type: 
pub fn stub_0x942af8() -> ! {
    todo!("0x942af8 __ZN3RBX13SpatialRegion32centerOfRegionInGlobalCoordStudsERKNS0_2IdE")
}

#[doc(alias = "std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>::erase(__gnu_cxx::__normal_iterator<RBX::ContentId*,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId*,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x942cb0 — __ZNSt6vectorIN3RBX9ContentIdESaIS1_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS1_S3_EES7_
// type: int __fastcall(int, std::string *this, std::string *)
pub fn stub_0x942cb0() -> ! {
    todo!("0x942cb0 __ZNSt6vectorIN3RBX9ContentIdESaIS1_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS1_S3_EES7_")
}

#[doc(alias = "RBX::SceneUpdater::checkFastClusters(void)")]
// 0x943028 — __ZN3RBX12SceneUpdater17checkFastClustersEv
// type: _DWORD __fastcall(RBX::SceneUpdater *__hidden this)
pub fn stub_0x943028() -> ! {
    todo!("0x943028 __ZN3RBX12SceneUpdater17checkFastClustersEv")
}

#[doc(alias = "RBX::SceneUpdater::computeLighting(bool)")]
// 0x94302c — __ZN3RBX12SceneUpdater15computeLightingEb
// type: _DWORD __fastcall(RBX::SceneUpdater *__hidden this, bool)
pub fn stub_0x94302c() -> ! {
    todo!("0x94302c __ZN3RBX12SceneUpdater15computeLightingEb")
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,int>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,int)")]
// 0x945a8c — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEiEvT_S9_T0_
// type: int __fastcall(int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0x945a8c() -> ! {
    todo!("0x945a8c __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEiEvT_S9_T0_")
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x945ca0 — __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x945ca0() -> ! {
    todo!("0x945ca0 __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_")
}

#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x945e60 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x945e60() -> ! {
    todo!("0x945e60 __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>> std::__unguarded_partition<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,RBX::ContentId>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,RBX::ContentId)")]
// 0x9460e0 — __ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEES3_ET_S9_S9_T0_
// type: 
pub fn stub_0x9460e0() -> ! {
    todo!("0x9460e0 __ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEES3_ET_S9_S9_T0_")
}

#[doc(alias = "void std::__iter_swap<true>::iter_swap<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x946128 — __ZNSt11__iter_swapILb1EE9iter_swapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS5_SaIS5_EEEESA_EEvT_T0_
// type: int __fastcall(std::string *)
pub fn stub_0x946128() -> ! {
    todo!("0x946128 __ZNSt11__iter_swapILb1EE9iter_swapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS5_SaIS5_EEEESA_EEvT_T0_")
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x946264 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_S9_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, char, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x946264() -> ! {
    todo!("0x946264 __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_S9_")
}

#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x946454 — __ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_
// type: 
pub fn stub_0x946454() -> ! {
    todo!("0x946454 __ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_")
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,int,RBX::ContentId>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,int,int,RBX::ContentId)")]
// 0x946624 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEiS3_EvT_T0_SA_T1_
// type: 
pub fn stub_0x946624() -> ! {
    todo!("0x946624 __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEiS3_EvT_T0_SA_T1_")
}

#[doc(alias = "void std::make_heap<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x946850 — __ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0x946850() -> ! {
    todo!("0x946850 __ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_")
}

#[doc(alias = "std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>::vector(std::vector<RBX::ContentId,std::allocator<RBX::ContentId>> const&)")]
// 0x9469a4 — __ZNSt6vectorIN3RBX9ContentIdESaIS1_EEC2ERKS3_
// type: 
pub fn stub_0x9469a4() -> ! {
    todo!("0x9469a4 __ZNSt6vectorIN3RBX9ContentIdESaIS1_EEC2ERKS3_")
}

#[doc(alias = "std::vector<RBX::SceneUpdater::MegaClusterChunk,std::allocator<RBX::SceneUpdater::MegaClusterChunk>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SceneUpdater::MegaClusterChunk*,std::vector<RBX::SceneUpdater::MegaClusterChunk,std::allocator<RBX::SceneUpdater::MegaClusterChunk>>>,RBX::SceneUpdater::MegaClusterChunk const&)")]
// 0x946f50 — __ZNSt6vectorIN3RBX12SceneUpdater16MegaClusterChunkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: 
pub fn stub_0x946f50() -> ! {
    todo!("0x946f50 __ZNSt6vectorIN3RBX12SceneUpdater16MegaClusterChunkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "RBX::CircleRadialNormal::eval(float)")]
// 0x94ec20 — __ZN3RBX18CircleRadialNormal4evalEf
// type: _DWORD __fastcall(RBX::CircleRadialNormal *__hidden this, float)
pub fn stub_0x94ec20() -> ! {
    todo!("0x94ec20 __ZN3RBX18CircleRadialNormal4evalEf")
}

#[doc(alias = "RBX::CircleRadialNormal::evalTangent(float)")]
// 0x94ece0 — __ZN3RBX18CircleRadialNormal11evalTangentEf
// type: _DWORD __fastcall(RBX::CircleRadialNormal *__hidden this, float)
pub fn stub_0x94ece0() -> ! {
    todo!("0x94ece0 __ZN3RBX18CircleRadialNormal11evalTangentEf")
}

#[doc(alias = "RBX::CircleRadialNormal::evalNormal(float)")]
// 0x94ed88 — __ZN3RBX18CircleRadialNormal10evalNormalEf
// type: _DWORD __fastcall(RBX::CircleRadialNormal *__hidden this, float)
pub fn stub_0x94ed88() -> ! {
    todo!("0x94ed88 __ZN3RBX18CircleRadialNormal10evalNormalEf")
}

#[doc(alias = "RBX::CircleRadialNormal::evalBinormal(float)")]
// 0x94ee30 — __ZN3RBX18CircleRadialNormal12evalBinormalEf
// type: _DWORD __fastcall(RBX::CircleRadialNormal *__hidden this, float)
pub fn stub_0x94ee30() -> ! {
    todo!("0x94ee30 __ZN3RBX18CircleRadialNormal12evalBinormalEf")
}

#[doc(alias = "RBX::CircleRadialNormal::hashString(void)")]
// 0x94ee5c — __ZN3RBX18CircleRadialNormal10hashStringEv
// type: _DWORD __fastcall(RBX::CircleRadialNormal *__hidden this)
pub fn stub_0x94ee5c() -> ! {
    todo!("0x94ee5c __ZN3RBX18CircleRadialNormal10hashStringEv")
}

#[doc(alias = "RBX::isDebuggerPresentFast(void)")]
// 0x9573bc — __ZN3RBX21isDebuggerPresentFastEv
// type: _DWORD __fastcall(RBX *__hidden this)
pub fn stub_0x9573bc() -> ! {
    todo!("0x9573bc __ZN3RBX21isDebuggerPresentFastEv")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned int>::deref(unsigned int const*)")]
// 0x9842a8 — __ZN3RBX5Stats14TypedStatsItemIjE5derefEPKj
// type: void()
pub fn stub_0x9842a8() -> ! {
    todo!("0x9842a8 __ZN3RBX5Stats14TypedStatsItemIjE5derefEPKj")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x9842b0 — __ZN3RBX5Stats14TypedStatsItemIjED0Ev
// type: void __fastcall(void *)
pub fn stub_0x9842b0() -> ! {
    todo!("0x9842b0 __ZN3RBX5Stats14TypedStatsItemIjED0Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned int>::update(void)")]
// 0x984350 — __ZN3RBX5Stats14TypedStatsItemIjE6updateEv
// type: void __fastcall(int)
pub fn stub_0x984350() -> ! {
    todo!("0x984350 __ZN3RBX5Stats14TypedStatsItemIjE6updateEv")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x9844d0 — __ZThn32_N3RBX5Stats14TypedStatsItemIjED0Ev
// type: void __fastcall(int)
pub fn stub_0x9844d0() -> ! {
    todo!("0x9844d0 __ZThn32_N3RBX5Stats14TypedStatsItemIjED0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x984578 — __ZThn36_N3RBX5Stats14TypedStatsItemIjED0Ev
// type: void __fastcall(int)
pub fn stub_0x984578() -> ! {
    todo!("0x984578 __ZThn36_N3RBX5Stats14TypedStatsItemIjED0Ev")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<unsigned long long>(char const*,unsigned long long const&)")]
// 0x984b88 — __ZN3RBX5Stats4Item20createBoundChildItemIyEEPS1_PKcRKT_
// type: RBX::Instance *__fastcall(pthread_mutex_t *, pthread_mutex_t *, int)
pub fn stub_0x984b88() -> ! {
    todo!("0x984b88 __ZN3RBX5Stats4Item20createBoundChildItemIyEEPS1_PKcRKT_")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<float>(char const*,float const&)")]
// 0x9851e0 — __ZN3RBX5Stats4Item20createBoundChildItemIfEEPS1_PKcRKT_
// type: RBX::Instance *__fastcall(pthread_mutex_t *, pthread_mutex_t *, int)
pub fn stub_0x9851e0() -> ! {
    todo!("0x9851e0 __ZN3RBX5Stats4Item20createBoundChildItemIfEEPS1_PKcRKT_")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<float>::~TypedStatsItem()")]
// 0x985ad8 — __ZThn36_N3RBX5Stats14TypedStatsItemIfED1Ev
// type: int __fastcall(int)
pub fn stub_0x985ad8() -> ! {
    todo!("0x985ad8 __ZThn36_N3RBX5Stats14TypedStatsItemIfED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<float>::~TypedStatsItem()")]
// 0x985ae8 — __ZN3RBX5Stats14TypedStatsItemIfED2Ev
// type: int __fastcall(RBX::Instance *)
pub fn stub_0x985ae8() -> ! {
    todo!("0x985ae8 __ZN3RBX5Stats14TypedStatsItemIfED2Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<double>::deref(double const*)")]
// 0x986000 — __ZN3RBX5Stats14TypedStatsItemIdE5derefEPKd
// type: void()
pub fn stub_0x986000() -> ! {
    todo!("0x986000 __ZN3RBX5Stats14TypedStatsItemIdE5derefEPKd")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<double>::update(void)")]
// 0x986008 — __ZN3RBX5Stats14TypedStatsItemIdE6updateEv
// type: void __fastcall(int)
pub fn stub_0x986008() -> ! {
    todo!("0x986008 __ZN3RBX5Stats14TypedStatsItemIdE6updateEv")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
// 0x986188 — __ZThn36_N3RBX5Stats14TypedStatsItemIdED0Ev
// type: void __fastcall(int)
pub fn stub_0x986188() -> ! {
    todo!("0x986188 __ZThn36_N3RBX5Stats14TypedStatsItemIdED0Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<bool>::deref(bool const*)")]
// 0x986258 — __ZN3RBX5Stats14TypedStatsItemIbE5derefEPKb
// type: void()
pub fn stub_0x986258() -> ! {
    todo!("0x986258 __ZN3RBX5Stats14TypedStatsItemIbE5derefEPKb")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<bool>::update(void)")]
// 0x986260 — __ZN3RBX5Stats14TypedStatsItemIbE6updateEv
// type: void __fastcall(int)
pub fn stub_0x986260() -> ! {
    todo!("0x986260 __ZN3RBX5Stats14TypedStatsItemIbE6updateEv")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()")]
// 0x9863e0 — __ZThn32_N3RBX5Stats14TypedStatsItemIbED0Ev
// type: void __fastcall(int)
pub fn stub_0x9863e0() -> ! {
    todo!("0x9863e0 __ZThn32_N3RBX5Stats14TypedStatsItemIbED0Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long long>::deref(unsigned long long const*)")]
// 0x9864b0 — __ZN3RBX5Stats14TypedStatsItemIyE5derefEPKy
// type: void()
pub fn stub_0x9864b0() -> ! {
    todo!("0x9864b0 __ZN3RBX5Stats14TypedStatsItemIyE5derefEPKy")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x9864b8 — __ZN3RBX5Stats14TypedStatsItemIyED0Ev
// type: void __fastcall(void *)
pub fn stub_0x9864b8() -> ! {
    todo!("0x9864b8 __ZN3RBX5Stats14TypedStatsItemIyED0Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long long>::update(void)")]
// 0x986558 — __ZN3RBX5Stats14TypedStatsItemIyE6updateEv
// type: void __fastcall(int)
pub fn stub_0x986558() -> ! {
    todo!("0x986558 __ZN3RBX5Stats14TypedStatsItemIyE6updateEv")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x9866d8 — __ZThn32_N3RBX5Stats14TypedStatsItemIyED0Ev
// type: void __fastcall(int)
pub fn stub_0x9866d8() -> ! {
    todo!("0x9866d8 __ZThn32_N3RBX5Stats14TypedStatsItemIyED0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x986780 — __ZThn36_N3RBX5Stats14TypedStatsItemIyED0Ev
// type: void __fastcall(int)
pub fn stub_0x986780() -> ! {
    todo!("0x986780 __ZThn36_N3RBX5Stats14TypedStatsItemIyED0Ev")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Name const* const,unsigned char>>,RBX::Name const*,unsigned char,boost::hash<RBX::Name const*>,std::equal_to<RBX::Name const*>>>::erase_key(RBX::Name const* const&)")]
// 0x9a2a5c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX4NameEhEES8_hNS_4hashIS8_EESt8equal_toIS8_EEEE9erase_keyERS9_
// type: int __fastcall(_DWORD *, unsigned int *)
pub fn stub_0x9a2a5c() -> ! {
    todo!("0x9a2a5c __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX4NameEhEES8_hNS_4hashIS8_EESt8equal_toIS8_EEEE9erase_keyERS9_")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Name const* const,unsigned char>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Name const* const,unsigned char>>,RBX::Name const*,unsigned char,boost::hash<RBX::Name const*>,std::equal_to<RBX::Name const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Name const* const,unsigned char>>>(RBX::Name const* const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Name const* const,unsigned char>> const&)")]
// 0x9a2b2c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX4NameEhEES8_hNS_4hashIS8_EESt8equal_toIS8_EEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS9_RKT_
// type: void __fastcall(int, _DWORD *, unsigned int *, _DWORD **, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x9a2b2c() -> ! {
    todo!("0x9a2b2c __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX4NameEhEES8_hNS_4hashIS8_EESt8equal_toIS8_EEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS9_RKT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Name const* const,unsigned char>>,RBX::Name const*,unsigned char,boost::hash<RBX::Name const*>,std::equal_to<RBX::Name const*>>>::reserve_for_insert(unsigned long)")]
// 0x9a2d00 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX4NameEhEES8_hNS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
pub fn stub_0x9a2d00() -> ! {
    todo!("0x9a2d00 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX4NameEhEES8_hNS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Name const* const,unsigned char>>,RBX::Name const*,unsigned char,boost::hash<RBX::Name const*>,std::equal_to<RBX::Name const*>>>::create_buckets(unsigned long)")]
// 0x9a2ea8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX4NameEhEES8_hNS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
pub fn stub_0x9a2ea8() -> ! {
    todo!("0x9a2ea8 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX4NameEhEES8_hNS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm")
}

#[doc(alias = "RBX::PhysicsService::begin(void)")]
// 0x9a8ee0 — __ZN3RBX14PhysicsService5beginEv
// type: _DWORD *__fastcall(RBX::PhysicsService *this)
pub fn stub_0x9a8ee0() -> ! {
    todo!("0x9a8ee0 __ZN3RBX14PhysicsService5beginEv")
}

#[doc(alias = "RBX::MechanismItem::~MechanismItem()")]
// 0x9ae8a8 — __ZN3RBX13MechanismItemD1Ev
// type: void __fastcall(RBX::MechanismItem *__hidden this)
pub fn stub_0x9ae8a8() -> ! {
    todo!("0x9ae8a8 __ZN3RBX13MechanismItemD1Ev")
}

#[doc(alias = "RBX::MechanismItem::~MechanismItem()")]
// 0x9ae8b4 — __ZN3RBX13MechanismItemD2Ev
// type: void __fastcall(RBX::MechanismItem *__hidden this)
pub fn stub_0x9ae8b4() -> ! {
    todo!("0x9ae8b4 __ZN3RBX13MechanismItemD2Ev")
}

#[doc(alias = "RBX::MechanismItem::reset(int)")]
// 0x9ae9d0 — __ZN3RBX13MechanismItem5resetEi
// type: void __fastcall(RBX::MechanismItem *this, int)
pub fn stub_0x9ae9d0() -> ! {
    todo!("0x9ae9d0 __ZN3RBX13MechanismItem5resetEi")
}

#[doc(alias = "RBX::MechanismItem::appendAssembly(void)")]
// 0x9aeaa8 — __ZN3RBX13MechanismItem14appendAssemblyEv
// type: int __fastcall(RBX::MechanismItem *this, int, int)
pub fn stub_0x9aeaa8() -> ! {
    todo!("0x9aeaa8 __ZN3RBX13MechanismItem14appendAssemblyEv")
}

#[doc(alias = "RBX::MechanismItem::consistent(RBX::MechanismItem const*,RBX::MechanismItem const*)")]
// 0x9aecb8 — __ZN3RBX13MechanismItem10consistentEPKS0_S2_
// type: bool __fastcall(RBX::MechanismItem *this, const RBX::MechanismItem *, const RBX::MechanismItem *)
pub fn stub_0x9aecb8() -> ! {
    todo!("0x9aecb8 __ZN3RBX13MechanismItem10consistentEPKS0_S2_")
}

#[doc(alias = "RBX::MechanismItem::lerp(RBX::MechanismItem const*,RBX::MechanismItem const*,RBX::MechanismItem*,float)")]
// 0x9aee00 — __ZN3RBX13MechanismItem4lerpEPKS0_S2_PS0_f
// type: void __fastcall(RBX::MechanismItem *this, const RBX::MechanismItem *, const RBX::MechanismItem *, RBX::MechanismItem *, float)
pub fn stub_0x9aee00() -> ! {
    todo!("0x9aee00 __ZN3RBX13MechanismItem4lerpEPKS0_S2_PS0_f")
}

#[doc(alias = "RBX::AssemblyItem::AssemblyItem(void)")]
// 0x9af8d0 — __ZN3RBX12AssemblyItemC2Ev
// type: RBX::AssemblyItem *__fastcall(RBX::AssemblyItem *this)
pub fn stub_0x9af8d0() -> ! {
    todo!("0x9af8d0 __ZN3RBX12AssemblyItemC2Ev")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot*)")]
// 0x9c4534 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotEEaSEPSA_
// type: int32_t **__fastcall(int32_t **, int32_t *)
pub fn stub_0x9c4534() -> ! {
    todo!("0x9c4534 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotEEaSEPSA_")
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot> const&)")]
// 0x9c45e8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotEEaSERKSB_
// type: int32_t **__fastcall(int32_t **, int32_t **)
pub fn stub_0x9c45e8() -> ! {
    todo!("0x9c45e8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotEEaSERKSB_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::remove(rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot *)")]
// 0x9c49b8 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE6removeEPNS7_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
pub fn stub_0x9c49b8() -> ! {
    todo!("0x9c49b8 __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE6removeEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot::safe_static_init_mutex(void)")]
// 0x9c4aa4 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slot22safe_static_init_mutexEv
// type: void()
pub fn stub_0x9c4aa4() -> ! {
    todo!("0x9c4aa4 __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot::~slot()")]
// 0x9c4b8c — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotD1Ev
// type: int __fastcall(int)
pub fn stub_0x9c4b8c() -> ! {
    todo!("0x9c4b8c __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot::~slot()")]
// 0x9c4be8 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x9c4be8() -> ! {
    todo!("0x9c4be8 __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4slotD0Ev")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::TouchPair>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::TouchPair>,RBX::TouchPair,boost::hash<RBX::TouchPair>,std::equal_to<RBX::TouchPair>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::TouchPair>>(RBX::TouchPair const&,boost::unordered::detail::emplace_args1<RBX::TouchPair> const&)")]
// 0x9c4cf0 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEE12emplace_implINS1_13emplace_args1IS5_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS5_EEEEbERKS5_RKT_
// type: int __fastcall(int, _DWORD *, _DWORD *, _DWORD **)
pub fn stub_0x9c4cf0() -> ! {
    todo!("0x9c4cf0 __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEE12emplace_implINS1_13emplace_args1IS5_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS5_EEEEbERKS5_RKT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::TouchPair>,RBX::TouchPair,boost::hash<RBX::TouchPair>,std::equal_to<RBX::TouchPair>>>::reserve_for_insert(unsigned long)")]
// 0x9c5058 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
pub fn stub_0x9c5058() -> ! {
    todo!("0x9c5058 __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::TouchPair>,RBX::TouchPair,boost::hash<RBX::TouchPair>,std::equal_to<RBX::TouchPair>>>::create_buckets(unsigned long)")]
// 0x9c5200 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
pub fn stub_0x9c5200() -> ! {
    todo!("0x9c5200 __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::TouchPair>,RBX::TouchPair,boost::hash<RBX::TouchPair>,std::equal_to<RBX::TouchPair>>>::~table()")]
// 0x9c53d0 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEED2Ev
// type: _DWORD *__fastcall(_DWORD *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
pub fn stub_0x9c53d0() -> ! {
    todo!("0x9c53d0 __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX9TouchPairEES5_NS_4hashIS5_EESt8equal_toIS5_EEEED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::reset(void)")]
// 0x9cb36c — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0x9cb36c() -> ! {
    todo!("0x9cb36c __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEE5resetEv")
}

#[doc(alias = "RBX::ObjectValue::~ObjectValue()")]
// 0x9e63e0 — __ZN3RBX11ObjectValueD1Ev
// type: void __fastcall(RBX::ObjectValue *__hidden this)
pub fn stub_0x9e63e0() -> ! {
    todo!("0x9e63e0 __ZN3RBX11ObjectValueD1Ev")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<unsigned int>(char const*,unsigned int const&)")]
// 0x9e8e08 — __ZN3RBX5Stats4Item20createBoundChildItemIjEEPS1_PKcRKT_
// type: RBX::Instance *__fastcall(pthread_mutex_t *, pthread_mutex_t *, int)
pub fn stub_0x9e8e08() -> ! {
    todo!("0x9e8e08 __ZN3RBX5Stats4Item20createBoundChildItemIjEEPS1_PKcRKT_")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<int>::deref(int const*)")]
// 0x9ea4c8 — __ZN3RBX5Stats14TypedStatsItemIiE5derefEPKi
// type: void()
pub fn stub_0x9ea4c8() -> ! {
    todo!("0x9ea4c8 __ZN3RBX5Stats14TypedStatsItemIiE5derefEPKi")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x9ea4d0 — __ZN3RBX5Stats14TypedStatsItemIjED1Ev
// type: int __fastcall(int)
pub fn stub_0x9ea4d0() -> ! {
    todo!("0x9ea4d0 __ZN3RBX5Stats14TypedStatsItemIjED1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x9ea4e0 — __ZThn32_N3RBX5Stats14TypedStatsItemIjED1Ev
// type: int __fastcall(int)
pub fn stub_0x9ea4e0() -> ! {
    todo!("0x9ea4e0 __ZThn32_N3RBX5Stats14TypedStatsItemIjED1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x9ea4f0 — __ZThn36_N3RBX5Stats14TypedStatsItemIjED1Ev
// type: int __fastcall(int)
pub fn stub_0x9ea4f0() -> ! {
    todo!("0x9ea4f0 __ZThn36_N3RBX5Stats14TypedStatsItemIjED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x9ea500 — __ZN3RBX5Stats14TypedStatsItemIjED2Ev
// type: int __fastcall(RBX::Instance *)
pub fn stub_0x9ea500() -> ! {
    todo!("0x9ea500 __ZN3RBX5Stats14TypedStatsItemIjED2Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<float>::~TypedStatsItem()")]
// 0x9eac40 — __ZN3RBX5Stats14TypedStatsItemIfED0Ev
// type: void __fastcall(void *)
pub fn stub_0x9eac40() -> ! {
    todo!("0x9eac40 __ZN3RBX5Stats14TypedStatsItemIfED0Ev")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<double>(char const*,double const&)")]
// 0x9eace0 — __ZN3RBX5Stats4Item20createBoundChildItemIdEEPS1_PKcRKT_
// type: RBX::Instance *__fastcall(RBX::Instance *, int, int)
pub fn stub_0x9eace0() -> ! {
    todo!("0x9eace0 __ZN3RBX5Stats4Item20createBoundChildItemIdEEPS1_PKcRKT_")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
// 0x9eb338 — __ZThn36_N3RBX5Stats14TypedStatsItemIdED1Ev
// type: int __fastcall(int)
pub fn stub_0x9eb338() -> ! {
    todo!("0x9eb338 __ZThn36_N3RBX5Stats14TypedStatsItemIdED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
// 0x9eb348 — __ZN3RBX5Stats14TypedStatsItemIdED2Ev
// type: int __fastcall(RBX::Instance *)
pub fn stub_0x9eb348() -> ! {
    todo!("0x9eb348 __ZN3RBX5Stats14TypedStatsItemIdED2Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()")]
// 0x9eb5b8 — __ZThn32_N3RBX5Stats14TypedStatsItemIbED1Ev
// type: int __fastcall(int)
pub fn stub_0x9eb5b8() -> ! {
    todo!("0x9eb5b8 __ZThn32_N3RBX5Stats14TypedStatsItemIbED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()")]
// 0x9eb5c8 — __ZN3RBX5Stats14TypedStatsItemIbED2Ev
// type: int __fastcall(RBX::Instance *)
pub fn stub_0x9eb5c8() -> ! {
    todo!("0x9eb5c8 __ZN3RBX5Stats14TypedStatsItemIbED2Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x9ebaf0 — __ZN3RBX5Stats14TypedStatsItemIyED1Ev
// type: int()
pub fn stub_0x9ebaf0() -> ! {
    todo!("0x9ebaf0 __ZN3RBX5Stats14TypedStatsItemIyED1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x9ebb00 — __ZThn32_N3RBX5Stats14TypedStatsItemIyED1Ev
// type: int __fastcall(int)
pub fn stub_0x9ebb00() -> ! {
    todo!("0x9ebb00 __ZThn32_N3RBX5Stats14TypedStatsItemIyED1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x9ebb10 — __ZThn36_N3RBX5Stats14TypedStatsItemIyED1Ev
// type: int __fastcall(int)
pub fn stub_0x9ebb10() -> ! {
    todo!("0x9ebb10 __ZThn36_N3RBX5Stats14TypedStatsItemIyED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x9ebb20 — __ZN3RBX5Stats14TypedStatsItemIyED2Ev
// type: int __fastcall(RBX::Instance *)
pub fn stub_0x9ebb20() -> ! {
    todo!("0x9ebb20 __ZN3RBX5Stats14TypedStatsItemIyED2Ev")
}

#[doc(alias = "`anonymous namespace::onCrispEventLogged(rbx_core::SharedPtr<RBX::CrispResponse> const&)")]
// 0xa18b90 — __ZN12_GLOBAL__N_118onCrispEventLoggedERKN5boost10shared_ptrIN3RBX13CrispResponseEEE
// type: int __fastcall(int *, int, int, const std::string *)
pub fn stub_0xa18b90() -> ! {
    todo!("0xa18b90 __ZN12_GLOBAL__N_118onCrispEventLoggedERKN5boost10shared_ptrIN3RBX13CrispResponseEEE")
}

#[doc(alias = "XmlElement::XmlElement(RBX::Name const&)")]
// 0xa1c5a0 — __ZN10XmlElementC1ERKN3RBX4NameE
// type: void __fastcall(XmlElement *this, const RBX::Name *)
pub fn stub_0xa1c5a0() -> ! {
    todo!("0xa1c5a0 __ZN10XmlElementC1ERKN3RBX4NameE")
}

#[doc(alias = "void XmlElement::addAttribute<int>(RBX::Name const&,int)")]
// 0xa1c6fc — __ZN10XmlElement12addAttributeIiEEvRKN3RBX4NameET_
// type: void __fastcall(int, int, int, int, int, XmlNameValuePair *, int, int, int, int)
pub fn stub_0xa1c6fc() -> ! {
    todo!("0xa1c6fc __ZN10XmlElement12addAttributeIiEEvRKN3RBX4NameET_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list_av_3<boost::function0<void>,RBX::MessageType,bool>::type> boost::bind<void,boost::function0<void> const&,RBX::MessageType,bool,boost::function0<void>,RBX::MessageType,bool>(void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::function0<void>,RBX::MessageType,bool)")]
// 0xa1e8e8 — __ZN5boost4bindIvRKNS_9function0IvEEN3RBX11MessageTypeEbS2_S6_bEENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_
// type: void __fastcall(int, int, int *, int, unsigned __int8)
pub fn stub_0xa1e8e8() -> ! {
    todo!("0xa1e8e8 __ZN5boost4bindIvRKNS_9function0IvEEN3RBX11MessageTypeEbS2_S6_bEENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(RBX::Name const* const&)")]
// 0xa239f0 — __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEE5eraseERSA_
// type: int __fastcall(_DWORD *, int *)
pub fn stub_0xa239f0() -> ! {
    todo!("0xa239f0 __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEE5eraseERSA_")
}

#[doc(alias = "`non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
// 0xa24538 — __ZThn32_N3RBX12CylinderMeshD1Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
pub fn stub_0xa24538() -> ! {
    todo!("0xa24538 __ZThn32_N3RBX12CylinderMeshD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
// 0xa24548 — __ZThn36_N3RBX12CylinderMeshD0Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
pub fn stub_0xa24548() -> ! {
    todo!("0xa24548 __ZThn36_N3RBX12CylinderMeshD0Ev")
}

#[doc(alias = "RBX::BlockMesh::~BlockMesh()")]
// 0xa24bc8 — __ZN3RBX9BlockMeshD0Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
pub fn stub_0xa24bc8() -> ! {
    todo!("0xa24bc8 __ZN3RBX9BlockMeshD0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
// 0xa24cd8 — __ZThn32_N3RBX9BlockMeshD0Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
pub fn stub_0xa24cd8() -> ! {
    todo!("0xa24cd8 __ZThn32_N3RBX9BlockMeshD0Ev")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::CrispResponse> const&),boost::_bi::list1<boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0xa2baf0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_10shared_ptrIN3RBX13CrispResponseEEEENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// type: int __fastcall(__int64, unsigned int)
pub fn stub_0xa2baf0() -> ! {
    todo!("0xa2baf0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_10shared_ptrIN3RBX13CrispResponseEEEENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::CrispResponse> const&),boost::_bi::list1<boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::CrispResponse>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::CrispResponse>)")]
// 0xa2bb50 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvRKNS_10shared_ptrIN3RBX13CrispResponseEEEENS3_5list1INS_3argILi1EEEEEEEvS8_E6invokeERNS1_15function_bufferES8_
// type: int __fastcall(int (__fastcall **)(int), int)
pub fn stub_0xa2bb50() -> ! {
    todo!("0xa2bb50 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvRKNS_10shared_ptrIN3RBX13CrispResponseEEEENS3_5list1INS_3argILi1EEEEEEEvS8_E6invokeERNS1_15function_bufferES8_")
}
