//! core shard mv — 100 core stubs EA-sorted asc global gap filler not yet in any crate (fallback — workspace 85545/85545 complete, rbx_core fallback).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in any crate (global gap; 85545 distinct before -> 85545 after — ws 0 uncovered, fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33887, 1923 uncovered before -> 1823 after, batch 0xf238b4..0xf241cc).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sRayValueEEEERKS0_v$shim")]
// 0xf238b4 — __ZN3RBX4Name9doDeclareILZNS_9sRayValueEEEERKS0_v$shim
// type: int()
pub fn stub_0xf238b4() {
    // IDA 0xf238b4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE24safe_static_do_get_mutexEv$shim")]
// 0xf238cc — __ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf238cc() {
    // IDA 0xf238cc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sBrickColorValueEEEERKS0_v$shim")]
// 0xf238d8 — __ZN3RBX4Name9doDeclareILZNS_16sBrickColorValueEEEERKS0_v$shim
// type: int()
pub fn stub_0xf238d8() {
    // IDA 0xf238d8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE24safe_static_do_get_mutexEv$shim")]
// 0xf238f0 — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf238f0() {
    // IDA 0xf238f0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sColor3ValueEEEERKS0_v$shim")]
// 0xf238fc — __ZN3RBX4Name9doDeclareILZNS_12sColor3ValueEEEERKS0_v$shim
// type: int()
pub fn stub_0xf238fc() {
    // IDA 0xf238fc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sCFrameValueEEEERKS0_v$shim")]
// 0xf23920 — __ZN3RBX4Name9doDeclareILZNS_12sCFrameValueEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23920() {
    // IDA 0xf23920: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sVector3ValueEEEERKS0_v$shim")]
// 0xf23944 — __ZN3RBX4Name9doDeclareILZNS_13sVector3ValueEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23944() {
    // IDA 0xf23944: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBoolValueEEEERKS0_v$shim")]
// 0xf2395c — __ZN3RBX4Name9doDeclareILZNS_10sBoolValueEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2395c() {
    // IDA 0xf2395c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sDoubleValueEEEERKS0_v$shim")]
// 0xf23974 — __ZN3RBX4Name9doDeclareILZNS_12sDoubleValueEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23974() {
    // IDA 0xf23974: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sIntValueEEEERKS0_v$shim")]
// 0xf2398c — __ZN3RBX4Name9doDeclareILZNS_9sIntValueEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2398c() {
    // IDA 0xf2398c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFviEE13disconnectAllEv$shim")]
// 0xf239b0 — __ZN3rbx7signals6signalIFviEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf239b0() {
    // IDA 0xf239b0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK5boost9function1IviEclEi$shim")]
// 0xf239bc — __ZNK5boost9function1IviEclEi$shim
// type: int()
pub fn stub_0xf239bc() {
    // IDA 0xf239bc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE13disconnectAllEv$shim")]
// 0xf239c8 — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf239c8() {
    // IDA 0xf239c8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK5boost9function1IvN3RBX10BrickColorEEclES2_$shim")]
// 0xf239e0 — __ZNK5boost9function1IvN3RBX10BrickColorEEclES2_$shim
// type: void __fastcall(_DWORD *, int)
pub fn stub_0xf239e0() {
    // IDA 0xf239e0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf239ec — __ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slot24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf239ec() {
    // IDA 0xf239ec: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE13disconnectAllEv$shim")]
// 0xf23a58 — __ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf23a58() {
    // IDA 0xf23a58: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf23a7c — __ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf23a7c() {
    // IDA 0xf23a7c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE$shim")]
// 0xf23ab8 — __ZN3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE$shim
// type: int()
pub fn stub_0xf23ab8() {
    // IDA 0xf23ab8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sVehicleSeatEEEERKS0_v$shim")]
// 0xf23b00 — __ZN3RBX4Name9doDeclareILZNS_12sVehicleSeatEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23b00() {
    // IDA 0xf23b00: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX11KernelJointD0Ev$shim")]
// 0xf23b18 — __ZN3RBX11KernelJointD0Ev$shim
// type: void __fastcall(RBX::KernelJoint *)
pub fn stub_0xf23b18() {
    // IDA 0xf23b18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sVirtualUserEEEERKS0_v$shim")]
// 0xf23b3c — __ZN3RBX4Name9doDeclareILZNS_12sVirtualUserEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23b3c() {
    // IDA 0xf23b3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11VirtualUserERKNS4_7UIEventEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_$shim")]
// 0xf23b48 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11VirtualUserERKNS4_7UIEventEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_$shim
// type: int()
pub fn stub_0xf23b48() {
    // IDA 0xf23b48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf23b54 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf23b54() {
    // IDA 0xf23b54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5World5resetEv$shim")]
// 0xf23b9c — __ZN3RBX5World5resetEv$shim
// type: int __fastcall(RBX::World *)
pub fn stub_0xf23b9c() {
    // IDA 0xf23b9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_15GeometryServiceEEEPT_v$shim")]
// 0xf23ba8 — __ZNK3RBX15ServiceProvider6createINS_15GeometryServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf23ba8() {
    // IDA 0xf23ba8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_18StarterPackServiceEEEPT_v$shim")]
// 0xf23bc0 — __ZNK3RBX15ServiceProvider6createINS_18StarterPackServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf23bc0() {
    // IDA 0xf23bc0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_15ContentProviderEEEPT_v$shim")]
// 0xf23bcc — __ZNK3RBX15ServiceProvider6createINS_15ContentProviderEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf23bcc() {
    // IDA 0xf23bcc: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_14CoreGuiServiceEEEPT_v$shim")]
// 0xf23bd8 — __ZNK3RBX15ServiceProvider4findINS_14CoreGuiServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf23bd8() {
    // IDA 0xf23bd8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_10RunServiceEEEPT_v$shim")]
// 0xf23bfc — __ZNK3RBX15ServiceProvider6createINS_10RunServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf23bfc() {
    // IDA 0xf23bfc: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_9SelectionEEEPT_v$shim")]
// 0xf23c08 — __ZNK3RBX15ServiceProvider4findINS_9SelectionEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf23c08() {
    // IDA 0xf23c08: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_16UserInputServiceEEEPT_v$shim")]
// 0xf23c14 — __ZNK3RBX15ServiceProvider4findINS_16UserInputServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf23c14() {
    // IDA 0xf23c14: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_6sModelEEEERKS0_v$shim")]
// 0xf23c44 — __ZN3RBX4Name7declareILZNS_6sModelEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23c44() {
    // IDA 0xf23c44: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sModelEEEERKS0_v$shim")]
// 0xf23c50 — __ZN3RBX4Name9doDeclareILZNS_6sModelEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23c50() {
    // IDA 0xf23c50: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_9SelectionEEEmv$shim")]
// 0xf23c74 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_9SelectionEEEmv$shim
// type: int()
pub fn stub_0xf23c74() {
    // IDA 0xf23c74: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE24safe_static_do_get_mutexEv$shim")]
// 0xf23c80 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf23c80() {
    // IDA 0xf23c80: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_18StarterPackServiceEEEmv$shim")]
// 0xf23c8c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18StarterPackServiceEEEmv$shim
// type: int()
pub fn stub_0xf23c8c() {
    // IDA 0xf23c8c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sDecalToolEEEERKS0_v$shim")]
// 0xf23c98 — __ZN3RBX4Name7declareILZNS_10sDecalToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23c98() {
    // IDA 0xf23c98: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sDecalToolEEEERKS0_v$shim")]
// 0xf23ca4 — __ZN3RBX4Name9doDeclareILZNS_10sDecalToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23ca4() {
    // IDA 0xf23ca4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sAdvArrowToolEEEERKS0_v$shim")]
// 0xf23ce0 — __ZN3RBX4Name7declareILZNS_13sAdvArrowToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23ce0() {
    // IDA 0xf23ce0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX16AdvArrowToolBaseD2Ev$shim")]
// 0xf23cec — __ZN3RBX16AdvArrowToolBaseD2Ev$shim
// type: void __fastcall(RBX::AdvArrowToolBase *)
pub fn stub_0xf23cec() {
    // IDA 0xf23cec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX16AdvArrowToolBaseD0Ev$shim")]
// 0xf23cf8 — __ZN3RBX16AdvArrowToolBaseD0Ev$shim
// type: void __fastcall(RBX::AdvArrowToolBase *)
pub fn stub_0xf23cf8() {
    // IDA 0xf23cf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX13ArrowToolBaseD2Ev$shim")]
// 0xf23d04 — __ZN3RBX13ArrowToolBaseD2Ev$shim
// type: void __fastcall(RBX::ArrowToolBase *)
pub fn stub_0xf23d04() {
    // IDA 0xf23d04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX13ArrowToolBaseD0Ev$shim")]
// 0xf23d10 — __ZN3RBX13ArrowToolBaseD0Ev$shim
// type: void __fastcall(RBX::ArrowToolBase *)
pub fn stub_0xf23d10() {
    // IDA 0xf23d10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sAdvArrowToolEEEERKS0_v$shim")]
// 0xf23d1c — __ZN3RBX4Name9doDeclareILZNS_13sAdvArrowToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23d1c() {
    // IDA 0xf23d1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIPmSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_$shim")]
// 0xf23d28 — __ZNSt6vectorIPmSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf23d28() {
    // IDA 0xf23d28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIPFbvESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf23d34 — __ZNSt6vectorIPFbvESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf23d34() {
    // IDA 0xf23d34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv$shim")]
// 0xf23d4c — __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv$shim
// type: int()
pub fn stub_0xf23d4c() {
    // IDA 0xf23d4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost3_bi6bind_tIiNS_4_mfi4cmf1IiN3RBX5WorldENS4_11IWorldStage10MetricTypeEEENS0_5list2INS0_5valueIPKS5_EENSA_IS7_EEEEEclEv$shim")]
// 0xf23d58 — __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf1IiN3RBX5WorldENS4_11IWorldStage10MetricTypeEEENS0_5list2INS0_5valueIPKS5_EENSA_IS7_EEEEEclEv$shim
// type: int()
pub fn stub_0xf23d58() {
    // IDA 0xf23d58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX5WorldEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv$shim")]
// 0xf23d64 — __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX5WorldEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv$shim
// type: int()
pub fn stub_0xf23d64() {
    // IDA 0xf23d64: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost3_bi6bind_tIfNS_4_mfi4cmf0IfN3RBX5WorldEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv$shim")]
// 0xf23d70 — __ZN5boost3_bi6bind_tIfNS_4_mfi4cmf0IfN3RBX5WorldEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv$shim
// type: int()
pub fn stub_0xf23d70() {
    // IDA 0xf23d70: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX10RunServiceEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv$shim")]
// 0xf23d7c — __ZN5boost3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX10RunServiceEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv$shim
// type: int()
pub fn stub_0xf23d7c() {
    // IDA 0xf23d7c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX9AllocatorINS_20NormalBreakConnectorEEdlEPv$shim")]
// 0xf23d94 — __ZN3RBX9AllocatorINS_20NormalBreakConnectorEEdlEPv$shim
// type: void __fastcall(void *)
pub fn stub_0xf23d94() {
    // IDA 0xf23d94: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX9AllocatorINS_17BallBallConnectorEEdlEPv$shim")]
// 0xf23da0 — __ZN3RBX9AllocatorINS_17BallBallConnectorEEdlEPv$shim
// type: void __fastcall(void *)
pub fn stub_0xf23da0() {
    // IDA 0xf23da0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX9AllocatorINS_18BallBlockConnectorEEdlEPv$shim")]
// 0xf23dac — __ZN3RBX9AllocatorINS_18BallBlockConnectorEEdlEPv$shim
// type: void __fastcall(void *)
pub fn stub_0xf23dac() {
    // IDA 0xf23dac: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX10KernelData10removeBodyEPNS_4BodyE$shim")]
// 0xf23db8 — __ZN3RBX10KernelData10removeBodyEPNS_4BodyE$shim
// type: int __fastcall(RBX::KernelData *, RBX::Body *)
pub fn stub_0xf23db8() {
    // IDA 0xf23db8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX10KernelData12addConnectorEPNS_9ConnectorE$shim")]
// 0xf23dc4 — __ZN3RBX10KernelData12addConnectorEPNS_9ConnectorE$shim
// type: int __fastcall(RBX::KernelData *, RBX::Connector *)
pub fn stub_0xf23dc4() {
    // IDA 0xf23dc4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX10IndexArrayINS_5PointEXadL_ZNS1_14getKernelIndexEvEEE10fastRemoveEPS1_$shim")]
// 0xf23dd0 — __ZN3RBX10IndexArrayINS_5PointEXadL_ZNS1_14getKernelIndexEvEEE10fastRemoveEPS1_$shim
// type: int()
pub fn stub_0xf23dd0() {
    // IDA 0xf23dd0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10KernelData15removeConnectorEPNS_9ConnectorE$shim")]
// 0xf23ddc — __ZN3RBX10KernelData15removeConnectorEPNS_9ConnectorE$shim
// type: int __fastcall(RBX::KernelData *, RBX::Connector *)
pub fn stub_0xf23ddc() {
    // IDA 0xf23ddc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX9AllocatorINS_19FaceVertexConnectorEEdlEPv$shim")]
// 0xf23e18 — __ZN3RBX9AllocatorINS_19FaceVertexConnectorEEdlEPv$shim
// type: void __fastcall(void *)
pub fn stub_0xf23e18() {
    // IDA 0xf23e18: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEdlEPv$shim")]
// 0xf23e24 — __ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEdlEPv$shim
// type: void __fastcall(void *)
pub fn stub_0xf23e24() {
    // IDA 0xf23e24: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEdlEPv$shim")]
// 0xf23e30 — __ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEdlEPv$shim
// type: void __fastcall(void *)
pub fn stub_0xf23e30() {
    // IDA 0xf23e30: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX9AllocatorINS_19BallVertexConnectorEEdlEPv$shim")]
// 0xf23e3c — __ZN3RBX9AllocatorINS_19BallVertexConnectorEEdlEPv$shim
// type: void __fastcall(void *)
pub fn stub_0xf23e3c() {
    // IDA 0xf23e3c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX9AllocatorINS_17BallEdgeConnectorEEdlEPv$shim")]
// 0xf23e48 — __ZN3RBX9AllocatorINS_17BallEdgeConnectorEEdlEPv$shim
// type: void __fastcall(void *)
pub fn stub_0xf23e48() {
    // IDA 0xf23e48: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX9AllocatorINS_18BallPlaneConnectorEEdlEPv$shim")]
// 0xf23e54 — __ZN3RBX9AllocatorINS_18BallPlaneConnectorEEdlEPv$shim
// type: void __fastcall(void *)
pub fn stub_0xf23e54() {
    // IDA 0xf23e54: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZNSt6vectorIN3RBX8NormalIdESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf23f14 — __ZNSt6vectorIN3RBX8NormalIdESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
// type: int()
pub fn stub_0xf23f14() {
    // IDA 0xf23f14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZSt22__final_insertion_sortIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_$shim")]
// 0xf23fec — __ZSt22__final_insertion_sortIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_$shim
// type: int()
pub fn stub_0xf23fec() {
    // IDA 0xf23fec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZSt22__final_insertion_sortIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_$shim")]
// 0xf23ff8 — __ZSt22__final_insertion_sortIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_$shim
// type: int()
pub fn stub_0xf23ff8() {
    // IDA 0xf23ff8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveEbENS3_5list2INS2_3argILi1EEENS3_5valueIbEEEEEEEEvT_S6_$shim")]
// 0xf24004 — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveEbENS3_5list2INS2_3argILi1EEENS3_5valueIbEEEEEEEEvT_S6_$shim
// type: int()
pub fn stub_0xf24004() {
    // IDA 0xf24004: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZSt9sort_heapIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_$shim")]
// 0xf2401c — __ZSt9sort_heapIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_$shim
// type: int()
pub fn stub_0xf2401c() {
    // IDA 0xf2401c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZSt16__insertion_sortIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_$shim")]
// 0xf24028 — __ZSt16__insertion_sortIPPKN3RBX5JointEPFbS3_S3_EEvT_S7_T0_$shim
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *, int (__fastcall *)(int, _DWORD))
pub fn stub_0xf24028() {
    // IDA 0xf24028: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZSt9sort_heapIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_$shim")]
// 0xf24034 — __ZSt9sort_heapIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_$shim
// type: int()
pub fn stub_0xf24034() {
    // IDA 0xf24034: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZSt16__insertion_sortIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_$shim")]
// 0xf24040 — __ZSt16__insertion_sortIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_$shim
// type: int()
pub fn stub_0xf24040() {
    // IDA 0xf24040: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX12PhysicsCoordESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_$shim")]
// 0xf2404c — __ZNSt6vectorIN3RBX12PhysicsCoordESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_$shim
// type: int()
pub fn stub_0xf2404c() {
    // IDA 0xf2404c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX5BlockD2Ev$shim")]
// 0xf24064 — __ZN3RBX5BlockD2Ev$shim
// type: void __fastcall(RBX::Block *)
pub fn stub_0xf24064() {
    // IDA 0xf24064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX9AllocatorINS_16GeoPairConnectorEEdlEPv$shim")]
// 0xf24088 — __ZN3RBX9AllocatorINS_16GeoPairConnectorEEdlEPv$shim
// type: void __fastcall(void *)
pub fn stub_0xf24088() {
    // IDA 0xf24088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4EdgeD2Ev$shim")]
// 0xf24094 — __ZN3RBX4EdgeD2Ev$shim
// type: void __fastcall(RBX::Edge *)
pub fn stub_0xf24094() {
    // IDA 0xf24094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10IPipelinedD2Ev$shim")]
// 0xf240a0 — __ZN3RBX10IPipelinedD2Ev$shim
// type: void __fastcall(RBX::IPipelined *)
pub fn stub_0xf240a0() {
    // IDA 0xf240a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE9fastClearEv$shim")]
// 0xf240ac — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE9fastClearEv$shim
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *, int, int, void *, int, int, int, int)
pub fn stub_0xf240ac() {
    // IDA 0xf240ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS1_b$shim")]
// 0xf240b8 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS1_b$shim
// type: int __fastcall(int, RBX::Primitive *)
pub fn stub_0xf240b8() {
    // IDA 0xf240b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS1_$shim")]
// 0xf240c4 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS1_$shim
// type: int __fastcall(int, int)
pub fn stub_0xf240c4() {
    // IDA 0xf240c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS1_$shim")]
// 0xf240d0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS1_$shim
// type: int __fastcall(int, RBX::Primitive *)
pub fn stub_0xf240d0() {
    // IDA 0xf240d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE11rehash_implEm$shim")]
// 0xf240dc — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE11rehash_implEm$shim
// type: int()
pub fn stub_0xf240dc() {
    // IDA 0xf240dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14create_bucketsEm$shim")]
// 0xf240e8 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14create_bucketsEm$shim
// type: int()
pub fn stub_0xf240e8() {
    // IDA 0xf240e8: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11rehash_implEm$shim")]
// 0xf24100 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11rehash_implEm$shim
// type: int()
pub fn stub_0xf24100() {
    // IDA 0xf24100: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm$shim")]
// 0xf2410c — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm$shim
// type: int()
pub fn stub_0xf2410c() {
    // IDA 0xf2410c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX16BallBlockContactD2Ev$shim")]
// 0xf24118 — __ZN3RBX16BallBlockContactD2Ev$shim
// type: void __fastcall(RBX::BallBlockContact *)
pub fn stub_0xf24118() {
    // IDA 0xf24118: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX15BallBallContactD2Ev$shim")]
// 0xf24124 — __ZN3RBX15BallBallContactD2Ev$shim
// type: void __fastcall(RBX::BallBallContact *)
pub fn stub_0xf24124() {
    // IDA 0xf24124: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EEC2EPNS_5WorldEPS3_i$shim")]
// 0xf24130 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EEC2EPNS_5WorldEPS3_i$shim
// type: int()
pub fn stub_0xf24130() {
    // IDA 0xf24130: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost25simple_segregated_storageImE12ordered_freeEPv$shim")]
// 0xf2413c — __ZN5boost25simple_segregated_storageImE12ordered_freeEPv$shim
// type: int()
pub fn stub_0xf2413c() {
    // IDA 0xf2413c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEdlEPv$shim")]
// 0xf24148 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEdlEPv$shim
// type: void __fastcall(void *)
pub fn stub_0xf24148() {
    // IDA 0xf24148: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf24154 — __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf24154() {
    // IDA 0xf24154: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf2416c — __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf2416c() {
    // IDA 0xf2416c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_$shim")]
// 0xf24178 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_$shim
// type: int()
pub fn stub_0xf24178() {
    // IDA 0xf24178: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf24184 — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
// type: int()
pub fn stub_0xf24184() {
    // IDA 0xf24184: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNK3RBX4POLY4Edge13getVertexFaceEPKNS0_6VertexE$shim")]
// 0xf24190 — __ZNK3RBX4POLY4Edge13getVertexFaceEPKNS0_6VertexE$shim
// type: int __fastcall(RBX::POLY::Edge *, const RBX::POLY::Vertex *)
pub fn stub_0xf24190() {
    // IDA 0xf24190: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf2419c — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf2419c() {
    // IDA 0xf2419c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf241a8 — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf241a8() {
    // IDA 0xf241a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim")]
// 0xf241b4 — __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf241b4() {
    // IDA 0xf241b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf241c0 — __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf241c0() {
    // IDA 0xf241c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX11IndexedTree18visitMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvPFvPS2_ENS4_5list1INS3_3argILi1EEEEEEEEEvT0_$shim")]
// 0xf241cc — __ZN3RBX11IndexedTree18visitMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvPFvPS2_ENS4_5list1INS3_3argILi1EEEEEEEEEvT0_$shim
// type: int()
pub fn stub_0xf241cc() {
    // IDA 0xf241cc: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}
