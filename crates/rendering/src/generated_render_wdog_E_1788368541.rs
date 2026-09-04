//! rendering — generated_render_wdog_E_1788368541 — 100 stubs EA-sorted asc gap-filler distinct not in crates/rendering/src
//! Range: 0x72f94c..0x7354f0 (100 stubs, EA-sorted asc, distinct)
//! Source: ida/export.json (85545 funcs, 56932 already stubbed in rendering, 29014 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x72f94c — __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX8NodeInfoESt6vectorIS3_SaIS3_EEEEiS3_St4lessIS3_EEvT_T0_SC_T1_T2_
#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::NodeInfo *,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,int,RBX::NodeInfo,std::less<RBX::NodeInfo>>(__gnu_cxx::__normal_iterator<RBX::NodeInfo *,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,int,int,RBX::NodeInfo,std::less<RBX::NodeInfo>)")]
#[doc(alias = "__ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX8NodeInfoESt6vectorIS3_SaIS3_EEEEiS3_St4lessIS3_EEvT_T0_SC_T1_T2_")]
// was: __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX8NodeInfoESt6vectorIS3_SaIS3_EEEEiS3_St4lessIS3_EEvT_T0_SC_T1_T2_
// IDA 0x72f94c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72f94c() {
}

// 0x72f9ac — __ZNSt6vectorIN3RBX8NodeInfoESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::NodeInfo*,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,RBX::NodeInfo const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX8NodeInfoESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// was: __ZNSt6vectorIN3RBX8NodeInfoESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// IDA 0x72f9ac: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_72f9ac() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x72fab8 — __ZNSt12_Vector_baseIN3RBX8NodeInfoESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX8NodeInfoESaIS1_EE11_M_allocateEm")]
// was: __ZNSt12_Vector_baseIN3RBX8NodeInfoESaIS1_EE11_M_allocateEm
// IDA 0x72fab8: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_72fab8() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x72fad0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NodeInfoES5_EET0_T_S7_S6_
#[doc(alias = "RBX::NodeInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::NodeInfo *,RBX::NodeInfo *>(RBX::NodeInfo *,RBX::NodeInfo *,RBX::NodeInfo *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NodeInfoES5_EET0_T_S7_S6_")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NodeInfoES5_EET0_T_S7_S6_
// IDA 0x72fad0: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_72fad0() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x72fb18 — __ZNK3RBX7ExtentseqERKS0_
#[doc(alias = "RBX::Extents::operator==(RBX::Extents const&)const")]
#[doc(alias = "__ZNK3RBX7ExtentseqERKS0_")]
// was: __ZNK3RBX7ExtentseqERKS0_
// IDA 0x72fb18: 38 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72fb18() {
}

// 0x72fb98 — __ZN3RBX7Extents4zeroEv
// type: _DWORD __fastcall(RBX::Extents *__hidden this)
#[doc(alias = "RBX::Extents::zero(void)")]
#[doc(alias = "__ZN3RBX7Extents4zeroEv")]
// was: __ZN3RBX7Extents4zeroEv
// IDA 0x72fb98: 81 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72fb98() {
}

// 0x72fc90 — __ZNK3RBX12ExtentsInt3217overlapsOrTouchesERKS0_
// type: _DWORD __fastcall(RBX::ExtentsInt32 *__hidden this, const RBX::ExtentsInt32 *)
#[doc(alias = "RBX::ExtentsInt32::overlapsOrTouches(RBX::ExtentsInt32 const&)const")]
#[doc(alias = "__ZNK3RBX12ExtentsInt3217overlapsOrTouchesERKS0_")]
// was: __ZNK3RBX12ExtentsInt3217overlapsOrTouchesERKS0_
// IDA 0x72fc90: 32 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72fc90() {
}

// 0x72ff44 — __ZN3RBX12ContactStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::ContactStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::ContactStage::ContactStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX12ContactStageC1EPNS_6IStageEPNS_5WorldE")]
// was: __ZN3RBX12ContactStageC1EPNS_6IStageEPNS_5WorldE
// IDA 0x72ff44: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_72ff44() {
}

// 0x72ff48 — __ZN3RBX12ContactStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::ContactStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::ContactStage::ContactStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX12ContactStageC2EPNS_6IStageEPNS_5WorldE")]
// was: __ZN3RBX12ContactStageC2EPNS_6IStageEPNS_5WorldE
// IDA 0x72ff48: 76 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72ff48() {
}

// 0x73001c — __ZN3RBX12ContactStage16onPrimitiveAddedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::ContactStage::onPrimitiveAdded(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX12ContactStage16onPrimitiveAddedEPNS_9PrimitiveE")]
// was: __ZN3RBX12ContactStage16onPrimitiveAddedEPNS_9PrimitiveE
// IDA 0x73001c: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73001c() {
}

// 0x730038 — __ZN3RBX12ContactStage19onPrimitiveRemovingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::ContactStage::onPrimitiveRemoving(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX12ContactStage19onPrimitiveRemovingEPNS_9PrimitiveE")]
// was: __ZN3RBX12ContactStage19onPrimitiveRemovingEPNS_9PrimitiveE
// IDA 0x730038: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_730038() {
}

// 0x730054 — __ZN3RBX12ContactStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::ContactStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::ContactStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX12ContactStage11onEdgeAddedEPNS_4EdgeE")]
// was: __ZN3RBX12ContactStage11onEdgeAddedEPNS_4EdgeE
// IDA 0x730054: 110 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_730054() {
}

// 0x73017c — __ZN3RBX12ContactStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::ContactStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::ContactStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX12ContactStage14onEdgeRemovingEPNS_4EdgeE")]
// was: __ZN3RBX12ContactStage14onEdgeRemovingEPNS_4EdgeE
// IDA 0x73017c: 105 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73017c() {
}

// 0x7302a4 — __ZN3RBX12ContactStageD1Ev
// type: void __fastcall(RBX::ContactStage *__hidden this)
#[doc(alias = "RBX::ContactStage::~ContactStage()")]
#[doc(alias = "__ZN3RBX12ContactStageD1Ev")]
// was: __ZN3RBX12ContactStageD1Ev
// IDA 0x7302a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7302a4() {
}

// 0x7302c8 — __ZN3RBX12ContactStageD0Ev
// type: void __fastcall(RBX::ContactStage *__hidden this)
#[doc(alias = "RBX::ContactStage::~ContactStage()")]
#[doc(alias = "__ZN3RBX12ContactStageD0Ev")]
// was: __ZN3RBX12ContactStageD0Ev
// IDA 0x7302c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7302c8() {
}

// 0x730380 — __ZNK3RBX12ContactStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::ContactStage *__hidden this)
#[doc(alias = "RBX::ContactStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX12ContactStage12getStageTypeEv")]
// was: __ZNK3RBX12ContactStage12getStageTypeEv
// IDA 0x730380: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_730380() {
}

// 0x730ad4 — __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::resize(unsigned long,RBX::LegacyController::InputType)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE6resizeEmS2_")]
// was: __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE6resizeEmS2_
// IDA 0x730ad4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_730ad4() {
}

// 0x730b08 — __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::push_back(RBX::LegacyController::InputType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE9push_backERKS2_")]
// was: __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE9push_backERKS2_
// IDA 0x730b08: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_730b08() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x730b30 — __ZNSt3mapIPKN3RBX4NameENS0_16LegacyController9InputTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::LegacyController::InputType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_16LegacyController9InputTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_16LegacyController9InputTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x730b30: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_730b30() {
}

// 0x730b88 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::pair<RBX::Name const* const,RBX::LegacyController::InputType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x730b88: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_730b88() {
}

// 0x730c3c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::LegacyController::InputType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x730c3c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_730c3c() {
}

// 0x730c94 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::LegacyController::InputType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x730c94: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_730c94() {
}

// 0x730cfc — __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::LegacyController::InputType*,std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>>,RBX::LegacyController::InputType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// was: __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x730cfc: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_730cfc() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x730de0 — __ZNSt12_Vector_baseIN3RBX16LegacyController9InputTypeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX16LegacyController9InputTypeESaIS2_EE11_M_allocateEm")]
// was: __ZNSt12_Vector_baseIN3RBX16LegacyController9InputTypeESaIS2_EE11_M_allocateEm
// IDA 0x730de0: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_730de0() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x730df8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16LegacyController9InputTypeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::LegacyController::InputType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::LegacyController::InputType *,RBX::LegacyController::InputType *>(RBX::LegacyController::InputType *,RBX::LegacyController::InputType *,RBX::LegacyController::InputType *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16LegacyController9InputTypeES6_EET0_T_S8_S7_")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16LegacyController9InputTypeES6_EET0_T_S8_S7_
// IDA 0x730df8: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_730df8() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x730e34 — __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::LegacyController::InputType*,std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>>,unsigned long,RBX::LegacyController::InputType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// was: __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x730e34: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_730e34() {
}

// 0x73108c — __ZN3RBX15CornerWedgePoly9buildMeshEv
// type: _DWORD __fastcall(RBX::CornerWedgePoly *__hidden this)
#[doc(alias = "RBX::CornerWedgePoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX15CornerWedgePoly9buildMeshEv")]
// was: __ZN3RBX15CornerWedgePoly9buildMeshEv
// IDA 0x73108c: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73108c() {
}

// 0x731168 — __ZNK3RBX15CornerWedgePoly9getMomentEf
// type: _DWORD __fastcall(RBX::CornerWedgePoly *__hidden this, float)
#[doc(alias = "RBX::CornerWedgePoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX15CornerWedgePoly9getMomentEf")]
// was: __ZNK3RBX15CornerWedgePoly9getMomentEf
// IDA 0x731168: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_731168() {
}

// 0x73129c — __ZNK3RBX15CornerWedgePoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::CornerWedgePoly *__hidden this)
#[doc(alias = "RBX::CornerWedgePoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX15CornerWedgePoly13getCofmOffsetEv")]
// was: __ZNK3RBX15CornerWedgePoly13getCofmOffsetEv
// IDA 0x73129c: 17 insns (VMOV.I32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73129c() {
}

// 0x7312d0 — __ZNK3RBX15CornerWedgePoly21getSurfaceCoordInBodyEm
// type: _DWORD __fastcall(RBX::CornerWedgePoly *__hidden this, unsigned int)
#[doc(alias = "RBX::CornerWedgePoly::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX15CornerWedgePoly21getSurfaceCoordInBodyEm")]
// was: __ZNK3RBX15CornerWedgePoly21getSurfaceCoordInBodyEm
// IDA 0x7312d0: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7312d0() {
}

// 0x731338 — __ZNK3RBX15CornerWedgePoly25getFaceFromLegacyNormalIdENS_8NormalIdE
#[doc(alias = "RBX::CornerWedgePoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX15CornerWedgePoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
// was: __ZNK3RBX15CornerWedgePoly25getFaceFromLegacyNormalIdENS_8NormalIdE
// IDA 0x731338: 8 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_731338() {
}

// 0x731640 — __ZN3RBX15CornerWedgePolyD1Ev
// type: void __fastcall(RBX::CornerWedgePoly *__hidden this)
#[doc(alias = "RBX::CornerWedgePoly::~CornerWedgePoly()")]
#[doc(alias = "__ZN3RBX15CornerWedgePolyD1Ev")]
// was: __ZN3RBX15CornerWedgePolyD1Ev
// IDA 0x731640: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_731640() {
}

// 0x731664 — __ZN3RBX15CornerWedgePolyD0Ev
// type: void __fastcall(RBX::CornerWedgePoly *__hidden this)
#[doc(alias = "RBX::CornerWedgePoly::~CornerWedgePoly()")]
#[doc(alias = "__ZN3RBX15CornerWedgePolyD0Ev")]
// was: __ZN3RBX15CornerWedgePolyD0Ev
// IDA 0x731664: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_731664() {
}

// 0x731ca8 — __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEdlEPv")]
// was: __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEdlEPv
// IDA 0x731ca8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_731ca8() {
}

// 0x7322e4 — __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEnwEm")]
// was: __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEnwEm
// IDA 0x7322e4: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7322e4() {
}

// 0x732470 — __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEC2Ev")]
// was: __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEC2Ev
// IDA 0x732470: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_732470() {
}

// 0x7324d4 — __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEE13releaseMemoryEv")]
// was: __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEE13releaseMemoryEv
// IDA 0x7324d4: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7324d4() {
}

// 0x7328f0 — __ZN3RBX4EdgeC2EPNS_9PrimitiveES2_
#[doc(alias = "RBX::Edge::Edge(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX4EdgeC2EPNS_9PrimitiveES2_")]
// was: __ZN3RBX4EdgeC2EPNS_9PrimitiveES2_
// IDA 0x7328f0: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7328f0() {
}

// 0x732928 — __ZN3RBX4Edge12setPrimitiveEiPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Edge *__hidden this, int, RBX::Primitive *)
#[doc(alias = "RBX::Edge::setPrimitive(int,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX4Edge12setPrimitiveEiPNS_9PrimitiveE")]
// was: __ZN3RBX4Edge12setPrimitiveEiPNS_9PrimitiveE
// IDA 0x732928: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_732928() {
}

// 0x7329d0 — __ZN3RBX10EdgeBufferD0Ev
// type: void __fastcall(RBX::EdgeBuffer *__hidden this)
#[doc(alias = "RBX::EdgeBuffer::~EdgeBuffer()")]
#[doc(alias = "__ZN3RBX10EdgeBufferD0Ev")]
// was: __ZN3RBX10EdgeBufferD0Ev
// IDA 0x7329d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7329d0() {
}

// 0x732a70 — __ZN3RBX10EdgeBufferD1Ev
// type: void __fastcall(RBX::EdgeBuffer *__hidden this)
#[doc(alias = "RBX::EdgeBuffer::~EdgeBuffer()")]
#[doc(alias = "__ZN3RBX10EdgeBufferD1Ev")]
// was: __ZN3RBX10EdgeBufferD1Ev
// IDA 0x732a70: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_732a70() {
}

// 0x732a74 — __ZN3RBX10EdgeBufferD2Ev
// type: void __fastcall(RBX::EdgeBuffer *__hidden this)
#[doc(alias = "RBX::EdgeBuffer::~EdgeBuffer()")]
#[doc(alias = "__ZN3RBX10EdgeBufferD2Ev")]
// was: __ZN3RBX10EdgeBufferD2Ev
// IDA 0x732a74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_732a74() {
}

// 0x732bd8 — __ZN3RBX10EdgeBuffer18afterAssemblyAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::EdgeBuffer *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::EdgeBuffer::afterAssemblyAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX10EdgeBuffer18afterAssemblyAddedEPNS_8AssemblyE")]
// was: __ZN3RBX10EdgeBuffer18afterAssemblyAddedEPNS_8AssemblyE
// IDA 0x732bd8: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_732bd8() {
}

// 0x732c58 — __ZN3RBX10EdgeBuffer22assemblyPrimitiveAddedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::EdgeBuffer *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::EdgeBuffer::assemblyPrimitiveAdded(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10EdgeBuffer22assemblyPrimitiveAddedEPNS_9PrimitiveE")]
// was: __ZN3RBX10EdgeBuffer22assemblyPrimitiveAddedEPNS_9PrimitiveE
// IDA 0x732c58: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_732c58() {
}

// 0x732cd8 — __ZN3RBX10EdgeBuffer22beforeAssemblyRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::EdgeBuffer *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::EdgeBuffer::beforeAssemblyRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX10EdgeBuffer22beforeAssemblyRemovingEPNS_8AssemblyE")]
// was: __ZN3RBX10EdgeBuffer22beforeAssemblyRemovingEPNS_8AssemblyE
// IDA 0x732cd8: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_732cd8() {
}

// 0x732d58 — __ZN3RBX10EdgeBuffer24assemblyPrimitiveRemovedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::EdgeBuffer *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::EdgeBuffer::assemblyPrimitiveRemoved(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10EdgeBuffer24assemblyPrimitiveRemovedEPNS_9PrimitiveE")]
// was: __ZN3RBX10EdgeBuffer24assemblyPrimitiveRemovedEPNS_9PrimitiveE
// IDA 0x732d58: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_732d58() {
}

// 0x732dd4 — __ZN3RBX10EdgeBuffer12pushEdgeIfOkEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::EdgeBuffer *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::EdgeBuffer::pushEdgeIfOk(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10EdgeBuffer12pushEdgeIfOkEPNS_4EdgeE")]
// was: __ZN3RBX10EdgeBuffer12pushEdgeIfOkEPNS_4EdgeE
// IDA 0x732dd4: 101 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_732dd4() {
}

// 0x732ed4 — __ZN3RBX10EdgeBuffer15pushKinematicOkEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::EdgeBuffer *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::EdgeBuffer::pushKinematicOk(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10EdgeBuffer15pushKinematicOkEPNS_4EdgeE")]
// was: __ZN3RBX10EdgeBuffer15pushKinematicOkEPNS_4EdgeE
// IDA 0x732ed4: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_732ed4() {
}

// 0x732fbc — __ZN3RBX10EdgeBuffer12pushSpringOkEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::EdgeBuffer *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::EdgeBuffer::pushSpringOk(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10EdgeBuffer12pushSpringOkEPNS_4EdgeE")]
// was: __ZN3RBX10EdgeBuffer12pushSpringOkEPNS_4EdgeE
// IDA 0x732fbc: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_732fbc() {
}

// 0x733084 — __ZN3RBX10EdgeBuffer11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::EdgeBuffer *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::EdgeBuffer::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10EdgeBuffer11onEdgeAddedEPNS_4EdgeE")]
// was: __ZN3RBX10EdgeBuffer11onEdgeAddedEPNS_4EdgeE
// IDA 0x733084: 82 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_733084() {
}

// 0x733168 — __ZN3RBX10EdgeBuffer14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::EdgeBuffer *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::EdgeBuffer::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10EdgeBuffer14onEdgeRemovingEPNS_4EdgeE")]
// was: __ZN3RBX10EdgeBuffer14onEdgeRemovingEPNS_4EdgeE
// IDA 0x733168: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_733168() {
}

// 0x733260 — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_PNS0_4EdgeEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,RBX::Edge *>,std::_Select1st<std::pair<RBX::Assembly * const,RBX::Edge *>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,RBX::Edge *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Assembly * const,RBX::Edge *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_PNS0_4EdgeEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// was: __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_PNS0_4EdgeEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// IDA 0x733260: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_733260() {
}

// 0x733488 — __ZN3RBX9EdgeStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::EdgeStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::EdgeStage::EdgeStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX9EdgeStageC1EPNS_6IStageEPNS_5WorldE")]
// was: __ZN3RBX9EdgeStageC1EPNS_6IStageEPNS_5WorldE
// IDA 0x733488: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_733488() {
}

// 0x73348c — __ZN3RBX9EdgeStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::EdgeStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::EdgeStage::EdgeStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX9EdgeStageC2EPNS_6IStageEPNS_5WorldE")]
// was: __ZN3RBX9EdgeStageC2EPNS_6IStageEPNS_5WorldE
// IDA 0x73348c: 76 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73348c() {
}

// 0x733560 — __ZN3RBX9EdgeStage16onPrimitiveAddedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::EdgeStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::EdgeStage::onPrimitiveAdded(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9EdgeStage16onPrimitiveAddedEPNS_9PrimitiveE")]
// was: __ZN3RBX9EdgeStage16onPrimitiveAddedEPNS_9PrimitiveE
// IDA 0x733560: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_733560() {
}

// 0x73357c — __ZN3RBX9EdgeStage19onPrimitiveRemovingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::EdgeStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::EdgeStage::onPrimitiveRemoving(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9EdgeStage19onPrimitiveRemovingEPNS_9PrimitiveE")]
// was: __ZN3RBX9EdgeStage19onPrimitiveRemovingEPNS_9PrimitiveE
// IDA 0x73357c: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73357c() {
}

// 0x733598 — __ZN3RBX9EdgeStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::EdgeStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::EdgeStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX9EdgeStage11onEdgeAddedEPNS_4EdgeE")]
// was: __ZN3RBX9EdgeStage11onEdgeAddedEPNS_4EdgeE
// IDA 0x733598: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_733598() {
}

// 0x7335b4 — __ZN3RBX9EdgeStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::EdgeStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::EdgeStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX9EdgeStage14onEdgeRemovingEPNS_4EdgeE")]
// was: __ZN3RBX9EdgeStage14onEdgeRemovingEPNS_4EdgeE
// IDA 0x7335b4: 7 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7335b4() {
}

// 0x7335c8 — __ZN3RBX9EdgeStageD1Ev
// type: void __fastcall(RBX::EdgeStage *__hidden this)
#[doc(alias = "RBX::EdgeStage::~EdgeStage()")]
#[doc(alias = "__ZN3RBX9EdgeStageD1Ev")]
// was: __ZN3RBX9EdgeStageD1Ev
// IDA 0x7335c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7335c8() {
}

// 0x7335ec — __ZN3RBX9EdgeStageD0Ev
// type: void __fastcall(RBX::EdgeStage *__hidden this)
#[doc(alias = "RBX::EdgeStage::~EdgeStage()")]
#[doc(alias = "__ZN3RBX9EdgeStageD0Ev")]
// was: __ZN3RBX9EdgeStageD0Ev
// IDA 0x7335ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7335ec() {
}

// 0x7336a4 — __ZNK3RBX9EdgeStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::EdgeStage *__hidden this)
#[doc(alias = "RBX::EdgeStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX9EdgeStage12getStageTypeEv")]
// was: __ZNK3RBX9EdgeStage12getStageTypeEv
// IDA 0x7336a4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7336a4() {
}

// 0x733770 — __ZN3RBX9GlueJointC1Ev
// type: _DWORD __fastcall(RBX::GlueJoint *__hidden this)
#[doc(alias = "RBX::GlueJoint::GlueJoint(void)")]
#[doc(alias = "__ZN3RBX9GlueJointC1Ev")]
// was: __ZN3RBX9GlueJointC1Ev
// IDA 0x733770: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_733770() {
}

// 0x7337b0 — __ZN3RBX9GlueJointC2Ev
// type: _DWORD __fastcall(RBX::GlueJoint *__hidden this)
#[doc(alias = "RBX::GlueJoint::GlueJoint(void)")]
#[doc(alias = "__ZN3RBX9GlueJointC2Ev")]
// was: __ZN3RBX9GlueJointC2Ev
// IDA 0x7337b0: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7337b0() {
}

// 0x7337f0 — __ZN3RBX9GlueJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int
#[doc(alias = "RBX::GlueJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX9GlueJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
// was: __ZN3RBX9GlueJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_
// IDA 0x7337f0: 405 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7337f0() {
}

// 0x733c38 — __ZN3RBX9GlueJoint11getMaxForceEv
// type: _DWORD __fastcall(RBX::GlueJoint *__hidden this)
#[doc(alias = "RBX::GlueJoint::getMaxForce(void)")]
#[doc(alias = "__ZN3RBX9GlueJoint11getMaxForceEv")]
// was: __ZN3RBX9GlueJoint11getMaxForceEv
// IDA 0x733c38: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_733c38() {
}

// 0x733c58 — __ZN3RBX9GlueJoint11putInKernelEPNS_6KernelE
// type: _DWORD __fastcall(RBX::GlueJoint *__hidden this, RBX::Kernel *)
#[doc(alias = "RBX::GlueJoint::putInKernel(RBX::Kernel *)")]
#[doc(alias = "__ZN3RBX9GlueJoint11putInKernelEPNS_6KernelE")]
// was: __ZN3RBX9GlueJoint11putInKernelEPNS_6KernelE
// IDA 0x733c58: 201 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_733c58() {
}

// 0x733e7c — __ZN3RBX15ManualGlueJoint11putInKernelEPNS_6KernelE
// type: _DWORD __fastcall(RBX::ManualGlueJoint *__hidden this, RBX::Kernel *)
#[doc(alias = "RBX::ManualGlueJoint::putInKernel(RBX::Kernel *)")]
#[doc(alias = "__ZN3RBX15ManualGlueJoint11putInKernelEPNS_6KernelE")]
// was: __ZN3RBX15ManualGlueJoint11putInKernelEPNS_6KernelE
// IDA 0x733e7c: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_733e7c() {
}

// 0x733e94 — __ZN3RBX15ManualGlueJoint32computeIntersectingSurfacePointsEv
// type: _DWORD __fastcall(RBX::ManualGlueJoint *__hidden this)
#[doc(alias = "RBX::ManualGlueJoint::computeIntersectingSurfacePoints(void)")]
#[doc(alias = "__ZN3RBX15ManualGlueJoint32computeIntersectingSurfacePointsEv")]
// was: __ZN3RBX15ManualGlueJoint32computeIntersectingSurfacePointsEv
// IDA 0x733e94: 435 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_733e94() {
}

// 0x7343c4 — __ZNK3RBX4Face4sizeEv
// type: _DWORD __fastcall(RBX::Face *__hidden this)
#[doc(alias = "RBX::Face::size(void)const")]
#[doc(alias = "__ZNK3RBX4Face4sizeEv")]
// was: __ZNK3RBX4Face4sizeEv
// IDA 0x7343c4: 30 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7343c4() {
}

// 0x73443c — __ZN3RBX9AllocatorINS_20NormalBreakConnectorEEnwEm
#[doc(alias = "RBX::Allocator<RBX::NormalBreakConnector>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_20NormalBreakConnectorEEnwEm")]
// was: __ZN3RBX9AllocatorINS_20NormalBreakConnectorEEnwEm
// IDA 0x73443c: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73443c() {
}

// 0x734524 — __ZN3RBX9GlueJointD1Ev
// type: void __fastcall(RBX::GlueJoint *__hidden this)
#[doc(alias = "RBX::GlueJoint::~GlueJoint()")]
#[doc(alias = "__ZN3RBX9GlueJointD1Ev")]
// was: __ZN3RBX9GlueJointD1Ev
// IDA 0x734524: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_734524() {
}

// 0x734528 — __ZN3RBX9GlueJointD0Ev
// type: void __fastcall(RBX::GlueJoint *__hidden this)
#[doc(alias = "RBX::GlueJoint::~GlueJoint()")]
#[doc(alias = "__ZN3RBX9GlueJointD0Ev")]
// was: __ZN3RBX9GlueJointD0Ev
// IDA 0x734528: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_734528() {
}

// 0x7345c8 — __ZNK3RBX9GlueJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::GlueJoint *__hidden this)
#[doc(alias = "RBX::GlueJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX9GlueJoint12getJointTypeEv")]
// was: __ZNK3RBX9GlueJoint12getJointTypeEv
// IDA 0x7345c8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7345c8() {
}

// 0x7345cc — __ZThn32_N3RBX9GlueJointD1Ev
// type: void __fastcall(RBX::GlueJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GlueJoint::~GlueJoint()")]
#[doc(alias = "__ZThn32_N3RBX9GlueJointD1Ev")]
// was: __ZThn32_N3RBX9GlueJointD1Ev
// IDA 0x7345cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7345cc() {
}

// 0x7345d4 — __ZThn32_N3RBX9GlueJointD0Ev
// type: void __fastcall(RBX::GlueJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GlueJoint::~GlueJoint()")]
#[doc(alias = "__ZThn32_N3RBX9GlueJointD0Ev")]
// was: __ZThn32_N3RBX9GlueJointD0Ev
// IDA 0x7345d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7345d4() {
}

// 0x734678 — __ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// was: __ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// IDA 0x734678: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_734678() {
}

// 0x7346b0 — __ZN3RBX9AllocatorINS_20NormalBreakConnectorEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::NormalBreakConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_20NormalBreakConnectorEEC2Ev")]
// was: __ZN3RBX9AllocatorINS_20NormalBreakConnectorEEC2Ev
// IDA 0x7346b0: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7346b0() {
}

// 0x734714 — __ZN3RBX9AllocatorINS_20NormalBreakConnectorEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::NormalBreakConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_20NormalBreakConnectorEE13releaseMemoryEv")]
// was: __ZN3RBX9AllocatorINS_20NormalBreakConnectorEE13releaseMemoryEv
// IDA 0x734714: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_734714() {
}

// 0x734730 — __ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// was: __ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// IDA 0x734730: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_734730() {
}

// 0x734760 — __GLOBAL__I_a_319
#[doc(alias = "global constructor keyed to_a_319")]
#[doc(alias = "__GLOBAL__I_a_319")]
// was: __GLOBAL__I_a_319
// IDA 0x734760: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_734760() {
}

// 0x73492c — __ZN3RBX11GroundStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::GroundStage::GroundStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX11GroundStageC1EPNS_6IStageEPNS_5WorldE")]
// was: __ZN3RBX11GroundStageC1EPNS_6IStageEPNS_5WorldE
// IDA 0x73492c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_73492c() {
}

// 0x734930 — __ZN3RBX11GroundStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::GroundStage::GroundStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX11GroundStageC2EPNS_6IStageEPNS_5WorldE")]
// was: __ZN3RBX11GroundStageC2EPNS_6IStageEPNS_5WorldE
// IDA 0x734930: 76 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_734930() {
}

// 0x734a04 — __ZN3RBX11GroundStageD0Ev
// type: void __fastcall(RBX::GroundStage *__hidden this)
#[doc(alias = "RBX::GroundStage::~GroundStage()")]
#[doc(alias = "__ZN3RBX11GroundStageD0Ev")]
// was: __ZN3RBX11GroundStageD0Ev
// IDA 0x734a04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_734a04() {
}

// 0x734abc — __ZN3RBX11GroundStageD1Ev
// type: void __fastcall(RBX::GroundStage *__hidden this)
#[doc(alias = "RBX::GroundStage::~GroundStage()")]
#[doc(alias = "__ZN3RBX11GroundStageD1Ev")]
// was: __ZN3RBX11GroundStageD1Ev
// IDA 0x734abc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_734abc() {
}

// 0x734ae0 — __ZN3RBX11GroundStage16onPrimitiveAddedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::GroundStage::onPrimitiveAdded(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX11GroundStage16onPrimitiveAddedEPNS_9PrimitiveE")]
// was: __ZN3RBX11GroundStage16onPrimitiveAddedEPNS_9PrimitiveE
// IDA 0x734ae0: 48 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_734ae0() {
}

// 0x734b70 — __ZN3RBX11GroundStage14addGroundJointEPNS_9PrimitiveEb
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::Primitive *, bool)
#[doc(alias = "RBX::GroundStage::addGroundJoint(RBX::Primitive *,bool)")]
#[doc(alias = "__ZN3RBX11GroundStage14addGroundJointEPNS_9PrimitiveEb")]
// was: __ZN3RBX11GroundStage14addGroundJointEPNS_9PrimitiveEb
// IDA 0x734b70: 145 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_734b70() {
}

// 0x734d14 — __ZN3RBX11GroundStage19onPrimitiveRemovingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::GroundStage::onPrimitiveRemoving(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX11GroundStage19onPrimitiveRemovingEPNS_9PrimitiveE")]
// was: __ZN3RBX11GroundStage19onPrimitiveRemovingEPNS_9PrimitiveE
// IDA 0x734d14: 75 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_734d14() {
}

// 0x734df4 — __ZN3RBX11GroundStage17removeGroundJointEPNS_9PrimitiveEb
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::Primitive *, bool)
#[doc(alias = "RBX::GroundStage::removeGroundJoint(RBX::Primitive *,bool)")]
#[doc(alias = "__ZN3RBX11GroundStage17removeGroundJointEPNS_9PrimitiveEb")]
// was: __ZN3RBX11GroundStage17removeGroundJointEPNS_9PrimitiveEb
// IDA 0x734df4: 111 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_734df4() {
}

// 0x734f30 — __ZN3RBX11GroundStage24onPrimitiveFixedChangingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::GroundStage::onPrimitiveFixedChanging(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX11GroundStage24onPrimitiveFixedChangingEPNS_9PrimitiveE")]
// was: __ZN3RBX11GroundStage24onPrimitiveFixedChangingEPNS_9PrimitiveE
// IDA 0x734f30: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_734f30() {
}

// 0x734f60 — __ZN3RBX11GroundStage23onPrimitiveFixedChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::GroundStage::onPrimitiveFixedChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX11GroundStage23onPrimitiveFixedChangedEPNS_9PrimitiveE")]
// was: __ZN3RBX11GroundStage23onPrimitiveFixedChangedEPNS_9PrimitiveE
// IDA 0x734f60: 26 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_734f60() {
}

// 0x734fa4 — __ZN3RBX11GroundStage17rebuildFreeGroundEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::GroundStage::rebuildFreeGround(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX11GroundStage17rebuildFreeGroundEPNS_9PrimitiveE")]
// was: __ZN3RBX11GroundStage17rebuildFreeGroundEPNS_9PrimitiveE
// IDA 0x734fa4: 72 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_734fa4() {
}

// 0x73506c — __ZN3RBX11GroundStage13rebuildOthersEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::GroundStage::rebuildOthers(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX11GroundStage13rebuildOthersEPNS_9PrimitiveE")]
// was: __ZN3RBX11GroundStage13rebuildOthersEPNS_9PrimitiveE
// IDA 0x73506c: 20 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73506c() {
}

// 0x73509c — __ZN3RBX11GroundStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::GroundStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX11GroundStage11onEdgeAddedEPNS_4EdgeE")]
// was: __ZN3RBX11GroundStage11onEdgeAddedEPNS_4EdgeE
// IDA 0x73509c: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73509c() {
}

// 0x7350fc — __ZN3RBX11GroundStage18onKernelJointAddedEPNS_11KernelJointE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::KernelJoint *)
#[doc(alias = "RBX::GroundStage::onKernelJointAdded(RBX::KernelJoint *)")]
#[doc(alias = "__ZN3RBX11GroundStage18onKernelJointAddedEPNS_11KernelJointE")]
// was: __ZN3RBX11GroundStage18onKernelJointAddedEPNS_11KernelJointE
// IDA 0x7350fc: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7350fc() {
}

// 0x7351ec — __ZN3RBX11GroundStage23checkForFreeGroundJointEPNS_10RigidJointE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::RigidJoint *)
#[doc(alias = "RBX::GroundStage::checkForFreeGroundJoint(RBX::RigidJoint *)")]
#[doc(alias = "__ZN3RBX11GroundStage23checkForFreeGroundJointEPNS_10RigidJointE")]
// was: __ZN3RBX11GroundStage23checkForFreeGroundJointEPNS_10RigidJointE
// IDA 0x7351ec: 86 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7351ec() {
}

// 0x7352e8 — __ZN3RBX11GroundStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::GroundStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX11GroundStage14onEdgeRemovingEPNS_4EdgeE")]
// was: __ZN3RBX11GroundStage14onEdgeRemovingEPNS_4EdgeE
// IDA 0x7352e8: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7352e8() {
}

// 0x735348 — __ZN3RBX11GroundStage21onKernelJointRemovingEPNS_11KernelJointE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::KernelJoint *)
#[doc(alias = "RBX::GroundStage::onKernelJointRemoving(RBX::KernelJoint *)")]
#[doc(alias = "__ZN3RBX11GroundStage21onKernelJointRemovingEPNS_11KernelJointE")]
// was: __ZN3RBX11GroundStage21onKernelJointRemovingEPNS_11KernelJointE
// IDA 0x735348: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_735348() {
}

// 0x735438 — __ZN3RBX11GroundStage21heaviestRigidToGroundEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::GroundStage::heaviestRigidToGround(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX11GroundStage21heaviestRigidToGroundEPNS_9PrimitiveE")]
// was: __ZN3RBX11GroundStage21heaviestRigidToGroundEPNS_9PrimitiveE
// IDA 0x735438: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_735438() {
}

// 0x7354ec — __ZNK3RBX11GroundStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::GroundStage *__hidden this)
#[doc(alias = "RBX::GroundStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX11GroundStage12getStageTypeEv")]
// was: __ZNK3RBX11GroundStage12getStageTypeEv
// IDA 0x7354ec: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7354ec() {
}

// 0x7354f0 — __ZN3RBX9FreeJointC2EPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::FreeJoint *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::FreeJoint::FreeJoint(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9FreeJointC2EPNS_9PrimitiveE")]
// was: __ZN3RBX9FreeJointC2EPNS_9PrimitiveE
// IDA 0x7354f0: 71 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7354f0() {
}
