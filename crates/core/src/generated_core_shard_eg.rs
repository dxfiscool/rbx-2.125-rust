//! core shard EG — 100 core stubs EA-sorted, lowest uncovered 0x8e4ea0..0x8f2e40 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EF 0x8e4e70).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>::~vector()")]
// 0x8e4ea0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EED2Ev
// was: std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::~vector()
pub fn stub_8e4ea0() {
    // IDA 0x8e4ea0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>,std::allocator<std::vector<rbx_core::SharedPtr<RBX::GuiBase>,std::allocator<rbx_core::SharedPtr<RBX::GuiBase>>>>>::~vector()")]
// 0x8e4f6c — __ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EED2Ev
// was: std::vector<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>,std::allocator<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>>::~vector()
pub fn stub_8e4f6c() {
    // IDA 0x8e4f6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContextActionService::setContextButtonEnabled(bool)")]
// 0x8e56a4 — __ZN3RBX20ContextActionService23setContextButtonEnabledEb
pub fn stub_8e56a4() {
    // IDA 0x8e56a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContextActionService::getCurrentLocalToolIcon(void)")]
// 0x8e56c4 — __ZN3RBX20ContextActionService23getCurrentLocalToolIconEv
pub fn stub_8e56c4() {
    // IDA 0x8e56c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContextActionService::activateLocalTool(void)")]
// 0x8e580c — __ZN3RBX20ContextActionService17activateLocalToolEv
pub fn stub_8e580c() {
    // IDA 0x8e580c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContextActionService::deactivateLocalTool(void)")]
// 0x8e5814 — __ZN3RBX20ContextActionService19deactivateLocalToolEv
pub fn stub_8e5814() {
    // IDA 0x8e5814: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContextActionService::ContextActionService(void)")]
// 0x8e581c — __ZN3RBX20ContextActionServiceC1Ev
pub fn stub_8e581c() {
    // IDA 0x8e581c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ContextActionService::ContextActionService(void)")]
// 0x8e5820 — __ZN3RBX20ContextActionServiceC2Ev
pub fn stub_8e5820() {
    // IDA 0x8e5820: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ContextActionService::handleToolActivation(bool)")]
// 0x8e5c18 — __ZN3RBX20ContextActionService20handleToolActivationEb
pub fn stub_8e5c18() {
    // IDA 0x8e5c18: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ContextActionService::getCurrentLocalTool(void)")]
// 0x8e5c90 — __ZN3RBX20ContextActionService19getCurrentLocalToolEv
pub fn stub_8e5c90() {
    // IDA 0x8e5c90: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ContextActionService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x8e6304 — __ZN3RBX20ContextActionService17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_8e6304() {
    // IDA 0x8e6304: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ContextActionService::getContextButtonEnabled(void)const")]
// 0x8e6424 — __ZNK3RBX20ContextActionService23getContextButtonEnabledEv
pub fn stub_8e6424() {
    // IDA 0x8e6424: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Tool> RBX::shared_from<RBX::Tool>(RBX::Tool*)")]
// 0x8e64d4 — __ZN3RBX11shared_fromINS_4ToolEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Tool> RBX::shared_from<RBX::Tool>(RBX::Tool*)
pub fn stub_8e64d4() {
    // IDA 0x8e64d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::ContextActionService::~ContextActionService()")]
// 0x8e66b8 — __ZN3RBX20ContextActionServiceD1Ev
pub fn stub_8e66b8() {
    // IDA 0x8e66b8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::ContextActionService::~ContextActionService()")]
// 0x8e66bc — __ZN3RBX20ContextActionServiceD0Ev
pub fn stub_8e66bc() {
    // IDA 0x8e66bc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ContextActionService::~ContextActionService()")]
// 0x8e6784 — __ZThn32_N3RBX20ContextActionServiceD1Ev
pub fn stub_8e6784() {
    // IDA 0x8e6784: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ContextActionService::~ContextActionService()")]
// 0x8e678c — __ZThn32_N3RBX20ContextActionServiceD0Ev
pub fn stub_8e678c() {
    // IDA 0x8e678c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ContextActionService::~ContextActionService()")]
// 0x8e6858 — __ZThn36_N3RBX20ContextActionServiceD1Ev
pub fn stub_8e6858() {
    // IDA 0x8e6858: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ContextActionService::~ContextActionService()")]
// 0x8e6860 — __ZThn36_N3RBX20ContextActionServiceD0Ev
pub fn stub_8e6860() {
    // IDA 0x8e6860: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContextActionService::~ContextActionService()")]
// 0x8e7ae0 — __ZN3RBX20ContextActionServiceD2Ev
pub fn stub_8e7ae0() {
    // IDA 0x8e7ae0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ProgramMemoryChecker::areMemoryPagePermissionsSetupForHacking(void)")]
// 0x8e968c — __ZN3RBX20ProgramMemoryChecker39areMemoryPagePermissionsSetupForHackingEv
pub fn stub_8e968c() {
    // IDA 0x8e968c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Voxel::Grid::Chunk::Chunk(void)")]
// 0x8e9714 — __ZN3RBX5Voxel4Grid5ChunkC1Ev
pub fn stub_8e9714() {
    // IDA 0x8e9714: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Voxel::Grid::Chunk::init(RBX::Voxel::Grid const*)")]
// 0x8e972c — __ZN3RBX5Voxel4Grid5Chunk4initEPKS1_
pub fn stub_8e972c() {
    // IDA 0x8e972c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::vector(unsigned long,RBX::Voxel::Cell const&,std::allocator<RBX::Voxel::Cell> const&)")]
// 0x8e9850 — __ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EEC2EmRKS2_RKS3_
pub fn stub_8e9850() {
    // IDA 0x8e9850: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Voxel::Grid::Grid(void)")]
// 0x8e9960 — __ZN3RBX5Voxel4GridC1Ev
pub fn stub_8e9960() {
    // IDA 0x8e9960: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::Grid::connectListener(RBX::Voxel::CellChangeListener *)")]
// 0x8ea170 — __ZN3RBX5Voxel4Grid15connectListenerEPNS0_18CellChangeListenerE
pub fn stub_8ea170() {
    // IDA 0x8ea170: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::Grid::disconnectListener(RBX::Voxel::CellChangeListener *)")]
// 0x8ea1b0 — __ZN3RBX5Voxel4Grid18disconnectListenerEPNS0_18CellChangeListenerE
pub fn stub_8ea1b0() {
    // IDA 0x8ea1b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::Grid::isAllocated(void)const")]
// 0x8ea1f8 — __ZNK3RBX5Voxel4Grid11isAllocatedEv
pub fn stub_8ea1f8() {
    // IDA 0x8ea1f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::find(RBX::SpatialRegion::Id const&)const")]
// 0x8ea204 — __ZNK3RBX5Voxel8ChunkMapINS0_4Grid5ChunkEE4findERKNS_13SpatialRegion2IdE
pub fn stub_8ea204() {
    // IDA 0x8ea204: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::insert(RBX::SpatialRegion::Id const&)")]
// 0x8ea244 — __ZN3RBX5Voxel8ChunkMapINS0_4Grid5ChunkEE6insertERKNS_13SpatialRegion2IdE
pub fn stub_8ea244() {
    // IDA 0x8ea244: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Voxel::Grid::Chunk::updateCountOfNonEmptyCells(int)")]
// 0x8ea4c8 — __ZN3RBX5Voxel4Grid5Chunk26updateCountOfNonEmptyCellsEi
pub fn stub_8ea4c8() {
    // IDA 0x8ea4c8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::ChunkMap(void)")]
// 0x8ea52c — __ZN3RBX5Voxel8ChunkMapINS0_4Grid5ChunkEEC2Ev
pub fn stub_8ea52c() {
    // IDA 0x8ea52c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::vector<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>>::vector(unsigned long,RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue const&,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue> const&)")]
// 0x8ea600 — __ZNSt6vectorIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EEC2EmRKS6_RKS7_
pub fn stub_8ea600() {
    // IDA 0x8ea600: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>>::_Vector_base(unsigned long,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue> const&)")]
// 0x8ea6cc — __ZNSt12_Vector_baseIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EEC2EmRKS7_
pub fn stub_8ea6cc() {
    // IDA 0x8ea6cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue *,unsigned long,RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>(RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue *,unsigned long,RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue const&,std::__false_type)")]
// 0x8ea700 — __ZSt26__uninitialized_fill_n_auxIPN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueEmS6_EvT_T0_RKT1_St12__false_type
pub fn stub_8ea700() {
    // IDA 0x8ea700: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<unsigned char,std::allocator<unsigned char>>::vector(std::vector<unsigned char,std::allocator<unsigned char>> const&)")]
// 0x8ea884 — __ZNSt6vectorIhSaIhEEC2ERKS1_
pub fn stub_8ea884() {
    // IDA 0x8ea884: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::vector(std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>> const&)")]
// 0x8ea8d4 — __ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EEC2ERKS4_
pub fn stub_8ea8d4() {
    // IDA 0x8ea8d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>>::_M_allocate(unsigned long)")]
// 0x8ea920 — __ZNSt12_Vector_baseIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EE11_M_allocateEm
pub fn stub_8ea920() {
    // IDA 0x8ea920: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::ComputeOrientedFace(RBX::Voxel::BlockFaceInfo const&,RBX::Voxel::FaceDirection,RBX::Voxel::CellOrientation)")]
// 0x8eabcc — __ZN3RBX5Voxel19ComputeOrientedFaceERKNS0_13BlockFaceInfoENS0_13FaceDirectionENS0_15CellOrientationE
pub fn stub_8eabcc() {
    // IDA 0x8eabcc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::initBlockOrientationFaceMap(void)")]
// 0x8eac28 — __ZN3RBX5Voxel27initBlockOrientationFaceMapEv
pub fn stub_8eac28() {
    // IDA 0x8eac28: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<RBX::InputObject>::convertToString(RBX::InputObject const&)")]
// 0x8eacf8 — __ZN3RBX15StringConverterINS_11InputObjectEE15convertToStringERKS1_
pub fn stub_8eacf8() {
    // IDA 0x8eacf8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::InputObject::operator=(RBX::InputObject const&)")]
// 0x8eb470 — __ZN3RBX11InputObjectaSERKS0_
pub fn stub_8eb470() {
    // IDA 0x8eb470: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::InputObject::operator==(RBX::InputObject const&)const")]
// 0x8eb488 — __ZNK3RBX11InputObjecteqERKS0_
pub fn stub_8eb488() {
    // IDA 0x8eb488: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>::resize(unsigned long,RBX::InputObject::UserInputState)")]
// 0x8ebb98 — __ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE6resizeEmS2_
pub fn stub_8ebb98() {
    // IDA 0x8ebb98: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>::push_back(RBX::InputObject::UserInputState const&)")]
// 0x8ebbcc — __ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE9push_backERKS2_
pub fn stub_8ebbcc() {
    // IDA 0x8ebbcc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::InputObject::UserInputState,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::operator[](RBX::Name const* const&)")]
// 0x8ebbf4 — __ZNSt3mapIPKN3RBX4NameENS0_11InputObject14UserInputStateESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_8ebbf4() {
    // IDA 0x8ebbf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState> const&)")]
// 0x8ebc4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_8ebc4c() {
    // IDA 0x8ebc4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState> const&)")]
// 0x8ebd00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_8ebd00() {
    // IDA 0x8ebd00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::InputObject::UserInputState> const&)")]
// 0x8ebd58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_8ebd58() {
    // IDA 0x8ebd58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::InputObject::UserInputState*,std::vector<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>>,RBX::InputObject::UserInputState const&)")]
// 0x8ebdc0 — __ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_8ebdc0() {
    // IDA 0x8ebdc0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>::_M_allocate(unsigned long)")]
// 0x8ebea4 — __ZNSt12_Vector_baseIN3RBX11InputObject14UserInputStateESaIS2_EE11_M_allocateEm
pub fn stub_8ebea4() {
    // IDA 0x8ebea4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::InputObject::UserInputState * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::InputObject::UserInputState *,RBX::InputObject::UserInputState *>(RBX::InputObject::UserInputState *,RBX::InputObject::UserInputState *,RBX::InputObject::UserInputState *)")]
// 0x8ebebc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11InputObject14UserInputStateES6_EET0_T_S8_S7_
pub fn stub_8ebebc() {
    // IDA 0x8ebebc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::InputObject::UserInputState*,std::vector<RBX::InputObject::UserInputState,std::allocator<RBX::InputObject::UserInputState>>>,unsigned long,RBX::InputObject::UserInputState const&)")]
// 0x8ebef8 — __ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_8ebef8() {
    // IDA 0x8ebef8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>::resize(unsigned long,RBX::InputObject::UserInputType)")]
// 0x8ec088 — __ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE6resizeEmS2_
pub fn stub_8ec088() {
    // IDA 0x8ec088: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>::push_back(RBX::InputObject::UserInputType const&)")]
// 0x8ec0bc — __ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE9push_backERKS2_
pub fn stub_8ec0bc() {
    // IDA 0x8ec0bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::InputObject::UserInputType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::operator[](RBX::Name const* const&)")]
// 0x8ec0e4 — __ZNSt3mapIPKN3RBX4NameENS0_11InputObject13UserInputTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_8ec0e4() {
    // IDA 0x8ec0e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType> const&)")]
// 0x8ec13c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_8ec13c() {
    // IDA 0x8ec13c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType> const&)")]
// 0x8ec1f0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_8ec1f0() {
    // IDA 0x8ec1f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::InputObject::UserInputType> const&)")]
// 0x8ec248 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_8ec248() {
    // IDA 0x8ec248: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::InputObject::UserInputType*,std::vector<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>>,RBX::InputObject::UserInputType const&)")]
// 0x8ec2b0 — __ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_8ec2b0() {
    // IDA 0x8ec2b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>::_M_allocate(unsigned long)")]
// 0x8ec394 — __ZNSt12_Vector_baseIN3RBX11InputObject13UserInputTypeESaIS2_EE11_M_allocateEm
pub fn stub_8ec394() {
    // IDA 0x8ec394: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::InputObject::UserInputType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::InputObject::UserInputType *,RBX::InputObject::UserInputType *>(RBX::InputObject::UserInputType *,RBX::InputObject::UserInputType *,RBX::InputObject::UserInputType *)")]
// 0x8ec3ac — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11InputObject13UserInputTypeES6_EET0_T_S8_S7_
pub fn stub_8ec3ac() {
    // IDA 0x8ec3ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::InputObject::UserInputType*,std::vector<RBX::InputObject::UserInputType,std::allocator<RBX::InputObject::UserInputType>>>,unsigned long,RBX::InputObject::UserInputType const&)")]
// 0x8ec3e8 — __ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_8ec3e8() {
    // IDA 0x8ec3e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Voxel::Water::RelevantNeighbors::RelevantNeighbors(RBX::Voxel::CellOrientation)")]
// 0x8ec640 — __ZN3RBX5Voxel5Water17RelevantNeighborsC2ENS0_15CellOrientationE
pub fn stub_8ec640() {
    // IDA 0x8ec640: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::OnScreenProfiler::OnScreenProfiler(void)")]
// 0x8ec788 — __ZN3RBX16OnScreenProfilerC1Ev
pub fn stub_8ec788() {
    // IDA 0x8ec788: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::OnScreenProfiler::OnScreenProfiler(void)")]
// 0x8ec78c — __ZN3RBX16OnScreenProfilerC2Ev
pub fn stub_8ec78c() {
    // IDA 0x8ec78c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::OnScreenProfiler::~OnScreenProfiler()")]
// 0x8ec978 — __ZN3RBX16OnScreenProfilerD0Ev
pub fn stub_8ec978() {
    // IDA 0x8ec978: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::OnScreenProfiler::~OnScreenProfiler()")]
// 0x8eca18 — __ZN3RBX16OnScreenProfilerD1Ev
pub fn stub_8eca18() {
    // IDA 0x8eca18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::OnScreenProfiler::~OnScreenProfiler()")]
// 0x8eca1c — __ZN3RBX16OnScreenProfilerD2Ev
pub fn stub_8eca1c() {
    // IDA 0x8eca1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::OnScreenProfiler::UpdateJobStart(RBX::TaskScheduler::Job *)")]
// 0x8eca94 — __ZN3RBX16OnScreenProfiler14UpdateJobStartEPNS_13TaskScheduler3JobE
pub fn stub_8eca94() {
    // IDA 0x8eca94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::OnScreenProfiler::AllocateNewJobInfo(RBX::TaskScheduler::Job *)")]
// 0x8ecb00 — __ZN3RBX16OnScreenProfiler18AllocateNewJobInfoEPNS_13TaskScheduler3JobE
pub fn stub_8ecb00() {
    // IDA 0x8ecb00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::OnScreenProfiler::UpdateJobEnd(RBX::TaskScheduler::Job *)")]
// 0x8ece38 — __ZN3RBX16OnScreenProfiler12UpdateJobEndEPNS_13TaskScheduler3JobE
pub fn stub_8ece38() {
    // IDA 0x8ece38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::OnScreenProfiler::AllocateThreadInfo(long)")]
// 0x8ed16c — __ZN3RBX16OnScreenProfiler18AllocateThreadInfoEl
pub fn stub_8ed16c() {
    // IDA 0x8ed16c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::OnScreenProfiler::BeginMarker(char const*)")]
// 0x8ed3c0 — __ZN3RBX16OnScreenProfiler11BeginMarkerEPKc
pub fn stub_8ed3c0() {
    // IDA 0x8ed3c0: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::OnScreenProfiler::EndMarker(void)")]
// 0x8ed59c — __ZN3RBX16OnScreenProfiler9EndMarkerEv
pub fn stub_8ed59c() {
    // IDA 0x8ed59c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::OnScreenProfiler::DrawProfilerBar(RBX::Adorn *)")]
// 0x8ed908 — __ZN3RBX16OnScreenProfiler15DrawProfilerBarEPNS_5AdornE
pub fn stub_8ed908() {
    // IDA 0x8ed908: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::DrawJobInfo(RBX::Adorn *,int)")]
// 0x8eda78 — __ZN3RBX16OnScreenProfiler11DrawJobInfoEPNS_5AdornEi
pub fn stub_8eda78() {
    // IDA 0x8eda78: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::DrawThreadInfo(RBX::Adorn *,int)")]
// 0x8ee5ac — __ZN3RBX16OnScreenProfiler14DrawThreadInfoEPNS_5AdornEi
pub fn stub_8ee5ac() {
    // IDA 0x8ee5ac: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::DrawJobInfoLine(RBX::Adorn *,int)")]
// 0x8eecf4 — __ZN3RBX16OnScreenProfiler15DrawJobInfoLineEPNS_5AdornEi
pub fn stub_8eecf4() {
    // IDA 0x8eecf4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::DrawJobTimeLine(RBX::Adorn *,int)")]
// 0x8ef5c4 — __ZN3RBX16OnScreenProfiler15DrawJobTimeLineEPNS_5AdornEi
pub fn stub_8ef5c4() {
    // IDA 0x8ef5c4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::DrawMarker(RBX::Adorn *,int)")]
// 0x8f02f8 — __ZN3RBX16OnScreenProfiler10DrawMarkerEPNS_5AdornEi
pub fn stub_8f02f8() {
    // IDA 0x8f02f8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::DrawGeneralInfo(RBX::Adorn *,int)")]
// 0x8f13c8 — __ZN3RBX16OnScreenProfiler15DrawGeneralInfoEPNS_5AdornEi
pub fn stub_8f13c8() {
    // IDA 0x8f13c8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::DrawLegend(RBX::Adorn *,float,float)")]
// 0x8f1df8 — __ZN3RBX16OnScreenProfiler10DrawLegendEPNS_5AdornEff
pub fn stub_8f1df8() {
    // IDA 0x8f1df8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::ToHash(char const*)")]
// 0x8f1e9c — __ZN3RBX16OnScreenProfiler6ToHashEPKc
pub fn stub_8f1e9c() {
    // IDA 0x8f1e9c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::MarkerCompareName(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)")]
// 0x8f1fc4 — __ZN3RBX16OnScreenProfiler17MarkerCompareNameERKNS_27OSProfilerMarkerTempDataStrES3_
pub fn stub_8f1fc4() {
    // IDA 0x8f1fc4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::TempMarkerCompare(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)")]
// 0x8f1fd8 — __ZN3RBX16OnScreenProfiler17TempMarkerCompareERKNS_24OSProfilerMarkerTempDataES3_
pub fn stub_8f1fd8() {
    // IDA 0x8f1fd8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::CaptureEvt(RBX::OSProfilerCaptureEvtType,RBX::OSProfilerEvtData const&)")]
// 0x8f212c — __ZN3RBX16OnScreenProfiler10CaptureEvtENS_24OSProfilerCaptureEvtTypeERKNS_17OSProfilerEvtDataE
pub fn stub_8f212c() {
    // IDA 0x8f212c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::UpdateInput(RBX::GuiEvent const&)")]
// 0x8f2588 — __ZN3RBX16OnScreenProfiler11UpdateInputERKNS_8GuiEventE
pub fn stub_8f2588() {
    // IDA 0x8f2588: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::GetJobFrameInfo(RBX::OSProfilerJobInfo *,int)")]
// 0x8f270c — __ZN3RBX16OnScreenProfiler15GetJobFrameInfoEPNS_17OSProfilerJobInfoEi
pub fn stub_8f270c() {
    // IDA 0x8f270c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::OnScreenProfiler::IsPreAllocateJob(char const*,long &)")]
// 0x8f281c — __ZN3RBX16OnScreenProfiler16IsPreAllocateJobEPKcRl
pub fn stub_8f281c() {
    // IDA 0x8f281c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "void std::__introsort_loop<RBX::OSProfilerMarkerTempData *,int,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,int,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0x8f29f4 — __ZSt16__introsort_loopIPN3RBX24OSProfilerMarkerTempDataEiPFbRKS1_S4_EEvT_S7_T0_T1_
pub fn stub_8f29f4() {
    // IDA 0x8f29f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__final_insertion_sort<RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0x8f2ad8 — __ZSt22__final_insertion_sortIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_8f2ad8() {
    // IDA 0x8f2ad8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__insertion_sort<RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0x8f2b34 — __ZSt16__insertion_sortIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_8f2b34() {
    // IDA 0x8f2b34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__unguarded_linear_insert<RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0x8f2bb0 — __ZSt25__unguarded_linear_insertIPN3RBX24OSProfilerMarkerTempDataES1_PFbRKS1_S4_EEvT_T0_T1_
pub fn stub_8f2bb0() {
    // IDA 0x8f2bb0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::OSProfilerMarkerTempData * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *)")]
// 0x8f2c20 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX24OSProfilerMarkerTempDataES5_EET0_T_S7_S6_
pub fn stub_8f2c20() {
    // IDA 0x8f2c20: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::OSProfilerMarkerTempData * std::__unguarded_partition<RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0x8f2c7c — __ZSt21__unguarded_partitionIPN3RBX24OSProfilerMarkerTempDataES1_PFbRKS1_S4_EET_S7_S7_T0_T1_
pub fn stub_8f2c7c() {
    // IDA 0x8f2c7c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__heap_select<RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0x8f2d08 — __ZSt13__heap_selectIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_S7_T0_
pub fn stub_8f2d08() {
    // IDA 0x8f2d08: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::sort_heap<RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0x8f2db8 — __ZSt9sort_heapIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_8f2db8() {
    // IDA 0x8f2db8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::pop_heap<RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,RBX::OSProfilerMarkerTempData *,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0x8f2de0 — __ZSt8pop_heapIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_8f2de0() {
    // IDA 0x8f2de0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__adjust_heap<RBX::OSProfilerMarkerTempData *,int,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,int,int,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0x8f2e40 — __ZSt13__adjust_heapIPN3RBX24OSProfilerMarkerTempDataEiS1_PFbRKS1_S4_EEvT_T0_S8_T1_T2_
pub fn stub_8f2e40() {
    // IDA 0x8f2e40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
