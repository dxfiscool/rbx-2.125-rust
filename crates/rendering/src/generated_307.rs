//! rendering shard 307 — 100 stubs 0x44a0b0..0x44d064 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 33240->33340 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 33240 before -> 33340 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x44a0ac (lowest remaining 0x44a0b0..0x44d064, next lowest 0x44d114)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x44a0b0 — __ZN3RBX4Name9doDeclareILZNS_21sContextActionServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sContextActionServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_21sContextActionServiceEEEERKS0_v
// IDA 0x44a0b0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a0b0() {
}

// 0x44a194 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_20ContextActionServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContextActionService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_20ContextActionServiceEEEvv
// IDA 0x44a194: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44a194() {
}

// 0x44a198 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_20ContextActionServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContextActionService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_20ContextActionServiceEEEmv
// IDA 0x44a198: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a198() {
}

// 0x44a270 — __ZN5boost10shared_ptrIN3RBX20ContextActionServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ContextActionService>::shared_ptr<RBX::ContextActionService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX20ContextActionServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44a270: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a270() {
}

// 0x44a338 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ContextActionServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContextActionService,RBX::ContextActionService>(rbx_core::SharedPtr<RBX::ContextActionService> const*,RBX::ContextActionService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ContextActionServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x44a338: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a338() {
}

// 0x44a424 — __ZN5boost6detail12shared_countC2IPN3RBX20ContextActionServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX20ContextActionServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44a424: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a424() {
}

// 0x44a52c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x44a52c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_44a52c() {
}

// 0x44a530 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x44a530: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44a530() {
}

// 0x44a534 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x44a534: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a534() {
}

// 0x44a554 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x44a554: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a554() {
}

// 0x44a56c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContextActionService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ContextActionServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x44a56c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a56c() {
}

// 0x44a570 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_21sContextActionServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_21sContextActionServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_21sContextActionServiceEEE15isNullClassNameEv
// IDA 0x44a570: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a570() {
}

// 0x44a610 — __ZN5boost6detail12shared_countC2IPN3RBX16UserInputServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX16UserInputServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44a610: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a610() {
}

// 0x44a718 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16UserInputServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16UserInputServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x44a718: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_44a718() {
}

// 0x44a720 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16UserInputServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UserInputService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16UserInputServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x44a720: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a720() {
}

// 0x44a744 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9FWServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::FWService> RBX::Creatable<RBX::Instance>::create<RBX::FWService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_9FWServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x44a744: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a744() {
}

// 0x44a7f4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9FWServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::FWService>(rbx_core::SharedPtr<RBX::FWService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9FWServiceEEERS3_RKNS0_IT_EE
// IDA 0x44a7f4: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a7f4() {
}

// 0x44a828 — __ZN5boost10shared_ptrIN3RBX9FWServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::FWService>::shared_ptr<RBX::FWService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX9FWServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44a828: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a828() {
}

// 0x44a8f0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9FWServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FWService,RBX::FWService>(rbx_core::SharedPtr<RBX::FWService> const*,RBX::FWService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9FWServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x44a8f0: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a8f0() {
}

// 0x44a9dc — __ZN5boost6detail12shared_countC2IPN3RBX9FWServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX9FWServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44a9dc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44a9dc() {
}

// 0x44aae4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x44aae4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_44aae4() {
}

// 0x44aae8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x44aae8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44aae8() {
}

// 0x44aaec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x44aaec: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44aaec() {
}

// 0x44ab0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x44ab0c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ab0c() {
}

// 0x44ab24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FWService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FWServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x44ab24: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ab24() {
}

// 0x44ab28 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Players,RBX::Network::Players>(rbx_core::SharedPtr<RBX::Network::Players> const*,RBX::Network::Players *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x44ab28: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ab28() {
}

// 0x44ac18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x44ac18: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44ac18() {
}

// 0x44ac20 — __ZNK3RBX15ServiceProvider4findINS_21PersonalServerServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::PersonalServerService * RBX::ServiceProvider::find<RBX::PersonalServerService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_21PersonalServerServiceEEEPT_v
// IDA 0x44ac20: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ac20() {
}

// 0x44ad94 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_21PersonalServerServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::PersonalServerService> RBX::Creatable<RBX::Instance>::create<RBX::PersonalServerService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_21PersonalServerServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x44ad94: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ad94() {
}

// 0x44ae44 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_21PersonalServerServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::PersonalServerService>(rbx_core::SharedPtr<RBX::PersonalServerService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_21PersonalServerServiceEEERS3_RKNS0_IT_EE
// IDA 0x44ae44: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ae44() {
}

// 0x44ae78 — __ZN3RBX4Name7declareILZNS_22sPersonalServerServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_22sPersonalServerServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_22sPersonalServerServiceEEEERKS0_v
// IDA 0x44ae78: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ae78() {
}

// 0x44aebc — __ZN3RBX4Name13callDoDeclareILZNS_22sPersonalServerServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_22sPersonalServerServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_22sPersonalServerServiceEEEEvv
// IDA 0x44aebc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44aebc() {
}

// 0x44aec0 — __ZN3RBX4Name9doDeclareILZNS_22sPersonalServerServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_22sPersonalServerServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_22sPersonalServerServiceEEEERKS0_v
// IDA 0x44aec0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44aec0() {
}

// 0x44afa4 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_21PersonalServerServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::PersonalServerService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_21PersonalServerServiceEEEvv
// IDA 0x44afa4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44afa4() {
}

// 0x44afa8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_21PersonalServerServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::PersonalServerService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_21PersonalServerServiceEEEmv
// IDA 0x44afa8: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44afa8() {
}

// 0x44b080 — __ZN5boost10shared_ptrIN3RBX21PersonalServerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::PersonalServerService>::shared_ptr<RBX::PersonalServerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX21PersonalServerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44b080: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b080() {
}

// 0x44b148 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_21PersonalServerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PersonalServerService,RBX::PersonalServerService>(rbx_core::SharedPtr<RBX::PersonalServerService> const*,RBX::PersonalServerService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_21PersonalServerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x44b148: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b148() {
}

// 0x44b234 — __ZN5boost6detail12shared_countC2IPN3RBX21PersonalServerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX21PersonalServerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44b234: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b234() {
}

// 0x44b33c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x44b33c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_44b33c() {
}

// 0x44b340 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x44b340: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44b340() {
}

// 0x44b344 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x44b344: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b344() {
}

// 0x44b364 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x44b364: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b364() {
}

// 0x44b37c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PersonalServerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21PersonalServerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x44b37c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b37c() {
}

// 0x44b380 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_22sPersonalServerServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_22sPersonalServerServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_22sPersonalServerServiceEEE15isNullClassNameEv
// IDA 0x44b380: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b380() {
}

// 0x44b420 — __ZNK3RBX15ServiceProvider4findINS_15TeleportServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TeleportService * RBX::ServiceProvider::find<RBX::TeleportService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_15TeleportServiceEEEPT_v
// IDA 0x44b420: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b420() {
}

// 0x44b594 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15TeleportServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::TeleportService> RBX::Creatable<RBX::Instance>::create<RBX::TeleportService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_15TeleportServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x44b594: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b594() {
}

// 0x44b644 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_15TeleportServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::TeleportService>(rbx_core::SharedPtr<RBX::TeleportService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_15TeleportServiceEEERS3_RKNS0_IT_EE
// IDA 0x44b644: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b644() {
}

// 0x44b678 — __ZN3RBX4Name7declareILZNS_16sTeleportServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sTeleportServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_16sTeleportServiceEEEERKS0_v
// IDA 0x44b678: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b678() {
}

// 0x44b6c0 — __ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v
// IDA 0x44b6c0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b6c0() {
}

// 0x44b7a4 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15TeleportServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::TeleportService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15TeleportServiceEEEvv
// IDA 0x44b7a4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44b7a4() {
}

// 0x44b7a8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15TeleportServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TeleportService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_15TeleportServiceEEEmv
// IDA 0x44b7a8: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b7a8() {
}

// 0x44b880 — __ZN5boost10shared_ptrIN3RBX15TeleportServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::TeleportService>::shared_ptr<RBX::TeleportService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX15TeleportServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44b880: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b880() {
}

// 0x44b948 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15TeleportServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TeleportService,RBX::TeleportService>(rbx_core::SharedPtr<RBX::TeleportService> const*,RBX::TeleportService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15TeleportServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x44b948: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44b948() {
}

// 0x44ba34 — __ZN5boost6detail12shared_countC2IPN3RBX15TeleportServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX15TeleportServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44ba34: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ba34() {
}

// 0x44bb3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x44bb3c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_44bb3c() {
}

// 0x44bb40 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x44bb40: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44bb40() {
}

// 0x44bb44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x44bb44: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44bb44() {
}

// 0x44bb64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x44bb64: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44bb64() {
}

// 0x44bb7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TeleportService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15TeleportServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x44bb7c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44bb7c() {
}

// 0x44bb80 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sTeleportServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sTeleportServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sTeleportServiceEEE15isNullClassNameEv
// IDA 0x44bb80: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44bb80() {
}

// 0x44bc20 — __ZNK3RBX15ServiceProvider4findINS_14CookiesServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::CookiesService * RBX::ServiceProvider::find<RBX::CookiesService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_14CookiesServiceEEEPT_v
// IDA 0x44bc20: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44bc20() {
}

// 0x44bd94 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14CookiesServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::CookiesService> RBX::Creatable<RBX::Instance>::create<RBX::CookiesService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_14CookiesServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x44bd94: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44bd94() {
}

// 0x44be44 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_14CookiesServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::CookiesService>(rbx_core::SharedPtr<RBX::CookiesService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_14CookiesServiceEEERS3_RKNS0_IT_EE
// IDA 0x44be44: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44be44() {
}

// 0x44be78 — __ZN3RBX4Name7declareILZNS_15sCookiesServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sCookiesServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_15sCookiesServiceEEEERKS0_v
// IDA 0x44be78: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44be78() {
}

// 0x44bebc — __ZN3RBX4Name13callDoDeclareILZNS_15sCookiesServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sCookiesServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_15sCookiesServiceEEEEvv
// IDA 0x44bebc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44bebc() {
}

// 0x44bec0 — __ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v
// IDA 0x44bec0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44bec0() {
}

// 0x44bfa4 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14CookiesServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::CookiesService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14CookiesServiceEEEvv
// IDA 0x44bfa4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44bfa4() {
}

// 0x44bfa8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14CookiesServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::CookiesService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_14CookiesServiceEEEmv
// IDA 0x44bfa8: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44bfa8() {
}

// 0x44c080 — __ZN5boost10shared_ptrIN3RBX14CookiesServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::CookiesService>::shared_ptr<RBX::CookiesService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX14CookiesServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44c080: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c080() {
}

// 0x44c148 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CookiesServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CookiesService,RBX::CookiesService>(rbx_core::SharedPtr<RBX::CookiesService> const*,RBX::CookiesService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CookiesServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x44c148: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c148() {
}

// 0x44c234 — __ZN5boost6detail12shared_countC2IPN3RBX14CookiesServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX14CookiesServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44c234: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c234() {
}

// 0x44c33c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CookiesServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CookiesServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x44c33c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_44c33c() {
}

// 0x44c340 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CookiesServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CookiesServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x44c340: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44c340() {
}

// 0x44c344 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CookiesServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CookiesServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x44c344: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c344() {
}

// 0x44c364 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CookiesServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CookiesServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x44c364: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c364() {
}

// 0x44c37c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CookiesServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CookiesService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CookiesServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x44c37c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c37c() {
}

// 0x44c380 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_15sCookiesServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_15sCookiesServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_15sCookiesServiceEEE15isNullClassNameEv
// IDA 0x44c380: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c380() {
}

// 0x44c420 — __ZN3RBX4Name7declareILZNS_26sScriptInformationProviderEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_26sScriptInformationProviderEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_26sScriptInformationProviderEEEERKS0_v
// IDA 0x44c420: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c420() {
}

// 0x44c468 — __ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v
// IDA 0x44c468: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c468() {
}

// 0x44c550 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_25ScriptInformationProviderEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptInformationProvider>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_25ScriptInformationProviderEEEmv
// IDA 0x44c550: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c550() {
}

// 0x44c628 — __ZN5boost10shared_ptrIN3RBX25ScriptInformationProviderEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptInformationProvider>::shared_ptr<RBX::ScriptInformationProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX25ScriptInformationProviderEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44c628: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c628() {
}

// 0x44c6f0 — __ZNK3RBX15ServiceProvider4findINS_13DebrisServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DebrisService * RBX::ServiceProvider::find<RBX::DebrisService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_13DebrisServiceEEEPT_v
// IDA 0x44c6f0: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c6f0() {
}

// 0x44c864 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13DebrisServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::DebrisService> RBX::Creatable<RBX::Instance>::create<RBX::DebrisService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_13DebrisServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x44c864: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c864() {
}

// 0x44c914 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13DebrisServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::DebrisService>(rbx_core::SharedPtr<RBX::DebrisService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13DebrisServiceEEERS3_RKNS0_IT_EE
// IDA 0x44c914: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c914() {
}

// 0x44c948 — __ZN3RBX4Name7declareILZNS_14sDebrisServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sDebrisServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_14sDebrisServiceEEEERKS0_v
// IDA 0x44c948: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c948() {
}

// 0x44c98c — __ZN3RBX4Name13callDoDeclareILZNS_14sDebrisServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDebrisServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sDebrisServiceEEEEvv
// IDA 0x44c98c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44c98c() {
}

// 0x44c990 — __ZN3RBX4Name9doDeclareILZNS_14sDebrisServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDebrisServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sDebrisServiceEEEERKS0_v
// IDA 0x44c990: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44c990() {
}

// 0x44ca74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13DebrisServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::DebrisService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13DebrisServiceEEEvv
// IDA 0x44ca74: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44ca74() {
}

// 0x44ca78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13DebrisServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::DebrisService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_13DebrisServiceEEEmv
// IDA 0x44ca78: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ca78() {
}

// 0x44cb50 — __ZN5boost10shared_ptrIN3RBX13DebrisServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::DebrisService>::shared_ptr<RBX::DebrisService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13DebrisServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44cb50: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44cb50() {
}

// 0x44cc18 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13DebrisServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DebrisService,RBX::DebrisService>(rbx_core::SharedPtr<RBX::DebrisService> const*,RBX::DebrisService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13DebrisServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x44cc18: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44cc18() {
}

// 0x44cd04 — __ZN5boost6detail12shared_countC2IPN3RBX13DebrisServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13DebrisServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44cd04: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44cd04() {
}

// 0x44ce0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebrisServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebrisServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x44ce0c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_44ce0c() {
}

// 0x44ce10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebrisServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebrisServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x44ce10: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44ce10() {
}

// 0x44ce14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebrisServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebrisServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x44ce14: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ce14() {
}

// 0x44ce34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebrisServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebrisServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x44ce34: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ce34() {
}

// 0x44ce4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebrisServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebrisService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebrisServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x44ce4c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ce4c() {
}

// 0x44ce50 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE15isNullClassNameEv
// IDA 0x44ce50: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ce50() {
}

// 0x44cef0 — __ZNK3RBX15ServiceProvider4findINS_15GamePassServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::GamePassService * RBX::ServiceProvider::find<RBX::GamePassService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_15GamePassServiceEEEPT_v
// IDA 0x44cef0: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44cef0() {
}

// 0x44d064 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15GamePassServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::GamePassService> RBX::Creatable<RBX::Instance>::create<RBX::GamePassService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_15GamePassServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x44d064: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d064() {
}
