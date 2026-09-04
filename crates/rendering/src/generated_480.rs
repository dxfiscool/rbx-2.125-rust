//! rendering generated — next 100 stubs
//! Filter: Ogre|Gfx|Render|G3D (15058 total) — 0x940c50..0xb6b648, 100 UNIQUE EAs not in global set
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(
    non_snake_case,
    dead_code,
    unused_variables,
    unused_imports,
    clippy::all
)]

use rbx_core::SharedPtr;

// 0x940c50 — __ZN3RBX12SceneUpdater26queueInvalidateAttachementEPNS_13GfxAttachmentE
#[doc(alias = "RBX::SceneUpdater::queueInvalidateAttachement(RBX::GfxAttachment *)")]
// was: RBX::SceneUpdater::queueInvalidateAttachement(RBX::GfxAttachment *)
// IDA 0x940c50: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_940c50() {
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

// 0x94d5a4 — __ZN3RBX9DrawAdorn26lineSegmentRelativeToCoordEPNS_5AdornERKN3G3D15CoordinateFrameERKNS3_7Vector3ES9_RKNS3_6Color3Ef
#[doc(alias = "RBX::DrawAdorn::lineSegmentRelativeToCoord(RBX::Adorn *,G3D::CoordinateFrame const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Color3 const&,float)")]
// was: RBX::DrawAdorn::lineSegmentRelativeToCoord(RBX::Adorn *,G3D::CoordinateFrame const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Color3 const&,float)
// IDA 0x94d5a4: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94d5a4() {
}

// 0x94d854 — __ZN3RBX9DrawAdorn22polygonRelativeToCoordEPNS_5AdornERKN3G3D15CoordinateFrameERSt6vectorINS3_7Vector3ESaIS8_EERKNS3_6Color4Ef
#[doc(alias = "RBX::DrawAdorn::polygonRelativeToCoord(RBX::Adorn *,G3D::CoordinateFrame const&,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> &,G3D::Color4 const&,float)")]
// was: RBX::DrawAdorn::polygonRelativeToCoord(RBX::Adorn *,G3D::CoordinateFrame const&,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> &,G3D::Color4 const&,float)
// IDA 0x94d854: 235 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94d854() {
}

// 0x94dba8 — __ZN3RBX9DrawAdorn27scaleHandleRelativeToCameraERKN3G3D7Vector3ENS_10HandleTypeES4_
#[doc(alias = "RBX::DrawAdorn::scaleHandleRelativeToCamera(G3D::Vector3 const&,RBX::HandleType,G3D::Vector3 const&)")]
// was: RBX::DrawAdorn::scaleHandleRelativeToCamera(G3D::Vector3 const&,RBX::HandleType,G3D::Vector3 const&)
// IDA 0x94dba8: 38 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94dba8() {
}

// 0x94dc38 — __ZN3RBX9DrawAdorn9handles2dERKN3G3D7Vector3ERKNS1_15CoordinateFrameERKNS_6CameraEPNS_5AdornENS_10HandleTypeERKNS1_6Color4Ei
#[doc(alias = "RBX::DrawAdorn::handles2d(G3D::Vector3 const&,G3D::CoordinateFrame const&,RBX::Camera const&,RBX::Adorn *,RBX::HandleType,G3D::Color4 const&,int)")]
// was: RBX::DrawAdorn::handles2d(G3D::Vector3 const&,G3D::CoordinateFrame const&,RBX::Camera const&,RBX::Adorn *,RBX::HandleType,G3D::Color4 const&,int)
// IDA 0x94dc38: 281 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94dc38() {
}

// 0x94dfe8 — __ZN3RBX9DrawAdorn9handles3dERKN3G3D7Vector3ERKNS1_15CoordinateFrameEPNS_5AdornENS_10HandleTypeES4_RKNS1_6Color4EiNS_8NormalIdESD_
#[doc(alias = "RBX::DrawAdorn::handles3d(G3D::Vector3 const&,G3D::CoordinateFrame const&,RBX::Adorn *,RBX::HandleType,G3D::Vector3 const&,G3D::Color4 const&,int,RBX::NormalId,G3D::Color4 const&)")]
// was: RBX::DrawAdorn::handles3d(G3D::Vector3 const&,G3D::CoordinateFrame const&,RBX::Adorn *,RBX::HandleType,G3D::Vector3 const&,G3D::Color4 const&,int,RBX::NormalId,G3D::Color4 const&)
// IDA 0x94dfe8: 559 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94dfe8() {
}

// 0x94e680 — __ZN3RBX9DrawAdorn5torusEPNS_5AdornERKN3G3D15CoordinateFrameENS_8NormalIdEffRKNS3_6Color4E
#[doc(alias = "RBX::DrawAdorn::torus(RBX::Adorn *,G3D::CoordinateFrame const&,RBX::NormalId,float,float,G3D::Color4 const&)")]
// was: RBX::DrawAdorn::torus(RBX::Adorn *,G3D::CoordinateFrame const&,RBX::NormalId,float,float,G3D::Color4 const&)
// IDA 0x94e680: 126 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94e680() {
}

// 0x94e7c8 — __ZN3RBX9DrawAdorn4starEPNS_5AdornERKN3G3D7Vector3EfRKNS3_6Color4ES9_S9_
#[doc(alias = "RBX::DrawAdorn::star(RBX::Adorn *,G3D::Vector3 const&,float,G3D::Color4 const&,G3D::Color4 const&,G3D::Color4 const&)")]
// was: RBX::DrawAdorn::star(RBX::Adorn *,G3D::Vector3 const&,float,G3D::Color4 const&,G3D::Color4 const&,G3D::Color4 const&)
// IDA 0x94e7c8: 140 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94e7c8() {
}

// 0x94e9c0 — __ZN3RBX9DrawAdorn10outlineBoxEPNS_5AdornERKN3G3D5AABoxERKNS3_6Color4E
#[doc(alias = "RBX::DrawAdorn::outlineBox(RBX::Adorn *,G3D::AABox const&,G3D::Color4 const&)")]
// was: RBX::DrawAdorn::outlineBox(RBX::Adorn *,G3D::AABox const&,G3D::Color4 const&)
// IDA 0x94e9c0: 192 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94e9c0() {
}

// 0x9af470 — __ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::append(RBX::AssemblyItem * const&)")]
// was: G3D::Array<RBX::AssemblyItem *,10,32ul>::append(RBX::AssemblyItem * const&)
// IDA 0x9af470: 72 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9af470() {
}

// 0x9af52c — __ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::CompactCFrame,10,32ul>::resize(int,bool)
// IDA 0x9af52c: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9af52c() {
}

// 0x9af5f8 — __ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::AssemblyItem *,10,32ul>::realloc(int)
// IDA 0x9af5f8: 147 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9af5f8() {
}

// 0x9af7e0 — __ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_
#[doc(alias = "RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&)
// IDA 0x9af7e0: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9af7e0() {
}

// 0x9bf340 — __ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_f
#[doc(alias = "RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&,float)")]
// was: RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&,float)
// IDA 0x9bf340: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9bf340() {
}

// 0x9c3280 — __ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::CompactCFrame,10,32ul>::realloc(int)
// IDA 0x9c3280: 153 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9c3280() {
}

// 0x9c52b0 — __ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::~Array()")]
// was: G3D::Array<RBX::CompactCFrame,10,32ul>::~Array()
// IDA 0x9c52b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_9c52b0() {
}

// 0x9c54e8 — __ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::CompactCFrame,10,32ul>::Array(void)
// IDA 0x9c54e8: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9c54e8() {
}

// 0xa168dc — __ZN3RBX7Network7Players12remoteInsertEiSsN3G3D7Vector3E
#[doc(alias = "RBX::Network::Players::remoteInsert(int,std::string,G3D::Vector3)")]
// was: RBX::Network::Players::remoteInsert(int,std::string,G3D::Vector3)
// IDA 0xa168dc: 362 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a168dc() {
}

// 0xa25b0c — __ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::append(RBX::Region2::WeightedPoint const&)")]
// was: G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::append(RBX::Region2::WeightedPoint const&)
// IDA 0xa25b0c: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a25b0c() {
}

// 0xa25b98 — __ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::resize(int,bool)
// IDA 0xa25b98: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a25b98() {
}

// 0xa25c88 — __ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::realloc(int)
// IDA 0xa25c88: 154 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a25c88() {
}

// 0xa302f8 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEENS2_IN3G3D7Vector3EEEEC2ES8_SA_SD_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::list3(boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>)")]
// was: boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::list3(boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>)
// IDA 0xa302f8: 160 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a302f8() {
}

// 0xa304c0 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEENS2_IN3G3D7Vector3EEEEC2ES8_SA_SD_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>)")]
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>)
// IDA 0xa304c0: 164 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a304c0() {
}

// 0xa85560 — __ZN3RBX7Network6Player21distanceFromCharacterEN3G3D7Vector3E
#[doc(alias = "RBX::Network::Player::distanceFromCharacter(G3D::Vector3)")]
// was: RBX::Network::Player::distanceFromCharacter(G3D::Vector3)
// IDA 0xa85560: 152 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a85560() {
}

// 0xa88bcc — __ZNK3RBX7Network6Player16hasCharacterHeadERN3G3D15CoordinateFrameE
#[doc(alias = "RBX::Network::Player::hasCharacterHead(G3D::CoordinateFrame &)const")]
// was: RBX::Network::Player::hasCharacterHead(G3D::CoordinateFrame &)const
// IDA 0xa88bcc: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a88bcc() {
}

// 0xa97910 — __ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEED1Ev
#[doc(alias = "rbx::remote_signal<void ()(std::string,G3D::Vector3)>::~remote_signal()")]
// was: rbx::remote_signal<void ()(std::string,G3D::Vector3)>::~remote_signal()
// IDA 0xa97910: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a97910() {
}

// 0xaab5c0 — __ZN3rbx7signals16signal_with_argsILi2EFvSsN3G3D7Vector3EEEclESsS3_
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,G3D::Vector3)>::operator()(std::string,G3D::Vector3)")]
// was: rbx::signals::signal_with_args<2,void ()(std::string,G3D::Vector3)>::operator()(std::string,G3D::Vector3)
// IDA 0xaab5c0: 291 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_aab5c0() {
}

// 0xaab8ec — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> &)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> &)
// IDA 0xaab8ec: 186 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_aab8ec() {
}

// 0xaac66c — __ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEEC2Ev
#[doc(alias = "rbx::remote_signal<void ()(std::string,G3D::Vector3)>::remote_signal(void)")]
// was: rbx::remote_signal<void ()(std::string,G3D::Vector3)>::remote_signal(void)
// IDA 0xaac66c: 194 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_aac66c() {
}

// 0xaac86c — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::disconnectAll(void)
// IDA 0xaac86c: 158 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_aac86c() {
}

// 0xab09a0 — __ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEE7connectIN5boost8functionIS3_EEEENS_7signals10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string,G3D::Vector3)>::connect<boost::function<void ()(std::string,G3D::Vector3)>>(boost::function<void ()(std::string,G3D::Vector3)> const&)")]
// was: rbx::signals::connection rbx::remote_signal<void ()(std::string,G3D::Vector3)>::connect<boost::function<void ()(std::string,G3D::Vector3)>>(boost::function<void ()(std::string,G3D::Vector3)> const&)
// IDA 0xab09a0: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ab09a0() {
}

// 0xab0b64 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost8functionIS4_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::function<void ()(std::string,G3D::Vector3)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::function<void ()(std::string,G3D::Vector3)>>::~callable_slot()
// IDA 0xab0b64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ab0b64() {
}

// 0xab0b70 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost8functionIS4_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::function<void ()(std::string,G3D::Vector3)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::function<void ()(std::string,G3D::Vector3)>>::~callable_slot()
// IDA 0xab0b70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ab0b70() {
}

// 0xab0c24 — __ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)")]
// was: rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)
// IDA 0xab0c24: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ab0c24() {
}

// 0xab0d4c — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)
// IDA 0xab0d4c: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ab0d4c() {
}

// 0xab0e74 — __ZNK5boost9function2IvSsN3G3D7Vector3EEclESsS2_
#[doc(alias = "boost::function2<void,std::string,G3D::Vector3>::operator()(std::string,G3D::Vector3)const")]
// was: boost::function2<void,std::string,G3D::Vector3>::operator()(std::string,G3D::Vector3)const
// IDA 0xab0e74: 182 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ab0e74() {
}

// 0xab107c — __ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_ED2Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()
// IDA 0xab107c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ab107c() {
}

// 0xab1214 — __ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()
// IDA 0xab1214: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ab1214() {
}

// 0xab1220 — __ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()
// IDA 0xab1220: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ab1220() {
}

// 0xb16e90 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE18encodeFromPositionINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_5ChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *)const")]
// was: void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *)const
// IDA 0xb16e90: 441 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b16e90() {
}

// 0xb180c4 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE18encodeFromPositionINS_7Network19ClusterUpdateBufferEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_5ChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *)const")]
// was: void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *)const
// IDA 0xb180c4: 373 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b180c4() {
}

// 0xb193d8 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE18encodeFromPositionINS_19ClusterCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_5ChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::ClusterCellIterator &,RakNet::BitStream *)const")]
// was: void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::ClusterCellIterator &,RakNet::BitStream *)const
// IDA 0xb193d8: 397 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b193d8() {
}

// 0xb1f4e0 — __ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2int16>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<G3D::Vector2int16>::construct_func(char const*,char *)
// IDA 0xb1f4e0: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b1f4e0() {
}

// 0xb1f4f0 — __ZN3rbx14implementation12typed_holderIN3G3D12Vector3int16EE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3int16>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<G3D::Vector3int16>::construct_func(char const*,char *)
// IDA 0xb1f4f0: 7 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b1f4f0() {
}

// 0xb1f500 — __ZN3rbx14implementation12typed_holderIN3G3D12Vector3int16EE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3int16>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<G3D::Vector3int16>::destruct_func(char *)
// IDA 0xb1f500: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_b1f500() {
}

// 0xb1f508 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector3EE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<G3D::Vector3>::destruct_func(char *)
// IDA 0xb1f508: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_b1f508() {
}

// 0xb1f510 — __ZN3rbx14implementation12typed_holderIN3G3D6Color3EE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Color3>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<G3D::Color3>::construct_func(char const*,char *)
// IDA 0xb1f510: 10 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b1f510() {
}

// 0xb34bf8 — __ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::~Array()")]
// was: G3D::Array<RBX::AssemblyItem *,10,32ul>::~Array()
// IDA 0xb34bf8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b34bf8() {
}

// 0xb34d18 — __ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::AssemblyItem *,10,32ul>::Array(void)
// IDA 0xb34d18: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b34d18() {
}

// 0xb4d71c — __ZN3RBX7Network19ClusterUpdateBuffer4pushERKN3G3D12Vector3int16E
#[doc(alias = "RBX::Network::ClusterUpdateBuffer::push(G3D::Vector3int16 const&)")]
// was: RBX::Network::ClusterUpdateBuffer::push(G3D::Vector3int16 const&)
// IDA 0xb4d71c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4d71c() {
}

// 0xb4d770 — __ZN3RBX7Network19ClusterUpdateBuffer3chkERKN3G3D12Vector3int16E
#[doc(alias = "RBX::Network::ClusterUpdateBuffer::chk(G3D::Vector3int16 const&)")]
// was: RBX::Network::ClusterUpdateBuffer::chk(G3D::Vector3int16 const&)
// IDA 0xb4d770: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4d770() {
}

// 0xb4d7b8 — __ZN3RBX7Network19ClusterUpdateBuffer3popEPN3G3D12Vector3int16E
#[doc(alias = "RBX::Network::ClusterUpdateBuffer::pop(G3D::Vector3int16 *)")]
// was: RBX::Network::ClusterUpdateBuffer::pop(G3D::Vector3int16 *)
// IDA 0xb4d7b8: 94 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b4d7b8() {
}

// 0xb596c8 — __ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator11resetCenterERKN3G3D7Vector3Eb
#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamRegionIterator::resetCenter(G3D::Vector3 const&,bool)")]
// was: RBX::Network::Replicator::StreamJob::StreamRegionIterator::resetCenter(G3D::Vector3 const&,bool)
// IDA 0xb596c8: 240 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b596c8() {
}

// 0xb5ad0c — __ZNK3RBX7Network10Replicator9StreamJob33isTerrainRegionCollectedByCellPosEN3G3D12Vector3int16ERNS_12StreamRegion2IdE
#[doc(alias = "RBX::Network::Replicator::StreamJob::isTerrainRegionCollectedByCellPos(G3D::Vector3int16,RBX::StreamRegion::Id &)const")]
// was: RBX::Network::Replicator::StreamJob::isTerrainRegionCollectedByCellPos(G3D::Vector3int16,RBX::StreamRegion::Id &)const
// IDA 0xb5ad0c: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5ad0c() {
}

// 0xb5b270 — __ZN3RBX7Network10Replicator9StreamJob15setStreamCenterERKN3G3D7Vector3Eb
#[doc(alias = "RBX::Network::Replicator::StreamJob::setStreamCenter(G3D::Vector3 const&,bool)")]
// was: RBX::Network::Replicator::StreamJob::setStreamCenter(G3D::Vector3 const&,bool)
// IDA 0xb5b270: 62 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b5b270() {
}

// 0xb616c8 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE6insertEPNS5_4slotE
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::insert(rbx::signals::signal<void ()(G3D::Vector3)>::slot *)")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::insert(rbx::signals::signal<void ()(G3D::Vector3)>::slot *)
// IDA 0xb616c8: 249 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b616c8() {
}

// 0xb6197c — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE5mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::mutex(void)
// IDA 0xb6197c: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6197c() {
}

// 0xb61a90 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotEEaSEPS8_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3)>::slot*)
// IDA 0xb61a90: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b61a90() {
}

// 0xb61b48 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::safe_static_init_mutex(void)
// IDA 0xb61b48: 79 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b61b48() {
}

// 0xb61c30 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIbNS7_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS3_bEENS8_5list3INS8_5valueIPSF_EENS7_3argILi1EEENSK_IbEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()
// IDA 0xb61c30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b61c30() {
}

// 0xb61c8c — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIbNS7_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS3_bEENS8_5list3INS8_5valueIPSF_EENS7_3argILi1EEENSK_IbEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()
// IDA 0xb61c8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b61c8c() {
}

// 0xb61d98 — __ZNK3rbx7signals6signalIFvN3G3D7Vector3EEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::slot::connected(void)const
// IDA 0xb61d98: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b61d98() {
}

// 0xb61da4 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIbNS8_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS4_bEENS9_5list3INS9_5valueIPSG_EENS8_3argILi1EEENSL_IbEEEEEELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)
// IDA 0xb61da4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b61da4() {
}

// 0xb61dd0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIbNS8_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS4_bEENS9_5list3INS9_5valueIPSG_EENS8_3argILi1EEENSL_IbEEEEEELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)
// IDA 0xb61dd0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b61dd0() {
}

// 0xb61e00 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::slot::safe_static_init_mutex(void)
// IDA 0xb61e00: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b61e00() {
}

// 0xb61ee8 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::slot::~slot()
// IDA 0xb61ee8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b61ee8() {
}

// 0xb68e84 — __ZNSt3mapISsPN4Ogre17VertexDeclarationESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
#[doc(alias = "std::map<std::string,Ogre::VertexDeclaration *,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::VertexDeclaration *,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::operator[](std::string const&)
// IDA 0xb68e84: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b68e84() {
}

// 0xb69040 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::pair<std::string const,Ogre::VertexDeclaration *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::pair<std::string const,Ogre::VertexDeclaration *> const&)
// IDA 0xb69040: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b69040() {
}

// 0xb69220 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::VertexDeclaration *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::VertexDeclaration *> const&)
// IDA 0xb69220: 122 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b69220() {
}

// 0xb69368 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::pair<std::string const,Ogre::VertexDeclaration *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_insert_unique(std::pair<std::string const,Ogre::VertexDeclaration *> const&)
// IDA 0xb69368: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b69368() {
}

// 0xb6944c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17VertexDeclarationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::VertexDeclaration *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexDeclaration *>,std::_Select1st<std::pair<std::string const,Ogre::VertexDeclaration *>>,std::less<std::string>,std::allocator<std::pair<std::string const,Ogre::VertexDeclaration *>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::VertexDeclaration *>> *)
// IDA 0xb6944c: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6944c() {
}

// 0xb6a2c8 — __ZN3RBX26FastClusterShadowGenerator20getVertexDeclarationEPN4Ogre12VisualEngineE
#[doc(alias = "RBX::FastClusterShadowGenerator::getVertexDeclaration(Ogre::VisualEngine *)")]
// was: RBX::FastClusterShadowGenerator::getVertexDeclaration(Ogre::VisualEngine *)
// IDA 0xb6a2c8: 130 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6a2c8() {
}

// 0xb6a438 — __ZN3RBX26FastClusterShadowGenerator16createVertexDataEPN4Ogre12VisualEngineERKSt6vectorINS0_6VertexESaIS5_EEj
#[doc(alias = "RBX::FastClusterShadowGenerator::createVertexData(Ogre::VisualEngine *,std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>> const&,unsigned int)")]
// was: RBX::FastClusterShadowGenerator::createVertexData(Ogre::VisualEngine *,std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>> const&,unsigned int)
// IDA 0xb6a438: 279 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6a438() {
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
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")]
// was: non-virtual thunk toRBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()
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
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()")]
// was: non-virtual thunk toRBX::FastClusterShadowRenderable::~FastClusterShadowRenderable()
// IDA 0xb6b1e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b6b1e0() {
}

// 0xb6b1e8 — __ZN3RBX27FastClusterShadowRenderable14generateVolumeEPKN4Ogre5LightEfmPtj
#[doc(alias = "RBX::FastClusterShadowRenderable::generateVolume(Ogre::Light const*,float,unsigned long,unsigned short *,unsigned int)")]
// was: RBX::FastClusterShadowRenderable::generateVolume(Ogre::Light const*,float,unsigned long,unsigned short *,unsigned int)
// IDA 0xb6b1e8: 377 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b1e8() {
}

// 0xb6b620 — __ZNK3RBX27FastClusterShadowRenderable18getWorldTransformsEPN4Ogre7Matrix4E
#[doc(alias = "RBX::FastClusterShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: RBX::FastClusterShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xb6b620: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b620() {
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
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getCastShadows(void)const")]
// was: non-virtual thunk toRBX::FastClusterShadowRenderable::getCastShadows(void)const
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
#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getEdgeList(void)")]
// was: non-virtual thunk toRBX::FastClusterShadowRenderable::getEdgeList(void)
// IDA 0xb6b644: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b644() {
}

// 0xb6b648 — __ZN3RBX27FastClusterShadowRenderable11hasEdgeListEv
#[doc(alias = "RBX::FastClusterShadowRenderable::hasEdgeList(void)")]
// was: RBX::FastClusterShadowRenderable::hasEdgeList(void)
// IDA 0xb6b648: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6b648() {
}
