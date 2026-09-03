//! rendering — generated_rend_wdog2B_1788371993 — 120 stubs EA-sorted asc gap filler 0x86f1ac..0x880740 not yet in any crate (global gap filler distinct from rendering A, rbx_core::SharedPtr not boost, // 0xADDR mangled + #[doc(alias)] + todo)
//! Source: ida/export.json (85545 funcs) EA-sorted global filler distinct not yet in any crate
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x86f1ac — __ZN3RBX12MovePositionERN3G3D12Vector3int16ENS_5Voxel13FaceDirectionE
#[doc(alias = "RBX::MovePosition(G3D::Vector3int16 &,RBX::Voxel::FaceDirection)")]
#[doc(alias = "__ZN3RBX12MovePositionERN3G3D12Vector3int16ENS_5Voxel13FaceDirectionE")]
// was: RBX::MovePosition(G3D::Vector3int16 &,RBX::Voxel::FaceDirection)
// IDA 0x86f1ac: 36 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86f1ac() {
}

// 0x8715e4 — __ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::resize(unsigned long,RBX::Voxel::Cell)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EE6resizeEmS2_")]
// was: std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::resize(unsigned long,RBX::Voxel::Cell)
// IDA 0x8715e4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8715e4() {
}

// 0x8716a8 — __ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE9push_backERKS3_
#[doc(alias = "std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::push_back(RBX::Voxel::CellChangeListener * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE9push_backERKS3_")]
// was: std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::push_back(RBX::Voxel::CellChangeListener * const&)
// IDA 0x8716a8: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_8716a8() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x872db4 — __ZN3RBX16StringReadBufferrsERh
#[doc(alias = "RBX::StringReadBuffer::operator>>(unsigned char &)")]
#[doc(alias = "__ZN3RBX16StringReadBufferrsERh")]
// was: RBX::StringReadBuffer::operator>>(unsigned char &)
// IDA 0x872db4: 108 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872db4() {
}

// 0x872f0c — __ZN3RBX14readCountValueINS_16StringReadBufferEEEjRT_
#[doc(alias = "unsigned int RBX::readCountValue<RBX::StringReadBuffer>(RBX::StringReadBuffer &)")]
#[doc(alias = "__ZN3RBX14readCountValueINS_16StringReadBufferEEEjRT_")]
// was: unsigned int RBX::readCountValue<RBX::StringReadBuffer>(RBX::StringReadBuffer &)
// IDA 0x872f0c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872f0c() {
}

// 0x872f44 — __ZN3RBX15writeCountValueINS_17StringWriteBufferEEEvRT_j
// type: int __fastcall(std::string *this)
#[doc(alias = "void RBX::writeCountValue<RBX::StringWriteBuffer>(RBX::StringWriteBuffer &,unsigned int)")]
#[doc(alias = "__ZN3RBX15writeCountValueINS_17StringWriteBufferEEEvRT_j")]
// was: void RBX::writeCountValue<RBX::StringWriteBuffer>(RBX::StringWriteBuffer &,unsigned int)
// IDA 0x872f44: 41 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872f44() {
}

// 0x872fc4 — __ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
// was: std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener * const&)
// IDA 0x872fc4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_872fc4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x8730a4 — __ZNSt12_Vector_baseIPN3RBX5Voxel18CellChangeListenerESaIS3_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX5Voxel18CellChangeListenerESaIS3_EE11_M_allocateEm")]
// was: std::_Vector_base<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::_M_allocate(unsigned long)
// IDA 0x8730a4: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_8730a4() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x8730bc — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX5Voxel18CellChangeListenerESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener *>(__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener * const&,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX5Voxel18CellChangeListenerESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag")]
// was: __gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener *>(__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener * const&,std::random_access_iterator_tag)
// IDA 0x8730bc: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8730bc() {
}

// 0x87314c — __ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: char *__fastcall(char *result, char *__b, size_t __len, _BYTE *)
#[doc(alias = "std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::Cell*,std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>>,unsigned long,RBX::Voxel::Cell const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// was: std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::Cell*,std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>>,unsigned long,RBX::Voxel::Cell const&)
// IDA 0x87314c: 156 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87314c() {
}

// 0x87788c — __ZN3rbx8any_castIN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::WaterCellDirection * rbx::any_cast<RBX::Voxel::WaterCellDirection,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// was: RBX::Voxel::WaterCellDirection * rbx::any_cast<RBX::Voxel::WaterCellDirection,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// IDA 0x87788c: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87788c() {
}

// 0x8778e4 — __ZN3rbx8any_castIRN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::WaterCellDirection & rbx::any_cast<RBX::Voxel::WaterCellDirection &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: RBX::Voxel::WaterCellDirection & rbx::any_cast<RBX::Voxel::WaterCellDirection &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x8778e4: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8778e4() {
}

// 0x8779d4 — __ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::resize(unsigned long,RBX::Voxel::WaterCellDirection)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE6resizeEmS2_")]
// was: std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::resize(unsigned long,RBX::Voxel::WaterCellDirection)
// IDA 0x8779d4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8779d4() {
}

// 0x877a08 — __ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::push_back(RBX::Voxel::WaterCellDirection const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE9push_backERKS2_")]
// was: std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::push_back(RBX::Voxel::WaterCellDirection const&)
// IDA 0x877a08: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_877a08() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x877a30 — __ZNSt3mapIPKN3RBX4NameENS0_5Voxel18WaterCellDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::WaterCellDirection,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_5Voxel18WaterCellDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// was: std::map<RBX::Name const*,RBX::Voxel::WaterCellDirection,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::operator[](RBX::Name const* const&)
// IDA 0x877a30: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_877a30() {
}

// 0x877a88 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)
// IDA 0x877a88: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_877a88() {
}

// 0x877b3c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)
// IDA 0x877b3c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_877b3c() {
}

// 0x877b94 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)
// IDA 0x877b94: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_877b94() {
}

// 0x877bfc — __ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellDirection*,std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>>,RBX::Voxel::WaterCellDirection const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// was: std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellDirection*,std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>>,RBX::Voxel::WaterCellDirection const&)
// IDA 0x877bfc: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_877bfc() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x877ce0 — __ZNSt12_Vector_baseIN3RBX5Voxel18WaterCellDirectionESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX5Voxel18WaterCellDirectionESaIS2_EE11_M_allocateEm")]
// was: std::_Vector_base<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_allocate(unsigned long)
// IDA 0x877ce0: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_877ce0() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x877cf8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel18WaterCellDirectionES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Voxel::WaterCellDirection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *>(RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel18WaterCellDirectionES6_EET0_T_S8_S7_")]
// was: RBX::Voxel::WaterCellDirection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *>(RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *)
// IDA 0x877cf8: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_877cf8() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x877d34 — __ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellDirection*,std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>>,unsigned long,RBX::Voxel::WaterCellDirection const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// was: std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellDirection*,std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>>,unsigned long,RBX::Voxel::WaterCellDirection const&)
// IDA 0x877d34: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_877d34() {
}

// 0x877ec4 — __ZN3rbx8any_castIN3RBX5Voxel14WaterCellForceENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::WaterCellForce * rbx::any_cast<RBX::Voxel::WaterCellForce,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX5Voxel14WaterCellForceENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// was: RBX::Voxel::WaterCellForce * rbx::any_cast<RBX::Voxel::WaterCellForce,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// IDA 0x877ec4: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_877ec4() {
}

// 0x877f1c — __ZN3rbx8any_castIRN3RBX5Voxel14WaterCellForceENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::WaterCellForce & rbx::any_cast<RBX::Voxel::WaterCellForce &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX5Voxel14WaterCellForceENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: RBX::Voxel::WaterCellForce & rbx::any_cast<RBX::Voxel::WaterCellForce &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x877f1c: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_877f1c() {
}

// 0x87800c — __ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::resize(unsigned long,RBX::Voxel::WaterCellForce)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE6resizeEmS2_")]
// was: std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::resize(unsigned long,RBX::Voxel::WaterCellForce)
// IDA 0x87800c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87800c() {
}

// 0x878040 — __ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::push_back(RBX::Voxel::WaterCellForce const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE9push_backERKS2_")]
// was: std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::push_back(RBX::Voxel::WaterCellForce const&)
// IDA 0x878040: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_878040() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x878068 — __ZNSt3mapIPKN3RBX4NameENS0_5Voxel14WaterCellForceESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::WaterCellForce,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_5Voxel14WaterCellForceESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// was: std::map<RBX::Name const*,RBX::Voxel::WaterCellForce,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::operator[](RBX::Name const* const&)
// IDA 0x878068: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878068() {
}

// 0x8780c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)
// IDA 0x8780c0: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8780c0() {
}

// 0x878174 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)
// IDA 0x878174: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878174() {
}

// 0x8781cc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)
// IDA 0x8781cc: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8781cc() {
}

// 0x878234 — __ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellForce*,std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>>,RBX::Voxel::WaterCellForce const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// was: std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellForce*,std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>>,RBX::Voxel::WaterCellForce const&)
// IDA 0x878234: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_878234() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x878318 — __ZNSt12_Vector_baseIN3RBX5Voxel14WaterCellForceESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX5Voxel14WaterCellForceESaIS2_EE11_M_allocateEm")]
// was: std::_Vector_base<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_allocate(unsigned long)
// IDA 0x878318: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_878318() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x878330 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel14WaterCellForceES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Voxel::WaterCellForce * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *>(RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel14WaterCellForceES6_EET0_T_S8_S7_")]
// was: RBX::Voxel::WaterCellForce * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *>(RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *)
// IDA 0x878330: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_878330() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x87836c — __ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellForce*,std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>>,unsigned long,RBX::Voxel::WaterCellForce const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// was: std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellForce*,std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>>,unsigned long,RBX::Voxel::WaterCellForce const&)
// IDA 0x87836c: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87836c() {
}

// 0x8784fc — __ZN3rbx8any_castIN3RBX5Voxel15CellOrientationENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::CellOrientation * rbx::any_cast<RBX::Voxel::CellOrientation,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX5Voxel15CellOrientationENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// was: RBX::Voxel::CellOrientation * rbx::any_cast<RBX::Voxel::CellOrientation,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// IDA 0x8784fc: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8784fc() {
}

// 0x878554 — __ZN3rbx8any_castIRN3RBX5Voxel15CellOrientationENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::CellOrientation & rbx::any_cast<RBX::Voxel::CellOrientation &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX5Voxel15CellOrientationENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: RBX::Voxel::CellOrientation & rbx::any_cast<RBX::Voxel::CellOrientation &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x878554: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878554() {
}

// 0x878644 — __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::resize(unsigned long,RBX::Voxel::CellOrientation)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE6resizeEmS2_")]
// was: std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::resize(unsigned long,RBX::Voxel::CellOrientation)
// IDA 0x878644: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878644() {
}

// 0x878678 — __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::push_back(RBX::Voxel::CellOrientation const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE9push_backERKS2_")]
// was: std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::push_back(RBX::Voxel::CellOrientation const&)
// IDA 0x878678: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_878678() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x8786a0 — __ZNSt3mapIPKN3RBX4NameENS0_5Voxel15CellOrientationESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::CellOrientation,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_5Voxel15CellOrientationESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// was: std::map<RBX::Name const*,RBX::Voxel::CellOrientation,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::operator[](RBX::Name const* const&)
// IDA 0x8786a0: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8786a0() {
}

// 0x8786f8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)
// IDA 0x8786f8: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8786f8() {
}

// 0x8787ac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)
// IDA 0x8787ac: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8787ac() {
}

// 0x878804 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)
// IDA 0x878804: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878804() {
}

// 0x87886c — __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellOrientation*,std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>>,RBX::Voxel::CellOrientation const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// was: std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellOrientation*,std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>>,RBX::Voxel::CellOrientation const&)
// IDA 0x87886c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_87886c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x878950 — __ZNSt12_Vector_baseIN3RBX5Voxel15CellOrientationESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX5Voxel15CellOrientationESaIS2_EE11_M_allocateEm")]
// was: std::_Vector_base<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_allocate(unsigned long)
// IDA 0x878950: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_878950() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x878968 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel15CellOrientationES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Voxel::CellOrientation * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *>(RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel15CellOrientationES6_EET0_T_S8_S7_")]
// was: RBX::Voxel::CellOrientation * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *>(RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *)
// IDA 0x878968: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_878968() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x8789a4 — __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellOrientation*,std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>>,unsigned long,RBX::Voxel::CellOrientation const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// was: std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellOrientation*,std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>>,unsigned long,RBX::Voxel::CellOrientation const&)
// IDA 0x8789a4: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8789a4() {
}

// 0x878b34 — __ZN3rbx8any_castIN3RBX5Voxel9CellBlockENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::CellBlock * rbx::any_cast<RBX::Voxel::CellBlock,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX5Voxel9CellBlockENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// was: RBX::Voxel::CellBlock * rbx::any_cast<RBX::Voxel::CellBlock,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// IDA 0x878b34: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878b34() {
}

// 0x878b8c — __ZN3rbx8any_castIRN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::CellBlock & rbx::any_cast<RBX::Voxel::CellBlock &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: RBX::Voxel::CellBlock & rbx::any_cast<RBX::Voxel::CellBlock &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x878b8c: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878b8c() {
}

// 0x878c7c — __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::resize(unsigned long,RBX::Voxel::CellBlock)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE6resizeEmS2_")]
// was: std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::resize(unsigned long,RBX::Voxel::CellBlock)
// IDA 0x878c7c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878c7c() {
}

// 0x878cb0 — __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::push_back(RBX::Voxel::CellBlock const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE9push_backERKS2_")]
// was: std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::push_back(RBX::Voxel::CellBlock const&)
// IDA 0x878cb0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_878cb0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x878cd8 — __ZNSt3mapIPKN3RBX4NameENS0_5Voxel9CellBlockESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::CellBlock,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_5Voxel9CellBlockESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// was: std::map<RBX::Name const*,RBX::Voxel::CellBlock,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::operator[](RBX::Name const* const&)
// IDA 0x878cd8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878cd8() {
}

// 0x878d30 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)
// IDA 0x878d30: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878d30() {
}

// 0x878de4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)
// IDA 0x878de4: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878de4() {
}

// 0x878e3c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)
// IDA 0x878e3c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878e3c() {
}

// 0x878ea4 — __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellBlock*,std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>>,RBX::Voxel::CellBlock const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// was: std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellBlock*,std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>>,RBX::Voxel::CellBlock const&)
// IDA 0x878ea4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_878ea4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x878f88 — __ZNSt12_Vector_baseIN3RBX5Voxel9CellBlockESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX5Voxel9CellBlockESaIS2_EE11_M_allocateEm")]
// was: std::_Vector_base<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_allocate(unsigned long)
// IDA 0x878f88: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_878f88() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x878fa0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel9CellBlockES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Voxel::CellBlock * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *>(RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel9CellBlockES6_EET0_T_S8_S7_")]
// was: RBX::Voxel::CellBlock * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *>(RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *)
// IDA 0x878fa0: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_878fa0() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x878fdc — __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellBlock*,std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>>,unsigned long,RBX::Voxel::CellBlock const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// was: std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellBlock*,std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>>,unsigned long,RBX::Voxel::CellBlock const&)
// IDA 0x878fdc: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_878fdc() {
}

// 0x87916c — __ZN3rbx8any_castIN3RBX5Voxel12CellMaterialENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::CellMaterial * rbx::any_cast<RBX::Voxel::CellMaterial,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX5Voxel12CellMaterialENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// was: RBX::Voxel::CellMaterial * rbx::any_cast<RBX::Voxel::CellMaterial,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// IDA 0x87916c: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87916c() {
}

// 0x8791c4 — __ZN3rbx8any_castIRN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::CellMaterial & rbx::any_cast<RBX::Voxel::CellMaterial &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: RBX::Voxel::CellMaterial & rbx::any_cast<RBX::Voxel::CellMaterial &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x8791c4: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8791c4() {
}

// 0x8792b4 — __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::resize(unsigned long,RBX::Voxel::CellMaterial)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE6resizeEmS2_")]
// was: std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::resize(unsigned long,RBX::Voxel::CellMaterial)
// IDA 0x8792b4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8792b4() {
}

// 0x8792e8 — __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::push_back(RBX::Voxel::CellMaterial const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE9push_backERKS2_")]
// was: std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::push_back(RBX::Voxel::CellMaterial const&)
// IDA 0x8792e8: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_8792e8() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x879310 — __ZNSt3mapIPKN3RBX4NameENS0_5Voxel12CellMaterialESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::CellMaterial,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_5Voxel12CellMaterialESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// was: std::map<RBX::Name const*,RBX::Voxel::CellMaterial,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::operator[](RBX::Name const* const&)
// IDA 0x879310: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_879310() {
}

// 0x879368 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)
// IDA 0x879368: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_879368() {
}

// 0x87941c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)
// IDA 0x87941c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87941c() {
}

// 0x879474 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)
// IDA 0x879474: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_879474() {
}

// 0x8794dc — __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellMaterial*,std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>>,RBX::Voxel::CellMaterial const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// was: std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellMaterial*,std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>>,RBX::Voxel::CellMaterial const&)
// IDA 0x8794dc: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_8794dc() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x8795c0 — __ZNSt12_Vector_baseIN3RBX5Voxel12CellMaterialESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX5Voxel12CellMaterialESaIS2_EE11_M_allocateEm")]
// was: std::_Vector_base<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_allocate(unsigned long)
// IDA 0x8795c0: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_8795c0() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x8795d8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel12CellMaterialES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Voxel::CellMaterial * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *>(RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel12CellMaterialES6_EET0_T_S8_S7_")]
// was: RBX::Voxel::CellMaterial * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *>(RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *)
// IDA 0x8795d8: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_8795d8() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x879614 — __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellMaterial*,std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>>,unsigned long,RBX::Voxel::CellMaterial const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// was: std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellMaterial*,std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>>,unsigned long,RBX::Voxel::CellMaterial const&)
// IDA 0x879614: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_879614() {
}

// 0x879810 — __ZNSt6vectorIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EED2Ev
#[doc(alias = "std::vector<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EED2Ev")]
// was: std::vector<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>>::~vector()
// IDA 0x879810: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_879810() {
}

// 0x87a47c — __ZN3RBX11CellContactD0Ev
// type: void __fastcall(RBX::CellContact *__hidden this)
#[doc(alias = "RBX::CellContact::~CellContact()")]
#[doc(alias = "__ZN3RBX11CellContactD0Ev")]
// was: RBX::CellContact::~CellContact()
// IDA 0x87a47c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_87a47c() {
}

// 0x87a51c — __ZN3RBX11CellContactD1Ev
// type: void __fastcall(RBX::CellContact *__hidden this)
#[doc(alias = "RBX::CellContact::~CellContact()")]
#[doc(alias = "__ZN3RBX11CellContactD1Ev")]
// was: RBX::CellContact::~CellContact()
// IDA 0x87a51c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_87a51c() {
}

// 0x87a520 — __ZN3RBX11CellContactD2Ev
// type: void __fastcall(RBX::CellContact *__hidden this)
#[doc(alias = "RBX::CellContact::~CellContact()")]
#[doc(alias = "__ZN3RBX11CellContactD2Ev")]
// was: RBX::CellContact::~CellContact()
// IDA 0x87a520: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_87a520() {
}

// 0x87a650 — __ZN3RBX11CellContact16deleteConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::CellContact::deleteConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX11CellContact16deleteConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
// was: RBX::CellContact::deleteConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)
// IDA 0x87a650: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87a650() {
}

// 0x87a714 — __ZN3RBX11CellContact12getConnectorEi
// type: _DWORD __fastcall(RBX::CellContact *__hidden this, int)
#[doc(alias = "RBX::CellContact::getConnector(int)")]
#[doc(alias = "__ZN3RBX11CellContact12getConnectorEi")]
// was: RBX::CellContact::getConnector(int)
// IDA 0x87a714: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87a714() {
}

// 0x87a71c — __ZN3RBX11CellContact19deleteAllConnectorsEv
// type: _DWORD __fastcall(RBX::CellContact *__hidden this)
#[doc(alias = "RBX::CellContact::deleteAllConnectors(void)")]
#[doc(alias = "__ZN3RBX11CellContact19deleteAllConnectorsEv")]
// was: RBX::CellContact::deleteAllConnectors(void)
// IDA 0x87a71c: 2 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87a71c() {
}

// 0x87a724 — __ZN3RBX11CellContact29removeAllConnectorsFromKernelEv
// type: _DWORD __fastcall(RBX::CellContact *__hidden this)
#[doc(alias = "RBX::CellContact::removeAllConnectorsFromKernel(void)")]
#[doc(alias = "__ZN3RBX11CellContact29removeAllConnectorsFromKernelEv")]
// was: RBX::CellContact::removeAllConnectorsFromKernel(void)
// IDA 0x87a724: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87a724() {
}

// 0x87a794 — __ZN3RBX11CellContact24putAllConnectorsInKernelEv
// type: _DWORD __fastcall(RBX::CellContact *__hidden this)
#[doc(alias = "RBX::CellContact::putAllConnectorsInKernel(void)")]
#[doc(alias = "__ZN3RBX11CellContact24putAllConnectorsInKernelEv")]
// was: RBX::CellContact::putAllConnectorsInKernel(void)
// IDA 0x87a794: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87a794() {
}

// 0x87a830 — __ZN3RBX11CellContact11stepContactEv
// type: _DWORD __fastcall(RBX::CellContact *__hidden this)
#[doc(alias = "RBX::CellContact::stepContact(void)")]
#[doc(alias = "__ZN3RBX11CellContact11stepContactEv")]
// was: RBX::CellContact::stepContact(void)
// IDA 0x87a830: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87a830() {
}

// 0x87a86c — __ZN3RBX11CellContact18computeIsCollidingEf
// type: _DWORD __fastcall(RBX::CellContact *__hidden this, float)
#[doc(alias = "RBX::CellContact::computeIsColliding(float)")]
#[doc(alias = "__ZN3RBX11CellContact18computeIsCollidingEf")]
// was: RBX::CellContact::computeIsColliding(float)
// IDA 0x87a86c: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87a86c() {
}

// 0x87a8d4 — __ZN3RBX11CellContact21updateClosestFeaturesEv
// type: _DWORD __fastcall(RBX::CellContact *__hidden this)
#[doc(alias = "RBX::CellContact::updateClosestFeatures(void)")]
#[doc(alias = "__ZN3RBX11CellContact21updateClosestFeaturesEv")]
// was: RBX::CellContact::updateClosestFeatures(void)
// IDA 0x87a8d4: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87a8d4() {
}

// 0x87a914 — __ZN3RBX11CellContact19worstFeatureOverlapEv
// type: _DWORD __fastcall(RBX::CellContact *__hidden this)
#[doc(alias = "RBX::CellContact::worstFeatureOverlap(void)")]
#[doc(alias = "__ZN3RBX11CellContact19worstFeatureOverlapEv")]
// was: RBX::CellContact::worstFeatureOverlap(void)
// IDA 0x87a914: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87a914() {
}

// 0x87a9c8 — __ZN3RBX11CellContact20matchClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::CellContact::matchClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX11CellContact20matchClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
// was: RBX::CellContact::matchClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)
// IDA 0x87a9c8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87a9c8() {
}

// 0x87aa20 — __ZN3RBX11CellContact19updateContactPointsEv
// type: _DWORD __fastcall(RBX::CellContact *__hidden this)
#[doc(alias = "RBX::CellContact::updateContactPoints(void)")]
#[doc(alias = "__ZN3RBX11CellContact19updateContactPointsEv")]
// was: RBX::CellContact::updateContactPoints(void)
// IDA 0x87aa20: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87aa20() {
}

// 0x87aa50 — __ZN3RBX11CellContact19matchClosestFeatureEPNS_13PolyConnectorE
// type: _DWORD __fastcall(RBX::CellContact *__hidden this, RBX::PolyConnector *)
#[doc(alias = "RBX::CellContact::matchClosestFeature(RBX::PolyConnector *)")]
#[doc(alias = "__ZN3RBX11CellContact19matchClosestFeatureEPNS_13PolyConnectorE")]
// was: RBX::CellContact::matchClosestFeature(RBX::PolyConnector *)
// IDA 0x87aa50: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87aa50() {
}

// 0x87aaa8 — __ZN3RBX11CellContact13getVoxelStoreINS_5Voxel4GridEEEPT_v
// type: int(void)
#[doc(alias = "RBX::Voxel::Grid * RBX::CellContact::getVoxelStore<RBX::Voxel::Grid>(void)")]
#[doc(alias = "__ZN3RBX11CellContact13getVoxelStoreINS_5Voxel4GridEEEPT_v")]
// was: RBX::Voxel::Grid * RBX::CellContact::getVoxelStore<RBX::Voxel::Grid>(void)
// IDA 0x87aaa8: 8 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87aaa8() {
}

// 0x87aac0 — __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EEixEm
#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::operator[](unsigned long)")]
#[doc(alias = "__ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EEixEm")]
// was: RBX::FixedArray<RBX::PolyConnector *,40ul>::operator[](unsigned long)
// IDA 0x87aac0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87aac0() {
}

// 0x87ab20 — __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE7replaceEmRKS2_
#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::replace(unsigned long,RBX::PolyConnector * const&)")]
#[doc(alias = "__ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE7replaceEmRKS2_")]
// was: RBX::FixedArray<RBX::PolyConnector *,40ul>::replace(unsigned long,RBX::PolyConnector * const&)
// IDA 0x87ab20: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87ab20() {
}

// 0x87abd8 — __ZN3RBX13PolyConnector5matchEPS0_S1_
#[doc(alias = "RBX::PolyConnector::match(RBX::PolyConnector*,RBX::PolyConnector*)")]
#[doc(alias = "__ZN3RBX13PolyConnector5matchEPS0_S1_")]
// was: RBX::PolyConnector::match(RBX::PolyConnector*,RBX::PolyConnector*)
// IDA 0x87abd8: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87abd8() {
}

// 0x87ac14 — __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE10fastRemoveEm
#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::fastRemove(unsigned long)")]
#[doc(alias = "__ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE10fastRemoveEm")]
// was: RBX::FixedArray<RBX::PolyConnector *,40ul>::fastRemove(unsigned long)
// IDA 0x87ac14: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87ac14() {
}

// 0x87b2d8 — __ZN3RBX15MegaClusterPoly9buildMeshEv
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this)
#[doc(alias = "RBX::MegaClusterPoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX15MegaClusterPoly9buildMeshEv")]
// was: RBX::MegaClusterPoly::buildMesh(void)
// IDA 0x87b2d8: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b2d8() {
}

// 0x87b3bc — __ZN3RBX15MegaClusterPoly7hitTestERKNS_6RbxRayERN3G3D7Vector3ERbfRNS_6CellIDEbb
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, G3D::Vector3 *, bool *, float, RBX::CellID *, bool, bool)
#[doc(alias = "RBX::MegaClusterPoly::hitTest(RBX::RbxRay const&,G3D::Vector3 &,bool &,float,RBX::CellID &,bool,bool)")]
#[doc(alias = "__ZN3RBX15MegaClusterPoly7hitTestERKNS_6RbxRayERN3G3D7Vector3ERbfRNS_6CellIDEbb")]
// was: RBX::MegaClusterPoly::hitTest(RBX::RbxRay const&,G3D::Vector3 &,bool &,float,RBX::CellID &,bool,bool)
// IDA 0x87b3bc: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b3bc() {
}

// 0x87b414 — __ZN3RBX15MegaClusterPoly9hitTestMCERKNS_6RbxRayERN3G3D7Vector3ERbRiRNS4_15CoordinateFrameEfRNS_6CellIDEbb
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, G3D::Vector3 *, bool *, int *, G3D::CoordinateFrame *, float, RBX::CellID *, bool, bool)
#[doc(alias = "RBX::MegaClusterPoly::hitTestMC(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)")]
#[doc(alias = "__ZN3RBX15MegaClusterPoly9hitTestMCERKNS_6RbxRayERN3G3D7Vector3ERbRiRNS4_15CoordinateFrameEfRNS_6CellIDEbb")]
// was: RBX::MegaClusterPoly::hitTestMC(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)
// IDA 0x87b414: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b414() {
}

// 0x87b474 — __ZNK3RBX15MegaClusterPoly21getSurfaceCoordInBodyEm
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, unsigned int)
#[doc(alias = "RBX::MegaClusterPoly::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly21getSurfaceCoordInBodyEm")]
// was: RBX::MegaClusterPoly::getSurfaceCoordInBody(unsigned long)const
// IDA 0x87b474: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b474() {
}

// 0x87b480 — __ZNK3RBX15MegaClusterPoly25getFaceFromLegacyNormalIdENS_8NormalIdE
#[doc(alias = "RBX::MegaClusterPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
// was: RBX::MegaClusterPoly::getFaceFromLegacyNormalId(RBX::NormalId)const
// IDA 0x87b480: 2 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b480() {
}

// 0x87b488 — __ZNK3RBX15MegaClusterPoly26findTouchingSurfacesConvexERKN3G3D15CoordinateFrameERmRKNS_8GeometryES4_S5_
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const G3D::CoordinateFrame *, unsigned int *, const RBX::Geometry *, const G3D::CoordinateFrame *, unsigned int *)
#[doc(alias = "RBX::MegaClusterPoly::findTouchingSurfacesConvex(G3D::CoordinateFrame const&,unsigned long &,RBX::Geometry const&,G3D::CoordinateFrame const&,unsigned long &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly26findTouchingSurfacesConvexERKN3G3D15CoordinateFrameERmRKNS_8GeometryES4_S5_")]
// was: RBX::MegaClusterPoly::findTouchingSurfacesConvex(G3D::CoordinateFrame const&,unsigned long &,RBX::Geometry const&,G3D::CoordinateFrame const&,unsigned long &)const
// IDA 0x87b488: 113 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b488() {
}

// 0x87b5bc — __ZNK3RBX15MegaClusterPoly35findCellsTouchingGeometryWithBufferERKfRKN3G3D15CoordinateFrameERKNS_8GeometryES6_PSt3mapIiPNS3_12Vector3int16ESt4lessIiESaISt4pairIKiSC_EEE
// type: int __fastcall(int, int, int, int, G3D::CoordinateFrame *, int)
#[doc(alias = "RBX::MegaClusterPoly::findCellsTouchingGeometryWithBuffer(float const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly35findCellsTouchingGeometryWithBufferERKfRKN3G3D15CoordinateFrameERKNS_8GeometryES6_PSt3mapIiPNS3_12Vector3int16ESt4lessIiESaISt4pairIKiSC_EEE")]
// was: RBX::MegaClusterPoly::findCellsTouchingGeometryWithBuffer(float const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const
// IDA 0x87b5bc: 135 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b5bc() {
}

// 0x87b784 — __ZNK3RBX15MegaClusterPoly25findPlanarTouchesWithGeomERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::MegaClusterPoly::findPlanarTouchesWithGeom(G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly25findPlanarTouchesWithGeomERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE")]
// was: RBX::MegaClusterPoly::findPlanarTouchesWithGeom(G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const
// IDA 0x87b784: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b784() {
}

// 0x87b828 — __ZNK3RBX15MegaClusterPoly22hasPlanarTouchWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const G3D::Vector3int16 *, const G3D::CoordinateFrame *, const RBX::Geometry *, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hasPlanarTouchWithGeom(G3D::Vector3int16 const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly22hasPlanarTouchWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_")]
// was: RBX::MegaClusterPoly::hasPlanarTouchWithGeom(G3D::Vector3int16 const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&)const
// IDA 0x87b828: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b828() {
}

// 0x87b874 — __ZNK3RBX15MegaClusterPoly28findCellIntersectionWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_Rm
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const G3D::Vector3int16 *, const G3D::CoordinateFrame *, const RBX::Geometry *, const G3D::CoordinateFrame *, unsigned int *)
#[doc(alias = "RBX::MegaClusterPoly::findCellIntersectionWithGeom(G3D::Vector3int16 const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,unsigned long &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly28findCellIntersectionWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_Rm")]
// was: RBX::MegaClusterPoly::findCellIntersectionWithGeom(G3D::Vector3int16 const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,unsigned long &)const
// IDA 0x87b874: 500 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b874() {
}

// 0x87be18 — __ZNK3RBX15MegaClusterPoly28hitLocationOnCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, const G3D::Vector3int16 *, const int *, G3D::Vector3 *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnCornerWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly28hitLocationOnCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE")]
// was: RBX::MegaClusterPoly::hitLocationOnCornerWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const
// IDA 0x87be18: 477 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87be18() {
}

// 0x87c450 — __ZNK3RBX15MegaClusterPoly32hitLocationOnHorizontalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
// type: int __fastcall(RBX::MegaClusterPoly *this, const RBX::RbxRay *, const G3D::Vector3int16 *, int *, G3D::Vector3 *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnHorizontalWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly32hitLocationOnHorizontalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE")]
// was: RBX::MegaClusterPoly::hitLocationOnHorizontalWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const
// IDA 0x87c450: 593 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87c450() {
}

// 0x87cc0c — __ZNK3RBX15MegaClusterPoly30hitLocationOnVerticalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
// type: int __fastcall(RBX::MegaClusterPoly *this, const RBX::RbxRay *, const G3D::Vector3int16 *, int *, G3D::Vector3 *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnVerticalWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly30hitLocationOnVerticalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE")]
// was: RBX::MegaClusterPoly::hitLocationOnVerticalWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const
// IDA 0x87cc0c: 599 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87cc0c() {
}

// 0x87d3e0 — __ZNK3RBX15MegaClusterPoly35hitLocationOnInverseCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, const G3D::Vector3int16 *, const int *, G3D::Vector3 *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnInverseCornerWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly35hitLocationOnInverseCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE")]
// was: RBX::MegaClusterPoly::hitLocationOnInverseCornerWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const
// IDA 0x87d3e0: 783 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87d3e0() {
}

// 0x87de28 — __ZNK3RBX15MegaClusterPoly22hitLocationOnBlockCellERKNS_6RbxRayERKN3G3D12Vector3int16ERNS4_7Vector3ERiRNS4_15CoordinateFrameE
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, const G3D::Vector3int16 *, G3D::Vector3 *, int *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnBlockCell(RBX::RbxRay const&,G3D::Vector3int16 const&,G3D::Vector3 &,int &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly22hitLocationOnBlockCellERKNS_6RbxRayERKN3G3D12Vector3int16ERNS4_7Vector3ERiRNS4_15CoordinateFrameE")]
// was: RBX::MegaClusterPoly::hitLocationOnBlockCell(RBX::RbxRay const&,G3D::Vector3int16 const&,G3D::Vector3 &,int &,G3D::CoordinateFrame &)const
// IDA 0x87de28: 704 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87de28() {
}

// 0x87e738 — __ZNK3RBX15MegaClusterPoly25findCellsTouchingGeometryERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE
// type: int __fastcall(int, int, int, G3D::CoordinateFrame *, int)
#[doc(alias = "RBX::MegaClusterPoly::findCellsTouchingGeometry(G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly25findCellsTouchingGeometryERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE")]
// was: RBX::MegaClusterPoly::findCellsTouchingGeometry(G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const
// IDA 0x87e738: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87e738() {
}

// 0x87e758 — __ZN3RBX15MegaClusterPoly18cellsInBoundingBoxERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::MegaClusterPoly::cellsInBoundingBox(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX15MegaClusterPoly18cellsInBoundingBoxERKN3G3D7Vector3ES4_")]
// was: RBX::MegaClusterPoly::cellsInBoundingBox(G3D::Vector3 const&,G3D::Vector3 const&)
// IDA 0x87e758: 620 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87e758() {
}

// 0x87edfc — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE8getTokenERKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE8getTokenERKS2_")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)
// IDA 0x87edfc: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87edfc() {
}

// 0x87ef60 — __ZNK3RBX15MegaClusterPoly19hitTestMC_templatedINS_5Voxel4GridEEEbRKNS_6RbxRayERN3G3D7Vector3ERbRiRNS7_15CoordinateFrameEfRNS_6CellIDEbb
// type: int __fastcall(int, int, int, int, int, G3D::CoordinateFrame *, float, int, int, int)
#[doc(alias = "bool RBX::MegaClusterPoly::hitTestMC_templated<RBX::Voxel::Grid>(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly19hitTestMC_templatedINS_5Voxel4GridEEEbRKNS_6RbxRayERN3G3D7Vector3ERbRiRNS7_15CoordinateFrameEfRNS_6CellIDEbb")]
// was: bool RBX::MegaClusterPoly::hitTestMC_templated<RBX::Voxel::Grid>(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)const
// IDA 0x87ef60: 500 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87ef60() {
}

// 0x87fc58 — __ZN3RBX15MegaClusterPolyD1Ev
// type: void __fastcall(RBX::MegaClusterPoly *__hidden this)
#[doc(alias = "RBX::MegaClusterPoly::~MegaClusterPoly()")]
#[doc(alias = "__ZN3RBX15MegaClusterPolyD1Ev")]
// was: RBX::MegaClusterPoly::~MegaClusterPoly()
// IDA 0x87fc58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_87fc58() {
}

// 0x87fc7c — __ZN3RBX15MegaClusterPolyD0Ev
// type: void __fastcall(RBX::MegaClusterPoly *__hidden this)
#[doc(alias = "RBX::MegaClusterPoly::~MegaClusterPoly()")]
#[doc(alias = "__ZN3RBX15MegaClusterPolyD0Ev")]
// was: RBX::MegaClusterPoly::~MegaClusterPoly()
// IDA 0x87fc7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_87fc7c() {
}

// 0x88004c — __ZN3RBX24getRegionForCellLocationINS_5Voxel4GridEEEKNT_6RegionEPKS3_RKN3G3D12Vector3int16EPS4_
// type: int __fastcall(int, int, G3D::Vector3int16 *this)
#[doc(alias = "RBX::Voxel::Grid::Region const RBX::getRegionForCellLocation<RBX::Voxel::Grid>(RBX::Voxel::Grid::Region const*,G3D::Vector3int16 const&,RBX::Voxel::Grid::Region const*)")]
#[doc(alias = "__ZN3RBX24getRegionForCellLocationINS_5Voxel4GridEEEKNT_6RegionEPKS3_RKN3G3D12Vector3int16EPS4_")]
// was: RBX::Voxel::Grid::Region const RBX::getRegionForCellLocation<RBX::Voxel::Grid>(RBX::Voxel::Grid::Region const*,G3D::Vector3int16 const&,RBX::Voxel::Grid::Region const*)
// IDA 0x88004c: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88004c() {
}

// 0x8800ec — __ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_
#[doc(alias = "std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)")]
#[doc(alias = "__ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_")]
// was: std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)
// IDA 0x8800ec: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8800ec() {
}

// 0x880344 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *)")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *)
// IDA 0x880344: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880344() {
}

// 0x880520 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10ValueCountD2Ev
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount::~ValueCount()")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10ValueCountD2Ev")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount::~ValueCount()
// IDA 0x880520: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_880520() {
}

// 0x88067c — __ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEdlEPv")]
// was: RBX::Allocator<RBX::POLY::MegaClusterMesh>::operator delete(void *)
// IDA 0x88067c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88067c() {
}

// 0x8806b8 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)
// IDA 0x8806b8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8806b8() {
}

// 0x8806e0 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>)
// IDA 0x8806e0: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8806e0() {
}

// 0x880740 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>> *)
// IDA 0x880740: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880740() {
}
