//! core shard mt — 100 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33887, 2123 uncovered before -> 2023 after, batch 0xf22438..0xf22e88).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sBodyVelocityEEEERKS0_v$shim")]
// 0xf22438 — __ZN3RBX4Name9doDeclareILZNS_13sBodyVelocityEEEERKS0_v$shim
// type: int()
pub fn stub_0xf22438() -> ! { todo!("0xf22438 __ZN3RBX4Name9doDeclareILZNS_13sBodyVelocityEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sBodyGyroEEEERKS0_v$shim")]
// 0xf22444 — __ZN3RBX4Name9doDeclareILZNS_9sBodyGyroEEEERKS0_v$shim
// type: int()
pub fn stub_0xf22444() -> ! { todo!("0xf22444 __ZN3RBX4Name9doDeclareILZNS_9sBodyGyroEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sRocketEEEERKS0_v$shim")]
// 0xf22450 — __ZN3RBX4Name9doDeclareILZNS_7sRocketEEEERKS0_v$shim
// type: int()
pub fn stub_0xf22450() -> ! { todo!("0xf22450 __ZN3RBX4Name9doDeclareILZNS_7sRocketEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sBodyPositionEEEERKS0_v$shim")]
// 0xf2245c — __ZN3RBX4Name9doDeclareILZNS_13sBodyPositionEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2245c() -> ! { todo!("0xf2245c __ZN3RBX4Name9doDeclareILZNS_13sBodyPositionEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX7HandlesD2Ev$shim")]
// 0xf224a4 — __ZN3RBX7HandlesD2Ev$shim
// type: void __fastcall(RBX::Handles *)
pub fn stub_0xf224a4() -> ! { todo!("0xf224a4 __ZN3RBX7HandlesD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sHandlesEEEERKS0_v$shim")]
// 0xf224c8 — __ZN3RBX4Name9doDeclareILZNS_8sHandlesEEEERKS0_v$shim
// type: int()
pub fn stub_0xf224c8() -> ! { todo!("0xf224c8 __ZN3RBX4Name9doDeclareILZNS_8sHandlesEEEERKS0_v$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE24safe_static_do_get_mutexEv$shim")]
// 0xf224d4 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf224d4() -> ! { todo!("0xf224d4 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv$shim")]
// 0xf224e0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf224e0() -> ! { todo!("0xf224e0 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf22510 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22510() -> ! { todo!("0xf22510 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf22528 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22528() -> ! { todo!("0xf22528 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8NormalIdEfEEclES3_f$shim")]
// 0xf22534 — __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8NormalIdEfEEclES3_f$shim
// type: int()
pub fn stub_0xf22534() -> ! { todo!("0xf22534 __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8NormalIdEfEEclES3_f$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv$shim")]
// 0xf22540 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf22540() -> ! { todo!("0xf22540 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13disconnectAllEv$shim") }

#[doc(alias = "__ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f$shim")]
// 0xf22558 — __ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f$shim
// type: int()
pub fn stub_0xf22558() -> ! { todo!("0xf22558 __ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f$shim") }

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8NormalIdEEEclES3_$shim")]
// 0xf22564 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8NormalIdEEEclES3_$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf22564() -> ! { todo!("0xf22564 __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8NormalIdEEEclES3_$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv$shim")]
// 0xf22570 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf22570() -> ! { todo!("0xf22570 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv$shim") }

#[doc(alias = "__ZNK5boost9function1IvN3RBX8NormalIdEEclES2_$shim")]
// 0xf22588 — __ZNK5boost9function1IvN3RBX8NormalIdEEclES2_$shim
// type: int()
pub fn stub_0xf22588() -> ! { todo!("0xf22588 __ZNK5boost9function1IvN3RBX8NormalIdEEclES2_$shim") }

#[doc(alias = "__ZN3RBX9HopperBinD2Ev$shim")]
// 0xf225ac — __ZN3RBX9HopperBinD2Ev$shim
// type: void __fastcall(RBX::HopperBin *)
pub fn stub_0xf225ac() -> ! { todo!("0xf225ac __ZN3RBX9HopperBinD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sBackpackItemEEEERKS0_v$shim")]
// 0xf225d0 — __ZN3RBX4Name9doDeclareILZNS_13sBackpackItemEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf225d0() -> ! { todo!("0xf225d0 __ZN3RBX4Name9doDeclareILZNS_13sBackpackItemEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sStarterGearEEEERKS0_v$shim")]
// 0xf225f4 — __ZN3RBX4Name9doDeclareILZNS_12sStarterGearEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf225f4() -> ! { todo!("0xf225f4 __ZN3RBX4Name9doDeclareILZNS_12sStarterGearEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sHopperBinEEEERKS0_v$shim")]
// 0xf22600 — __ZN3RBX4Name9doDeclareILZNS_10sHopperBinEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22600() -> ! { todo!("0xf22600 __ZN3RBX4Name9doDeclareILZNS_10sHopperBinEEEERKS0_v$shim") }

#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9HopperBinEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim")]
// 0xf22618 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9HopperBinEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim
// type: int()
pub fn stub_0xf22618() -> ! { todo!("0xf22618 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9HopperBinEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf22630 — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf22630() -> ! { todo!("0xf22630 __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sGuiImageButtonEEEERKS0_v$shim")]
// 0xf22654 — __ZN3RBX4Name9doDeclareILZNS_15sGuiImageButtonEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22654() -> ! { todo!("0xf22654 __ZN3RBX4Name9doDeclareILZNS_15sGuiImageButtonEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sImageLabelEEEERKS0_v$shim")]
// 0xf22678 — __ZN3RBX4Name9doDeclareILZNS_11sImageLabelEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22678() -> ! { todo!("0xf22678 __ZN3RBX4Name9doDeclareILZNS_11sImageLabelEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX13InsertServiceD2Ev$shim")]
// 0xf22684 — __ZN3RBX13InsertServiceD2Ev$shim
// type: void __fastcall(RBX::InsertService *)
pub fn stub_0xf22684() -> ! { todo!("0xf22684 __ZN3RBX13InsertServiceD2Ev$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE24safe_static_do_get_mutexEv$shim")]
// 0xf226c0 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf226c0() -> ! { todo!("0xf226c0 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf226cc — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot24safe_static_do_get_mutexEv$shim
// type: void *()
pub fn stub_0xf226cc() -> ! { todo!("0xf226cc __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvSsiiEE13disconnectAllEv$shim")]
// 0xf22714 — __ZN3rbx7signals6signalIFvSsiiEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf22714() -> ! { todo!("0xf22714 __ZN3rbx7signals6signalIFvSsiiEE13disconnectAllEv$shim") }

#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsii$shim")]
// 0xf22720 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsii$shim
// type: void __fastcall(int, const std::string *)
pub fn stub_0xf22720() -> ! { todo!("0xf22720 __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsii$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13disconnectAllEv$shim")]
// 0xf2272c — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf2272c() -> ! { todo!("0xf2272c __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13disconnectAllEv$shim") }

#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_$shim")]
// 0xf22750 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_$shim
// type: int()
pub fn stub_0xf22750() -> ! { todo!("0xf22750 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDynamicRotateEEEERKS0_v$shim")]
// 0xf22840 — __ZN3RBX4Name9doDeclareILZNS_14sDynamicRotateEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22840() -> ! { todo!("0xf22840 __ZN3RBX4Name9doDeclareILZNS_14sDynamicRotateEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sMotor6DEEEERKS0_v$shim")]
// 0xf22864 — __ZN3RBX4Name9doDeclareILZNS_8sMotor6DEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22864() -> ! { todo!("0xf22864 __ZN3RBX4Name9doDeclareILZNS_8sMotor6DEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sManualGlueEEEERKS0_v$shim")]
// 0xf22870 — __ZN3RBX4Name9doDeclareILZNS_11sManualGlueEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22870() -> ! { todo!("0xf22870 __ZN3RBX4Name9doDeclareILZNS_11sManualGlueEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sManualWeldEEEERKS0_v$shim")]
// 0xf2287c — __ZN3RBX4Name9doDeclareILZNS_11sManualWeldEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2287c() -> ! { todo!("0xf2287c __ZN3RBX4Name9doDeclareILZNS_11sManualWeldEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sWeldEEEERKS0_v$shim")]
// 0xf22894 — __ZN3RBX4Name9doDeclareILZNS_5sWeldEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22894() -> ! { todo!("0xf22894 __ZN3RBX4Name9doDeclareILZNS_5sWeldEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sMotorEEEERKS0_v$shim")]
// 0xf228a0 — __ZN3RBX4Name9doDeclareILZNS_6sMotorEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf228a0() -> ! { todo!("0xf228a0 __ZN3RBX4Name9doDeclareILZNS_6sMotorEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ManualGlueJointD0Ev$shim")]
// 0xf228b8 — __ZN3RBX15ManualGlueJointD0Ev$shim
// type: void __fastcall(RBX::ManualGlueJoint *)
pub fn stub_0xf228b8() -> ! { todo!("0xf228b8 __ZN3RBX15ManualGlueJointD0Ev$shim") }

#[doc(alias = "__ZN3RBX15ManualWeldJointD0Ev$shim")]
// 0xf228c4 — __ZN3RBX15ManualWeldJointD0Ev$shim
// type: void __fastcall(RBX::ManualWeldJoint *)
pub fn stub_0xf228c4() -> ! { todo!("0xf228c4 __ZN3RBX15ManualWeldJointD0Ev$shim") }

#[doc(alias = "__ZN3RBX9WeldJointD0Ev$shim")]
// 0xf228d0 — __ZN3RBX9WeldJointD0Ev$shim
// type: void __fastcall(RBX::WeldJoint *)
pub fn stub_0xf228d0() -> ! { todo!("0xf228d0 __ZN3RBX9WeldJointD0Ev$shim") }

#[doc(alias = "__ZN3RBX9SnapJointD0Ev$shim")]
// 0xf228dc — __ZN3RBX9SnapJointD0Ev$shim
// type: void __fastcall(RBX::SnapJoint *)
pub fn stub_0xf228dc() -> ! { todo!("0xf228dc __ZN3RBX9SnapJointD0Ev$shim") }

#[doc(alias = "__ZN3RBX13JointsServiceD2Ev$shim")]
// 0xf228e8 — __ZN3RBX13JointsServiceD2Ev$shim
// type: void __fastcall(RBX::JointsService *)
pub fn stub_0xf228e8() -> ! { todo!("0xf228e8 __ZN3RBX13JointsServiceD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_8sRotateVEEEERKS0_v$shim")]
// 0xf22930 — __ZN3RBX4Name7declareILZNS_8sRotateVEEEERKS0_v$shim
// type: int()
pub fn stub_0xf22930() -> ! { todo!("0xf22930 __ZN3RBX4Name7declareILZNS_8sRotateVEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sRotateVEEEERKS0_v$shim")]
// 0xf2293c — __ZN3RBX4Name9doDeclareILZNS_8sRotateVEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf2293c() -> ! { todo!("0xf2293c __ZN3RBX4Name9doDeclareILZNS_8sRotateVEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_8sRotatePEEEERKS0_v$shim")]
// 0xf22948 — __ZN3RBX4Name7declareILZNS_8sRotatePEEEERKS0_v$shim
// type: int()
pub fn stub_0xf22948() -> ! { todo!("0xf22948 __ZN3RBX4Name7declareILZNS_8sRotatePEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sRotatePEEEERKS0_v$shim")]
// 0xf22954 — __ZN3RBX4Name9doDeclareILZNS_8sRotatePEEEERKS0_v$shim
// type: int()
pub fn stub_0xf22954() -> ! { todo!("0xf22954 __ZN3RBX4Name9doDeclareILZNS_8sRotatePEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_7sRotateEEEERKS0_v$shim")]
// 0xf22960 — __ZN3RBX4Name7declareILZNS_7sRotateEEEERKS0_v$shim
// type: int()
pub fn stub_0xf22960() -> ! { todo!("0xf22960 __ZN3RBX4Name7declareILZNS_7sRotateEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sRotateEEEERKS0_v$shim")]
// 0xf2296c — __ZN3RBX4Name9doDeclareILZNS_7sRotateEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2296c() -> ! { todo!("0xf2296c __ZN3RBX4Name9doDeclareILZNS_7sRotateEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5sGlueEEEERKS0_v$shim")]
// 0xf22978 — __ZN3RBX4Name7declareILZNS_5sGlueEEEERKS0_v$shim
// type: int()
pub fn stub_0xf22978() -> ! { todo!("0xf22978 __ZN3RBX4Name7declareILZNS_5sGlueEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sGlueEEEERKS0_v$shim")]
// 0xf22984 — __ZN3RBX4Name9doDeclareILZNS_5sGlueEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22984() -> ! { todo!("0xf22984 __ZN3RBX4Name9doDeclareILZNS_5sGlueEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5sSnapEEEERKS0_v$shim")]
// 0xf22990 — __ZN3RBX4Name7declareILZNS_5sSnapEEEERKS0_v$shim
// type: int()
pub fn stub_0xf22990() -> ! { todo!("0xf22990 __ZN3RBX4Name7declareILZNS_5sSnapEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sSnapEEEERKS0_v$shim")]
// 0xf2299c — __ZN3RBX4Name9doDeclareILZNS_5sSnapEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf2299c() -> ! { todo!("0xf2299c __ZN3RBX4Name9doDeclareILZNS_5sSnapEEEERKS0_v$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv$shim")]
// 0xf229a8 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf229a8() -> ! { todo!("0xf229a8 __ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf229b4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf229b4() -> ! { todo!("0xf229b4 __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sKeyframeEEEERKS0_v$shim")]
// 0xf229d8 — __ZN3RBX4Name9doDeclareILZNS_9sKeyframeEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf229d8() -> ! { todo!("0xf229d8 __ZN3RBX4Name9doDeclareILZNS_9sKeyframeEEEERKS0_v$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf229f0 — __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
// type: int()
pub fn stub_0xf229f0() -> ! { todo!("0xf229f0 __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf229fc — __ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf229fc() -> ! { todo!("0xf229fc __ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3RBX16KeyframeSequenceD1Ev$shim")]
// 0xf22a14 — __ZN3RBX16KeyframeSequenceD1Ev$shim
// type: void __fastcall(RBX::KeyframeSequence *)
pub fn stub_0xf22a14() -> ! { todo!("0xf22a14 __ZN3RBX16KeyframeSequenceD1Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sKeyframeSequenceEEEERKS0_v$shim")]
// 0xf22a2c — __ZN3RBX4Name9doDeclareILZNS_17sKeyframeSequenceEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22a2c() -> ! { todo!("0xf22a2c __ZN3RBX4Name9doDeclareILZNS_17sKeyframeSequenceEEEERKS0_v$shim") }

#[doc(alias = "__ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEaSERKS4_$shim")]
// 0xf22a38 — __ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEaSERKS4_$shim
// type: int()
pub fn stub_0xf22a38() -> ! { todo!("0xf22a38 __ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EEaSERKS4_$shim") }

#[doc(alias = "__ZNSt6vectorISt4pairImmESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf22a44 — __ZNSt6vectorISt4pairImmESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
// type: int()
pub fn stub_0xf22a44() -> ! { todo!("0xf22a44 __ZNSt6vectorISt4pairImmESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf22a50 — __ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf22a50() -> ! { todo!("0xf22a50 __ZNSt6vectorIN3RBX16KeyframeSequence8PriorityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE23removeLeastRecentlyUsedEv$shim")]
// 0xf22a74 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE23removeLeastRecentlyUsedEv$shim
// type: int()
pub fn stub_0xf22a74() -> ! { todo!("0xf22a74 __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE23removeLeastRecentlyUsedEv$shim") }

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
// 0xf22a80 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim
// type: int()
pub fn stub_0xf22a80() -> ! { todo!("0xf22a80 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim") }

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
// 0xf22a8c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
pub fn stub_0xf22a8c() -> ! { todo!("0xf22a8c __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim") }

#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSJ_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEmRKT_RKT0_$shim")]
// 0xf22a98 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSJ_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEmRKT_RKT0_$shim
// type: int()
pub fn stub_0xf22a98() -> ! { todo!("0xf22a98 __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSJ_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEmRKT_RKT0_$shim") }

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf22aa4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf22aa4() -> ! { todo!("0xf22aa4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim") }

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_$shim")]
// 0xf22ab0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_$shim
// type: int()
pub fn stub_0xf22ab0() -> ! { todo!("0xf22ab0 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_$shim") }

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf22abc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf22abc() -> ! { todo!("0xf22abc __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf22ac8 — __ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf22ac8() -> ! { todo!("0xf22ac8 __ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3RBX8LightingD2Ev$shim")]
// 0xf22ad4 — __ZN3RBX8LightingD2Ev$shim
// type: void __fastcall(RBX::Lighting *)
pub fn stub_0xf22ad4() -> ! { todo!("0xf22ad4 __ZN3RBX8LightingD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sHintEEEERKS0_v$shim")]
// 0xf22b4c — __ZN3RBX4Name9doDeclareILZNS_5sHintEEEERKS0_v$shim
// type: int()
pub fn stub_0xf22b4c() -> ! { todo!("0xf22b4c __ZN3RBX4Name9doDeclareILZNS_5sHintEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sMessageEEEERKS0_v$shim")]
// 0xf22b58 — __ZN3RBX4Name9doDeclareILZNS_8sMessageEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22b58() -> ! { todo!("0xf22b58 __ZN3RBX4Name9doDeclareILZNS_8sMessageEEEERKS0_v$shim") }

#[doc(alias = "__ZNSt6vectorIPN3RBX14IModelModifierESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf22b64 — __ZNSt6vectorIPN3RBX14IModelModifierESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf22b64() -> ! { todo!("0xf22b64 __ZNSt6vectorIPN3RBX14IModelModifierESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sMouseEEEERKS0_v$shim")]
// 0xf22ba0 — __ZN3RBX4Name9doDeclareILZNS_6sMouseEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22ba0() -> ! { todo!("0xf22ba0 __ZN3RBX4Name9doDeclareILZNS_6sMouseEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_4PART6sWedgeEEEERKS0_v$shim")]
// 0xf22be8 — __ZN3RBX4Name9doDeclareILZNS_4PART6sWedgeEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22be8() -> ! { todo!("0xf22be8 __ZN3RBX4Name9doDeclareILZNS_4PART6sWedgeEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sPartEEEERKS0_v$shim")]
// 0xf22c24 — __ZN3RBX4Name9doDeclareILZNS_5sPartEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22c24() -> ! { todo!("0xf22c24 __ZN3RBX4Name9doDeclareILZNS_5sPartEEEERKS0_v$shim") }

#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE5cloneEv$shim")]
// 0xf22c60 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE5cloneEv$shim
// type: int()
pub fn stub_0xf22c60() -> ! { todo!("0xf22c60 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE5cloneEv$shim") }

#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED2Ev$shim")]
// 0xf22c90 — __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED2Ev$shim
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf22c90() -> ! { todo!("0xf22c90 __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sFWServiceEEEERKS0_v$shim")]
// 0xf22c9c — __ZN3RBX4Name9doDeclareILZNS_10sFWServiceEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22c9c() -> ! { todo!("0xf22c9c __ZN3RBX4Name9doDeclareILZNS_10sFWServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_9FWServiceEEEmv$shim")]
// 0xf22ca8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_9FWServiceEEEmv$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22ca8() -> ! { todo!("0xf22ca8 __ZN3RBX15ServiceProvider15doGetClassIndexINS_9FWServiceEEEmv$shim") }

#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE5cloneEv$shim")]
// 0xf22cb4 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE5cloneEv$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22cb4() -> ! { todo!("0xf22cb4 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEE5cloneEv$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX8MaterialESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf22d5c — __ZNSt6vectorIN3RBX8MaterialESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
// type: int()
pub fn stub_0xf22d5c() -> ! { todo!("0xf22d5c __ZNSt6vectorIN3RBX8MaterialESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim") }

#[doc(alias = "__ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs$shim")]
// 0xf22d68 — __ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs$shim
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf22d68() -> ! { todo!("0xf22d68 __ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs$shim") }

#[doc(alias = "__ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim")]
// 0xf22d8c — __ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf22d8c() -> ! { todo!("0xf22d8c __ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim") }

#[doc(alias = "__ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEED2Ev$shim")]
// 0xf22d98 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEED2Ev$shim
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf22d98() -> ! { todo!("0xf22d98 __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEED2Ev$shim") }

#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorISt12length_errorED2Ev$shim")]
// 0xf22da4 — __ZN5boost16exception_detail19error_info_injectorISt12length_errorED2Ev$shim
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf22da4() -> ! { todo!("0xf22da4 __ZN5boost16exception_detail19error_info_injectorISt12length_errorED2Ev$shim") }

#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv$shim")]
// 0xf22db0 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22db0() -> ! { todo!("0xf22db0 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEE5cloneEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv$shim")]
// 0xf22dc8 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf22dc8() -> ! { todo!("0xf22dc8 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf22dd4 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22dd4() -> ! { todo!("0xf22dd4 __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED0Ev$shim")]
// 0xf22dec — __ZN3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED0Ev$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22dec() -> ! { todo!("0xf22dec __ZN3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED0Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sBasePlayerGuiEEEERKS0_v$shim")]
// 0xf22e04 — __ZN3RBX4Name9doDeclareILZNS_14sBasePlayerGuiEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22e04() -> ! { todo!("0xf22e04 __ZN3RBX4Name9doDeclareILZNS_14sBasePlayerGuiEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX17StarterGuiServiceD2Ev$shim")]
// 0xf22e1c — __ZN3RBX17StarterGuiServiceD2Ev$shim
// type: void __fastcall(RBX::StarterGuiService *)
pub fn stub_0xf22e1c() -> ! { todo!("0xf22e1c __ZN3RBX17StarterGuiServiceD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sPlayerGuiEEEERKS0_v$shim")]
// 0xf22e34 — __ZN3RBX4Name9doDeclareILZNS_10sPlayerGuiEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22e34() -> ! { todo!("0xf22e34 __ZN3RBX4Name9doDeclareILZNS_10sPlayerGuiEEEERKS0_v$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv$shim")]
// 0xf22e40 — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf22e40() -> ! { todo!("0xf22e40 __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm$shim")]
// 0xf22e4c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm$shim
// type: int()
pub fn stub_0xf22e4c() -> ! { todo!("0xf22e4c __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm$shim") }

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm$shim")]
// 0xf22e58 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
pub fn stub_0xf22e58() -> ! { todo!("0xf22e58 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf22e70 — __ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf22e70() -> ! { todo!("0xf22e70 __ZNSt6vectorIN3RBX17StarterGuiService11CoreGuiTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvN3RBX17StarterGuiService11CoreGuiTypeEbEEclES4_b$shim")]
// 0xf22e7c — __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX17StarterGuiService11CoreGuiTypeEbEEclES4_b$shim
// type: int __fastcall(int, int)
pub fn stub_0xf22e7c() -> ! { todo!("0xf22e7c __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX17StarterGuiService11CoreGuiTypeEbEEclES4_b$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13disconnectAllEv$shim")]
// 0xf22e88 — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf22e88() -> ! { todo!("0xf22e88 __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE13disconnectAllEv$shim") }
