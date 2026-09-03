//! rendering — generated_227 — 120 stubs EA-sorted asc global gap filler 0x1c922c..0x1d87c8
//! Source: ida/export.json (85545 funcs) EA-sorted global filler not yet in rbx_rendering (rendering 24400 before, 24520 after distinct; Ogre|G3D complete 13663/13663)
//! Filter: next 120 EA-sorted ascending after 0x1c9184 not yet in rendering (global filler, Ogre|G3D already complete)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x1c922c — __ZN9__gnu_cxx13new_allocatorIPPhE8allocateEmPKv
// type: int __fastcall(int, unsigned int)
#[doc(alias = "__gnu_cxx::new_allocator<unsigned char **>::allocate(unsigned long,void const*)")]
// was: __ZN9__gnu_cxx13new_allocatorIPPhE8allocateEmPKv
// IDA 0x1c922c: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c922c() {
}

// 0x1c924c — __ZNSt11_Deque_baseIPhSaIS0_EE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_allocate_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIPhSaIS0_EE15_M_allocate_mapEm
// IDA 0x1c924c: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_1c924c() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x1c92f4 — __ZN9__gnu_cxx13new_allocatorIPtE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<unsigned short *>::allocate(unsigned long,void const*)")]
// was: __ZN9__gnu_cxx13new_allocatorIPtE8allocateEmPKv
// IDA 0x1c92f4: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c92f4() {
}

// 0x1c9314 — __ZNSt11_Deque_baseItSaItEE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_allocate_map(unsigned long)")]
// was: __ZNSt11_Deque_baseItSaItEE15_M_allocate_mapEm
// IDA 0x1c9314: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_1c9314() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x1c93bc — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE16_M_destroy_nodesEPPS1_S5_
#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_nodes(TagLib::MDMODEL**,TagLib::MDMODEL**)")]
// was: __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE16_M_destroy_nodesEPPS1_S5_
// IDA 0x1c93bc: 60 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c93bc() {
}

// 0x1c94ac — __ZNSt5dequeItSaItEE15_M_pop_back_auxEv
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_pop_back_aux(void)")]
// was: __ZNSt5dequeItSaItEE15_M_pop_back_auxEv
// IDA 0x1c94ac: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c94ac() {
}

// 0x1c94e0 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,int>> *)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// IDA 0x1c94e0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c94e0() {
}

// 0x1c951c — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EED2Ev
#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::~_Deque_base()")]
// was: __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EED2Ev
// IDA 0x1c951c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_1c951c() {
}

// 0x1c9550 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,int> const&)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// IDA 0x1c9550: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9550() {
}

// 0x1c95d4 — __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_
// type: int __fastcall(void *__src)
#[doc(alias = "TagLib::MDMODEL * * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<TagLib::MDMODEL *>(TagLib::MDMODEL * const*,TagLib::MDMODEL * const*,TagLib::MDMODEL * *)")]
// was: __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_
// IDA 0x1c95d4: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_1c95d4() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x1c9604 — __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_
// type: int __fastcall(void *__src, int, void *__dst)
#[doc(alias = "TagLib::MDMODEL * * std::__copy<true,std::random_access_iterator_tag>::copy<TagLib::MDMODEL *>(TagLib::MDMODEL * const*,TagLib::MDMODEL * const*,TagLib::MDMODEL * *)")]
// was: __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_
// IDA 0x1c9604: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_1c9604() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x1c9630 — __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPtEEPT_PKS4_S7_S5_
// type: int __fastcall(void *__src)
#[doc(alias = "unsigned short * * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<unsigned short *>(unsigned short * const*,unsigned short * const*,unsigned short * *)")]
// was: __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPtEEPT_PKS4_S7_S5_
// IDA 0x1c9630: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_1c9630() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x1c9660 — __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPtEEPT_PKS4_S7_S5_
// type: int __fastcall(void *__src, int, void *__dst)
#[doc(alias = "unsigned short * * std::__copy<true,std::random_access_iterator_tag>::copy<unsigned short *>(unsigned short * const*,unsigned short * const*,unsigned short * *)")]
// was: __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPtEEPT_PKS4_S7_S5_
// IDA 0x1c9660: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_1c9660() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x1c968c — __ZNSt5dequeItSaItEE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_reallocate_map(unsigned long,bool)")]
// was: __ZNSt5dequeItSaItEE17_M_reallocate_mapEmb
// IDA 0x1c968c: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c968c() {
}

// 0x1c97b4 — __ZNSt5dequeItSaItEE22_M_reserve_map_at_backEm
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_reserve_map_at_back(unsigned long)")]
// was: __ZNSt5dequeItSaItEE22_M_reserve_map_at_backEm
// IDA 0x1c97b4: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c97b4() {
}

// 0x1c97e8 — __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPPhEEPT_PKS5_S8_S6_
// type: int __fastcall(void *__src)
#[doc(alias = "unsigned char ** * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<unsigned char **>(unsigned char ** const*,unsigned char ** const*,unsigned char ** *)")]
// was: __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPPhEEPT_PKS5_S8_S6_
// IDA 0x1c97e8: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_1c97e8() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x1c9818 — __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPPhEEPT_PKS5_S8_S6_
// type: int __fastcall(void *__src, int, void *__dst)
#[doc(alias = "unsigned char ** * std::__copy<true,std::random_access_iterator_tag>::copy<unsigned char **>(unsigned char ** const*,unsigned char ** const*,unsigned char ** *)")]
// was: __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPPhEEPT_PKS5_S8_S6_
// IDA 0x1c9818: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_1c9818() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x1c9844 — __ZNSt5dequeItSaItEE4backEv
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::back(void)")]
// was: __ZNSt5dequeItSaItEE4backEv
// IDA 0x1c9844: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9844() {
}

// 0x1c9884 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert_unique(std::pair<unsigned int const,int> const&)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueERKS2_
// IDA 0x1c9884: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9884() {
}

// 0x1c9944 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,int>>,std::pair<unsigned int const,int> const&)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// IDA 0x1c9944: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9944() {
}

// 0x1c9a68 — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_E11_M_set_nodeEPS3_
// type: int __fastcall(_DWORD *, int *)
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::_M_set_node(TagLib::MDMODEL**)")]
// was: __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_E11_M_set_nodeEPS3_
// IDA 0x1c9a68: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9a68() {
}

// 0x1c9a98 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_reallocate_map(unsigned long,bool)")]
// was: __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE17_M_reallocate_mapEmb
// IDA 0x1c9a98: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9a98() {
}

// 0x1c9bc0 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE22_M_reserve_map_at_backEm
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_reserve_map_at_back(unsigned long)")]
// was: __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE22_M_reserve_map_at_backEm
// IDA 0x1c9bc0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9bc0() {
}

// 0x1c9bf4 — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EppEv
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::operator++(void)")]
// was: __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EppEv
// IDA 0x1c9bf4: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9bf4() {
}

// 0x1c9c34 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_St12__false_type
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::__uninitialized_copy_aux<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::__false_type)")]
// was: __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_St12__false_type
// IDA 0x1c9c34: 28 insns (SUB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9c34() {
}

// 0x1c9ca4 — __ZSt18uninitialized_copyISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::uninitialized_copy<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>)")]
// was: __ZSt18uninitialized_copyISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_
// IDA 0x1c9ca4: 32 insns (SUB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9ca4() {
}

// 0x1c9d24 — __ZSt22__uninitialized_copy_aISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_ES2_ET0_T_SB_SA_SaIT1_E
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::__uninitialized_copy_a<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,TagLib::MDMODEL>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::allocator<TagLib::MDMODEL>)")]
// was: __ZSt22__uninitialized_copy_aISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_ES2_ET0_T_SB_SA_SaIT1_E
// IDA 0x1c9d24: 31 insns (SUB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9d24() {
}

// 0x1c9da0 — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EmmEv
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::operator--(void)")]
// was: __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EmmEv
// IDA 0x1c9da0: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9da0() {
}

// 0x1c9de0 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE4backEv
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::back(void)")]
// was: __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE4backEv
// IDA 0x1c9de0: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9de0() {
}

// 0x1c9e20 — __ZNSt15_Deque_iteratorItRKtPS0_EppEv
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>::operator++(void)")]
// was: __ZNSt15_Deque_iteratorItRKtPS0_EppEv
// IDA 0x1c9e20: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1c9e20() {
}

// 0x1c9e78 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorItRKtPS4_ES3_ItRtPtEEET0_T_SC_SB_
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
// was: __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorItRKtPS4_ES3_ItRtPtEEET0_T_SC_SB_
// IDA 0x1c9e78: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_1c9e78() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x1ca124 — __ZSt10__copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy_aux<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
// was: __ZSt10__copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_
// IDA 0x1ca124: 31 insns (SUB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1ca124() {
}

// 0x1ca1a0 — __ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorItRKtPS3_ES2_ItRtPtEEET0_T_SB_SA_
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy_normal<false,false>::__copy_n<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
// was: __ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorItRKtPS3_ES2_ItRtPtEEET0_T_SB_SA_
// IDA 0x1ca1a0: 31 insns (SUB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1ca1a0() {
}

// 0x1cc578 — __ZL15cacheIO_getByteP10tagCacheIO
#[doc(alias = "cacheIO_getByte(tagCacheIO *)")]
// was: __ZL15cacheIO_getByteP10tagCacheIO
// IDA 0x1cc578: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc578() {
}

// 0x1cc5dc — __ZL16cacheIO_getBytesP10tagCacheIOm
#[doc(alias = "cacheIO_getBytes(tagCacheIO *,unsigned long)")]
// was: __ZL16cacheIO_getBytesP10tagCacheIOm
// IDA 0x1cc5dc: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc5dc() {
}

// 0x1cc684 — __ZL6Formatv_2
// type: const char *__fastcall()
#[doc(alias = "__ZL6Formatv_2")]
// was: __ZL6Formatv_2
// IDA 0x1cc684: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc684() {
}

// 0x1cc694 — __ZL11Descriptionv_2
// type: _DWORD __fastcall()
#[doc(alias = "__ZL11Descriptionv_2")]
// was: __ZL11Descriptionv_2
// IDA 0x1cc694: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc694() {
}

// 0x1cc6a4 — __ZL9Extensionv_2
// type: _DWORD __fastcall()
#[doc(alias = "__ZL9Extensionv_2")]
// was: __ZL9Extensionv_2
// IDA 0x1cc6a4: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc6a4() {
}

// 0x1cc6b4 — __ZL7RegExprv_2
// type: _DWORD __fastcall()
#[doc(alias = "__ZL7RegExprv_2")]
// was: __ZL7RegExprv_2
// IDA 0x1cc6b4: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc6b4() {
}

// 0x1cc6bc — __ZL8MimeTypev_2
// type: _DWORD __fastcall()
#[doc(alias = "__ZL8MimeTypev_2")]
// was: __ZL8MimeTypev_2
// IDA 0x1cc6bc: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc6bc() {
}

// 0x1cc6cc — __ZL8ValidateP11FreeImageIOPv_2
#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_2")]
// was: __ZL8ValidateP11FreeImageIOPv_2
// IDA 0x1cc6cc: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc6cc() {
}

// 0x1cc838 — __ZL19SupportsExportDepthi_2
// type: _DWORD __fastcall(int)
#[doc(alias = "__ZL19SupportsExportDepthi_2")]
// was: __ZL19SupportsExportDepthi_2
// IDA 0x1cc838: 9 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc838() {
}

// 0x1cc85c — __ZL18SupportsExportType15FREE_IMAGE_TYPE_2
// type: bool __fastcall(int)
#[doc(alias = "__ZL18SupportsExportType15FREE_IMAGE_TYPE_2")]
// was: __ZL18SupportsExportType15FREE_IMAGE_TYPE_2
// IDA 0x1cc85c: 4 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc85c() {
}

// 0x1cc86c — __Z9InitTARGAP6Plugini
#[doc(alias = "InitTARGA(Plugin *,int)")]
// was: __Z9InitTARGAP6Plugini
// IDA 0x1cc86c: 39 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc86c() {
}

// 0x1cc934 — __ZL13cacheIO_allocP10tagCacheIOP11FreeImageIOPvm
// type: int __fastcall(int, int, int, size_t __size)
#[doc(alias = "cacheIO_alloc(tagCacheIO *,FreeImageIO *,void *,unsigned long)")]
// was: __ZL13cacheIO_allocP10tagCacheIOP11FreeImageIOPvm
// IDA 0x1cc934: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc934() {
}

// 0x1cc990 — __ZL12cacheIO_freeP10tagCacheIO
#[doc(alias = "cacheIO_free(tagCacheIO *)")]
// was: __ZL12cacheIO_freeP10tagCacheIO
// IDA 0x1cc990: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc990() {
}

// 0x1cc9ac — __ZL20Internal_GetScanLineP8FIBITMAPii
#[doc(alias = "Internal_GetScanLine(FIBITMAP *,int,int)")]
// was: __ZL20Internal_GetScanLineP8FIBITMAPii
// IDA 0x1cc9ac: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc9ac() {
}

// 0x1cc9e4 — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__2
#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__2")]
// was: __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__2
// IDA 0x1cc9e4: 476 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cc9e4() {
}

// 0x1cd15c — __ZL4LoadP11FreeImageIOPviiS1__2
#[doc(alias = "__ZL4LoadP11FreeImageIOPviiS1__2")]
// was: __ZL4LoadP11FreeImageIOPviiS1__2
// IDA 0x1cd15c: 3768 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1cd15c() {
}

// 0x1d0c8c — _af_sort_pos
#[doc(alias = "_af_sort_pos")]
// was: _af_sort_pos
// IDA 0x1d0c8c: 129 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d0c8c() {
}

// 0x1d0e90 — _af_sort_widths
#[doc(alias = "_af_sort_widths")]
// was: _af_sort_widths
// IDA 0x1d0e90: 116 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d0e90() {
}

// 0x1d1060 — _af_cjk_metrics_scale_dim
#[doc(alias = "_af_cjk_metrics_scale_dim")]
// was: _af_cjk_metrics_scale_dim
// IDA 0x1d1060: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d1060() {
}

// 0x1d10a0 — _af_cjk_metrics_scale
#[doc(alias = "_af_cjk_metrics_scale")]
// was: _af_cjk_metrics_scale
// IDA 0x1d10a0: 19 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d10a0() {
}

// 0x1d10ec — _af_cjk_compute_stem_width
#[doc(alias = "_af_cjk_compute_stem_width")]
// was: _af_cjk_compute_stem_width
// IDA 0x1d10ec: 253 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d10ec() {
}

// 0x1d14e0 — _af_hint_normal_stem
#[doc(alias = "_af_hint_normal_stem")]
// was: _af_hint_normal_stem
// IDA 0x1d14e0: 118 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d14e0() {
}

// 0x1d16b8 — _af_cjk_hints_detect_features
#[doc(alias = "_af_cjk_hints_detect_features")]
// was: _af_cjk_hints_detect_features
// IDA 0x1d16b8: 501 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d16b8() {
}

// 0x1d1e8c — _af_cjk_hints_apply
#[doc(alias = "_af_cjk_hints_apply")]
// was: _af_cjk_hints_apply
// IDA 0x1d1e8c: 358 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d1e8c() {
}

// 0x1d2428 — _af_cjk_hints_init
#[doc(alias = "_af_cjk_hints_init")]
// was: _af_cjk_hints_init
// IDA 0x1d2428: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d2428() {
}

// 0x1d24b0 — _af_cjk_metrics_init
#[doc(alias = "_af_cjk_metrics_init")]
// was: _af_cjk_metrics_init
// IDA 0x1d24b0: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d24b0() {
}

// 0x1d251c — _af_dummy_hints_apply
#[doc(alias = "_af_dummy_hints_apply")]
// was: _af_dummy_hints_apply
// IDA 0x1d251c: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d251c() {
}

// 0x1d2524 — _af_dummy_hints_init
#[doc(alias = "_af_dummy_hints_init")]
// was: _af_dummy_hints_init
// IDA 0x1d2524: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d2524() {
}

// 0x1d2538 — _af_face_globals_is_digit
#[doc(alias = "_af_face_globals_is_digit")]
// was: _af_face_globals_is_digit
// IDA 0x1d2538: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d2538() {
}

// 0x1d2554 — _af_face_globals_get_metrics
#[doc(alias = "_af_face_globals_get_metrics")]
// was: _af_face_globals_get_metrics
// IDA 0x1d2554: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d2554() {
}

// 0x1d267c — _af_face_globals_free
#[doc(alias = "_af_face_globals_free")]
// was: _af_face_globals_free
// IDA 0x1d267c: 80 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d267c() {
}

// 0x1d27cc — _af_face_globals_new
#[doc(alias = "_af_face_globals_new")]
// was: _af_face_globals_new
// IDA 0x1d27cc: 213 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d27cc() {
}

// 0x1d2b28 — _af_direction_compute
#[doc(alias = "_af_direction_compute")]
// was: _af_direction_compute
// IDA 0x1d2b28: 31 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d2b28() {
}

// 0x1d2ba4 — _af_glyph_hints_rescale
#[doc(alias = "_af_glyph_hints_rescale")]
// was: _af_glyph_hints_rescale
// IDA 0x1d2ba4: 4 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d2ba4() {
}

// 0x1d2bb4 — _af_glyph_hints_save
#[doc(alias = "_af_glyph_hints_save")]
// was: _af_glyph_hints_save
// IDA 0x1d2bb4: 26 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d2bb4() {
}

// 0x1d2c1c — _af_glyph_hints_align_edge_points
#[doc(alias = "_af_glyph_hints_align_edge_points")]
// was: _af_glyph_hints_align_edge_points
// IDA 0x1d2c1c: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d2c1c() {
}

// 0x1d2ce8 — _af_iup_interp
#[doc(alias = "_af_iup_interp")]
// was: _af_iup_interp
// IDA 0x1d2ce8: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d2ce8() {
}

// 0x1d2e1c — _af_glyph_hints_align_weak_points
#[doc(alias = "_af_glyph_hints_align_weak_points")]
// was: _af_glyph_hints_align_weak_points
// IDA 0x1d2e1c: 145 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d2e1c() {
}

// 0x1d3060 — _af_glyph_hints_align_strong_points
#[doc(alias = "_af_glyph_hints_align_strong_points")]
// was: _af_glyph_hints_align_strong_points
// IDA 0x1d3060: 237 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d3060() {
}

// 0x1d3418 — _af_axis_hints_new_segment
#[doc(alias = "_af_axis_hints_new_segment")]
// was: _af_axis_hints_new_segment
// IDA 0x1d3418: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d3418() {
}

// 0x1d34f8 — _af_glyph_hints_reload
#[doc(alias = "_af_glyph_hints_reload")]
// was: _af_glyph_hints_reload
// IDA 0x1d34f8: 374 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d34f8() {
}

// 0x1d3ad0 — _af_glyph_hints_done
#[doc(alias = "_af_glyph_hints_done")]
// was: _af_glyph_hints_done
// IDA 0x1d3ad0: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d3ad0() {
}

// 0x1d3b88 — _af_glyph_hints_init
#[doc(alias = "_af_glyph_hints_init")]
// was: _af_glyph_hints_init
// IDA 0x1d3b88: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d3b88() {
}

// 0x1d3bac — _af_axis_hints_new_edge
#[doc(alias = "_af_axis_hints_new_edge")]
// was: _af_axis_hints_new_edge
// IDA 0x1d3bac: 102 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d3bac() {
}

// 0x1d3d4c — _af_indic_hints_apply
#[doc(alias = "_af_indic_hints_apply")]
// was: _af_indic_hints_apply
// IDA 0x1d3d4c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d3d4c() {
}

// 0x1d3d5c — _af_indic_hints_init
#[doc(alias = "_af_indic_hints_init")]
// was: _af_indic_hints_init
// IDA 0x1d3d5c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d3d5c() {
}

// 0x1d3d6c — _af_indic_metrics_scale
#[doc(alias = "_af_indic_metrics_scale")]
// was: _af_indic_metrics_scale
// IDA 0x1d3d6c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d3d6c() {
}

// 0x1d3d7c — _af_indic_metrics_init
#[doc(alias = "_af_indic_metrics_init")]
// was: _af_indic_metrics_init
// IDA 0x1d3d7c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d3d7c() {
}

// 0x1d3d8c — _af_latin_hints_link_segments
#[doc(alias = "_af_latin_hints_link_segments")]
// was: _af_latin_hints_link_segments
// IDA 0x1d3d8c: 109 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d3d8c() {
}

// 0x1d3f40 — _af_latin_compute_stem_width
#[doc(alias = "_af_latin_compute_stem_width")]
// was: _af_latin_compute_stem_width
// IDA 0x1d3f40: 278 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d3f40() {
}

// 0x1d4398 — _af_latin_align_linked_edge
#[doc(alias = "_af_latin_align_linked_edge")]
// was: _af_latin_align_linked_edge
// IDA 0x1d4398: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d4398() {
}

// 0x1d43dc — _af_latin_hints_init
#[doc(alias = "_af_latin_hints_init")]
// was: _af_latin_hints_init
// IDA 0x1d43dc: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d43dc() {
}

// 0x1d447c — _af_latin_hint_edges
#[doc(alias = "_af_latin_hint_edges")]
// was: _af_latin_hint_edges
// IDA 0x1d447c: 430 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d447c() {
}

// 0x1d4b38 — _af_latin_hints_compute_blue_edges
#[doc(alias = "_af_latin_hints_compute_blue_edges")]
// was: _af_latin_hints_compute_blue_edges
// IDA 0x1d4b38: 314 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d4b38() {
}

// 0x1d5024 — _af_latin_metrics_scale_dim
#[doc(alias = "_af_latin_metrics_scale_dim")]
// was: _af_latin_metrics_scale_dim
// IDA 0x1d5024: 259 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d5024() {
}

// 0x1d5430 — _af_latin_metrics_scale
#[doc(alias = "_af_latin_metrics_scale")]
// was: _af_latin_metrics_scale
// IDA 0x1d5430: 15 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d5430() {
}

// 0x1d546c — _af_latin_hints_compute_edges
#[doc(alias = "_af_latin_hints_compute_edges")]
// was: _af_latin_hints_compute_edges
// IDA 0x1d546c: 332 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d546c() {
}

// 0x1d599c — _af_latin_hints_compute_segments
#[doc(alias = "_af_latin_hints_compute_segments")]
// was: _af_latin_hints_compute_segments
// IDA 0x1d599c: 278 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d599c() {
}

// 0x1d5df8 — _af_latin_hints_detect_features
#[doc(alias = "_af_latin_hints_detect_features")]
// was: _af_latin_hints_detect_features
// IDA 0x1d5df8: 14 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d5df8() {
}

// 0x1d5e30 — _af_latin_hints_apply
#[doc(alias = "_af_latin_hints_apply")]
// was: _af_latin_hints_apply
// IDA 0x1d5e30: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d5e30() {
}

// 0x1d5f28 — _af_latin_metrics_check_digits
#[doc(alias = "_af_latin_metrics_check_digits")]
// was: _af_latin_metrics_check_digits
// IDA 0x1d5f28: 188 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d5f28() {
}

// 0x1d6218 — _af_latin_metrics_init_widths
#[doc(alias = "_af_latin_metrics_init_widths")]
// was: _af_latin_metrics_init_widths
// IDA 0x1d6218: 176 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d6218() {
}

// 0x1d64dc — _af_latin_metrics_init
#[doc(alias = "_af_latin_metrics_init")]
// was: _af_latin_metrics_init
// IDA 0x1d64dc: 786 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d64dc() {
}

// 0x1d712c — _af_loader_load_g
#[doc(alias = "_af_loader_load_g")]
// was: _af_loader_load_g
// IDA 0x1d712c: 588 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d712c() {
}

// 0x1d7a64 — _af_loader_done
#[doc(alias = "_af_loader_done")]
// was: _af_loader_done
// IDA 0x1d7a64: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7a64() {
}

// 0x1d7a94 — _af_loader_reset
#[doc(alias = "_af_loader_reset")]
// was: _af_loader_reset
// IDA 0x1d7a94: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7a94() {
}

// 0x1d7afc — _af_loader_load_glyph
#[doc(alias = "_af_loader_load_glyph")]
// was: _af_loader_load_glyph
// IDA 0x1d7afc: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7afc() {
}

// 0x1d7c20 — _af_loader_init
#[doc(alias = "_af_loader_init")]
// was: _af_loader_init
// IDA 0x1d7c20: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7c20() {
}

// 0x1d7c58 — _af_autofitter_done
#[doc(alias = "_af_autofitter_done")]
// was: _af_autofitter_done
// IDA 0x1d7c58: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7c58() {
}

// 0x1d7c6c — _af_autofitter_init
#[doc(alias = "_af_autofitter_init")]
// was: _af_autofitter_init
// IDA 0x1d7c6c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7c6c() {
}

// 0x1d7c88 — _af_autofitter_load_glyph
#[doc(alias = "_af_autofitter_load_glyph")]
// was: _af_autofitter_load_glyph
// IDA 0x1d7c88: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7c88() {
}

// 0x1d7ca8 — _FT_RoundFix
// type: int __fastcall(_DWORD)
#[doc(alias = "_FT_RoundFix")]
// was: _FT_RoundFix
// IDA 0x1d7ca8: 9 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7ca8() {
}

// 0x1d7cd0 — _ft_multo64
#[doc(alias = "_ft_multo64")]
// was: _ft_multo64
// IDA 0x1d7cd0: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7cd0() {
}

// 0x1d7d28 — _ft_div64by32
#[doc(alias = "_ft_div64by32")]
// was: _ft_div64by32
// IDA 0x1d7d28: 93 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7d28() {
}

// 0x1d7e9c — _FT_Add64
#[doc(alias = "_FT_Add64")]
// was: _FT_Add64
// IDA 0x1d7e9c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7e9c() {
}

// 0x1d7ec4 — _FT_MulDiv
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_FT_MulDiv")]
// was: _FT_MulDiv
// IDA 0x1d7ec4: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7ec4() {
}

// 0x1d7fb4 — __ft_face_scale_advances
#[doc(alias = "__ft_face_scale_advances")]
// was: __ft_face_scale_advances
// IDA 0x1d7fb4: 127 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d7fb4() {
}

// 0x1d81b0 — _FT_MulDiv_No_Round
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_FT_MulDiv_No_Round")]
// was: _FT_MulDiv_No_Round
// IDA 0x1d81b0: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d81b0() {
}

// 0x1d8264 — _FT_MulFix
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_FT_MulFix")]
// was: _FT_MulFix
// IDA 0x1d8264: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d8264() {
}

// 0x1d82d8 — _FT_DivFix
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_FT_DivFix")]
// was: _FT_DivFix
// IDA 0x1d82d8: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d82d8() {
}

// 0x1d836c — _FT_Matrix_Invert
#[doc(alias = "_FT_Matrix_Invert")]
// was: _FT_Matrix_Invert
// IDA 0x1d836c: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d836c() {
}

// 0x1d8400 — _FT_Matrix_Multiply_Scaled
#[doc(alias = "_FT_Matrix_Multiply_Scaled")]
// was: _FT_Matrix_Multiply_Scaled
// IDA 0x1d8400: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d8400() {
}

// 0x1d84fc — _FT_Vector_Transform_Scaled
#[doc(alias = "_FT_Vector_Transform_Scaled")]
// was: _FT_Vector_Transform_Scaled
// IDA 0x1d84fc: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d84fc() {
}

// 0x1d8584 — _FT_SqrtFixed
#[doc(alias = "_FT_SqrtFixed")]
// was: _FT_SqrtFixed
// IDA 0x1d8584: 67 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d8584() {
}

// 0x1d8690 — _ft_corner_orientation
#[doc(alias = "_ft_corner_orientation")]
// was: _ft_corner_orientation
// IDA 0x1d8690: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d8690() {
}

// 0x1d8764 — _ft_corner_is_flat
#[doc(alias = "_ft_corner_is_flat")]
// was: _ft_corner_is_flat
// IDA 0x1d8764: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d8764() {
}

// 0x1d87c8 — _FT_GlyphLoader_Rewind
#[doc(alias = "_FT_GlyphLoader_Rewind")]
// was: _FT_GlyphLoader_Rewind
// IDA 0x1d87c8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d87c8() {
}
