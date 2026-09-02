//! rendering shard wdog_rend_1788392398 — 120 stubs 0x940c50..0xb70fe0 EA-sorted asc (Ogre|Gfx|Render|G3D|Adorn filtered, global dedup vs /tmp/global_eas.txt, gap filler if needed)
//! Source: ida/export.json (85545 funcs) EA asc not in /tmp/global_eas.txt — next 120 rendering-filtered uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x940c50 — __ZN3RBX12SceneUpdater26queueInvalidateAttachementEPNS_13GfxAttachmentE
#[doc(alias = "RBX::SceneUpdater::queueInvalidateAttachement(RBX::GfxAttachment *)")]
#[doc(alias = "__ZN3RBX12SceneUpdater26queueInvalidateAttachementEPNS_13GfxAttachmentE")]
pub fn stub_0x940c50() -> ! {
    todo!("0x940c50 RBX::SceneUpdater::queueInvalidateAttachement(RBX::GfxAttachment *)")
}

// 0x9417b8 — __ZN3RBX12SceneUpdater11notifyAwakeEPNS_13GfxAttachmentE
#[doc(alias = "RBX::SceneUpdater::notifyAwake(RBX::GfxAttachment *)")]
#[doc(alias = "__ZN3RBX12SceneUpdater11notifyAwakeEPNS_13GfxAttachmentE")]
pub fn stub_0x9417b8() -> ! {
    todo!("0x9417b8 RBX::SceneUpdater::notifyAwake(RBX::GfxAttachment *)")
}

// 0x941884 — __ZN3RBX12SceneUpdater14notifySleepingEPNS_13GfxAttachmentE
#[doc(alias = "RBX::SceneUpdater::notifySleeping(RBX::GfxAttachment *)")]
#[doc(alias = "__ZN3RBX12SceneUpdater14notifySleepingEPNS_13GfxAttachmentE")]
pub fn stub_0x941884() -> ! {
    todo!("0x941884 RBX::SceneUpdater::notifySleeping(RBX::GfxAttachment *)")
}

// 0x942ef8 — __ZSt4swapIN5boost9unordered13unordered_setIPN3RBX13GfxAttachmentENS0_4hashIS5_EESt8equal_toIS5_ESaIS5_EEEEvRT_SD_
#[doc(alias = "void std::swap<boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>>>(boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &,boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &)")]
#[doc(alias = "__ZSt4swapIN5boost9unordered13unordered_setIPN3RBX13GfxAttachmentENS0_4hashIS5_EESt8equal_toIS5_ESaIS5_EEEEvRT_SD_")]
pub fn stub_0x942ef8() -> ! {
    todo!("0x942ef8 void std::swap<boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>>>(boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &,boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &)")
}

// 0x943af8 — __ZNSt6vectorIPN3RBX13GfxAttachmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxAttachment **,std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>>,RBX::GfxAttachment * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13GfxAttachmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x943af8() -> ! {
    todo!("0x943af8 std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxAttachment **,std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>>,RBX::GfxAttachment * const&)")
}

// 0x943e20 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *,boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_")]
pub fn stub_0x943e20() -> ! {
    todo!("0x943e20 boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *,boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *)")
}

// 0x943eb8 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE6assignERKSD_NS1_17integral_constantIbLb0EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE6assignERKSD_NS1_17integral_constantIbLb0EEE")]
pub fn stub_0x943eb8() -> ! {
    todo!("0x943eb8 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")
}

// 0x9440f0 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm")]
pub fn stub_0x9440f0() -> ! {
    todo!("0x9440f0 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::create_buckets(unsigned long)")
}

// 0x9441a0 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISC_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEERSH_RT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>> &)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISC_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEERSH_RT_")]
pub fn stub_0x9441a0() -> ! {
    todo!("0x9441a0 void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>> &)")
}

// 0x9442b8 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2ERKSD_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::table_impl(boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2ERKSD_")]
pub fn stub_0x9442b8() -> ! {
    todo!("0x9442b8 boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::table_impl(boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)")
}

// 0x944438 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE4initERKSD_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE4initERKSD_")]
pub fn stub_0x944438() -> ! {
    todo!("0x944438 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)")
}

// 0x94451c — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeIS6_EEEEEEEvNS0_15iterator_detail8iteratorISH_EERNS1_5tableISC_EERT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>> &)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeIS6_EEEEEEEvNS0_15iterator_detail8iteratorISH_EERNS1_5tableISC_EERT_")]
pub fn stub_0x94451c() -> ! {
    todo!("0x94451c void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>> &)")
}

// 0x94460c — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxAttachment *>>(RBX::GfxAttachment * const&,boost::unordered::detail::emplace_args1<RBX::GfxAttachment *> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_")]
pub fn stub_0x94460c() -> ! {
    todo!("0x94460c std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxAttachment *>>(RBX::GfxAttachment * const&,boost::unordered::detail::emplace_args1<RBX::GfxAttachment *> const&)")
}

// 0x9447d8 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm")]
pub fn stub_0x9447d8() -> ! {
    todo!("0x9447d8 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::reserve_for_insert(unsigned long)")
}

// 0x94a8f0 — __ZN3RBX4Draw6spokesEffPNS_5AdornE
#[doc(alias = "RBX::Draw::spokes(float,float,RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX4Draw6spokesEffPNS_5AdornE")]
pub fn stub_0x94a8f0() -> ! {
    todo!("0x94a8f0 RBX::Draw::spokes(float,float,RBX::Adorn *)")
}

// 0x94c984 — __ZN3RBX9DrawAdorn10axisWidgetEPNS_5AdornERKNS_6CameraE
#[doc(alias = "RBX::DrawAdorn::axisWidget(RBX::Adorn *,RBX::Camera const&)")]
#[doc(alias = "__ZN3RBX9DrawAdorn10axisWidgetEPNS_5AdornERKNS_6CameraE")]
pub fn stub_0x94c984() -> ! {
    todo!("0x94c984 RBX::DrawAdorn::axisWidget(RBX::Adorn *,RBX::Camera const&)")
}

// 0x94d5a4 — __ZN3RBX9DrawAdorn26lineSegmentRelativeToCoordEPNS_5AdornERKN3G3D15CoordinateFrameERKNS3_7Vector3ES9_RKNS3_6Color3Ef
#[doc(alias = "RBX::DrawAdorn::lineSegmentRelativeToCoord(RBX::Adorn *,G3D::CoordinateFrame const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Color3 const&,float)")]
#[doc(alias = "__ZN3RBX9DrawAdorn26lineSegmentRelativeToCoordEPNS_5AdornERKN3G3D15CoordinateFrameERKNS3_7Vector3ES9_RKNS3_6Color3Ef")]
pub fn stub_0x94d5a4() -> ! {
    todo!("0x94d5a4 RBX::DrawAdorn::lineSegmentRelativeToCoord(RBX::Adorn *,G3D::CoordinateFrame const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Color3 const&,float)")
}

// 0x94d854 — __ZN3RBX9DrawAdorn22polygonRelativeToCoordEPNS_5AdornERKN3G3D15CoordinateFrameERSt6vectorINS3_7Vector3ESaIS8_EERKNS3_6Color4Ef
#[doc(alias = "RBX::DrawAdorn::polygonRelativeToCoord(RBX::Adorn *,G3D::CoordinateFrame const&,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> &,G3D::Color4 const&,float)")]
#[doc(alias = "__ZN3RBX9DrawAdorn22polygonRelativeToCoordEPNS_5AdornERKN3G3D15CoordinateFrameERSt6vectorINS3_7Vector3ESaIS8_EERKNS3_6Color4Ef")]
pub fn stub_0x94d854() -> ! {
    todo!("0x94d854 RBX::DrawAdorn::polygonRelativeToCoord(RBX::Adorn *,G3D::CoordinateFrame const&,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> &,G3D::Color4 const&,float)")
}

// 0x94dba8 — __ZN3RBX9DrawAdorn27scaleHandleRelativeToCameraERKN3G3D7Vector3ENS_10HandleTypeES4_
#[doc(alias = "RBX::DrawAdorn::scaleHandleRelativeToCamera(G3D::Vector3 const&,RBX::HandleType,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX9DrawAdorn27scaleHandleRelativeToCameraERKN3G3D7Vector3ENS_10HandleTypeES4_")]
pub fn stub_0x94dba8() -> ! {
    todo!("0x94dba8 RBX::DrawAdorn::scaleHandleRelativeToCamera(G3D::Vector3 const&,RBX::HandleType,G3D::Vector3 const&)")
}

// 0x94dc38 — __ZN3RBX9DrawAdorn9handles2dERKN3G3D7Vector3ERKNS1_15CoordinateFrameERKNS_6CameraEPNS_5AdornENS_10HandleTypeERKNS1_6Color4Ei
#[doc(alias = "RBX::DrawAdorn::handles2d(G3D::Vector3 const&,G3D::CoordinateFrame const&,RBX::Camera const&,RBX::Adorn *,RBX::HandleType,G3D::Color4 const&,int)")]
#[doc(alias = "__ZN3RBX9DrawAdorn9handles2dERKN3G3D7Vector3ERKNS1_15CoordinateFrameERKNS_6CameraEPNS_5AdornENS_10HandleTypeERKNS1_6Color4Ei")]
pub fn stub_0x94dc38() -> ! {
    todo!("0x94dc38 RBX::DrawAdorn::handles2d(G3D::Vector3 const&,G3D::CoordinateFrame const&,RBX::Camera const&,RBX::Adorn *,RBX::HandleType,G3D::Color4 const&,int)")
}

// 0x94dfe8 — __ZN3RBX9DrawAdorn9handles3dERKN3G3D7Vector3ERKNS1_15CoordinateFrameEPNS_5AdornENS_10HandleTypeES4_RKNS1_6Color4EiNS_8NormalIdESD_
#[doc(alias = "RBX::DrawAdorn::handles3d(G3D::Vector3 const&,G3D::CoordinateFrame const&,RBX::Adorn *,RBX::HandleType,G3D::Vector3 const&,G3D::Color4 const&,int,RBX::NormalId,G3D::Color4 const&)")]
#[doc(alias = "__ZN3RBX9DrawAdorn9handles3dERKN3G3D7Vector3ERKNS1_15CoordinateFrameEPNS_5AdornENS_10HandleTypeES4_RKNS1_6Color4EiNS_8NormalIdESD_")]
pub fn stub_0x94dfe8() -> ! {
    todo!("0x94dfe8 RBX::DrawAdorn::handles3d(G3D::Vector3 const&,G3D::CoordinateFrame const&,RBX::Adorn *,RBX::HandleType,G3D::Vector3 const&,G3D::Color4 const&,int,RBX::NormalId,G3D::Color4 const&)")
}

// 0x94e680 — __ZN3RBX9DrawAdorn5torusEPNS_5AdornERKN3G3D15CoordinateFrameENS_8NormalIdEffRKNS3_6Color4E
#[doc(alias = "RBX::DrawAdorn::torus(RBX::Adorn *,G3D::CoordinateFrame const&,RBX::NormalId,float,float,G3D::Color4 const&)")]
#[doc(alias = "__ZN3RBX9DrawAdorn5torusEPNS_5AdornERKN3G3D15CoordinateFrameENS_8NormalIdEffRKNS3_6Color4E")]
pub fn stub_0x94e680() -> ! {
    todo!("0x94e680 RBX::DrawAdorn::torus(RBX::Adorn *,G3D::CoordinateFrame const&,RBX::NormalId,float,float,G3D::Color4 const&)")
}

// 0x94e7c8 — __ZN3RBX9DrawAdorn4starEPNS_5AdornERKN3G3D7Vector3EfRKNS3_6Color4ES9_S9_
#[doc(alias = "RBX::DrawAdorn::star(RBX::Adorn *,G3D::Vector3 const&,float,G3D::Color4 const&,G3D::Color4 const&,G3D::Color4 const&)")]
#[doc(alias = "__ZN3RBX9DrawAdorn4starEPNS_5AdornERKN3G3D7Vector3EfRKNS3_6Color4ES9_S9_")]
pub fn stub_0x94e7c8() -> ! {
    todo!("0x94e7c8 RBX::DrawAdorn::star(RBX::Adorn *,G3D::Vector3 const&,float,G3D::Color4 const&,G3D::Color4 const&,G3D::Color4 const&)")
}

// 0x94e9c0 — __ZN3RBX9DrawAdorn10outlineBoxEPNS_5AdornERKN3G3D5AABoxERKNS3_6Color4E
#[doc(alias = "RBX::DrawAdorn::outlineBox(RBX::Adorn *,G3D::AABox const&,G3D::Color4 const&)")]
#[doc(alias = "__ZN3RBX9DrawAdorn10outlineBoxEPNS_5AdornERKN3G3D5AABoxERKNS3_6Color4E")]
pub fn stub_0x94e9c0() -> ! {
    todo!("0x94e9c0 RBX::DrawAdorn::outlineBox(RBX::Adorn *,G3D::AABox const&,G3D::Color4 const&)")
}

// 0x9af470 — __ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::append(RBX::AssemblyItem * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EE6appendERKS3_")]
pub fn stub_0x9af470() -> ! {
    todo!("0x9af470 G3D::Array<RBX::AssemblyItem *,10,32ul>::append(RBX::AssemblyItem * const&)")
}

// 0x9af52c — __ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EE6resizeEib")]
pub fn stub_0x9af52c() -> ! {
    todo!("0x9af52c G3D::Array<RBX::CompactCFrame,10,32ul>::resize(int,bool)")
}

// 0x9af5f8 — __ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EE7reallocEi")]
pub fn stub_0x9af5f8() -> ! {
    todo!("0x9af5f8 G3D::Array<RBX::AssemblyItem *,10,32ul>::realloc(int)")
}

// 0x9af7e0 — __ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_
#[doc(alias = "RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_")]
pub fn stub_0x9af7e0() -> ! {
    todo!("0x9af7e0 RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&)")
}

// 0x9bf340 — __ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_f
#[doc(alias = "RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&,float)")]
#[doc(alias = "__ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_f")]
pub fn stub_0x9bf340() -> ! {
    todo!("0x9bf340 RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&,float)")
}

// 0x9c3280 — __ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EE7reallocEi")]
pub fn stub_0x9c3280() -> ! {
    todo!("0x9c3280 G3D::Array<RBX::CompactCFrame,10,32ul>::realloc(int)")
}

// 0x9c52b0 — __ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EED2Ev")]
pub fn stub_0x9c52b0() -> ! {
    todo!("0x9c52b0 G3D::Array<RBX::CompactCFrame,10,32ul>::~Array()")
}

// 0x9c54e8 — __ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EEC2Ev")]
pub fn stub_0x9c54e8() -> ! {
    todo!("0x9c54e8 G3D::Array<RBX::CompactCFrame,10,32ul>::Array(void)")
}

// 0xa25b0c — __ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::append(RBX::Region2::WeightedPoint const&)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE6appendERKS3_")]
pub fn stub_0xa25b0c() -> ! {
    todo!("0xa25b0c G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::append(RBX::Region2::WeightedPoint const&)")
}

// 0xa25b98 — __ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE6resizeEib")]
pub fn stub_0xa25b98() -> ! {
    todo!("0xa25b98 G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::resize(int,bool)")
}

// 0xa25c88 — __ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE7reallocEi")]
pub fn stub_0xa25c88() -> ! {
    todo!("0xa25c88 G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::realloc(int)")
}

// 0xa85560 — __ZN3RBX7Network6Player21distanceFromCharacterEN3G3D7Vector3E
#[doc(alias = "RBX::Network::Player::distanceFromCharacter(G3D::Vector3)")]
#[doc(alias = "__ZN3RBX7Network6Player21distanceFromCharacterEN3G3D7Vector3E")]
pub fn stub_0xa85560() -> ! {
    todo!("0xa85560 RBX::Network::Player::distanceFromCharacter(G3D::Vector3)")
}

// 0xa8899c — __ZN3RBX7Network6Player20renderStreamedRegionEPNS_5AdornE
#[doc(alias = "RBX::Network::Player::renderStreamedRegion(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX7Network6Player20renderStreamedRegionEPNS_5AdornE")]
pub fn stub_0xa8899c() -> ! {
    todo!("0xa8899c RBX::Network::Player::renderStreamedRegion(RBX::Adorn *)")
}

// 0xa889c4 — __ZN3RBX7Network6Player20renderDPhysicsRegionEPNS_5AdornE
#[doc(alias = "RBX::Network::Player::renderDPhysicsRegion(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX7Network6Player20renderDPhysicsRegionEPNS_5AdornE")]
pub fn stub_0xa889c4() -> ! {
    todo!("0xa889c4 RBX::Network::Player::renderDPhysicsRegion(RBX::Adorn *)")
}

// 0xa88bcc — __ZNK3RBX7Network6Player16hasCharacterHeadERN3G3D15CoordinateFrameE
#[doc(alias = "RBX::Network::Player::hasCharacterHead(G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX7Network6Player16hasCharacterHeadERN3G3D15CoordinateFrameE")]
pub fn stub_0xa88bcc() -> ! {
    todo!("0xa88bcc RBX::Network::Player::hasCharacterHead(G3D::CoordinateFrame &)const")
}

// 0xa97910 — __ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEED1Ev
#[doc(alias = "rbx::remote_signal<void ()(std::string,G3D::Vector3)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEED1Ev")]
pub fn stub_0xa97910() -> ! {
    todo!("0xa97910 rbx::remote_signal<void ()(std::string,G3D::Vector3)>::~remote_signal()")
}

// 0xaab5c0 — __ZN3rbx7signals16signal_with_argsILi2EFvSsN3G3D7Vector3EEEclESsS3_
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,G3D::Vector3)>::operator()(std::string,G3D::Vector3)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvSsN3G3D7Vector3EEEclESsS3_")]
pub fn stub_0xaab5c0() -> ! {
    todo!("0xaab5c0 rbx::signals::signal_with_args<2,void ()(std::string,G3D::Vector3)>::operator()(std::string,G3D::Vector3)")
}

// 0xaab8ec — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
pub fn stub_0xaab8ec() -> ! {
    todo!("0xaab8ec rbx::signals::signal<void ()(std::string,G3D::Vector3)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> &)")
}

// 0xaac66c — __ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEEC2Ev
#[doc(alias = "rbx::remote_signal<void ()(std::string,G3D::Vector3)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEEC2Ev")]
pub fn stub_0xaac66c() -> ! {
    todo!("0xaac66c rbx::remote_signal<void ()(std::string,G3D::Vector3)>::remote_signal(void)")
}

// 0xaac86c — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13disconnectAllEv")]
pub fn stub_0xaac86c() -> ! {
    todo!("0xaac86c rbx::signals::signal<void ()(std::string,G3D::Vector3)>::disconnectAll(void)")
}

// 0xab09a0 — __ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEE7connectIN5boost8functionIS3_EEEENS_7signals10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string,G3D::Vector3)>::connect<boost::function<void ()(std::string,G3D::Vector3)>>(boost::function<void ()(std::string,G3D::Vector3)> const&)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEE7connectIN5boost8functionIS3_EEEENS_7signals10connectionERKT_")]
pub fn stub_0xab09a0() -> ! {
    todo!("0xab09a0 rbx::signals::connection rbx::remote_signal<void ()(std::string,G3D::Vector3)>::connect<boost::function<void ()(std::string,G3D::Vector3)>>(boost::function<void ()(std::string,G3D::Vector3)> const&)")
}

// 0xab0b64 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost8functionIS4_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::function<void ()(std::string,G3D::Vector3)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost8functionIS4_EEED1Ev")]
pub fn stub_0xab0b64() -> ! {
    todo!("0xab0b64 rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::function<void ()(std::string,G3D::Vector3)>>::~callable_slot()")
}

// 0xab0b70 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost8functionIS4_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::function<void ()(std::string,G3D::Vector3)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost8functionIS4_EEED0Ev")]
pub fn stub_0xab0b70() -> ! {
    todo!("0xab0b70 rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::function<void ()(std::string,G3D::Vector3)>>::~callable_slot()")
}

// 0xab0c24 — __ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_")]
pub fn stub_0xab0c24() -> ! {
    todo!("0xab0c24 rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)")
}

// 0xab0d4c — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_")]
pub fn stub_0xab0d4c() -> ! {
    todo!("0xab0d4c `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)")
}

// 0xab0e74 — __ZNK5boost9function2IvSsN3G3D7Vector3EEclESsS2_
#[doc(alias = "boost::function2<void,std::string,G3D::Vector3>::operator()(std::string,G3D::Vector3)const")]
#[doc(alias = "__ZNK5boost9function2IvSsN3G3D7Vector3EEclESsS2_")]
pub fn stub_0xab0e74() -> ! {
    todo!("0xab0e74 boost::function2<void,std::string,G3D::Vector3>::operator()(std::string,G3D::Vector3)const")
}

// 0xab107c — __ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_ED2Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_ED2Ev")]
pub fn stub_0xab107c() -> ! {
    todo!("0xab107c rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()")
}

// 0xab1214 — __ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev")]
pub fn stub_0xab1214() -> ! {
    todo!("0xab1214 rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()")
}

// 0xab1220 — __ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev")]
pub fn stub_0xab1220() -> ! {
    todo!("0xab1220 rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()")
}

// 0xb16e90 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE18encodeFromPositionINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_5ChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *)const")]
#[doc(alias = "__ZNK3RBX5Voxel10SerializerINS0_4GridEE18encodeFromPositionINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_5ChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_")]
pub fn stub_0xb16e90() -> ! {
    todo!("0xb16e90 void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *)const")
}

// 0xb180c4 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE18encodeFromPositionINS_7Network19ClusterUpdateBufferEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_5ChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *)const")]
#[doc(alias = "__ZNK3RBX5Voxel10SerializerINS0_4GridEE18encodeFromPositionINS_7Network19ClusterUpdateBufferEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_5ChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_")]
pub fn stub_0xb180c4() -> ! {
    todo!("0xb180c4 void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *)const")
}

// 0xb193d8 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE18encodeFromPositionINS_19ClusterCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_5ChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::ClusterCellIterator &,RakNet::BitStream *)const")]
#[doc(alias = "__ZNK3RBX5Voxel10SerializerINS0_4GridEE18encodeFromPositionINS_19ClusterCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_5ChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_")]
pub fn stub_0xb193d8() -> ! {
    todo!("0xb193d8 void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::ClusterCellIterator &,RakNet::BitStream *)const")
}

// 0xb1f4e0 — __ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2int16>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE14construct_funcEPKcPc")]
pub fn stub_0xb1f4e0() -> ! {
    todo!("0xb1f4e0 rbx::implementation::typed_holder<G3D::Vector2int16>::construct_func(char const*,char *)")
}

// 0xb1f4f0 — __ZN3rbx14implementation12typed_holderIN3G3D12Vector3int16EE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3int16>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D12Vector3int16EE14construct_funcEPKcPc")]
pub fn stub_0xb1f4f0() -> ! {
    todo!("0xb1f4f0 rbx::implementation::typed_holder<G3D::Vector3int16>::construct_func(char const*,char *)")
}

// 0xb1f500 — __ZN3rbx14implementation12typed_holderIN3G3D12Vector3int16EE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3int16>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D12Vector3int16EE13destruct_funcEPc")]
pub fn stub_0xb1f500() -> ! {
    todo!("0xb1f500 rbx::implementation::typed_holder<G3D::Vector3int16>::destruct_func(char *)")
}

// 0xb1f508 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector3EE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D7Vector3EE13destruct_funcEPc")]
pub fn stub_0xb1f508() -> ! {
    todo!("0xb1f508 rbx::implementation::typed_holder<G3D::Vector3>::destruct_func(char *)")
}

// 0xb1f510 — __ZN3rbx14implementation12typed_holderIN3G3D6Color3EE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Color3>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D6Color3EE14construct_funcEPKcPc")]
pub fn stub_0xb1f510() -> ! {
    todo!("0xb1f510 rbx::implementation::typed_holder<G3D::Color3>::construct_func(char const*,char *)")
}

// 0xb34bf8 — __ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EED2Ev")]
pub fn stub_0xb34bf8() -> ! {
    todo!("0xb34bf8 G3D::Array<RBX::AssemblyItem *,10,32ul>::~Array()")
}

// 0xb34d18 — __ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EEC2Ev")]
pub fn stub_0xb34d18() -> ! {
    todo!("0xb34d18 G3D::Array<RBX::AssemblyItem *,10,32ul>::Array(void)")
}

// 0xb4d71c — __ZN3RBX7Network19ClusterUpdateBuffer4pushERKN3G3D12Vector3int16E
#[doc(alias = "RBX::Network::ClusterUpdateBuffer::push(G3D::Vector3int16 const&)")]
#[doc(alias = "__ZN3RBX7Network19ClusterUpdateBuffer4pushERKN3G3D12Vector3int16E")]
pub fn stub_0xb4d71c() -> ! {
    todo!("0xb4d71c RBX::Network::ClusterUpdateBuffer::push(G3D::Vector3int16 const&)")
}

// 0xb4d770 — __ZN3RBX7Network19ClusterUpdateBuffer3chkERKN3G3D12Vector3int16E
#[doc(alias = "RBX::Network::ClusterUpdateBuffer::chk(G3D::Vector3int16 const&)")]
#[doc(alias = "__ZN3RBX7Network19ClusterUpdateBuffer3chkERKN3G3D12Vector3int16E")]
pub fn stub_0xb4d770() -> ! {
    todo!("0xb4d770 RBX::Network::ClusterUpdateBuffer::chk(G3D::Vector3int16 const&)")
}

// 0xb4d7b8 — __ZN3RBX7Network19ClusterUpdateBuffer3popEPN3G3D12Vector3int16E
#[doc(alias = "RBX::Network::ClusterUpdateBuffer::pop(G3D::Vector3int16 *)")]
#[doc(alias = "__ZN3RBX7Network19ClusterUpdateBuffer3popEPN3G3D12Vector3int16E")]
pub fn stub_0xb4d7b8() -> ! {
    todo!("0xb4d7b8 RBX::Network::ClusterUpdateBuffer::pop(G3D::Vector3int16 *)")
}

// 0xb596c8 — __ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator11resetCenterERKN3G3D7Vector3Eb
#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamRegionIterator::resetCenter(G3D::Vector3 const&,bool)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator11resetCenterERKN3G3D7Vector3Eb")]
pub fn stub_0xb596c8() -> ! {
    todo!("0xb596c8 RBX::Network::Replicator::StreamJob::StreamRegionIterator::resetCenter(G3D::Vector3 const&,bool)")
}

// 0xb5ad0c — __ZNK3RBX7Network10Replicator9StreamJob33isTerrainRegionCollectedByCellPosEN3G3D12Vector3int16ERNS_12StreamRegion2IdE
#[doc(alias = "RBX::Network::Replicator::StreamJob::isTerrainRegionCollectedByCellPos(G3D::Vector3int16,RBX::StreamRegion::Id &)const")]
#[doc(alias = "__ZNK3RBX7Network10Replicator9StreamJob33isTerrainRegionCollectedByCellPosEN3G3D12Vector3int16ERNS_12StreamRegion2IdE")]
pub fn stub_0xb5ad0c() -> ! {
    todo!("0xb5ad0c RBX::Network::Replicator::StreamJob::isTerrainRegionCollectedByCellPos(G3D::Vector3int16,RBX::StreamRegion::Id &)const")
}

// 0xb5b270 — __ZN3RBX7Network10Replicator9StreamJob15setStreamCenterERKN3G3D7Vector3Eb
#[doc(alias = "RBX::Network::Replicator::StreamJob::setStreamCenter(G3D::Vector3 const&,bool)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob15setStreamCenterERKN3G3D7Vector3Eb")]
pub fn stub_0xb5b270() -> ! {
    todo!("0xb5b270 RBX::Network::Replicator::StreamJob::setStreamCenter(G3D::Vector3 const&,bool)")
}

// 0xb616c8 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE6insertEPNS5_4slotE
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::insert(rbx::signals::signal<void ()(G3D::Vector3)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE6insertEPNS5_4slotE")]
pub fn stub_0xb616c8() -> ! {
    todo!("0xb616c8 rbx::signals::signal<void ()(G3D::Vector3)>::insert(rbx::signals::signal<void ()(G3D::Vector3)>::slot *)")
}

// 0xb6197c — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE5mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE5mutexEv")]
pub fn stub_0xb6197c() -> ! {
    todo!("0xb6197c rbx::signals::signal<void ()(G3D::Vector3)>::mutex(void)")
}

// 0xb61a90 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotEEaSEPS8_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotEEaSEPS8_")]
pub fn stub_0xb61a90() -> ! {
    todo!("0xb61a90 boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3)>::slot*)")
}

// 0xb61b48 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE22safe_static_init_mutexEv")]
pub fn stub_0xb61b48() -> ! {
    todo!("0xb61b48 rbx::signals::signal<void ()(G3D::Vector3)>::safe_static_init_mutex(void)")
}

// 0xb61c30 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIbNS7_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS3_bEENS8_5list3INS8_5valueIPSF_EENS7_3argILi1EEENSK_IbEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIbNS7_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS3_bEENS8_5list3INS8_5valueIPSF_EENS7_3argILi1EEENSK_IbEEEEEEED1Ev")]
pub fn stub_0xb61c30() -> ! {
    todo!("0xb61c30 rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")
}

// 0xb61c8c — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIbNS7_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS3_bEENS8_5list3INS8_5valueIPSF_EENS7_3argILi1EEENSK_IbEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIbNS7_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS3_bEENS8_5list3INS8_5valueIPSF_EENS7_3argILi1EEENSK_IbEEEEEEED0Ev")]
pub fn stub_0xb61c8c() -> ! {
    todo!("0xb61c8c rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")
}

// 0xb61d98 — __ZNK3rbx7signals6signalIFvN3G3D7Vector3EEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3G3D7Vector3EEE4slot9connectedEv")]
pub fn stub_0xb61d98() -> ! {
    todo!("0xb61d98 rbx::signals::signal<void ()(G3D::Vector3)>::slot::connected(void)const")
}

// 0xb61da4 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIbNS8_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS4_bEENS9_5list3INS9_5valueIPSG_EENS8_3argILi1EEENSL_IbEEEEEELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIbNS8_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS4_bEENS9_5list3INS9_5valueIPSG_EENS8_3argILi1EEENSL_IbEEEEEELi1ES5_E4callES4_")]
pub fn stub_0xb61da4() -> ! {
    todo!("0xb61da4 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)")
}

// 0xb61dd0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIbNS8_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS4_bEENS9_5list3INS9_5valueIPSG_EENS8_3argILi1EEENSL_IbEEEEEELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIbNS8_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS4_bEENS9_5list3INS9_5valueIPSG_EENS8_3argILi1EEENSL_IbEEEEEELi1ES5_E4callES4_")]
pub fn stub_0xb61dd0() -> ! {
    todo!("0xb61dd0 `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)")
}

// 0xb61e00 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot22safe_static_init_mutexEv")]
pub fn stub_0xb61e00() -> ! {
    todo!("0xb61e00 rbx::signals::signal<void ()(G3D::Vector3)>::slot::safe_static_init_mutex(void)")
}

// 0xb61ee8 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotD0Ev")]
pub fn stub_0xb61ee8() -> ! {
    todo!("0xb61ee8 rbx::signals::signal<void ()(G3D::Vector3)>::slot::~slot()")
}

// 0xb68e84 — __ZNSt3mapISsPN4Ogre17VertexDeclarationESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
#[doc(alias = "std::map<std::string,Ogre::VertexDeclaration *,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsPN4Ogre17VertexDeclarationESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")]
pub fn stub_0xb68e84() -> ! {
    todo!("0xb68e84 std::map<std::string,Ogre::VertexDeclaration *,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::operator[](std::string const&)")
}

// 0xb69040 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::pair<std::string const,Ogre::VertexDeclaration *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
pub fn stub_0xb69040() -> ! {
    todo!("0xb69040 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::pair<std::string const,Ogre::VertexDeclaration *> const&)")
}

// 0xb69220 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::VertexDeclaration *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
pub fn stub_0xb69220() -> ! {
    todo!("0xb69220 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::VertexDeclaration *> const&)")
}

// 0xb69368 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::pair<std::string const,Ogre::VertexDeclaration *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")]
pub fn stub_0xb69368() -> ! {
    todo!("0xb69368 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::pair<std::string const,Ogre::VertexDeclaration *> const&)")
}

// 0xb6944c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::VertexDeclaration *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
pub fn stub_0xb6944c() -> ! {
    todo!("0xb6944c std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::VertexDeclaration *>> *)")
}

// 0xb6a2c8 — __ZN3RBX26FastClusterShadowGenerator20getVertexDeclarationEPN4Ogre12VisualEngineE
#[doc(alias = "RBX::FastClusterShadowGenerator::getVertexDeclaration(Ogre::VisualEngine *)")]
#[doc(alias = "__ZN3RBX26FastClusterShadowGenerator20getVertexDeclarationEPN4Ogre12VisualEngineE")]
pub fn stub_0xb6a2c8() -> ! {
    todo!("0xb6a2c8 RBX::FastClusterShadowGenerator::getVertexDeclaration(Ogre::VisualEngine *)")
}

// 0xb6a438 — __ZN3RBX26FastClusterShadowGenerator16createVertexDataEPN4Ogre12VisualEngineERKSt6vectorINS0_6VertexESaIS5_EEj
#[doc(alias = "RBX::FastClusterShadowGenerator::createVertexData(Ogre::VisualEngine *,std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>> const&,unsigned int)")]
#[doc(alias = "__ZN3RBX26FastClusterShadowGenerator16createVertexDataEPN4Ogre12VisualEngineERKSt6vectorINS0_6VertexESaIS5_EEj")]
pub fn stub_0xb6a438() -> ! {
    todo!("0xb6a438 RBX::FastClusterShadowGenerator::createVertexData(Ogre::VisualEngine *,std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>> const&,unsigned int)")
}

// 0xb6acd0 — __ZN3RBX27FastClusterShadowRenderableC2EPNS_17FastClusterEntityEPNS_21FastClusterShadowDataE
#[doc(alias = "RBX::FastClusterShadowRenderable::FastClusterShadowRenderable(RBX::FastClusterEntity *,RBX::FastClusterShadowData *)")]
#[doc(alias = "__ZN3RBX27FastClusterShadowRenderableC2EPNS_17FastClusterEntityEPNS_21FastClusterShadowDataE")]
pub fn stub_0xb6acd0() -> ! {
    todo!("0xb6acd0 RBX::FastClusterShadowRenderable::FastClusterShadowRenderable(RBX::FastClusterEntity *,RBX::FastClusterShadowData *)")
}

// 0xb6af24 — __ZN3RBX27FastClusterShadowRenderableD0Ev
#[doc(alias = "RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")]
#[doc(alias = "__ZN3RBX27FastClusterShadowRenderableD0Ev")]
pub fn stub_0xb6af24() -> ! {
    todo!("0xb6af24 RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")
}

// 0xb6afd8 — __ZN3RBX27FastClusterShadowRenderableD1Ev
#[doc(alias = "RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")]
#[doc(alias = "__ZN3RBX27FastClusterShadowRenderableD1Ev")]
pub fn stub_0xb6afd8() -> ! {
    todo!("0xb6afd8 RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")
}

// 0xb6afdc — __ZThn96_N3RBX27FastClusterShadowRenderableD0Ev
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")]
#[doc(alias = "__ZThn96_N3RBX27FastClusterShadowRenderableD0Ev")]
pub fn stub_0xb6afdc() -> ! {
    todo!("0xb6afdc `non-virtual thunk to'RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")
}

// 0xb6b094 — __ZN3RBX27FastClusterShadowRenderableD2Ev
#[doc(alias = "RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")]
#[doc(alias = "__ZN3RBX27FastClusterShadowRenderableD2Ev")]
pub fn stub_0xb6b094() -> ! {
    todo!("0xb6b094 RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")
}

// 0xb6b1e0 — __ZThn96_N3RBX27FastClusterShadowRenderableD1Ev
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")]
#[doc(alias = "__ZThn96_N3RBX27FastClusterShadowRenderableD1Ev")]
pub fn stub_0xb6b1e0() -> ! {
    todo!("0xb6b1e0 `non-virtual thunk to'RBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")
}

// 0xb6b1e8 — __ZN3RBX27FastClusterShadowRenderable14generateVolumeEPKN4Ogre5LightEfmPtj
#[doc(alias = "RBX::FastClusterShadowRenderable::generateVolume(Ogre::Light const*,float,unsigned long,unsigned short *,unsigned int)")]
#[doc(alias = "__ZN3RBX27FastClusterShadowRenderable14generateVolumeEPKN4Ogre5LightEfmPtj")]
pub fn stub_0xb6b1e8() -> ! {
    todo!("0xb6b1e8 RBX::FastClusterShadowRenderable::generateVolume(Ogre::Light const*,float,unsigned long,unsigned short *,unsigned int)")
}

// 0xb6b620 — __ZNK3RBX27FastClusterShadowRenderable18getWorldTransformsEPN4Ogre7Matrix4E
#[doc(alias = "RBX::FastClusterShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")]
#[doc(alias = "__ZNK3RBX27FastClusterShadowRenderable18getWorldTransformsEPN4Ogre7Matrix4E")]
pub fn stub_0xb6b620() -> ! {
    todo!("0xb6b620 RBX::FastClusterShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")
}

// 0xb6b628 — __ZNK3RBX27FastClusterShadowRenderable21getNumWorldTransformsEv
#[doc(alias = "RBX::FastClusterShadowRenderable::getNumWorldTransforms(void)const")]
#[doc(alias = "__ZNK3RBX27FastClusterShadowRenderable21getNumWorldTransformsEv")]
pub fn stub_0xb6b628() -> ! {
    todo!("0xb6b628 RBX::FastClusterShadowRenderable::getNumWorldTransforms(void)const")
}

// 0xb6b638 — __ZNK3RBX27FastClusterShadowRenderable14getCastShadowsEv
#[doc(alias = "RBX::FastClusterShadowRenderable::getCastShadows(void)const")]
#[doc(alias = "__ZNK3RBX27FastClusterShadowRenderable14getCastShadowsEv")]
pub fn stub_0xb6b638() -> ! {
    todo!("0xb6b638 RBX::FastClusterShadowRenderable::getCastShadows(void)const")
}

// 0xb6b63c — __ZThn96_NK3RBX27FastClusterShadowRenderable14getCastShadowsEv
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getCastShadows(void)const")]
#[doc(alias = "__ZThn96_NK3RBX27FastClusterShadowRenderable14getCastShadowsEv")]
pub fn stub_0xb6b63c() -> ! {
    todo!("0xb6b63c `non-virtual thunk to'RBX::FastClusterShadowRenderable::getCastShadows(void)const")
}

// 0xb6b640 — __ZN3RBX27FastClusterShadowRenderable11getEdgeListEv
#[doc(alias = "RBX::FastClusterShadowRenderable::getEdgeList(void)")]
#[doc(alias = "__ZN3RBX27FastClusterShadowRenderable11getEdgeListEv")]
pub fn stub_0xb6b640() -> ! {
    todo!("0xb6b640 RBX::FastClusterShadowRenderable::getEdgeList(void)")
}

// 0xb6b644 — __ZThn96_N3RBX27FastClusterShadowRenderable11getEdgeListEv
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getEdgeList(void)")]
#[doc(alias = "__ZThn96_N3RBX27FastClusterShadowRenderable11getEdgeListEv")]
pub fn stub_0xb6b644() -> ! {
    todo!("0xb6b644 `non-virtual thunk to'RBX::FastClusterShadowRenderable::getEdgeList(void)")
}

// 0xb6b648 — __ZN3RBX27FastClusterShadowRenderable11hasEdgeListEv
#[doc(alias = "RBX::FastClusterShadowRenderable::hasEdgeList(void)")]
#[doc(alias = "__ZN3RBX27FastClusterShadowRenderable11hasEdgeListEv")]
pub fn stub_0xb6b648() -> ! {
    todo!("0xb6b648 RBX::FastClusterShadowRenderable::hasEdgeList(void)")
}

// 0xb6b64c — __ZThn96_N3RBX27FastClusterShadowRenderable11hasEdgeListEv
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::hasEdgeList(void)")]
#[doc(alias = "__ZThn96_N3RBX27FastClusterShadowRenderable11hasEdgeListEv")]
pub fn stub_0xb6b64c() -> ! {
    todo!("0xb6b64c `non-virtual thunk to'RBX::FastClusterShadowRenderable::hasEdgeList(void)")
}

// 0xb6b650 — __ZNK3RBX27FastClusterShadowRenderable19getWorldBoundingBoxEb
#[doc(alias = "RBX::FastClusterShadowRenderable::getWorldBoundingBox(bool)const")]
#[doc(alias = "__ZNK3RBX27FastClusterShadowRenderable19getWorldBoundingBoxEb")]
pub fn stub_0xb6b650() -> ! {
    todo!("0xb6b650 RBX::FastClusterShadowRenderable::getWorldBoundingBox(bool)const")
}

// 0xb6b65c — __ZThn96_NK3RBX27FastClusterShadowRenderable19getWorldBoundingBoxEb
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getWorldBoundingBox(bool)const")]
#[doc(alias = "__ZThn96_NK3RBX27FastClusterShadowRenderable19getWorldBoundingBoxEb")]
pub fn stub_0xb6b65c() -> ! {
    todo!("0xb6b65c `non-virtual thunk to'RBX::FastClusterShadowRenderable::getWorldBoundingBox(bool)const")
}

// 0xb6b668 — __ZNK3RBX27FastClusterShadowRenderable17getLightCapBoundsEv
#[doc(alias = "RBX::FastClusterShadowRenderable::getLightCapBounds(void)const")]
#[doc(alias = "__ZNK3RBX27FastClusterShadowRenderable17getLightCapBoundsEv")]
pub fn stub_0xb6b668() -> ! {
    todo!("0xb6b668 RBX::FastClusterShadowRenderable::getLightCapBounds(void)const")
}

// 0xb6b674 — __ZThn96_NK3RBX27FastClusterShadowRenderable17getLightCapBoundsEv
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getLightCapBounds(void)const")]
#[doc(alias = "__ZThn96_NK3RBX27FastClusterShadowRenderable17getLightCapBoundsEv")]
pub fn stub_0xb6b674() -> ! {
    todo!("0xb6b674 `non-virtual thunk to'RBX::FastClusterShadowRenderable::getLightCapBounds(void)const")
}

// 0xb6b680 — __ZNK3RBX27FastClusterShadowRenderable16getDarkCapBoundsERKN4Ogre5LightEf
#[doc(alias = "RBX::FastClusterShadowRenderable::getDarkCapBounds(Ogre::Light const&,float)const")]
#[doc(alias = "__ZNK3RBX27FastClusterShadowRenderable16getDarkCapBoundsERKN4Ogre5LightEf")]
pub fn stub_0xb6b680() -> ! {
    todo!("0xb6b680 RBX::FastClusterShadowRenderable::getDarkCapBounds(Ogre::Light const&,float)const")
}

// 0xb6b718 — __ZThn96_NK3RBX27FastClusterShadowRenderable16getDarkCapBoundsERKN4Ogre5LightEf
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getDarkCapBounds(Ogre::Light const&,float)const")]
#[doc(alias = "__ZThn96_NK3RBX27FastClusterShadowRenderable16getDarkCapBoundsERKN4Ogre5LightEf")]
pub fn stub_0xb6b718() -> ! {
    todo!("0xb6b718 `non-virtual thunk to'RBX::FastClusterShadowRenderable::getDarkCapBounds(Ogre::Light const&,float)const")
}

// 0xb6b7b0 — __ZN3RBX27FastClusterShadowRenderable33getShadowVolumeRenderableIteratorEN4Ogre15ShadowTechniqueEPKNS1_5LightEPNS1_28HardwareIndexBufferSharedPtrEbfm
#[doc(alias = "RBX::FastClusterShadowRenderable::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")]
#[doc(alias = "__ZN3RBX27FastClusterShadowRenderable33getShadowVolumeRenderableIteratorEN4Ogre15ShadowTechniqueEPKNS1_5LightEPNS1_28HardwareIndexBufferSharedPtrEbfm")]
pub fn stub_0xb6b7b0() -> ! {
    todo!("0xb6b7b0 RBX::FastClusterShadowRenderable::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")
}

// 0xb6b824 — __ZThn96_N3RBX27FastClusterShadowRenderable33getShadowVolumeRenderableIteratorEN4Ogre15ShadowTechniqueEPKNS1_5LightEPNS1_28HardwareIndexBufferSharedPtrEbfm
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")]
#[doc(alias = "__ZThn96_N3RBX27FastClusterShadowRenderable33getShadowVolumeRenderableIteratorEN4Ogre15ShadowTechniqueEPKNS1_5LightEPNS1_28HardwareIndexBufferSharedPtrEbfm")]
pub fn stub_0xb6b824() -> ! {
    todo!("0xb6b824 `non-virtual thunk to'RBX::FastClusterShadowRenderable::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")
}

// 0xb6b898 — __ZNK3RBX27FastClusterShadowRenderable25getPointExtrusionDistanceEPKN4Ogre5LightE
#[doc(alias = "RBX::FastClusterShadowRenderable::getPointExtrusionDistance(Ogre::Light const*)const")]
#[doc(alias = "__ZNK3RBX27FastClusterShadowRenderable25getPointExtrusionDistanceEPKN4Ogre5LightE")]
pub fn stub_0xb6b898() -> ! {
    todo!("0xb6b898 RBX::FastClusterShadowRenderable::getPointExtrusionDistance(Ogre::Light const*)const")
}

// 0xb6b89c — __ZThn96_NK3RBX27FastClusterShadowRenderable25getPointExtrusionDistanceEPKN4Ogre5LightE
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getPointExtrusionDistance(Ogre::Light const*)const")]
#[doc(alias = "__ZThn96_NK3RBX27FastClusterShadowRenderable25getPointExtrusionDistanceEPKN4Ogre5LightE")]
pub fn stub_0xb6b89c() -> ! {
    todo!("0xb6b89c `non-virtual thunk to'RBX::FastClusterShadowRenderable::getPointExtrusionDistance(Ogre::Light const*)const")
}

// 0xb6b8a0 — __ZN3RBX27FastClusterShadowRenderable17rebindIndexBufferERKN4Ogre28HardwareIndexBufferSharedPtrE
#[doc(alias = "RBX::FastClusterShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)")]
#[doc(alias = "__ZN3RBX27FastClusterShadowRenderable17rebindIndexBufferERKN4Ogre28HardwareIndexBufferSharedPtrE")]
pub fn stub_0xb6b8a0() -> ! {
    todo!("0xb6b8a0 RBX::FastClusterShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)")
}

// 0xb6b9a4 — __ZN3RBX17FastClusterEntityC2EPNS_11FastClusterEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrERKSt6vectorIjSaIjEEPNS_21FastClusterShadowDataERKNS3_14AxisAlignedBoxEh
#[doc(alias = "RBX::FastClusterEntity::FastClusterEntity(RBX::FastCluster *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,std::vector<unsigned int,std::allocator<unsigned int>> const&,RBX::FastClusterShadowData *,Ogre::AxisAlignedBox const&,unsigned char)")]
#[doc(alias = "__ZN3RBX17FastClusterEntityC2EPNS_11FastClusterEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrERKSt6vectorIjSaIjEEPNS_21FastClusterShadowDataERKNS3_14AxisAlignedBoxEh")]
pub fn stub_0xb6b9a4() -> ! {
    todo!("0xb6b9a4 RBX::FastClusterEntity::FastClusterEntity(RBX::FastCluster *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,std::vector<unsigned int,std::allocator<unsigned int>> const&,RBX::FastClusterShadowData *,Ogre::AxisAlignedBox const&,unsigned char)")
}

// 0xb6bdcc — __ZNK3RBX17FastClusterEntity18getWorldTransformsEPN4Ogre7Matrix4E
#[doc(alias = "RBX::FastClusterEntity::getWorldTransforms(Ogre::Matrix4 *)const")]
#[doc(alias = "__ZNK3RBX17FastClusterEntity18getWorldTransformsEPN4Ogre7Matrix4E")]
pub fn stub_0xb6bdcc() -> ! {
    todo!("0xb6bdcc RBX::FastClusterEntity::getWorldTransforms(Ogre::Matrix4 *)const")
}

// 0xb6bec0 — __ZNK3RBX17FastClusterEntity19getSquaredViewDepthEPKN4Ogre6CameraE
#[doc(alias = "RBX::FastClusterEntity::getSquaredViewDepth(Ogre::Camera const*)const")]
#[doc(alias = "__ZNK3RBX17FastClusterEntity19getSquaredViewDepthEPKN4Ogre6CameraE")]
pub fn stub_0xb6bec0() -> ! {
    todo!("0xb6bec0 RBX::FastClusterEntity::getSquaredViewDepth(Ogre::Camera const*)const")
}

// 0xb6dff0 — __ZN3RBX11FastCluster18invalidateLightingERKN4Ogre14AxisAlignedBoxE
#[doc(alias = "RBX::FastCluster::invalidateLighting(Ogre::AxisAlignedBox const&)")]
#[doc(alias = "__ZN3RBX11FastCluster18invalidateLightingERKN4Ogre14AxisAlignedBoxE")]
pub fn stub_0xb6dff0() -> ! {
    todo!("0xb6dff0 RBX::FastCluster::invalidateLighting(Ogre::AxisAlignedBox const&)")
}

// 0xb6f208 — __ZN4Ogre16ShadowRenderableD2Ev
#[doc(alias = "Ogre::ShadowRenderable::~ShadowRenderable()")]
#[doc(alias = "__ZN4Ogre16ShadowRenderableD2Ev")]
pub fn stub_0xb6f208() -> ! {
    todo!("0xb6f208 Ogre::ShadowRenderable::~ShadowRenderable()")
}

// 0xb6f488 — __ZNSt6vectorIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(alias = "std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev")]
pub fn stub_0xb6f488() -> ! {
    todo!("0xb6f488 std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")
}

// 0xb70fe0 — __ZNK4Ogre10Renderable12getTechniqueEv
#[doc(alias = "Ogre::Renderable::getTechnique(void)const")]
#[doc(alias = "__ZNK4Ogre10Renderable12getTechniqueEv")]
pub fn stub_0xb70fe0() -> ! {
    todo!("0xb70fe0 Ogre::Renderable::getTechnique(void)const")
}

