//! rendering shard 479 — 100 stubs EA-sorted asc Ogre|G3D|Gfx|Render
//! Filter: Ogre|Gfx|Render|G3D (15058 total, 10271 remaining before batch, 100 this batch = 10171 after)
//! Range 0x940c50..0xb61d98; global dedup vs /tmp/global_eas.txt (66066 EAs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc filtered, skipping global set

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

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

// 0x95f0d8 — __ZN3RBXlsERN6RakNet9BitStreamERKN3G3D7Vector3E
#[doc(alias = "RBX::operator<<(RakNet::BitStream &,G3D::Vector3 const&)")]
// was: RBX::operator<<(RakNet::BitStream &,G3D::Vector3 const&)
// IDA 0x95f0d8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f0d8() {
}

// 0x95f144 — __ZN3RBXlsERN6RakNet9BitStreamERKN3G3D6Color3E
#[doc(alias = "RBX::operator<<(RakNet::BitStream &,G3D::Color3 const&)")]
// was: RBX::operator<<(RakNet::BitStream &,G3D::Color3 const&)
// IDA 0x95f144: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f144() {
}

// 0x95f168 — __ZN3RBX7Network16writeBrickVectorERN6RakNet9BitStreamERKN3G3D7Vector3E
#[doc(alias = "RBX::Network::writeBrickVector(RakNet::BitStream &,G3D::Vector3 const&)")]
// was: RBX::Network::writeBrickVector(RakNet::BitStream &,G3D::Vector3 const&)
// IDA 0x95f168: 106 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f168() {
}

// 0x95f2cc — __ZN3RBX7Network15readBrickVectorERN6RakNet9BitStreamERN3G3D7Vector3E
#[doc(alias = "RBX::Network::readBrickVector(RakNet::BitStream &,G3D::Vector3 &)")]
// was: RBX::Network::readBrickVector(RakNet::BitStream &,G3D::Vector3 &)
// IDA 0x95f2cc: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f2cc() {
}

// 0x95f664 — __ZN3RBXlsERN6RakNet9BitStreamERKN3G3D7Vector2E
#[doc(alias = "RBX::operator<<(RakNet::BitStream &,G3D::Vector2 const&)")]
// was: RBX::operator<<(RakNet::BitStream &,G3D::Vector2 const&)
// IDA 0x95f664: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f664() {
}

// 0x95f69c — __ZN3RBXrsIN3G3D7Vector2EEERN6RakNet9BitStreamES5_RT_
#[doc(alias = "RakNet::BitStream & RBX::operator>><G3D::Vector2>(RakNet::BitStream &,G3D::Vector2 &)")]
// was: RakNet::BitStream & RBX::operator>><G3D::Vector2>(RakNet::BitStream &,G3D::Vector2 &)
// IDA 0x95f69c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f69c() {
}

// 0x95f7dc — __ZN3RBXrsIN3G3D7Vector3EEERN6RakNet9BitStreamES5_RT_
#[doc(alias = "RakNet::BitStream & RBX::operator>><G3D::Vector3>(RakNet::BitStream &,G3D::Vector3 &)")]
// was: RakNet::BitStream & RBX::operator>><G3D::Vector3>(RakNet::BitStream &,G3D::Vector3 &)
// IDA 0x95f7dc: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f7dc() {
}

// 0x95f828 — __ZN3RBXlsERN6RakNet9BitStreamERKN3G3D12Vector3int16E
#[doc(alias = "RBX::operator<<(RakNet::BitStream &,G3D::Vector3int16 const&)")]
// was: RBX::operator<<(RakNet::BitStream &,G3D::Vector3int16 const&)
// IDA 0x95f828: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f828() {
}

// 0x95f864 — __ZN3RBXrsIN3G3D12Vector3int16EEERN6RakNet9BitStreamES5_RT_
#[doc(alias = "RakNet::BitStream & RBX::operator>><G3D::Vector3int16>(RakNet::BitStream &,G3D::Vector3int16 &)")]
// was: RakNet::BitStream & RBX::operator>><G3D::Vector3int16>(RakNet::BitStream &,G3D::Vector3int16 &)
// IDA 0x95f864: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f864() {
}

// 0x95f884 — __ZN3RBXlsERN6RakNet9BitStreamERKN3G3D12Vector2int16E
#[doc(alias = "RBX::operator<<(RakNet::BitStream &,G3D::Vector2int16 const&)")]
// was: RBX::operator<<(RakNet::BitStream &,G3D::Vector2int16 const&)
// IDA 0x95f884: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f884() {
}

// 0x95f8b0 — __ZN3RBXrsIN3G3D12Vector2int16EEERN6RakNet9BitStreamES5_RT_
#[doc(alias = "RakNet::BitStream & RBX::operator>><G3D::Vector2int16>(RakNet::BitStream &,G3D::Vector2int16 &)")]
// was: RakNet::BitStream & RBX::operator>><G3D::Vector2int16>(RakNet::BitStream &,G3D::Vector2int16 &)
// IDA 0x95f8b0: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f8b0() {
}

// 0x95f8c8 — __ZN3RBXlsERN6RakNet9BitStreamERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::operator<<(RakNet::BitStream &,G3D::CoordinateFrame const&)")]
// was: RBX::operator<<(RakNet::BitStream &,G3D::CoordinateFrame const&)
// IDA 0x95f8c8: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f8c8() {
}

// 0x95f9d0 — __ZN3RBXrsIN3G3D15CoordinateFrameEEERN6RakNet9BitStreamES5_RT_
#[doc(alias = "RakNet::BitStream & RBX::operator>><G3D::CoordinateFrame>(RakNet::BitStream &,G3D::CoordinateFrame &)")]
// was: RakNet::BitStream & RBX::operator>><G3D::CoordinateFrame>(RakNet::BitStream &,G3D::CoordinateFrame &)
// IDA 0x95f9d0: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95f9d0() {
}

// 0x95fde0 — __ZN3RBXrsIN3G3D6Color3EEERN6RakNet9BitStreamES5_RT_
#[doc(alias = "RakNet::BitStream & RBX::operator>><G3D::Color3>(RakNet::BitStream &,G3D::Color3 &)")]
// was: RakNet::BitStream & RBX::operator>><G3D::Color3>(RakNet::BitStream &,G3D::Color3 &)
// IDA 0x95fde0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_95fde0() {
}

// 0x97f8f4 — __ZN3RBX7Network16ClientReplicator16streamOutTerrainERKN3G3D12Vector3int16E
#[doc(alias = "RBX::Network::ClientReplicator::streamOutTerrain(G3D::Vector3int16 const&)")]
// was: RBX::Network::ClientReplicator::streamOutTerrain(G3D::Vector3int16 const&)
// IDA 0x97f8f4: 69 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_97f8f4() {
}

// 0x988ad8 — __ZN3RBX7Network10Compressor13writeRotationERN6RakNet9BitStreamERKN3G3D7Matrix3ENS1_15CompressionTypeE
#[doc(alias = "RBX::Network::Compressor::writeRotation(RakNet::BitStream &,G3D::Matrix3 const&,RBX::Network::Compressor::CompressionType)")]
// was: RBX::Network::Compressor::writeRotation(RakNet::BitStream &,G3D::Matrix3 const&,RBX::Network::Compressor::CompressionType)
// IDA 0x988ad8: 114 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_988ad8() {
}

// 0x988c40 — __ZN3RBX7Network10Compressor16writeTranslationERN6RakNet9BitStreamERKN3G3D7Vector3ENS1_15CompressionTypeE
#[doc(alias = "RBX::Network::Compressor::writeTranslation(RakNet::BitStream &,G3D::Vector3 const&,RBX::Network::Compressor::CompressionType)")]
// was: RBX::Network::Compressor::writeTranslation(RakNet::BitStream &,G3D::Vector3 const&,RBX::Network::Compressor::CompressionType)
// IDA 0x988c40: 146 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_988c40() {
}

// 0x988e14 — __ZN3RBX7Network10Compressor12readRotationERN6RakNet9BitStreamERN3G3D7Matrix3E
#[doc(alias = "RBX::Network::Compressor::readRotation(RakNet::BitStream &,G3D::Matrix3 &)")]
// was: RBX::Network::Compressor::readRotation(RakNet::BitStream &,G3D::Matrix3 &)
// IDA 0x988e14: 202 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_988e14() {
}

// 0x989268 — __ZN3RBX7Network10Compressor15readTranslationERN6RakNet9BitStreamERN3G3D7Vector3E
#[doc(alias = "RBX::Network::Compressor::readTranslation(RakNet::BitStream &,G3D::Vector3 &)")]
// was: RBX::Network::Compressor::readTranslation(RakNet::BitStream &,G3D::Vector3 &)
// IDA 0x989268: 405 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_989268() {
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

// 0xa2f32c — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS8_5list3INS8_5valueISD_EENS_3argILi1EEENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS8_5list3INS8_5valueISD_EENS_3argILi1EEENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS8_5list3INS8_5valueISD_EENS_3argILi1EEENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// IDA 0xa2f32c: 163 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a2f32c() {
}

// 0xa2f508 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// IDA 0xa2f508: 165 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a2f508() {
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
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)
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

// 0xb168d0 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE18encodeFromPositionINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_9CellChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *)const")]
// was: void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *)const
// IDA 0xb168d0: 512 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b168d0() {
}

// 0xb16e90 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE18encodeFromPositionINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_5ChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *)const")]
// was: void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *)const
// IDA 0xb16e90: 441 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b16e90() {
}

// 0xb17b18 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE18encodeFromPositionINS_7Network19ClusterUpdateBufferEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_9CellChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *)const")]
// was: void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *)const
// IDA 0xb17b18: 478 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b17b18() {
}

// 0xb180c4 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE18encodeFromPositionINS_7Network19ClusterUpdateBufferEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_5ChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *)const")]
// was: void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeFromPosition<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::Voxel::Grid const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *)const
// IDA 0xb180c4: 373 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b180c4() {
}

// 0xb18de4 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE18encodeFromPositionINS_19ClusterCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_9CellChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::ClusterCellIterator &,RakNet::BitStream *)const")]
// was: void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::ClusterCellIterator &,RakNet::BitStream *)const
// IDA 0xb18de4: 512 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b18de4() {
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

// 0xb46a70 — __ZN3RBX7Network23ErrorCompPhysicsSender26Nugget17computeDeltaErrorERKN3G3D15CoordinateFrameEPKNS_13ModelInstanceEi
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::Nugget::computeDeltaError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)")]
// was: RBX::Network::ErrorCompPhysicsSender2::Nugget::computeDeltaError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)
// IDA 0xb46a70: 194 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b46a70() {
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
