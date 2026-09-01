//! core shard EX — 100 core stubs EA-sorted, lowest uncovered 0xc09780..0xc27c18 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EW 0xc08fec).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xc08fec.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::MegaCluster::getSharedVDecl(void)")]
// 0xc09780 — __ZN3RBX11MegaCluster14getSharedVDeclEv
pub fn stub_c09780() -> ! {
    todo!("0xc09780 __ZN3RBX11MegaCluster14getSharedVDeclEv")
}

#[doc(alias = "RBX::Voxel::Water::anonymous namespace::isWaterOnWedge(RBX::Voxel::Cell const&,RBX::Voxel::Water::LocalAreaInfo const&)")]
// 0xc09948 — __ZN3RBX5Voxel5Water12_GLOBAL__N_114isWaterOnWedgeERKNS0_4CellERKNS1_13LocalAreaInfoE
// was: RBX::Voxel::Water::`anonymous namespace'::isWaterOnWedge(RBX::Voxel::Cell const&,RBX::Voxel::Water::LocalAreaInfo const&)
pub fn stub_c09948() -> ! {
    todo!("0xc09948 __ZN3RBX5Voxel5Water12_GLOBAL__N_114isWaterOnWedgeERKNS0_4CellERKNS1_13LocalAreaInfoE")
}

#[doc(alias = "void RBX::MegaClusterLegacy::updateEntity_templated<RBX::MegaClusterLegacy::VoxelGridOverlay>(void)")]
// 0xc09de0 — __ZN3RBX17MegaClusterLegacy22updateEntity_templatedINS0_16VoxelGridOverlayEEEvv
pub fn stub_c09de0() -> ! {
    todo!("0xc09de0 __ZN3RBX17MegaClusterLegacy22updateEntity_templatedINS0_16VoxelGridOverlayEEEvv")
}

#[doc(alias = "void RBX::MegaClusterLegacy::updateChunk_templated<RBX::MegaClusterLegacy::VoxelGridOverlay>(RBX::SpatialRegion::Id const&,bool)")]
// 0xc0a0e0 — __ZN3RBX17MegaClusterLegacy21updateChunk_templatedINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdEb
pub fn stub_c0a0e0() -> ! {
    todo!("0xc0a0e0 __ZN3RBX17MegaClusterLegacy21updateChunk_templatedINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdEb")
}

#[doc(alias = "void RBX::MegaClusterLegacy::unbuild_templated<RBX::MegaClusterLegacy::VoxelGridOverlay>(void)")]
// 0xc0a3a0 — __ZN3RBX17MegaClusterLegacy17unbuild_templatedINS0_16VoxelGridOverlayEEEvv
pub fn stub_c0a3a0() -> ! {
    todo!("0xc0a3a0 __ZN3RBX17MegaClusterLegacy17unbuild_templatedINS0_16VoxelGridOverlayEEEvv")
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xc0a4ec — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_20SolidTerrainRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_c0a4ec() -> ! {
    todo!("0xc0a4ec __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_20SolidTerrainRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xc0acec — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_17WaterFaceRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_c0acec() -> ! {
    todo!("0xc0acec __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_17WaterFaceRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "RBX::GfxBinding::invalidateEntity(void)")]
// 0xc0b430 — __ZN3RBX10GfxBinding16invalidateEntityEv
pub fn stub_c0b430() -> ! {
    todo!("0xc0b430 __ZN3RBX10GfxBinding16invalidateEntityEv")
}

#[doc(alias = "RBX::GfxBinding::onCoordinateFrameChanged(void)")]
// 0xc0b434 — __ZN3RBX10GfxBinding24onCoordinateFrameChangedEv
pub fn stub_c0b434() -> ! {
    todo!("0xc0b434 __ZN3RBX10GfxBinding24onCoordinateFrameChangedEv")
}

#[doc(alias = "RBX::GfxPart::updateCoordinateFrame(bool)")]
// 0xc0b438 — __ZN3RBX7GfxPart21updateCoordinateFrameEb
pub fn stub_c0b438() -> ! {
    todo!("0xc0b438 __ZN3RBX7GfxPart21updateCoordinateFrameEb")
}

#[doc(alias = "RBX::GfxPart::getFastFuzzyExtents(void)")]
// 0xc0b43c — __ZN3RBX7GfxPart19getFastFuzzyExtentsEv
pub fn stub_c0b43c() -> ! {
    todo!("0xc0b43c __ZN3RBX7GfxPart19getFastFuzzyExtentsEv")
}

#[doc(alias = "RBX::GfxPart::getPartCount(void)")]
// 0xc0b4cc — __ZN3RBX7GfxPart12getPartCountEv
pub fn stub_c0b4cc() -> ! {
    todo!("0xc0b4cc __ZN3RBX7GfxPart12getPartCountEv")
}

#[doc(alias = "RBX::GfxPart::onClumpChanged(void)")]
// 0xc0b4d4 — __ZN3RBX7GfxPart14onClumpChangedEv
pub fn stub_c0b4d4() -> ! {
    todo!("0xc0b4d4 __ZN3RBX7GfxPart14onClumpChangedEv")
}

#[doc(alias = "RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
// 0xc0b4d8 — __ZNK3RBX20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE8internalERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionE
pub fn stub_c0b4d8() -> ! {
    todo!("0xc0b4d8 __ZNK3RBX20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE8internalERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionE")
}

#[doc(alias = "RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// 0xc0b66c — __ZN3RBX17WaterFaceRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_c0b66c() -> ! {
    todo!("0xc0b66c __ZN3RBX17WaterFaceRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE")
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xc0bf18 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_c0bf18() -> ! {
    todo!("0xc0bf18 __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// 0xc0c648 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_c0c648() -> ! {
    todo!("0xc0c648 __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE")
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// 0xc0c904 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE9wedgeFaceERKNS1_6RegionINS3_5ChunkEE8iteratorE
pub fn stub_c0c904() -> ! {
    todo!("0xc0c904 __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE9wedgeFaceERKNS1_6RegionINS3_5ChunkEE8iteratorE")
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// 0xc0cd30 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE19detectWedgeOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorE
pub fn stub_c0cd30() -> ! {
    todo!("0xc0cd30 __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE19detectWedgeOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorE")
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// 0xc0cf1c — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12wedgeUpEmptyERKNS1_6RegionINS3_5ChunkEE8iteratorE
pub fn stub_c0cf1c() -> ! {
    todo!("0xc0cf1c __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12wedgeUpEmptyERKNS1_6RegionINS3_5ChunkEE8iteratorE")
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// 0xc0d000 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE14detectOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_c0d000() -> ! {
    todo!("0xc0d000 __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE14detectOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE")
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xc0d418 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_c0d418() -> ! {
    todo!("0xc0d418 __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")]
// 0xc0dbd8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED1Ev
pub fn stub_c0dbd8() -> ! {
    todo!("0xc0dbd8 __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")]
// 0xc0dbdc — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED0Ev
pub fn stub_c0dbdc() -> ! {
    todo!("0xc0dbdc __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::dispose(void)")]
// 0xc0dbe0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE7disposeEv
pub fn stub_c0dbe0() -> ! {
    todo!("0xc0dbe0 __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_deleter(std::type_info const&)")]
// 0xc0dbf0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE11get_deleterERKSt9type_info
pub fn stub_c0dbf0() -> ! {
    todo!("0xc0dbf0 __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_untyped_deleter(void)")]
// 0xc0dbf4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE19get_untyped_deleterEv
pub fn stub_c0dbf4() -> ! {
    todo!("0xc0dbf4 __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::MegaCluster>,boost::_bi::list1<boost::_bi::value<RBX::MegaCluster*>>>>::~callable_slot()")]
// 0xc0dd8c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11MegaClusterEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
pub fn stub_c0dd8c() -> ! {
    todo!("0xc0dd8c __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11MegaClusterEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::MegaCluster>,boost::_bi::list1<boost::_bi::value<RBX::MegaCluster*>>>>::~callable_slot()")]
// 0xc0dde8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11MegaClusterEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
pub fn stub_c0dde8() -> ! {
    todo!("0xc0dde8 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11MegaClusterEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::MegaCluster>,boost::_bi::list1<boost::_bi::value<RBX::MegaCluster*>>>,0,void ()(void)>::call(void)")]
// 0xc0def0 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11MegaClusterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
pub fn stub_c0def0() -> ! {
    todo!("0xc0def0 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11MegaClusterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::MegaCluster>,boost::_bi::list1<boost::_bi::value<RBX::MegaCluster*>>>,0,void ()(void)>::call(void)")]
// 0xc0df08 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11MegaClusterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::MegaCluster>,boost::_bi::list1<boost::_bi::value<RBX::MegaCluster*>>>,0,void ()(void)>::call(void)
pub fn stub_c0df08() -> ! {
    todo!("0xc0df08 __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11MegaClusterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "std::vector<RBX::MegaCluster::ChunkData,std::allocator<RBX::MegaCluster::ChunkData>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::MegaCluster::ChunkData*,std::vector<RBX::MegaCluster::ChunkData,std::allocator<RBX::MegaCluster::ChunkData>>>,unsigned long,RBX::MegaCluster::ChunkData const&)")]
// 0xc0df20 — __ZNSt6vectorIN3RBX11MegaCluster9ChunkDataESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_c0df20() -> ! {
    todo!("0xc0df20 __ZNSt6vectorIN3RBX11MegaCluster9ChunkDataESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::MegaCluster::ChunkData *,unsigned long,RBX::MegaCluster::ChunkData>(RBX::MegaCluster::ChunkData *,unsigned long,RBX::MegaCluster::ChunkData const&,std::__false_type)")]
// 0xc0e8e8 — __ZSt26__uninitialized_fill_n_auxIPN3RBX11MegaCluster9ChunkDataEmS2_EvT_T0_RKT1_St12__false_type
pub fn stub_c0e8e8() -> ! {
    todo!("0xc0e8e8 __ZSt26__uninitialized_fill_n_auxIPN3RBX11MegaCluster9ChunkDataEmS2_EvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "RBX::MegaCluster::ChunkData * std::__copy_backward_normal<false,false>::__copy_b_n<RBX::MegaCluster::ChunkData *,RBX::MegaCluster::ChunkData *>(RBX::MegaCluster::ChunkData *,RBX::MegaCluster::ChunkData *,RBX::MegaCluster::ChunkData *)")]
// 0xc0ea84 — __ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN3RBX11MegaCluster9ChunkDataES5_EET0_T_S7_S6_
pub fn stub_c0ea84() -> ! {
    todo!("0xc0ea84 __ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN3RBX11MegaCluster9ChunkDataES5_EET0_T_S7_S6_")
}

#[doc(alias = "void RBX::MegaClusterLegacy::createChunk<RBX::MegaClusterLegacy::VoxelGridOverlay>(RBX::SpatialRegion::Id const&,unsigned int,unsigned int)")]
// 0xc135dc — __ZN3RBX17MegaClusterLegacy11createChunkINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdEjj
pub fn stub_c135dc() -> ! {
    todo!("0xc135dc __ZN3RBX17MegaClusterLegacy11createChunkINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdEjj")
}

#[doc(alias = "void RBX::MegaClusterLegacy::updateWaterGeometry<RBX::MegaClusterLegacy::VoxelGridOverlay>(RBX::SpatialRegion::Id const&,RBX::MegaClusterLegacy::ChunkData &,unsigned int)")]
// 0xc14600 — __ZN3RBX17MegaClusterLegacy19updateWaterGeometryINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdERNS0_9ChunkDataEj
pub fn stub_c14600() -> ! {
    todo!("0xc14600 __ZN3RBX17MegaClusterLegacy19updateWaterGeometryINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdERNS0_9ChunkDataEj")
}

#[doc(alias = "void RBX::MegaClusterLegacy::updateChunkGeometry<RBX::MegaClusterLegacy::VoxelGridOverlay>(RBX::SpatialRegion::Id const&,RBX::MegaClusterLegacy::ChunkData &,int)")]
// 0xc14aa8 — __ZN3RBX17MegaClusterLegacy19updateChunkGeometryINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdERNS0_9ChunkDataEi
pub fn stub_c14aa8() -> ! {
    todo!("0xc14aa8 __ZN3RBX17MegaClusterLegacy19updateChunkGeometryINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdERNS0_9ChunkDataEi")
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xc14f78 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_20SolidTerrainRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_c14f78() -> ! {
    todo!("0xc14f78 __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_20SolidTerrainRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// 0xc15780 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_c15780() -> ! {
    todo!("0xc15780 __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE")
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// 0xc15a3c — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE9wedgeFaceERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
pub fn stub_c15a3c() -> ! {
    todo!("0xc15a3c __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE9wedgeFaceERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE")
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// 0xc15e68 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
pub fn stub_c15e68() -> ! {
    todo!("0xc15e68 __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE")
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// 0xc16054 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12wedgeUpEmptyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
pub fn stub_c16054() -> ! {
    todo!("0xc16054 __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12wedgeUpEmptyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE")
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// 0xc16138 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE14detectOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_c16138() -> ! {
    todo!("0xc16138 __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE14detectOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE")
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xc16550 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_c16550() -> ! {
    todo!("0xc16550 __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xc16d18 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_17WaterFaceRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_c16d18() -> ! {
    todo!("0xc16d18 __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_17WaterFaceRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
// 0xc17464 — __ZNK3RBX20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEE8internalERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionE
pub fn stub_c17464() -> ! {
    todo!("0xc17464 __ZNK3RBX20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEE8internalERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionE")
}

#[doc(alias = "RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// 0xc175f8 — __ZN3RBX17WaterFaceRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_c175f8() -> ! {
    todo!("0xc175f8 __ZN3RBX17WaterFaceRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE")
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xc1799c — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_c1799c() -> ! {
    todo!("0xc1799c __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE")
}

#[doc(alias = "std::vector<RBX::MegaClusterLegacy::ChunkData,std::allocator<RBX::MegaClusterLegacy::ChunkData>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::MegaClusterLegacy::ChunkData*,std::vector<RBX::MegaClusterLegacy::ChunkData,std::allocator<RBX::MegaClusterLegacy::ChunkData>>>,unsigned long,RBX::MegaClusterLegacy::ChunkData const&)")]
// 0xc180d4 — __ZNSt6vectorIN3RBX17MegaClusterLegacy9ChunkDataESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_c180d4() -> ! {
    todo!("0xc180d4 __ZNSt6vectorIN3RBX17MegaClusterLegacy9ChunkDataESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator::iterator(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk> const&)")]
// 0xc18360 — __ZN3RBX5Voxel6RegionINS0_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorC2ERKS5_
pub fn stub_c18360() -> ! {
    todo!("0xc18360 __ZN3RBX5Voxel6RegionINS0_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorC2ERKS5_")
}

#[doc(alias = "void RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk::fillFromRegion<RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>>(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)")]
// 0xc189f8 — __ZN3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk14fillFromRegionINS0_6RegionINS0_4Grid5ChunkEEEEEvRKT_
pub fn stub_c189f8() -> ! {
    todo!("0xc189f8 __ZN3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk14fillFromRegionINS0_6RegionINS0_4Grid5ChunkEEEEEvRKT_")
}

#[doc(alias = "RBX::GfxBinding::updateEntity(bool)")]
// 0xc18ea8 — __ZN3RBX10GfxBinding12updateEntityEb
pub fn stub_c18ea8() -> ! {
    todo!("0xc18ea8 __ZN3RBX10GfxBinding12updateEntityEb")
}

#[doc(alias = "RBX::ExtentsInt32::ExtentsInt32(void)")]
// 0xc18eac — __ZN3RBX12ExtentsInt32C2Ev
pub fn stub_c18eac() -> ! {
    todo!("0xc18eac __ZN3RBX12ExtentsInt32C2Ev")
}

#[doc(alias = "RBX::fillDummyLighting(RBX::LightGridChunk &,unsigned char,unsigned char)")]
// 0xc1a41c — __ZN3RBX17fillDummyLightingERNS_14LightGridChunkEhh
pub fn stub_c1a41c() -> ! {
    todo!("0xc1a41c __ZN3RBX17fillDummyLightingERNS_14LightGridChunkEhh")
}

#[doc(alias = "RBX::LightGrid::getChunkByIndex(RBX::Vector3int32 const&)")]
// 0xc1a8d4 — __ZN3RBX9LightGrid15getChunkByIndexERKNS_12Vector3int32E
pub fn stub_c1a8d4() -> ! {
    todo!("0xc1a8d4 __ZN3RBX9LightGrid15getChunkByIndexERKNS_12Vector3int32E")
}

#[doc(alias = "RBX::LightGrid::precomputeShadowLUT(void)")]
// 0xc1bc08 — __ZN3RBX9LightGrid19precomputeShadowLUTEv
pub fn stub_c1bc08() -> ! {
    todo!("0xc1bc08 __ZN3RBX9LightGrid19precomputeShadowLUTEv")
}

#[doc(alias = "RBX::LightGrid::~LightGrid()")]
// 0xc1be30 — __ZN3RBX9LightGridD0Ev
pub fn stub_c1be30() -> ! {
    todo!("0xc1be30 __ZN3RBX9LightGridD0Ev")
}

#[doc(alias = "RBX::LightGrid::~LightGrid()")]
// 0xc1bed0 — __ZN3RBX9LightGridD1Ev
pub fn stub_c1bed0() -> ! {
    todo!("0xc1bed0 __ZN3RBX9LightGridD1Ev")
}

#[doc(alias = "RBX::LightGrid::~LightGrid()")]
// 0xc1bed4 — __ZN3RBX9LightGridD2Ev
pub fn stub_c1bed4() -> ! {
    todo!("0xc1bed4 __ZN3RBX9LightGridD2Ev")
}

#[doc(alias = "RBX::LightGrid::setNonFixedPartsEnabled(bool)")]
// 0xc1c0dc — __ZN3RBX9LightGrid23setNonFixedPartsEnabledEb
pub fn stub_c1c0dc() -> ! {
    todo!("0xc1c0dc __ZN3RBX9LightGrid23setNonFixedPartsEnabledEb")
}

#[doc(alias = "RBX::LightGrid::lightingUpdateLightScratch(RBX::LightGridChunk const&,RBX::Extents const&,RBX::LightObject *)")]
// 0xc1e1f0 — __ZN3RBX9LightGrid26lightingUpdateLightScratchERKNS_14LightGridChunkERKNS_7ExtentsEPNS_11LightObjectE
pub fn stub_c1e1f0() -> ! {
    todo!("0xc1e1f0 __ZN3RBX9LightGrid26lightingUpdateLightScratchERKNS_14LightGridChunkERKNS_7ExtentsEPNS_11LightObjectE")
}

#[doc(alias = "RBX::LightGrid::lightingBlurAxisXScratchToChunkSIMD(RBX::LightGridChunk &)")]
// 0xc1e6c8 — __ZN3RBX9LightGrid35lightingBlurAxisXScratchToChunkSIMDERNS_14LightGridChunkE
pub fn stub_c1e6c8() -> ! {
    todo!("0xc1e6c8 __ZN3RBX9LightGrid35lightingBlurAxisXScratchToChunkSIMDERNS_14LightGridChunkE")
}

#[doc(alias = "RBX::LightGrid::lightingBlurAxisXScratchToChunk(RBX::LightGridChunk &)")]
// 0xc1e7f0 — __ZN3RBX9LightGrid31lightingBlurAxisXScratchToChunkERNS_14LightGridChunkE
pub fn stub_c1e7f0() -> ! {
    todo!("0xc1e7f0 __ZN3RBX9LightGrid31lightingBlurAxisXScratchToChunkERNS_14LightGridChunkE")
}

#[doc(alias = "RBX::LightGrid::lightingClearLocal(RBX::LightGridChunk &)")]
// 0xc1e908 — __ZN3RBX9LightGrid18lightingClearLocalERNS_14LightGridChunkE
pub fn stub_c1e908() -> ! {
    todo!("0xc1e908 __ZN3RBX9LightGrid18lightingClearLocalERNS_14LightGridChunkE")
}

#[doc(alias = "RBX::LightGrid::lightingUpdateChunkGlobal(RBX::LightGridChunk &)")]
// 0xc1e9d8 — __ZN3RBX9LightGrid25lightingUpdateChunkGlobalERNS_14LightGridChunkE
pub fn stub_c1e9d8() -> ! {
    todo!("0xc1e9d8 __ZN3RBX9LightGrid25lightingUpdateChunkGlobalERNS_14LightGridChunkE")
}

#[doc(alias = "RBX::LightGrid::lightingClearGlobal(RBX::LightGridChunk &)")]
// 0xc1ebd8 — __ZN3RBX9LightGrid19lightingClearGlobalERNS_14LightGridChunkE
pub fn stub_c1ebd8() -> ! {
    todo!("0xc1ebd8 __ZN3RBX9LightGrid19lightingClearGlobalERNS_14LightGridChunkE")
}

#[doc(alias = "RBX::LightGrid::lightingUpdateChunkSkylight(RBX::LightGridChunk &)")]
// 0xc1ece8 — __ZN3RBX9LightGrid27lightingUpdateChunkSkylightERNS_14LightGridChunkE
pub fn stub_c1ece8() -> ! {
    todo!("0xc1ece8 __ZN3RBX9LightGrid27lightingUpdateChunkSkylightERNS_14LightGridChunkE")
}

#[doc(alias = "RBX::LightGrid::lightingUpdateSkylight(RBX::LightGridChunk &)")]
// 0xc1ee80 — __ZN3RBX9LightGrid22lightingUpdateSkylightERNS_14LightGridChunkE
pub fn stub_c1ee80() -> ! {
    todo!("0xc1ee80 __ZN3RBX9LightGrid22lightingUpdateSkylightERNS_14LightGridChunkE")
}

#[doc(alias = "RBX::LightGrid::lightingUpdateChunkAverage(RBX::LightGridChunk &)")]
// 0xc1f788 — __ZN3RBX9LightGrid26lightingUpdateChunkAverageERNS_14LightGridChunkE
pub fn stub_c1f788() -> ! {
    todo!("0xc1f788 __ZN3RBX9LightGrid26lightingUpdateChunkAverageERNS_14LightGridChunkE")
}

#[doc(alias = "RBX::LightGrid::lightingUpdateAverageImplSIMD(RBX::LightGridChunk &)")]
// 0xc1f798 — __ZN3RBX9LightGrid29lightingUpdateAverageImplSIMDERNS_14LightGridChunkE
pub fn stub_c1f798() -> ! {
    todo!("0xc1f798 __ZN3RBX9LightGrid29lightingUpdateAverageImplSIMDERNS_14LightGridChunkE")
}

#[doc(alias = "RBX::LightGrid::lightingUpdateAverageImpl(RBX::LightGridChunk &)")]
// 0xc1fa58 — __ZN3RBX9LightGrid25lightingUpdateAverageImplERNS_14LightGridChunkE
pub fn stub_c1fa58() -> ! {
    todo!("0xc1fa58 __ZN3RBX9LightGrid25lightingUpdateAverageImplERNS_14LightGridChunkE")
}

#[doc(alias = "RBX::LightGrid::invalidateAll(unsigned int)")]
// 0xc1fb88 — __ZN3RBX9LightGrid13invalidateAllEj
pub fn stub_c1fb88() -> ! {
    todo!("0xc1fb88 __ZN3RBX9LightGrid13invalidateAllEj")
}

#[doc(alias = "RBX::LightGrid::invalidateExtents(RBX::Extents const&,unsigned int)")]
// 0xc1fbb8 — __ZN3RBX9LightGrid17invalidateExtentsERKNS_7ExtentsEj
pub fn stub_c1fbb8() -> ! {
    todo!("0xc1fbb8 __ZN3RBX9LightGrid17invalidateExtentsERKNS_7ExtentsEj")
}

#[doc(alias = "RBX::LightGrid::relocateGrid(RBX::Vector3int32 const&,bool)")]
// 0xc1fff8 — __ZN3RBX9LightGrid12relocateGridERKNS_12Vector3int32Eb
pub fn stub_c1fff8() -> ! {
    todo!("0xc1fff8 __ZN3RBX9LightGrid12relocateGridERKNS_12Vector3int32Eb")
}

#[doc(alias = "RBX::LightGrid::setLightShadows(bool)")]
// 0xc20e08 — __ZN3RBX9LightGrid15setLightShadowsEb
pub fn stub_c20e08() -> ! {
    todo!("0xc20e08 __ZN3RBX9LightGrid15setLightShadowsEb")
}

#[doc(alias = "RBX::LightGrid::stepCursor(RBX::Vector3int32 &)")]
// 0xc21048 — __ZN3RBX9LightGrid10stepCursorERNS_12Vector3int32E
pub fn stub_c21048() -> ! {
    todo!("0xc21048 __ZN3RBX9LightGrid10stepCursorERNS_12Vector3int32E")
}

#[doc(alias = "RBX::LightGrid::findDirtyChunk(void)")]
// 0xc2119c — __ZN3RBX9LightGrid14findDirtyChunkEv
pub fn stub_c2119c() -> ! {
    todo!("0xc2119c __ZN3RBX9LightGrid14findDirtyChunkEv")
}

#[doc(alias = "RBX::LightGrid::findFirstDirtyChunk(void)")]
// 0xc212a0 — __ZN3RBX9LightGrid19findFirstDirtyChunkEv
pub fn stub_c212a0() -> ! {
    todo!("0xc212a0 __ZN3RBX9LightGrid19findFirstDirtyChunkEv")
}

#[doc(alias = "RBX::LightGrid::findOldestChunk(void)")]
// 0xc21358 — __ZN3RBX9LightGrid15findOldestChunkEv
pub fn stub_c21358() -> ! {
    todo!("0xc21358 __ZN3RBX9LightGrid15findOldestChunkEv")
}

#[doc(alias = "RBX::LightGrid::getGridCornerOffset(void)const")]
// 0xc21434 — __ZNK3RBX9LightGrid19getGridCornerOffsetEv
pub fn stub_c21434() -> ! {
    todo!("0xc21434 __ZNK3RBX9LightGrid19getGridCornerOffsetEv")
}

#[doc(alias = "RBX::LightGrid::getWrapSafeOffset(void)const")]
// 0xc21484 — __ZNK3RBX9LightGrid17getWrapSafeOffsetEv
pub fn stub_c21484() -> ! {
    todo!("0xc21484 __ZNK3RBX9LightGrid17getWrapSafeOffsetEv")
}

#[doc(alias = "RBX::LightGrid::getGridSize(void)const")]
// 0xc2158c — __ZNK3RBX9LightGrid11getGridSizeEv
pub fn stub_c2158c() -> ! {
    todo!("0xc2158c __ZNK3RBX9LightGrid11getGridSizeEv")
}

#[doc(alias = "RBX::LightGrid::getBorderColor(void)const")]
// 0xc215dc — __ZNK3RBX9LightGrid14getBorderColorEv
pub fn stub_c215dc() -> ! {
    todo!("0xc215dc __ZNK3RBX9LightGrid14getBorderColorEv")
}

#[doc(alias = "RBX::LightGrid::lightingUpdateSkylightRow(RBX::LightGridChunk &,int,int,unsigned char const*)")]
// 0xc2165c — __ZN3RBX9LightGrid25lightingUpdateSkylightRowERNS_14LightGridChunkEiiPKh
pub fn stub_c2165c() -> ! {
    todo!("0xc2165c __ZN3RBX9LightGrid25lightingUpdateSkylightRowERNS_14LightGridChunkEiiPKh")
}

#[doc(alias = "RBX::LightGrid::lightingFixupShadowMaskBorder(RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&)")]
// 0xc21af4 — __ZN3RBX9LightGrid29lightingFixupShadowMaskBorderERKNS_12Vector3int32ES3_S3_
pub fn stub_c21af4() -> ! {
    todo!("0xc21af4 __ZN3RBX9LightGrid29lightingFixupShadowMaskBorderERKNS_12Vector3int32ES3_S3_")
}

#[doc(alias = "RBX::LightGrid::lightingComposit(RBX::LightGridChunk const&,unsigned char *,unsigned int,unsigned int)")]
// 0xc21ea8 — __ZN3RBX9LightGrid16lightingCompositERKNS_14LightGridChunkEPhjj
pub fn stub_c21ea8() -> ! {
    todo!("0xc21ea8 __ZN3RBX9LightGrid16lightingCompositERKNS_14LightGridChunkEPhjj")
}

#[doc(alias = "RBX::LightGrid::lightingUploadChunk(RBX::LightGridChunk &)")]
// 0xc22400 — __ZN3RBX9LightGrid19lightingUploadChunkERNS_14LightGridChunkE
pub fn stub_c22400() -> ! {
    todo!("0xc22400 __ZN3RBX9LightGrid19lightingUploadChunkERNS_14LightGridChunkE")
}

#[doc(alias = "RBX::LightGrid::lightingUploadAll(void)")]
// 0xc22c80 — __ZN3RBX9LightGrid17lightingUploadAllEv
pub fn stub_c22c80() -> ! {
    todo!("0xc22c80 __ZN3RBX9LightGrid17lightingUploadAllEv")
}

#[doc(alias = "boost::system::system_error::what(void)const")]
// 0xc23250 — __ZNK5boost6system12system_error4whatEv
pub fn stub_c23250() -> ! {
    todo!("0xc23250 __ZNK5boost6system12system_error4whatEv")
}

#[doc(alias = "std::invalid_argument::~invalid_argument()")]
// 0xc233f0 — __ZNSt16invalid_argumentD0Ev
pub fn stub_c233f0() -> ! {
    todo!("0xc233f0 __ZNSt16invalid_argumentD0Ev")
}

#[doc(alias = "std::domain_error::~domain_error()")]
// 0xc23410 — __ZNSt12domain_errorD1Ev
pub fn stub_c23410() -> ! {
    todo!("0xc23410 __ZNSt12domain_errorD1Ev")
}

#[doc(alias = "std::length_error::~length_error()")]
// 0xc23420 — __ZNSt12length_errorD2Ev
pub fn stub_c23420() -> ! {
    todo!("0xc23420 __ZNSt12length_errorD2Ev")
}

#[doc(alias = "std::overflow_error::~overflow_error()")]
// 0xc23430 — __ZNSt14overflow_errorD0Ev
pub fn stub_c23430() -> ! {
    todo!("0xc23430 __ZNSt14overflow_errorD0Ev")
}

#[doc(alias = "std::underflow_error::~underflow_error()")]
// 0xc23450 — __ZNSt15underflow_errorD1Ev
pub fn stub_c23450() -> ! {
    todo!("0xc23450 __ZNSt15underflow_errorD1Ev")
}

#[doc(alias = "void RBX::LightGrid::occupancyFillTerrainSIMD<RBX::Voxel::Grid>(RBX::LightGridChunk &,RBX::Voxel::Grid &,RBX::Vector3int32 const&,RBX::Extents const&)")]
// 0xc234b0 — __ZN3RBX9LightGrid24occupancyFillTerrainSIMDINS_5Voxel4GridEEEvRNS_14LightGridChunkERT_RKNS_12Vector3int32ERKNS_7ExtentsE
pub fn stub_c234b0() -> ! {
    todo!("0xc234b0 __ZN3RBX9LightGrid24occupancyFillTerrainSIMDINS_5Voxel4GridEEEvRNS_14LightGridChunkERT_RKNS_12Vector3int32ERKNS_7ExtentsE")
}

#[doc(alias = "void RBX::LightGrid::occupancyFillTerrain<RBX::Voxel::Grid>(RBX::LightGridChunk &,RBX::Voxel::Grid &,RBX::Vector3int32 const&,RBX::Extents const&)")]
// 0xc239b8 — __ZN3RBX9LightGrid20occupancyFillTerrainINS_5Voxel4GridEEEvRNS_14LightGridChunkERT_RKNS_12Vector3int32ERKNS_7ExtentsE
pub fn stub_c239b8() -> ! {
    todo!("0xc239b8 __ZN3RBX9LightGrid20occupancyFillTerrainINS_5Voxel4GridEEEvRNS_14LightGridChunkERT_RKNS_12Vector3int32ERKNS_7ExtentsE")
}

#[doc(alias = "void RBX::LightGrid::lightingBlurAxisYZScratch<true>(void)")]
// 0xc27130 — __ZN3RBX9LightGrid25lightingBlurAxisYZScratchILb1EEEvv
pub fn stub_c27130() -> ! {
    todo!("0xc27130 __ZN3RBX9LightGrid25lightingBlurAxisYZScratchILb1EEEvv")
}

#[doc(alias = "void RBX::LightGrid::lightingBlurAxisYZScratch<false>(void)")]
// 0xc27240 — __ZN3RBX9LightGrid25lightingBlurAxisYZScratchILb0EEEvv
pub fn stub_c27240() -> ! {
    todo!("0xc27240 __ZN3RBX9LightGrid25lightingBlurAxisYZScratchILb0EEEvv")
}

#[doc(alias = "void RBX::LightGrid::lightingUpdateDirectionalImpl<true,true>(RBX::LightGridChunk &,RBX::Vector3int32 const&)")]
// 0xc27330 — __ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb1ELb1EEEvRNS_14LightGridChunkERKNS_12Vector3int32E
pub fn stub_c27330() -> ! {
    todo!("0xc27330 __ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb1ELb1EEEvRNS_14LightGridChunkERKNS_12Vector3int32E")
}

#[doc(alias = "void RBX::LightGrid::lightingUpdateDirectionalImpl<true,false>(RBX::LightGridChunk &,RBX::Vector3int32 const&)")]
// 0xc27c18 — __ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb1ELb0EEEvRNS_14LightGridChunkERKNS_12Vector3int32E
pub fn stub_c27c18() -> ! {
    todo!("0xc27c18 __ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb1ELb0EEEvRNS_14LightGridChunkERKNS_12Vector3int32E")
}
