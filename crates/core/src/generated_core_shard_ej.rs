//! core shard EJ — 100 core stubs EA-sorted, lowest uncovered 0x940350..0x974120 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EI 0x91199c).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::SceneUpdater::queuePriorityInvalidateFastCluster(RBX::GfxPart *)")]
// 0x940350 — __ZN3RBX12SceneUpdater34queuePriorityInvalidateFastClusterEPNS_7GfxPartE
pub fn stub_940350() -> ! {
    todo!("0x940350 __ZN3RBX12SceneUpdater34queuePriorityInvalidateFastClusterEPNS_7GfxPartE")
}

#[doc(alias = "RBX::SceneUpdater::notifyWaitingForAssets(RBX::GfxPart *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>> const&)")]
// 0x940394 — __ZN3RBX12SceneUpdater22notifyWaitingForAssetsEPNS_7GfxPartERKSt6vectorINS_9ContentIdESaIS4_EE
pub fn stub_940394() -> ! {
    todo!("0x940394 __ZN3RBX12SceneUpdater22notifyWaitingForAssetsEPNS_7GfxPartERKSt6vectorINS_9ContentIdESaIS4_EE")
}

#[doc(alias = "RBX::SceneUpdater::updateWaitingParts(void)")]
// 0x9408b0 — __ZN3RBX12SceneUpdater18updateWaitingPartsEv
pub fn stub_9408b0() -> ! {
    todo!("0x9408b0 __ZN3RBX12SceneUpdater18updateWaitingPartsEv")
}

#[doc(alias = "RBX::SceneUpdater::queueInvalidateAttachement(RBX::GfxAttachment *)")]
// 0x940c50 — __ZN3RBX12SceneUpdater26queueInvalidateAttachementEPNS_13GfxAttachmentE
pub fn stub_940c50() -> ! {
    todo!("0x940c50 __ZN3RBX12SceneUpdater26queueInvalidateAttachementEPNS_13GfxAttachmentE")
}

#[doc(alias = "RBX::SceneUpdater::updateMegaClusters(void)")]
// 0x940d50 — __ZN3RBX12SceneUpdater18updateMegaClustersEv
pub fn stub_940d50() -> ! {
    todo!("0x940d50 __ZN3RBX12SceneUpdater18updateMegaClustersEv")
}

#[doc(alias = "RBX::SceneUpdater::updateInvalidatedFastClusters(bool)")]
// 0x941290 — __ZN3RBX12SceneUpdater29updateInvalidatedFastClustersEb
pub fn stub_941290() -> ! {
    todo!("0x941290 __ZN3RBX12SceneUpdater29updateInvalidatedFastClustersEb")
}

#[doc(alias = "RBX::SceneUpdater::arePartsWaitingForAssets(void)")]
// 0x941568 — __ZN3RBX12SceneUpdater24arePartsWaitingForAssetsEv
pub fn stub_941568() -> ! {
    todo!("0x941568 __ZN3RBX12SceneUpdater24arePartsWaitingForAssetsEv")
}

#[doc(alias = "RBX::SceneUpdater::notifyAwake(RBX::GfxPart *)")]
// 0x941574 — __ZN3RBX12SceneUpdater11notifyAwakeEPNS_7GfxPartE
pub fn stub_941574() -> ! {
    todo!("0x941574 __ZN3RBX12SceneUpdater11notifyAwakeEPNS_7GfxPartE")
}

#[doc(alias = "RBX::SceneUpdater::notifySleeping(RBX::GfxPart *)")]
// 0x941668 — __ZN3RBX12SceneUpdater14notifySleepingEPNS_7GfxPartE
pub fn stub_941668() -> ! {
    todo!("0x941668 __ZN3RBX12SceneUpdater14notifySleepingEPNS_7GfxPartE")
}

#[doc(alias = "RBX::SceneUpdater::notifyAwake(RBX::GfxAttachment *)")]
// 0x9417b8 — __ZN3RBX12SceneUpdater11notifyAwakeEPNS_13GfxAttachmentE
pub fn stub_9417b8() -> ! {
    todo!("0x9417b8 __ZN3RBX12SceneUpdater11notifyAwakeEPNS_13GfxAttachmentE")
}

#[doc(alias = "RBX::SceneUpdater::notifySleeping(RBX::GfxAttachment *)")]
// 0x941884 — __ZN3RBX12SceneUpdater14notifySleepingEPNS_13GfxAttachmentE
pub fn stub_941884() -> ! {
    todo!("0x941884 __ZN3RBX12SceneUpdater14notifySleepingEPNS_13GfxAttachmentE")
}

#[doc(alias = "RBX::SceneUpdater::notifyDestroyed(RBX::GfxPart *)")]
// 0x94190c — __ZN3RBX12SceneUpdater15notifyDestroyedEPNS_7GfxPartE
pub fn stub_94190c() -> ! {
    todo!("0x94190c __ZN3RBX12SceneUpdater15notifyDestroyedEPNS_7GfxPartE")
}

#[doc(alias = "RBX::SceneUpdater::queueFastClusterCheck(RBX::GfxPart *,bool)")]
// 0x941a6c — __ZN3RBX12SceneUpdater21queueFastClusterCheckEPNS_7GfxPartEb
pub fn stub_941a6c() -> ! {
    todo!("0x941a6c __ZN3RBX12SceneUpdater21queueFastClusterCheckEPNS_7GfxPartEb")
}

#[doc(alias = "RBX::SceneUpdater::updateDynamicParts(void)")]
// 0x941abc — __ZN3RBX12SceneUpdater18updateDynamicPartsEv
pub fn stub_941abc() -> ! {
    todo!("0x941abc __ZN3RBX12SceneUpdater18updateDynamicPartsEv")
}

#[doc(alias = "RBX::SceneUpdater::updateDynamicAttachements(void)")]
// 0x941cbc — __ZN3RBX12SceneUpdater25updateDynamicAttachementsEv
pub fn stub_941cbc() -> ! {
    todo!("0x941cbc __ZN3RBX12SceneUpdater25updateDynamicAttachementsEv")
}

#[doc(alias = "RBX::SceneUpdater::processPendingMegaClusters(void)")]
// 0x941e98 — __ZN3RBX12SceneUpdater26processPendingMegaClustersEv
pub fn stub_941e98() -> ! {
    todo!("0x941e98 __ZN3RBX12SceneUpdater26processPendingMegaClustersEv")
}

#[doc(alias = "RBX::SceneUpdater::processPendingParts(bool)")]
// 0x9421d0 — __ZN3RBX12SceneUpdater19processPendingPartsEb
pub fn stub_9421d0() -> ! {
    todo!("0x9421d0 __ZN3RBX12SceneUpdater19processPendingPartsEb")
}

#[doc(alias = "RBX::SceneUpdater::update(unsigned long,RBX::Frustum const&)")]
// 0x9424cc — __ZN3RBX12SceneUpdater6updateEmRKNS_7FrustumE
pub fn stub_9424cc() -> ! {
    todo!("0x9424cc __ZN3RBX12SceneUpdater6updateEmRKNS_7FrustumE")
}

#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::~vector()")]
// 0x942728 — __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EED1Ev
pub fn stub_942728() -> ! {
    todo!("0x942728 __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EED1Ev")
}

#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::clear(void)")]
// 0x9429c8 — __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE5clearEv
pub fn stub_9429c8() -> ! {
    todo!("0x9429c8 __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE5clearEv")
}

#[doc(alias = "RBX::SpatialRegion::centerOfRegionInGlobalCoordStuds(RBX::SpatialRegion::Id const&)")]
// 0x942af8 — __ZN3RBX13SpatialRegion32centerOfRegionInGlobalCoordStudsERKNS0_2IdE
pub fn stub_942af8() -> ! {
    todo!("0x942af8 __ZN3RBX13SpatialRegion32centerOfRegionInGlobalCoordStudsERKNS0_2IdE")
}

#[doc(alias = "std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>::erase(__gnu_cxx::__normal_iterator<RBX::ContentId*,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId*,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x942cb0 — __ZNSt6vectorIN3RBX9ContentIdESaIS1_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS1_S3_EES7_
pub fn stub_942cb0() -> ! {
    todo!("0x942cb0 __ZNSt6vectorIN3RBX9ContentIdESaIS1_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS1_S3_EES7_")
}

#[doc(alias = "void std::swap<boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>>(boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &,boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &)")]
// 0x942d70 — __ZSt4swapIN5boost9unordered13unordered_setIPN3RBX7GfxPartENS0_4hashIS5_EESt8equal_toIS5_ENS0_19fast_pool_allocatorIS5_NS0_33default_user_allocator_new_deleteENS0_5mutexELj32ELj0EEEEEEvRT_SG_
pub fn stub_942d70() -> ! {
    todo!("0x942d70 __ZSt4swapIN5boost9unordered13unordered_setIPN3RBX7GfxPartENS0_4hashIS5_EESt8equal_toIS5_ENS0_19fast_pool_allocatorIS5_NS0_33default_user_allocator_new_deleteENS0_5mutexELj32ELj0EEEEEEvRT_SG_")
}

#[doc(alias = "void std::swap<boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>>>(boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &,boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &)")]
// 0x942ef8 — __ZSt4swapIN5boost9unordered13unordered_setIPN3RBX13GfxAttachmentENS0_4hashIS5_EESt8equal_toIS5_ESaIS5_EEEEvRT_SD_
pub fn stub_942ef8() -> ! {
    todo!("0x942ef8 __ZSt4swapIN5boost9unordered13unordered_setIPN3RBX13GfxAttachmentENS0_4hashIS5_EESt8equal_toIS5_ESaIS5_EEEEvRT_SD_")
}

#[doc(alias = "RBX::SceneUpdater::checkFastClusters(void)")]
// 0x943028 — __ZN3RBX12SceneUpdater17checkFastClustersEv
pub fn stub_943028() -> ! {
    todo!("0x943028 __ZN3RBX12SceneUpdater17checkFastClustersEv")
}

#[doc(alias = "RBX::SceneUpdater::computeLighting(bool)")]
// 0x94302c — __ZN3RBX12SceneUpdater15computeLightingEb
pub fn stub_94302c() -> ! {
    todo!("0x94302c __ZN3RBX12SceneUpdater15computeLightingEb")
}

#[doc(alias = "std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxAttachment **,std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>>,RBX::GfxAttachment * const&)")]
// 0x943af8 — __ZNSt6vectorIPN3RBX13GfxAttachmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_943af8() -> ! {
    todo!("0x943af8 __ZNSt6vectorIPN3RBX13GfxAttachmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::GfxPart *,std::allocator<RBX::GfxPart *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxPart **,std::vector<RBX::GfxPart *,std::allocator<RBX::GfxPart *>>>,RBX::GfxPart * const&)")]
// 0x943bf0 — __ZNSt6vectorIPN3RBX7GfxPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_943bf0() -> ! {
    todo!("0x943bf0 __ZNSt6vectorIPN3RBX7GfxPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::erase_key(RBX::GfxPart * const&)")]
// 0x943ce8 — __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE9erase_keyERKS7_
pub fn stub_943ce8() -> ! {
    todo!("0x943ce8 __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE9erase_keyERKS7_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *,boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *)")]
// 0x943e20 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_
pub fn stub_943e20() -> ! {
    todo!("0x943e20 __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")]
// 0x943eb8 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE6assignERKSD_NS1_17integral_constantIbLb0EEE
pub fn stub_943eb8() -> ! {
    todo!("0x943eb8 __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE6assignERKSD_NS1_17integral_constantIbLb0EEE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::create_buckets(unsigned long)")]
// 0x9440f0 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
pub fn stub_9440f0() -> ! {
    todo!("0x9440f0 __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm")
}

#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>> &)")]
// 0x9441a0 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISC_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEERSH_RT_
pub fn stub_9441a0() -> ! {
    todo!("0x9441a0 __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISC_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEERSH_RT_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::table_impl(boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)")]
// 0x9442b8 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2ERKSD_
pub fn stub_9442b8() -> ! {
    todo!("0x9442b8 __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2ERKSD_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)")]
// 0x944438 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE4initERKSD_
pub fn stub_944438() -> ! {
    todo!("0x944438 __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE4initERKSD_")
}

#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>> &)")]
// 0x94451c — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeIS6_EEEEEEEvNS0_15iterator_detail8iteratorISH_EERNS1_5tableISC_EERT_
pub fn stub_94451c() -> ! {
    todo!("0x94451c __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeIS6_EEEEEEEvNS0_15iterator_detail8iteratorISH_EERNS1_5tableISC_EERT_")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxAttachment *>>(RBX::GfxAttachment * const&,boost::unordered::detail::emplace_args1<RBX::GfxAttachment *> const&)")]
// 0x94460c — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
pub fn stub_94460c() -> ! {
    todo!("0x94460c __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::reserve_for_insert(unsigned long)")]
// 0x9447d8 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
pub fn stub_9447d8() -> ! {
    todo!("0x9447d8 __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxPart *> *,boost::unordered::detail::ptr_node<RBX::GfxPart *> *)")]
// 0x944998 — __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11erase_nodesEPNS1_8ptr_nodeIS7_EESJ_
pub fn stub_944998() -> ! {
    todo!("0x944998 __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11erase_nodesEPNS1_8ptr_nodeIS7_EESJ_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")]
// 0x944a88 — __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE6assignERKSG_NS1_17integral_constantIbLb0EEE
pub fn stub_944a88() -> ! {
    todo!("0x944a88 __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE6assignERKSG_NS1_17integral_constantIbLb0EEE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::create_buckets(unsigned long)")]
// 0x944c98 — __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
pub fn stub_944c98() -> ! {
    todo!("0x944c98 __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm")
}

#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>> &)")]
// 0x944dd8 — __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISF_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEERSK_RT_
pub fn stub_944dd8() -> ! {
    todo!("0x944dd8 __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISF_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEERSK_RT_")
}

#[doc(alias = "boost::unordered::detail::node_holder<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::~node_holder()")]
// 0x944f84 — __ZN5boost9unordered6detail11node_holderINS_19fast_pool_allocatorINS1_8ptr_nodeIPN3RBX7GfxPartEEENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED2Ev
pub fn stub_944f84() -> ! {
    todo!("0x944f84 __ZN5boost9unordered6detail11node_holderINS_19fast_pool_allocatorINS1_8ptr_nodeIPN3RBX7GfxPartEEENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED2Ev")
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,12u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::malloc(void)")]
// 0x945148 — __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj12ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_945148() -> ! {
    todo!("0x945148 __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj12ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::destroy_buckets(void)")]
// 0x9451b8 — __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15destroy_bucketsEv
pub fn stub_9451b8() -> ! {
    todo!("0x9451b8 __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15destroy_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::array_constructor<boost::fast_pool_allocator<boost::unordered::detail::ptr_bucket,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::~array_constructor()")]
// 0x945228 — __ZN5boost9unordered6detail17array_constructorINS_19fast_pool_allocatorINS1_10ptr_bucketENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED2Ev
pub fn stub_945228() -> ! {
    todo!("0x945228 __ZN5boost9unordered6detail17array_constructorINS_19fast_pool_allocatorINS1_10ptr_bucketENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED2Ev")
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,4u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::free(void *,unsigned long)")]
// 0x9452a0 — __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj4ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE4freeEPvm
pub fn stub_9452a0() -> ! {
    todo!("0x9452a0 __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj4ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE4freeEPvm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> const&)")]
// 0x9453bc — __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE4initERKSG_
pub fn stub_9453bc() -> ! {
    todo!("0x9453bc __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE4initERKSG_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::~table()")]
// 0x945578 — __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEED2Ev
pub fn stub_945578() -> ! {
    todo!("0x945578 __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEED2Ev")
}

#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::fill_buckets<boost::unordered::detail::copy_nodes<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> &,boost::unordered::detail::copy_nodes<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &)")]
// 0x945624 — __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_10copy_nodesINS4_INS1_8ptr_nodeIS7_EES8_S9_Lj32ELj0EEEEEEEvNS0_15iterator_detail8iteratorISK_EERNS1_5tableISF_EERT_
pub fn stub_945624() -> ! {
    todo!("0x945624 __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_10copy_nodesINS4_INS1_8ptr_nodeIS7_EES8_S9_Lj32ELj0EEEEEEEvNS0_15iterator_detail8iteratorISK_EERNS1_5tableISF_EERT_")
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,4u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::is_from(void *)")]
// 0x9457a0 — __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj4ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv
pub fn stub_9457a0() -> ! {
    todo!("0x9457a0 __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj4ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv")
}

#[doc(alias = "std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::GfxPart * const,RBX::ContentId> const&)")]
// 0x945828 — __ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_
pub fn stub_945828() -> ! {
    todo!("0x945828 __ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_")
}

#[doc(alias = "std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::_Rb_tree_iterator<std::pair<RBX::GfxPart * const,RBX::ContentId>>)")]
// 0x94595c — __ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE5eraseESt17_Rb_tree_iteratorIS6_ESE_
pub fn stub_94595c() -> ! {
    todo!("0x94595c __ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE5eraseESt17_Rb_tree_iteratorIS6_ESE_")
}

#[doc(alias = "std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::GfxPart * const,RBX::ContentId>> *)")]
// 0x945a14 — __ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
pub fn stub_945a14() -> ! {
    todo!("0x945a14 __ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,int>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,int)")]
// 0x945a8c — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEiEvT_S9_T0_
pub fn stub_945a8c() -> ! {
    todo!("0x945a8c __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEiEvT_S9_T0_")
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x945ca0 — __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_
pub fn stub_945ca0() -> ! {
    todo!("0x945ca0 __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_")
}

#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x945e60 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_
pub fn stub_945e60() -> ! {
    todo!("0x945e60 __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>> std::__unguarded_partition<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,RBX::ContentId>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,RBX::ContentId)")]
// 0x9460e0 — __ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEES3_ET_S9_S9_T0_
pub fn stub_9460e0() -> ! {
    todo!("0x9460e0 __ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEES3_ET_S9_S9_T0_")
}

#[doc(alias = "void std::__iter_swap<true>::iter_swap<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x946128 — __ZNSt11__iter_swapILb1EE9iter_swapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS5_SaIS5_EEEESA_EEvT_T0_
pub fn stub_946128() -> ! {
    todo!("0x946128 __ZNSt11__iter_swapILb1EE9iter_swapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS5_SaIS5_EEEESA_EEvT_T0_")
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x946264 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_S9_
pub fn stub_946264() -> ! {
    todo!("0x946264 __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_S9_")
}

#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x946454 — __ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_
pub fn stub_946454() -> ! {
    todo!("0x946454 __ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_")
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,int,RBX::ContentId>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,int,int,RBX::ContentId)")]
// 0x946624 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEiS3_EvT_T0_SA_T1_
pub fn stub_946624() -> ! {
    todo!("0x946624 __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEiS3_EvT_T0_SA_T1_")
}

#[doc(alias = "void std::make_heap<__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>>(__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,__gnu_cxx::__normal_iterator<RBX::ContentId *,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>)")]
// 0x946850 — __ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_
pub fn stub_946850() -> ! {
    todo!("0x946850 __ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX9ContentIdESt6vectorIS3_SaIS3_EEEEEvT_S9_")
}

#[doc(alias = "std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>::vector(std::vector<RBX::ContentId,std::allocator<RBX::ContentId>> const&)")]
// 0x9469a4 — __ZNSt6vectorIN3RBX9ContentIdESaIS1_EEC2ERKS3_
pub fn stub_9469a4() -> ! {
    todo!("0x9469a4 __ZNSt6vectorIN3RBX9ContentIdESaIS1_EEC2ERKS3_")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxPart *>>(RBX::GfxPart * const&,boost::unordered::detail::emplace_args1<RBX::GfxPart *> const&)")]
// 0x946b70 — __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
pub fn stub_946b70() -> ! {
    todo!("0x946b70 __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::reserve_for_insert(unsigned long)")]
// 0x946db0 — __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
pub fn stub_946db0() -> ! {
    todo!("0x946db0 __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm")
}

#[doc(alias = "std::vector<RBX::SceneUpdater::MegaClusterChunk,std::allocator<RBX::SceneUpdater::MegaClusterChunk>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SceneUpdater::MegaClusterChunk*,std::vector<RBX::SceneUpdater::MegaClusterChunk,std::allocator<RBX::SceneUpdater::MegaClusterChunk>>>,RBX::SceneUpdater::MegaClusterChunk const&)")]
// 0x946f50 — __ZNSt6vectorIN3RBX12SceneUpdater16MegaClusterChunkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_946f50() -> ! {
    todo!("0x946f50 __ZNSt6vectorIN3RBX12SceneUpdater16MegaClusterChunkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "boost::detail::weak_count::operator=(boost::detail::weak_count const&)")]
// 0x9486a0 — __ZN5boost6detail10weak_countaSERKS1_
pub fn stub_9486a0() -> ! {
    todo!("0x9486a0 __ZN5boost6detail10weak_countaSERKS1_")
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,12u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::is_from(void *)")]
// 0x949164 — __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj12ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv
pub fn stub_949164() -> ! {
    todo!("0x949164 __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj12ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv")
}

#[doc(alias = "RBX::Draw::spokes(float,float,RBX::Adorn *)")]
// 0x94a8f0 — __ZN3RBX4Draw6spokesEffPNS_5AdornE
pub fn stub_94a8f0() -> ! {
    todo!("0x94a8f0 __ZN3RBX4Draw6spokesEffPNS_5AdornE")
}

#[doc(alias = "RBX::Draw::selectionBox(RBX::Part const&,RBX::Adorn *,RBX::SelectState,float)")]
// 0x94ac18 — __ZN3RBX4Draw12selectionBoxERKNS_4PartEPNS_5AdornENS_11SelectStateEf
pub fn stub_94ac18() -> ! {
    todo!("0x94ac18 __ZN3RBX4Draw12selectionBoxERKNS_4PartEPNS_5AdornENS_11SelectStateEf")
}

#[doc(alias = "RBX::DrawAdorn::axisWidget(RBX::Adorn *,RBX::Camera const&)")]
// 0x94c984 — __ZN3RBX9DrawAdorn10axisWidgetEPNS_5AdornERKNS_6CameraE
pub fn stub_94c984() -> ! {
    todo!("0x94c984 __ZN3RBX9DrawAdorn10axisWidgetEPNS_5AdornERKNS_6CameraE")
}

#[doc(alias = "RBX::CircleRadialNormal::eval(float)")]
// 0x94ec20 — __ZN3RBX18CircleRadialNormal4evalEf
pub fn stub_94ec20() -> ! {
    todo!("0x94ec20 __ZN3RBX18CircleRadialNormal4evalEf")
}

#[doc(alias = "RBX::CircleRadialNormal::evalTangent(float)")]
// 0x94ece0 — __ZN3RBX18CircleRadialNormal11evalTangentEf
pub fn stub_94ece0() -> ! {
    todo!("0x94ece0 __ZN3RBX18CircleRadialNormal11evalTangentEf")
}

#[doc(alias = "RBX::CircleRadialNormal::evalNormal(float)")]
// 0x94ed88 — __ZN3RBX18CircleRadialNormal10evalNormalEf
pub fn stub_94ed88() -> ! {
    todo!("0x94ed88 __ZN3RBX18CircleRadialNormal10evalNormalEf")
}

#[doc(alias = "RBX::CircleRadialNormal::evalBinormal(float)")]
// 0x94ee30 — __ZN3RBX18CircleRadialNormal12evalBinormalEf
pub fn stub_94ee30() -> ! {
    todo!("0x94ee30 __ZN3RBX18CircleRadialNormal12evalBinormalEf")
}

#[doc(alias = "RBX::CircleRadialNormal::hashString(void)")]
// 0x94ee5c — __ZN3RBX18CircleRadialNormal10hashStringEv
pub fn stub_94ee5c() -> ! {
    todo!("0x94ee5c __ZN3RBX18CircleRadialNormal10hashStringEv")
}

#[doc(alias = "RBX::isDebuggerPresentFast(void)")]
// 0x9573bc — __ZN3RBX21isDebuggerPresentFastEv
pub fn stub_9573bc() -> ! {
    todo!("0x9573bc __ZN3RBX21isDebuggerPresentFastEv")
}

#[doc(alias = "rbx::implementation::typed_holder<PacketReliability>::construct_func(char const*,char *)")]
// 0x95b38c — __ZN3rbx14implementation12typed_holderI17PacketReliabilityE14construct_funcEPKcPc
pub fn stub_95b38c() -> ! {
    todo!("0x95b38c __ZN3rbx14implementation12typed_holderI17PacketReliabilityE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<PacketReliability>::destruct_func(char *)")]
// 0x95b398 — __ZN3rbx14implementation12typed_holderI17PacketReliabilityE13destruct_funcEPc
pub fn stub_95b398() -> ! {
    todo!("0x95b398 __ZN3rbx14implementation12typed_holderI17PacketReliabilityE13destruct_funcEPc")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,PacketReliability>,std::_Select1st<std::pair<RBX::Name const* const,PacketReliability>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,PacketReliability>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,PacketReliability>> *)")]
// 0x95b468 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_17PacketReliabilityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_95b468() -> ! {
    todo!("0x95b468 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_17PacketReliabilityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "rbx::implementation::typed_holder<PacketPriority>::construct_func(char const*,char *)")]
// 0x95bef0 — __ZN3rbx14implementation12typed_holderI14PacketPriorityE14construct_funcEPKcPc
pub fn stub_95bef0() -> ! {
    todo!("0x95bef0 __ZN3rbx14implementation12typed_holderI14PacketPriorityE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<PacketPriority>::destruct_func(char *)")]
// 0x95befc — __ZN3rbx14implementation12typed_holderI14PacketPriorityE13destruct_funcEPc
pub fn stub_95befc() -> ! {
    todo!("0x95befc __ZN3rbx14implementation12typed_holderI14PacketPriorityE13destruct_funcEPc")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,PacketPriority>,std::_Select1st<std::pair<RBX::Name const* const,PacketPriority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,PacketPriority>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,PacketPriority>> *)")]
// 0x95bfcc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_14PacketPriorityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_95bfcc() -> ! {
    todo!("0x95bfcc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_14PacketPriorityESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,int,std::string)>::operator()(std::string,int,std::string)")]
// 0x96cf60 — __ZN3rbx7signals16signal_with_argsILi3EFvSsiSsEEclESsiSs
pub fn stub_96cf60() -> ! {
    todo!("0x96cf60 __ZN3rbx7signals16signal_with_argsILi3EFvSsiSsEEclESsiSs")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,int,std::string)>::fireItem(rbx::signals::signal<void ()(std::string,int,std::string)>::slot *,std::string,int,std::string)")]
// 0x970090 — __ZN3rbx7signals16signal_with_argsILi3EFvSsiSsEE8fireItemEPNS0_6signalIS2_E4slotESsiSs
pub fn stub_970090() -> ! {
    todo!("0x970090 __ZN3rbx7signals16signal_with_argsILi3EFvSsiSsEE8fireItemEPNS0_6signalIS2_E4slotESsiSs")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,std::string)>::slot> const&)")]
// 0x970250 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiSsEE4slotEEaSERKS7_
pub fn stub_970250() -> ! {
    todo!("0x970250 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiSsEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::disconnectAll(void)")]
// 0x971c04 — __ZN3rbx7signals6signalIFvSsiSsEE13disconnectAllEv
pub fn stub_971c04() -> ! {
    todo!("0x971c04 __ZN3rbx7signals6signalIFvSsiSsEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::insert(rbx::signals::signal<void ()(std::string,int,std::string)>::slot *)")]
// 0x97356c — __ZN3rbx7signals6signalIFvSsiSsEE6insertEPNS3_4slotE
pub fn stub_97356c() -> ! {
    todo!("0x97356c __ZN3rbx7signals6signalIFvSsiSsEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,int,std::string)>::slot*)")]
// 0x973820 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiSsEE4slotEEaSEPS6_
pub fn stub_973820() -> ! {
    todo!("0x973820 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiSsEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::callable_slot<boost::function<void ()(std::string,int,std::string)>>::~callable_slot()")]
// 0x9738d4 — __ZN3rbx7signals6signalIFvSsiSsEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_9738d4() -> ! {
    todo!("0x9738d4 __ZN3rbx7signals6signalIFvSsiSsEE13callable_slotIN5boost8functionIS2_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::callable_slot<boost::function<void ()(std::string,int,std::string)>>::~callable_slot()")]
// 0x9738e0 — __ZN3rbx7signals6signalIFvSsiSsEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_9738e0() -> ! {
    todo!("0x9738e0 __ZN3rbx7signals6signalIFvSsiSsEE13callable_slotIN5boost8functionIS2_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::slot::disconnect(void)")]
// 0x973994 — __ZN3rbx7signals6signalIFvSsiSsEE4slot10disconnectEv
pub fn stub_973994() -> ! {
    todo!("0x973994 __ZN3rbx7signals6signalIFvSsiSsEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::slot::connected(void)const")]
// 0x973b08 — __ZNK3rbx7signals6signalIFvSsiSsEE4slot9connectedEv
pub fn stub_973b08() -> ! {
    todo!("0x973b08 __ZNK3rbx7signals6signalIFvSsiSsEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,std::string)>::slot,boost::function<void ()(std::string,int,std::string)>,3,void ()(std::string,int,std::string)>::call(std::string,int,std::string)")]
// 0x973b14 — __ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsiSs
pub fn stub_973b14() -> ! {
    todo!("0x973b14 __ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsiSs")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,int,std::string)>::slot,boost::function<void ()(std::string,int,std::string)>,3,void ()(std::string,int,std::string)>::call(std::string,int,std::string)")]
// 0x973cb8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsiSs
pub fn stub_973cb8() -> ! {
    todo!("0x973cb8 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsiSs")
}

#[doc(alias = "boost::function3<void,std::string,int,std::string>::operator()(std::string,int,std::string)const")]
// 0x973cc4 — __ZNK5boost9function3IvSsiSsEclESsiSs
pub fn stub_973cc4() -> ! {
    todo!("0x973cc4 __ZNK5boost9function3IvSsiSsEclESsiSs")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::remove(rbx::signals::signal<void ()(std::string,int,std::string)>::slot *)")]
// 0x973f50 — __ZN3rbx7signals6signalIFvSsiSsEE6removeEPNS3_4slotE
pub fn stub_973f50() -> ! {
    todo!("0x973f50 __ZN3rbx7signals6signalIFvSsiSsEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::slot::safe_static_init_mutex(void)")]
// 0x97403c — __ZN3rbx7signals6signalIFvSsiSsEE4slot22safe_static_init_mutexEv
pub fn stub_97403c() -> ! {
    todo!("0x97403c __ZN3rbx7signals6signalIFvSsiSsEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,std::string)>::slot,boost::function<void ()(std::string,int,std::string)>,3,void ()(std::string,int,std::string)>::~callable()")]
// 0x974120 — __ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_ED2Ev
pub fn stub_974120() -> ! {
    todo!("0x974120 __ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_ED2Ev")
}
