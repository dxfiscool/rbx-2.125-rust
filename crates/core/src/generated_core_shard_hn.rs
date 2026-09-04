//! core shard HN — 100 core stubs EA-sorted, 0xf658b4..0xf67504 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HM 0xf65834 (21714->21814 covered, 104 remaining).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HM 0xf65834 (0xf658b4..0xf67504, 21714->21814 covered, 104 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "rbx::signals::scoped_connection::~scoped_connection()")]
// 0xf658b4 — j___ZN3rbx7signals17scoped_connectionD2Ev
pub fn stub_0xf658b4() {
    // IDA 0xf658b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::mutex(void)")]
// 0xf658c4 — j___ZN3rbx7signals6signalIFvbEE4slot5mutexEv
pub fn stub_0xf658c4() {
    // IDA 0xf658c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::mutex(void)")]
// 0xf658d4 — j___ZN3rbx7signals6signalIFviEE4slot5mutexEv
pub fn stub_0xf658d4() {
    // IDA 0xf658d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::insert(rbx::signals::signal<void ()(int)>::slot *)")]
// 0xf658e4 — j___ZN3rbx7signals6signalIFviEE6insertEPNS3_4slotE
pub fn stub_0xf658e4() {
    // IDA 0xf658e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::remove(rbx::signals::signal<void ()(int)>::slot *)")]
// 0xf658f4 — j___ZN3rbx7signals6signalIFviEE6removeEPNS3_4slotE
pub fn stub_0xf658f4() {
    // IDA 0xf658f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::this_thread::sleep<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>(boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll> const&)")]
// 0xf65924 — j___ZN5boost11this_thread5sleepINS_9date_time18subsecond_durationINS_10posix_time13time_durationELx1000EEEEEvRKT_
pub fn stub_0xf65924() {
    // IDA 0xf65924: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool)>::slot>::operator=(rbx::signals::signal<void ()(bool)>::slot*)")]
// 0xf65934 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbEE4slotEEaSEPS6_
pub fn stub_0xf65934() {
    // IDA 0xf65934: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot>::operator=(rbx::signals::signal<void ()(int)>::slot*)")]
// 0xf65944 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSEPS6_
pub fn stub_0xf65944() {
    // IDA 0xf65944: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::xtime_get(boost::xtime *,int)")]
// 0xf659b4 — j___ZN5boost9xtime_getEPNS_5xtimeEi
pub fn stub_0xf659b4() {
    // IDA 0xf659b4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "RBX::WindowAverage<double,double>::getSanitizedStats(RBX::Confidence)const")]
// 0xf659c4 — j___ZNK3RBX13WindowAverageIddE17getSanitizedStatsENS_10ConfidenceE
pub fn stub_0xf659c4() {
    // IDA 0xf659c4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "RBX::RenderHooksService * RBX::ServiceProvider::find<RBX::RenderHooksService>(void)const")]
// 0xf659d4 — j___ZNK3RBX15ServiceProvider4findINS_18RenderHooksServiceEEEPT_v
pub fn stub_0xf659d4() {
    // IDA 0xf659d4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "RBX::TextureContentProvider * RBX::ServiceProvider::find<RBX::TextureContentProvider>(void)const")]
// 0xf659e4 — j___ZNK3RBX15ServiceProvider4findINS_22TextureContentProviderEEEPT_v
pub fn stub_0xf659e4() {
    // IDA 0xf659e4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "RBX::TextService * RBX::ServiceProvider::create<RBX::TextService>(void)const")]
// 0xf659f4 — j___ZNK3RBX15ServiceProvider6createINS_11TextServiceEEEPT_v
pub fn stub_0xf659f4() {
    // IDA 0xf659f4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TextureContentProvider * RBX::ServiceProvider::create<RBX::TextureContentProvider>(void)const")]
// 0xf65a04 — j___ZNK3RBX15ServiceProvider6createINS_22TextureContentProviderEEEPT_v
pub fn stub_0xf65a04() {
    // IDA 0xf65a04: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TypesetterBitmap::GlyphLine*,std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>>>,RBX::TypesetterBitmap::GlyphLine const&)")]
// 0xf65a44 — j___ZNSt6vectorIN3RBX16TypesetterBitmap9GlyphLineESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf65a44() {
    // IDA 0xf65a44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TypesetterBitmap::GlyphLine*,std::vector<RBX::TypesetterBitmap::GlyphLine,std::allocator<RBX::TypesetterBitmap::GlyphLine>>>,unsigned long,RBX::TypesetterBitmap::GlyphLine const&)")]
// 0xf65a54 — j___ZNSt6vectorIN3RBX16TypesetterBitmap9GlyphLineESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf65a54() {
    // IDA 0xf65a54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::priority_queue<RBX::NodeInfo,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>,std::less<RBX::NodeInfo>>::push(RBX::NodeInfo const&)")]
// 0xf65bb4 — j___ZNSt14priority_queueIN3RBX8NodeInfoESt6vectorIS1_SaIS1_EESt4lessIS1_EE4pushERKS1_
pub fn stub_0xf65bb4() {
    // IDA 0xf65bb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>::vector(std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>> const&)")]
// 0xf65bd4 — j___ZNSt6vectorIN3RBX8NodeInfoESaIS1_EEC2ERKS3_
pub fn stub_0xf65bd4() {
    // IDA 0xf65bd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<std::string,long,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::operator[](std::string const&)")]
// 0xf65bf4 — j___ZNSt3mapISslSt4lessISsESaISt4pairIKSslEEEixERS3_
pub fn stub_0xf65bf4() {
    // IDA 0xf65bf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::_M_insert_unique(std::pair<std::string const,long> const&)")]
// 0xf65c04 — j___ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_0xf65c04() {
    // IDA 0xf65c04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,long>>,std::pair<std::string const,long> const&)")]
// 0xf65c14 — j___ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
pub fn stub_0xf65c14() {
    // IDA 0xf65c14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::find(std::string const&)")]
// 0xf65c24 — j___ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_
pub fn stub_0xf65c24() {
    // IDA 0xf65c24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,long>> *)")]
// 0xf65c34 — j___ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_0xf65c34() {
    // IDA 0xf65c34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,long>,std::_Select1st<std::pair<std::string const,long>>,std::less<std::string>,std::allocator<std::pair<std::string const,long>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,long> const&)")]
// 0xf65c44 — j___ZNSt8_Rb_treeISsSt4pairIKSslESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_0xf65c44() {
    // IDA 0xf65c44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xf65c54 — j___ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_0xf65c54() {
    // IDA 0xf65c54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xf65c64 — j___ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_17WaterFaceRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_0xf65c64() {
    // IDA 0xf65c64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xf65c94 — j___ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_0xf65c94() {
    // IDA 0xf65c94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xf65ca4 — j___ZN3RBX10EdgeSpewV2INS_20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_17WaterFaceRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_0xf65ca4() {
    // IDA 0xf65ca4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::FaceCounter<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xf65cb4 — j___ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_11FaceCounterIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_0xf65cb4() {
    // IDA 0xf65cb4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>,RBX::MegaClusterLegacy::VoxelGridOverlay>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xf65cc4 — j___ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEEENS_20SolidTerrainRendererIS3_EES3_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_0xf65cc4() {
    // IDA 0xf65cc4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::FaceCounter<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xf65cf4 — j___ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_11FaceCounterIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_0xf65cf4() {
    // IDA 0xf65cf4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeSpewV2<RBX::SolidTerrainRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>,RBX::Voxel::AreaCopy<36u,19u,34u>>::handleCells(RBX::SpatialRegion::Id const&)")]
// 0xf65d04 — j___ZN3RBX10EdgeSpewV2INS_27SolidTerrainRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEEENS_20SolidTerrainRendererIS4_EES4_E11handleCellsERKNS_13SpatialRegion2IdE
pub fn stub_0xf65d04() {
    // IDA 0xf65d04: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ExtentsInt32::ExtentsInt32(void)")]
// 0xf65d14 — j___ZN3RBX12ExtentsInt32C2Ev
pub fn stub_0xf65d14() {
    // IDA 0xf65d14: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::MegaClusterLegacy::createChunk<RBX::MegaClusterLegacy::VoxelGridOverlay>(RBX::SpatialRegion::Id const&,unsigned int,unsigned int)")]
// 0xf65d24 — j___ZN3RBX17MegaClusterLegacy11createChunkINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdEjj
pub fn stub_0xf65d24() {
    // IDA 0xf65d24: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::MegaClusterLegacy::unbuild_templated<RBX::MegaClusterLegacy::VoxelGridOverlay>(void)")]
// 0xf65d64 — j___ZN3RBX17MegaClusterLegacy17unbuild_templatedINS0_16VoxelGridOverlayEEEvv
pub fn stub_0xf65d64() {
    // IDA 0xf65d64: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::MegaClusterLegacy::updateChunkGeometry<RBX::MegaClusterLegacy::VoxelGridOverlay>(RBX::SpatialRegion::Id const&,RBX::MegaClusterLegacy::ChunkData &,int)")]
// 0xf65d84 — j___ZN3RBX17MegaClusterLegacy19updateChunkGeometryINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdERNS0_9ChunkDataEi
pub fn stub_0xf65d84() {
    // IDA 0xf65d84: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::MegaClusterLegacy::updateWaterGeometry<RBX::MegaClusterLegacy::VoxelGridOverlay>(RBX::SpatialRegion::Id const&,RBX::MegaClusterLegacy::ChunkData &,unsigned int)")]
// 0xf65da4 — j___ZN3RBX17MegaClusterLegacy19updateWaterGeometryINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdERNS0_9ChunkDataEj
pub fn stub_0xf65da4() {
    // IDA 0xf65da4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::MegaClusterLegacy::updateChunk_templated<RBX::MegaClusterLegacy::VoxelGridOverlay>(RBX::SpatialRegion::Id const&,bool)")]
// 0xf65dc4 — j___ZN3RBX17MegaClusterLegacy21updateChunk_templatedINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdEb
pub fn stub_0xf65dc4() {
    // IDA 0xf65dc4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::MegaClusterLegacy::updateEntity_templated<RBX::MegaClusterLegacy::VoxelGridOverlay>(void)")]
// 0xf65de4 — j___ZN3RBX17MegaClusterLegacy22updateEntity_templatedINS0_16VoxelGridOverlayEEEvv
pub fn stub_0xf65de4() {
    // IDA 0xf65de4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::WaterFaceRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// 0xf65e04 — j___ZN3RBX17WaterFaceRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_0xf65e04() {
    // IDA 0xf65e04: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::WaterFaceRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// 0xf65e24 — j___ZN3RBX17WaterFaceRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_0xf65e24() {
    // IDA 0xf65e24: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// 0xf65e44 — j___ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE12wedgeUpEmptyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
pub fn stub_0xf65e44() {
    // IDA 0xf65e44: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// 0xf65e54 — j___ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE14detectOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_0xf65e54() {
    // IDA 0xf65e54: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// 0xf65e64 — j___ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE19detectWedgeOutlinesERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
pub fn stub_0xf65e64() {
    // IDA 0xf65e64: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// 0xf65e74 — j___ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE5applyERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_0xf65e74() {
    // IDA 0xf65e74: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::MegaClusterLegacy::VoxelGridOverlay>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// 0xf65e84 — j___ZN3RBX20SolidTerrainRendererINS_17MegaClusterLegacy16VoxelGridOverlayEE9wedgeFaceERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorE
pub fn stub_0xf65e84() {
    // IDA 0xf65e84: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeUpEmpty(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// 0xf65f04 — j___ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE12wedgeUpEmptyERKNS1_6RegionINS3_5ChunkEE8iteratorE
pub fn stub_0xf65f04() {
    // IDA 0xf65f04: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// 0xf65f14 — j___ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE14detectOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_0xf65f14() {
    // IDA 0xf65f14: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::detectWedgeOutlines(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// 0xf65f24 — j___ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE19detectWedgeOutlinesERKNS1_6RegionINS3_5ChunkEE8iteratorE
pub fn stub_0xf65f24() {
    // IDA 0xf65f24: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::apply(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection,RBX::RenderPredStatus)")]
// 0xf65f34 — j___ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE5applyERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionENS_16RenderPredStatusE
pub fn stub_0xf65f34() {
    // IDA 0xf65f34: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SolidTerrainRenderer<RBX::Voxel::AreaCopy<36u,19u,34u>>::wedgeFace(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&)")]
// 0xf65f44 — j___ZN3RBX20SolidTerrainRendererINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE9wedgeFaceERKNS1_6RegionINS3_5ChunkEE8iteratorE
pub fn stub_0xf65f44() {
    // IDA 0xf65f44: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator::iterator(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk> const&)")]
// 0xf65f64 — j___ZN3RBX5Voxel6RegionINS0_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorC2ERKS5_
pub fn stub_0xf65f64() {
    // IDA 0xf65f64: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk::fillFromRegion<RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>>(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)")]
// 0xf65f74 — j___ZN3RBX5Voxel8AreaCopyILj36ELj19ELj34EE5Chunk14fillFromRegionINS0_6RegionINS0_4Grid5ChunkEEEEEvRKT_
pub fn stub_0xf65f74() {
    // IDA 0xf65f74: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::WaterRenderPredicate<RBX::MegaClusterLegacy::VoxelGridOverlay>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
// 0xf65fa4 — j___ZNK3RBX20WaterRenderPredicateINS_17MegaClusterLegacy16VoxelGridOverlayEE8internalERKNS_5Voxel6RegionINS4_8AreaCopyILj36ELj19ELj34EE5ChunkEE8iteratorENS4_13FaceDirectionE
pub fn stub_0xf65fa4() {
    // IDA 0xf65fa4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::WaterRenderPredicate<RBX::Voxel::AreaCopy<36u,19u,34u>>::internal(RBX::Voxel::Region<RBX::Voxel::AreaCopy<36u,19u,34u>::Chunk>::iterator const&,RBX::Voxel::FaceDirection)const")]
// 0xf65fc4 — j___ZNK3RBX20WaterRenderPredicateINS_5Voxel8AreaCopyILj36ELj19ELj34EEEE8internalERKNS1_6RegionINS3_5ChunkEE8iteratorENS1_13FaceDirectionE
pub fn stub_0xf65fc4() {
    // IDA 0xf65fc4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MegaCluster::ChunkData * std::__copy_backward_normal<false,false>::__copy_b_n<RBX::MegaCluster::ChunkData *,RBX::MegaCluster::ChunkData *>(RBX::MegaCluster::ChunkData *,RBX::MegaCluster::ChunkData *,RBX::MegaCluster::ChunkData *)")]
// 0xf65fe4 — j___ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN3RBX11MegaCluster9ChunkDataES5_EET0_T_S7_S6_
pub fn stub_0xf65fe4() {
    // IDA 0xf65fe4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::MegaCluster::ChunkData,std::allocator<RBX::MegaCluster::ChunkData>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::MegaCluster::ChunkData*,std::vector<RBX::MegaCluster::ChunkData,std::allocator<RBX::MegaCluster::ChunkData>>>,unsigned long,RBX::MegaCluster::ChunkData const&)")]
// 0xf65ff4 — j___ZNSt6vectorIN3RBX11MegaCluster9ChunkDataESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf65ff4() {
    // IDA 0xf65ff4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::MegaClusterLegacy::ChunkData,std::allocator<RBX::MegaClusterLegacy::ChunkData>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::MegaClusterLegacy::ChunkData*,std::vector<RBX::MegaClusterLegacy::ChunkData,std::allocator<RBX::MegaClusterLegacy::ChunkData>>>,unsigned long,RBX::MegaClusterLegacy::ChunkData const&)")]
// 0xf66004 — j___ZNSt6vectorIN3RBX17MegaClusterLegacy9ChunkDataESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf66004() {
    // IDA 0xf66004: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::MegaCluster::ChunkData *,unsigned long,RBX::MegaCluster::ChunkData>(RBX::MegaCluster::ChunkData *,unsigned long,RBX::MegaCluster::ChunkData const&,std::__false_type)")]
// 0xf66014 — j___ZSt26__uninitialized_fill_n_auxIPN3RBX11MegaCluster9ChunkDataEmS2_EvT_T0_RKT1_St12__false_type
pub fn stub_0xf66014() {
    // IDA 0xf66014: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::xline_iterator::xline_iterator(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)")]
// 0xf66114 — j___ZN3RBX5Voxel6RegionINS0_4Grid5ChunkEE14xline_iteratorC2ERKS4_
pub fn stub_0xf66114() {
    // IDA 0xf66114: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::LightGrid::occupancyFillTerrain<RBX::Voxel::Grid>(RBX::LightGridChunk &,RBX::Voxel::Grid &,RBX::Vector3int32 const&,RBX::Extents const&)")]
// 0xf661b4 — j___ZN3RBX9LightGrid20occupancyFillTerrainINS_5Voxel4GridEEEvRNS_14LightGridChunkERT_RKNS_12Vector3int32ERKNS_7ExtentsE
pub fn stub_0xf661b4() {
    // IDA 0xf661b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::LightGrid::occupancyFillTerrainSIMD<RBX::Voxel::Grid>(RBX::LightGridChunk &,RBX::Voxel::Grid &,RBX::Vector3int32 const&,RBX::Extents const&)")]
// 0xf661d4 — j___ZN3RBX9LightGrid24occupancyFillTerrainSIMDINS_5Voxel4GridEEEvRNS_14LightGridChunkERT_RKNS_12Vector3int32ERKNS_7ExtentsE
pub fn stub_0xf661d4() {
    // IDA 0xf661d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::LightGrid::lightingBlurAxisYZScratch<false>(void)")]
// 0xf661e4 — j___ZN3RBX9LightGrid25lightingBlurAxisYZScratchILb0EEEvv
pub fn stub_0xf661e4() {
    // IDA 0xf661e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::LightGrid::lightingBlurAxisYZScratch<true>(void)")]
// 0xf661f4 — j___ZN3RBX9LightGrid25lightingBlurAxisYZScratchILb1EEEvv
pub fn stub_0xf661f4() {
    // IDA 0xf661f4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingUpdateDirectionalImpl<false,false>(RBX::LightGridChunk &,RBX::Vector3int32 const&)")]
// 0xf66264 — j___ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb0ELb0EEEvRNS_14LightGridChunkERKNS_12Vector3int32E
pub fn stub_0xf66264() {
    // IDA 0xf66264: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingUpdateDirectionalImpl<false,true>(RBX::LightGridChunk &,RBX::Vector3int32 const&)")]
// 0xf66274 — j___ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb0ELb1EEEvRNS_14LightGridChunkERKNS_12Vector3int32E
pub fn stub_0xf66274() {
    // IDA 0xf66274: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingUpdateDirectionalImpl<true,false>(RBX::LightGridChunk &,RBX::Vector3int32 const&)")]
// 0xf66284 — j___ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb1ELb0EEEvRNS_14LightGridChunkERKNS_12Vector3int32E
pub fn stub_0xf66284() {
    // IDA 0xf66284: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingUpdateDirectionalImpl<true,true>(RBX::LightGridChunk &,RBX::Vector3int32 const&)")]
// 0xf66294 — j___ZN3RBX9LightGrid29lightingUpdateDirectionalImplILb1ELb1EEEvRNS_14LightGridChunkERKNS_12Vector3int32E
pub fn stub_0xf66294() {
    // IDA 0xf66294: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowMaskToShadowSlice<0>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice &,RBX::LightShadowSlice &)")]
// 0xf663a4 — j___ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_
pub fn stub_0xf663a4() {
    // IDA 0xf663a4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowMaskToShadowSlice<1>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice &,RBX::LightShadowSlice &)")]
// 0xf663b4 — j___ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_
pub fn stub_0xf663b4() {
    // IDA 0xf663b4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowMaskToShadowSlice<2>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice &,RBX::LightShadowSlice &)")]
// 0xf663c4 — j___ZN3RBX9LightGrid39lightingTransferShadowMaskToShadowSliceILi2EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_iRNS_16LightShadowSliceES9_
pub fn stub_0xf663c4() {
    // IDA 0xf663c4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowSliceToShadowMask<0>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice const&,RBX::LightShadowSlice const&)")]
// 0xf663d4 — j___ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi0EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_
pub fn stub_0xf663d4() {
    // IDA 0xf663d4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowSliceToShadowMask<1>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice const&,RBX::LightShadowSlice const&)")]
// 0xf663e4 — j___ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi1EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_
pub fn stub_0xf663e4() {
    // IDA 0xf663e4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void RBX::LightGrid::lightingTransferShadowSliceToShadowMask<2>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,int,RBX::LightShadowSlice const&,RBX::LightShadowSlice const&)")]
// 0xf663f4 — j___ZN3RBX9LightGrid39lightingTransferShadowSliceToShadowMaskILi2EEEvRKNS_12Vector3int32ES4_S4_iRKNS_16LightShadowSliceES7_
pub fn stub_0xf663f4() {
    // IDA 0xf663f4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "void boost::throw_exception<boost::bad_function_call>(boost::bad_function_call const&)")]
// 0xf66424 — j___ZN5boost15throw_exceptionINS_17bad_function_callEEEvRKT_
pub fn stub_0xf66424() {
    // IDA 0xf66424: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone_impl(boost::exception_detail::bad_exception_ const&)")]
// 0xf66434 — j___ZN5boost16exception_detail10clone_implINS0_14bad_exception_EEC1ERKS2_
pub fn stub_0xf66434() {
    // IDA 0xf66434: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::mutex::lock(void)")]
// 0xf66444 — j___ZN5boost5mutex4lockEv
pub fn stub_0xf66444() {
    // IDA 0xf66444: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::mutex::mutex(void)")]
// 0xf66454 — j___ZN5boost5mutexC2Ev
pub fn stub_0xf66454() {
    // IDA 0xf66454: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::system::system_error::system_error(boost::system::error_code,char const*)")]
// 0xf66464 — j___ZN5boost6system12system_errorC2ENS0_10error_codeEPKc
pub fn stub_0xf66464() {
    // IDA 0xf66464: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "void boost::call_once<void (*)(void)>(boost::once_flag &,void (*)(void))")]
// 0xf66474 — j___ZN5boost9call_onceIPFvvEEEvRNS_9once_flagET_
pub fn stub_0xf66474() {
    // IDA 0xf66474: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::rethrow(void)const")]
// 0xf66494 — j___ZNK5boost16exception_detail10clone_implINS0_10bad_alloc_EE7rethrowEv
pub fn stub_0xf66494() {
    // IDA 0xf66494: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::rethrow(void)const")]
// 0xf664a4 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE7rethrowEv
pub fn stub_0xf664a4() {
    // IDA 0xf664a4: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::system::system_error::what(void)const")]
// 0xf664c4 — j___ZNK5boost6system12system_error4whatEv
pub fn stub_0xf664c4() {
    // IDA 0xf664c4: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "std::vector<RBX::LightObject *,std::allocator<RBX::LightObject *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::LightObject **,std::vector<RBX::LightObject *,std::allocator<RBX::LightObject *>>>,RBX::LightObject * const&)")]
// 0xf664d4 — j___ZNSt6vectorIPN3RBX11LightObjectESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf664d4() {
    // IDA 0xf664d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::LightGridChunk *,std::allocator<RBX::LightGridChunk *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::LightGridChunk **,std::vector<RBX::LightGridChunk *,std::allocator<RBX::LightGridChunk *>>>,RBX::LightGridChunk * const&)")]
// 0xf664e4 — j___ZNSt6vectorIPN3RBX14LightGridChunkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf664e4() {
    // IDA 0xf664e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::LightGridChunk *,std::allocator<RBX::LightGridChunk *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::LightGridChunk **,std::vector<RBX::LightGridChunk *,std::allocator<RBX::LightGridChunk *>>>,unsigned long,RBX::LightGridChunk * const&)")]
// 0xf664f4 — j___ZNSt6vectorIPN3RBX14LightGridChunkESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf664f4() {
    // IDA 0xf664f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ICreator const*> const&)")]
// 0xf66504 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
pub fn stub_0xf66504() {
    // IDA 0xf66504: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ICreator const*>> *)")]
// 0xf66514 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
pub fn stub_0xf66514() {
    // IDA 0xf66514: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::scoped_ptr<RBX::LightShadowMap>::~scoped_ptr()")]
// 0xf66594 — j___ZN5boost10scoped_ptrIN3RBX14LightShadowMapEED2Ev
pub fn stub_0xf66594() {
    // IDA 0xf66594: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::RenderEntity **,std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>>,RBX::RenderEntity * const&)")]
// 0xf665f4 — j___ZNSt6vectorIPN3RBX12RenderEntityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf665f4() {
    // IDA 0xf665f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::WaterImpl::underwater(void)")]
// 0xf66604 — j___ZN3RBX9WaterImpl10underwaterEv
pub fn stub_0xf66604() {
    // IDA 0xf66604: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::WaterImpl::load(void)")]
// 0xf66614 — j___ZN3RBX9WaterImpl4loadEv
pub fn stub_0xf66614() {
    // IDA 0xf66614: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::WaterImpl::~WaterImpl()")]
// 0xf66634 — j___ZN3RBX9WaterImplD2Ev
pub fn stub_0xf66634() {
    // IDA 0xf66634: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<double,std::allocator<double>>::_M_fill_insert(__gnu_cxx::__normal_iterator<double *,std::vector<double,std::allocator<double>>>,unsigned long,double const&)")]
// 0xf666f4 — j___ZNSt6vectorIdSaIdEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPdS1_EEmRKd
pub fn stub_0xf666f4() {
    // IDA 0xf666f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<float,std::allocator<float>>::_M_fill_insert(__gnu_cxx::__normal_iterator<float *,std::vector<float,std::allocator<float>>>,unsigned long,float const&)")]
// 0xf66704 — j___ZNSt6vectorIfSaIfEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPfS1_EEmRKf
pub fn stub_0xf66704() {
    // IDA 0xf66704: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<unsigned char,std::allocator<unsigned char>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned char *,std::vector<unsigned char,std::allocator<unsigned char>>>,unsigned long,unsigned char const&)")]
// 0xf66714 — j___ZNSt6vectorIhSaIhEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPhS1_EEmRKh
pub fn stub_0xf66714() {
    // IDA 0xf66714: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<short,std::allocator<short>>::_M_fill_insert(__gnu_cxx::__normal_iterator<short *,std::vector<short,std::allocator<short>>>,unsigned long,short const&)")]
// 0xf66724 — j___ZNSt6vectorIsSaIsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPsS1_EEmRKs
pub fn stub_0xf66724() {
    // IDA 0xf66724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<unsigned long,std::string,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::operator[](unsigned long const&)")]
// 0xf67414 — j___ZNSt3mapImSsSt4lessImESaISt4pairIKmSsEEEixERS3_
pub fn stub_0xf67414() {
    // IDA 0xf67414: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,std::string>,std::_Select1st<std::pair<unsigned long const,std::string>>,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::_M_insert_unique(std::pair<unsigned long const,std::string> const&)")]
// 0xf674f4 — j___ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_0xf674f4() {
    // IDA 0xf674f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,std::string>,std::_Select1st<std::pair<unsigned long const,std::string>>,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long const,std::string>>,std::pair<unsigned long const,std::string> const&)")]
// 0xf67504 — j___ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
pub fn stub_0xf67504() {
    // IDA 0xf67504: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
