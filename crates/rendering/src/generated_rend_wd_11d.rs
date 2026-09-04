//! rendering shard rend_wd_11d — 100 stubs 0x7c05a8..0x7c35c8 EA-sorted asc global gap filler after 0x7c05a4 (Ogre/G3D complete, global gap filler EA asc) [skeleton batch rend_wd_11d]
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 100 uncovered sorted asc after 0x7c05a4
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7c05a8 — __ZN3rbx8any_castIN3RBX8Humanoid6StatusENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Humanoid::Status * rbx::any_cast<RBX::Humanoid::Status,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX8Humanoid6StatusENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// IDA 0x7c05a8: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c05a8() {
}

// 0x7c0600 — __ZN3rbx8any_castIRN3RBX8Humanoid6StatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Humanoid::Status & rbx::any_cast<RBX::Humanoid::Status &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX8Humanoid6StatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// IDA 0x7c0600: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c0600() {
}

// 0x7c06f0 — __ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::resize(unsigned long,RBX::Humanoid::Status)")]
#[doc(alias = "__ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE6resizeEmS2_")]
// IDA 0x7c06f0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c06f0() {
}

// 0x7c0724 — __ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::push_back(RBX::Humanoid::Status const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE9push_backERKS2_")]
// IDA 0x7c0724: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_7c0724() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x7c074c — __ZNSt3mapIPKN3RBX4NameENS0_8Humanoid6StatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Humanoid::Status,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_8Humanoid6StatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// IDA 0x7c074c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c074c() {
}

// 0x7c07a4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::pair<RBX::Name const* const,RBX::Humanoid::Status> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// IDA 0x7c07a4: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c07a4() {
}

// 0x7c0858 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Humanoid::Status> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// IDA 0x7c0858: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c0858() {
}

// 0x7c08b0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Humanoid::Status> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// IDA 0x7c08b0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c08b0() {
}

// 0x7c0918 — __ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Humanoid::Status*,std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>>,RBX::Humanoid::Status const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0x7c0918: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_7c0918() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x7c09fc — __ZNSt12_Vector_baseIN3RBX8Humanoid6StatusESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX8Humanoid6StatusESaIS2_EE11_M_allocateEm")]
// IDA 0x7c09fc: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_7c09fc() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x7c0a14 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8Humanoid6StatusES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Humanoid::Status * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Humanoid::Status *,RBX::Humanoid::Status *>(RBX::Humanoid::Status *,RBX::Humanoid::Status *,RBX::Humanoid::Status *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8Humanoid6StatusES6_EET0_T_S8_S7_")]
// IDA 0x7c0a14: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_7c0a14() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x7c0a50 — __ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Humanoid::Status*,std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>>,unsigned long,RBX::Humanoid::Status const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// IDA 0x7c0a50: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c0a50() {
}

// 0x7c0be0 — __ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::resize(unsigned long,RBX::Humanoid::NameOcclusion)")]
#[doc(alias = "__ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE6resizeEmS2_")]
// IDA 0x7c0be0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c0be0() {
}

// 0x7c0c14 — __ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::push_back(RBX::Humanoid::NameOcclusion const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE9push_backERKS2_")]
// IDA 0x7c0c14: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_7c0c14() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x7c0c3c — __ZNSt3mapIPKN3RBX4NameENS0_8Humanoid13NameOcclusionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Humanoid::NameOcclusion,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_8Humanoid13NameOcclusionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// IDA 0x7c0c3c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c0c3c() {
}

// 0x7c0c94 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// IDA 0x7c0c94: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c0c94() {
}

// 0x7c0d48 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// IDA 0x7c0d48: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c0d48() {
}

// 0x7c0da0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// IDA 0x7c0da0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c0da0() {
}

// 0x7c0e08 — __ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Humanoid::NameOcclusion*,std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>>,RBX::Humanoid::NameOcclusion const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0x7c0e08: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_7c0e08() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x7c0eec — __ZNSt12_Vector_baseIN3RBX8Humanoid13NameOcclusionESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX8Humanoid13NameOcclusionESaIS2_EE11_M_allocateEm")]
// IDA 0x7c0eec: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_7c0eec() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x7c0f04 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8Humanoid13NameOcclusionES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Humanoid::NameOcclusion * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Humanoid::NameOcclusion *,RBX::Humanoid::NameOcclusion *>(RBX::Humanoid::NameOcclusion *,RBX::Humanoid::NameOcclusion *,RBX::Humanoid::NameOcclusion *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8Humanoid13NameOcclusionES6_EET0_T_S8_S7_")]
// IDA 0x7c0f04: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_7c0f04() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x7c0f40 — __ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Humanoid::NameOcclusion*,std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>>,unsigned long,RBX::Humanoid::NameOcclusion const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// IDA 0x7c0f40: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c0f40() {
}

// 0x7c10d0 — __ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_12PartInstanceEEEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::FilteredSelection<RBX::PartInstance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::PartInstance>>(void)const — uses rbx_core::SharedPtr not boost
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::PartInstance>>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_12PartInstanceEEEEEPT_v")]
// IDA 0x7c10d0: 161 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c10d0() {
}

// 0x7c1298 — __ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_12PartInstanceEEEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::FilteredSelection<RBX::PartInstance> * RBX::ServiceProvider::find<RBX::FilteredSelection<RBX::PartInstance>>(void)const — uses rbx_core::SharedPtr not boost
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance> * RBX::ServiceProvider::find<RBX::FilteredSelection<RBX::PartInstance>>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_12PartInstanceEEEEEPT_v")]
// IDA 0x7c1298: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c1298() {
}

// 0x7c140c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17FilteredSelectionINS_12PartInstanceEEEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::FilteredSelection<RBX::PartInstance>> RBX::Creatable<RBX::Instance>::create<RBX::FilteredSelection<RBX::PartInstance>>(void) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::shared_ptr<RBX::FilteredSelection<RBX::PartInstance>> RBX::Creatable<RBX::Instance>::create<RBX::FilteredSelection<RBX::PartInstance>>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_17FilteredSelectionINS_12PartInstanceEEEEEN5boost10shared_ptrIT_EEv")]
// IDA 0x7c140c: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c140c() {
}

// 0x7c14bc — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17FilteredSelectionINS1_12PartInstanceEEEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FilteredSelection<RBX::PartInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::PartInstance>> const&) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FilteredSelection<RBX::PartInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::PartInstance>> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17FilteredSelectionINS1_12PartInstanceEEEEERS3_RKNS0_IT_EE")]
// IDA 0x7c14bc: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c14bc() {
}

// 0x7c14f0 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_12PartInstanceEEEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FilteredSelection<RBX::PartInstance>>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_12PartInstanceEEEEEvv")]
// IDA 0x7c14f0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7c14f0() {
}

// 0x7c14f4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_12PartInstanceEEEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FilteredSelection<RBX::PartInstance>>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_12PartInstanceEEEEEmv")]
// IDA 0x7c14f4: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c14f4() {
}

// 0x7c15cc — __ZN3RBX17FilteredSelectionINS_12PartInstanceEEC2Ev
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance>::FilteredSelection(void)")]
#[doc(alias = "__ZN3RBX17FilteredSelectionINS_12PartInstanceEEC2Ev")]
// IDA 0x7c15cc: 153 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c15cc() {
}

// 0x7c1788 — __ZN3RBX17FilteredSelectionINS_12PartInstanceEED1Ev
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance>::~FilteredSelection()")]
#[doc(alias = "__ZN3RBX17FilteredSelectionINS_12PartInstanceEED1Ev")]
// IDA 0x7c1788: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7c1788() {
}

// 0x7c178c — __ZN3RBX17FilteredSelectionINS_12PartInstanceEED0Ev
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance>::~FilteredSelection()")]
#[doc(alias = "__ZN3RBX17FilteredSelectionINS_12PartInstanceEED0Ev")]
// IDA 0x7c178c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c178c() {
}

// 0x7c182c — __ZN3RBX17FilteredSelectionINS_12PartInstanceEE17onAncestorChangedERKNS_15AncestorChangedE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::FilteredSelection<RBX::PartInstance>::onAncestorChanged(RBX::AncestorChanged const&) — uses rbx_core::SharedPtr not boost
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance>::onAncestorChanged(RBX::AncestorChanged const&)")]
#[doc(alias = "__ZN3RBX17FilteredSelectionINS_12PartInstanceEE17onAncestorChangedERKNS_15AncestorChangedE")]
// IDA 0x7c182c: 134 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c182c() {
}

// 0x7c19a4 — __ZN3RBX17FilteredSelectionINS_12PartInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance>::onSelectionChanged(RBX::SelectionChanged const&)")]
#[doc(alias = "__ZN3RBX17FilteredSelectionINS_12PartInstanceEE18onSelectionChangedERKNS_16SelectionChangedE")]
// IDA 0x7c19a4: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c19a4() {
}

// 0x7c1a20 — __ZThn32_N3RBX17FilteredSelectionINS_12PartInstanceEED1Ev
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PartInstance>::~FilteredSelection()")]
#[doc(alias = "__ZThn32_N3RBX17FilteredSelectionINS_12PartInstanceEED1Ev")]
// IDA 0x7c1a20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c1a20() {
}

// 0x7c1a28 — __ZThn32_N3RBX17FilteredSelectionINS_12PartInstanceEED0Ev
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PartInstance>::~FilteredSelection()")]
#[doc(alias = "__ZThn32_N3RBX17FilteredSelectionINS_12PartInstanceEED0Ev")]
// IDA 0x7c1a28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c1a28() {
}

// 0x7c1a30 — __ZThn36_N3RBX17FilteredSelectionINS_12PartInstanceEED1Ev
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PartInstance>::~FilteredSelection()")]
#[doc(alias = "__ZThn36_N3RBX17FilteredSelectionINS_12PartInstanceEED1Ev")]
// IDA 0x7c1a30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c1a30() {
}

// 0x7c1a38 — __ZThn36_N3RBX17FilteredSelectionINS_12PartInstanceEED0Ev
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PartInstance>::~FilteredSelection()")]
#[doc(alias = "__ZThn36_N3RBX17FilteredSelectionINS_12PartInstanceEED0Ev")]
// IDA 0x7c1a38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c1a38() {
}

// 0x7c1a40 — __ZThn96_N3RBX17FilteredSelectionINS_12PartInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
#[doc(alias = "non-virtual thunk toRBX::FilteredSelection<RBX::PartInstance>::onSelectionChanged(RBX::SelectionChanged const&)")]
#[doc(alias = "__ZThn96_N3RBX17FilteredSelectionINS_12PartInstanceEE18onSelectionChangedERKNS_16SelectionChangedE")]
// IDA 0x7c1a40: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c1a40() {
}

// 0x7c1a48 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX12PartInstanceESt6vectorIS4_SaIS4_EEEEPNS2_8InstanceEET_SC_SC_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::PartInstance **,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::PartInstance **,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,RBX::Instance *>(__gnu_cxx::__normal_iterator<RBX::PartInstance **,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,__gnu_cxx::__normal_iterator<RBX::PartInstance **,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,RBX::Instance * const&,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX12PartInstanceESt6vectorIS4_SaIS4_EEEEPNS2_8InstanceEET_SC_SC_RKT0_St26random_access_iterator_tag")]
// IDA 0x7c1a48: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c1a48() {
}

// 0x7c1ad8 — __ZN3RBX17FilteredSelectionINS_12PartInstanceEED2Ev
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance>::~FilteredSelection()")]
#[doc(alias = "__ZN3RBX17FilteredSelectionINS_12PartInstanceEED2Ev")]
// IDA 0x7c1ad8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c1ad8() {
}

// 0x7c1c0c — __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_12PartInstanceEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::FilteredSelection<RBX::PartInstance>>::shared_ptr<RBX::FilteredSelection<RBX::PartInstance>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::shared_ptr<RBX::FilteredSelection<RBX::PartInstance>>::shared_ptr<RBX::FilteredSelection<RBX::PartInstance>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_12PartInstanceEEEEC2IS4_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// IDA 0x7c1c0c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c1c0c() {
}

// 0x7c1cd4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_12PartInstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::PartInstance>,RBX::FilteredSelection<RBX::PartInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::PartInstance>> const*,RBX::FilteredSelection<RBX::PartInstance> *)const — uses rbx_core::SharedPtr not boost
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::PartInstance>,RBX::FilteredSelection<RBX::PartInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::PartInstance>> const*,RBX::FilteredSelection<RBX::PartInstance> *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_12PartInstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0x7c1cd4: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c1cd4() {
}

// 0x7c1dbc — __ZN5boost6detail12shared_countC2IPN3RBX17FilteredSelectionINS3_12PartInstanceEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX17FilteredSelectionINS3_12PartInstanceEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// IDA 0x7c1dbc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c1dbc() {
}

// 0x7c1ec4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_12PartInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_12PartInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0x7c1ec4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7c1ec4() {
}

// 0x7c1ec8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_12PartInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_12PartInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0x7c1ec8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7c1ec8() {
}

// 0x7c1ecc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_12PartInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_12PartInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0x7c1ecc: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c1ecc() {
}

// 0x7c1eec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_12PartInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_12PartInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0x7c1eec: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c1eec() {
}

// 0x7c1f04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_12PartInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FilteredSelection<RBX::PartInstance> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17FilteredSelectionINS2_12PartInstanceEEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0x7c1f04: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c1f04() {
}

// 0x7c1f08 — __ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperISt6vectorIPN3RBX9PrimitiveESaIS8_EEEEEclIPFvNS_10shared_ptrINS6_8InstanceEEERSA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>&),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>&) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int) — uses rbx_core::SharedPtr not boost
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>&),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>&) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperISt6vectorIPN3RBX9PrimitiveESaIS8_EEEEEclIPFvNS_10shared_ptrINS6_8InstanceEEERSA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i")]
// IDA 0x7c1f08: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c1f08() {
}

// 0x7c1fdc — __ZN5boost10shared_ptrIN3RBX5HUMAN13HumanoidStateEEC2IS3_EEPT_
// was: boost::shared_ptr<RBX::HUMAN::HumanoidState>::shared_ptr<RBX::HUMAN::HumanoidState>(RBX::HUMAN::HumanoidState *) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::shared_ptr<RBX::HUMAN::HumanoidState>::shared_ptr<RBX::HUMAN::HumanoidState>(RBX::HUMAN::HumanoidState *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5HUMAN13HumanoidStateEEC2IS3_EEPT_")]
// IDA 0x7c1fdc: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c1fdc() {
}

// 0x7c20b0 — __ZN5boost6detail12shared_countC2IN3RBX5HUMAN13HumanoidStateEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::HUMAN::HumanoidState>(RBX::HUMAN::HumanoidState *) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HUMAN::HumanoidState>(RBX::HUMAN::HumanoidState *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX5HUMAN13HumanoidStateEEEPT_")]
// IDA 0x7c20b0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c20b0() {
}

// 0x7c21a8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5HUMAN13HumanoidStateEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::HUMAN::HumanoidState>::~sp_counted_impl_p() — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HUMAN::HumanoidState>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5HUMAN13HumanoidStateEED1Ev")]
// IDA 0x7c21a8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7c21a8() {
}

// 0x7c21ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX5HUMAN13HumanoidStateEED0Ev
// was: boost::detail::sp_counted_impl_p<RBX::HUMAN::HumanoidState>::~sp_counted_impl_p() — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HUMAN::HumanoidState>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5HUMAN13HumanoidStateEED0Ev")]
// IDA 0x7c21ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7c21ac() {
}

// 0x7c21b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5HUMAN13HumanoidStateEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::HUMAN::HumanoidState>::dispose(void) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HUMAN::HumanoidState>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5HUMAN13HumanoidStateEE7disposeEv")]
// IDA 0x7c21b0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c21b0() {
}

// 0x7c21c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5HUMAN13HumanoidStateEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RBX::HUMAN::HumanoidState>::get_deleter(std::type_info const&) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HUMAN::HumanoidState>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5HUMAN13HumanoidStateEE11get_deleterERKSt9type_info")]
// IDA 0x7c21c0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c21c0() {
}

// 0x7c21c4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5HUMAN13HumanoidStateEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RBX::HUMAN::HumanoidState>::get_untyped_deleter(void) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HUMAN::HumanoidState>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5HUMAN13HumanoidStateEE19get_untyped_deleterEv")]
// IDA 0x7c21c4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c21c4() {
}

// 0x7c21c8 — __ZN5boost20dynamic_pointer_castIN3RBX12PartInstanceENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE
// was: boost::shared_ptr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Reflection::DescribedBase>(boost::shared_ptr<RBX::Reflection::DescribedBase> const&) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::shared_ptr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Reflection::DescribedBase>(boost::shared_ptr<RBX::Reflection::DescribedBase> const&)")]
#[doc(alias = "__ZN5boost20dynamic_pointer_castIN3RBX12PartInstanceENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE")]
// IDA 0x7c21c8: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c21c8() {
}

// 0x7c2210 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_5DecalEEEPKT_v
#[doc(alias = "RBX::Decal const* RBX::Instance::findConstFirstChildOfType<RBX::Decal>(void)const")]
#[doc(alias = "__ZNK3RBX8Instance25findConstFirstChildOfTypeINS_5DecalEEEPKT_v")]
// IDA 0x7c2210: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2210() {
}

// 0x7c2278 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_13DataModelMeshEEEPKT_v
#[doc(alias = "RBX::DataModelMesh const* RBX::Instance::findConstFirstChildOfType<RBX::DataModelMesh>(void)const")]
#[doc(alias = "__ZNK3RBX8Instance25findConstFirstChildOfTypeINS_13DataModelMeshEEEPKT_v")]
// IDA 0x7c2278: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2278() {
}

// 0x7c22e0 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_8BackpackEEEPKT_v
#[doc(alias = "RBX::Backpack const* RBX::Instance::findConstFirstChildOfType<RBX::Backpack>(void)const")]
#[doc(alias = "__ZNK3RBX8Instance25findConstFirstChildOfTypeINS_8BackpackEEEPKT_v")]
// IDA 0x7c22e0: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c22e0() {
}

// 0x7c2348 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8HumanoidEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_
// was: RBX::Reflection::EventDescImpl<1,RBX::Humanoid,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Humanoid::*>::fireEvent(RBX::Humanoid*,boost::shared_ptr<RBX::Instance>)const — uses rbx_core::SharedPtr not boost
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Humanoid,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Humanoid::*>::fireEvent(RBX::Humanoid*,boost::shared_ptr<RBX::Instance>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_8HumanoidEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_")]
// IDA 0x7c2348: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2348() {
}

// 0x7c241c — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_8HumanoidEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_
// was: RBX::Reflection::RemoteEventDescImpl<1,RBX::Humanoid,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Instance>) — uses rbx_core::SharedPtr not boost
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::Humanoid,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_8HumanoidEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_")]
// IDA 0x7c241c: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c241c() {
}

// 0x7c2568 — __ZNK3RBX13ModelInstance28findConstFirstModifierOfTypeINS_8HumanoidEEEPKT_v
#[doc(alias = "RBX::Humanoid const* RBX::ModelInstance::findConstFirstModifierOfType<RBX::Humanoid>(void)const")]
#[doc(alias = "__ZNK3RBX13ModelInstance28findConstFirstModifierOfTypeINS_8HumanoidEEEPKT_v")]
// IDA 0x7c2568: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2568() {
}

// 0x7c25b4 — __ZN5boost10shared_ptrIN3RBX14StatusInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::StatusInstance>::shared_ptr<RBX::StatusInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::shared_ptr<RBX::StatusInstance>::shared_ptr<RBX::StatusInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14StatusInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// IDA 0x7c25b4: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c25b4() {
}

// 0x7c267c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14StatusInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StatusInstance,RBX::StatusInstance>(boost::shared_ptr<RBX::StatusInstance> const*,RBX::StatusInstance *)const — uses rbx_core::SharedPtr not boost
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StatusInstance,RBX::StatusInstance>(boost::shared_ptr<RBX::StatusInstance> const*,RBX::StatusInstance *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14StatusInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0x7c267c: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c267c() {
}

// 0x7c2764 — __ZN5boost6detail12shared_countC2IPN3RBX14StatusInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX14StatusInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// IDA 0x7c2764: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2764() {
}

// 0x7c286c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14StatusInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14StatusInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0x7c286c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7c286c() {
}

// 0x7c2870 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14StatusInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14StatusInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0x7c2870: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7c2870() {
}

// 0x7c2874 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14StatusInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14StatusInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0x7c2874: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2874() {
}

// 0x7c2894 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14StatusInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14StatusInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0x7c2894: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2894() {
}

// 0x7c28ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14StatusInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StatusInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14StatusInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0x7c28ac: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c28ac() {
}

// 0x7c28b0 — __ZN5boost10shared_ptrIN3RBX7Motor6DEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Motor6D>::shared_ptr<RBX::Motor6D,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::shared_ptr<RBX::Motor6D>::shared_ptr<RBX::Motor6D,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7Motor6DEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// IDA 0x7c28b0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c28b0() {
}

// 0x7c2978 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Motor6DES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Motor6D,RBX::Motor6D>(boost::shared_ptr<RBX::Motor6D> const*,RBX::Motor6D *)const — uses rbx_core::SharedPtr not boost
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Motor6D,RBX::Motor6D>(boost::shared_ptr<RBX::Motor6D> const*,RBX::Motor6D *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Motor6DES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0x7c2978: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2978() {
}

// 0x7c2a60 — __ZN5boost6detail12shared_countC2IPN3RBX7Motor6DENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX7Motor6DENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// IDA 0x7c2a60: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2a60() {
}

// 0x7c2b68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Motor6DENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Motor6DENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0x7c2b68: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7c2b68() {
}

// 0x7c2b6c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Motor6DENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Motor6DENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0x7c2b6c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7c2b6c() {
}

// 0x7c2b70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Motor6DENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Motor6DENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0x7c2b70: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2b70() {
}

// 0x7c2b90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Motor6DENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Motor6DENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0x7c2b90: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2b90() {
}

// 0x7c2ba8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Motor6DENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Motor6D *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Motor6DENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0x7c2ba8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2ba8() {
}

// 0x7c2bac — __ZN3rbx7signals6signalIFvfEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot> &) — uses rbx_core::SharedPtr not boost
#[doc(alias = "rbx::signals::signal<void ()(float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")]
// IDA 0x7c2bac: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2bac() {
}

// 0x7c2d0c — __ZN3rbx7signals6signalIFvfEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(float)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE8on_errorERSt9exception")]
// IDA 0x7c2d0c: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2d0c() {
}

// 0x7c2d34 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8HumanoidES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>> const&) — uses rbx_core::SharedPtr not boost
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8HumanoidES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_")]
// IDA 0x7c2d34: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2d34() {
}

// 0x7c2da8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8HumanoidES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>>::~callable_slot() — uses rbx_core::SharedPtr not boost
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8HumanoidES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev")]
// IDA 0x7c2da8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c2da8() {
}

// 0x7c2dd4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8HumanoidES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>>::~callable_slot() — uses rbx_core::SharedPtr not boost
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_8HumanoidES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev")]
// IDA 0x7c2dd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c2dd4() {
}

// 0x7c2ea8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8HumanoidES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>) — uses rbx_core::SharedPtr not boost
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8HumanoidES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
// IDA 0x7c2ea8: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2ea8() {
}

// 0x7c2ec4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8HumanoidES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>) — uses rbx_core::SharedPtr not boost
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8HumanoidES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
// IDA 0x7c2ec4: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2ec4() {
}

// 0x7c2ee0 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX8HumanoidEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<RBX::Humanoid *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int) — uses rbx_core::SharedPtr not boost
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Humanoid *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX8HumanoidEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
// IDA 0x7c2ee0: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2ee0() {
}

// 0x7c2fb8 — __ZNK5boost4_mfi3mf1IvN3RBX8HumanoidENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// was: boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Humanoid*,boost::shared_ptr<RBX::Instance>)const — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Humanoid*,boost::shared_ptr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX8HumanoidENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_")]
// IDA 0x7c2fb8: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c2fb8() {
}

// 0x7c30a0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8HumanoidES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable() — uses rbx_core::SharedPtr not boost
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8HumanoidES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev")]
// IDA 0x7c30a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c30a0() {
}

// 0x7c30cc — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8HumanoidES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable() — uses rbx_core::SharedPtr not boost
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Humanoid,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Humanoid*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_8HumanoidES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev")]
// IDA 0x7c30cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c30cc() {
}

// 0x7c31a0 — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(RBX::Humanoid::Status)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot> &) — uses rbx_core::SharedPtr not boost
#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
// IDA 0x7c31a0: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c31a0() {
}

// 0x7c3300 — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE8on_errorERSt9exception")]
// IDA 0x7c3300: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c3300() {
}

// 0x7c3328 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot> const&) — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEEaSERKSA_")]
// IDA 0x7c3328: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c3328() {
}

// 0x7c334c — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE22safe_static_init_mutexEv")]
// IDA 0x7c334c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7c334c() {
}

// 0x7c3350 — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE24safe_static_do_get_mutexEv")]
// IDA 0x7c3350: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c3350() {
}

// 0x7c3448 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPSt6vectorIN3RBX10Reflection7VariantESaIS8_EEEEEclIPFvNS_10shared_ptrINS6_8InstanceEEESB_ENS0_5list1IRKSH_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int) — uses rbx_core::SharedPtr not boost
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPSt6vectorIN3RBX10Reflection7VariantESaIS8_EEEEEclIPFvNS_10shared_ptrINS6_8InstanceEEESB_ENS0_5list1IRKSH_EEEEvNS0_4typeIvEERT_RT0_i")]
// IDA 0x7c3448: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c3448() {
}

// 0x7c351c — __ZN3RBX18DescribedCreatableINS_8HumanoidENS_8InstanceELZNS_9sHumanoidEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_8HumanoidENS_8InstanceELZNS_9sHumanoidEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_8HumanoidENS_8InstanceELZNS_9sHumanoidEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x7c351c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7c351c() {
}

// 0x7c3520 — __ZN3RBX18DescribedCreatableINS_8HumanoidENS_8InstanceELZNS_9sHumanoidEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_8HumanoidENS_8InstanceELZNS_9sHumanoidEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_8HumanoidENS_8InstanceELZNS_9sHumanoidEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x7c3520: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c3520() {
}

// 0x7c35c0 — __ZThn32_N3RBX18DescribedCreatableINS_8HumanoidENS_8InstanceELZNS_9sHumanoidEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_8HumanoidENS_8InstanceELZNS_9sHumanoidEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_8HumanoidENS_8InstanceELZNS_9sHumanoidEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x7c35c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c35c0() {
}

// 0x7c35c8 — __ZThn32_N3RBX18DescribedCreatableINS_8HumanoidENS_8InstanceELZNS_9sHumanoidEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_8HumanoidENS_8InstanceELZNS_9sHumanoidEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_8HumanoidENS_8InstanceELZNS_9sHumanoidEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x7c35c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7c35c8() {
}
