//! core bg12 — 100 core stubs EA-sorted asc distinct not in /tmp/global_eas.txt.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua, exclude boost) global distinct not yet in /tmp/global_eas.txt — next 100 uncovered after 0xbc1d7c (prior max 0xbc1d7c) -> 0x5c01b0..0xead7e8.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ContentId>(RBX::ContentId const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9ContentIdEEERS3_RKT_")]
// 0x5c01b0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9ContentIdEEERS3_RKT_
// type: int(void)
pub fn stub_0x5c01b0() -> ! {
    todo!("0x5c01b0 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9ContentIdEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ContentId>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE9singletonEv")]
// 0x5c0210 — __ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE9singletonEv
// type: int(void)
pub fn stub_0x5c0210() -> ! {
    todo!("0x5c0210 __ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ContentId>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE13destruct_funcEPc")]
// 0x5c0280 — __ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE13destruct_funcEPc
// type: 
pub fn stub_0x5c0280() -> ! {
    todo!("0x5c0280 __ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE13destruct_funcEPc")
}

#[doc(alias = "std::_Deque_base<XmlElement *,std::allocator<XmlElement *>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIP10XmlElementSaIS1_EED2Ev")]
// 0x5c0470 — __ZNSt11_Deque_baseIP10XmlElementSaIS1_EED2Ev
// type: int __fastcall(_DWORD)
pub fn stub_0x5c0470() -> ! {
    todo!("0x5c0470 __ZNSt11_Deque_baseIP10XmlElementSaIS1_EED2Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ICreator const*>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")]
// 0xc2d550 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: 
pub fn stub_0xc2d550() -> ! {
    todo!("0xc2d550 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ICreator const*> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_")]
// 0xc2d580 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: 
pub fn stub_0xc2d580() -> ! {
    todo!("0xc2d580 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_")
}

#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
#[doc(alias = "__ZThn32_N3RBX9BlockMeshD1Ev")]
// 0xc2d7f0 — __ZThn32_N3RBX9BlockMeshD1Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
pub fn stub_0xc2d7f0() -> ! {
    todo!("0xc2d7f0 __ZThn32_N3RBX9BlockMeshD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
#[doc(alias = "__ZThn36_N3RBX9BlockMeshD1Ev")]
// 0xc2d800 — __ZThn36_N3RBX9BlockMeshD1Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
pub fn stub_0xc2d800() -> ! {
    todo!("0xc2d800 __ZThn36_N3RBX9BlockMeshD1Ev")
}

#[doc(alias = "std::vector<RBX::LightGridChunk *,std::allocator<RBX::LightGridChunk *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::LightGridChunk **,std::vector<RBX::LightGridChunk *,std::allocator<RBX::LightGridChunk *>>>,RBX::LightGridChunk * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX14LightGridChunkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xc2e058 — __ZNSt6vectorIPN3RBX14LightGridChunkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xc2e058() -> ! {
    todo!("0xc2e058 __ZNSt6vectorIPN3RBX14LightGridChunkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::LightObject *,std::allocator<RBX::LightObject *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::LightObject **,std::vector<RBX::LightObject *,std::allocator<RBX::LightObject *>>>,RBX::LightObject * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX11LightObjectESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xc2f8c4 — __ZNSt6vectorIPN3RBX11LightObjectESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xc2f8c4() -> ! {
    todo!("0xc2f8c4 __ZNSt6vectorIPN3RBX11LightObjectESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::xline_iterator::xline_iterator(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)")]
#[doc(alias = "__ZN3RBX5Voxel6RegionINS0_4Grid5ChunkEE14xline_iteratorC2ERKS4_")]
// 0xc30224 — __ZN3RBX5Voxel6RegionINS0_4Grid5ChunkEE14xline_iteratorC2ERKS4_
// type: 
pub fn stub_0xc30224() -> ! {
    todo!("0xc30224 __ZN3RBX5Voxel6RegionINS0_4Grid5ChunkEE14xline_iteratorC2ERKS4_")
}

#[doc(alias = "std::vector<RBX::LightGridChunk *,std::allocator<RBX::LightGridChunk *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::LightGridChunk **,std::vector<RBX::LightGridChunk *,std::allocator<RBX::LightGridChunk *>>>,unsigned long,RBX::LightGridChunk * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX14LightGridChunkESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xc30328 — __ZNSt6vectorIPN3RBX14LightGridChunkESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xc30328() -> ! {
    todo!("0xc30328 __ZNSt6vectorIPN3RBX14LightGridChunkESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::LightObject::~LightObject()")]
#[doc(alias = "__ZN3RBX11LightObjectD0Ev")]
// 0xc31b6c — __ZN3RBX11LightObjectD0Ev
// type: void __fastcall(RBX::LightObject *__hidden this)
pub fn stub_0xc31b6c() -> ! {
    todo!("0xc31b6c __ZN3RBX11LightObjectD0Ev")
}

#[doc(alias = "RBX::LightObject::~LightObject()")]
#[doc(alias = "__ZN3RBX11LightObjectD1Ev")]
// 0xc31c20 — __ZN3RBX11LightObjectD1Ev
// type: void __fastcall(RBX::LightObject *__hidden this)
pub fn stub_0xc31c20() -> ! {
    todo!("0xc31c20 __ZN3RBX11LightObjectD1Ev")
}

#[doc(alias = "RBX::LightObject::~LightObject()")]
#[doc(alias = "__ZN3RBX11LightObjectD2Ev")]
// 0xc31c24 — __ZN3RBX11LightObjectD2Ev
// type: void __fastcall(RBX::LightObject *__hidden this)
pub fn stub_0xc31c24() -> ! {
    todo!("0xc31c24 __ZN3RBX11LightObjectD2Ev")
}

#[doc(alias = "RBX::LightObject::onSleepingChangedEx(bool)")]
#[doc(alias = "__ZN3RBX11LightObject19onSleepingChangedExEb")]
// 0xc31dbc — __ZN3RBX11LightObject19onSleepingChangedExEb
// type: _DWORD __fastcall(RBX::LightObject *__hidden this, bool)
pub fn stub_0xc31dbc() -> ! {
    todo!("0xc31dbc __ZN3RBX11LightObject19onSleepingChangedExEb")
}

#[doc(alias = "RBX::LightObject::updateCoordinateFrame(bool)")]
#[doc(alias = "__ZN3RBX11LightObject21updateCoordinateFrameEb")]
// 0xc31ee0 — __ZN3RBX11LightObject21updateCoordinateFrameEb
// type: _DWORD __fastcall(RBX::LightObject *__hidden this, bool)
pub fn stub_0xc31ee0() -> ! {
    todo!("0xc31ee0 __ZN3RBX11LightObject21updateCoordinateFrameEb")
}

#[doc(alias = "RBX::LightObject::getLightPosition(void)const")]
#[doc(alias = "__ZNK3RBX11LightObject16getLightPositionEv")]
// 0xc3281c — __ZNK3RBX11LightObject16getLightPositionEv
// type: _DWORD __fastcall(RBX::LightObject *__hidden this)
pub fn stub_0xc3281c() -> ! {
    todo!("0xc3281c __ZNK3RBX11LightObject16getLightPositionEv")
}

#[doc(alias = "RBX::LightObject::getLightDirection(void)const")]
#[doc(alias = "__ZNK3RBX11LightObject17getLightDirectionEv")]
// 0xc3282c — __ZNK3RBX11LightObject17getLightDirectionEv
// type: int __fastcall(int this, int)
pub fn stub_0xc3282c() -> ! {
    todo!("0xc3282c __ZNK3RBX11LightObject17getLightDirectionEv")
}

#[doc(alias = "non-virtual thunk toRBX::LightObject::updateCoordinateFrame(bool)")]
#[doc(alias = "__ZThn392_N3RBX11LightObject21updateCoordinateFrameEb")]
// 0xc3283c — __ZThn392_N3RBX11LightObject21updateCoordinateFrameEb
// type: _DWORD __fastcall(RBX::LightObject *__hidden this, bool)
pub fn stub_0xc3283c() -> ! {
    todo!("0xc3283c __ZThn392_N3RBX11LightObject21updateCoordinateFrameEb")
}

#[doc(alias = "RBX::LightObject::onAncestorChangedEx(void)")]
#[doc(alias = "__ZN3RBX11LightObject19onAncestorChangedExEv")]
// 0xc32860 — __ZN3RBX11LightObject19onAncestorChangedExEv
// type: _DWORD __fastcall(RBX::LightObject *__hidden this)
pub fn stub_0xc32860() -> ! {
    todo!("0xc32860 __ZN3RBX11LightObject19onAncestorChangedExEv")
}

#[doc(alias = "RBX::LightObject::unbind(void)")]
#[doc(alias = "__ZN3RBX11LightObject6unbindEv")]
// 0xc331b4 — __ZN3RBX11LightObject6unbindEv
// type: _DWORD __fastcall(RBX::LightObject *__hidden this)
pub fn stub_0xc331b4() -> ! {
    todo!("0xc331b4 __ZN3RBX11LightObject6unbindEv")
}

#[doc(alias = "non-virtual thunk toRBX::LightObject::unbind(void)")]
#[doc(alias = "__ZThn392_N3RBX11LightObject6unbindEv")]
// 0xc331f4 — __ZThn392_N3RBX11LightObject6unbindEv
// type: _DWORD __fastcall(RBX::LightObject *__hidden this)
pub fn stub_0xc331f4() -> ! {
    todo!("0xc331f4 __ZThn392_N3RBX11LightObject6unbindEv")
}

#[doc(alias = "RBX::LightObject::invalidateEntity(void)")]
#[doc(alias = "__ZN3RBX11LightObject16invalidateEntityEv")]
// 0xc3322c — __ZN3RBX11LightObject16invalidateEntityEv
// type: _DWORD __fastcall(RBX::LightObject *__hidden this)
pub fn stub_0xc3322c() -> ! {
    todo!("0xc3322c __ZN3RBX11LightObject16invalidateEntityEv")
}

#[doc(alias = "non-virtual thunk toRBX::LightObject::invalidateEntity(void)")]
#[doc(alias = "__ZThn392_N3RBX11LightObject16invalidateEntityEv")]
// 0xc3325c — __ZThn392_N3RBX11LightObject16invalidateEntityEv
// type: _DWORD __fastcall(RBX::LightObject *__hidden this)
pub fn stub_0xc3325c() -> ! {
    todo!("0xc3325c __ZThn392_N3RBX11LightObject16invalidateEntityEv")
}

#[doc(alias = "RBX::LightObject::updateEntity(bool)")]
#[doc(alias = "__ZN3RBX11LightObject12updateEntityEb")]
// 0xc33290 — __ZN3RBX11LightObject12updateEntityEb
// type: _DWORD __fastcall(RBX::LightObject *__hidden this, bool)
pub fn stub_0xc33290() -> ! {
    todo!("0xc33290 __ZN3RBX11LightObject12updateEntityEb")
}

#[doc(alias = "non-virtual thunk toRBX::LightObject::updateEntity(bool)")]
#[doc(alias = "__ZThn392_N3RBX11LightObject12updateEntityEb")]
// 0xc33680 — __ZThn392_N3RBX11LightObject12updateEntityEb
// type: _DWORD __fastcall(RBX::LightObject *__hidden this, bool)
pub fn stub_0xc33680() -> ! {
    todo!("0xc33680 __ZThn392_N3RBX11LightObject12updateEntityEb")
}

#[doc(alias = "RBX::LightObject::_updateBounds(void)")]
#[doc(alias = "__ZN3RBX11LightObject13_updateBoundsEv")]
// 0xc3368c — __ZN3RBX11LightObject13_updateBoundsEv
// type: _DWORD __fastcall(RBX::LightObject *__hidden this)
pub fn stub_0xc3368c() -> ! {
    todo!("0xc3368c __ZN3RBX11LightObject13_updateBoundsEv")
}

#[doc(alias = "RBX::LightObject::getLightExtents(void)const")]
#[doc(alias = "__ZNK3RBX11LightObject15getLightExtentsEv")]
// 0xc33690 — __ZNK3RBX11LightObject15getLightExtentsEv
// type: _DWORD __fastcall(RBX::LightObject *__hidden this)
pub fn stub_0xc33690() -> ! {
    todo!("0xc33690 __ZNK3RBX11LightObject15getLightExtentsEv")
}

#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
#[doc(alias = "__ZN3RBX12RenderEntityD0Ev")]
// 0xc35418 — __ZN3RBX12RenderEntityD0Ev
// type: void __fastcall(RBX::RenderEntity *__hidden this)
pub fn stub_0xc35418() -> ! {
    todo!("0xc35418 __ZN3RBX12RenderEntityD0Ev")
}

#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
#[doc(alias = "__ZN3RBX12RenderEntityD1Ev")]
// 0xc354b8 — __ZN3RBX12RenderEntityD1Ev
// type: void __fastcall(RBX::RenderEntity *__hidden this)
pub fn stub_0xc354b8() -> ! {
    todo!("0xc354b8 __ZN3RBX12RenderEntityD1Ev")
}

#[doc(alias = "RBX::RenderEntity::~RenderEntity()")]
#[doc(alias = "__ZN3RBX12RenderEntityD2Ev")]
// 0xc354bc — __ZN3RBX12RenderEntityD2Ev
// type: void __fastcall(RBX::RenderEntity *__hidden this)
pub fn stub_0xc354bc() -> ! {
    todo!("0xc354bc __ZN3RBX12RenderEntityD2Ev")
}

#[doc(alias = "RBX::RenderEntity::getActualMaterial(void)const")]
#[doc(alias = "__ZNK3RBX12RenderEntity17getActualMaterialEv")]
// 0xc35838 — __ZNK3RBX12RenderEntity17getActualMaterialEv
// type: _DWORD __fastcall(RBX::RenderEntity *__hidden this)
pub fn stub_0xc35838() -> ! {
    todo!("0xc35838 __ZNK3RBX12RenderEntity17getActualMaterialEv")
}

#[doc(alias = "RBX::RenderEntity::getMaterial(void)const")]
#[doc(alias = "__ZNK3RBX12RenderEntity11getMaterialEv")]
// 0xc35980 — __ZNK3RBX12RenderEntity11getMaterialEv
// type: _DWORD __fastcall(RBX::RenderEntity *__hidden this)
pub fn stub_0xc35980() -> ! {
    todo!("0xc35980 __ZNK3RBX12RenderEntity11getMaterialEv")
}

#[doc(alias = "RBX::RenderEntity::getTechnique(void)const")]
#[doc(alias = "__ZNK3RBX12RenderEntity12getTechniqueEv")]
// 0xc359e8 — __ZNK3RBX12RenderEntity12getTechniqueEv
// type: _DWORD __fastcall(RBX::RenderEntity *__hidden this)
pub fn stub_0xc359e8() -> ! {
    todo!("0xc359e8 __ZNK3RBX12RenderEntity12getTechniqueEv")
}

#[doc(alias = "RBX::RenderEntity::getDebugMaterial(void)const")]
#[doc(alias = "__ZNK3RBX12RenderEntity16getDebugMaterialEv")]
// 0xc359ec — __ZNK3RBX12RenderEntity16getDebugMaterialEv
// type: _DWORD __fastcall(RBX::RenderEntity *__hidden this)
pub fn stub_0xc359ec() -> ! {
    todo!("0xc359ec __ZNK3RBX12RenderEntity16getDebugMaterialEv")
}

#[doc(alias = "RBX::RenderEntity::getLights(void)const")]
#[doc(alias = "__ZNK3RBX12RenderEntity9getLightsEv")]
// 0xc35aa8 — __ZNK3RBX12RenderEntity9getLightsEv
// type: _DWORD __fastcall(RBX::RenderEntity *__hidden this)
pub fn stub_0xc35aa8() -> ! {
    todo!("0xc35aa8 __ZNK3RBX12RenderEntity9getLightsEv")
}

#[doc(alias = "RBX::RenderNode::~RenderNode()")]
#[doc(alias = "__ZN3RBX10RenderNodeD0Ev")]
// 0xc35b20 — __ZN3RBX10RenderNodeD0Ev
// type: void __fastcall(RBX::RenderNode *__hidden this)
pub fn stub_0xc35b20() -> ! {
    todo!("0xc35b20 __ZN3RBX10RenderNodeD0Ev")
}

#[doc(alias = "RBX::RenderNode::~RenderNode()")]
#[doc(alias = "__ZN3RBX10RenderNodeD1Ev")]
// 0xc35bd4 — __ZN3RBX10RenderNodeD1Ev
// type: void __fastcall(RBX::RenderNode *__hidden this)
pub fn stub_0xc35bd4() -> ! {
    todo!("0xc35bd4 __ZN3RBX10RenderNodeD1Ev")
}

#[doc(alias = "RBX::RenderNode::~RenderNode()")]
#[doc(alias = "__ZN3RBX10RenderNodeD2Ev")]
// 0xc35bd8 — __ZN3RBX10RenderNodeD2Ev
// type: void __fastcall(RBX::RenderNode *__hidden this)
pub fn stub_0xc35bd8() -> ! {
    todo!("0xc35bd8 __ZN3RBX10RenderNodeD2Ev")
}

#[doc(alias = "RBX::RenderNode::addEntity(RBX::RenderEntity *)")]
#[doc(alias = "__ZN3RBX10RenderNode9addEntityEPNS_12RenderEntityE")]
// 0xc35d9c — __ZN3RBX10RenderNode9addEntityEPNS_12RenderEntityE
// type: int __fastcall(RBX::RenderNode *this, RBX::RenderEntity *)
pub fn stub_0xc35d9c() -> ! {
    todo!("0xc35d9c __ZN3RBX10RenderNode9addEntityEPNS_12RenderEntityE")
}

#[doc(alias = "RBX::RenderNode::removeEntity(RBX::RenderEntity *)")]
#[doc(alias = "__ZN3RBX10RenderNode12removeEntityEPNS_12RenderEntityE")]
// 0xc35e2c — __ZN3RBX10RenderNode12removeEntityEPNS_12RenderEntityE
// type: _DWORD __fastcall(RBX::RenderNode *__hidden this, RBX::RenderEntity *)
pub fn stub_0xc35e2c() -> ! {
    todo!("0xc35e2c __ZN3RBX10RenderNode12removeEntityEPNS_12RenderEntityE")
}

#[doc(alias = "RBX::RenderNode::getFastFuzzyExtents(void)")]
#[doc(alias = "__ZN3RBX10RenderNode19getFastFuzzyExtentsEv")]
// 0xc35f64 — __ZN3RBX10RenderNode19getFastFuzzyExtentsEv
// type: _DWORD __fastcall(RBX::RenderNode *__hidden this)
pub fn stub_0xc35f64() -> ! {
    todo!("0xc35f64 __ZN3RBX10RenderNode19getFastFuzzyExtentsEv")
}

#[doc(alias = "non-virtual thunk toRBX::RenderNode::getFastFuzzyExtents(void)")]
#[doc(alias = "__ZThn392_N3RBX10RenderNode19getFastFuzzyExtentsEv")]
// 0xc35f74 — __ZThn392_N3RBX10RenderNode19getFastFuzzyExtentsEv
// type: _DWORD __fastcall(RBX::RenderNode *__hidden this)
pub fn stub_0xc35f74() -> ! {
    todo!("0xc35f74 __ZThn392_N3RBX10RenderNode19getFastFuzzyExtentsEv")
}

#[doc(alias = "RBX::RenderNode::_updateBounds(void)")]
#[doc(alias = "__ZN3RBX10RenderNode13_updateBoundsEv")]
// 0xc35f80 — __ZN3RBX10RenderNode13_updateBoundsEv
// type: _DWORD __fastcall(RBX::RenderNode *__hidden this)
pub fn stub_0xc35f80() -> ! {
    todo!("0xc35f80 __ZN3RBX10RenderNode13_updateBoundsEv")
}

#[doc(alias = "std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::RenderEntity **,std::vector<RBX::RenderEntity *,std::allocator<RBX::RenderEntity *>>>,RBX::RenderEntity * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX12RenderEntityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xc3602c — __ZNSt6vectorIPN3RBX12RenderEntityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xc3602c() -> ! {
    todo!("0xc3602c __ZNSt6vectorIPN3RBX12RenderEntityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "RBX::WaterImpl::~WaterImpl()")]
#[doc(alias = "__ZN3RBX9WaterImplD1Ev")]
// 0xc36a48 — __ZN3RBX9WaterImplD1Ev
// type: void __fastcall(RBX::WaterImpl *__hidden this)
pub fn stub_0xc36a48() -> ! {
    todo!("0xc36a48 __ZN3RBX9WaterImplD1Ev")
}

#[doc(alias = "RBX::WaterImpl::~WaterImpl()")]
#[doc(alias = "__ZN3RBX9WaterImplD0Ev")]
// 0xc36a4c — __ZN3RBX9WaterImplD0Ev
// type: void __fastcall(RBX::WaterImpl *__hidden this)
pub fn stub_0xc36a4c() -> ! {
    todo!("0xc36a4c __ZN3RBX9WaterImplD0Ev")
}

#[doc(alias = "RBX::WaterImpl::activate(void)")]
#[doc(alias = "__ZN3RBX9WaterImpl8activateEv")]
// 0xc36af4 — __ZN3RBX9WaterImpl8activateEv
// type: _DWORD __fastcall(RBX::WaterImpl *__hidden this)
pub fn stub_0xc36af4() -> ! {
    todo!("0xc36af4 __ZN3RBX9WaterImpl8activateEv")
}

#[doc(alias = "RBX::WaterImpl::update(void)")]
#[doc(alias = "__ZN3RBX9WaterImpl6updateEv")]
// 0xc36b10 — __ZN3RBX9WaterImpl6updateEv
// type: _DWORD __fastcall(RBX::WaterImpl *__hidden this)
pub fn stub_0xc36b10() -> ! {
    todo!("0xc36b10 __ZN3RBX9WaterImpl6updateEv")
}

#[doc(alias = "RBX::WaterImpl::underwater(void)")]
#[doc(alias = "__ZN3RBX9WaterImpl10underwaterEv")]
// 0xc37310 — __ZN3RBX9WaterImpl10underwaterEv
// type: _DWORD __fastcall(RBX::WaterImpl *__hidden this)
pub fn stub_0xc37310() -> ! {
    todo!("0xc37310 __ZN3RBX9WaterImpl10underwaterEv")
}

#[doc(alias = "RBX::WaterImpl::load(void)")]
#[doc(alias = "__ZN3RBX9WaterImpl4loadEv")]
// 0xc37cb0 — __ZN3RBX9WaterImpl4loadEv
// type: _DWORD __fastcall(RBX::WaterImpl *__hidden this)
pub fn stub_0xc37cb0() -> ! {
    todo!("0xc37cb0 __ZN3RBX9WaterImpl4loadEv")
}

#[doc(alias = "RBX::WaterImpl::~WaterImpl()")]
#[doc(alias = "__ZN3RBX9WaterImplD2Ev")]
// 0xc38384 — __ZN3RBX9WaterImplD2Ev
// type: void __fastcall(RBX::WaterImpl *__hidden this)
pub fn stub_0xc38384() -> ! {
    todo!("0xc38384 __ZN3RBX9WaterImplD2Ev")
}

#[doc(alias = "std::vector<double,std::allocator<double>>::_M_fill_insert(__gnu_cxx::__normal_iterator<double *,std::vector<double,std::allocator<double>>>,unsigned long,double const&)")]
#[doc(alias = "__ZNSt6vectorIdSaIdEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPdS1_EEmRKd")]
// 0xc39e78 — __ZNSt6vectorIdSaIdEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPdS1_EEmRKd
// type: int __fastcall(int, void *__src)
pub fn stub_0xc39e78() -> ! {
    todo!("0xc39e78 __ZNSt6vectorIdSaIdEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPdS1_EEmRKd")
}

#[doc(alias = "std::vector<float,std::allocator<float>>::_M_fill_insert(__gnu_cxx::__normal_iterator<float *,std::vector<float,std::allocator<float>>>,unsigned long,float const&)")]
#[doc(alias = "__ZNSt6vectorIfSaIfEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPfS1_EEmRKf")]
// 0xc39fec — __ZNSt6vectorIfSaIfEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPfS1_EEmRKf
// type: int __fastcall(int, void *__src)
pub fn stub_0xc39fec() -> ! {
    todo!("0xc39fec __ZNSt6vectorIfSaIfEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPfS1_EEmRKf")
}

#[doc(alias = "std::vector<short,std::allocator<short>>::_M_fill_insert(__gnu_cxx::__normal_iterator<short *,std::vector<short,std::allocator<short>>>,unsigned long,short const&)")]
#[doc(alias = "__ZNSt6vectorIsSaIsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPsS1_EEmRKs")]
// 0xc3a164 — __ZNSt6vectorIsSaIsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPsS1_EEmRKs
// type: int __fastcall(int, void *__src)
pub fn stub_0xc3a164() -> ! {
    todo!("0xc3a164 __ZNSt6vectorIsSaIsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPsS1_EEmRKs")
}

#[doc(alias = "std::vector<unsigned char,std::allocator<unsigned char>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned char *,std::vector<unsigned char,std::allocator<unsigned char>>>,unsigned long,unsigned char const&)")]
#[doc(alias = "__ZNSt6vectorIhSaIhEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPhS1_EEmRKh")]
// 0xc3a2bc — __ZNSt6vectorIhSaIhEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPhS1_EEmRKh
// type: int __fastcall(int, int, size_t __len)
pub fn stub_0xc3a2bc() -> ! {
    todo!("0xc3a2bc __ZNSt6vectorIhSaIhEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPhS1_EEmRKh")
}

#[doc(alias = "std::map<unsigned long,std::string,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::operator[](unsigned long const&)")]
#[doc(alias = "__ZNSt3mapImSsSt4lessImESaISt4pairIKmSsEEEixERS3_")]
// 0xcaa994 — __ZNSt3mapImSsSt4lessImESaISt4pairIKmSsEEEixERS3_
// type: 
pub fn stub_0xcaa994() -> ! {
    todo!("0xcaa994 __ZNSt3mapImSsSt4lessImESaISt4pairIKmSsEEEixERS3_")
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,std::string>,std::_Select1st<std::pair<unsigned long const,std::string>>,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long const,std::string>>,std::pair<unsigned long const,std::string> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")]
// 0xcaac64 — __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xcaac64() -> ! {
    todo!("0xcaac64 __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,std::string>,std::_Select1st<std::pair<unsigned long const,std::string>>,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned long const,std::string> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// 0xcaad18 — __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
pub fn stub_0xcaad18() -> ! {
    todo!("0xcaad18 __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,std::string>,std::_Select1st<std::pair<unsigned long const,std::string>>,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::_M_insert_unique(std::pair<unsigned long const,std::string> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE16_M_insert_uniqueERKS2_")]
// 0xcaae44 — __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE16_M_insert_uniqueERKS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xcaae44() -> ! {
    todo!("0xcaae44 __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,std::string>,std::_Select1st<std::pair<unsigned long const,std::string>>,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,std::string>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xcabfdc — __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: 
pub fn stub_0xcabfdc() -> ! {
    todo!("0xcabfdc __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>>(std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_")]
// 0xd9950c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_
// type: 
pub fn stub_0xd9950c() -> ! {
    todo!("0xd9950c __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *> std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>>(std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_")]
// 0xd995b0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_
// type: 
pub fn stub_0xd995b0() -> ! {
    todo!("0xd995b0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::basic_string<unsigned short,std::char_traits<unsigned short>,std::allocator<unsigned short>>::append(unsigned short const*,unsigned long)")]
#[doc(alias = "__ZNSbItSt11char_traitsItESaItEE6appendEPKtm")]
// 0xe51b14 — __ZNSbItSt11char_traitsItESaItEE6appendEPKtm
// type: unsigned int *__fastcall(unsigned int *, _WORD *__src, unsigned int)
pub fn stub_0xe51b14() -> ! {
    todo!("0xe51b14 __ZNSbItSt11char_traitsItESaItEE6appendEPKtm")
}

#[doc(alias = "std::basic_string<unsigned short,std::char_traits<unsigned short>,std::allocator<unsigned short>>::_M_mutate(unsigned long,unsigned long,unsigned long)")]
#[doc(alias = "__ZNSbItSt11char_traitsItESaItEE9_M_mutateEmmm")]
// 0xe51c90 — __ZNSbItSt11char_traitsItESaItEE9_M_mutateEmmm
// type: int __fastcall(_WORD **, int, int, int)
pub fn stub_0xe51c90() -> ! {
    todo!("0xe51c90 __ZNSbItSt11char_traitsItESaItEE9_M_mutateEmmm")
}

#[doc(alias = "std::basic_string<unsigned short,std::char_traits<unsigned short>,std::allocator<unsigned short>>::_Rep::_M_clone(std::allocator<unsigned short> const&,unsigned long)")]
#[doc(alias = "__ZNSbItSt11char_traitsItESaItEE4_Rep8_M_cloneERKS1_m")]
// 0xe51e28 — __ZNSbItSt11char_traitsItESaItEE4_Rep8_M_cloneERKS1_m
// type: _WORD *__fastcall(_DWORD *, int, int)
pub fn stub_0xe51e28() -> ! {
    todo!("0xe51e28 __ZNSbItSt11char_traitsItESaItEE4_Rep8_M_cloneERKS1_m")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::_M_insert_unique(std::pair<std::string const,unsigned short> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_")]
// 0xe5a058 — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xe5a058() -> ! {
    todo!("0xe5a058 __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,unsigned short> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// 0xe5a13c — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
pub fn stub_0xe5a13c() -> ! {
    todo!("0xe5a13c __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::find(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_")]
// 0xe5a284 — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_
// type: int(void)
pub fn stub_0xe5a284() -> ! {
    todo!("0xe5a284 __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned short>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xe5a3f8 — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0xe5a3f8() -> ! {
    todo!("0xe5a3f8 __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "anonymous namespace::readMeshFromV1(std::istream &,float)")]
#[doc(alias = "__ZN12_GLOBAL__N_114readMeshFromV1ERSif")]
// 0xea7f00 — __ZN12_GLOBAL__N_114readMeshFromV1ERSif
// type: _DWORD __fastcall(_anonymous_namespace_ *__hidden this, std::istream *, float)
pub fn stub_0xea7f00() -> ! {
    todo!("0xea7f00 __ZN12_GLOBAL__N_114readMeshFromV1ERSif")
}

#[doc(alias = "void std::vector<char,std::allocator<char>>::_M_range_insert<char *>(__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,char *,char *,std::forward_iterator_tag)")]
#[doc(alias = "__ZNSt6vectorIcSaIcEE15_M_range_insertIPcEEvN9__gnu_cxx17__normal_iteratorIS3_S1_EET_S7_St20forward_iterator_tag")]
// 0xea96b0 — __ZNSt6vectorIcSaIcEE15_M_range_insertIPcEEvN9__gnu_cxx17__normal_iteratorIS3_S1_EET_S7_St20forward_iterator_tag
// type: 
pub fn stub_0xea96b0() -> ! {
    todo!("0xea96b0 __ZNSt6vectorIcSaIcEE15_M_range_insertIPcEEvN9__gnu_cxx17__normal_iteratorIS3_S1_EET_S7_St20forward_iterator_tag")
}

#[doc(alias = "RBX::AdornBillboarder::isVisibleAndValid(void)const")]
#[doc(alias = "__ZNK3RBX16AdornBillboarder17isVisibleAndValidEv")]
// 0xeaa004 — __ZNK3RBX16AdornBillboarder17isVisibleAndValidEv
// type: _DWORD __fastcall(RBX::AdornBillboarder *__hidden this)
pub fn stub_0xeaa004() -> ! {
    todo!("0xeaa004 __ZNK3RBX16AdornBillboarder17isVisibleAndValidEv")
}

#[doc(alias = "RBX::AdornBillboarder::getViewport(void)const")]
#[doc(alias = "__ZNK3RBX16AdornBillboarder11getViewportEv")]
// 0xeaa00c — __ZNK3RBX16AdornBillboarder11getViewportEv
// type: _DWORD __fastcall(RBX::AdornBillboarder *__hidden this)
pub fn stub_0xeaa00c() -> ! {
    todo!("0xeaa00c __ZNK3RBX16AdornBillboarder11getViewportEv")
}

#[doc(alias = "RBX::ViewportBillboarder::ViewportBillboarder(void)")]
#[doc(alias = "__ZN3RBX19ViewportBillboarderC1Ev")]
// 0xeaa6c0 — __ZN3RBX19ViewportBillboarderC1Ev
// type: _DWORD __fastcall(RBX::ViewportBillboarder *__hidden this)
pub fn stub_0xeaa6c0() -> ! {
    todo!("0xeaa6c0 __ZN3RBX19ViewportBillboarderC1Ev")
}

#[doc(alias = "RBX::AdornBillboarder::getCamera(void)const")]
#[doc(alias = "__ZNK3RBX16AdornBillboarder9getCameraEv")]
// 0xeab23c — __ZNK3RBX16AdornBillboarder9getCameraEv
// type: _DWORD __fastcall(RBX::AdornBillboarder *__hidden this)
pub fn stub_0xeab23c() -> ! {
    todo!("0xeab23c __ZNK3RBX16AdornBillboarder9getCameraEv")
}

#[doc(alias = "RBX::AdornBillboarder::~AdornBillboarder()")]
#[doc(alias = "__ZN3RBX16AdornBillboarderD1Ev")]
// 0xeab240 — __ZN3RBX16AdornBillboarderD1Ev
// type: void __fastcall(RBX::AdornBillboarder *__hidden this)
pub fn stub_0xeab240() -> ! {
    todo!("0xeab240 __ZN3RBX16AdornBillboarderD1Ev")
}

#[doc(alias = "RBX::AdornBillboarder::~AdornBillboarder()")]
#[doc(alias = "__ZN3RBX16AdornBillboarderD0Ev")]
// 0xeab24c — __ZN3RBX16AdornBillboarderD0Ev
// type: void __fastcall(RBX::AdornBillboarder *__hidden this)
pub fn stub_0xeab24c() -> ! {
    todo!("0xeab24c __ZN3RBX16AdornBillboarderD0Ev")
}

#[doc(alias = "RBX::AdornBillboarder::createTextureProxy(RBX::ContentId const&,bool &,bool)")]
#[doc(alias = "__ZN3RBX16AdornBillboarder18createTextureProxyERKNS_9ContentIdERbb")]
// 0xeab2ec — __ZN3RBX16AdornBillboarder18createTextureProxyERKNS_9ContentIdERbb
// type: 
pub fn stub_0xeab2ec() -> ! {
    todo!("0xeab2ec __ZN3RBX16AdornBillboarder18createTextureProxyERKNS_9ContentIdERbb")
}

#[doc(alias = "RBX::AdornBillboarder::getRenderCaps(void)const")]
#[doc(alias = "__ZNK3RBX16AdornBillboarder13getRenderCapsEv")]
// 0xeac1f0 — __ZNK3RBX16AdornBillboarder13getRenderCapsEv
// type: _DWORD __fastcall(RBX::AdornBillboarder *__hidden this)
pub fn stub_0xeac1f0() -> ! {
    todo!("0xeac1f0 __ZNK3RBX16AdornBillboarder13getRenderCapsEv")
}

#[doc(alias = "RBX::FrameRateManager::FrameRateManager(void)")]
#[doc(alias = "__ZN3RBX16FrameRateManagerC1Ev")]
// 0xeac5f0 — __ZN3RBX16FrameRateManagerC1Ev
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xeac5f0() -> ! {
    todo!("0xeac5f0 __ZN3RBX16FrameRateManagerC1Ev")
}

#[doc(alias = "RBX::FrameRateManager::FrameRateManager(void)")]
#[doc(alias = "__ZN3RBX16FrameRateManagerC2Ev")]
// 0xeac5fc — __ZN3RBX16FrameRateManagerC2Ev
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xeac5fc() -> ! {
    todo!("0xeac5fc __ZN3RBX16FrameRateManagerC2Ev")
}

#[doc(alias = "RBX::FrameRateManager::getAntialiasingMode(void)")]
#[doc(alias = "__ZN3RBX16FrameRateManager19getAntialiasingModeEv")]
// 0xeac948 — __ZN3RBX16FrameRateManager19getAntialiasingModeEv
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xeac948() -> ! {
    todo!("0xeac948 __ZN3RBX16FrameRateManager19getAntialiasingModeEv")
}

#[doc(alias = "RBX::FrameRateManager::~FrameRateManager()")]
#[doc(alias = "__ZN3RBX16FrameRateManagerD1Ev")]
// 0xeac958 — __ZN3RBX16FrameRateManagerD1Ev
// type: void __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xeac958() -> ! {
    todo!("0xeac958 __ZN3RBX16FrameRateManagerD1Ev")
}

#[doc(alias = "RBX::FrameRateManager::~FrameRateManager()")]
#[doc(alias = "__ZN3RBX16FrameRateManagerD2Ev")]
// 0xeac964 — __ZN3RBX16FrameRateManagerD2Ev
// type: void __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xeac964() -> ! {
    todo!("0xeac964 __ZN3RBX16FrameRateManagerD2Ev")
}

#[doc(alias = "RBX::FrameRateManager::TestAndAddBlockQuota(int,int,float,float,RBX::eShadowCullingPriority)")]
#[doc(alias = "__ZN3RBX16FrameRateManager20TestAndAddBlockQuotaEiiffNS_22eShadowCullingPriorityE")]
// 0xeaca48 — __ZN3RBX16FrameRateManager20TestAndAddBlockQuotaEiiffNS_22eShadowCullingPriorityE
// type: int __fastcall(int, int, int, int, float, int)
pub fn stub_0xeaca48() -> ! {
    todo!("0xeaca48 __ZN3RBX16FrameRateManager20TestAndAddBlockQuotaEiiffNS_22eShadowCullingPriorityE")
}

#[doc(alias = "RBX::FrameRateManager::SubmitCurrentFrame(double,double,double)")]
#[doc(alias = "__ZN3RBX16FrameRateManager18SubmitCurrentFrameEddd")]
// 0xeacae8 — __ZN3RBX16FrameRateManager18SubmitCurrentFrameEddd
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this, double, double, double)
pub fn stub_0xeacae8() -> ! {
    todo!("0xeacae8 __ZN3RBX16FrameRateManager18SubmitCurrentFrameEddd")
}

#[doc(alias = "RBX::FrameRateManager::UpdateStats(double,double,double)")]
#[doc(alias = "__ZN3RBX16FrameRateManager11UpdateStatsEddd")]
// 0xeacd40 — __ZN3RBX16FrameRateManager11UpdateStatsEddd
// type: int __fastcall(int this, double, double, double)
pub fn stub_0xeacd40() -> ! {
    todo!("0xeacd40 __ZN3RBX16FrameRateManager11UpdateStatsEddd")
}

#[doc(alias = "RBX::FrameRateManager::AdjustQuality(double,double,bool)")]
#[doc(alias = "__ZN3RBX16FrameRateManager13AdjustQualityEddb")]
// 0xeace90 — __ZN3RBX16FrameRateManager13AdjustQualityEddb
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this, double, double, bool)
pub fn stub_0xeace90() -> ! {
    todo!("0xeace90 __ZN3RBX16FrameRateManager13AdjustQualityEddb")
}

#[doc(alias = "RBX::FrameRateManager::StartCapturingMetrics(void)")]
#[doc(alias = "__ZN3RBX16FrameRateManager21StartCapturingMetricsEv")]
// 0xead260 — __ZN3RBX16FrameRateManager21StartCapturingMetricsEv
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xead260() -> ! {
    todo!("0xead260 __ZN3RBX16FrameRateManager21StartCapturingMetricsEv")
}

#[doc(alias = "RBX::FrameRateManager::GetTargetFrameTimeForNextLevel(void)const")]
#[doc(alias = "__ZNK3RBX16FrameRateManager30GetTargetFrameTimeForNextLevelEv")]
// 0xead2a0 — __ZNK3RBX16FrameRateManager30GetTargetFrameTimeForNextLevelEv
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xead2a0() -> ! {
    todo!("0xead2a0 __ZNK3RBX16FrameRateManager30GetTargetFrameTimeForNextLevelEv")
}

#[doc(alias = "RBX::FrameRateManager::GetTargetRenderTimeForNextLevel(void)const")]
#[doc(alias = "__ZNK3RBX16FrameRateManager31GetTargetRenderTimeForNextLevelEv")]
// 0xead2c0 — __ZNK3RBX16FrameRateManager31GetTargetRenderTimeForNextLevelEv
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xead2c0() -> ! {
    todo!("0xead2c0 __ZNK3RBX16FrameRateManager31GetTargetRenderTimeForNextLevelEv")
}

#[doc(alias = "RBX::FrameRateManager::StepQuality(bool,bool)")]
#[doc(alias = "__ZN3RBX16FrameRateManager11StepQualityEbb")]
// 0xead318 — __ZN3RBX16FrameRateManager11StepQualityEbb
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this, bool, bool)
pub fn stub_0xead318() -> ! {
    todo!("0xead318 __ZN3RBX16FrameRateManager11StepQualityEbb")
}

#[doc(alias = "RBX::FrameRateManager::getMetricValue(std::string const&)")]
#[doc(alias = "__ZN3RBX16FrameRateManager14getMetricValueERKSs")]
// 0xead528 — __ZN3RBX16FrameRateManager14getMetricValueERKSs
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this, const std::string *)
pub fn stub_0xead528() -> ! {
    todo!("0xead528 __ZN3RBX16FrameRateManager14getMetricValueERKSs")
}

#[doc(alias = "RBX::FrameRateManager::GetViewCullDistance(void)")]
#[doc(alias = "__ZN3RBX16FrameRateManager19GetViewCullDistanceEv")]
// 0xead770 — __ZN3RBX16FrameRateManager19GetViewCullDistanceEv
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xead770() -> ! {
    todo!("0xead770 __ZN3RBX16FrameRateManager19GetViewCullDistanceEv")
}

#[doc(alias = "RBX::FrameRateManager::GetFrameTimeAverage(void)")]
#[doc(alias = "__ZN3RBX16FrameRateManager19GetFrameTimeAverageEv")]
// 0xead784 — __ZN3RBX16FrameRateManager19GetFrameTimeAverageEv
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xead784() -> ! {
    todo!("0xead784 __ZN3RBX16FrameRateManager19GetFrameTimeAverageEv")
}

#[doc(alias = "RBX::FrameRateManager::GetPrepareTimeAverage(void)")]
#[doc(alias = "__ZN3RBX16FrameRateManager21GetPrepareTimeAverageEv")]
// 0xead7a4 — __ZN3RBX16FrameRateManager21GetPrepareTimeAverageEv
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xead7a4() -> ! {
    todo!("0xead7a4 __ZN3RBX16FrameRateManager21GetPrepareTimeAverageEv")
}

#[doc(alias = "RBX::FrameRateManager::GetRenderTimeAverage(void)")]
#[doc(alias = "__ZN3RBX16FrameRateManager20GetRenderTimeAverageEv")]
// 0xead7c4 — __ZN3RBX16FrameRateManager20GetRenderTimeAverageEv
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xead7c4() -> ! {
    todo!("0xead7c4 __ZN3RBX16FrameRateManager20GetRenderTimeAverageEv")
}

#[doc(alias = "RBX::FrameRateManager::GetRenderTimeStats(void)")]
#[doc(alias = "__ZN3RBX16FrameRateManager18GetRenderTimeStatsEv")]
// 0xead7e8 — __ZN3RBX16FrameRateManager18GetRenderTimeStatsEv
// type: _DWORD __fastcall(RBX::FrameRateManager *__hidden this)
pub fn stub_0xead7e8() -> ! {
    todo!("0xead7e8 __ZN3RBX16FrameRateManager18GetRenderTimeStatsEv")
}
