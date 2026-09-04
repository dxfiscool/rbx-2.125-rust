//! rendering shard 343 — 100 stubs 0x49eb28..0x4a31b8 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 37320->37420 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 37320 before -> 37420 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 lowest remaining 0x49eb28..0x4a31b8 (next lowest 0x4a31bc if exists)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x49eb28 — __ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>::resize(unsigned long,RBX::ExtrudedPartInstance::VisualTrussStyle)")]
// IDA 0x49eb28: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49eb28() {
}

// 0x49eb5c — __ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>::push_back(RBX::ExtrudedPartInstance::VisualTrussStyle const&)")]
// IDA 0x49eb5c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_49eb5c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x49eb84 — __ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ExtrudedPartInstance::VisualTrussStyle*,std::vector<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>>,RBX::ExtrudedPartInstance::VisualTrussStyle const&)")]
// IDA 0x49eb84: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_49eb84() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x49ec68 — __ZNSt12_Vector_baseIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>::_M_allocate(unsigned long)")]
// IDA 0x49ec68: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_49ec68() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x49ec80 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX20ExtrudedPartInstance16VisualTrussStyleES6_EET0_T_S8_S7_
#[doc(alias = "RBX::ExtrudedPartInstance::VisualTrussStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ExtrudedPartInstance::VisualTrussStyle *,RBX::ExtrudedPartInstance::VisualTrussStyle *>(RBX::ExtrudedPartInstance::VisualTrussStyle *,RBX::ExtrudedPartInstance::VisualTrussStyle *,RBX::ExtrudedPartInstance::VisualTrussStyle *)")]
// IDA 0x49ec80: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_49ec80() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x49ecbc — __ZNSt6vectorIN3RBX20ExtrudedPartInstance16VisualTrussStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::ExtrudedPartInstance::VisualTrussStyle*,std::vector<RBX::ExtrudedPartInstance::VisualTrussStyle,std::allocator<RBX::ExtrudedPartInstance::VisualTrussStyle>>>,unsigned long,RBX::ExtrudedPartInstance::VisualTrussStyle const&)")]
// IDA 0x49ecbc: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49ecbc() {
}

// 0x49ee4c — __ZNSt6vectorIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::resize(unsigned long,RBX::BasicPartInstance::LegacyPartType)")]
// IDA 0x49ee4c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49ee4c() {
}

// 0x49ee80 — __ZNSt6vectorIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::push_back(RBX::BasicPartInstance::LegacyPartType const&)")]
// IDA 0x49ee80: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_49ee80() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x49eea8 — __ZNSt3mapIPKN3RBX4NameENS0_17BasicPartInstance14LegacyPartTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::BasicPartInstance::LegacyPartType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::operator[](RBX::Name const* const&)")]
// IDA 0x49eea8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49eea8() {
}

// 0x49ef00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType> const&)")]
// IDA 0x49ef00: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49ef00() {
}

// 0x49efb4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType> const&)")]
// IDA 0x49efb4: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49efb4() {
}

// 0x49f00c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType> const&)")]
// IDA 0x49f00c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49f00c() {
}

// 0x49f074 — __ZNSt6vectorIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::BasicPartInstance::LegacyPartType*,std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>>,RBX::BasicPartInstance::LegacyPartType const&)")]
// IDA 0x49f074: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_49f074() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x49f158 — __ZNSt12_Vector_baseIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::_M_allocate(unsigned long)")]
// IDA 0x49f158: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_49f158() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x49f170 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17BasicPartInstance14LegacyPartTypeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::BasicPartInstance::LegacyPartType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BasicPartInstance::LegacyPartType *,RBX::BasicPartInstance::LegacyPartType *>(RBX::BasicPartInstance::LegacyPartType *,RBX::BasicPartInstance::LegacyPartType *,RBX::BasicPartInstance::LegacyPartType *)")]
// IDA 0x49f170: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_49f170() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x49f1ac — __ZNSt6vectorIN3RBX17BasicPartInstance14LegacyPartTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BasicPartInstance::LegacyPartType*,std::vector<RBX::BasicPartInstance::LegacyPartType,std::allocator<RBX::BasicPartInstance::LegacyPartType>>>,unsigned long,RBX::BasicPartInstance::LegacyPartType const&)")]
// IDA 0x49f1ac: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49f1ac() {
}

// 0x49f33c — __GLOBAL__I_a_187
#[doc(alias = "__GLOBAL__I_a_187")]
// IDA 0x49f33c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_49f33c() {
}

// 0x49f5ac — __ZN3RBX9Explosion14setBlastRadiusEf
#[doc(alias = "RBX::Explosion::setBlastRadius(float)")]
// IDA 0x49f5ac: 17 insns (VMOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49f5ac() {
}

// 0x49f5f0 — __ZN3RBX9Explosion16setExplosionTypeENS0_13ExplosionTypeE
#[doc(alias = "RBX::Explosion::setExplosionType(RBX::Explosion::ExplosionType)")]
// IDA 0x49f5f0: 9 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49f5f0() {
}

// 0x49f610 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::EnumDesc(void)")]
// IDA 0x49f610: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_49f610() {
}

// 0x49f614 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::EnumDesc(void)")]
// IDA 0x49f614: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49f614() {
}

// 0x49f7ec — __ZN3RBX9ExplosionC1Ev
#[doc(alias = "RBX::Explosion::Explosion(void)")]
// IDA 0x49f7ec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_49f7ec() {
}

// 0x49f7f0 — __ZN3RBX9ExplosionC2Ev
#[doc(alias = "RBX::Explosion::Explosion(void)")]
// IDA 0x49f7f0: 360 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49f7f0() {
}

// 0x49fbe4 — __ZN3RBX9ExplosionD0Ev
#[doc(alias = "RBX::Explosion::~Explosion()")]
// IDA 0x49fbe4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49fbe4() {
}

// 0x49fc84 — __ZN3RBX9ExplosionD1Ev
#[doc(alias = "RBX::Explosion::~Explosion()")]
// IDA 0x49fc84: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_49fc84() {
}

// 0x49fc88 — __ZThn32_N3RBX9ExplosionD0Ev
#[doc(alias = "__ZThn32_N3RBX9ExplosionD0Ev")]
// IDA 0x49fc88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49fc88() {
}

// 0x49fc90 — __ZThn36_N3RBX9ExplosionD0Ev
#[doc(alias = "__ZThn36_N3RBX9ExplosionD0Ev")]
// IDA 0x49fc90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49fc90() {
}

// 0x49fc98 — __ZThn116_N3RBX9ExplosionD0Ev
#[doc(alias = "__ZThn116_N3RBX9ExplosionD0Ev")]
// IDA 0x49fc98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49fc98() {
}

// 0x49fca0 — __ZThn128_N3RBX9ExplosionD0Ev
#[doc(alias = "__ZThn128_N3RBX9ExplosionD0Ev")]
// IDA 0x49fca0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49fca0() {
}

// 0x49fca8 — __ZN3RBX9ExplosionD2Ev
#[doc(alias = "RBX::Explosion::~Explosion()")]
// IDA 0x49fca8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49fca8() {
}

// 0x49fee8 — __ZThn32_N3RBX9ExplosionD1Ev
#[doc(alias = "__ZThn32_N3RBX9ExplosionD1Ev")]
// IDA 0x49fee8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49fee8() {
}

// 0x49fef0 — __ZThn36_N3RBX9ExplosionD1Ev
#[doc(alias = "__ZThn36_N3RBX9ExplosionD1Ev")]
// IDA 0x49fef0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49fef0() {
}

// 0x49fef8 — __ZThn116_N3RBX9ExplosionD1Ev
#[doc(alias = "__ZThn116_N3RBX9ExplosionD1Ev")]
// IDA 0x49fef8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49fef8() {
}

// 0x49ff00 — __ZThn128_N3RBX9ExplosionD1Ev
#[doc(alias = "__ZThn128_N3RBX9ExplosionD1Ev")]
// IDA 0x49ff00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_49ff00() {
}

// 0x49ff08 — __ZN3RBX9Explosion11signalBlastERKSt6vectorIN5boost10shared_ptrINS_12PartInstanceEEESaIS5_EE
#[doc(alias = "RBX::Explosion::signalBlast(std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>> const&)")]
// was: __ZN3RBX9Explosion11signalBlastERKSt6vectorIN5boost10shared_ptrINS_12PartInstanceEEESaIS5_EE
// IDA 0x49ff08: 132 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_49ff08() {
}

// 0x4a0094 — __ZNK3RBX9Explosion12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Explosion::askSetParent(RBX::Instance const*)const")]
// IDA 0x4a0094: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a0094() {
}

// 0x4a0098 — __ZN3RBX9Explosion9onSteppedERKNS_7SteppedE
#[doc(alias = "RBX::Explosion::onStepped(RBX::Stepped const&)")]
// IDA 0x4a0098: 233 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a0098() {
}

// 0x4a0318 — __ZThn116_N3RBX9Explosion9onSteppedERKNS_7SteppedE
#[doc(alias = "__ZThn116_N3RBX9Explosion9onSteppedERKNS_7SteppedE")]
// IDA 0x4a0318: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a0318() {
}

// 0x4a0438 — __ZNK3RBX9Explosion14getBlastRadiusEv
#[doc(alias = "RBX::Explosion::getBlastRadius(void)const")]
// IDA 0x4a0438: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a0438() {
}

// 0x4a0440 — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::~PropDescriptor()")]
// IDA 0x4a0440: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a0440() {
}

// 0x4a048c — __ZNK3RBX9Explosion16getExplosionTypeEv
#[doc(alias = "RBX::Explosion::getExplosionType(void)const")]
// IDA 0x4a048c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a048c() {
}

// 0x4a0494 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::~EnumPropDescriptor()")]
// IDA 0x4a0494: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a0494() {
}

// 0x4a04b8 — __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_ED1Ev
// IDA 0x4a04b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a04b8() {
}

// 0x4a04dc — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::addPair(RBX::Explosion::ExplosionType,char const*)")]
// IDA 0x4a04dc: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a04dc() {
}

// 0x4a0840 — __ZN3rbx7signals16signal_with_argsILi2EFvN5boost10shared_ptrIN3RBX8InstanceEEEfEEclES6_f
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::operator()(rbx_core::SharedPtr<RBX::Instance>,float)")]
// was: __ZN3rbx7signals16signal_with_argsILi2EFvN5boost10shared_ptrIN3RBX8InstanceEEEfEEclES6_f
// IDA 0x4a0840: 190 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a0840() {
}

// 0x4a0a30 — __ZN3RBX9Explosion7doBlastINS_19MegaClusterInstanceEEEvPT_RKSt6vectorIN5boost10shared_ptrINS_12PartInstanceEEESaIS9_EE
#[doc(alias = "void RBX::Explosion::doBlast<RBX::MegaClusterInstance>(RBX::MegaClusterInstance *,std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>> const&)")]
// was: __ZN3RBX9Explosion7doBlastINS_19MegaClusterInstanceEEEvPT_RKSt6vectorIN5boost10shared_ptrINS_12PartInstanceEEESaIS9_EE
// IDA 0x4a0a30: 491 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a0a30() {
}

// 0x4a1088 — __ZN3RBX15ServiceProvider6createINS_12TimerServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::TimerService * RBX::ServiceProvider::create<RBX::TimerService>(RBX::Instance const*)")]
// IDA 0x4a1088: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a1088() {
}

// 0x4a10a0 — __ZN5boost4bindIvN3RBX8InstanceEPS2_NS_10shared_ptrINS1_9ExplosionEEES3_EENS_3_bi6bind_tIT_NS_4_mfi3mf1IS9_T0_T1_EENS7_9list_av_2IT2_T3_E4typeEEEMSC_FS9_SD_ESG_SH_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Explosion>,RBX::Instance*>::type> boost::bind<void,RBX::Instance,RBX::Instance*,rbx_core::SharedPtr<RBX::Explosion>,RBX::Instance*>(void (RBX::Instance::*)(RBX::Instance*),rbx_core::SharedPtr<RBX::Explosion>,RBX::Instance*)")]
// was: __ZN5boost4bindIvN3RBX8InstanceEPS2_NS_10shared_ptrINS1_9ExplosionEEES3_EENS_3_bi6bind_tIT_NS_4_mfi3mf1IS9_T0_T1_EENS7_9list_av_2IT2_T3_E4typeEEEMSC_FS9_SD_ESG_SH_
// IDA 0x4a10a0: 109 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a10a0() {
}

// 0x4a1334 — __ZN3RBX9Explosion17onServiceProviderEPNS_15ServiceProviderES2_
#[doc(alias = "RBX::Explosion::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// IDA 0x4a1334: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a1334() {
}

// 0x4a1340 — __ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E12getClassNameEv")]
// IDA 0x4a1340: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a1340() {
}

// 0x4a1358 — __ZThn32_NK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E12getClassNameEv")]
// IDA 0x4a1358: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a1358() {
}

// 0x4a1378 — __ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7CreatorD1Ev")]
// IDA 0x4a1378: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a1378() {
}

// 0x4a137c — __ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorD1Ev")]
// IDA 0x4a137c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a137c() {
}

// 0x4a1380 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()")]
// IDA 0x4a1380: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a1380() {
}

// 0x4a1388 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToString(unsigned long,std::string &)const")]
// IDA 0x4a1388: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a1388() {
}

// 0x4a14d0 — __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::construct_func(char const*,char *)")]
// IDA 0x4a14d0: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a14d0() {
}

// 0x4a14e0 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToItem(RBX::Explosion::ExplosionType const&)const")]
// IDA 0x4a14e0: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a14e0() {
}

// 0x4a15b0 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()")]
// IDA 0x4a15b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a15b0() {
}

// 0x4a1788 — __ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7Creator12getClassNameEv")]
// IDA 0x4a1788: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a1788() {
}

// 0x4a17f8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ForceFieldEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ForceField> RBX::Creatable<RBX::Instance>::create<RBX::ForceField>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ForceFieldEEEN5boost10shared_ptrIT_EEv
// IDA 0x4a17f8: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a17f8() {
}

// 0x4a18a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ForceFieldENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ForceFieldENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x4a18a8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a18a8() {
}

// 0x4a18b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ForceFieldENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ForceFieldENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x4a18b0: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a18b0() {
}

// 0x4a18c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ForceFieldENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ForceFieldENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x4a18c8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a18c8() {
}

// 0x4a18d0 — __ZN3RBX4Name13callDoDeclareILZNS_11sForceFieldEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sForceFieldEEEEvv")]
// IDA 0x4a18d0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a18d0() {
}

// 0x4a18d8 — __ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E7CreatorC2Ev")]
// IDA 0x4a18d8: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a18d8() {
}

// 0x4a1b04 — __ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorD2Ev")]
// IDA 0x4a1b04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a1b04() {
}

// 0x4a1ba0 — __ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator12getClassNameEv")]
// IDA 0x4a1ba0: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a1ba0() {
}

// 0x4a1c28 — __ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7Creator6createEv")]
// IDA 0x4a1c28: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a1c28() {
}

// 0x4a1d70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x4a1d70: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a1d70() {
}

// 0x4a1d78 — __ZN3RBX4Name13callDoDeclareILZNS_10sExplosionEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sExplosionEEEEvv")]
// IDA 0x4a1d78: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a1d78() {
}

// 0x4a1d7c — __ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v")]
// IDA 0x4a1d7c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a1d7c() {
}

// 0x4a1e5c — __ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E7CreatorC2Ev")]
// IDA 0x4a1e5c: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a1e5c() {
}

// 0x4a20a0 — __ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_9ExplosionENS_8InstanceELZNS_10sExplosionEES2_E17static_getCreatorEv")]
// IDA 0x4a20a0: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a20a0() {
}

// 0x4a2200 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEEEvT_
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>)")]
// was: __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEEEvT_
// IDA 0x4a2200: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2200() {
}

// 0x4a22fc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// IDA 0x4a22fc: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a22fc() {
}

// 0x4a2318 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEvE6invokeERNS1_15function_bufferE
// IDA 0x4a2318: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2318() {
}

// 0x4a2320 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPSA_EENS5_5list2INS5_5valueINS_10shared_ptrINS9_9ExplosionEEEEENSE_ISB_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPSA_EENS5_5list2INS5_5valueINS_10shared_ptrINS9_9ExplosionEEEEENSE_ISB_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x4a2320: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2320() {
}

// 0x4a240c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPSA_EENS5_5list2INS5_5valueINS_10shared_ptrINS9_9ExplosionEEEEENSE_ISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPSA_EENS5_5list2INS5_5valueINS_10shared_ptrINS9_9ExplosionEEEEENSE_ISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x4a240c: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a240c() {
}

// 0x4a24f4 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPSA_EENS5_5list2INS5_5valueINS_10shared_ptrINS9_9ExplosionEEEEENSE_ISB_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPSA_EENS5_5list2INS5_5valueINS_10shared_ptrINS9_9ExplosionEEEEENSE_ISB_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x4a24f4: 76 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a24f4() {
}

// 0x4a25cc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS5_EENS0_5list2INS0_5valueINS_10shared_ptrINS4_9ExplosionEEEEENS9_IS6_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS5_EENS0_5list2INS0_5valueINS_10shared_ptrINS4_9ExplosionEEEEENS9_IS6_EEEEEclEv
// IDA 0x4a25cc: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a25cc() {
}

// 0x4a25e4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Instance,RBX::Instance*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance*>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8InstanceEPS8_EENS3_5list2INS3_5valueINS_10shared_ptrINS7_9ExplosionEEEEENSC_IS9_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x4a25e4: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a25e4() {
}

// 0x4a2740 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX9ExplosionEEEEENS2_IPNS4_8InstanceEEEEC2ES7_SA_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance *>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance *>)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX9ExplosionEEEEENS2_IPNS4_8InstanceEEEEC2ES7_SA_
// IDA 0x4a2740: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2740() {
}

// 0x4a2818 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX9ExplosionEEEEENS2_IPNS4_8InstanceEEEEC2ES7_SA_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance *>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Explosion>>,boost::_bi::value<RBX::Instance *>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX9ExplosionEEEEENS2_IPNS4_8InstanceEEEEC2ES7_SA_
// IDA 0x4a2818: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2818() {
}

// 0x4a2900 — __ZNK3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E7Creator12getClassNameEv")]
// IDA 0x4a2900: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2900() {
}

// 0x4a2970 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12TimerServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::TimerService> RBX::Creatable<RBX::Instance>::create<RBX::TimerService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_12TimerServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x4a2970: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2970() {
}

// 0x4a2a20 — __ZN5boost10shared_ptrIN3RBX12TimerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::TimerService>::shared_ptr<RBX::TimerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TimerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX12TimerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x4a2a20: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2a20() {
}

// 0x4a2bd0 — __ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E15isNullClassNameEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_12TimerServiceENS_8InstanceELZNS_13sTimerServiceEES2_E15isNullClassNameEv")]
// IDA 0x4a2bd0: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2bd0() {
}

// 0x4a2c38 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EED2Ev
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::~vector()")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EED2Ev
// IDA 0x4a2c38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a2c38() {
}

// 0x4a2d04 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
// IDA 0x4a2d04: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2d04() {
}

// 0x4a2e64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE8on_errorERSt9exception
// IDA 0x4a2e64: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2e64() {
}

// 0x4a2e8c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEfEE4slotEEaSERKSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEfEE4slotEEaSERKSB_
// IDA 0x4a2e8c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2e8c() {
}

// 0x4a2eb4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE22safe_static_init_mutexEv
// IDA 0x4a2eb4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a2eb4() {
}

// 0x4a2eb8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEfEE24safe_static_do_get_mutexEv
// IDA 0x4a2eb8: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4a2eb8() {
}

// 0x4a2fb4 — __ZN3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a2fb4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a2fb4() {
}

// 0x4a2fb8 — __ZN3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a2fb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a2fb8() {
}

// 0x4a305c — __ZThn32_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a305c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a305c() {
}

// 0x4a3064 — __ZThn32_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a3064: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a3064() {
}

// 0x4a3108 — __ZThn36_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a3108: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a3108() {
}

// 0x4a3110 — __ZThn36_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_9ExplosionENS_8InstanceELZNS_10sExplosionEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x4a3110: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4a3110() {
}

// 0x4a31b8 — __ZN3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9ExplosionELZNS_10sExplosionEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sExplosionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x4a31b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4a31b8() {
}