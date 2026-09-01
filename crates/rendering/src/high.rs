//! rendering high — shard 2 (high EA >= 0xC00000)
//! Filter: Ogre|Gfx|Render|G3D (11144 total, 4927 prior, 100 this batch) — 0xc03acc..0xc8af30
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(
    non_snake_case,
    dead_code,
    unused_variables,
    unused_imports,
    clippy::all
)]

use rbx_core::SharedPtr;

// 0xc03acc — __ZN19ResourceGroupHelper23updateOnEveryRenderableEv
#[doc(alias = "ResourceGroupHelper::updateOnEveryRenderable(void)")]
// was: ResourceGroupHelper::updateOnEveryRenderable(void)
pub fn stub_c03acc() -> ! {
    todo!("0xc03acc ResourceGroupHelper::updateOnEveryRenderable(void)")
}

// 0xc04c88 — __ZN19ResourceGroupHelper31UpdateMaterialRenderableVisitorD1Ev
#[doc(
    alias = "ResourceGroupHelper::UpdateMaterialRenderableVisitor::~UpdateMaterialRenderableVisitor()"
)]
// was: ResourceGroupHelper::UpdateMaterialRenderableVisitor::~UpdateMaterialRenderableVisitor()
pub fn stub_c04c88() -> ! {
    todo!("0xc04c88 ResourceGroupHelper::UpdateMaterialRenderableVisitor::~UpdateMaterialRenderableVisitor()")
}

// 0xc04c8c — __ZN19ResourceGroupHelper31UpdateMaterialRenderableVisitorD0Ev
#[doc(
    alias = "ResourceGroupHelper::UpdateMaterialRenderableVisitor::~UpdateMaterialRenderableVisitor()"
)]
// was: ResourceGroupHelper::UpdateMaterialRenderableVisitor::~UpdateMaterialRenderableVisitor()
pub fn stub_c04c8c() -> ! {
    todo!("0xc04c8c ResourceGroupHelper::UpdateMaterialRenderableVisitor::~UpdateMaterialRenderableVisitor()")
}

// 0xc08ddc — __ZN3RBX11MegaCluster19createSolidGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj
#[doc(
    alias = "RBX::MegaCluster::createSolidGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)"
)]
// was: RBX::MegaCluster::createSolidGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)
pub fn stub_c08ddc() -> ! {
    todo!("0xc08ddc RBX::MegaCluster::createSolidGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)")
}

// 0xc08fec — __ZN3RBX11MegaCluster19createWaterGeometryEPNS_10RenderNodeERKNS_13SpatialRegion2IdEPj
#[doc(
    alias = "RBX::MegaCluster::createWaterGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)"
)]
// was: RBX::MegaCluster::createWaterGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)
pub fn stub_c08fec() -> ! {
    todo!("0xc08fec RBX::MegaCluster::createWaterGeometry(RBX::RenderNode *,RBX::SpatialRegion::Id const&,unsigned int *)")
}

// 0xc0a4ec — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_20SolidTerrainRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)
pub fn stub_c0a4ec() -> ! {
    todo!("0xc0a4ec RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc0acec — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_17WaterFaceRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)
pub fn stub_c0acec() -> ! {
    todo!("0xc0acec RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc0b430 — __ZN3RBX10GfxBinding16invalidateEntityEv
#[doc(alias = "RBX::GfxBinding::invalidateEntity(void)")]
// was: RBX::GfxBinding::invalidateEntity(void)
pub fn stub_c0b430() -> ! {
    todo!("0xc0b430 RBX::GfxBinding::invalidateEntity(void)")
}

// 0xc0b434 — __ZN3RBX10GfxBinding24onCoordinateFrameChangedEv
#[doc(alias = "RBX::GfxBinding::onCoordinateFrameChanged(void)")]
// was: RBX::GfxBinding::onCoordinateFrameChanged(void)
pub fn stub_c0b434() -> ! {
    todo!("0xc0b434 RBX::GfxBinding::onCoordinateFrameChanged(void)")
}

// 0xc0b438 — __ZN3RBX7GfxPart21updateCoordinateFrameEb
#[doc(alias = "RBX::GfxPart::updateCoordinateFrame(bool)")]
// was: RBX::GfxPart::updateCoordinateFrame(bool)
pub fn stub_c0b438() -> ! {
    todo!("0xc0b438 RBX::GfxPart::updateCoordinateFrame(bool)")
}

// 0xc0b43c — __ZN3RBX7GfxPart19getFastFuzzyExtentsEv
#[doc(alias = "RBX::GfxPart::getFastFuzzyExtents(void)")]
// was: RBX::GfxPart::getFastFuzzyExtents(void)
pub fn stub_c0b43c() -> ! {
    todo!("0xc0b43c RBX::GfxPart::getFastFuzzyExtents(void)")
}

// 0xc0b4cc — __ZN3RBX7GfxPart12getPartCountEv
#[doc(alias = "RBX::GfxPart::getPartCount(void)")]
// was: RBX::GfxPart::getPartCount(void)
pub fn stub_c0b4cc() -> ! {
    todo!("0xc0b4cc RBX::GfxPart::getPartCount(void)")
}

// 0xc0b4d0 — __ZN3RBX7GfxPart17onSleepingChangedEbPNS_12PartInstanceE
#[doc(alias = "RBX::GfxPart::onSleepingChanged(bool,RBX::PartInstance *)")]
// was: RBX::GfxPart::onSleepingChanged(bool,RBX::PartInstance *)
pub fn stub_c0b4d0() -> ! {
    todo!("0xc0b4d0 RBX::GfxPart::onSleepingChanged(bool,RBX::PartInstance *)")
}

// 0xc0b4d4 — __ZN3RBX7GfxPart14onClumpChangedEv
#[doc(alias = "RBX::GfxPart::onClumpChanged(void)")]
// was: RBX::GfxPart::onClumpChanged(void)
pub fn stub_c0b4d4() -> ! {
    todo!("0xc0b4d4 RBX::GfxPart::onClumpChanged(void)")
}

// 0xc0b4d8 — __ZNK3RBX20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE8internalERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionE
#[doc(
    alias = "RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const"
)]
// was: RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const
pub fn stub_c0b4d8() -> ! {
    todo!("0xc0b4d8 RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")
}

// 0xc0b66c — __ZN3RBX17WaterFaceRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
pub fn stub_c0b66c() -> ! {
    todo!("0xc0b66c RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc0bf18 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)
pub fn stub_c0bf18() -> ! {
    todo!("0xc0bf18 RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc0c648 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
pub fn stub_c0c648() -> ! {
    todo!("0xc0c648 RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc0c904 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE9wedgeFaceERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
pub fn stub_c0c904() -> ! {
    todo!("0xc0c904 RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")
}

// 0xc0cd30 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE19detectWedgeOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
pub fn stub_c0cd30() -> ! {
    todo!("0xc0cd30 RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")
}

// 0xc0cf1c — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12wedgeUpEmptyERKNS1_6RegionINS3_5ChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
pub fn stub_c0cf1c() -> ! {
    todo!("0xc0cf1c RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")
}

// 0xc0d000 — __ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE14detectOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
pub fn stub_c0d000() -> ! {
    todo!("0xc0d000 RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc0d418 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)
pub fn stub_c0d418() -> ! {
    todo!("0xc0d418 RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc0dbd8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()
pub fn stub_c0dbd8() -> ! {
    todo!("0xc0dbd8 boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")
}

// 0xc0dbdc — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()
pub fn stub_c0dbdc() -> ! {
    todo!("0xc0dbdc boost::detail::sp_counted_impl_p<RBX::RenderNode>::~sp_counted_impl_p()")
}

// 0xc0dbe0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::dispose(void)")]
// was: boost::detail::sp_counted_impl_p<RBX::RenderNode>::dispose(void)
pub fn stub_c0dbe0() -> ! {
    todo!("0xc0dbe0 boost::detail::sp_counted_impl_p<RBX::RenderNode>::dispose(void)")
}

// 0xc0dbf0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE11get_deleterERKSt9type_info
#[doc(
    alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_deleter(std::type_info const&)"
)]
// was: boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_deleter(std::type_info const&)
pub fn stub_c0dbf0() -> ! {
    todo!("0xc0dbf0 boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_deleter(std::type_info const&)")
}

// 0xc0dbf4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10RenderNodeEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_untyped_deleter(void)
pub fn stub_c0dbf4() -> ! {
    todo!("0xc0dbf4 boost::detail::sp_counted_impl_p<RBX::RenderNode>::get_untyped_deleter(void)")
}

// 0xc10528 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_19MegaClusterInstanceEEENS_20SolidTerrainRendererIS2_EES2_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)
pub fn stub_c10528() -> ! {
    todo!("0xc10528 RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc10cec — __ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE5applyERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
pub fn stub_c10cec() -> ! {
    todo!("0xc10cec RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc10fb8 — __ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE9wedgeFaceERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeFace(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeFace(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)
pub fn stub_c10fb8() -> ! {
    todo!("0xc10fb8 RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeFace(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)")
}

// 0xc113f0 — __ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectWedgeOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectWedgeOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)
pub fn stub_c113f0() -> ! {
    todo!("0xc113f0 RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectWedgeOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)")
}

// 0xc11604 — __ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE12wedgeUpEmptyERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeUpEmpty(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeUpEmpty(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)
pub fn stub_c11604() -> ! {
    todo!("0xc11604 RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::wedgeUpEmpty(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&)")
}

// 0xc116f4 — __ZN3RBX20SolidTerrainRendererINS_19MegaClusterInstanceEE14detectOutlinesERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
pub fn stub_c116f4() -> ! {
    todo!("0xc116f4 RBX::SolidTerrainRenderer<RBX::MegaClusterInstance>::detectOutlines(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc11b14 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_19MegaClusterInstanceEEENS_11FaceCounterIS2_EES2_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)
pub fn stub_c11b14() -> ! {
    todo!("0xc11b14 RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc12294 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_19MegaClusterInstanceEEENS_17WaterFaceRendererIS2_EES2_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::WaterFaceRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::WaterFaceRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)
pub fn stub_c12294() -> ! {
    todo!("0xc12294 RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::WaterFaceRenderer<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc129a8 — __ZNK3RBX20WaterRenderPredicateINS_19MegaClusterInstanceEE8internalERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionE
#[doc(
    alias = "RBX::WaterRenderPredicate<RBX::MegaClusterInstance>::internal(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection)const"
)]
// was: RBX::WaterRenderPredicate<RBX::MegaClusterInstance>::internal(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection)const
pub fn stub_c129a8() -> ! {
    todo!("0xc129a8 RBX::WaterRenderPredicate<RBX::MegaClusterInstance>::internal(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection)const")
}

// 0xc12b3c — __ZN3RBX17WaterFaceRendererINS_19MegaClusterInstanceEE5applyERKNS_5Voxel6RegionINS1_9CellChunkEE8iteratorENS3_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::WaterFaceRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::WaterFaceRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
pub fn stub_c12b3c() -> ! {
    todo!("0xc12b3c RBX::WaterFaceRenderer<RBX::MegaClusterInstance>::apply(RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc12ee0 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_19MegaClusterInstanceEEENS_11FaceCounterIS2_EES2_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)
pub fn stub_c12ee0() -> ! {
    todo!("0xc12ee0 RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterInstance>,RBX::FaceCounter<RBX::MegaClusterInstance>,RBX::MegaClusterInstance>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc14f78 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_20SolidTerrainRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)
pub fn stub_c14f78() -> ! {
    todo!("0xc14f78 RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc15780 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
pub fn stub_c15780() -> ! {
    todo!("0xc15780 RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc15a3c — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE9wedgeFaceERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
pub fn stub_c15a3c() -> ! {
    todo!("0xc15a3c RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")
}

// 0xc15e68 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
pub fn stub_c15e68() -> ! {
    todo!("0xc15e68 RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")
}

// 0xc16054 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12wedgeUpEmptyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)
pub fn stub_c16054() -> ! {
    todo!("0xc16054 RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")
}

// 0xc16138 — __ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE14detectOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
pub fn stub_c16138() -> ! {
    todo!("0xc16138 RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc16550 — __ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)
pub fn stub_c16550() -> ! {
    todo!("0xc16550 RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc16d18 — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_17WaterFaceRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)
pub fn stub_c16d18() -> ! {
    todo!("0xc16d18 RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc17464 — __ZNK3RBX20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEE8internalERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionE
#[doc(
    alias = "RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const"
)]
// was: RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const
pub fn stub_c17464() -> ! {
    todo!("0xc17464 RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")
}

// 0xc175f8 — __ZN3RBX17WaterFaceRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
#[doc(
    alias = "RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)"
)]
// was: RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)
pub fn stub_c175f8() -> ! {
    todo!("0xc175f8 RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")
}

// 0xc1799c — __ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
#[doc(
    alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)"
)]
// was: RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)
pub fn stub_c1799c() -> ! {
    todo!("0xc1799c RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")
}

// 0xc18ea8 — __ZN3RBX10GfxBinding12updateEntityEb
#[doc(alias = "RBX::GfxBinding::updateEntity(bool)")]
// was: RBX::GfxBinding::updateEntity(bool)
pub fn stub_c18ea8() -> ! {
    todo!("0xc18ea8 RBX::GfxBinding::updateEntity(bool)")
}

// 0xc35418 — __ZN3RBX12RenderEntityD0Ev
#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
// was: RBX::RenderEntity::~RenderEntity()
pub fn stub_c35418() -> ! {
    todo!("0xc35418 RBX::RenderEntity::~RenderEntity()")
}

// 0xc354b8 — __ZN3RBX12RenderEntityD1Ev
#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
// was: RBX::RenderEntity::~RenderEntity()
pub fn stub_c354b8() -> ! {
    todo!("0xc354b8 RBX::RenderEntity::~RenderEntity()")
}

// 0xc354bc — __ZN3RBX12RenderEntityD2Ev
#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
// was: RBX::RenderEntity::~RenderEntity()
pub fn stub_c354bc() -> ! {
    todo!("0xc354bc RBX::RenderEntity::~RenderEntity()")
}

// 0xc35838 — __ZNK3RBX12RenderEntity17getActualMaterialEv
#[doc(alias = "RBX::RenderEntity::getActualMaterial(void)const")]
// was: RBX::RenderEntity::getActualMaterial(void)const
pub fn stub_c35838() -> ! {
    todo!("0xc35838 RBX::RenderEntity::getActualMaterial(void)const")
}

// 0xc35980 — __ZNK3RBX12RenderEntity11getMaterialEv
#[doc(alias = "RBX::RenderEntity::getMaterial(void)const")]
// was: RBX::RenderEntity::getMaterial(void)const
pub fn stub_c35980() -> ! {
    todo!("0xc35980 RBX::RenderEntity::getMaterial(void)const")
}

// 0xc359e8 — __ZNK3RBX12RenderEntity12getTechniqueEv
#[doc(alias = "RBX::RenderEntity::getTechnique(void)const")]
// was: RBX::RenderEntity::getTechnique(void)const
pub fn stub_c359e8() -> ! {
    todo!("0xc359e8 RBX::RenderEntity::getTechnique(void)const")
}

// 0xc359ec — __ZNK3RBX12RenderEntity16getDebugMaterialEv
#[doc(alias = "RBX::RenderEntity::getDebugMaterial(void)const")]
// was: RBX::RenderEntity::getDebugMaterial(void)const
pub fn stub_c359ec() -> ! {
    todo!("0xc359ec RBX::RenderEntity::getDebugMaterial(void)const")
}

// 0xc35aa8 — __ZNK3RBX12RenderEntity9getLightsEv
#[doc(alias = "RBX::RenderEntity::getLights(void)const")]
// was: RBX::RenderEntity::getLights(void)const
pub fn stub_c35aa8() -> ! {
    todo!("0xc35aa8 RBX::RenderEntity::getLights(void)const")
}

// 0xc35b20 — __ZN3RBX10RenderNodeD0Ev
#[doc(alias = "RBX::RenderNode::~RenderNode()")]
// was: RBX::RenderNode::~RenderNode()
pub fn stub_c35b20() -> ! {
    todo!("0xc35b20 RBX::RenderNode::~RenderNode()")
}

// 0xc35bd4 — __ZN3RBX10RenderNodeD1Ev
#[doc(alias = "RBX::RenderNode::~RenderNode()")]
// was: RBX::RenderNode::~RenderNode()
pub fn stub_c35bd4() -> ! {
    todo!("0xc35bd4 RBX::RenderNode::~RenderNode()")
}

// 0xc35bd8 — __ZN3RBX10RenderNodeD2Ev
#[doc(alias = "RBX::RenderNode::~RenderNode()")]
// was: RBX::RenderNode::~RenderNode()
pub fn stub_c35bd8() -> ! {
    todo!("0xc35bd8 RBX::RenderNode::~RenderNode()")
}

// 0xc35d9c — __ZN3RBX10RenderNode9addEntityEPNS_12RenderEntityE
#[doc(alias = "RBX::RenderNode::addEntity(RBX::RenderEntity *)")]
// was: RBX::RenderNode::addEntity(RBX::RenderEntity *)
pub fn stub_c35d9c() -> ! {
    todo!("0xc35d9c RBX::RenderNode::addEntity(RBX::RenderEntity *)")
}

// 0xc35e2c — __ZN3RBX10RenderNode12removeEntityEPNS_12RenderEntityE
#[doc(alias = "RBX::RenderNode::removeEntity(RBX::RenderEntity *)")]
// was: RBX::RenderNode::removeEntity(RBX::RenderEntity *)
pub fn stub_c35e2c() -> ! {
    todo!("0xc35e2c RBX::RenderNode::removeEntity(RBX::RenderEntity *)")
}

// 0xc35f64 — __ZN3RBX10RenderNode19getFastFuzzyExtentsEv
#[doc(alias = "RBX::RenderNode::getFastFuzzyExtents(void)")]
// was: RBX::RenderNode::getFastFuzzyExtents(void)
pub fn stub_c35f64() -> ! {
    todo!("0xc35f64 RBX::RenderNode::getFastFuzzyExtents(void)")
}

// 0xc35f74 — __ZThn392_N3RBX10RenderNode19getFastFuzzyExtentsEv
#[doc(alias = "non-virtual thunk to RBX::RenderNode::getFastFuzzyExtents(void)")]
// was: non-virtual thunk to RBX::RenderNode::getFastFuzzyExtents(void)
pub fn stub_c35f74() -> ! {
    todo!("0xc35f74 non-virtual thunk to RBX::RenderNode::getFastFuzzyExtents(void)")
}

// 0xc35f80 — __ZN3RBX10RenderNode13_updateBoundsEv
#[doc(alias = "RBX::RenderNode::_updateBounds(void)")]
// was: RBX::RenderNode::_updateBounds(void)
pub fn stub_c35f80() -> ! {
    todo!("0xc35f80 RBX::RenderNode::_updateBounds(void)")
}

// 0xc3602c — __ZNSt6vectorIPN3RBX12RenderEntityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(
    alias = "std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::RenderEntity **,std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>>,RBX::RenderEntity * const&)"
)]
// was: std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::RenderEntity **,std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>>,RBX::RenderEntity * const&)
pub fn stub_c3602c() -> ! {
    todo!("0xc3602c std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::RenderEntity **,std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>>,RBX::RenderEntity * const&)")
}

// 0xc50e64 — __ZN4Ogre9Animation27setDefaultInterpolationModeENS0_17InterpolationModeE
#[doc(alias = "Ogre::Animation::setDefaultInterpolationMode(Ogre::Animation::InterpolationMode)")]
// was: Ogre::Animation::setDefaultInterpolationMode(Ogre::Animation::InterpolationMode)
pub fn stub_c50e64() -> ! {
    todo!(
        "0xc50e64 Ogre::Animation::setDefaultInterpolationMode(Ogre::Animation::InterpolationMode)"
    )
}

// 0xc5738c — __ZN4Ogre19AutoParamDataSource16setWorldMatricesEPKNS_7Matrix4Em
#[doc(alias = "Ogre::AutoParamDataSource::setWorldMatrices(Ogre::Matrix4 const*,unsigned long)")]
// was: Ogre::AutoParamDataSource::setWorldMatrices(Ogre::Matrix4 const*,unsigned long)
pub fn stub_c5738c() -> ! {
    todo!(
        "0xc5738c Ogre::AutoParamDataSource::setWorldMatrices(Ogre::Matrix4 const*,unsigned long)"
    )
}

// 0xc6a064 — __ZN4Ogre25BorderPanelOverlayElement13CmdBorderSize5doSetEPvRKSs
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderSize::doSet(void *,std::string const&)")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderSize::doSet(void *,std::string const&)
pub fn stub_c6a064() -> ! {
    todo!(
        "0xc6a064 Ogre::BorderPanelOverlayElement::CmdBorderSize::doSet(void *,std::string const&)"
    )
}

// 0xc6a554 — __ZNK4Ogre25BorderPanelOverlayElement21CmdBorderBottomLeftUV5doGetEPKv
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::doGet(void const*)const")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::doGet(void const*)const
pub fn stub_c6a554() -> ! {
    todo!(
        "0xc6a554 Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::doGet(void const*)const"
    )
}

// 0xc6a7e8 — __ZNK4Ogre25BorderPanelOverlayElement22CmdBorderBottomRightUV5doGetEPKv
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderBottomRightUV::doGet(void const*)const")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderBottomRightUV::doGet(void const*)const
pub fn stub_c6a7e8() -> ! {
    todo!(
        "0xc6a7e8 Ogre::BorderPanelOverlayElement::CmdBorderBottomRightUV::doGet(void const*)const"
    )
}

// 0xc6ba1c — __ZN4Ogre25BorderPanelOverlayElement21CmdBorderBottomLeftUVD1Ev
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()
pub fn stub_c6ba1c() -> ! {
    todo!(
        "0xc6ba1c Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()"
    )
}

// 0xc6ba7c — __ZN4Ogre25BorderPanelOverlayElement21CmdBorderBottomLeftUVD0Ev
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()
pub fn stub_c6ba7c() -> ! {
    todo!(
        "0xc6ba7c Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()"
    )
}

// 0xc6eb18 — __ZNK4Ogre10Renderable19setRenderSystemDataEPNS0_16RenderSystemDataE
#[doc(alias = "Ogre::Renderable::setRenderSystemData(Ogre::Renderable::RenderSystemData *)const")]
// was: Ogre::Renderable::setRenderSystemData(Ogre::Renderable::RenderSystemData *)const
pub fn stub_c6eb18() -> ! {
    todo!(
        "0xc6eb18 Ogre::Renderable::setRenderSystemData(Ogre::Renderable::RenderSystemData *)const"
    )
}

// 0xc708cc — __ZN4Ogre15CompositionPass8setInputEmRKSsm
#[doc(alias = "Ogre::CompositionPass::setInput(unsigned long,std::string const&,unsigned long)")]
// was: Ogre::CompositionPass::setInput(unsigned long,std::string const&,unsigned long)
pub fn stub_c708cc() -> ! {
    todo!(
        "0xc708cc Ogre::CompositionPass::setInput(unsigned long,std::string const&,unsigned long)"
    )
}

// 0xc70ad8 — __ZN4Ogre21CompositionTargetPassC1EPNS_20CompositionTechniqueE
#[doc(alias = "Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")]
// was: Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)
pub fn stub_c70ad8() -> ! {
    todo!(
        "0xc70ad8 Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)"
    )
}

// 0xc70ae4 — __ZN4Ogre21CompositionTargetPassC2EPNS_20CompositionTechniqueE
#[doc(alias = "Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")]
// was: Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)
pub fn stub_c70ae4() -> ! {
    todo!(
        "0xc70ae4 Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)"
    )
}

// 0xc7967c — __ZN4Ogre17ControllerManager23createTextureUVScrollerEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureUVScroller(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureUVScroller(Ogre::TextureUnitState *,float)
pub fn stub_c7967c() -> ! {
    todo!(
        "0xc7967c Ogre::ControllerManager::createTextureUVScroller(Ogre::TextureUnitState *,float)"
    )
}

// 0xc7998c — __ZN4Ogre17ControllerManager22createTextureUScrollerEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureUScroller(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureUScroller(Ogre::TextureUnitState *,float)
pub fn stub_c7998c() -> ! {
    todo!(
        "0xc7998c Ogre::ControllerManager::createTextureUScroller(Ogre::TextureUnitState *,float)"
    )
}

// 0xc79c9c — __ZN4Ogre17ControllerManager22createTextureVScrollerEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureVScroller(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureVScroller(Ogre::TextureUnitState *,float)
pub fn stub_c79c9c() -> ! {
    todo!(
        "0xc79c9c Ogre::ControllerManager::createTextureVScroller(Ogre::TextureUnitState *,float)"
    )
}

// 0xc7f2cc — __ZN4Ogre27DefaultHardwareVertexBuffer8readDataEmmPv
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::readData(unsigned long,unsigned long,void *)")]
// was: Ogre::DefaultHardwareVertexBuffer::readData(unsigned long,unsigned long,void *)
pub fn stub_c7f2cc() -> ! {
    todo!(
        "0xc7f2cc Ogre::DefaultHardwareVertexBuffer::readData(unsigned long,unsigned long,void *)"
    )
}

// 0xc85874 — __ZThn188_N4Ogre6Entity25backgroundLoadingCompleteEPNS_8ResourceE
#[doc(alias = "non-virtual thunk toOgre::Entity::backgroundLoadingComplete(Ogre::Resource *)")]
// was: non-virtual thunk to Ogre::Entity::backgroundLoadingComplete(Ogre::Resource *)
pub fn stub_c85874() -> ! {
    todo!(
        "0xc85874 non-virtual thunk to Ogre::Entity::backgroundLoadingComplete(Ogre::Resource *)"
    )
}

// 0xc8a3e0 — __ZN4Ogre6Entity22EntityShadowRenderableD2Ev
#[doc(alias = "Ogre::Entity::EntityShadowRenderable::~EntityShadowRenderable()")]
// was: Ogre::Entity::EntityShadowRenderable::~EntityShadowRenderable()
pub fn stub_c8a3e0() -> ! {
    todo!("0xc8a3e0 Ogre::Entity::EntityShadowRenderable::~EntityShadowRenderable()")
}

// 0xc8a580 — __ZNK4Ogre6Entity22EntityShadowRenderable18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::Entity::EntityShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::Entity::EntityShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const
pub fn stub_c8a580() -> ! {
    todo!("0xc8a580 Ogre::Entity::EntityShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")
}

// 0xc8a5c8 — __ZNK4Ogre6Entity22EntityShadowRenderable9isVisibleEv
#[doc(alias = "Ogre::Entity::EntityShadowRenderable::isVisible(void)const")]
// was: Ogre::Entity::EntityShadowRenderable::isVisible(void)const
pub fn stub_c8a5c8() -> ! {
    todo!("0xc8a5c8 Ogre::Entity::EntityShadowRenderable::isVisible(void)const")
}

// 0xc8a5e0 — __ZN4Ogre6Entity22EntityShadowRenderable17rebindIndexBufferERKNS_28HardwareIndexBufferSharedPtrE
#[doc(
    alias = "Ogre::Entity::EntityShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)"
)]
// was: Ogre::Entity::EntityShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)
pub fn stub_c8a5e0() -> ! {
    todo!("0xc8a5e0 Ogre::Entity::EntityShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)")
}

// 0xc8a600 — __ZN4Ogre6Entity19setRenderQueueGroupEh
#[doc(alias = "Ogre::Entity::setRenderQueueGroup(unsigned char)")]
// was: Ogre::Entity::setRenderQueueGroup(unsigned char)
pub fn stub_c8a600() -> ! {
    todo!("0xc8a600 Ogre::Entity::setRenderQueueGroup(unsigned char)")
}

// 0xc8a638 — __ZN4Ogre6Entity30setRenderQueueGroupAndPriorityEht
#[doc(alias = "Ogre::Entity::setRenderQueueGroupAndPriority(unsigned char,unsigned short)")]
// was: Ogre::Entity::setRenderQueueGroupAndPriority(unsigned char,unsigned short)
pub fn stub_c8a638() -> ! {
    todo!("0xc8a638 Ogre::Entity::setRenderQueueGroupAndPriority(unsigned char,unsigned short)")
}

// 0xc8a67c — __ZNK4Ogre6Entity12getTypeFlagsEv
#[doc(alias = "Ogre::Entity::getTypeFlags(void)const")]
// was: Ogre::Entity::getTypeFlags(void)const
pub fn stub_c8a67c() -> ! {
    todo!("0xc8a67c Ogre::Entity::getTypeFlags(void)const")
}

// 0xc8a68c — __ZN4Ogre6Entity23getVertexDataForBindingEv
#[doc(alias = "Ogre::Entity::getVertexDataForBinding(void)")]
// was: Ogre::Entity::getVertexDataForBinding(void)
pub fn stub_c8a68c() -> ! {
    todo!("0xc8a68c Ogre::Entity::getVertexDataForBinding(void)")
}

// 0xc8a6f8 — __ZN4Ogre6Entity26chooseVertexDataForBindingEb
#[doc(alias = "Ogre::Entity::chooseVertexDataForBinding(bool)")]
// was: Ogre::Entity::chooseVertexDataForBinding(bool)
pub fn stub_c8a6f8() -> ! {
    todo!("0xc8a6f8 Ogre::Entity::chooseVertexDataForBinding(bool)")
}

// 0xc8a738 — __ZN4Ogre6Entity16visitRenderablesEPNS_10Renderable7VisitorEb
#[doc(alias = "Ogre::Entity::visitRenderables(Ogre::Renderable::Visitor *,bool)")]
// was: Ogre::Entity::visitRenderables(Ogre::Renderable::Visitor *,bool)
pub fn stub_c8a738() -> ! {
    todo!("0xc8a738 Ogre::Entity::visitRenderables(Ogre::Renderable::Visitor *,bool)")
}

// 0xc8a7d4 — __ZNK4Ogre13EntityFactory7getTypeEv
#[doc(alias = "Ogre::EntityFactory::getType(void)const")]
// was: Ogre::EntityFactory::getType(void)const
pub fn stub_c8a7d4() -> ! {
    todo!("0xc8a7d4 Ogre::EntityFactory::getType(void)const")
}

// 0xc8a7e0 — __ZN4Ogre13EntityFactory18createInstanceImplERKSsPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(
    alias = "Ogre::EntityFactory::createInstanceImpl(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)"
)]
// was: Ogre::EntityFactory::createInstanceImpl(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
pub fn stub_c8a7e0() -> ! {
    todo!("0xc8a7e0 Ogre::EntityFactory::createInstanceImpl(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")
}

// 0xc8ad84 — __ZN4Ogre13EntityFactory15destroyInstanceEPNS_13MovableObjectE
#[doc(alias = "Ogre::EntityFactory::destroyInstance(Ogre::MovableObject *)")]
// was: Ogre::EntityFactory::destroyInstance(Ogre::MovableObject *)
pub fn stub_c8ad84() -> ! {
    todo!("0xc8ad84 Ogre::EntityFactory::destroyInstance(Ogre::MovableObject *)")
}

// 0xc8ad98 — __ZN4Ogre14AxisAlignedBox15transformAffineERKNS_7Matrix4E
#[doc(alias = "Ogre::AxisAlignedBox::transformAffine(Ogre::Matrix4 const&)")]
// was: Ogre::AxisAlignedBox::transformAffine(Ogre::Matrix4 const&)
pub fn stub_c8ad98() -> ! {
    todo!("0xc8ad98 Ogre::AxisAlignedBox::transformAffine(Ogre::Matrix4 const&)")
}

// 0xc8af24 — __ZN4Ogre25RuntimeAssertionExceptionD1Ev
#[doc(alias = "Ogre::RuntimeAssertionException::~RuntimeAssertionException()")]
// was: Ogre::RuntimeAssertionException::~RuntimeAssertionException()
pub fn stub_c8af24() -> ! {
    todo!("0xc8af24 Ogre::RuntimeAssertionException::~RuntimeAssertionException()")
}

// 0xc8af30 — __ZN4Ogre8Resource8Listener27backgroundPreparingCompleteEPS0_
#[doc(alias = "Ogre::Resource::Listener::backgroundPreparingComplete(Ogre::Resource*)")]
// was: Ogre::Resource::Listener::backgroundPreparingComplete(Ogre::Resource*)
pub fn stub_c8af30() -> ! {
    todo!("0xc8af30 Ogre::Resource::Listener::backgroundPreparingComplete(Ogre::Resource*)")
}