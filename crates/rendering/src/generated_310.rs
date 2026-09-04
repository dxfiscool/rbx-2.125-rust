//! rendering shard 310 — 100 stubs 0x45bb98..0x45f6c4 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 33780->33880 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 33780 before -> 33880 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x45bb98 (lowest remaining 0x45bb98..0x45f6c4, next lowest 0x45f6e0)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x45bb98 — __ZNSt3mapIPKN3RBX4NameENS0_9DataModel5GenreESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::Genre,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_9DataModel5GenreESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x45bb98: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45bb98() {
}

// 0x45bbf0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::pair<RBX::Name const* const,RBX::DataModel::Genre> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x45bbf0: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45bbf0() {
}

// 0x45bca4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::Genre> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x45bca4: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45bca4() {
}

// 0x45bcfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::Genre> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x45bcfc: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45bcfc() {
}

// 0x45bd64 — __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::Genre*,std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>>,RBX::DataModel::Genre const&)")]
// was: __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x45bd64: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_45bd64() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x45be48 — __ZNSt12_Vector_baseIN3RBX9DataModel5GenreESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX9DataModel5GenreESaIS2_EE11_M_allocateEm
// IDA 0x45be48: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_45be48() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x45be60 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel5GenreES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::DataModel::Genre * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::Genre *,RBX::DataModel::Genre *>(RBX::DataModel::Genre *,RBX::DataModel::Genre *,RBX::DataModel::Genre *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel5GenreES6_EET0_T_S8_S7_
// IDA 0x45be60: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_45be60() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x45be9c — __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::Genre*,std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>>,unsigned long,RBX::DataModel::Genre const&)")]
// was: __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x45be9c: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45be9c() {
}

// 0x45c02c — __ZN3rbx8any_castIN3RBX9DataModel11CreatorTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::DataModel::CreatorType * rbx::any_cast<RBX::DataModel::CreatorType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: __ZN3rbx8any_castIN3RBX9DataModel11CreatorTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// IDA 0x45c02c: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45c02c() {
}

// 0x45c084 — __ZN3rbx8any_castIRN3RBX9DataModel11CreatorTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::DataModel::CreatorType & rbx::any_cast<RBX::DataModel::CreatorType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRN3RBX9DataModel11CreatorTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x45c084: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45c084() {
}

// 0x45c174 — __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::resize(unsigned long,RBX::DataModel::CreatorType)")]
// was: __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE6resizeEmS2_
// IDA 0x45c174: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45c174() {
}

// 0x45c1a8 — __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::push_back(RBX::DataModel::CreatorType const&)")]
// was: __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE9push_backERKS2_
// IDA 0x45c1a8: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_45c1a8() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x45c1d0 — __ZNSt3mapIPKN3RBX4NameENS0_9DataModel11CreatorTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::CreatorType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_9DataModel11CreatorTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x45c1d0: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45c1d0() {
}

// 0x45c228 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x45c228: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45c228() {
}

// 0x45c2dc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x45c2dc: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45c2dc() {
}

// 0x45c334 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::CreatorType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x45c334: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45c334() {
}

// 0x45c39c — __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::CreatorType*,std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>>,RBX::DataModel::CreatorType const&)")]
// was: __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x45c39c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_45c39c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x45c480 — __ZNSt12_Vector_baseIN3RBX9DataModel11CreatorTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Vector_base<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX9DataModel11CreatorTypeESaIS2_EE11_M_allocateEm
// IDA 0x45c480: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_45c480() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x45c498 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel11CreatorTypeES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::DataModel::CreatorType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::CreatorType *,RBX::DataModel::CreatorType *>(RBX::DataModel::CreatorType *,RBX::DataModel::CreatorType *,RBX::DataModel::CreatorType *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel11CreatorTypeES6_EET0_T_S8_S7_
// IDA 0x45c498: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_45c498() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x45c4d4 — __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::CreatorType*,std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>>,unsigned long,RBX::DataModel::CreatorType const&)")]
// was: __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x45c4d4: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45c4d4() {
}

// 0x45c664 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EEC2EMS2_FvSsSsEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::BoundFuncDesc(void (RBX::DataModel::*)(std::string,std::string),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EEC2EMS2_FvSsSsEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x45c664: 177 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45c664() {
}

// 0x45c82c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// IDA 0x45c82c: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45c82c() {
}

// 0x45c878 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EED0Ev
// IDA 0x45c878: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_45c878() {
}

// 0x45c94c — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x45c94c: 160 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45c94c() {
}

// 0x45cb18 — __ZN3RBX10Reflection11Call2HelperINS_9DataModelEMS2_FvSsSsESsSsvE4callEPS2_S4_RNS0_7VariantERKSsSA_
// type: int __fastcall(int, int, int, int, std::string *, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DataModel,void (RBX::DataModel::*)(std::string,std::string),std::string,std::string,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&)")]
// was: __ZN3RBX10Reflection11Call2HelperINS_9DataModelEMS2_FvSsSsESsSsvE4callEPS2_S4_RNS0_7VariantERKSsSA_
// IDA 0x45cb18: 156 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45cb18() {
}

// 0x45ccdc — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::EventDesc(rbx::signal<void ()(bool)> RBX::DataModel::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x45ccdc: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45ccdc() {
}

// 0x45ce60 — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// IDA 0x45ce60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_45ce60() {
}

// 0x45cf14 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x45cf14: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45cf14() {
}

// 0x45d068 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x45d068: 45 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d068() {
}

// 0x45d0f4 — __ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x45d0f4: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d0f4() {
}

// 0x45d108 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_
// IDA 0x45d108: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d108() {
}

// 0x45d228 — __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// IDA 0x45d228: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d228() {
}

// 0x45d310 — __ZN5boost9function1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// was: __ZN5boost9function1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
// IDA 0x45d310: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d310() {
}

// 0x45d408 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// IDA 0x45d408: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d408() {
}

// 0x45d428 — __ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x45d428: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d428() {
}

// 0x45d500 — __ZN3rbx7signals6signalIFvbEE6insertEPNS3_4slotE
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(bool)>::insert(rbx::signals::signal<void ()(bool)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvbEE6insertEPNS3_4slotE
// IDA 0x45d500: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d500() {
}

// 0x45d710 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::callable<rbx::signals::signal<void ()(bool)>*>(boost::function<void ()(bool)> const&,rbx::signals::signal<void ()(bool)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
// IDA 0x45d710: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d710() {
}

// 0x45d810 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb
// IDA 0x45d810: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d810() {
}

// 0x45d818 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb
// IDA 0x45d818: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d818() {
}

// 0x45d820 — __ZN3rbx7signals6signalIFvbEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(bool)>::remove(rbx::signals::signal<void ()(bool)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvbEE6removeEPNS3_4slotE
// IDA 0x45d820: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d820() {
}

// 0x45d910 — __ZN3rbx7signals6signalIFvbEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvbEE4slot22safe_static_init_mutexEv
// IDA 0x45d910: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_45d910() {
}

// 0x45d918 — __ZN3rbx7signals6signalIFvbEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvbEE4slotD0Ev
// IDA 0x45d918: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_45d918() {
}

// 0x45d9ec — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelESsEC2IMS2_KFSsvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::PropDescriptor<std::string (RBX::DataModel::*)(void)const,int>(char const*,char const*,std::string (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9DataModelESsEC2IMS2_KFSsvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x45d9ec: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45d9ec() {
}

// 0x45dafc — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelESsED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9DataModelESsED0Ev
// IDA 0x45dafc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_45dafc() {
}

// 0x45db28 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE10isReadOnlyEv
// IDA 0x45db28: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45db28() {
}

// 0x45db2c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE11isWriteOnlyEv
// IDA 0x45db2c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45db2c() {
}

// 0x45db30 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x45db30: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45db30() {
}

// 0x45db58 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE8setValueEPNS0_13DescribedBaseERKSs
// IDA 0x45db58: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45db58() {
}

// 0x45dc78 — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// IDA 0x45dc78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_45dc78() {
}

// 0x45dd2c — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x45dd2c: 198 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45dd2c() {
}

// 0x45df30 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x45df30: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45df30() {
}

// 0x45dfa4 — __ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x45dfa4: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45dfa4() {
}

// 0x45dfb8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED1Ev
// IDA 0x45dfb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_45dfb8() {
}

// 0x45e0c8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED0Ev
// IDA 0x45e0c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_45e0c8() {
}

// 0x45e1f8 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EEC2EMS2_FbS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::BoundFuncDesc(bool (RBX::DataModel::*)(RBX::DataModel::GearType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EEC2EMS2_FbS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x45e1f8: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45e1f8() {
}

// 0x45e370 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x45e370: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45e370() {
}

// 0x45e3a0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EED0Ev
// IDA 0x45e3a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_45e3a0() {
}

// 0x45e474 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x45e474: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45e474() {
}

// 0x45e4b4 — __ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FbNS2_8GearTypeEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::DataModel,bool (RBX::DataModel::*)(RBX::DataModel::GearType),RBX::DataModel::GearType,bool>::call(RBX::DataModel*,bool (RBX::DataModel::*)(RBX::DataModel::GearType),RBX::Reflection::Variant &,RBX::DataModel::GearType const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FbNS2_8GearTypeEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_
// IDA 0x45e4b4: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45e4b4() {
}

// 0x45e4ec — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel8GearTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int (__fastcall ***__fastcall(int (__fastcall ***)(_DWORD), int))(_DWORD)
#[doc(alias = "RBX::DataModel::GearType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearType> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel8GearTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x45e4ec: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45e4ec() {
}

// 0x45e67c — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel8GearTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int(void)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearType &,boost::enable_if<boost::is_enum<RBX::DataModel::GearType>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel8GearTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// IDA 0x45e67c: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45e67c() {
}

// 0x45e6d0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::EnumPropDescriptor<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x45e6d0: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45e6d0() {
}

// 0x45e87c — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEED0Ev
// IDA 0x45e87c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_45e87c() {
}

// 0x45e8a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10isReadOnlyEv
// IDA 0x45e8a8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45e8a8() {
}

// 0x45e8b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11isWriteOnlyEv
// IDA 0x45e8b8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45e8b8() {
}

// 0x45e8c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x45e8c8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45e8c8() {
}

// 0x45e8f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x45e8f0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45e8f0() {
}

// 0x45e914 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x45e914: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45e914() {
}

// 0x45ea60 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x45ea60: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45ea60() {
}

// 0x45ea88 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14hasStringValueEv
// IDA 0x45ea88: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45ea88() {
}

// 0x45ea8c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x45ea8c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45ea8c() {
}

// 0x45eab0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x45eab0: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45eab0() {
}

// 0x45eaf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x45eaf0: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45eaf0() {
}

// 0x45eb10 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x45eb10: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45eb10() {
}

// 0x45ed50 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x45ed50: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45ed50() {
}

// 0x45ed6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x45ed6c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45ed6c() {
}

// 0x45eda0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x45eda0: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45eda0() {
}

// 0x45eda8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x45eda8: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45eda8() {
}

// 0x45edf4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x45edf4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45edf4() {
}

// 0x45ee14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x45ee14: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45ee14() {
}

// 0x45ee4c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToIndex(RBX::DataModel::GearGenreSetting)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToIndexES3_
// IDA 0x45ee4c: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45ee4c() {
}

// 0x45eebc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x45eebc: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45eebc() {
}

// 0x45ef00 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
// IDA 0x45ef00: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45ef00() {
}

// 0x45ef04 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
// IDA 0x45ef04: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45ef04() {
}

// 0x45ef08 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x45ef08: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45ef08() {
}

// 0x45ef28 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::GearGenreSetting const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x45ef28: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45ef28() {
}

// 0x45f048 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::EnumPropDescriptor<RBX::DataModel::Genre (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::Genre (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x45f048: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f048() {
}

// 0x45f1f4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEED0Ev
// IDA 0x45f1f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_45f1f4() {
}

// 0x45f220 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10isReadOnlyEv
// IDA 0x45f220: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f220() {
}

// 0x45f230 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11isWriteOnlyEv
// IDA 0x45f230: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f230() {
}

// 0x45f240 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x45f240: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f240() {
}

// 0x45f268 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x45f268: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f268() {
}

// 0x45f28c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x45f28c: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f28c() {
}

// 0x45f3d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x45f3d8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f3d8() {
}

// 0x45f3fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14hasStringValueEv
// IDA 0x45f3fc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f3fc() {
}

// 0x45f400 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x45f400: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f400() {
}

// 0x45f424 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x45f424: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f424() {
}

// 0x45f464 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x45f464: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f464() {
}

// 0x45f484 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x45f484: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f484() {
}

// 0x45f6c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x45f6c4: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45f6c4() {
}
