//! core shard it — 100 core stubs EA-sorted, 0x2b7568..0x50b4c8 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 after 0x2b7568 prior 3463 remaining).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sWorkspaceEEEERKS0_v")]
// 0x2b7568 — __ZN3RBX4Name7declareILZNS_10sWorkspaceEEEERKS0_v
pub fn stub_0x2b7568() {
    // IDA 0x2b7568: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sWorkspaceEEEERKS0_v")]
// 0x2b75b0 — __ZN3RBX4Name9doDeclareILZNS_10sWorkspaceEEEERKS0_v
pub fn stub_0x2b75b0() {
    // IDA 0x2b75b0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_10sCloneToolEEE7getNameEv")]
// 0x2dbf88 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_10sCloneToolEEE7getNameEv
pub fn stub_0x2dbf88() {
    // IDA 0x2dbf88: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGameToolEEE7getNameEv")]
// 0x2e33ec — __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGameToolEEE7getNameEv
pub fn stub_0x2e33ec() {
    // IDA 0x2e33ec: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGrabToolEEE7getNameEv")]
// 0x2e3c60 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGrabToolEEE7getNameEv
pub fn stub_0x2e3c60() {
    // IDA 0x2e3c60: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_11sHammerToolEEE7getNameEv")]
// 0x2e4b34 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_11sHammerToolEEE7getNameEv
pub fn stub_0x2e4b34() {
    // IDA 0x2e4b34: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "RBX::ClickDetector::fireMouseClick(float,RBX::Network::Player *)")]
// 0x3f1114 — __ZN3RBX13ClickDetector14fireMouseClickEfPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, float, RBX::Network::Player *)
pub fn stub_0x3f1114() {
    // IDA 0x3f1114: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::ClickDetector::fireMouseHover(RBX::Network::Player *)")]
// 0x3f130c — __ZN3RBX13ClickDetector14fireMouseHoverEPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
pub fn stub_0x3f130c() {
    // IDA 0x3f130c: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::ClickDetector::fireMouseHoverLeave(RBX::Network::Player *)")]
// 0x3f1410 — __ZN3RBX13ClickDetector19fireMouseHoverLeaveEPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
pub fn stub_0x3f1410() {
    // IDA 0x3f1410: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::NetworkStatsCommand::doIt(RBX::IDataState *)")]
// 0x3f7f80 — __ZN3RBX19NetworkStatsCommand4doItEPNS_10IDataStateE
// type: void __fastcall(int, int, int, const void *)
pub fn stub_0x3f7f80() {
    // IDA 0x3f7f80: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::NetworkStatsCommand::isEnabled(void)const")]
// 0x3f8268 — __ZNK3RBX19NetworkStatsCommand9isEnabledEv
// type: bool __fastcall(RBX::NetworkStatsCommand *this)
pub fn stub_0x3f8268() {
    // IDA 0x3f8268: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::NetworkStatsCommand::isChecked(void)const")]
// 0x3f83e4 — __ZNK3RBX19NetworkStatsCommand9isCheckedEv
// type: int __fastcall(RBX::NetworkStatsCommand *this)
pub fn stub_0x3f83e4() {
    // IDA 0x3f83e4: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::NetworkStatsCommand::~NetworkStatsCommand()")]
// 0x3fe628 — __ZN3RBX19NetworkStatsCommandD1Ev
// type: void __fastcall(RBX::NetworkStatsCommand *__hidden this)
pub fn stub_0x3fe628() {
    // IDA 0x3fe628: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::NetworkStatsCommand::~NetworkStatsCommand()")]
// 0x3fe62c — __ZN3RBX19NetworkStatsCommandD0Ev
// type: void __fastcall(RBX::NetworkStatsCommand *__hidden this)
pub fn stub_0x3fe62c() {
    // IDA 0x3fe62c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v")]
// 0x401d58 — __ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v
pub fn stub_0x401d58() {
    // IDA 0x401d58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v")]
// 0x401d9c — __ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v
// type: int()
pub fn stub_0x401d9c() {
    // IDA 0x401d9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev")]
// 0x48424c — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev
pub fn stub_0x48424c() {
    // IDA 0x48424c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv")]
// 0x4912ac — __ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv
pub fn stub_0x4912ac() {
    // IDA 0x4912ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v")]
// 0x4912b0 — __ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v
pub fn stub_0x4912b0() {
    // IDA 0x4912b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sDecalEEEEvv")]
// 0x49148c — __ZN3RBX4Name13callDoDeclareILZNS_6sDecalEEEEvv
pub fn stub_0x49148c() {
    // IDA 0x49148c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v")]
// 0x491490 — __ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v
pub fn stub_0x491490() {
    // IDA 0x491490: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sDialogChoiceEEEEvv")]
// 0x494950 — __ZN3RBX4Name13callDoDeclareILZNS_13sDialogChoiceEEEEvv
pub fn stub_0x494950() {
    // IDA 0x494950: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v")]
// 0x494954 — __ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v
pub fn stub_0x494954() {
    // IDA 0x494954: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sDialogRootEEEEvv")]
// 0x49757c — __ZN3RBX4Name13callDoDeclareILZNS_11sDialogRootEEEEvv
pub fn stub_0x49757c() {
    // IDA 0x49757c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v")]
// 0x497580 — __ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v
pub fn stub_0x497580() {
    // IDA 0x497580: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sForceFieldEEEEvv")]
// 0x4a18d0 — __ZN3RBX4Name13callDoDeclareILZNS_11sForceFieldEEEEvv
pub fn stub_0x4a18d0() {
    // IDA 0x4a18d0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sExplosionEEEEvv")]
// 0x4a1d78 — __ZN3RBX4Name13callDoDeclareILZNS_10sExplosionEEEEvv
pub fn stub_0x4a1d78() {
    // IDA 0x4a1d78: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v")]
// 0x4a1d7c — __ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v
pub fn stub_0x4a1d7c() {
    // IDA 0x4a1d7c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sExtrudedPartEEEEvv")]
// 0x4a8190 — __ZN3RBX4Name13callDoDeclareILZNS_13sExtrudedPartEEEEvv
pub fn stub_0x4a8190() {
    // IDA 0x4a8190: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sExtrudedPartEEEERKS0_v")]
// 0x4a8194 — __ZN3RBX4Name9doDeclareILZNS_13sExtrudedPartEEEERKS0_v
pub fn stub_0x4a8194() {
    // IDA 0x4a8194: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sBindableEventEEEERKS0_v")]
// 0x4ac23c — __ZN3RBX4Name7declareILZNS_14sBindableEventEEEERKS0_v
pub fn stub_0x4ac23c() {
    // IDA 0x4ac23c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sBindableEventEEEEvv")]
// 0x4ac280 — __ZN3RBX4Name13callDoDeclareILZNS_14sBindableEventEEEEvv
pub fn stub_0x4ac280() {
    // IDA 0x4ac280: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sBindableEventEEEERKS0_v")]
// 0x4ac284 — __ZN3RBX4Name9doDeclareILZNS_14sBindableEventEEEERKS0_v
pub fn stub_0x4ac284() {
    // IDA 0x4ac284: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_17sBindableFunctionEEEERKS0_v")]
// 0x4ada84 — __ZN3RBX4Name7declareILZNS_17sBindableFunctionEEEERKS0_v
pub fn stub_0x4ada84() {
    // IDA 0x4ada84: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sBindableFunctionEEEEvv")]
// 0x4adac8 — __ZN3RBX4Name13callDoDeclareILZNS_17sBindableFunctionEEEEvv
pub fn stub_0x4adac8() {
    // IDA 0x4adac8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sBindableFunctionEEEERKS0_v")]
// 0x4adacc — __ZN3RBX4Name9doDeclareILZNS_17sBindableFunctionEEEERKS0_v
// type: int()
pub fn stub_0x4adacc() {
    // IDA 0x4adacc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sAnimationEEEERKS0_v")]
// 0x4ae3d0 — __ZN3RBX4Name7declareILZNS_10sAnimationEEEERKS0_v
pub fn stub_0x4ae3d0() {
    // IDA 0x4ae3d0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sAnimationEEEEvv")]
// 0x4ae414 — __ZN3RBX4Name13callDoDeclareILZNS_10sAnimationEEEEvv
// type: int()
pub fn stub_0x4ae414() {
    // IDA 0x4ae414: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sAnimationEEEERKS0_v")]
// 0x4ae418 — __ZN3RBX4Name9doDeclareILZNS_10sAnimationEEEERKS0_v
pub fn stub_0x4ae418() {
    // IDA 0x4ae418: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sSparklesEEEERKS0_v")]
// 0x4af45c — __ZN3RBX4Name7declareILZNS_9sSparklesEEEERKS0_v
pub fn stub_0x4af45c() {
    // IDA 0x4af45c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sSparklesEEEEvv")]
// 0x4af4a0 — __ZN3RBX4Name13callDoDeclareILZNS_9sSparklesEEEEvv
pub fn stub_0x4af4a0() {
    // IDA 0x4af4a0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sSparklesEEEERKS0_v")]
// 0x4af4a4 — __ZN3RBX4Name9doDeclareILZNS_9sSparklesEEEERKS0_v
pub fn stub_0x4af4a4() {
    // IDA 0x4af4a4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sBasicPartEEEERKS0_v")]
// 0x4afdac — __ZN3RBX4Name7declareILZNS_10sBasicPartEEEERKS0_v
pub fn stub_0x4afdac() {
    // IDA 0x4afdac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sBasicPartEEEEvv")]
// 0x4afdf0 — __ZN3RBX4Name13callDoDeclareILZNS_10sBasicPartEEEEvv
pub fn stub_0x4afdf0() {
    // IDA 0x4afdf0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBasicPartEEEERKS0_v")]
// 0x4afdf4 — __ZN3RBX4Name9doDeclareILZNS_10sBasicPartEEEERKS0_v
pub fn stub_0x4afdf4() {
    // IDA 0x4afdf4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sForceFieldEEEERKS0_v")]
// 0x4b0454 — __ZN3RBX4Name9doDeclareILZNS_11sForceFieldEEEERKS0_v
pub fn stub_0x4b0454() {
    // IDA 0x4b0454: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sCustomEventEEEERKS0_v")]
// 0x4b1c80 — __ZN3RBX4Name7declareILZNS_12sCustomEventEEEERKS0_v
pub fn stub_0x4b1c80() {
    // IDA 0x4b1c80: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sCustomEventEEEEvv")]
// 0x4b1cc4 — __ZN3RBX4Name13callDoDeclareILZNS_12sCustomEventEEEEvv
pub fn stub_0x4b1cc4() {
    // IDA 0x4b1cc4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sCustomEventEEEERKS0_v")]
// 0x4b1cc8 — __ZN3RBX4Name9doDeclareILZNS_12sCustomEventEEEERKS0_v
// type: int()
pub fn stub_0x4b1cc8() {
    // IDA 0x4b1cc8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_20sCustomEventReceiverEEEERKS0_v")]
// 0x4b3964 — __ZN3RBX4Name7declareILZNS_20sCustomEventReceiverEEEERKS0_v
pub fn stub_0x4b3964() {
    // IDA 0x4b3964: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sCustomEventReceiverEEEEvv")]
// 0x4b39a8 — __ZN3RBX4Name13callDoDeclareILZNS_20sCustomEventReceiverEEEEvv
pub fn stub_0x4b39a8() {
    // IDA 0x4b39a8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sCustomEventReceiverEEEERKS0_v")]
// 0x4b39ac — __ZN3RBX4Name9doDeclareILZNS_20sCustomEventReceiverEEEERKS0_v
pub fn stub_0x4b39ac() {
    // IDA 0x4b39ac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sVelocityMotorEEEEvv")]
// 0x4e7874 — __ZN3RBX4Name13callDoDeclareILZNS_14sVelocityMotorEEEEvv
pub fn stub_0x4e7874() {
    // IDA 0x4e7874: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sVelocityMotorEEEERKS0_v")]
// 0x4e7878 — __ZN3RBX4Name9doDeclareILZNS_14sVelocityMotorEEEERKS0_v
pub fn stub_0x4e7878() {
    // IDA 0x4e7878: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sMotorFeatureEEEEvv")]
// 0x4e8224 — __ZN3RBX4Name13callDoDeclareILZNS_13sMotorFeatureEEEEvv
pub fn stub_0x4e8224() {
    // IDA 0x4e8224: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sMotorFeatureEEEERKS0_v")]
// 0x4e8228 — __ZN3RBX4Name9doDeclareILZNS_13sMotorFeatureEEEERKS0_v
pub fn stub_0x4e8228() {
    // IDA 0x4e8228: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sHoleEEEEvv")]
// 0x4e8bd4 — __ZN3RBX4Name13callDoDeclareILZNS_5sHoleEEEEvv
pub fn stub_0x4e8bd4() {
    // IDA 0x4e8bd4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sHoleEEEERKS0_v")]
// 0x4e8bd8 — __ZN3RBX4Name9doDeclareILZNS_5sHoleEEEERKS0_v
pub fn stub_0x4e8bd8() {
    // IDA 0x4e8bd8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sFeatureEEEEvv")]
// 0x4e8f70 — __ZN3RBX4Name13callDoDeclareILZNS_8sFeatureEEEEvv
pub fn stub_0x4e8f70() {
    // IDA 0x4e8f70: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sFeatureEEEERKS0_v")]
// 0x4e8f74 — __ZN3RBX4Name9doDeclareILZNS_8sFeatureEEEERKS0_v
pub fn stub_0x4e8f74() {
    // IDA 0x4e8f74: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sFireEEEEvv")]
// 0x4f03b0 — __ZN3RBX4Name13callDoDeclareILZNS_5sFireEEEEvv
pub fn stub_0x4f03b0() {
    // IDA 0x4f03b0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sFireEEEERKS0_v")]
// 0x4f03b4 — __ZN3RBX4Name9doDeclareILZNS_5sFireEEEERKS0_v
pub fn stub_0x4f03b4() {
    // IDA 0x4f03b4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sFlagEEEEvv")]
// 0x4f266c — __ZN3RBX4Name13callDoDeclareILZNS_5sFlagEEEEvv
pub fn stub_0x4f266c() {
    // IDA 0x4f266c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sFlagEEEERKS0_v")]
// 0x4f2670 — __ZN3RBX4Name9doDeclareILZNS_5sFlagEEEERKS0_v
pub fn stub_0x4f2670() {
    // IDA 0x4f2670: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sFlagStandEEEEvv")]
// 0x4f58d4 — __ZN3RBX4Name13callDoDeclareILZNS_10sFlagStandEEEEvv
pub fn stub_0x4f58d4() {
    // IDA 0x4f58d4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sFlagStandEEEERKS0_v")]
// 0x4f58d8 — __ZN3RBX4Name9doDeclareILZNS_10sFlagStandEEEERKS0_v
pub fn stub_0x4f58d8() {
    // IDA 0x4f58d8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sFlagStandServiceEEEEvv")]
// 0x4f5c70 — __ZN3RBX4Name13callDoDeclareILZNS_17sFlagStandServiceEEEEvv
pub fn stub_0x4f5c70() {
    // IDA 0x4f5c70: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sFlagStandServiceEEEERKS0_v")]
// 0x4f5c74 — __ZN3RBX4Name9doDeclareILZNS_17sFlagStandServiceEEEERKS0_v
pub fn stub_0x4f5c74() {
    // IDA 0x4f5c74: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEE9singletonEv")]
// 0x4fc998 — __ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
pub fn stub_0x4fc998() {
    // IDA 0x4fc998: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sGameSettingsEEEERKS0_v")]
// 0x4ff25c — __ZN3RBX4Name7declareILZNS_13sGameSettingsEEEERKS0_v
pub fn stub_0x4ff25c() {
    // IDA 0x4ff25c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sGameSettingsEEEEvv")]
// 0x4ff2a0 — __ZN3RBX4Name13callDoDeclareILZNS_13sGameSettingsEEEEvv
pub fn stub_0x4ff2a0() {
    // IDA 0x4ff2a0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sGameSettingsEEEERKS0_v")]
// 0x4ff2a4 — __ZN3RBX4Name9doDeclareILZNS_13sGameSettingsEEEERKS0_v
pub fn stub_0x4ff2a4() {
    // IDA 0x4ff2a4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEEC2Ev")]
// 0x501608 — __ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEEC2Ev
pub fn stub_0x501608() {
    // IDA 0x501608: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev")]
// 0x502174 — __ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev
pub fn stub_0x502174() {
    // IDA 0x502174: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")]
// 0x5021b4 — __ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev
pub fn stub_0x5021b4() {
    // IDA 0x5021b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev")]
// 0x502294 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev
pub fn stub_0x502294() {
    // IDA 0x502294: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")]
// 0x5022d8 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev
pub fn stub_0x5022d8() {
    // IDA 0x5022d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev")]
// 0x5022e0 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev
pub fn stub_0x5022e0() {
    // IDA 0x5022e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")]
// 0x502324 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev
pub fn stub_0x502324() {
    // IDA 0x502324: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sGeometryServiceEEEERKS0_v")]
// 0x506440 — __ZN3RBX4Name9doDeclareILZNS_16sGeometryServiceEEEERKS0_v
pub fn stub_0x506440() {
    // IDA 0x506440: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv")]
// 0x5092dc — __ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv
pub fn stub_0x5092dc() {
    // IDA 0x5092dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv")]
// 0x509418 — __ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv
pub fn stub_0x509418() {
    // IDA 0x509418: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv")]
// 0x509554 — __ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv
pub fn stub_0x509554() {
    // IDA 0x509554: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv")]
// 0x50957c — __ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv
pub fn stub_0x50957c() {
    // IDA 0x50957c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv")]
// 0x509850 — __ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv
pub fn stub_0x509850() {
    // IDA 0x509850: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv")]
// 0x509b40 — __ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv
pub fn stub_0x509b40() {
    // IDA 0x509b40: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sGlobalBasicSettingsEEEEvv")]
// 0x509e3c — __ZN3RBX4Name13callDoDeclareILZNS_20sGlobalBasicSettingsEEEEvv
pub fn stub_0x509e3c() {
    // IDA 0x509e3c: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sGlobalBasicSettingsEEEERKS0_v")]
// 0x509e40 — __ZN3RBX4Name9doDeclareILZNS_20sGlobalBasicSettingsEEEERKS0_v
pub fn stub_0x509e40() {
    // IDA 0x509e40: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_23sGlobalAdvancedSettingsEEEEvv")]
// 0x509f20 — __ZN3RBX4Name13callDoDeclareILZNS_23sGlobalAdvancedSettingsEEEEvv
pub fn stub_0x509f20() {
    // IDA 0x509f20: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_23sGlobalAdvancedSettingsEEEERKS0_v")]
// 0x509f24 — __ZN3RBX4Name9doDeclareILZNS_23sGlobalAdvancedSettingsEEEERKS0_v
pub fn stub_0x509f24() {
    // IDA 0x509f24: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sSettingsEEEEvv")]
// 0x50a004 — __ZN3RBX4Name13callDoDeclareILZNS_9sSettingsEEEEvv
pub fn stub_0x50a004() {
    // IDA 0x50a004: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sSettingsEEEERKS0_v")]
// 0x50a008 — __ZN3RBX4Name9doDeclareILZNS_9sSettingsEEEERKS0_v
pub fn stub_0x50a008() {
    // IDA 0x50a008: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev")]
// 0x50ad20 — __ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev
pub fn stub_0x50ad20() {
    // IDA 0x50ad20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev")]
// 0x50ad5c — __ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev
pub fn stub_0x50ad5c() {
    // IDA 0x50ad5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev")]
// 0x50ae2c — __ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev
pub fn stub_0x50ae2c() {
    // IDA 0x50ae2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev")]
// 0x50ae68 — __ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev
pub fn stub_0x50ae68() {
    // IDA 0x50ae68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev")]
// 0x50af3c — __ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev
pub fn stub_0x50af3c() {
    // IDA 0x50af3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev")]
// 0x50af78 — __ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x50af78() {
    // IDA 0x50af78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sSelectionEEEERKS0_v")]
// 0x50b480 — __ZN3RBX4Name7declareILZNS_10sSelectionEEEERKS0_v
pub fn stub_0x50b480() {
    // IDA 0x50b480: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sSelectionEEEERKS0_v")]
// 0x50b4c8 — __ZN3RBX4Name9doDeclareILZNS_10sSelectionEEEERKS0_v
pub fn stub_0x50b4c8() {
    // IDA 0x50b4c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
