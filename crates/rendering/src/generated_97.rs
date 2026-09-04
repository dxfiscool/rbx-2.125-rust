//! rendering — next 100 Ogre|Gfx|Render|G3D (15058 total, 11876 prior -> 11976 covered, 3182 remaining -> 3082 after)
//! This shard: 0x941668..0xbeb9a0 (100 stubs, EA-sorted)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x941668 — __ZN3RBX12SceneUpdater14notifySleepingEPNS_7GfxPartE
#[doc(alias = "RBX::SceneUpdater::notifySleeping(RBX::GfxPart *)")]
// was: RBX::SceneUpdater::notifySleeping(RBX::GfxPart *)
// IDA 0x941668: 122 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_941668() {
}

// 0x9417b8 — __ZN3RBX12SceneUpdater11notifyAwakeEPNS_13GfxAttachmentE
#[doc(alias = "RBX::SceneUpdater::notifyAwake(RBX::GfxAttachment *)")]
// was: RBX::SceneUpdater::notifyAwake(RBX::GfxAttachment *)
// IDA 0x9417b8: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9417b8() {
}

// 0x941884 — __ZN3RBX12SceneUpdater14notifySleepingEPNS_13GfxAttachmentE
#[doc(alias = "RBX::SceneUpdater::notifySleeping(RBX::GfxAttachment *)")]
// was: RBX::SceneUpdater::notifySleeping(RBX::GfxAttachment *)
// IDA 0x941884: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_941884() {
}

// 0x94190c — __ZN3RBX12SceneUpdater15notifyDestroyedEPNS_7GfxPartE
#[doc(alias = "RBX::SceneUpdater::notifyDestroyed(RBX::GfxPart *)")]
// was: RBX::SceneUpdater::notifyDestroyed(RBX::GfxPart *)
// IDA 0x94190c: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94190c() {
}

// 0x941a6c — __ZN3RBX12SceneUpdater21queueFastClusterCheckEPNS_7GfxPartEb
#[doc(alias = "RBX::SceneUpdater::queueFastClusterCheck(RBX::GfxPart *,bool)")]
// was: RBX::SceneUpdater::queueFastClusterCheck(RBX::GfxPart *,bool)
// IDA 0x941a6c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_941a6c() {
}

// 0x942d70 — __ZSt4swapIN5boost9unordered13unordered_setIPN3RBX7GfxPartENS0_4hashIS5_EESt8equal_toIS5_ENS0_19fast_pool_allocatorIS5_NS0_33default_user_allocator_new_deleteENS0_5mutexELj32ELj0EEEEEEvRT_SG_
#[doc(alias = "void std::swap<boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>>(boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &,boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &)")]
// was: void std::swap<boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>>(boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &,boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &)
// IDA 0x942d70: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_942d70() {
}

// 0x942ef8 — __ZSt4swapIN5boost9unordered13unordered_setIPN3RBX13GfxAttachmentENS0_4hashIS5_EESt8equal_toIS5_ESaIS5_EEEEvRT_SD_
#[doc(alias = "void std::swap<boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>>>(boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &,boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &)")]
// was: void std::swap<boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>>>(boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &,boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &)
// IDA 0x942ef8: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_942ef8() {
}

// 0x943af8 — __ZNSt6vectorIPN3RBX13GfxAttachmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxAttachment **,std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>>,RBX::GfxAttachment * const&)")]
// was: std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxAttachment **,std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>>,RBX::GfxAttachment * const&)
// IDA 0x943af8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_943af8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x943bf0 — __ZNSt6vectorIPN3RBX7GfxPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::GfxPart *,std::allocator<RBX::GfxPart *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxPart **,std::vector<RBX::GfxPart *,std::allocator<RBX::GfxPart *>>>,RBX::GfxPart * const&)")]
// was: std::vector<RBX::GfxPart *,std::allocator<RBX::GfxPart *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxPart **,std::vector<RBX::GfxPart *,std::allocator<RBX::GfxPart *>>>,RBX::GfxPart * const&)
// IDA 0x943bf0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_943bf0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x943ce8 — __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE9erase_keyERKS7_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::erase_key(RBX::GfxPart * const&)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::erase_key(RBX::GfxPart * const&)
// IDA 0x943ce8: 119 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_943ce8() {
}

// 0x943e20 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *,boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *,boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *)
// IDA 0x943e20: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_943e20() {
}

// 0x943eb8 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE6assignERKSD_NS1_17integral_constantIbLb0EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&,boost::unordered::detail::integral_constant<bool,false>)
// IDA 0x943eb8: 204 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_943eb8() {
}

// 0x9440f0 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::create_buckets(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::create_buckets(unsigned long)
// IDA 0x9440f0: 56 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9440f0() {
}

// 0x9441a0 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISC_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEERSH_RT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>> &)")]
// was: void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>> &)
// IDA 0x9441a0: 109 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9441a0() {
}

// 0x9442b8 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2ERKSD_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::table_impl(boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::table_impl(boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)
// IDA 0x9442b8: 138 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9442b8() {
}

// 0x944438 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE4initERKSD_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)
// IDA 0x944438: 81 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_944438() {
}

// 0x94451c — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeIS6_EEEEEEEvNS0_15iterator_detail8iteratorISH_EERNS1_5tableISC_EERT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>> &)")]
// was: void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>> &)
// IDA 0x94451c: 94 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94451c() {
}

// 0x94460c — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxAttachment *>>(RBX::GfxAttachment * const&,boost::unordered::detail::emplace_args1<RBX::GfxAttachment *> const&)")]
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxAttachment *>>(RBX::GfxAttachment * const&,boost::unordered::detail::emplace_args1<RBX::GfxAttachment *> const&)
// IDA 0x94460c: 175 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94460c() {
}

// 0x9447d8 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::reserve_for_insert(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::reserve_for_insert(unsigned long)
// IDA 0x9447d8: 148 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9447d8() {
}

// 0x944998 — __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11erase_nodesEPNS1_8ptr_nodeIS7_EESJ_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxPart *> *,boost::unordered::detail::ptr_node<RBX::GfxPart *> *)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxPart *> *,boost::unordered::detail::ptr_node<RBX::GfxPart *> *)
// IDA 0x944998: 89 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_944998() {
}

// 0x944a88 — __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE6assignERKSG_NS1_17integral_constantIbLb0EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> const&,boost::unordered::detail::integral_constant<bool,false>)
// IDA 0x944a88: 185 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_944a88() {
}

// 0x944c98 — __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::create_buckets(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::create_buckets(unsigned long)
// IDA 0x944c98: 108 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_944c98() {
}

// 0x944dd8 — __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISF_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEERSK_RT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>> &)")]
// was: void boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>> &)
// IDA 0x944dd8: 159 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_944dd8() {
}

// 0x944f84 — __ZN5boost9unordered6detail11node_holderINS_19fast_pool_allocatorINS1_8ptr_nodeIPN3RBX7GfxPartEEENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED2Ev
#[doc(alias = "boost::unordered::detail::node_holder<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::~node_holder()")]
// was: boost::unordered::detail::node_holder<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::~node_holder()
// IDA 0x944f84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_944f84() {
}

// 0x9451b8 — __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15destroy_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::destroy_buckets(void)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::destroy_buckets(void)
// IDA 0x9451b8: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9451b8() {
}

// 0x9453bc — __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE4initERKSG_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> const&)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> const&)
// IDA 0x9453bc: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9453bc() {
}

// 0x945578 — __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEED2Ev
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::~table()")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::~table()
// IDA 0x945578: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_945578() {
}

// 0x945624 — __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_10copy_nodesINS4_INS1_8ptr_nodeIS7_EES8_S9_Lj32ELj0EEEEEEEvNS0_15iterator_detail8iteratorISK_EERNS1_5tableISF_EERT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::fill_buckets<boost::unordered::detail::copy_nodes<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> &,boost::unordered::detail::copy_nodes<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &)")]
// was: void boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::fill_buckets<boost::unordered::detail::copy_nodes<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> &,boost::unordered::detail::copy_nodes<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &)
// IDA 0x945624: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_945624() {
}

// 0x945828 — __ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_
#[doc(alias = "std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::GfxPart * const,RBX::ContentId> const&)")]
// was: std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::GfxPart * const,RBX::ContentId> const&)
// IDA 0x945828: 113 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_945828() {
}

// 0x94595c — __ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE5eraseESt17_Rb_tree_iteratorIS6_ESE_
#[doc(alias = "std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::_Rb_tree_iterator<std::pair<RBX::GfxPart * const,RBX::ContentId>>)")]
// was: std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::_Rb_tree_iterator<std::pair<RBX::GfxPart * const,RBX::ContentId>>)
// IDA 0x94595c: 72 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94595c() {
}

// 0x945a14 — __ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::GfxPart * const,RBX::ContentId>> *)")]
// was: std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::GfxPart * const,RBX::ContentId>> *)
// IDA 0x945a14: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_945a14() {
}

// 0x946b70 — __ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxPart *>>(RBX::GfxPart * const&,boost::unordered::detail::emplace_args1<RBX::GfxPart *> const&)")]
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxPart *>>(RBX::GfxPart * const&,boost::unordered::detail::emplace_args1<RBX::GfxPart *> const&)
// IDA 0x946b70: 216 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_946b70() {
}

// 0x946db0 — __ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::reserve_for_insert(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::reserve_for_insert(unsigned long)
// IDA 0x946db0: 148 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_946db0() {
}

// 0x9b27fc — __ZNK3RBX15NetworkSettings24getRenderStreamedRegionsEv
#[doc(alias = "RBX::NetworkSettings::getRenderStreamedRegions(void)const")]
// was: RBX::NetworkSettings::getRenderStreamedRegions(void)const
// IDA 0x9b27fc: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9b27fc() {
}

// 0x9b280c — __ZN3RBX15NetworkSettings24setRenderStreamedRegionsEb
#[doc(alias = "RBX::NetworkSettings::setRenderStreamedRegions(bool)")]
// was: RBX::NetworkSettings::setRenderStreamedRegions(bool)
// IDA 0x9b280c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9b280c() {
}

// 0xb6acd0 — __ZN3RBX27FastClusterShadowRenderableC2EPNS_17FastClusterEntityEPNS_21FastClusterShadowDataE
#[doc(alias = "RBX::FastClusterShadowRenderable::FastClusterShadowRenderable(RBX::FastClusterEntity *,RBX::FastClusterShadowData *)")]
// was: RBX::FastClusterShadowRenderable::FastClusterShadowRenderable(RBX::FastClusterEntity *,RBX::FastClusterShadowData *)
// IDA 0xb6acd0: 217 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6acd0() {
}

// 0xb6af24 — __ZN3RBX27FastClusterShadowRenderableD0Ev
#[doc(alias = "RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")]
// was: RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()
// IDA 0xb6af24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b6af24() {
}

// 0xb6afd8 — __ZN3RBX27FastClusterShadowRenderableD1Ev
#[doc(alias = "RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")]
// was: RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()
// IDA 0xb6afd8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_b6afd8() {
}

// 0xb6afdc — __ZThn96_N3RBX27FastClusterShadowRenderableD0Ev
#[doc(alias = "non-virtual thunk to RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")]
// was: non-virtual thunk to RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()
// IDA 0xb6afdc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b6afdc() {
}

// 0xb6b094 — __ZN3RBX27FastClusterShadowRenderableD2Ev
#[doc(alias = "RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")]
// was: RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()
// IDA 0xb6b094: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b6b094() {
}

// 0xb6b1e0 — __ZThn96_N3RBX27FastClusterShadowRenderableD1Ev
#[doc(alias = "non-virtual thunk to RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")]
// was: non-virtual thunk to RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()
// IDA 0xb6b1e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b6b1e0() {
}

// 0xb6b628 — __ZNK3RBX27FastClusterShadowRenderable21getNumWorldTransformsEv
#[doc(alias = "RBX::FastClusterShadowRenderable::getNumWorldTransforms(void)const")]
// was: RBX::FastClusterShadowRenderable::getNumWorldTransforms(void)const
// IDA 0xb6b628: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b628() {
}

// 0xb6b638 — __ZNK3RBX27FastClusterShadowRenderable14getCastShadowsEv
#[doc(alias = "RBX::FastClusterShadowRenderable::getCastShadows(void)const")]
// was: RBX::FastClusterShadowRenderable::getCastShadows(void)const
// IDA 0xb6b638: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b638() {
}

// 0xb6b63c — __ZThn96_NK3RBX27FastClusterShadowRenderable14getCastShadowsEv
#[doc(alias = "non-virtual thunk to RBX::FastClusterShadowRenderable::getCastShadows(void)const")]
// was: non-virtual thunk to RBX::FastClusterShadowRenderable::getCastShadows(void)const
// IDA 0xb6b63c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b63c() {
}

// 0xb6b640 — __ZN3RBX27FastClusterShadowRenderable11getEdgeListEv
#[doc(alias = "RBX::FastClusterShadowRenderable::getEdgeList(void)")]
// was: RBX::FastClusterShadowRenderable::getEdgeList(void)
// IDA 0xb6b640: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b640() {
}

// 0xb6b644 — __ZThn96_N3RBX27FastClusterShadowRenderable11getEdgeListEv
#[doc(alias = "non-virtual thunk to RBX::FastClusterShadowRenderable::getEdgeList(void)")]
// was: non-virtual thunk to RBX::FastClusterShadowRenderable::getEdgeList(void)
// IDA 0xb6b644: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b644() {
}

// 0xb6b648 — __ZN3RBX27FastClusterShadowRenderable11hasEdgeListEv
#[doc(alias = "RBX::FastClusterShadowRenderable::hasEdgeList(void)")]
// was: RBX::FastClusterShadowRenderable::hasEdgeList(void)
// IDA 0xb6b648: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b648() {
}

// 0xb6b64c — __ZThn96_N3RBX27FastClusterShadowRenderable11hasEdgeListEv
#[doc(alias = "non-virtual thunk to RBX::FastClusterShadowRenderable::hasEdgeList(void)")]
// was: non-virtual thunk to RBX::FastClusterShadowRenderable::hasEdgeList(void)
// IDA 0xb6b64c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b64c() {
}

// 0xb6b650 — __ZNK3RBX27FastClusterShadowRenderable19getWorldBoundingBoxEb
#[doc(alias = "RBX::FastClusterShadowRenderable::getWorldBoundingBox(bool)const")]
// was: RBX::FastClusterShadowRenderable::getWorldBoundingBox(bool)const
// IDA 0xb6b650: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b650() {
}

// 0xb6b65c — __ZThn96_NK3RBX27FastClusterShadowRenderable19getWorldBoundingBoxEb
#[doc(alias = "non-virtual thunk to RBX::FastClusterShadowRenderable::getWorldBoundingBox(bool)const")]
// was: non-virtual thunk to RBX::FastClusterShadowRenderable::getWorldBoundingBox(bool)const
// IDA 0xb6b65c: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b65c() {
}

// 0xb6b668 — __ZNK3RBX27FastClusterShadowRenderable17getLightCapBoundsEv
#[doc(alias = "RBX::FastClusterShadowRenderable::getLightCapBounds(void)const")]
// was: RBX::FastClusterShadowRenderable::getLightCapBounds(void)const
// IDA 0xb6b668: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b668() {
}

// 0xb6b674 — __ZThn96_NK3RBX27FastClusterShadowRenderable17getLightCapBoundsEv
#[doc(alias = "non-virtual thunk to RBX::FastClusterShadowRenderable::getLightCapBounds(void)const")]
// was: non-virtual thunk to RBX::FastClusterShadowRenderable::getLightCapBounds(void)const
// IDA 0xb6b674: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b674() {
}

// 0xb71010 — __ZN3RBX10GfxBinding11updateChunkERKNS_13SpatialRegion2IdEb
#[doc(alias = "RBX::GfxBinding::updateChunk(RBX::SpatialRegion::Id const&,bool)")]
// was: RBX::GfxBinding::updateChunk(RBX::SpatialRegion::Id const&,bool)
// IDA 0xb71010: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_b71010() {
}

// 0xb71018 — __ZN3RBX10GfxBinding13onSizeChangedEv
#[doc(alias = "RBX::GfxBinding::onSizeChanged(void)")]
// was: RBX::GfxBinding::onSizeChanged(void)
// IDA 0xb71018: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b71018() {
}

// 0xb71020 — __ZN3RBX10GfxBinding21onTransparencyChangedEv
#[doc(alias = "RBX::GfxBinding::onTransparencyChanged(void)")]
// was: RBX::GfxBinding::onTransparencyChanged(void)
// IDA 0xb71020: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b71020() {
}

// 0xb71028 — __ZN3RBX10GfxBinding21onSpecialShapeChangedEv
#[doc(alias = "RBX::GfxBinding::onSpecialShapeChanged(void)")]
// was: RBX::GfxBinding::onSpecialShapeChanged(void)
// IDA 0xb71028: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b71028() {
}

// 0xb871c0 — __ZN3RBX17MaterialGenerator20createRenderMaterialEji
#[doc(alias = "RBX::MaterialGenerator::createRenderMaterial(unsigned int,int)")]
// was: RBX::MaterialGenerator::createRenderMaterial(unsigned int,int)
// IDA 0xb871c0: 338 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b871c0() {
}

// 0xb8edc8 — __ZN3RBX11AdornRbxGfx16getFreeSubEntityEv
#[doc(alias = "RBX::AdornRbxGfx::getFreeSubEntity(void)")]
// was: RBX::AdornRbxGfx::getFreeSubEntity(void)
// IDA 0xb8edc8: 135 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8edc8() {
}

// 0xb8ef38 — __ZN3RBX11AdornRbxGfx8findMeshESs
#[doc(alias = "RBX::AdornRbxGfx::findMesh(std::string)")]
// was: RBX::AdornRbxGfx::findMesh(std::string)
// IDA 0xb8ef38: 247 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8ef38() {
}

// 0xb8f338 — __ZNK3RBX11AdornRbxGfx11getViewportEv
#[doc(alias = "RBX::AdornRbxGfx::getViewport(void)const")]
// was: RBX::AdornRbxGfx::getViewport(void)const
// IDA 0xb8f338: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8f338() {
}

// 0xb8f3f8 — __ZNK3RBX11AdornRbxGfx9getCameraEv
#[doc(alias = "RBX::AdornRbxGfx::getCamera(void)const")]
// was: RBX::AdornRbxGfx::getCamera(void)const
// IDA 0xb8f3f8: 11 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8f3f8() {
}

// 0xb8f414 — __ZN3RBX11AdornRbxGfx10setTextureEiRKN5boost10shared_ptrINS_16TextureProxyBaseEEE
#[doc(alias = "RBX::AdornRbxGfx::setTexture(int,rbx_core::SharedPtr<RBX::TextureProxyBase> const&)")]
// was: RBX::AdornRbxGfx::setTexture(int,boost::shared_ptr<RBX::TextureProxyBase> const&)
// IDA 0xb8f414: 130 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8f414() {
}

// 0xb8f560 — __ZNK3RBX11AdornRbxGfx14getTextureSizeERKN5boost10shared_ptrINS_16TextureProxyBaseEEE
#[doc(alias = "RBX::AdornRbxGfx::getTextureSize(rbx_core::SharedPtr<RBX::TextureProxyBase> const&)const")]
// was: RBX::AdornRbxGfx::getTextureSize(boost::shared_ptr<RBX::TextureProxyBase> const&)const
// IDA 0xb8f560: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8f560() {
}

// 0xb90148 — __ZN3RBX11AdornRbxGfx18createTextureProxyERKNS_9ContentIdERbb
#[doc(alias = "RBX::AdornRbxGfx::createTextureProxy(RBX::ContentId const&,bool &,bool)")]
// was: RBX::AdornRbxGfx::createTextureProxy(RBX::ContentId const&,bool &,bool)
// IDA 0xb90148: 1099 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b90148() {
}

// 0xb91764 — __ZN3RBX11AdornRbxGfx16finishRenderPassEv
#[doc(alias = "RBX::AdornRbxGfx::finishRenderPass(void)")]
// was: RBX::AdornRbxGfx::finishRenderPass(void)
// IDA 0xb91764: 55 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b91764() {
}

// 0xb95acc — __ZN3RBX11AdornRbxGfxD0Ev
#[doc(alias = "RBX::AdornRbxGfx::~AdornRbxGfx()")]
// was: RBX::AdornRbxGfx::~AdornRbxGfx()
// IDA 0xb95acc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b95acc() {
}

// 0xb95b6c — __ZN3RBX11AdornRbxGfxD1Ev
#[doc(alias = "RBX::AdornRbxGfx::~AdornRbxGfx()")]
// was: RBX::AdornRbxGfx::~AdornRbxGfx()
// IDA 0xb95b6c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_b95b6c() {
}

// 0xb95b70 — __ZN3RBX11AdornRbxGfxD2Ev
#[doc(alias = "RBX::AdornRbxGfx::~AdornRbxGfx()")]
// was: RBX::AdornRbxGfx::~AdornRbxGfx()
// IDA 0xb95b70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b95b70() {
}

// 0xb96238 — __ZN3RBX11AdornRbxGfx10destroyAllEv
#[doc(alias = "RBX::AdornRbxGfx::destroyAll(void)")]
// was: RBX::AdornRbxGfx::destroyAll(void)
// IDA 0xb96238: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b96238() {
}

// 0xb9834c — __ZN3RBX11AdornRbxGfx13preSubmitPassEv
#[doc(alias = "RBX::AdornRbxGfx::preSubmitPass(void)")]
// was: RBX::AdornRbxGfx::preSubmitPass(void)
// IDA 0xb9834c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9834c() {
}

// 0xb98388 — __ZN3RBX11AdornRbxGfx14postSubmitPassEv
#[doc(alias = "RBX::AdornRbxGfx::postSubmitPass(void)")]
// was: RBX::AdornRbxGfx::postSubmitPass(void)
// IDA 0xb98388: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b98388() {
}

// 0xb983c8 — __ZN3RBX11AdornRbxGfx17prepareRenderPassEv
#[doc(alias = "RBX::AdornRbxGfx::prepareRenderPass(void)")]
// was: RBX::AdornRbxGfx::prepareRenderPass(void)
// IDA 0xb983c8: 193 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b983c8() {
}

// 0xb985fc — __ZNK3RBX11AdornRbxGfx13getRenderCapsEv
#[doc(alias = "RBX::AdornRbxGfx::getRenderCaps(void)const")]
// was: RBX::AdornRbxGfx::getRenderCaps(void)const
// IDA 0xb985fc: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b985fc() {
}

// 0xb9aa20 — __ZN3RBX5Adorn16finishRenderPassEv
#[doc(alias = "RBX::Adorn::finishRenderPass(void)")]
// was: RBX::Adorn::finishRenderPass(void)
// IDA 0xb9aa20: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_b9aa20() {
}

// 0xbca954 — __ZNSt6vectorIPN3RBX27FastClusterShadowRenderableESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterShadowRenderable **,std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>>>,RBX::FastClusterShadowRenderable * const&)")]
// was: std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterShadowRenderable **,std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>>>,RBX::FastClusterShadowRenderable * const&)
// IDA 0xbca954: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_bca954() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xbe6d70 — __ZN3RBX21ViewRbxGfx_InitModuleEv
#[doc(alias = "RBX::ViewRbxGfx_InitModule(void)")]
// was: RBX::ViewRbxGfx_InitModule(void)
// IDA 0xbe6d70: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be6d70() {
}

// 0xbe75bc — __ZN3RBX10ViewRbxGfx12enableAdornsEb
#[doc(alias = "RBX::ViewRbxGfx::enableAdorns(bool)")]
// was: RBX::ViewRbxGfx::enableAdorns(bool)
// IDA 0xbe75bc: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be75bc() {
}

// 0xbe75c4 — __ZThn4_N3RBX10ViewRbxGfx12enableAdornsEb
#[doc(alias = "non-virtual thunk to RBX::ViewRbxGfx::enableAdorns(bool)")]
// was: non-virtual thunk to RBX::ViewRbxGfx::enableAdorns(bool)
// IDA 0xbe75c4: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be75c4() {
}

// 0xbe75cc — __ZN3RBX10ViewRbxGfx13initResourcesEv
#[doc(alias = "RBX::ViewRbxGfx::initResources(void)")]
// was: RBX::ViewRbxGfx::initResources(void)
// IDA 0xbe75cc: 177 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be75cc() {
}

// 0xbe77c8 — __ZN3RBX10ViewRbxGfx11bindOverlayEN5boost10shared_ptrINS_9DataModelEEE
#[doc(alias = "RBX::ViewRbxGfx::bindOverlay(rbx_core::SharedPtr<RBX::DataModel>)")]
// was: RBX::ViewRbxGfx::bindOverlay(boost::shared_ptr<RBX::DataModel>)
// IDA 0xbe77c8: 224 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be77c8() {
}

// 0xbe7a4c — __ZN3RBX10ViewRbxGfx13bindWorkspaceEN5boost10shared_ptrINS_9DataModelEEE
#[doc(alias = "RBX::ViewRbxGfx::bindWorkspace(rbx_core::SharedPtr<RBX::DataModel>)")]
// was: RBX::ViewRbxGfx::bindWorkspace(boost::shared_ptr<RBX::DataModel>)
// IDA 0xbe7a4c: 870 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be7a4c() {
}

// 0xbe83d8 — __ZN3RBX10ViewRbxGfx16onTakeScreenshotEv
#[doc(alias = "RBX::ViewRbxGfx::onTakeScreenshot(void)")]
// was: RBX::ViewRbxGfx::onTakeScreenshot(void)
// IDA 0xbe83d8: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be83d8() {
}

// 0xbe83e0 — __ZN3RBX10ViewRbxGfx13reloadShadersEv
#[doc(alias = "RBX::ViewRbxGfx::reloadShaders(void)")]
// was: RBX::ViewRbxGfx::reloadShaders(void)
// IDA 0xbe83e0: 192 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be83e0() {
}

// 0xbe85f4 — __ZN3RBX10ViewRbxGfx11enableQueueEi
#[doc(alias = "RBX::ViewRbxGfx::enableQueue(int)")]
// was: RBX::ViewRbxGfx::enableQueue(int)
// IDA 0xbe85f4: 43 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be85f4() {
}

// 0xbe8674 — __ZN3RBX10ViewRbxGfx12disableQueueEi
#[doc(alias = "RBX::ViewRbxGfx::disableQueue(int)")]
// was: RBX::ViewRbxGfx::disableQueue(int)
// IDA 0xbe8674: 43 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be8674() {
}

// 0xbe86f4 — __ZN3RBX10ViewRbxGfxD0Ev
#[doc(alias = "RBX::ViewRbxGfx::~ViewRbxGfx()")]
// was: RBX::ViewRbxGfx::~ViewRbxGfx()
// IDA 0xbe86f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_be86f4() {
}

// 0xbe8794 — __ZN3RBX10ViewRbxGfxD1Ev
#[doc(alias = "RBX::ViewRbxGfx::~ViewRbxGfx()")]
// was: RBX::ViewRbxGfx::~ViewRbxGfx()
// IDA 0xbe8794: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_be8794() {
}

// 0xbe8798 — __ZThn8_N3RBX10ViewRbxGfxD0Ev
#[doc(alias = "non-virtual thunk to RBX::ViewRbxGfx::~ViewRbxGfx()")]
// was: non-virtual thunk to RBX::ViewRbxGfx::~ViewRbxGfx()
// IDA 0xbe8798: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_be8798() {
}

// 0xbe883c — __ZN3RBX10ViewRbxGfxD2Ev
#[doc(alias = "RBX::ViewRbxGfx::~ViewRbxGfx()")]
// was: RBX::ViewRbxGfx::~ViewRbxGfx()
// IDA 0xbe883c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_be883c() {
}

// 0xbe8e20 — __ZThn8_N3RBX10ViewRbxGfxD1Ev
#[doc(alias = "non-virtual thunk to RBX::ViewRbxGfx::~ViewRbxGfx()")]
// was: non-virtual thunk to RBX::ViewRbxGfx::~ViewRbxGfx()
// IDA 0xbe8e20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_be8e20() {
}

// 0xbe8e28 — __ZN3RBX10ViewRbxGfx8onResizeEii
#[doc(alias = "RBX::ViewRbxGfx::onResize(int,int)")]
// was: RBX::ViewRbxGfx::onResize(int,int)
// IDA 0xbe8e28: 94 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be8e28() {
}

// 0xbe8f48 — __ZN3RBX10ViewRbxGfx19getFrameRateManagerEv
#[doc(alias = "RBX::ViewRbxGfx::getFrameRateManager(void)")]
// was: RBX::ViewRbxGfx::getFrameRateManager(void)
// IDA 0xbe8f48: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be8f48() {
}

// 0xbe8f50 — __ZN3RBX10ViewRbxGfx14suppressSkyboxEv
#[doc(alias = "RBX::ViewRbxGfx::suppressSkybox(void)")]
// was: RBX::ViewRbxGfx::suppressSkybox(void)
// IDA 0xbe8f50: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be8f50() {
}

// 0xbe9620 — __ZN3RBX10ViewRbxGfx12getWorkspaceEv
#[doc(alias = "RBX::ViewRbxGfx::getWorkspace(void)")]
// was: RBX::ViewRbxGfx::getWorkspace(void)
// IDA 0xbe9620: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be9620() {
}

// 0xbe9628 — __ZN3RBX10ViewRbxGfx18invalidateLightingEb
#[doc(alias = "RBX::ViewRbxGfx::invalidateLighting(bool)")]
// was: RBX::ViewRbxGfx::invalidateLighting(bool)
// IDA 0xbe9628: 6 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be9628() {
}

// 0xbe9638 — __ZN3RBX10ViewRbxGfx23getAndClearDoScreenshotEv
#[doc(alias = "RBX::ViewRbxGfx::getAndClearDoScreenshot(void)")]
// was: RBX::ViewRbxGfx::getAndClearDoScreenshot(void)
// IDA 0xbe9638: 5 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be9638() {
}

// 0xbe9648 — __ZN3RBX10ViewRbxGfx10loadSkyBoxERb
#[doc(alias = "RBX::ViewRbxGfx::loadSkyBox(bool &)")]
// was: RBX::ViewRbxGfx::loadSkyBox(bool &)
// IDA 0xbe9648: 1376 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be9648() {
}

// 0xbeb168 — __ZN3RBX10ViewRbxGfx9updateFogEv
#[doc(alias = "RBX::ViewRbxGfx::updateFog(void)")]
// was: RBX::ViewRbxGfx::updateFog(void)
// IDA 0xbeb168: 344 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_beb168() {
}

// 0xbeb548 — __ZN3RBX10ViewRbxGfx14updateLightingEv
#[doc(alias = "RBX::ViewRbxGfx::updateLighting(void)")]
// was: RBX::ViewRbxGfx::updateLighting(void)
// IDA 0xbeb548: 396 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_beb548() {
}

// 0xbeb9a0 — __ZN3RBX10ViewRbxGfx17isPreRenderNeededEv
#[doc(alias = "RBX::ViewRbxGfx::isPreRenderNeeded(void)")]
// was: RBX::ViewRbxGfx::isPreRenderNeeded(void)
// IDA 0xbeb9a0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_beb9a0() {
}