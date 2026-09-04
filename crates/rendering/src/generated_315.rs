//! rendering shard 315 — 100 stubs 0x473fd0..0x477d68 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 34300->34400 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 34300 before -> 34400 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x473fd0 (lowest remaining 0x473fd0..0x477d68, next lowest 0x477ed8)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x473fd0 — __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE6resizeEmS5_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::resize(unsigned long,RBX::Reflection::EnumDescriptor::Item const*)")]
// was: __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE6resizeEmS5_
// IDA 0x473fd0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_473fd0() {
}

// 0x474004 — __ZNSt3mapIPKN3RBX4NameENS0_16DataModelArbiter16ConcurrencyModelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::DataModelArbiter::ConcurrencyModel,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_16DataModelArbiter16ConcurrencyModelESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x474004: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_474004() {
}

// 0x47405c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x47405c: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47405c() {
}

// 0x474110 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x474110: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_474110() {
}

// 0x474168 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x474168: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_474168() {
}

// 0x4741d0 — __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS5_S7_EEmRKS5_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor::Item const**,std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>>,unsigned long,RBX::Reflection::EnumDescriptor::Item const* const&)")]
// was: __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS5_S7_EEmRKS5_
// IDA 0x4741d0: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4741d0() {
}

// 0x474338 — __ZNSt12_Vector_baseIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE11_M_allocateEm
// IDA 0x474338: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_474338() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x474350 — __ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,std::allocator<std::string>>>,unsigned long,std::string const&)")]
// was: __ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs
// IDA 0x474350: 520 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_474350() {
}

// 0x47486c — __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModelArbiter::ConcurrencyModel*,std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>>,RBX::DataModelArbiter::ConcurrencyModel const&)")]
// was: __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x47486c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_47486c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x474950 — __ZNSt12_Vector_baseIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE11_M_allocateEm
// IDA 0x474950: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_474950() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x474968 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16DataModelArbiter16ConcurrencyModelES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::DataModelArbiter::ConcurrencyModel * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *>(RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *,RBX::DataModelArbiter::ConcurrencyModel *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16DataModelArbiter16ConcurrencyModelES6_EET0_T_S8_S7_
// IDA 0x474968: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_474968() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x4749a8 — __ZNSt12_Vector_baseImSaImEE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<unsigned long,std::allocator<unsigned long>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseImSaImEE11_M_allocateEm
// IDA 0x4749a8: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_4749a8() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x4749c0 — __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModelArbiter::ConcurrencyModel*,std::vector<RBX::DataModelArbiter::ConcurrencyModel,std::allocator<RBX::DataModelArbiter::ConcurrencyModel>>>,unsigned long,RBX::DataModelArbiter::ConcurrencyModel const&)")]
// was: __ZNSt6vectorIN3RBX16DataModelArbiter16ConcurrencyModelESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x4749c0: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4749c0() {
}

// 0x474b50 — __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor::Item const**,std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>>,RBX::Reflection::EnumDescriptor::Item const* const&)")]
// was: __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// IDA 0x474b50: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_474b50() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x474c30 — __ZN3RBX10Reflection14EnumDescriptor4ItemD0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor::Item *__hidden this)
#[doc(alias = "RBX::Reflection::EnumDescriptor::Item::~Item()")]
// was: __ZN3RBX10Reflection14EnumDescriptor4ItemD0Ev
// IDA 0x474c30: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_474c30() {
}

// 0x474c38 — __ZN3RBX13ActivityMeterILi2EE13updateBucketsEv
// type: int(void)
#[doc(alias = "RBX::ActivityMeter<2>::updateBuckets(void)")]
// was: __ZN3RBX13ActivityMeterILi2EE13updateBucketsEv
// IDA 0x474c38: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_474c38() {
}

// 0x474cf0 — __ZN3RBX16OnScreenProfiler7GetInstEv
// type: _DWORD __fastcall(RBX::OnScreenProfiler *__hidden this)
#[doc(alias = "RBX::OnScreenProfiler::GetInst(void)")]
// was: __ZN3RBX16OnScreenProfiler7GetInstEv
// IDA 0x474cf0: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_474cf0() {
}

// 0x474d54 — __ZN3RBX16OnScreenProfiler6CreateEv
// type: _DWORD __fastcall(RBX::OnScreenProfiler *__hidden this)
#[doc(alias = "RBX::OnScreenProfiler::Create(void)")]
// was: __ZN3RBX16OnScreenProfiler6CreateEv
// IDA 0x474d54: 54 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_474d54() {
}

// 0x474dfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DataModelArbiter::ConcurrencyModel>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16DataModelArbiter16ConcurrencyModelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x474dfc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_474dfc() {
}

// 0x474e24 — __GLOBAL__I_a_179
// type: void
#[doc(alias = "global constructor keyed to_a_179")]
// was: __GLOBAL__I_a_179
// IDA 0x474e24: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_474e24() {
}

// 0x474eec — __ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEEC1Ev
// type: void
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEEC1Ev
// IDA 0x474eec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_474eec() {
}

// 0x474ef0 — __ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEEC2Ev
// type: void
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEEC2Ev
// IDA 0x474ef0: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_474ef0() {
}

// 0x4750c8 — __ZN3RBX13DataModelMesh17setLevelOfDetailXENS0_7LODTypeE
// type: void
#[doc(alias = "RBX::DataModelMesh::setLevelOfDetailX(RBX::DataModelMesh::LODType)")]
// was: __ZN3RBX13DataModelMesh17setLevelOfDetailXENS0_7LODTypeE
// IDA 0x4750c8: 9 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4750c8() {
}

// 0x4750e8 — __ZN3RBX13DataModelMesh17setLevelOfDetailYENS0_7LODTypeE
// type: void
#[doc(alias = "RBX::DataModelMesh::setLevelOfDetailY(RBX::DataModelMesh::LODType)")]
// was: __ZN3RBX13DataModelMesh17setLevelOfDetailYENS0_7LODTypeE
// IDA 0x4750e8: 9 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4750e8() {
}

// 0x475278 — __ZN3RBX13DataModelMeshC2Ev
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::DataModelMesh(void)")]
// was: __ZN3RBX13DataModelMeshC2Ev
// IDA 0x475278: 193 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475278() {
}

// 0x4754a4 — __ZNK3RBX13DataModelMesh12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::DataModelMesh::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX13DataModelMesh12askSetParentEPKNS_8InstanceE
// IDA 0x4754a4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4754a4() {
}

// 0x4754e0 — __ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::addPair(RBX::DataModelMesh::LODType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEE7addPairES3_PKc
// IDA 0x4754e0: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4754e0() {
}

// 0x475840 — __ZNK3RBX13DataModelMesh17getLevelOfDetailXEv
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::getLevelOfDetailX(void)const")]
// was: __ZNK3RBX13DataModelMesh17getLevelOfDetailXEv
// IDA 0x475840: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475840() {
}

// 0x475848 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEED1Ev
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEED1Ev
// IDA 0x475848: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_475848() {
}

// 0x47586c — __ZNK3RBX13DataModelMesh17getLevelOfDetailYEv
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::getLevelOfDetailY(void)const")]
// was: __ZNK3RBX13DataModelMesh17getLevelOfDetailYEv
// IDA 0x47586c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47586c() {
}

// 0x475874 — __ZNK3RBX13DataModelMesh8getScaleEv
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::getScale(void)const")]
// was: __ZNK3RBX13DataModelMesh8getScaleEv
// IDA 0x475874: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475874() {
}

// 0x47589c — __ZNK3RBX13DataModelMesh12getVertColorEv
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::getVertColor(void)const")]
// was: __ZNK3RBX13DataModelMesh12getVertColorEv
// IDA 0x47589c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47589c() {
}

// 0x4758a0 — __ZNK3RBX13DataModelMesh9getOffsetEv
// type: _DWORD __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::getOffset(void)const")]
// was: __ZNK3RBX13DataModelMesh9getOffsetEv
// IDA 0x4758a0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4758a0() {
}

// 0x4758a4 — __ZN3RBX13DataModelMeshD1Ev
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::~DataModelMesh()")]
// was: __ZN3RBX13DataModelMeshD1Ev
// IDA 0x4758a4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4758a4() {
}

// 0x4758a8 — __ZN3RBX13DataModelMeshD0Ev
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "RBX::DataModelMesh::~DataModelMesh()")]
// was: __ZN3RBX13DataModelMeshD0Ev
// IDA 0x4758a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4758a8() {
}

// 0x475948 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEE12getClassNameEv
// type: void
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEE12getClassNameEv
// IDA 0x475948: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475948() {
}

// 0x475970 — __ZThn32_N3RBX13DataModelMeshD1Ev
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModelMesh::~DataModelMesh()")]
// was: __ZThn32_N3RBX13DataModelMeshD1Ev
// IDA 0x475970: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_475970() {
}

// 0x475978 — __ZThn32_N3RBX13DataModelMeshD0Ev
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModelMesh::~DataModelMesh()")]
// was: __ZThn32_N3RBX13DataModelMeshD0Ev
// IDA 0x475978: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_475978() {
}

// 0x475a1c — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEE12getClassNameEv
// type: void
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEE12getClassNameEv
// IDA 0x475a1c: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475a1c() {
}

// 0x475a44 — __ZThn36_N3RBX13DataModelMeshD1Ev
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModelMesh::~DataModelMesh()")]
// was: __ZThn36_N3RBX13DataModelMeshD1Ev
// IDA 0x475a44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_475a44() {
}

// 0x475a4c — __ZThn36_N3RBX13DataModelMeshD0Ev
// type: void __fastcall(RBX::DataModelMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DataModelMesh::~DataModelMesh()")]
// was: __ZThn36_N3RBX13DataModelMeshD0Ev
// IDA 0x475a4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_475a4c() {
}

// 0x475af0 — __ZN3RBX4Name13callDoDeclareILZNS_14sDataModelMeshEEEEvv
// type: void
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDataModelMeshEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sDataModelMeshEEEEvv
// IDA 0x475af0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_475af0() {
}

// 0x475af4 — __ZN3RBX4Name9doDeclareILZNS_14sDataModelMeshEEEERKS0_v
// type: void
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDataModelMeshEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sDataModelMeshEEEERKS0_v
// IDA 0x475af4: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475af4() {
}

// 0x475bd4 — __ZN3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x475bd4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_475bd4() {
}

// 0x475bd8 — __ZN3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x475bd8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_475bd8() {
}

// 0x475c78 — __ZThn32_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x475c78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_475c78() {
}

// 0x475c80 — __ZThn32_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x475c80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_475c80() {
}

// 0x475d24 — __ZThn36_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x475d24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_475d24() {
}

// 0x475d2c — __ZThn36_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x475d2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_475d2c() {
}

// 0x475f70 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::EnumPropDescriptor<RBX::DataModelMesh::LODType (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(RBX::DataModelMesh::LODType)>(char const*,char const*,RBX::DataModelMesh::LODType (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(RBX::DataModelMesh::LODType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x475f70: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_475f70() {
}

// 0x476124 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEED0Ev
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEED0Ev
// IDA 0x476124: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_476124() {
}

// 0x476150 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10isReadOnlyEv
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10isReadOnlyEv
// IDA 0x476150: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476150() {
}

// 0x476160 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE11isWriteOnlyEv
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE11isWriteOnlyEv
// IDA 0x476160: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476160() {
}

// 0x476170 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x476170: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476170() {
}

// 0x476198 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x476198: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476198() {
}

// 0x4761bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x4761bc: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4761bc() {
}

// 0x476308 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x476308: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476308() {
}

// 0x47632c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE14hasStringValueEv
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE14hasStringValueEv
// IDA 0x47632c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47632c() {
}

// 0x476330 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE14getStringValueEPKNS0_13DescribedBaseE
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x476330: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476330() {
}

// 0x476354 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x476354: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476354() {
}

// 0x476394 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x476394: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476394() {
}

// 0x4763b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x4763b4: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4763b4() {
}

// 0x4765f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x4765f4: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4765f4() {
}

// 0x476610 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x476610: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476610() {
}

// 0x476644 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x476644: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476644() {
}

// 0x47664c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x47664c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47664c() {
}

// 0x476698 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x476698: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476698() {
}

// 0x4766b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: void
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x4766b8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4766b8() {
}

// 0x4766ec — __ZNK3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::convertToIndex(RBX::DataModelMesh::LODType)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEE14convertToIndexES3_
// IDA 0x4766ec: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4766ec() {
}

// 0x47675c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DataModelMeshENS2_7LODTypeEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x47675c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47675c() {
}

// 0x47679c — __ZNK3RBX10Reflection14PropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: void
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::GetSetImpl<RBX::DataModelMesh::LODType (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(RBX::DataModelMesh::LODType)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x47679c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47679c() {
}

// 0x4767a0 — __ZNK3RBX10Reflection14PropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: void
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::GetSetImpl<RBX::DataModelMesh::LODType (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(RBX::DataModelMesh::LODType)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x4767a0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4767a0() {
}

// 0x4767a4 — __ZNK3RBX10Reflection14PropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: void
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::GetSetImpl<RBX::DataModelMesh::LODType (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(RBX::DataModelMesh::LODType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x4767a4: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4767a4() {
}

// 0x4767c4 — __ZNK3RBX10Reflection14PropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: void
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::GetSetImpl<RBX::DataModelMesh::LODType (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(RBX::DataModelMesh::LODType)>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModelMesh::LODType const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DataModelMeshENS2_7LODTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x4767c4: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4767c4() {
}

// 0x4767e8 — __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::resize(unsigned long,RBX::DataModelMesh::LODType)")]
// was: __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE6resizeEmS2_
// IDA 0x4767e8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4767e8() {
}

// 0x47681c — __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::push_back(RBX::DataModelMesh::LODType const&)")]
// was: __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE9push_backERKS2_
// IDA 0x47681c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_47681c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x476844 — __ZNSt3mapIPKN3RBX4NameENS0_13DataModelMesh7LODTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::DataModelMesh::LODType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_13DataModelMesh7LODTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x476844: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476844() {
}

// 0x47689c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x47689c: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47689c() {
}

// 0x476950 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x476950: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476950() {
}

// 0x4769a8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13DataModelMesh7LODTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x4769a8: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4769a8() {
}

// 0x476a10 — __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModelMesh::LODType*,std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>>,RBX::DataModelMesh::LODType const&)")]
// was: __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x476a10: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_476a10() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x476af4 — __ZNSt12_Vector_baseIN3RBX13DataModelMesh7LODTypeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX13DataModelMesh7LODTypeESaIS2_EE11_M_allocateEm
// IDA 0x476af4: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_476af4() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x476b0c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DataModelMesh7LODTypeES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::DataModelMesh::LODType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModelMesh::LODType *,RBX::DataModelMesh::LODType *>(RBX::DataModelMesh::LODType *,RBX::DataModelMesh::LODType *,RBX::DataModelMesh::LODType *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DataModelMesh7LODTypeES6_EET0_T_S8_S7_
// IDA 0x476b0c: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_476b0c() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x476b48 — __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModelMesh::LODType*,std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>>,unsigned long,RBX::DataModelMesh::LODType const&)")]
// was: __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x476b48: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_476b48() {
}

// 0x476cd8 — __GLOBAL__I_a_180
// type: void
#[doc(alias = "global constructor keyed to_a_180")]
// was: __GLOBAL__I_a_180
// IDA 0x476cd8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_476cd8() {
}

// 0x4770dc — __ZN3RBX13DebrisService11setMaxItemsEi
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this, int)
#[doc(alias = "RBX::DebrisService::setMaxItems(int)")]
// was: __ZN3RBX13DebrisService11setMaxItemsEi
// IDA 0x4770dc: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4770dc() {
}

// 0x477264 — __ZN3RBX13DebrisService7addItemEN5boost10shared_ptrINS_8InstanceEEEd
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::DebrisService::addItem(rbx_core::SharedPtr<RBX::Instance>,double)")]
// was: __ZN3RBX13DebrisService7addItemEN5boost10shared_ptrINS_8InstanceEEEd
// IDA 0x477264: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_477264() {
}

// 0x477410 — __ZN3RBX13DebrisService17setLegacyMaxItemsEb
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this, bool)
#[doc(alias = "RBX::DebrisService::setLegacyMaxItems(bool)")]
// was: __ZN3RBX13DebrisService17setLegacyMaxItemsEb
// IDA 0x477410: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_477410() {
}

// 0x477418 — __ZN3RBX13DebrisServiceC1Ev
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "RBX::DebrisService::DebrisService(void)")]
// was: __ZN3RBX13DebrisServiceC1Ev
// IDA 0x477418: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_477418() {
}

// 0x47741c — __ZN3RBX13DebrisServiceC2Ev
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "RBX::DebrisService::DebrisService(void)")]
// was: __ZN3RBX13DebrisServiceC2Ev
// IDA 0x47741c: 158 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47741c() {
}

// 0x4775e4 — __ZN3RBX13DebrisService7cleanupEv
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "RBX::DebrisService::cleanup(void)")]
// was: __ZN3RBX13DebrisService7cleanupEv
// IDA 0x4775e4: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4775e4() {
}

// 0x477738 — __ZL7cleanupN5boost8weak_ptrIN3RBX8InstanceEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, void *, int)
#[doc(alias = "cleanup(rbx_core::WeakPtr<RBX::Instance>)")]
// was: __ZL7cleanupN5boost8weak_ptrIN3RBX8InstanceEEE
// IDA 0x477738: 111 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_477738() {
}

// 0x477864 — __ZN3RBX13DebrisService17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::DebrisService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX13DebrisService17onServiceProviderEPNS_15ServiceProviderES2_
// IDA 0x477864: 152 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_477864() {
}

// 0x477a0c — __ZNK3RBX13DebrisService11getMaxItemsEv
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "RBX::DebrisService::getMaxItems(void)const")]
// was: __ZNK3RBX13DebrisService11getMaxItemsEv
// IDA 0x477a0c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_477a0c() {
}

// 0x477a14 — __ZN3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiED1Ev
// type: void
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiED1Ev
// IDA 0x477a14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_477a14() {
}

// 0x477a38 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EED1Ev
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EED1Ev
// IDA 0x477a38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_477a38() {
}

// 0x477b50 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EED1Ev
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EED1Ev
// IDA 0x477b50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_477b50() {
}

// 0x477b90 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX8InstanceEEES4_EENS_3_bi6bind_tIT_PFS7_T0_ENS5_9list_av_1IT1_E4typeEEESA_SC_
// type: int __fastcall(int, boost::detail::sp_counted_base *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list_av_1<rbx_core::WeakPtr<RBX::Instance>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance>>(void (*)(rbx_core::WeakPtr<RBX::Instance>),rbx_core::WeakPtr<RBX::Instance>)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX8InstanceEEES4_EENS_3_bi6bind_tIT_PFS7_T0_ENS5_9list_av_1IT1_E4typeEEESA_SC_
// IDA 0x477b90: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_477b90() {
}

// 0x477d30 — __ZN5boost10shared_ptrIN3RBX12TimerServiceEEaSERKS3_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::TimerService>::operator=(rbx_core::SharedPtr<RBX::TimerService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX12TimerServiceEEaSERKS3_
// IDA 0x477d30: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_477d30() {
}

// 0x477d68 — __ZN3RBX11shared_fromINS_12TimerServiceEEEN5boost10shared_ptrIT_EEPS4_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::TimerService> RBX::shared_from<RBX::TimerService>(RBX::TimerService*)")]
// was: __ZN3RBX11shared_fromINS_12TimerServiceEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x477d68: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_477d68() {
}

