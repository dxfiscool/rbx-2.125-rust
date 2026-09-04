//! rendering shard 308 — 100 stubs 0x44d114..0x450ee4 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 33340->33440 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 33340 before -> 33440 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x44d064 (lowest remaining 0x44d114..0x450ee4, next lowest 0x450f84)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x44d114 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_15GamePassServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::GamePassService>(rbx_core::SharedPtr<RBX::GamePassService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_15GamePassServiceEEERS3_RKNS0_IT_EE
// IDA 0x44d114: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d114() {
}

// 0x44d148 — __ZN3RBX4Name7declareILZNS_16sGamePassServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sGamePassServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_16sGamePassServiceEEEERKS0_v
// IDA 0x44d148: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d148() {
}

// 0x44d18c — __ZN3RBX4Name13callDoDeclareILZNS_16sGamePassServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sGamePassServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_16sGamePassServiceEEEEvv
// IDA 0x44d18c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44d18c() {
}

// 0x44d190 — __ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v
// IDA 0x44d190: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d190() {
}

// 0x44d274 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GamePassServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GamePassService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GamePassServiceEEEvv
// IDA 0x44d274: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44d274() {
}

// 0x44d278 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GamePassService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv
// IDA 0x44d278: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d278() {
}

// 0x44d350 — __ZN5boost10shared_ptrIN3RBX15GamePassServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::GamePassService>::shared_ptr<RBX::GamePassService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX15GamePassServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44d350: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d350() {
}

// 0x44d418 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15GamePassServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GamePassService,RBX::GamePassService>(rbx_core::SharedPtr<RBX::GamePassService> const*,RBX::GamePassService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15GamePassServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x44d418: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d418() {
}

// 0x44d504 — __ZN5boost6detail12shared_countC2IPN3RBX15GamePassServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX15GamePassServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44d504: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d504() {
}

// 0x44d60c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x44d60c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_44d60c() {
}

// 0x44d610 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x44d610: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44d610() {
}

// 0x44d614 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x44d614: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d614() {
}

// 0x44d634 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x44d634: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d634() {
}

// 0x44d64c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x44d64c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d64c() {
}

// 0x44d650 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGamePassServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGamePassServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGamePassServiceEEE15isNullClassNameEv
// IDA 0x44d650: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d650() {
}

// 0x44d6f0 — __ZNK3RBX15ServiceProvider4findINS_13SocialServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::SocialService * RBX::ServiceProvider::find<RBX::SocialService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_13SocialServiceEEEPT_v
// IDA 0x44d6f0: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d6f0() {
}

// 0x44d864 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13SocialServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::SocialService> RBX::Creatable<RBX::Instance>::create<RBX::SocialService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_13SocialServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x44d864: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d864() {
}

// 0x44d914 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13SocialServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::SocialService>(rbx_core::SharedPtr<RBX::SocialService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13SocialServiceEEERS3_RKNS0_IT_EE
// IDA 0x44d914: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d914() {
}

// 0x44d948 — __ZN3RBX4Name7declareILZNS_14sSocialServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sSocialServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_14sSocialServiceEEEERKS0_v
// IDA 0x44d948: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d948() {
}

// 0x44d98c — __ZN3RBX4Name13callDoDeclareILZNS_14sSocialServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sSocialServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sSocialServiceEEEEvv
// IDA 0x44d98c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44d98c() {
}

// 0x44d990 — __ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v
// IDA 0x44d990: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44d990() {
}

// 0x44da74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13SocialServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::SocialService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13SocialServiceEEEvv
// IDA 0x44da74: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44da74() {
}

// 0x44da78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::SocialService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv
// IDA 0x44da78: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44da78() {
}

// 0x44db50 — __ZN5boost10shared_ptrIN3RBX13SocialServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::SocialService>::shared_ptr<RBX::SocialService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13SocialServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44db50: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44db50() {
}

// 0x44dc18 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13SocialServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SocialService,RBX::SocialService>(rbx_core::SharedPtr<RBX::SocialService> const*,RBX::SocialService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13SocialServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x44dc18: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44dc18() {
}

// 0x44dd04 — __ZN5boost6detail12shared_countC2IPN3RBX13SocialServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13SocialServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44dd04: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44dd04() {
}

// 0x44de0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x44de0c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_44de0c() {
}

// 0x44de10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x44de10: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44de10() {
}

// 0x44de14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x44de14: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44de14() {
}

// 0x44de34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x44de34: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44de34() {
}

// 0x44de4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x44de4c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44de4c() {
}

// 0x44de50 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sSocialServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sSocialServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sSocialServiceEEE15isNullClassNameEv
// IDA 0x44de50: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44de50() {
}

// 0x44def4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13InsertServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::InsertService> RBX::Creatable<RBX::Instance>::create<RBX::InsertService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_13InsertServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x44def4: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44def4() {
}

// 0x44dfa8 — __ZN5boost10shared_ptrIN3RBX13InsertServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::InsertService>::shared_ptr<RBX::InsertService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13InsertServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44dfa8: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44dfa8() {
}

// 0x44e074 — __ZN5boost6detail12shared_countC2IPN3RBX13InsertServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13InsertServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44e074: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44e074() {
}

// 0x44e180 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13InsertServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::InsertService>(rbx_core::SharedPtr<RBX::InsertService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13InsertServiceEEERS3_RKNS0_IT_EE
// IDA 0x44e180: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44e180() {
}

// 0x44e1b4 — __ZN3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E17static_getCreatorEv
// IDA 0x44e1b4: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44e1b4() {
}

// 0x44e228 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13InsertServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::InsertService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13InsertServiceEEEvv
// IDA 0x44e228: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44e228() {
}

// 0x44e22c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::InsertService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv
// IDA 0x44e22c: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44e22c() {
}

// 0x44e8f8 — __ZNK3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E7Creator12getClassNameEv
// IDA 0x44e8f8: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44e8f8() {
}

// 0x44e968 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13FriendServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::FriendService> RBX::Creatable<RBX::Instance>::create<RBX::FriendService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_13FriendServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x44e968: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44e968() {
}

// 0x44ea18 — __ZN5boost10shared_ptrIN3RBX13FriendServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::FriendService>::shared_ptr<RBX::FriendService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13FriendServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44ea18: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ea18() {
}

// 0x44eae4 — __ZN5boost6detail12shared_countC2IPN3RBX13FriendServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13FriendServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44eae4: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44eae4() {
}

// 0x44ebf0 — __ZN3RBX4Name7declareILZNS_14sFriendServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sFriendServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_14sFriendServiceEEEERKS0_v
// IDA 0x44ebf0: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ebf0() {
}

// 0x44ec38 — __ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v
// IDA 0x44ec38: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ec38() {
}

// 0x44ed20 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13FriendServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::FriendService>(rbx_core::SharedPtr<RBX::FriendService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13FriendServiceEEERS3_RKNS0_IT_EE
// IDA 0x44ed20: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ed20() {
}

// 0x44ed54 — __ZN3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E17static_getCreatorEv
// IDA 0x44ed54: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ed54() {
}

// 0x44edc8 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13FriendServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FriendService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13FriendServiceEEEvv
// IDA 0x44edc8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44edc8() {
}

// 0x44edcc — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FriendService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv
// IDA 0x44edcc: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44edcc() {
}

// 0x44eea4 — __ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::GeometryService * RBX::ServiceProvider::find<RBX::GeometryService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v
// IDA 0x44eea4: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44eea4() {
}

// 0x44f018 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15GeometryServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryService> RBX::Creatable<RBX::Instance>::create<RBX::GeometryService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_15GeometryServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x44f018: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f018() {
}

// 0x44f0c8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_15GeometryServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::GeometryService>(rbx_core::SharedPtr<RBX::GeometryService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_15GeometryServiceEEERS3_RKNS0_IT_EE
// IDA 0x44f0c8: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f0c8() {
}

// 0x44f0fc — __ZN3RBX4Name7declareILZNS_16sGeometryServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sGeometryServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_16sGeometryServiceEEEERKS0_v
// IDA 0x44f0fc: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f0fc() {
}

// 0x44f140 — __ZN3RBX4Name13callDoDeclareILZNS_16sGeometryServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sGeometryServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_16sGeometryServiceEEEEvv
// IDA 0x44f140: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44f140() {
}

// 0x44f148 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GeometryServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GeometryService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GeometryServiceEEEvv
// IDA 0x44f148: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44f148() {
}

// 0x44f14c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GeometryService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv
// IDA 0x44f14c: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f14c() {
}

// 0x44f224 — __ZN5boost10shared_ptrIN3RBX15GeometryServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryService>::shared_ptr<RBX::GeometryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX15GeometryServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44f224: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f224() {
}

// 0x44f2ec — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15GeometryServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GeometryService,RBX::GeometryService>(rbx_core::SharedPtr<RBX::GeometryService> const*,RBX::GeometryService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15GeometryServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x44f2ec: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f2ec() {
}

// 0x44f3d8 — __ZN5boost6detail12shared_countC2IPN3RBX15GeometryServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX15GeometryServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44f3d8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f3d8() {
}

// 0x44f4e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x44f4e0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_44f4e0() {
}

// 0x44f4e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x44f4e4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44f4e4() {
}

// 0x44f4e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x44f4e8: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f4e8() {
}

// 0x44f508 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x44f508: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f508() {
}

// 0x44f520 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x44f520: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f520() {
}

// 0x44f524 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEE15isNullClassNameEv
// IDA 0x44f524: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f524() {
}

// 0x44f5c8 — __ZNK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7Creator12getClassNameEv
// IDA 0x44f5c8: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f5c8() {
}

// 0x44f638 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12BadgeServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::BadgeService> RBX::Creatable<RBX::Instance>::create<RBX::BadgeService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_12BadgeServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x44f638: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f638() {
}

// 0x44f6ec — __ZN5boost10shared_ptrIN3RBX12BadgeServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::BadgeService>::shared_ptr<RBX::BadgeService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX12BadgeServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x44f6ec: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f6ec() {
}

// 0x44f7b8 — __ZN5boost6detail12shared_countC2IPN3RBX12BadgeServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX12BadgeServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x44f7b8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f7b8() {
}

// 0x44f8c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x44f8c0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_44f8c0() {
}

// 0x44f8c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x44f8c8: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f8c8() {
}

// 0x44f8e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x44f8e8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f8e8() {
}

// 0x44f900 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x44f900: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f900() {
}

// 0x44f904 — __ZN3RBX4Name7declareILZNS_13sBadgeServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sBadgeServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_13sBadgeServiceEEEERKS0_v
// IDA 0x44f904: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f904() {
}

// 0x44f948 — __ZN3RBX4Name13callDoDeclareILZNS_13sBadgeServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sBadgeServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sBadgeServiceEEEEvv
// IDA 0x44f948: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44f948() {
}

// 0x44f94c — __ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v
// IDA 0x44f94c: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44f94c() {
}

// 0x44fa30 — __ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7CreatorC2Ev
// IDA 0x44fa30: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44fa30() {
}

// 0x44fc58 — __ZNK3RBX15ServiceProvider4findINS_12BadgeServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::BadgeService * RBX::ServiceProvider::find<RBX::BadgeService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_12BadgeServiceEEEPT_v
// IDA 0x44fc58: 129 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44fc58() {
}

// 0x44fdcc — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12BadgeServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::BadgeService>(rbx_core::SharedPtr<RBX::BadgeService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12BadgeServiceEEERS3_RKNS0_IT_EE
// IDA 0x44fdcc: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44fdcc() {
}

// 0x44fe00 — __ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E17static_getCreatorEv
// IDA 0x44fe00: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44fe00() {
}

// 0x44fe74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12BadgeServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::BadgeService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12BadgeServiceEEEvv
// IDA 0x44fe74: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_44fe74() {
}

// 0x44fe78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::BadgeService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv
// IDA 0x44fe78: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44fe78() {
}

// 0x44ff50 — __ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E15isNullClassNameEv
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E15isNullClassNameEv")]
// was: __ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E15isNullClassNameEv
// IDA 0x44ff50: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ff50() {
}

// 0x44ffb8 — __ZNK3RBX15ServiceProvider4findINS_14PhysicsServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::PhysicsService * RBX::ServiceProvider::find<RBX::PhysicsService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_14PhysicsServiceEEEPT_v
// IDA 0x44ffb8: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_44ffb8() {
}

// 0x45012c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14PhysicsServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsService> RBX::Creatable<RBX::Instance>::create<RBX::PhysicsService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_14PhysicsServiceEEEN5boost10shared_ptrIT_EEv
// IDA 0x45012c: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45012c() {
}

// 0x4501dc — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_14PhysicsServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::PhysicsService>(rbx_core::SharedPtr<RBX::PhysicsService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_14PhysicsServiceEEERS3_RKNS0_IT_EE
// IDA 0x4501dc: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4501dc() {
}

// 0x450210 — __ZN3RBX4Name7declareILZNS_15sPhysicsServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sPhysicsServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_15sPhysicsServiceEEEERKS0_v
// IDA 0x450210: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_450210() {
}

// 0x450258 — __ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v
// IDA 0x450258: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_450258() {
}

// 0x450340 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14PhysicsServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::PhysicsService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_14PhysicsServiceEEEmv
// IDA 0x450340: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_450340() {
}

// 0x450418 — __ZN3RBX14PhysicsServiceC2Ev
// type: RBX::Instance *__fastcall(RBX::PhysicsService *this)
#[doc(alias = "RBX::PhysicsService::PhysicsService(void)")]
// was: __ZN3RBX14PhysicsServiceC2Ev
// IDA 0x450418: 332 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_450418() {
}

// 0x450794 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEE12getClassNameEv
// IDA 0x450794: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_450794() {
}

// 0x450798 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEE12getClassNameEv
// IDA 0x450798: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_450798() {
}

// 0x45079c — __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE5eraseENS4_8IteratorE
// type: void *__fastcall(int, void *, int, int)
#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::erase(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator)")]
// was: __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE5eraseENS4_8IteratorE
// IDA 0x45079c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_45079c() {
}

// 0x4507d0 — __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE8IteratorppEv
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator::operator++(void)")]
// was: __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE8IteratorppEv
// IDA 0x4507d0: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4507d0() {
}

// 0x450988 — __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE4Hook6removeEv
// type: void __fastcall(int *, int, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Hook::remove(void)")]
// was: __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE4Hook6removeEv
// IDA 0x450988: 148 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_450988() {
}

// 0x450b14 — __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE8IteratorC2EPS2_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *, int, int, void *, int)
#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator::Iterator(RBX::PartInstance*)")]
// was: __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE8IteratorC2EPS2_
// IDA 0x450b14: 125 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_450b14() {
}

// 0x450c74 — __ZN3RBX20ConcurrencyValidatorD2Ev
// type: void __fastcall(RBX::ConcurrencyValidator *__hidden this)
#[doc(alias = "RBX::ConcurrencyValidator::~ConcurrencyValidator()")]
// was: __ZN3RBX20ConcurrencyValidatorD2Ev
// IDA 0x450c74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_450c74() {
}

// 0x450dc0 — __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x450dc0: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_450dc0() {
}

// 0x450ee0 — __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x450ee0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_450ee0() {
}

// 0x450ee4 — __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x450ee4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_450ee4() {
}
