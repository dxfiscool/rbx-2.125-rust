//! rendering shard 254 — 150 stubs EA-sorted asc global gap filler after 0x2e4a84 not yet in rendering (Ogre|G3D|Render 15420/15420 complete, 27570->27720 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2e4b34 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_11sHammerToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_11sHammerToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_12MouseCommandELZNS_11sHammerToolEEE7getNameEv
// IDA 0x2e4b34: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4b34() {
}

// 0x2e4b5c — __ZNK3RBX10HammerTool8isStickyEv
// type: _DWORD __fastcall(RBX::HammerTool *__hidden this)
#[doc(alias = "RBX::HammerTool::isSticky(void)const")]
// was: __ZNK3RBX10HammerTool8isStickyEv
// IDA 0x2e4b5c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4b5c() {
}

// 0x2e4c24 — __ZN5boost10shared_ptrIN3RBX9ExplosionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Explosion>::shared_ptr<RBX::Explosion,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX9ExplosionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x2e4c24: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4c24() {
}

// 0x2e4cec — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9ExplosionES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Explosion,RBX::Explosion>(rbx_core::SharedPtr<RBX::Explosion> const*,RBX::Explosion *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9ExplosionES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2e4cec: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4cec() {
}

// 0x2e4dd8 — __ZN5boost6detail12shared_countC2IPN3RBX9ExplosionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX9ExplosionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x2e4dd8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4dd8() {
}

// 0x2e4ee0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x2e4ee0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2e4ee0() {
}

// 0x2e4ee4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x2e4ee4: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4ee4() {
}

// 0x2e4f04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x2e4f04: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4f04() {
}

// 0x2e4f1c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Explosion *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ExplosionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x2e4f1c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4f1c() {
}

// 0x2e4f20 — __GLOBAL__I_a_94
#[doc(alias = "global constructor keyed to_a_94")]
// was: __GLOBAL__I_a_94
// IDA 0x2e4f20: 222 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4f20() {
}

// 0x2e56d4 — __ZN3RBX10LuaDragger9mouseMoveENS_6RbxRayE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::LuaDragger::mouseMove(RBX::RbxRay)")]
// was: __ZN3RBX10LuaDragger9mouseMoveENS_6RbxRayE
// IDA 0x2e56d4: 251 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e56d4() {
}

// 0x2e59b4 — __ZN3RBX10LuaDragger7mouseUpEv
// type: _DWORD __fastcall(RBX::LuaDragger *__hidden this)
#[doc(alias = "RBX::LuaDragger::mouseUp(void)")]
// was: __ZN3RBX10LuaDragger7mouseUpEv
// IDA 0x2e59b4: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e59b4() {
}

// 0x2e5c24 — __ZN3RBX10LuaDraggerC2Ev
// type: _DWORD __fastcall(RBX::LuaDragger *__hidden this)
#[doc(alias = "RBX::LuaDragger::LuaDragger(void)")]
// was: __ZN3RBX10LuaDraggerC2Ev
// IDA 0x2e5c24: 171 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e5c24() {
}

// 0x2e5e10 — __ZN3RBX10LuaDraggerD0Ev
// type: void __fastcall(RBX::LuaDragger *__hidden this)
#[doc(alias = "RBX::LuaDragger::~LuaDragger()")]
// was: __ZN3RBX10LuaDraggerD0Ev
// IDA 0x2e5e10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e5e10() {
}

// 0x2e5eb0 — __ZN3RBX10LuaDraggerD1Ev
// type: void __fastcall(RBX::LuaDragger *__hidden this)
#[doc(alias = "RBX::LuaDragger::~LuaDragger()")]
// was: __ZN3RBX10LuaDraggerD1Ev
// IDA 0x2e5eb0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e5eb0() {
}

// 0x2e5eb4 — __ZThn32_N3RBX10LuaDraggerD0Ev
// type: void __fastcall(RBX::LuaDragger *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaDragger::~LuaDragger()")]
// was: __ZThn32_N3RBX10LuaDraggerD0Ev
// IDA 0x2e5eb4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e5eb4() {
}

// 0x2e5ebc — __ZThn36_N3RBX10LuaDraggerD0Ev
// type: void __fastcall(RBX::LuaDragger *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaDragger::~LuaDragger()")]
// was: __ZThn36_N3RBX10LuaDraggerD0Ev
// IDA 0x2e5ebc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e5ebc() {
}

// 0x2e5ec4 — __ZN3RBX10LuaDraggerD2Ev
// type: void __fastcall(RBX::LuaDragger *__hidden this)
#[doc(alias = "RBX::LuaDragger::~LuaDragger()")]
// was: __ZN3RBX10LuaDraggerD2Ev
// IDA 0x2e5ec4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e5ec4() {
}

// 0x2e6060 — __ZThn32_N3RBX10LuaDraggerD1Ev
// type: void __fastcall(RBX::LuaDragger *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaDragger::~LuaDragger()")]
// was: __ZThn32_N3RBX10LuaDraggerD1Ev
// IDA 0x2e6060: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e6060() {
}

// 0x2e6068 — __ZThn36_N3RBX10LuaDraggerD1Ev
// type: void __fastcall(RBX::LuaDragger *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaDragger::~LuaDragger()")]
// was: __ZThn36_N3RBX10LuaDraggerD1Ev
// IDA 0x2e6068: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e6068() {
}

// 0x2e6314 — __ZN3RBX10LuaDragger16tryStartDraggingERKNS_6RbxRayE
// type: _DWORD __fastcall(RBX::LuaDragger *__hidden this, const RBX::RbxRay *)
#[doc(alias = "RBX::LuaDragger::tryStartDragging(RBX::RbxRay const&)")]
// was: __ZN3RBX10LuaDragger16tryStartDraggingERKNS_6RbxRayE
// IDA 0x2e6314: 161 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e6314() {
}

// 0x2e6524 — __ZN3RBX10LuaDragger6doDragERKNS_6RbxRayE
// type: _DWORD __fastcall(RBX::LuaDragger *__hidden this, const RBX::RbxRay *)
#[doc(alias = "RBX::LuaDragger::doDrag(RBX::RbxRay const&)")]
// was: __ZN3RBX10LuaDragger6doDragERKNS_6RbxRayE
// IDA 0x2e6524: 212 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e6524() {
}

// 0x2e68c8 — __ZN3RBX10LuaDragger13startDraggingEv
// type: _DWORD __fastcall(RBX::LuaDragger *__hidden this)
#[doc(alias = "RBX::LuaDragger::startDragging(void)")]
// was: __ZN3RBX10LuaDragger13startDraggingEv
// IDA 0x2e68c8: 257 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e68c8() {
}

// 0x2e6d94 — __ZN3RBXL7addPartEN5boost10shared_ptrINS_8InstanceEEEPSt6vectorINS0_8weak_ptrINS_12PartInstanceEEESaIS7_EE
#[doc(alias = "RBX::addPart(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *)")]
// was: __ZN3RBXL7addPartEN5boost10shared_ptrINS_8InstanceEEEPSt6vectorINS0_8weak_ptrINS_12PartInstanceEEESaIS7_EE
// IDA 0x2e6d94: 223 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e6d94() {
}

// 0x2e7010 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EED1Ev
// IDA 0x2e7010: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e7010() {
}

// 0x2e7108 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EED1Ev
// IDA 0x2e7108: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e7108() {
}

// 0x2e716c — __ZN5boost20dynamic_pointer_castIN3RBX12PartInstanceENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
// type: void *__fastcall(_DWORD *, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN5boost20dynamic_pointer_castIN3RBX12PartInstanceENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
// IDA 0x2e716c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e716c() {
}

// 0x2e71b4 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EEaSERKS6_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::operator=(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
// was: __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EEaSERKS6_
// IDA 0x2e71b4: 247 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e71b4() {
}

// 0x2e7428 — __ZNSt8auto_ptrIN3RBX10RunDraggerEE5resetEPS1_
#[doc(alias = "std::auto_ptr<RBX::RunDragger>::reset(RBX::RunDragger*)")]
// was: __ZNSt8auto_ptrIN3RBX10RunDraggerEE5resetEPS1_
// IDA 0x2e7428: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e7428() {
}

// 0x2e74dc — __ZNK3RBX10LuaDragger12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::LuaDragger *__hidden this, const Instance *)
#[doc(alias = "RBX::LuaDragger::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX10LuaDragger12askSetParentEPKNS_8InstanceE
// IDA 0x2e74dc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e74dc() {
}

// 0x2e74e0 — __ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E12getClassNameEv
// IDA 0x2e74e0: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e74e0() {
}

// 0x2e74f0 — __ZThn32_NK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E12getClassNameEv
// IDA 0x2e74f0: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e74f0() {
}

// 0x2e7500 — __ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD1Ev
// IDA 0x2e7500: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e7500() {
}

// 0x2e7504 — __ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD2Ev
// IDA 0x2e7504: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e7504() {
}

// 0x2e75a0 — __ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator12getClassNameEv
// IDA 0x2e75a0: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e75a0() {
}

// 0x2e7628 — __ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator6createEv
// IDA 0x2e7628: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e7628() {
}

// 0x2e776c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10LuaDraggerEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger> RBX::Creatable<RBX::Instance>::create<RBX::LuaDragger>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_10LuaDraggerEEEN5boost10shared_ptrIT_EEv
// IDA 0x2e776c: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e776c() {
}

// 0x2e781c — __ZN5boost10shared_ptrIN3RBX10LuaDraggerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger>::shared_ptr<RBX::LuaDragger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10LuaDraggerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x2e781c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e781c() {
}

// 0x2e78e4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10LuaDraggerES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaDragger,RBX::LuaDragger>(rbx_core::SharedPtr<RBX::LuaDragger> const*,RBX::LuaDragger *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10LuaDraggerES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2e78e4: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e78e4() {
}

// 0x2e79cc — __ZN5boost6detail12shared_countC2IPN3RBX10LuaDraggerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10LuaDraggerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x2e79cc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e79cc() {
}

// 0x2e7ad4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x2e7ad4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2e7ad4() {
}

// 0x2e7ad8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x2e7ad8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e7ad8() {
}

// 0x2e7adc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x2e7adc: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e7adc() {
}

// 0x2e7afc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x2e7afc: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e7afc() {
}

// 0x2e7b14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10LuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x2e7b14: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e7b14() {
}

// 0x2e7b18 — __ZN3RBX4Name13callDoDeclareILZNS_11sLuaDraggerEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sLuaDraggerEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_11sLuaDraggerEEEEvv
// IDA 0x2e7b18: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e7b18() {
}

// 0x2e7b1c — __ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v
// IDA 0x2e7b1c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e7b1c() {
}

// 0x2e7bfc — __ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorC2Ev
// IDA 0x2e7bfc: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e7bfc() {
}

// 0x2e7e40 — __ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E17static_getCreatorEv
// IDA 0x2e7e40: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e7e40() {
}

// 0x2e7eb4 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS4_S6_EEEEPS4_mT_SE_
// type: char *__fastcall(int, int, int, int)
#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance>* std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>>(unsigned long,__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>,__gnu_cxx::__normal_iterator<rbx_core::WeakPtr<RBX::PartInstance> const*,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>>)")]
// was: __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS4_S6_EEEEPS4_mT_SE_
// IDA 0x2e7eb4: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_2e7eb4() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x2e8078 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost8weak_ptrIN3RBX12PartInstanceEEES8_EET0_T_SA_S9_
#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *>(rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *,rbx_core::WeakPtr<RBX::PartInstance> *)")]
// was: __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost8weak_ptrIN3RBX12PartInstanceEEES8_EET0_T_SA_S9_
// IDA 0x2e8078: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_2e8078() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x2e80d0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN5boost8weak_ptrIN3RBX12PartInstanceEEEPS7_EET0_T_SC_SB_
#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance>* std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance>*>(rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance> const*,rbx_core::WeakPtr<RBX::PartInstance>*)")]
// was: __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN5boost8weak_ptrIN3RBX12PartInstanceEEEPS7_EET0_T_SC_SB_
// IDA 0x2e80d0: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_2e80d0() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x2e8128 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEES6_ET_SD_SD_RKT0_St26random_access_iterator_tag
// type: _DWORD *__fastcall(_DWORD *, int, int *)
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance> const&,std::random_access_iterator_tag)")]
// was: __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEES6_ET_SD_SD_RKT0_St26random_access_iterator_tag
// IDA 0x2e8128: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e8128() {
}

// 0x2e81b8 — __ZNSt8auto_ptrIN3RBX10RunDraggerEED2Ev
#[doc(alias = "std::auto_ptr<RBX::RunDragger>::~auto_ptr()")]
// was: __ZNSt8auto_ptrIN3RBX10RunDraggerEED2Ev
// IDA 0x2e81b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e81b8() {
}

// 0x2e8260 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5JointEEESaIS4_EED2Ev
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Joint>,std::allocator<rbx_core::SharedPtr<RBX::Joint>>>::~vector()")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX5JointEEESaIS4_EED2Ev
// IDA 0x2e8260: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e8260() {
}

// 0x2e832c — __ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x2e832c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e832c() {
}

// 0x2e8330 — __ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x2e8330: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e8330() {
}

// 0x2e83d0 — __ZThn32_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x2e83d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e83d0() {
}

// 0x2e83d8 — __ZThn32_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x2e83d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e83d8() {
}

// 0x2e847c — __ZThn36_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x2e847c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e847c() {
}

// 0x2e8484 — __ZThn36_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x2e8484: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e8484() {
}

// 0x2e89f0 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(void),0>::BoundFuncDesc(void (RBX::LuaDragger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2e89f0: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e89f0() {
}

// 0x2e8af4 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EED0Ev
// IDA 0x2e8af4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e8af4() {
}

// 0x2e8ba8 — __ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x2e8ba8: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e8ba8() {
}

// 0x2e8bc8 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(RBX::RbxRay),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2e8bc8: 143 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e8bc8() {
}

// 0x2e8d44 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x2e8d44: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e8d44() {
}

// 0x2e8d74 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EED0Ev
// IDA 0x2e8d74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e8d74() {
}

// 0x2e8e80 — __ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvNS_6RbxRayEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x2e8e80: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e8e80() {
}

// 0x2e96ac — __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISC_EEPNS3_10disable_ifINS3_7is_sameISC_NS4_IKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISC_EEPNS3_10disable_ifINS3_7is_sameISC_NS4_IKNS0_5TupleEEEEEvE4typeE
// IDA 0x2e96ac: 171 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e96ac() {
}

// 0x2e9870 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvPFvS6_PS9_INS2_8weak_ptrINS4_12PartInstanceEEESaISH_EEENSD_5list2INS2_3argILi1EEENSD_5valueISK_EEEEEEET0_T_SV_SU_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvPFvS6_PS9_INS2_8weak_ptrINS4_12PartInstanceEEESaISH_EEENSD_5list2INS2_3argILi1EEENSD_5valueISK_EEEEEEET0_T_SV_SU_
// IDA 0x2e9870: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e9870() {
}

// 0x2e98b8 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPSt6vectorINS_8weak_ptrIN3RBX12PartInstanceEEESaIS9_EEEEEclIPFvNS_10shared_ptrINS7_8InstanceEEESC_ENS0_5list1IRKSI_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPSt6vectorINS_8weak_ptrIN3RBX12PartInstanceEEESaIS9_EEEEEclIPFvNS_10shared_ptrINS7_8InstanceEEESC_ENS0_5list1IRKSI_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x2e98b8: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e98b8() {
}

// 0x2e9b08 — __GLOBAL__I_a_95
#[doc(alias = "global constructor keyed to_a_95")]
// was: __GLOBAL__I_a_95
// IDA 0x2e9b08: 349 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e9b08() {
}

// 0x2e9f50 — __ZNK3RBX11LuaDragTool13getCursorNameEv
// type: _DWORD __fastcall(RBX::LuaDragTool *__hidden this)
#[doc(alias = "RBX::LuaDragTool::getCursorName(void)const")]
// was: __ZNK3RBX11LuaDragTool13getCursorNameEv
// IDA 0x2e9f50: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e9f50() {
}

// 0x2ea20c — __ZN3RBX11LuaDragToolD0Ev
// type: void __fastcall(RBX::LuaDragTool *__hidden this)
#[doc(alias = "RBX::LuaDragTool::~LuaDragTool()")]
// was: __ZN3RBX11LuaDragToolD0Ev
// IDA 0x2ea20c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ea20c() {
}

// 0x2ea2ac — __ZN3RBX11LuaDragToolD1Ev
// type: void __fastcall(RBX::LuaDragTool *__hidden this)
#[doc(alias = "RBX::LuaDragTool::~LuaDragTool()")]
// was: __ZN3RBX11LuaDragToolD1Ev
// IDA 0x2ea2ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2ea2ac() {
}

// 0x2ea2b0 — __ZThn36_N3RBX11LuaDragToolD0Ev
// type: void __fastcall(RBX::LuaDragTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaDragTool::~LuaDragTool()")]
// was: __ZThn36_N3RBX11LuaDragToolD0Ev
// IDA 0x2ea2b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ea2b0() {
}

// 0x2ea2b8 — __ZN3RBX11LuaDragToolD2Ev
// type: void __fastcall(RBX::LuaDragTool *__hidden this)
#[doc(alias = "RBX::LuaDragTool::~LuaDragTool()")]
// was: __ZN3RBX11LuaDragToolD2Ev
// IDA 0x2ea2b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ea2b8() {
}

// 0x2ea3f8 — __ZThn36_N3RBX11LuaDragToolD1Ev
// type: void __fastcall(RBX::LuaDragTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaDragTool::~LuaDragTool()")]
// was: __ZThn36_N3RBX11LuaDragToolD1Ev
// IDA 0x2ea3f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ea3f8() {
}

// 0x2ea400 — __ZN3RBX11LuaDragTool11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::LuaDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::LuaDragTool::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX11LuaDragTool11onMouseDownERKNS_7UIEventE
// IDA 0x2ea400: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ea400() {
}

// 0x2ea4d4 — __ZN3RBX11LuaDragTool11onMouseMoveERKNS_7UIEventE
// type: int __fastcall(RBX::LuaDragTool *this, const RBX::UIEvent *, int)
#[doc(alias = "RBX::LuaDragTool::onMouseMove(RBX::UIEvent const&)")]
// was: __ZN3RBX11LuaDragTool11onMouseMoveERKNS_7UIEventE
// IDA 0x2ea4d4: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ea4d4() {
}

// 0x2ea550 — __ZN3RBX11LuaDragTool11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::LuaDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::LuaDragTool::onMouseIdle(RBX::UIEvent const&)")]
// was: __ZN3RBX11LuaDragTool11onMouseIdleERKNS_7UIEventE
// IDA 0x2ea550: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ea550() {
}

// 0x2ea5cc — __ZN3RBX11LuaDragTool9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::LuaDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::LuaDragTool::onMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX11LuaDragTool9onMouseUpERKNS_7UIEventE
// IDA 0x2ea5cc: 205 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ea5cc() {
}

// 0x2ea814 — __ZN3RBX11LuaDragTool9onKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::LuaDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::LuaDragTool::onKeyDown(RBX::UIEvent const&)")]
// was: __ZN3RBX11LuaDragTool9onKeyDownERKNS_7UIEventE
// IDA 0x2ea814: 128 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ea814() {
}

// 0x2ea984 — __ZN5boost10shared_ptrIN3RBX10LuaDraggerEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger>::operator=(rbx_core::SharedPtr<RBX::LuaDragger> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX10LuaDraggerEEaSERKS3_
// IDA 0x2ea984: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ea984() {
}

// 0x2ea9bc — __ZN3RBX11shared_fromINS_11LuaDragToolEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool> RBX::shared_from<RBX::LuaDragTool>(RBX::LuaDragTool*)")]
// was: __ZN3RBX11shared_fromINS_11LuaDragToolEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x2ea9bc: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ea9bc() {
}

// 0x2eab24 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_12sLuaDragToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_12sLuaDragToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_12MouseCommandELZNS_12sLuaDragToolEEE7getNameEv
// IDA 0x2eab24: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eab24() {
}

// 0x2eab4c — __ZN3RBX4Name13callDoDeclareILZNS_12sLuaDragToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sLuaDragToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_12sLuaDragToolEEEEvv
// IDA 0x2eab4c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2eab4c() {
}

// 0x2eab50 — __ZN3RBX4Name9doDeclareILZNS_12sLuaDragToolEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sLuaDragToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_12sLuaDragToolEEEERKS0_v
// IDA 0x2eab50: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eab50() {
}

// 0x2eac30 — __GLOBAL__I_a_96
#[doc(alias = "global constructor keyed to_a_96")]
// was: __GLOBAL__I_a_96
// IDA 0x2eac30: 202 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eac30() {
}

// 0x2eaea0 — __ZN3RBX11MegaDraggerC1EPNS_12PartInstanceERKSt6vectorIPNS_10PVInstanceESaIS5_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
// was: __ZN3RBX11MegaDraggerC1EPNS_12PartInstanceERKSt6vectorIPNS_10PVInstanceESaIS5_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
// IDA 0x2eaea0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2eaea0() {
}

// 0x2eaea4 — __ZN3RBX11MegaDraggerC2EPNS_12PartInstanceERKSt6vectorIPNS_10PVInstanceESaIS5_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
// was: __ZN3RBX11MegaDraggerC2EPNS_12PartInstanceERKSt6vectorIPNS_10PVInstanceESaIS5_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
// IDA 0x2eaea4: 115 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eaea4() {
}

// 0x2eafd4 — __ZN3RBX11MegaDraggerC1EPNS_12PartInstanceERKSt6vectorIN5boost8weak_ptrIS1_EESaIS6_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
// was: __ZN3RBX11MegaDraggerC1EPNS_12PartInstanceERKSt6vectorIN5boost8weak_ptrIS1_EESaIS6_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
// IDA 0x2eafd4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2eafd4() {
}

// 0x2eafd8 — __ZN3RBX11MegaDraggerC2EPNS_12PartInstanceERKSt6vectorIN5boost8weak_ptrIS1_EESaIS6_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
// was: __ZN3RBX11MegaDraggerC2EPNS_12PartInstanceERKSt6vectorIN5boost8weak_ptrIS1_EESaIS6_EEPNS_12RootInstanceENS_4DRAG8JoinTypeE
// IDA 0x2eafd8: 102 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eafd8() {
}

// 0x2eb0e8 — __ZN3RBX11MegaDraggerD1Ev
// type: void __fastcall(RBX::MegaDragger *__hidden this)
#[doc(alias = "RBX::MegaDragger::~MegaDragger()")]
// was: __ZN3RBX11MegaDraggerD1Ev
// IDA 0x2eb0e8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2eb0e8() {
}

// 0x2eb0ec — __ZN3RBX11MegaDraggerD2Ev
// type: void __fastcall(RBX::MegaDragger *__hidden this)
#[doc(alias = "RBX::MegaDragger::~MegaDragger()")]
// was: __ZN3RBX11MegaDraggerD2Ev
// IDA 0x2eb0ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2eb0ec() {
}

// 0x2eb224 — __ZN3RBX11MegaDragger13startDraggingEv
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this)
#[doc(alias = "RBX::MegaDragger::startDragging(void)")]
// was: __ZN3RBX11MegaDragger13startDraggingEv
// IDA 0x2eb224: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eb224() {
}

// 0x2eb248 — __ZN3RBX11MegaDragger16continueDraggingEv
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this)
#[doc(alias = "RBX::MegaDragger::continueDragging(void)")]
// was: __ZN3RBX11MegaDragger16continueDraggingEv
// IDA 0x2eb248: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eb248() {
}

// 0x2eb2b4 — __ZN3RBX11MegaDragger14finishDraggingEv
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this)
#[doc(alias = "RBX::MegaDragger::finishDragging(void)")]
// was: __ZN3RBX11MegaDragger14finishDraggingEv
// IDA 0x2eb2b4: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eb2b4() {
}

// 0x2eb380 — __ZN3RBX11MegaDragger18alignAndCleanPartsEv
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this)
#[doc(alias = "RBX::MegaDragger::alignAndCleanParts(void)")]
// was: __ZN3RBX11MegaDragger18alignAndCleanPartsEv
// IDA 0x2eb380: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eb380() {
}

// 0x2eb540 — __ZN3RBX11MegaDragger14mousePartAliveEv
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this)
#[doc(alias = "RBX::MegaDragger::mousePartAlive(void)")]
// was: __ZN3RBX11MegaDragger14mousePartAliveEv
// IDA 0x2eb540: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eb540() {
}

// 0x2ebf7c — __ZN3RBX11MegaDragger16anyDragPartAliveEv
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this)
#[doc(alias = "RBX::MegaDragger::anyDragPartAlive(void)")]
// was: __ZN3RBX11MegaDragger16anyDragPartAliveEv
// IDA 0x2ebf7c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ebf7c() {
}

// 0x2ebf88 — __GLOBAL__I_a_97
#[doc(alias = "global constructor keyed to_a_97")]
// was: __GLOBAL__I_a_97
// IDA 0x2ebf88: 287 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ebf88() {
}

// 0x2ec2fc — __ZN3RBX18MoveResizeJoinTool12findTargetPVERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MoveResizeJoinTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::MoveResizeJoinTool::findTargetPV(RBX::UIEvent const&)")]
// was: __ZN3RBX18MoveResizeJoinTool12findTargetPVERKNS_7UIEventE
// IDA 0x2ec2fc: 221 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ec2fc() {
}

// 0x2ed9dc — __ZN3RBX18MoveResizeJoinTool12onMouseHoverERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MoveResizeJoinTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::MoveResizeJoinTool::onMouseHover(RBX::UIEvent const&)")]
// was: __ZN3RBX18MoveResizeJoinTool12onMouseHoverERKNS_7UIEventE
// IDA 0x2ed9dc: 43 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ed9dc() {
}

// 0x2eda60 — __ZN3RBX18MoveResizeJoinTool11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MoveResizeJoinTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::MoveResizeJoinTool::onMouseIdle(RBX::UIEvent const&)")]
// was: __ZN3RBX18MoveResizeJoinTool11onMouseIdleERKNS_7UIEventE
// IDA 0x2eda60: 121 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eda60() {
}

// 0x2edbcc — __ZN3RBX18MoveResizeJoinTool11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MoveResizeJoinTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::MoveResizeJoinTool::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX18MoveResizeJoinTool11onMouseDownERKNS_7UIEventE
// IDA 0x2edbcc: 204 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2edbcc() {
}

// 0x2ede04 — __ZN3RBX18MoveResizeJoinTool13moveIncrementEv
// type: _DWORD __fastcall(RBX::MoveResizeJoinTool *__hidden this)
#[doc(alias = "RBX::MoveResizeJoinTool::moveIncrement(void)")]
// was: __ZN3RBX18MoveResizeJoinTool13moveIncrementEv
// IDA 0x2ede04: 134 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ede04() {
}

// 0x2edf9c — __ZN3RBX18MoveResizeJoinTool9onKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MoveResizeJoinTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::MoveResizeJoinTool::onKeyDown(RBX::UIEvent const&)")]
// was: __ZN3RBX18MoveResizeJoinTool9onKeyDownERKNS_7UIEventE
// IDA 0x2edf9c: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2edf9c() {
}

// 0x2ee084 — __ZN3RBX18MoveResizeJoinTool11onMouseMoveERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MoveResizeJoinTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::MoveResizeJoinTool::onMouseMove(RBX::UIEvent const&)")]
// was: __ZN3RBX18MoveResizeJoinTool11onMouseMoveERKNS_7UIEventE
// IDA 0x2ee084: 213 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ee084() {
}

// 0x2ee324 — __ZN3RBX18MoveResizeJoinTool12capturedDragEf
// type: _DWORD __fastcall(RBX::MoveResizeJoinTool *__hidden this, float)
#[doc(alias = "RBX::MoveResizeJoinTool::capturedDrag(float)")]
// was: __ZN3RBX18MoveResizeJoinTool12capturedDragEf
// IDA 0x2ee324: 157 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ee324() {
}

// 0x2ee4e4 — __ZN3RBX18MoveResizeJoinTool9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MoveResizeJoinTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::MoveResizeJoinTool::onMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX18MoveResizeJoinTool9onMouseUpERKNS_7UIEventE
// IDA 0x2ee4e4: 170 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ee4e4() {
}

// 0x2ee6b0 — __ZN3RBX11shared_fromINS_18MoveResizeJoinToolEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::MoveResizeJoinTool> RBX::shared_from<RBX::MoveResizeJoinTool>(RBX::MoveResizeJoinTool*)")]
// was: __ZN3RBX11shared_fromINS_18MoveResizeJoinToolEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x2ee6b0: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ee6b0() {
}

// 0x2ee818 — __ZN3RBX18MoveResizeJoinToolD1Ev
// type: void __fastcall(RBX::MoveResizeJoinTool *__hidden this)
#[doc(alias = "RBX::MoveResizeJoinTool::~MoveResizeJoinTool()")]
// was: __ZN3RBX18MoveResizeJoinToolD1Ev
// IDA 0x2ee818: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ee818() {
}

// 0x2ee900 — __ZN3RBX18MoveResizeJoinToolD0Ev
// type: void __fastcall(RBX::MoveResizeJoinTool *__hidden this)
#[doc(alias = "RBX::MoveResizeJoinTool::~MoveResizeJoinTool()")]
// was: __ZN3RBX18MoveResizeJoinToolD0Ev
// IDA 0x2ee900: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ee900() {
}

// 0x2ee9f8 — __ZThn36_N3RBX18MoveResizeJoinToolD1Ev
// type: void __fastcall(RBX::MoveResizeJoinTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::MoveResizeJoinTool::~MoveResizeJoinTool()")]
// was: __ZThn36_N3RBX18MoveResizeJoinToolD1Ev
// IDA 0x2ee9f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ee9f8() {
}

// 0x2eeadc — __ZThn36_N3RBX18MoveResizeJoinToolD0Ev
// type: void __fastcall(RBX::MoveResizeJoinTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::MoveResizeJoinTool::~MoveResizeJoinTool()")]
// was: __ZThn36_N3RBX18MoveResizeJoinToolD0Ev
// IDA 0x2eeadc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2eeadc() {
}

// 0x2eebd8 — __GLOBAL__I_a_98
#[doc(alias = "global constructor keyed to_a_98")]
// was: __GLOBAL__I_a_98
// IDA 0x2eebd8: 222 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eebd8() {
}

// 0x2eee88 — __ZN3RBX8NullToolC1EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::NullTool *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::NullTool::NullTool(RBX::Workspace *)")]
// was: __ZN3RBX8NullToolC1EPNS_9WorkspaceE
// IDA 0x2eee88: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2eee88() {
}

// 0x2eee8c — __ZN3RBX8NullToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::NullTool *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::NullTool::NullTool(RBX::Workspace *)")]
// was: __ZN3RBX8NullToolC2EPNS_9WorkspaceE
// IDA 0x2eee8c: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eee8c() {
}

// 0x2eef84 — __ZN3RBX8NullToolD0Ev
// type: void __fastcall(RBX::NullTool *__hidden this)
#[doc(alias = "RBX::NullTool::~NullTool()")]
// was: __ZN3RBX8NullToolD0Ev
// IDA 0x2eef84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2eef84() {
}

// 0x2ef024 — __ZN3RBX8NullToolD1Ev
// type: void __fastcall(RBX::NullTool *__hidden this)
#[doc(alias = "RBX::NullTool::~NullTool()")]
// was: __ZN3RBX8NullToolD1Ev
// IDA 0x2ef024: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2ef024() {
}

// 0x2ef028 — __ZThn36_N3RBX8NullToolD0Ev
// type: void __fastcall(RBX::NullTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NullTool::~NullTool()")]
// was: __ZThn36_N3RBX8NullToolD0Ev
// IDA 0x2ef028: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ef028() {
}

// 0x2ef030 — __ZN3RBX8NullToolD2Ev
// type: void __fastcall(RBX::NullTool *__hidden this)
#[doc(alias = "RBX::NullTool::~NullTool()")]
// was: __ZN3RBX8NullToolD2Ev
// IDA 0x2ef030: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ef030() {
}

// 0x2ef124 — __ZThn36_N3RBX8NullToolD1Ev
// type: void __fastcall(RBX::NullTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NullTool::~NullTool()")]
// was: __ZThn36_N3RBX8NullToolD1Ev
// IDA 0x2ef124: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ef124() {
}

// 0x2ef12c — __ZN3RBX11NewNullToolC1EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::NewNullTool::NewNullTool(RBX::Workspace *)")]
// was: __ZN3RBX11NewNullToolC1EPNS_9WorkspaceE
// IDA 0x2ef12c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2ef12c() {
}

// 0x2ef130 — __ZN3RBX11NewNullToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::NewNullTool::NewNullTool(RBX::Workspace *)")]
// was: __ZN3RBX11NewNullToolC2EPNS_9WorkspaceE
// IDA 0x2ef130: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ef130() {
}

// 0x2ef22c — __ZN3RBX11NewNullToolD0Ev
// type: void __fastcall(RBX::NewNullTool *__hidden this)
#[doc(alias = "RBX::NewNullTool::~NewNullTool()")]
// was: __ZN3RBX11NewNullToolD0Ev
// IDA 0x2ef22c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ef22c() {
}

// 0x2ef2f4 — __ZN3RBX11NewNullToolD1Ev
// type: void __fastcall(RBX::NewNullTool *__hidden this)
#[doc(alias = "RBX::NewNullTool::~NewNullTool()")]
// was: __ZN3RBX11NewNullToolD1Ev
// IDA 0x2ef2f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ef2f4() {
}

// 0x2ef328 — __ZThn36_N3RBX11NewNullToolD0Ev
// type: void __fastcall(RBX::NewNullTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NewNullTool::~NewNullTool()")]
// was: __ZThn36_N3RBX11NewNullToolD0Ev
// IDA 0x2ef328: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ef328() {
}

// 0x2ef330 — __ZThn36_N3RBX11NewNullToolD1Ev
// type: void __fastcall(RBX::NewNullTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NewNullTool::~NewNullTool()")]
// was: __ZThn36_N3RBX11NewNullToolD1Ev
// IDA 0x2ef330: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ef330() {
}

// 0x2ef48c — __ZN3RBX11NewNullTool11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::NewNullTool::onMouseIdle(RBX::UIEvent const&)")]
// was: __ZN3RBX11NewNullTool11onMouseIdleERKNS_7UIEventE
// IDA 0x2ef48c: 179 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ef48c() {
}

// 0x2ef694 — __ZN3RBX11NewNullTool24updateClickDetectorHoverERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::NewNullTool::updateClickDetectorHover(RBX::UIEvent const&)")]
// was: __ZN3RBX11NewNullTool24updateClickDetectorHoverERKNS_7UIEventE
// IDA 0x2ef694: 190 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ef694() {
}

// 0x2ef888 — __ZN3RBX11NewNullTool12onMouseHoverERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::NewNullTool::onMouseHover(RBX::UIEvent const&)")]
// was: __ZN3RBX11NewNullTool12onMouseHoverERKNS_7UIEventE
// IDA 0x2ef888: 227 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ef888() {
}

// 0x2efb14 — __ZN3RBX11NewNullTool16onRightMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::NewNullTool::onRightMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX11NewNullTool16onRightMouseDownERKNS_7UIEventE
// IDA 0x2efb14: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2efb14() {
}

// 0x2efc0c — __ZN3RBX11NewNullTool11onMouseDownERKNS_7UIEventE
// type: void __fastcall(RBX::NewNullTool *this, const RBX::UIEvent *, const RBX::UIEvent *)
#[doc(alias = "RBX::NewNullTool::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX11NewNullTool11onMouseDownERKNS_7UIEventE
// IDA 0x2efc0c: 113 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2efc0c() {
}

// 0x2efd44 — __ZN3RBX11NewNullTool14onRightMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::NewNullTool::onRightMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX11NewNullTool14onRightMouseUpERKNS_7UIEventE
// IDA 0x2efd44: 160 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2efd44() {
}

// 0x2efef8 — __ZN3RBX11shared_fromINS_11NewNullToolEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::NewNullTool> RBX::shared_from<RBX::NewNullTool>(RBX::NewNullTool*)")]
// was: __ZN3RBX11shared_fromINS_11NewNullToolEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x2efef8: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2efef8() {
}

// 0x2f0060 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sNullToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sNullToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sNullToolEEE7getNameEv
// IDA 0x2f0060: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0060() {
}

// 0x2f0088 — __ZN3RBX8NullTool9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::NullTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::NullTool::onMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX8NullTool9onMouseUpERKNS_7UIEventE
// IDA 0x2f0088: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0088() {
}

// 0x2f015c — __ZNK3RBX8NullTool8isStickyEv
// type: _DWORD __fastcall(RBX::NullTool *__hidden this)
#[doc(alias = "RBX::NullTool::isSticky(void)const")]
// was: __ZNK3RBX8NullTool8isStickyEv
// IDA 0x2f015c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f015c() {
}

// 0x2f0224 — __ZNK3RBX8NullTool13getCursorNameEv
// type: _DWORD __fastcall(RBX::NullTool *__hidden this)
#[doc(alias = "RBX::NullTool::getCursorName(void)const")]
// was: __ZNK3RBX8NullTool13getCursorNameEv
// IDA 0x2f0224: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0224() {
}

// 0x2f0240 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_12sNewNullToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_12sNewNullToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_12MouseCommandELZNS_12sNewNullToolEEE7getNameEv
// IDA 0x2f0240: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0240() {
}

// 0x2f0268 — __ZN3RBX11NewNullTool9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::NewNullTool::onMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX11NewNullTool9onMouseUpERKNS_7UIEventE
// IDA 0x2f0268: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0268() {
}

// 0x2f033c — __ZNK3RBX11NewNullTool8isStickyEv
// type: void __fastcall(RBX::NewNullTool *this, int)
#[doc(alias = "RBX::NewNullTool::isSticky(void)const")]
// was: __ZNK3RBX11NewNullTool8isStickyEv
// IDA 0x2f033c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f033c() {
}

// 0x2f0404 — __ZNK3RBX11NewNullTool13getCursorNameEv
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this)
#[doc(alias = "RBX::NewNullTool::getCursorName(void)const")]
// was: __ZNK3RBX11NewNullTool13getCursorNameEv
// IDA 0x2f0404: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0404() {
}

// 0x2f0418 — __ZN3RBX4Name13callDoDeclareILZNS_12sNewNullToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sNewNullToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_12sNewNullToolEEEEvv
// IDA 0x2f0418: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2f0418() {
}

// 0x2f041c — __ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v
// IDA 0x2f041c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f041c() {
}

// 0x2f04fc — __ZN3RBX11shared_fromINS_8NullToolEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::NullTool> RBX::shared_from<RBX::NullTool>(RBX::NullTool*)")]
// was: __ZN3RBX11shared_fromINS_8NullToolEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x2f04fc: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f04fc() {
}

// 0x2f0664 — __GLOBAL__I_a_99
#[doc(alias = "global constructor keyed to_a_99")]
// was: __GLOBAL__I_a_99
// IDA 0x2f0664: 239 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0664() {
}

// 0x2f0bb8 — __ZN3RBX12PartDragTool11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::PartDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::PartDragTool::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX12PartDragTool11onMouseDownERKNS_7UIEventE
// IDA 0x2f0bb8: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0bb8() {
}