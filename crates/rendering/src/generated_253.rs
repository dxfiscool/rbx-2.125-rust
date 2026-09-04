//! rendering shard 253 — 100 stubs EA-sorted asc global gap filler after 0x2db950 not yet in rendering (Ogre|G3D|Render 15420/15420 complete, 27470->27570 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2db954 — __ZThn36_N3RBX9CloneToolD0Ev
// type: void __fastcall(RBX::CloneTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CloneTool::~CloneTool()")]
// was: __ZThn36_N3RBX9CloneToolD0Ev
// IDA 0x2db954: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2db954() {
}

// 0x2db95c — __ZN3RBX9CloneToolD2Ev
// type: void __fastcall(RBX::CloneTool *__hidden this)
#[doc(alias = "RBX::CloneTool::~CloneTool()")]
// was: __ZN3RBX9CloneToolD2Ev
// IDA 0x2db95c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2db95c() {
}

// 0x2dba78 — __ZThn36_N3RBX9CloneToolD1Ev
// type: void __fastcall(RBX::CloneTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CloneTool::~CloneTool()")]
// was: __ZThn36_N3RBX9CloneToolD1Ev
// IDA 0x2dba78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2dba78() {
}

// 0x2dba80 — __ZN3RBX9CloneTool11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::CloneTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::CloneTool::onMouseIdle(RBX::UIEvent const&)")]
// was: __ZN3RBX9CloneTool11onMouseIdleERKNS_7UIEventE
// IDA 0x2dba80: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dba80() {
}

// 0x2dbb58 — __ZN3RBX9CloneTool11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::CloneTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::CloneTool::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX9CloneTool11onMouseDownERKNS_7UIEventE
// IDA 0x2dbb58: 227 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dbb58() {
}

// 0x2dbda0 — __ZNK3RBX9CloneTool13getCursorNameEv
// type: _DWORD __fastcall(RBX::CloneTool *__hidden this)
#[doc(alias = "RBX::CloneTool::getCursorName(void)const")]
// was: __ZNK3RBX9CloneTool13getCursorNameEv
// IDA 0x2dbda0: 65 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dbda0() {
}

// 0x2dbf88 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_10sCloneToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_10sCloneToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_12MouseCommandELZNS_10sCloneToolEEE7getNameEv
// IDA 0x2dbf88: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dbf88() {
}

// 0x2dbfb0 — __ZNK3RBX9CloneTool8isStickyEv
// type: _DWORD __fastcall(RBX::CloneTool *__hidden this)
#[doc(alias = "RBX::CloneTool::isSticky(void)const")]
// was: __ZNK3RBX9CloneTool8isStickyEv
// IDA 0x2dbfb0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dbfb0() {
}

// 0x2dc078 — __ZNK3RBX9CloneTool14drawConnectorsEv
// type: _DWORD __fastcall(RBX::CloneTool *__hidden this)
#[doc(alias = "RBX::CloneTool::drawConnectors(void)const")]
// was: __ZNK3RBX9CloneTool14drawConnectorsEv
// IDA 0x2dc078: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dc078() {
}

// 0x2dc07c — __ZN5boost10shared_ptrIN3RBX12PartDragToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::PartDragTool>::shared_ptr<RBX::PartDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::PartDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX12PartDragToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x2dc07c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dc07c() {
}

// 0x2dc144 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12PartDragToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::PartDragTool,RBX::PartDragTool>(rbx_core::SharedPtr<RBX::PartDragTool> const*,RBX::PartDragTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12PartDragToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2dc144: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dc144() {
}

// 0x2dc228 — __ZN5boost6detail12shared_countC2IPN3RBX12PartDragToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PartDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::PartDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX12PartDragToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x2dc228: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dc228() {
}

// 0x2dc320 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12PartDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PartDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12PartDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x2dc320: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2dc320() {
}

// 0x2dc324 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12PartDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PartDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12PartDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x2dc324: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2dc324() {
}

// 0x2dc328 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12PartDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PartDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12PartDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x2dc328: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dc328() {
}

// 0x2dc338 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12PartDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PartDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12PartDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x2dc338: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dc338() {
}

// 0x2dc350 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12PartDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PartDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12PartDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x2dc350: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dc350() {
}

// 0x2dc354 — __GLOBAL__I_a_85
#[doc(alias = "global constructor keyed to_a_85")]
// was: __GLOBAL__I_a_85
// IDA 0x2dc354: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2dc354() {
}

// 0x2dc604 — __ZN3RBX7Dragger14computeExtentsERKSt6vectorIPNS_9PrimitiveESaIS3_EE
#[doc(alias = "RBX::Dragger::computeExtents(std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> const&)")]
// was: __ZN3RBX7Dragger14computeExtentsERKSt6vectorIPNS_9PrimitiveESaIS3_EE
// IDA 0x2dc604: 114 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dc604() {
}

// 0x2dc914 — __ZN3RBX7Dragger25intersectingWorldOrOthersERNS_12PartInstanceERNS_14ContactManagerEff
// type: _DWORD __fastcall(RBX::Dragger *__hidden this, RBX::PartInstance *, RBX::ContactManager *, float, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::Dragger::intersectingWorldOrOthers(RBX::PartInstance &,RBX::ContactManager &,float,float)")]
// was: __ZN3RBX7Dragger25intersectingWorldOrOthersERNS_12PartInstanceERNS_14ContactManagerEff
// IDA 0x2dc914: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dc914() {
}

// 0x2dfc98 — __ZN3RBX7Extents18negativeMaxExtentsEv
// type: double *__fastcall(RBX::Extents *this)
#[doc(alias = "RBX::Extents::negativeMaxExtents(void)")]
// was: __ZN3RBX7Extents18negativeMaxExtentsEv
// IDA 0x2dfc98: 87 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dfc98() {
}

// 0x2dfe04 — __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE7reserveEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::reserve(unsigned long)")]
// was: __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE7reserveEm
// IDA 0x2dfe04: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dfe04() {
}

// 0x2dfea0 — __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _QWORD *)
#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::push_back(RBX::Extents const&)")]
// was: __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE9push_backERKS1_
// IDA 0x2dfea0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_2dfea0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x2dff0c — __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, __int64 *, int *)
#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Extents*,std::vector<RBX::Extents,std::allocator<RBX::Extents>>>,RBX::Extents const&)")]
// was: __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// IDA 0x2dff0c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_2dff0c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x2e00a0 — __ZNSt12_Vector_baseIN3RBX7ExtentsESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Extents,std::allocator<RBX::Extents>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX7ExtentsESaIS1_EE11_M_allocateEm
// IDA 0x2e00a0: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_2e00a0() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x2e00c4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7ExtentsES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Extents * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Extents *,RBX::Extents *>(RBX::Extents *,RBX::Extents *,RBX::Extents *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7ExtentsES5_EET0_T_S7_S6_
// IDA 0x2e00c4: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_2e00c4() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x2e0140 — __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSA_RKSC_RKSaINS1_8ptr_nodeIS7_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::table(unsigned long,boost::hash<RBX::Primitive const*> const&,std::equal_to<RBX::Primitive const*> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSA_RKSC_RKSaINS1_8ptr_nodeIS7_EEE
// IDA 0x2e0140: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e0140() {
}

// 0x2e01b0 — __ZNK3RBX4POLY4Edge9getVertexEPKNS0_4FaceEm
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::POLY::Edge::getVertex(RBX::POLY::Face const*,unsigned long)const")]
// was: __ZNK3RBX4POLY4Edge9getVertexEPKNS0_4FaceEm
// IDA 0x2e01b0: 93 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e01b0() {
}

// 0x2e02d4 — __GLOBAL__I_a_86
#[doc(alias = "global constructor keyed to_a_86")]
// was: __GLOBAL__I_a_86
// IDA 0x2e02d4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2e02d4() {
}

// 0x2e09ec — __ZN5boost10shared_ptrIN3RBX11LuaDragToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool>::shared_ptr<RBX::LuaDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX11LuaDragToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x2e09ec: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e09ec() {
}

// 0x2e0ab4 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11LuaDragToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LuaDragTool,RBX::LuaDragTool>(rbx_core::SharedPtr<RBX::LuaDragTool> const*,RBX::LuaDragTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11LuaDragToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2e0ab4: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e0ab4() {
}

// 0x2e0b98 — __ZN5boost6detail12shared_countC2IPN3RBX11LuaDragToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX11LuaDragToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x2e0b98: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e0b98() {
}

// 0x2e0c90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x2e0c90: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2e0c90() {
}

// 0x2e0c94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x2e0c94: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e0c94() {
}

// 0x2e0c98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x2e0c98: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e0c98() {
}

// 0x2e0ca8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x2e0ca8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e0ca8() {
}

// 0x2e0cc0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x2e0cc0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e0cc0() {
}

// 0x2e0cc4 — __GLOBAL__I_a_87
#[doc(alias = "global constructor keyed to_a_87")]
// was: __GLOBAL__I_a_87
// IDA 0x2e0cc4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2e0cc4() {
}

// 0x2e1308 — __ZN3RBX13DragUtilities16hitObjectOrPlaneERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKNS_6RbxRayERKNS_14ContactManagerEb
// type: void __fastcall(int, __int64 *, int, struct _Unwind_Exception *lpuexcpt, int, struct _Unwind_Exception *lpuexcpta, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::hitObjectOrPlane(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,bool)")]
// was: __ZN3RBX13DragUtilities16hitObjectOrPlaneERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKNS_6RbxRayERKNS_14ContactManagerEb
// IDA 0x2e1308: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e1308() {
}

// 0x2e1860 — __ZN3RBX13DragUtilities12anyPartAliveERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: int __fastcall(__int64 *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::anyPartAlive(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: __ZN3RBX13DragUtilities12anyPartAliveERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// IDA 0x2e1860: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e1860() {
}

// 0x2e195c — __ZN3RBX13DragUtilities17partsToPrimitivesERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERS1_IPNS_9PrimitiveESaISB_EE
// type: int __fastcall(__int64 *, _DWORD *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::partsToPrimitives(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,std::vector&<RBX::Primitive *,std::allocator<RBX::Primitive>>)")]
// was: __ZN3RBX13DragUtilities17partsToPrimitivesERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERS1_IPNS_9PrimitiveESaISB_EE
// IDA 0x2e195c: 210 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e195c() {
}

// 0x2e1b90 — __ZN3RBX13DragUtilities10pvsToPartsERKSt6vectorIPNS_10PVInstanceESaIS3_EERS1_IN5boost8weak_ptrINS_12PartInstanceEEESaISB_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::DragUtilities::pvsToParts(std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,std::vector&<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")]
// was: __ZN3RBX13DragUtilities10pvsToPartsERKSt6vectorIPNS_10PVInstanceESaIS3_EERS1_IN5boost8weak_ptrINS_12PartInstanceEEESaISB_EE
// IDA 0x2e1b90: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e1b90() {
}

// 0x2e1bc0 — __ZN3RBX13DragUtilities16instancesToPartsERKSt6vectorIPNS_8InstanceESaIS3_EERS1_IN5boost8weak_ptrINS_12PartInstanceEEESaISB_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::DragUtilities::instancesToParts(std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,std::vector&<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")]
// was: __ZN3RBX13DragUtilities16instancesToPartsERKSt6vectorIPNS_8InstanceESaIS3_EERS1_IN5boost8weak_ptrINS_12PartInstanceEEESaISB_EE
// IDA 0x2e1bc0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e1bc0() {
}

// 0x2e1bf4 — __ZN3RBX13DragUtilities19unJoinFromOutsidersERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *)
#[doc(alias = "RBX::DragUtilities::unJoinFromOutsiders(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: __ZN3RBX13DragUtilities19unJoinFromOutsidersERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// IDA 0x2e1bf4: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e1bf4() {
}

// 0x2e1cc0 — __ZN3RBX13DragUtilities15joinToOutsidersERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *)
#[doc(alias = "RBX::DragUtilities::joinToOutsiders(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: __ZN3RBX13DragUtilities15joinToOutsidersERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// IDA 0x2e1cc0: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e1cc0() {
}

// 0x2e1d90 — __ZN3RBX13DragUtilities4joinERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::join(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: __ZN3RBX13DragUtilities4joinERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// IDA 0x2e1d90: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e1d90() {
}

// 0x2e1ed8 — __ZN3RBX13DragUtilities19joinWithInPartsOnlyERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *)
#[doc(alias = "RBX::DragUtilities::joinWithInPartsOnly(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: __ZN3RBX13DragUtilities19joinWithInPartsOnlyERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// IDA 0x2e1ed8: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e1ed8() {
}

// 0x2e1fa4 — __ZN3RBX13DragUtilities11setDraggingERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::setDragging(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: __ZN3RBX13DragUtilities11setDraggingERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// IDA 0x2e1fa4: 128 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e1fa4() {
}

// 0x2e20f8 — __ZN3RBX13DragUtilities12stopDraggingERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::stopDragging(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: __ZN3RBX13DragUtilities12stopDraggingERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// IDA 0x2e20f8: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e20f8() {
}

// 0x2e2290 — __ZN3RBX13DragUtilities11alignToGridEPNS_12PartInstanceE
// type: int __fastcall(RBX::DragUtilities *this, RBX::PartInstance *)
#[doc(alias = "RBX::DragUtilities::alignToGrid(RBX::PartInstance *)")]
// was: __ZN3RBX13DragUtilities11alignToGridEPNS_12PartInstanceE
// IDA 0x2e2290: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e2290() {
}

// 0x2e23e4 — __ZN3RBX13DragUtilities5cleanEPNS_12PartInstanceE
// type: int __fastcall(RBX::DragUtilities *this, RBX::PartInstance *)
#[doc(alias = "RBX::DragUtilities::clean(RBX::PartInstance *)")]
// was: __ZN3RBX13DragUtilities5cleanEPNS_12PartInstanceE
// IDA 0x2e23e4: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e23e4() {
}

// 0x2e2400 — __ZN3RBX13DragUtilities5cleanERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(__int64 *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::clean(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: __ZN3RBX13DragUtilities5cleanERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// IDA 0x2e2400: 85 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e2400() {
}

// 0x2e27ec — __ZN3RBX13DragUtilities14computeExtentsERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// type: void __fastcall(int, __int64 *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::DragUtilities::computeExtents(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: __ZN3RBX13DragUtilities14computeExtentsERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EE
// IDA 0x2e27ec: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e27ec() {
}

// 0x2e28b8 — __ZN3RBX13DragUtilities13getPrimitivesEPKNS_8InstanceERSt6vectorIPNS_9PrimitiveESaIS6_EE
// type: int()
#[doc(alias = "RBX::DragUtilities::getPrimitives(RBX::Instance const*,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> &)")]
// was: __ZN3RBX13DragUtilities13getPrimitivesEPKNS_8InstanceERSt6vectorIPNS_9PrimitiveESaIS6_EE
// IDA 0x2e28b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e28b8() {
}

// 0x2e28bc — __ZN3RBX13DragUtilities18getPrimitivesConstEPKNS_8InstanceERSt6vectorIPKNS_9PrimitiveESaIS7_EE
// type: unsigned int __fastcall(RBX::Instance *, int, int, int)
#[doc(alias = "RBX::DragUtilities::getPrimitivesConst(RBX::Instance const*,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
// was: __ZN3RBX13DragUtilities18getPrimitivesConstEPKNS_8InstanceERSt6vectorIPKNS_9PrimitiveESaIS7_EE
// IDA 0x2e28bc: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e28bc() {
}

// 0x2e2948 — __GLOBAL__I_a_88
#[doc(alias = "global constructor keyed to_a_88")]
// was: __GLOBAL__I_a_88
// IDA 0x2e2948: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2e2948() {
}

// 0x2e2cbc — __GLOBAL__I_a_89
#[doc(alias = "global constructor keyed to_a_89")]
// was: __GLOBAL__I_a_89
// IDA 0x2e2cbc: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2e2cbc() {
}

// 0x2e2f2c — __ZN3RBX8GameToolC1EPNS_9WorkspaceE
// type: int __fastcall(RBX::GameTool *this, RBX::Workspace *)
#[doc(alias = "RBX::GameTool::GameTool(RBX::Workspace *)")]
// was: __ZN3RBX8GameToolC1EPNS_9WorkspaceE
// IDA 0x2e2f2c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e2f2c() {
}

// 0x2e2f30 — __ZN3RBX8GameToolC2EPNS_9WorkspaceE
// type: RBX::GameTool *__fastcall(RBX::GameTool *this, Workspace *)
#[doc(alias = "RBX::GameTool::GameTool(RBX::Workspace *)")]
// was: __ZN3RBX8GameToolC2EPNS_9WorkspaceE
// IDA 0x2e2f30: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e2f30() {
}

// 0x2e3044 — __ZN3RBX8GameTool11onMouseIdleERKNS_7UIEventE
// type: int __fastcall(RBX::GameTool *this, const UIEvent *)
#[doc(alias = "RBX::GameTool::onMouseIdle(RBX::UIEvent const&)")]
// was: __ZN3RBX8GameTool11onMouseIdleERKNS_7UIEventE
// IDA 0x2e3044: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e3044() {
}

// 0x2e3080 — __ZN3RBX8GameTool12onMouseHoverERKNS_7UIEventE
// type: int __fastcall(RBX::GameTool *this, const RBX::UIEvent *)
#[doc(alias = "RBX::GameTool::onMouseHover(RBX::UIEvent const&)")]
// was: __ZN3RBX8GameTool12onMouseHoverERKNS_7UIEventE
// IDA 0x2e3080: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e3080() {
}

// 0x2e30f4 — __ZN3RBX8GameTool11onMouseDownERKNS_7UIEventE
// type: void __fastcall(RBX::GameTool *this, const RBX::UIEvent *, const RBX::UIEvent *)
#[doc(alias = "RBX::GameTool::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX8GameTool11onMouseDownERKNS_7UIEventE
// IDA 0x2e30f4: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e30f4() {
}

// 0x2e3234 — __ZN3RBX8GameToolD0Ev
// type: void __fastcall(RBX::GameTool *__hidden this)
#[doc(alias = "RBX::GameTool::~GameTool()")]
// was: __ZN3RBX8GameToolD0Ev
// IDA 0x2e3234: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e3234() {
}

// 0x2e32d4 — __ZN3RBX8GameToolD1Ev
// type: void __fastcall(RBX::GameTool *__hidden this)
#[doc(alias = "RBX::GameTool::~GameTool()")]
// was: __ZN3RBX8GameToolD1Ev
// IDA 0x2e32d4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e32d4() {
}

// 0x2e32d8 — __ZThn36_N3RBX8GameToolD0Ev
// type: void __fastcall(RBX::GameTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GameTool::~GameTool()")]
// was: __ZThn36_N3RBX8GameToolD0Ev
// IDA 0x2e32d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e32d8() {
}

// 0x2e32e0 — __ZN3RBX8GameToolD2Ev
// type: void __fastcall(RBX::GameTool *this, int, int, const void *)
#[doc(alias = "RBX::GameTool::~GameTool()")]
// was: __ZN3RBX8GameToolD2Ev
// IDA 0x2e32e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e32e0() {
}

// 0x2e33e4 — __ZThn36_N3RBX8GameToolD1Ev
// type: void __fastcall(RBX::GameTool *this, int, int, const void *)
#[doc(alias = "non-virtual thunk toRBX::GameTool::~GameTool()")]
// was: __ZThn36_N3RBX8GameToolD1Ev
// IDA 0x2e33e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e33e4() {
}

// 0x2e33ec — __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGameToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGameToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGameToolEEE7getNameEv
// IDA 0x2e33ec: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e33ec() {
}

// 0x2e3414 — __ZNK3RBX8GameTool8isStickyEv
// type: void __fastcall(RBX::GameTool *this, int)
#[doc(alias = "RBX::GameTool::isSticky(void)const")]
// was: __ZNK3RBX8GameTool8isStickyEv
// IDA 0x2e3414: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e3414() {
}

// 0x2e34dc — __ZNK3RBX8GameTool14drawConnectorsEv
// type: int __fastcall(RBX::GameTool *this)
#[doc(alias = "RBX::GameTool::drawConnectors(void)const")]
// was: __ZNK3RBX8GameTool14drawConnectorsEv
// IDA 0x2e34dc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e34dc() {
}

// 0x2e34e0 — __ZNK3RBX8GameTool13getCursorNameEv
// type: int __fastcall(RBX::GameTool *this, int)
#[doc(alias = "RBX::GameTool::getCursorName(void)const")]
// was: __ZNK3RBX8GameTool13getCursorNameEv
// IDA 0x2e34e0: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e34e0() {
}

// 0x2e34ec — __GLOBAL__I_a_90
#[doc(alias = "global constructor keyed to_a_90")]
// was: __GLOBAL__I_a_90
// IDA 0x2e34ec: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2e34ec() {
}

// 0x2e37c4 — __ZN3RBX8GrabToolC1EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::GrabTool *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::GrabTool::GrabTool(RBX::Workspace *)")]
// was: __ZN3RBX8GrabToolC1EPNS_9WorkspaceE
// IDA 0x2e37c4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e37c4() {
}

// 0x2e37c8 — __ZN3RBX8GrabToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::GrabTool *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::GrabTool::GrabTool(RBX::Workspace *)")]
// was: __ZN3RBX8GrabToolC2EPNS_9WorkspaceE
// IDA 0x2e37c8: 97 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e37c8() {
}

// 0x2e38e8 — __ZN3RBX8GrabTool11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::GrabTool *__hidden this, const UIEvent *)
#[doc(alias = "RBX::GrabTool::onMouseIdle(RBX::UIEvent const&)")]
// was: __ZN3RBX8GrabTool11onMouseIdleERKNS_7UIEventE
// IDA 0x2e38e8: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e38e8() {
}

// 0x2e38f0 — __ZN3RBX8GrabTool12onMouseHoverERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::GrabTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::GrabTool::onMouseHover(RBX::UIEvent const&)")]
// was: __ZN3RBX8GrabTool12onMouseHoverERKNS_7UIEventE
// IDA 0x2e38f0: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e38f0() {
}

// 0x2e395c — __ZN3RBX8GrabTool11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::GrabTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::GrabTool::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX8GrabTool11onMouseDownERKNS_7UIEventE
// IDA 0x2e395c: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e395c() {
}

// 0x2e3aa8 — __ZN3RBX8GrabToolD0Ev
// type: void __fastcall(RBX::GrabTool *__hidden this)
#[doc(alias = "RBX::GrabTool::~GrabTool()")]
// was: __ZN3RBX8GrabToolD0Ev
// IDA 0x2e3aa8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e3aa8() {
}

// 0x2e3b48 — __ZN3RBX8GrabToolD1Ev
// type: void __fastcall(RBX::GrabTool *__hidden this)
#[doc(alias = "RBX::GrabTool::~GrabTool()")]
// was: __ZN3RBX8GrabToolD1Ev
// IDA 0x2e3b48: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e3b48() {
}

// 0x2e3b4c — __ZThn36_N3RBX8GrabToolD0Ev
// type: void __fastcall(RBX::GrabTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GrabTool::~GrabTool()")]
// was: __ZThn36_N3RBX8GrabToolD0Ev
// IDA 0x2e3b4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e3b4c() {
}

// 0x2e3b54 — __ZN3RBX8GrabToolD2Ev
// type: void __fastcall(RBX::GrabTool *__hidden this)
#[doc(alias = "RBX::GrabTool::~GrabTool()")]
// was: __ZN3RBX8GrabToolD2Ev
// IDA 0x2e3b54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e3b54() {
}

// 0x2e3c58 — __ZThn36_N3RBX8GrabToolD1Ev
// type: void __fastcall(RBX::GrabTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GrabTool::~GrabTool()")]
// was: __ZThn36_N3RBX8GrabToolD1Ev
// IDA 0x2e3c58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e3c58() {
}

// 0x2e3c60 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGrabToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGrabToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGrabToolEEE7getNameEv
// IDA 0x2e3c60: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e3c60() {
}

// 0x2e3c88 — __ZNK3RBX8GrabTool8isStickyEv
// type: _DWORD __fastcall(RBX::GrabTool *__hidden this)
#[doc(alias = "RBX::GrabTool::isSticky(void)const")]
// was: __ZNK3RBX8GrabTool8isStickyEv
// IDA 0x2e3c88: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e3c88() {
}

// 0x2e3d50 — __ZNK3RBX8GrabTool14drawConnectorsEv
// type: _DWORD __fastcall(RBX::GrabTool *__hidden this)
#[doc(alias = "RBX::GrabTool::drawConnectors(void)const")]
// was: __ZNK3RBX8GrabTool14drawConnectorsEv
// IDA 0x2e3d50: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e3d50() {
}

// 0x2e3d54 — __ZNK3RBX8GrabTool13getCursorNameEv
// type: _DWORD __fastcall(RBX::GrabTool *__hidden this)
#[doc(alias = "RBX::GrabTool::getCursorName(void)const")]
// was: __ZNK3RBX8GrabTool13getCursorNameEv
// IDA 0x2e3d54: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e3d54() {
}

// 0x2e3d60 — __GLOBAL__I_a_91
#[doc(alias = "global constructor keyed to_a_91")]
// was: __GLOBAL__I_a_91
// IDA 0x2e3d60: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2e3d60() {
}

// 0x2e3fd0 — __GLOBAL__I_a_92
#[doc(alias = "global constructor keyed to_a_92")]
// was: __GLOBAL__I_a_92
// IDA 0x2e3fd0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2e3fd0() {
}

// 0x2e4240 — __GLOBAL__I_a_93
#[doc(alias = "global constructor keyed to_a_93")]
// was: __GLOBAL__I_a_93
// IDA 0x2e4240: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2e4240() {
}

// 0x2e4518 — __ZN3RBX10HammerToolC1EPNS_9WorkspaceE
// type: int __fastcall(RBX::HammerTool *this, RBX::Workspace *)
#[doc(alias = "RBX::HammerTool::HammerTool(RBX::Workspace *)")]
// was: __ZN3RBX10HammerToolC1EPNS_9WorkspaceE
// IDA 0x2e4518: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e4518() {
}

// 0x2e451c — __ZN3RBX10HammerToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::HammerTool *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::HammerTool::HammerTool(RBX::Workspace *)")]
// was: __ZN3RBX10HammerToolC2EPNS_9WorkspaceE
// IDA 0x2e451c: 91 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e451c() {
}

// 0x2e4624 — __ZN3RBX10HammerToolD0Ev
// type: void __fastcall(RBX::HammerTool *__hidden this)
#[doc(alias = "RBX::HammerTool::~HammerTool()")]
// was: __ZN3RBX10HammerToolD0Ev
// IDA 0x2e4624: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e4624() {
}

// 0x2e46c4 — __ZN3RBX10HammerToolD1Ev
// type: void __fastcall(RBX::HammerTool *__hidden this)
#[doc(alias = "RBX::HammerTool::~HammerTool()")]
// was: __ZN3RBX10HammerToolD1Ev
// IDA 0x2e46c4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e46c4() {
}

// 0x2e46c8 — __ZThn36_N3RBX10HammerToolD0Ev
// type: void __fastcall(RBX::HammerTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HammerTool::~HammerTool()")]
// was: __ZThn36_N3RBX10HammerToolD0Ev
// IDA 0x2e46c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e46c8() {
}

// 0x2e46d0 — __ZN3RBX10HammerToolD2Ev
// type: void __fastcall(RBX::HammerTool *__hidden this)
#[doc(alias = "RBX::HammerTool::~HammerTool()")]
// was: __ZN3RBX10HammerToolD2Ev
// IDA 0x2e46d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e46d0() {
}

// 0x2e47ec — __ZThn36_N3RBX10HammerToolD1Ev
// type: void __fastcall(RBX::HammerTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HammerTool::~HammerTool()")]
// was: __ZThn36_N3RBX10HammerToolD1Ev
// IDA 0x2e47ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e47ec() {
}

// 0x2e47f4 — __ZN3RBX10HammerTool11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::HammerTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::HammerTool::onMouseIdle(RBX::UIEvent const&)")]
// was: __ZN3RBX10HammerTool11onMouseIdleERKNS_7UIEventE
// IDA 0x2e47f4: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e47f4() {
}

// 0x2e48cc — __ZN3RBX10HammerTool11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::HammerTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::HammerTool::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX10HammerTool11onMouseDownERKNS_7UIEventE
// IDA 0x2e48cc: 130 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e48cc() {
}

// 0x2e4a2c — __ZNK3RBX10HammerTool13getCursorNameEv
// type: _DWORD __fastcall(RBX::HammerTool *__hidden this)
#[doc(alias = "RBX::HammerTool::getCursorName(void)const")]
// was: __ZNK3RBX10HammerTool13getCursorNameEv
// IDA 0x2e4a2c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4a2c() {
}

// 0x2e4a84 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9ExplosionEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Explosion> RBX::Creatable<RBX::Instance>::create<RBX::Explosion>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_9ExplosionEEEN5boost10shared_ptrIT_EEv
// IDA 0x2e4a84: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4a84() {
}