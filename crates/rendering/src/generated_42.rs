//! rendering generated_42 — Ogre::|G3D:: strict 13333 total (13663 substr Ogre|G3D), 5348 prior, 120 this batch — 0x8c423c..0xa2f6ec
//! EA-sorted ascending earliest gap after 0x8c4220 — rbx_core::SharedPtr not boost
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x8c423c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::call(G3D::Vector2)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::call(G3D::Vector2)
// IDA 0x8c423c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c423c() {
}

// 0x8c4258 — __ZNK5boost9function1IvN3G3D7Vector2EEclES2_
#[doc(alias = "boost::function1<void,G3D::Vector2>::operator()(G3D::Vector2)const")]
// was: boost::function1<void,G3D::Vector2>::operator()(G3D::Vector2)const
// IDA 0x8c4258: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c4258() {
}

// 0x8c432c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6removeEPNS5_4slotE
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::remove(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::remove(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)
// IDA 0x8c432c: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c432c() {
}

// 0x8c4420 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_do_get_mutex(void)
// IDA 0x8c4420: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c4420() {
}

// 0x8c4510 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::~callable()
// IDA 0x8c4510: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8c4510() {
}

// 0x8c4620 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::~callable()
// IDA 0x8c4620: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8c4620() {
}

// 0x8c4750 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::slot::~slot()
// IDA 0x8c4750: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8c4750() {
}

// 0x8c477c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::slot::~slot()
// IDA 0x8c477c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8c477c() {
}

// 0x8c4850 — __ZN5boost9function1IvN3G3D7Vector2EE13assign_to_ownERKS3_
#[doc(alias = "boost::function1<void,G3D::Vector2>::assign_to_own(boost::function1<void,G3D::Vector2> const&)")]
// was: boost::function1<void,G3D::Vector2>::assign_to_own(boost::function1<void,G3D::Vector2> const&)
// IDA 0x8c4850: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c4850() {
}

// 0x8e12cc — __ZN3RBX9GuiBase2d34recalculateAbsoluteSizeAndPositionERKN3G3D6Rect2DE
#[doc(alias = "RBX::GuiBase2d::recalculateAbsoluteSizeAndPosition(G3D::Rect2D const&)")]
// was: RBX::GuiBase2d::recalculateAbsoluteSizeAndPosition(G3D::Rect2D const&)
// IDA 0x8e12cc: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e12cc() {
}

// 0x8e13a4 — __ZN3RBX9GuiBase2d19setAbsolutePositionERKN3G3D7Vector2E
#[doc(alias = "RBX::GuiBase2d::setAbsolutePosition(G3D::Vector2 const&)")]
// was: RBX::GuiBase2d::setAbsolutePosition(G3D::Vector2 const&)
// IDA 0x8e13a4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e13a4() {
}

// 0x8e13f0 — __ZN3RBX9GuiBase2d15setAbsoluteSizeERKN3G3D7Vector2E
#[doc(alias = "RBX::GuiBase2d::setAbsoluteSize(G3D::Vector2 const&)")]
// was: RBX::GuiBase2d::setAbsoluteSize(G3D::Vector2 const&)
// IDA 0x8e13f0: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e13f0() {
}

// 0x8e158c — __ZN3RBX9GuiBase2d12handleResizeERKN3G3D6Rect2DEb
#[doc(alias = "RBX::GuiBase2d::handleResize(G3D::Rect2D const&,bool)")]
// was: RBX::GuiBase2d::handleResize(G3D::Rect2D const&,bool)
// IDA 0x8e158c: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e158c() {
}

// 0x8e16d4 — __ZN3RBXL14ResizeChildrenEN5boost10shared_ptrINS_8InstanceEEERKN3G3D6Rect2DEb
#[doc(alias = "RBX::ResizeChildren(rbx_core::SharedPtr<RBX::Instance>,G3D::Rect2D const&,bool)")]
// was: RBX::ResizeChildren(boost::shared_ptr<RBX::Instance>,G3D::Rect2D const&,bool)
// IDA 0x8e16d4: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e16d4() {
}

// 0x8e1de8 — __ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIN3G3D6Rect2DEEENS4_IbEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEERKS6_bENS0_5list1IRKSE_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::Rect2D>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Rect2D const&,bool),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Rect2D const&,bool) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::Rect2D>,boost::_bi::value<bool>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,G3D::Rect2D const&,bool),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,G3D::Rect2D const&,bool) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
// IDA 0x8e1de8: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e1de8() {
}

// 0x8e242c — __ZN3RBX10Reflection14PropDescriptorINS_9GuiBase2dEN3G3D7Vector2EEC2IMS2_KFRKS4_vEiEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::PropDescriptor<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,int>(char const*,char const*,G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::PropDescriptor<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,int>(char const*,char const*,G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x8e242c: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e242c() {
}

// 0x8e2564 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase2dEN3G3D7Vector2EE7GetImplIMS2_KFRKS4_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::GetImpl<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::GetImpl<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const>::isReadOnly(void)const
// IDA 0x8e2564: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e2564() {
}

// 0x8e2568 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase2dEN3G3D7Vector2EE7GetImplIMS2_KFRKS4_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::GetImpl<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::GetImpl<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const>::isWriteOnly(void)const
// IDA 0x8e2568: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e2568() {
}

// 0x8e256c — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase2dEN3G3D7Vector2EE7GetImplIMS2_KFRKS4_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::GetImpl<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::GetImpl<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x8e256c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e256c() {
}

// 0x8e259c — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase2dEN3G3D7Vector2EE7GetImplIMS2_KFRKS4_vEE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::GetImpl<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::GetImpl<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const
// IDA 0x8e259c: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e259c() {
}

// 0x8e339c — __ZN3RBX17GuiLayerCollector27render2dStandardGuiElementsEPNS_5AdornEPKNS_8InstanceERSt6vectorIN5boost10shared_ptrINS_7GuiBaseEEESaISA_EERKN3G3D6Rect2DE
#[doc(alias = "RBX::GuiLayerCollector::render2dStandardGuiElements(RBX::Adorn *,RBX::Instance const*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> &,G3D::Rect2D const&)")]
// was: RBX::GuiLayerCollector::render2dStandardGuiElements(RBX::Adorn *,RBX::Instance const*,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> &,G3D::Rect2D const&)
// IDA 0x8e339c: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e339c() {
}

// 0x8e342c — __ZN3RBX17GuiLayerCollector23render2dTextGuiElementsEPNS_5AdornEPKNS_8InstanceERSt6vectorIN5boost10shared_ptrINS_7GuiBaseEEESaISA_EERKN3G3D6Rect2DE
#[doc(alias = "RBX::GuiLayerCollector::render2dTextGuiElements(RBX::Adorn *,RBX::Instance const*,std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>> &,G3D::Rect2D const&)")]
// was: RBX::GuiLayerCollector::render2dTextGuiElements(RBX::Adorn *,RBX::Instance const*,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> &,G3D::Rect2D const&)
// IDA 0x8e342c: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e342c() {
}

// 0x8e997c — __ZNK3RBX5Voxel4Grid23getVoxelLikelyThisChunkERKNS_13SpatialRegion2IdERKNS1_5ChunkERKN3G3D12Vector3int16E
#[doc(alias = "RBX::Voxel::Grid::getVoxelLikelyThisChunk(RBX::SpatialRegion::Id const&,RBX::Voxel::Grid::Chunk const&,G3D::Vector3int16 const&)const")]
// was: RBX::Voxel::Grid::getVoxelLikelyThisChunk(RBX::SpatialRegion::Id const&,RBX::Voxel::Grid::Chunk const&,G3D::Vector3int16 const&)const
// IDA 0x8e997c: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e997c() {
}

// 0x8e9a08 — __ZNK3RBX5Voxel4Grid17fillLocalAreaInfoERKN3G3D12Vector3int16ERKNS0_5Water17RelevantNeighborsEPNS6_13LocalAreaInfoE
#[doc(alias = "RBX::Voxel::Grid::fillLocalAreaInfo(G3D::Vector3int16 const&,RBX::Voxel::Water::RelevantNeighbors const&,RBX::Voxel::Water::LocalAreaInfo *)const")]
// was: RBX::Voxel::Grid::fillLocalAreaInfo(G3D::Vector3int16 const&,RBX::Voxel::Water::RelevantNeighbors const&,RBX::Voxel::Water::LocalAreaInfo *)const
// IDA 0x8e9a08: 254 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e9a08() {
}

// 0x8e9cd0 — __ZN3RBX5Voxel4Grid7setCellERKN3G3D12Vector3int16ENS0_4CellENS0_12CellMaterialE
#[doc(alias = "RBX::Voxel::Grid::setCell(G3D::Vector3int16 const&,RBX::Voxel::Cell,RBX::Voxel::CellMaterial)")]
// was: RBX::Voxel::Grid::setCell(G3D::Vector3int16 const&,RBX::Voxel::Cell,RBX::Voxel::CellMaterial)
// IDA 0x8e9cd0: 214 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e9cd0() {
}

// 0x8e9f34 — __ZNK3RBX5Voxel4Grid9getRegionERKN3G3D12Vector3int16ES5_
#[doc(alias = "RBX::Voxel::Grid::getRegion(G3D::Vector3int16 const&,G3D::Vector3int16 const&)const")]
// was: RBX::Voxel::Grid::getRegion(G3D::Vector3int16 const&,G3D::Vector3int16 const&)const
// IDA 0x8e9f34: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8e9f34() {
}

// 0x8ea008 — __ZNK3RBX5Voxel4Grid15getCellInternalERKN3G3D12Vector3int16E
#[doc(alias = "RBX::Voxel::Grid::getCellInternal(G3D::Vector3int16 const&)const")]
// was: RBX::Voxel::Grid::getCellInternal(G3D::Vector3int16 const&)const
// IDA 0x8ea008: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8ea008() {
}

// 0x8ea068 — __ZNK3RBX5Voxel4Grid23getCellMaterialInternalERKN3G3D12Vector3int16E
#[doc(alias = "RBX::Voxel::Grid::getCellMaterialInternal(G3D::Vector3int16 const&)const")]
// was: RBX::Voxel::Grid::getCellMaterialInternal(G3D::Vector3int16 const&)const
// IDA 0x8ea068: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8ea068() {
}

// 0x8ea0dc — __ZNK3RBX5Voxel4Grid20getWaterCellInternalERKN3G3D12Vector3int16E
#[doc(alias = "RBX::Voxel::Grid::getWaterCellInternal(G3D::Vector3int16 const&)const")]
// was: RBX::Voxel::Grid::getWaterCellInternal(G3D::Vector3int16 const&)const
// IDA 0x8ea0dc: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8ea0dc() {
}

// 0x8ea2f0 — __ZN3RBX5Voxel5Water12cellHasWaterINS0_4Grid5ChunkEEEbPKT_RKNS0_4CellERKN3G3D12Vector3int16E
#[doc(alias = "bool RBX::Voxel::Water::cellHasWater<RBX::Voxel::Grid::Chunk>(RBX::Voxel::Grid::Chunk const*,RBX::Voxel::Cell const&,G3D::Vector3int16 const&)")]
// was: bool RBX::Voxel::Water::cellHasWater<RBX::Voxel::Grid::Chunk>(RBX::Voxel::Grid::Chunk const*,RBX::Voxel::Cell const&,G3D::Vector3int16 const&)
// IDA 0x8ea2f0: 166 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8ea2f0() {
}

// 0x8ed314 — __ZN3RBX16OnScreenProfiler14GetRandomColorEPN3G3D6Color4E
#[doc(alias = "RBX::OnScreenProfiler::GetRandomColor(G3D::Color4 *)")]
// was: RBX::OnScreenProfiler::GetRandomColor(G3D::Color4 *)
// IDA 0x8ed314: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8ed314() {
}

// 0x8f1c30 — __ZN3RBX16OnScreenProfiler7AddTextEPNS_5AdornEPKcRffffRKN3G3D6Color4EPNS6_7Vector2E
#[doc(alias = "RBX::OnScreenProfiler::AddText(RBX::Adorn *,char const*,float &,float,float,float,G3D::Color4 const&,G3D::Vector2 *)")]
// was: RBX::OnScreenProfiler::AddText(RBX::Adorn *,char const*,float &,float,float,float,G3D::Color4 const&,G3D::Vector2 *)
// IDA 0x8f1c30: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8f1c30() {
}

// 0x8f2008 — __ZN3RBX16OnScreenProfiler7DrawBarEPNS_5AdornERN3G3D7Vector2EfRKNS3_6Color4ES8_ff
#[doc(alias = "RBX::OnScreenProfiler::DrawBar(RBX::Adorn *,G3D::Vector2 &,float,G3D::Color4 const&,G3D::Color4 const&,float,float)")]
// was: RBX::OnScreenProfiler::DrawBar(RBX::Adorn *,G3D::Vector2 &,float,G3D::Color4 const&,G3D::Color4 const&,float,float)
// IDA 0x8f2008: 92 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8f2008() {
}

// 0x8f7130 — __ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE6resizeEmS1_
#[doc(alias = "std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::resize(unsigned long,G3D::Matrix3)")]
// was: std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::resize(unsigned long,G3D::Matrix3)
// IDA 0x8f7130: 24 insns (PUSH.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8f7130() {
}

// 0x8f7274 — __ZNSt12_Vector_baseIN3G3D7Matrix3ESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_allocate(unsigned long)
// IDA 0x8f7274: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_8f7274() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x8f7298 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Matrix3ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Matrix3 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Matrix3 *,G3D::Matrix3 *>(G3D::Matrix3 *,G3D::Matrix3 *,G3D::Matrix3 *)")]
// was: G3D::Matrix3 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Matrix3 *,G3D::Matrix3 *>(G3D::Matrix3 *,G3D::Matrix3 *,G3D::Matrix3 *)
// IDA 0x8f7298: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_8f7298() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x8f7308 — __ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
#[doc(alias = "std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Matrix3*,std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>>,unsigned long,G3D::Matrix3 const&)")]
// was: std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Matrix3*,std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>>,unsigned long,G3D::Matrix3 const&)
// IDA 0x8f7308: 366 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8f7308() {
}

// 0x8f76c4 — __ZSt26__uninitialized_fill_n_auxIPN3G3D7Matrix3EmS1_EvT_T0_RKT1_St12__false_type
#[doc(alias = "void std::__uninitialized_fill_n_aux<G3D::Matrix3 *,unsigned long,G3D::Matrix3>(G3D::Matrix3 *,unsigned long,G3D::Matrix3 const&,std::__false_type)")]
// was: void std::__uninitialized_fill_n_aux<G3D::Matrix3 *,unsigned long,G3D::Matrix3>(G3D::Matrix3 *,unsigned long,G3D::Matrix3 const&,std::__false_type)
// IDA 0x8f76c4: 77 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8f76c4() {
}

// 0x9491f0 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::resize(int,bool)")]
// was: G3D::Array<G3D::Plane,10,32ul>::resize(int,bool)
// IDA 0x9491f0: 103 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9491f0() {
}

// 0x949328 — __ZN3G3D5PlaneD1Ev
#[doc(alias = "G3D::Plane::~Plane()")]
// was: G3D::Plane::~Plane()
// IDA 0x949328: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_949328() {
}

// 0x949330 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::Array(void)")]
// was: G3D::Array<G3D::Plane,10,32ul>::Array(void)
// IDA 0x949330: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_949330() {
}

// 0x949adc — __ZNK3RBX6RbxRay17intersectionPlaneERKN3G3D5PlaneE
#[doc(alias = "RBX::RbxRay::intersectionPlane(G3D::Plane const&)const")]
// was: RBX::RbxRay::intersectionPlane(G3D::Plane const&)const
// IDA 0x949adc: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_949adc() {
}

// 0x949be4 — __ZN3RBX6RbxG3D8SpecDataC1Ev
#[doc(alias = "RBX::RbxG3D::SpecData::SpecData(void)")]
// was: RBX::RbxG3D::SpecData::SpecData(void)
// IDA 0x949be4: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_949be4() {
}

// 0x949bf0 — __ZN3RBX6RbxG3D8SpecDataC2Ev
#[doc(alias = "RBX::RbxG3D::SpecData::SpecData(void)")]
// was: RBX::RbxG3D::SpecData::SpecData(void)
// IDA 0x949bf0: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_949bf0() {
}

// 0x949d08 — __ZN3RBX7FrustumC1ERKN3G3D7Vector3ES4_S4_ffff
#[doc(alias = "RBX::Frustum::Frustum(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,float,float,float,float)")]
// was: RBX::Frustum::Frustum(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,float,float,float,float)
// IDA 0x949d08: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_949d08() {
}

// 0x949d40 — __ZN3RBX7FrustumC2ERKN3G3D7Vector3ES4_S4_ffff
#[doc(alias = "RBX::Frustum::Frustum(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,float,float,float,float)")]
// was: RBX::Frustum::Frustum(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,float,float,float,float)
// IDA 0x949d40: 592 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_949d40() {
}

// 0x94a45c — __ZNK3RBX7Frustum13containsPointERKN3G3D7Vector3E
#[doc(alias = "RBX::Frustum::containsPoint(G3D::Vector3 const&)const")]
// was: RBX::Frustum::containsPoint(G3D::Vector3 const&)const
// IDA 0x94a45c: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94a45c() {
}

// 0x94a620 — __ZNK3RBX7Frustum16intersectsSphereERKN3G3D7Vector3Ef
#[doc(alias = "RBX::Frustum::intersectsSphere(G3D::Vector3 const&,float)const")]
// was: RBX::Frustum::intersectsSphere(G3D::Vector3 const&,float)const
// IDA 0x94a620: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94a620() {
}

// 0x94a724 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6appendERKS1_
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::append(G3D::Plane const&)")]
// was: G3D::Array<G3D::Plane,10,32ul>::append(G3D::Plane const&)
// IDA 0x94a724: 140 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94a724() {
}

// 0x94ab54 — __ZN3RBX4Draw9partAdornERKNS_4PartEPNS_5AdornERKN3G3D6Color3E
#[doc(alias = "RBX::Draw::partAdorn(RBX::Part const&,RBX::Adorn *,G3D::Color3 const&)")]
// was: RBX::Draw::partAdorn(RBX::Part const&,RBX::Adorn *,G3D::Color3 const&)
// IDA 0x94ab54: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94ab54() {
}

// 0x94ab90 — __ZN3RBX4Draw13adornSurfacesERKNS_4PartEPNS_5AdornERKN3G3D6Color3E
#[doc(alias = "RBX::Draw::adornSurfaces(RBX::Part const&,RBX::Adorn *,G3D::Color3 const&)")]
// was: RBX::Draw::adornSurfaces(RBX::Part const&,RBX::Adorn *,G3D::Color3 const&)
// IDA 0x94ab90: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94ab90() {
}

// 0x94b044 — __ZN3RBX4Draw10constraintERKNS_4PartEPNS_5AdornEiRKN3G3D6Color3E
#[doc(alias = "RBX::Draw::constraint(RBX::Part const&,RBX::Adorn *,int,G3D::Color3 const&)")]
// was: RBX::Draw::constraint(RBX::Part const&,RBX::Adorn *,int,G3D::Color3 const&)
// IDA 0x94b044: 218 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94b044() {
}

// 0x94b738 — __ZN3RBX9DrawAdorn8cylinderEPNS_5AdornERKN3G3D15CoordinateFrameEiffRKNS3_6Color4Eb
#[doc(alias = "RBX::DrawAdorn::cylinder(RBX::Adorn *,G3D::CoordinateFrame const&,int,float,float,G3D::Color4 const&,bool)")]
// was: RBX::DrawAdorn::cylinder(RBX::Adorn *,G3D::CoordinateFrame const&,int,float,float,G3D::Color4 const&,bool)
// IDA 0x94b738: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94b738() {
}

// 0x94b79c — __ZN3RBX9DrawAdorn13surfaceBorderEPNS_5AdornERKN3G3D7Vector3EfiRKNS3_6Color4E
#[doc(alias = "RBX::DrawAdorn::surfaceBorder(RBX::Adorn *,G3D::Vector3 const&,float,int,G3D::Color4 const&)")]
// was: RBX::DrawAdorn::surfaceBorder(RBX::Adorn *,G3D::Vector3 const&,float,int,G3D::Color4 const&)
// IDA 0x94b79c: 154 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94b79c() {
}

// 0x94b974 — __ZN3RBX9DrawAdorn18surfaceGridAtCoordEPNS_5AdornERN3G3D15CoordinateFrameERKNS3_7Vector4ERKNS3_7Vector3ESB_RKNS3_6Color4Ei
#[doc(alias = "RBX::DrawAdorn::surfaceGridAtCoord(RBX::Adorn *,G3D::CoordinateFrame &,G3D::Vector4 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Color4 const&,int)")]
// was: RBX::DrawAdorn::surfaceGridAtCoord(RBX::Adorn *,G3D::CoordinateFrame &,G3D::Vector4 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Color4 const&,int)
// IDA 0x94b974: 568 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94b974() {
}

// 0x94c058 — __ZN3RBX9DrawAdorn13zeroPlaneGridEPNS_5AdornERKNS_6CameraEiiRKN3G3D6Color4ES9_
#[doc(alias = "RBX::DrawAdorn::zeroPlaneGrid(RBX::Adorn *,RBX::Camera const&,int,int,G3D::Color4 const&,G3D::Color4 const&)")]
// was: RBX::DrawAdorn::zeroPlaneGrid(RBX::Adorn *,RBX::Camera const&,int,int,G3D::Color4 const&,G3D::Color4 const&)
// IDA 0x94c058: 711 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94c058() {
}

// 0x94d01c — __ZN3RBX9DrawAdorn19circularGridAtCoordEPNS_5AdornERKN3G3D15CoordinateFrameERKNS3_7Vector3ES9_NS_8NormalIdERKNS3_6Color4Ei
#[doc(alias = "RBX::DrawAdorn::circularGridAtCoord(RBX::Adorn *,G3D::CoordinateFrame const&,G3D::Vector3 const&,G3D::Vector3 const&,RBX::NormalId,G3D::Color4 const&,int)")]
// was: RBX::DrawAdorn::circularGridAtCoord(RBX::Adorn *,G3D::CoordinateFrame const&,G3D::Vector3 const&,G3D::Vector3 const&,RBX::NormalId,G3D::Color4 const&,int)
// IDA 0x94d01c: 246 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94d01c() {
}

// 0x94d374 — __ZN3RBX9DrawAdorn17handlePosInObjectERKN3G3D7Vector3ERKNS_7ExtentsENS_10HandleTypeENS_8NormalIdE
#[doc(alias = "RBX::DrawAdorn::handlePosInObject(G3D::Vector3 const&,RBX::Extents const&,RBX::HandleType,RBX::NormalId)")]
// was: RBX::DrawAdorn::handlePosInObject(G3D::Vector3 const&,RBX::Extents const&,RBX::HandleType,RBX::NormalId)
// IDA 0x94d374: 147 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94d374() {
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

// 0x94db48 — __ZN3RBX9DrawAdorn11partSurfaceERKNS_4PartEiPNS_5AdornERKN3G3D6Color4Ef
#[doc(alias = "RBX::DrawAdorn::partSurface(RBX::Part const&,int,RBX::Adorn *,G3D::Color4 const&,float)")]
// was: RBX::DrawAdorn::partSurface(RBX::Part const&,int,RBX::Adorn *,G3D::Color4 const&,float)
// IDA 0x94db48: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94db48() {
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

// 0x94f6a0 — __ZN3RBX7HitTest7hitTestERKNS_4PartERNS_6RbxRayERN3G3D7Vector3Ef
#[doc(alias = "RBX::HitTest::hitTest(RBX::Part const&,RBX::RbxRay &,G3D::Vector3 &,float)")]
// was: RBX::HitTest::hitTest(RBX::Part const&,RBX::RbxRay &,G3D::Vector3 &,float)
// IDA 0x94f6a0: 142 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94f6a0() {
}

// 0x952b38 — __ZN3RBX7Network23TopNErrorsPhysicsSender6Nugget12computeErrorERKN3G3D15CoordinateFrameEPKNS_13ModelInstanceEi
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::Nugget::computeError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)")]
// was: RBX::Network::TopNErrorsPhysicsSender::Nugget::computeError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)
// IDA 0x952b38: 172 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_952b38() {
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

// 0x963a58 — __ZNK3G3D7Vector38isFiniteEv
#[doc(alias = "G3D::Vector3::isFinite(void)const")]
// was: G3D::Vector3::isFinite(void)const
// IDA 0x963a58: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_963a58() {
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

// 0x9a75f0 — __ZN3RBX7Network22ErrorCompPhysicsSender6Nugget12computeErrorERKN3G3D15CoordinateFrameEPKNS_13ModelInstanceEi
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::Nugget::computeError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)")]
// was: RBX::Network::ErrorCompPhysicsSender::Nugget::computeError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)
// IDA 0x9a75f0: 194 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9a75f0() {
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

// 0xa1624c — __ZN3RBX7Network7Players24remoteInsertResultHelperEN5boost8weak_ptrIS1_EENS2_10shared_ptrINS_8InstanceEEEN3G3D7Vector3E
#[doc(alias = "RBX::Network::Players::remoteInsertResultHelper(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3)")]
// was: RBX::Network::Players::remoteInsertResultHelper(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3)
// IDA 0xa1624c: 386 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a1624c() {
}

// 0xa16648 — __ZN3RBX7Network7Players18remoteInsertResultEN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3E
#[doc(alias = "RBX::Network::Players::remoteInsertResult(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3)")]
// was: RBX::Network::Players::remoteInsertResult(boost::shared_ptr<RBX::Instance>,G3D::Vector3)
// IDA 0xa16648: 232 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a16648() {
}

// 0xa168dc — __ZN3RBX7Network7Players12remoteInsertEiSsN3G3D7Vector3E
#[doc(alias = "RBX::Network::Players::remoteInsert(int,std::string,G3D::Vector3)")]
// was: RBX::Network::Players::remoteInsert(int,std::string,G3D::Vector3)
// IDA 0xa168dc: 362 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a168dc() {
}

// 0xa1ff60 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS2_8InstanceEEEN3G3D7Vector3ES5_NS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<rbx_core::Weak<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::Weak<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),rbx_core::Weak<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)
// IDA 0xa1ff60: 287 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a1ff60() {
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

// 0xa29454 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6insertEPNS5_4slotE
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::insert(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::insert(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)
// IDA 0xa29454: 253 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a29454() {
}

// 0xa29714 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE5mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::mutex(void)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::mutex(void)
// IDA 0xa29714: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a29714() {
}

// 0xa29828 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSEPS8_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot*)
// IDA 0xa29828: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a29828() {
}

// 0xa298dc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSERKS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> const&)
// IDA 0xa298dc: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a298dc() {
}

// 0xa29990 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::safe_static_init_mutex(void)
// IDA 0xa29990: 79 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a29990() {
}

// 0xa29a78 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS3_EENS8_5list4INS8_5valueIPSE_EENSH_IiEENS7_3argILi1EEENSL_ILi2EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
// IDA 0xa29a78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a29a78() {
}

// 0xa29ad4 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS3_EENS8_5list4INS8_5valueIPSE_EENSH_IiEENS7_3argILi1EEENSL_ILi2EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
// IDA 0xa29ad4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a29ad4() {
}

// 0xa29d5c — __ZNK3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::connected(void)const
// IDA 0xa29d5c: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a29d5c() {
}

// 0xa29d68 — __ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS4_EENS9_5list4INS9_5valueIPSF_EENSI_IiEENS8_3argILi1EEENSM_ILi2EEEEEEELi2ES5_E4callESsS4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)")]
// was: rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)
// IDA 0xa29d68: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a29d68() {
}

// 0xa29d90 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS4_EENS9_5list4INS9_5valueIPSF_EENSI_IiEENS8_3argILi1EEENSM_ILi2EEEEEEELi2ES5_E4callESsS4_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)
// IDA 0xa29d90: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a29d90() {
}

// 0xa29db8 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEclINS_4_mfi3mf3IvS5_iSsN3G3D7Vector3EEENS0_5list2IRSsRSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list2<std::string &,G3D::Vector3&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3> &,boost::_bi::list2<std::string &,G3D::Vector3&> &,int)")]
// was: void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list2<std::string &,G3D::Vector3&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3> &,boost::_bi::list2<std::string &,G3D::Vector3&> &,int)
// IDA 0xa29db8: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a29db8() {
}

// 0xa29f90 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6removeEPNS5_4slotE
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::remove(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::remove(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)
// IDA 0xa29f90: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a29f90() {
}

// 0xa2a07c — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::safe_static_init_mutex(void)
// IDA 0xa2a07c: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a2a07c() {
}

// 0xa2a160 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::~slot()
// IDA 0xa2a160: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a2a160() {
}

// 0xa2a1bc — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::~slot()
// IDA 0xa2a1bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a2a1bc() {
}

// 0xa2f6ec — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>)")]
// was: void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>)
// IDA 0xa2f6ec: 170 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a2f6ec() {
}
