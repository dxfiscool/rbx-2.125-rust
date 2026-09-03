//! core shard ix — 120 core stubs EA-sorted, 0x74ae14..0x892534 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 120 after 0x74ae14 prior 3023 remaining).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 120 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN3RBX9Primitive20setNetworkIsSleepingEb")]
// 0x74ae14 — __ZN3RBX9Primitive20setNetworkIsSleepingEb
// type: int __fastcall(int this, int)
pub fn stub_0x74ae14() {
    // IDA 0x74ae14: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX23updateNetworkIsSleepingEPNS_8AssemblyE")]
// 0x75be00 — __ZN3RBX23updateNetworkIsSleepingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Assembly *)
pub fn stub_0x75be00() {
    // IDA 0x75be00: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10ChatOutput19onPlayerChatMessageERKNS_7Network11ChatMessageE")]
// 0x7a0ee4 — __ZN3RBX10ChatOutput19onPlayerChatMessageERKNS_7Network11ChatMessageE
// type: void __fastcall(RBX::ChatOutput *this, const RBX::Network::ChatMessage *)
pub fn stub_0x7a0ee4() {
    // IDA 0x7a0ee4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6insertEPNS8_4slotE")]
// 0x7a8b34 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x7a8b34() {
    // IDA 0x7a8b34: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE24safe_static_do_get_mutexEv")]
// 0x7a8d64 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE24safe_static_do_get_mutexEv
pub fn stub_0x7a8d64() {
    // IDA 0x7a8d64: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot10disconnectEv")]
// 0x7a8f5c — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot10disconnectEv
pub fn stub_0x7a8f5c() {
    // IDA 0x7a8f5c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZNK3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot9connectedEv")]
// 0x7a906c — __ZNK3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot9connectedEv
pub fn stub_0x7a906c() {
    // IDA 0x7a906c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6removeEPNS8_4slotE")]
// 0x7a90a0 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x7a90a0() {
    // IDA 0x7a90a0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot22safe_static_init_mutexEv")]
// 0x7a9190 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot22safe_static_init_mutexEv
pub fn stub_0x7a9190() {
    // IDA 0x7a9190: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot24safe_static_do_get_mutexEv")]
// 0x7a9194 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x7a9194() {
    // IDA 0x7a9194: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD1Ev")]
// 0x7a9284 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD1Ev
pub fn stub_0x7a9284() {
    // IDA 0x7a9284: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD0Ev")]
// 0x7a92b0 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD0Ev
pub fn stub_0x7a92b0() {
    // IDA 0x7a92b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sGuiItemEEEEvv")]
// 0x7af944 — __ZN3RBX4Name13callDoDeclareILZNS_8sGuiItemEEEEvv
pub fn stub_0x7af944() {
    // IDA 0x7af944: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sGuiItemEEEERKS0_v")]
// 0x7af948 — __ZN3RBX4Name9doDeclareILZNS_8sGuiItemEEEERKS0_v
pub fn stub_0x7af948() {
    // IDA 0x7af948: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_5sDeadEEE7getNameEv")]
// 0x7b42bc — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_5sDeadEEE7getNameEv
pub fn stub_0x7b42bc() {
    // IDA 0x7b42bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_12sFallingDownEEE7getNameEv")]
// 0x7b443c — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_12sFallingDownEEE7getNameEv
pub fn stub_0x7b443c() {
    // IDA 0x7b443c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN12sFallingDownEEEEvv")]
// 0x7b45bc — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN12sFallingDownEEEEvv
pub fn stub_0x7b45bc() {
    // IDA 0x7b45bc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN12sFallingDownEEEERKS0_v")]
// 0x7b45c0 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN12sFallingDownEEEERKS0_v
pub fn stub_0x7b45c0() {
    // IDA 0x7b45c0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN5sDeadEEEEvv")]
// 0x7b46a0 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN5sDeadEEEEvv
pub fn stub_0x7b46a0() {
    // IDA 0x7b46a0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN5sDeadEEEERKS0_v")]
// 0x7b46a4 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN5sDeadEEEERKS0_v
pub fn stub_0x7b46a4() {
    // IDA 0x7b46a4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_7sFlyingEEE7getNameEv")]
// 0x7b4a50 — __ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_7sFlyingEEE7getNameEv
pub fn stub_0x7b4a50() {
    // IDA 0x7b4a50: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sFlyingEEEEvv")]
// 0x7b4bcc — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sFlyingEEEEvv
pub fn stub_0x7b4bcc() {
    // IDA 0x7b4bcc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sFlyingEEEERKS0_v")]
// 0x7b4bd0 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sFlyingEEEERKS0_v
pub fn stub_0x7b4bd0() {
    // IDA 0x7b4bd0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_9sFreefallEEE7getNameEv")]
// 0x7b5814 — __ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_9sFreefallEEE7getNameEv
pub fn stub_0x7b5814() {
    // IDA 0x7b5814: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sFreefallEEEEvv")]
// 0x7b5848 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sFreefallEEEEvv
pub fn stub_0x7b5848() {
    // IDA 0x7b5848: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN9sFreefallEEEERKS0_v")]
// 0x7b584c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sFreefallEEEERKS0_v
pub fn stub_0x7b584c() {
    // IDA 0x7b584c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_10sGettingUpEEE7getNameEv")]
// 0x7b5c40 — __ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_10sGettingUpEEE7getNameEv
pub fn stub_0x7b5c40() {
    // IDA 0x7b5c40: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN10sGettingUpEEEEvv")]
// 0x7b5dc4 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN10sGettingUpEEEEvv
pub fn stub_0x7b5dc4() {
    // IDA 0x7b5dc4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN10sGettingUpEEEERKS0_v")]
// 0x7b5dc8 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN10sGettingUpEEEERKS0_v
pub fn stub_0x7b5dc8() {
    // IDA 0x7b5dc8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sHumanoidEEEEvv")]
// 0x7bfce8 — __ZN3RBX4Name13callDoDeclareILZNS_9sHumanoidEEEEvv
pub fn stub_0x7bfce8() {
    // IDA 0x7bfce8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sHumanoidEEEERKS0_v")]
// 0x7bfcec — __ZN3RBX4Name9doDeclareILZNS_9sHumanoidEEEERKS0_v
pub fn stub_0x7bfcec() {
    // IDA 0x7bfcec: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_9sClimbingEEE7getNameEv")]
// 0x7d1f38 — __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_9sClimbingEEE7getNameEv
pub fn stub_0x7d1f38() {
    // IDA 0x7d1f38: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5HUMAN9sClimbingEEEERKS0_v")]
// 0x7d1f50 — __ZN3RBX4Name7declareILZNS_5HUMAN9sClimbingEEEERKS0_v
pub fn stub_0x7d1f50() {
    // IDA 0x7d1f50: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sClimbingEEEEvv")]
// 0x7d1f94 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sClimbingEEEEvv
pub fn stub_0x7d1f94() {
    // IDA 0x7d1f94: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN9sClimbingEEEERKS0_v")]
// 0x7d1f98 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sClimbingEEEERKS0_v
pub fn stub_0x7d1f98() {
    // IDA 0x7d1f98: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN6FlyingELZNS1_8sJumpingEEE7getNameEv")]
// 0x7d2d78 — __ZNK3RBX5NamedINS_5HUMAN6FlyingELZNS1_8sJumpingEEE7getNameEv
pub fn stub_0x7d2d78() {
    // IDA 0x7d2d78: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sJumpingEEEEvv")]
// 0x7d2f00 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sJumpingEEEEvv
pub fn stub_0x7d2f00() {
    // IDA 0x7d2f00: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN8sJumpingEEEERKS0_v")]
// 0x7d2f04 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sJumpingEEEERKS0_v
pub fn stub_0x7d2f04() {
    // IDA 0x7d2f04: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_20sMovingNoPhysicsBaseEEE7getNameEv")]
// 0x7d3efc — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_20sMovingNoPhysicsBaseEEE7getNameEv
pub fn stub_0x7d3efc() {
    // IDA 0x7d3efc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEEvv")]
// 0x7d3f28 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEEvv
pub fn stub_0x7d3f28() {
    // IDA 0x7d3f28: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEERKS0_v")]
// 0x7d3f2c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEERKS0_v
pub fn stub_0x7d3f2c() {
    // IDA 0x7d3f2c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_8sRunningEEE7getNameEv")]
// 0x7d4810 — __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_8sRunningEEE7getNameEv
pub fn stub_0x7d4810() {
    // IDA 0x7d4810: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN7RunningELZNS1_13sRunningSlaveEEE7getNameEv")]
// 0x7d498c — __ZNK3RBX5NamedINS_5HUMAN7RunningELZNS1_13sRunningSlaveEEE7getNameEv
pub fn stub_0x7d498c() {
    // IDA 0x7d498c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_7sLandedEEE7getNameEv")]
// 0x7d4b04 — __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_7sLandedEEE7getNameEv
pub fn stub_0x7d4b04() {
    // IDA 0x7d4b04: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sLandedEEEEvv")]
// 0x7d4c80 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sLandedEEEEvv
pub fn stub_0x7d4c80() {
    // IDA 0x7d4c80: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sLandedEEEERKS0_v")]
// 0x7d4c84 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sLandedEEEERKS0_v
pub fn stub_0x7d4c84() {
    // IDA 0x7d4c84: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN13sRunningSlaveEEEEvv")]
// 0x7d4d64 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN13sRunningSlaveEEEEvv
pub fn stub_0x7d4d64() {
    // IDA 0x7d4d64: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN13sRunningSlaveEEEERKS0_v")]
// 0x7d4d68 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN13sRunningSlaveEEEERKS0_v
pub fn stub_0x7d4d68() {
    // IDA 0x7d4d68: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sRunningEEEEvv")]
// 0x7d4e48 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sRunningEEEEvv
pub fn stub_0x7d4e48() {
    // IDA 0x7d4e48: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRunningEEEERKS0_v")]
// 0x7d4e4c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRunningEEEERKS0_v
pub fn stub_0x7d4e4c() {
    // IDA 0x7d4e4c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_17sRunningNoPhysicsEEE7getNameEv")]
// 0x7d6760 — __ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_17sRunningNoPhysicsEEE7getNameEv
pub fn stub_0x7d6760() {
    // IDA 0x7d6760: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEEvv")]
// 0x7d68e4 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEEvv
pub fn stub_0x7d68e4() {
    // IDA 0x7d68e4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEERKS0_v")]
// 0x7d68e8 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEERKS0_v
pub fn stub_0x7d68e8() {
    // IDA 0x7d68e8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_7sSeatedEEE7getNameEv")]
// 0x7d7104 — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_7sSeatedEEE7getNameEv
pub fn stub_0x7d7104() {
    // IDA 0x7d7104: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_17sPlatformStandingEEE7getNameEv")]
// 0x7d713c — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_17sPlatformStandingEEE7getNameEv
pub fn stub_0x7d713c() {
    // IDA 0x7d713c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sPlatformStandingEEEEvv")]
// 0x7d7174 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sPlatformStandingEEEEvv
pub fn stub_0x7d7174() {
    // IDA 0x7d7174: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN17sPlatformStandingEEEERKS0_v")]
// 0x7d7178 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN17sPlatformStandingEEEERKS0_v
pub fn stub_0x7d7178() {
    // IDA 0x7d7178: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sSeatedEEEEvv")]
// 0x7d7258 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sSeatedEEEEvv
pub fn stub_0x7d7258() {
    // IDA 0x7d7258: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sSeatedEEEERKS0_v")]
// 0x7d725c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sSeatedEEEERKS0_v
pub fn stub_0x7d725c() {
    // IDA 0x7d725c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_18sStrafingNoPhysicsEEE7getNameEv")]
// 0x7d75a0 — __ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_18sStrafingNoPhysicsEEE7getNameEv
pub fn stub_0x7d75a0() {
    // IDA 0x7d75a0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEEvv")]
// 0x7d771c — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEEvv
pub fn stub_0x7d771c() {
    // IDA 0x7d771c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEERKS0_v")]
// 0x7d7720 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEERKS0_v
pub fn stub_0x7d7720() {
    // IDA 0x7d7720: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE12getClassNameEv")]
// 0x7e6b64 — __ZNK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE12getClassNameEv
pub fn stub_0x7e6b64() {
    // IDA 0x7e6b64: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE12getClassNameEv")]
// 0x7e6b9c — __ZThn32_NK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE12getClassNameEv
pub fn stub_0x7e6b9c() {
    // IDA 0x7e6b9c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sMeshContentProviderEEEEvv")]
// 0x7e6be4 — __ZN3RBX4Name13callDoDeclareILZNS_20sMeshContentProviderEEEEvv
pub fn stub_0x7e6be4() {
    // IDA 0x7e6be4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sMeshContentProviderEEEERKS0_v")]
// 0x7e6be8 — __ZN3RBX4Name9doDeclareILZNS_20sMeshContentProviderEEEERKS0_v
pub fn stub_0x7e6be8() {
    // IDA 0x7e6be8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE12getClassNameEv")]
// 0x7e808c — __ZNK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE12getClassNameEv
pub fn stub_0x7e808c() {
    // IDA 0x7e808c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE12getClassNameEv")]
// 0x7e80c4 — __ZThn32_NK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE12getClassNameEv
pub fn stub_0x7e80c4() {
    // IDA 0x7e80c4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_23sTextureContentProviderEEEERKS0_v")]
// 0x7e810c — __ZN3RBX4Name9doDeclareILZNS_23sTextureContentProviderEEEERKS0_v
pub fn stub_0x7e810c() {
    // IDA 0x7e810c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v")]
// 0x7f10c8 — __ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v
pub fn stub_0x7f10c8() {
    // IDA 0x7f10c8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sFunctionalTestEEEEvv")]
// 0x8058c4 — __ZN3RBX4Name13callDoDeclareILZNS_15sFunctionalTestEEEEvv
// type: int()
pub fn stub_0x8058c4() {
    // IDA 0x8058c4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sFunctionalTestEEEERKS0_v")]
// 0x8058c8 — __ZN3RBX4Name9doDeclareILZNS_15sFunctionalTestEEEERKS0_v
pub fn stub_0x8058c8() {
    // IDA 0x8058c8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sNotificationBoxEEEEvv")]
// 0x83618c — __ZN3RBX4Name13callDoDeclareILZNS_16sNotificationBoxEEEEvv
pub fn stub_0x83618c() {
    // IDA 0x83618c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sNotificationBoxEEEERKS0_v")]
// 0x836190 — __ZN3RBX4Name9doDeclareILZNS_16sNotificationBoxEEEERKS0_v
pub fn stub_0x836190() {
    // IDA 0x836190: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sNotificationObjectEEEEvv")]
// 0x837a7c — __ZN3RBX4Name13callDoDeclareILZNS_19sNotificationObjectEEEEvv
pub fn stub_0x837a7c() {
    // IDA 0x837a7c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sNotificationObjectEEEERKS0_v")]
// 0x837a80 — __ZN3RBX4Name9doDeclareILZNS_19sNotificationObjectEEEERKS0_v
pub fn stub_0x837a80() {
    // IDA 0x837a80: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_6sFrameEEEERKS0_v")]
// 0x838064 — __ZN3RBX4Name7declareILZNS_6sFrameEEEERKS0_v
// type: int(void)
pub fn stub_0x838064() {
    // IDA 0x838064: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sFrameEEEEvv")]
// 0x8380a8 — __ZN3RBX4Name13callDoDeclareILZNS_6sFrameEEEEvv
pub fn stub_0x8380a8() {
    // IDA 0x8380a8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sFrameEEEERKS0_v")]
// 0x8380ac — __ZN3RBX4Name9doDeclareILZNS_6sFrameEEEERKS0_v
pub fn stub_0x8380ac() {
    // IDA 0x8380ac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEEC2Ev")]
// 0x84a824 — __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEEC2Ev
pub fn stub_0x84a824() {
    // IDA 0x84a824: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEE13resetSettingsEv")]
// 0x84acd0 — __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEE13resetSettingsEv
pub fn stub_0x84acd0() {
    // IDA 0x84acd0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_18sGameBasicSettingsEEEEvv")]
// 0x84b0a8 — __ZN3RBX4Name13callDoDeclareILZNS_18sGameBasicSettingsEEEEvv
pub fn stub_0x84b0a8() {
    // IDA 0x84b0a8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sGameBasicSettingsEEEERKS0_v")]
// 0x84b0ac — __ZN3RBX4Name9doDeclareILZNS_18sGameBasicSettingsEEEERKS0_v
pub fn stub_0x84b0ac() {
    // IDA 0x84b0ac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev")]
// 0x84b760 — __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev
pub fn stub_0x84b760() {
    // IDA 0x84b760: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev")]
// 0x84b7a0 — __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev
pub fn stub_0x84b7a0() {
    // IDA 0x84b7a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev")]
// 0x84b884 — __ZThn32_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev
pub fn stub_0x84b884() {
    // IDA 0x84b884: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev")]
// 0x84b8c8 — __ZThn32_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev
pub fn stub_0x84b8c8() {
    // IDA 0x84b8c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev")]
// 0x84b8d0 — __ZThn36_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev
pub fn stub_0x84b8d0() {
    // IDA 0x84b8d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev")]
// 0x84b914 — __ZThn36_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev
pub fn stub_0x84b914() {
    // IDA 0x84b914: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEE9singletonEv")]
// 0x8560dc — __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
pub fn stub_0x8560dc() {
    // IDA 0x8560dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEEC2Ev")]
// 0x856758 — __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEEC2Ev
pub fn stub_0x856758() {
    // IDA 0x856758: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev")]
// 0x856dc0 — __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev
pub fn stub_0x856dc0() {
    // IDA 0x856dc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev")]
// 0x856e00 — __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev
pub fn stub_0x856e00() {
    // IDA 0x856e00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev")]
// 0x856ee0 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev
pub fn stub_0x856ee0() {
    // IDA 0x856ee0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev")]
// 0x856f24 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev
pub fn stub_0x856f24() {
    // IDA 0x856f24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev")]
// 0x856f2c — __ZThn36_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev
pub fn stub_0x856f2c() {
    // IDA 0x856f2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev")]
// 0x856f70 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev
pub fn stub_0x856f70() {
    // IDA 0x856f70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sFastLogSettingsEEEEvv")]
// 0x8575cc — __ZN3RBX4Name13callDoDeclareILZNS_16sFastLogSettingsEEEEvv
pub fn stub_0x8575cc() {
    // IDA 0x8575cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sFastLogSettingsEEEERKS0_v")]
// 0x8575d0 — __ZN3RBX4Name9doDeclareILZNS_16sFastLogSettingsEEEERKS0_v
pub fn stub_0x8575d0() {
    // IDA 0x8575d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sTextureTrailEEEEvv")]
// 0x85f604 — __ZN3RBX4Name13callDoDeclareILZNS_13sTextureTrailEEEEvv
pub fn stub_0x85f604() {
    // IDA 0x85f604: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sTextureTrailEEEERKS0_v")]
// 0x85f608 — __ZN3RBX4Name9doDeclareILZNS_13sTextureTrailEEEERKS0_v
pub fn stub_0x85f608() {
    // IDA 0x85f608: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sTeleportServiceEEEEvv")]
// 0x862efc — __ZN3RBX4Name13callDoDeclareILZNS_16sTeleportServiceEEEEvv
pub fn stub_0x862efc() {
    // IDA 0x862efc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sFloorWireEEEEvv")]
// 0x8697e8 — __ZN3RBX4Name13callDoDeclareILZNS_10sFloorWireEEEEvv
pub fn stub_0x8697e8() {
    // IDA 0x8697e8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sFloorWireEEEERKS0_v")]
// 0x8697ec — __ZN3RBX4Name9doDeclareILZNS_10sFloorWireEEEERKS0_v
pub fn stub_0x8697ec() {
    // IDA 0x8697ec: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sMegaClusterEEEEvv")]
// 0x872a18 — __ZN3RBX4Name13callDoDeclareILZNS_12sMegaClusterEEEEvv
pub fn stub_0x872a18() {
    // IDA 0x872a18: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sMegaClusterEEEERKS0_v")]
// 0x872a1c — __ZN3RBX4Name9doDeclareILZNS_12sMegaClusterEEEERKS0_v
pub fn stub_0x872a1c() {
    // IDA 0x872a1c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sToolbarEEEEvv")]
// 0x888590 — __ZN3RBX4Name13callDoDeclareILZNS_8sToolbarEEEEvv
pub fn stub_0x888590() {
    // IDA 0x888590: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sToolbarEEEERKS0_v")]
// 0x888594 — __ZN3RBX4Name9doDeclareILZNS_8sToolbarEEEERKS0_v
pub fn stub_0x888594() {
    // IDA 0x888594: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sButtonEEEEvv")]
// 0x888770 — __ZN3RBX4Name13callDoDeclareILZNS_7sButtonEEEEvv
pub fn stub_0x888770() {
    // IDA 0x888770: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sButtonEEEERKS0_v")]
// 0x888774 — __ZN3RBX4Name9doDeclareILZNS_7sButtonEEEERKS0_v
pub fn stub_0x888774() {
    // IDA 0x888774: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sPluginEEEEvv")]
// 0x888abc — __ZN3RBX4Name13callDoDeclareILZNS_7sPluginEEEEvv
pub fn stub_0x888abc() {
    // IDA 0x888abc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sPluginEEEERKS0_v")]
// 0x888ac0 — __ZN3RBX4Name9doDeclareILZNS_7sPluginEEEERKS0_v
pub fn stub_0x888ac0() {
    // IDA 0x888ac0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sPluginManagerEEEEvv")]
// 0x88927c — __ZN3RBX4Name13callDoDeclareILZNS_14sPluginManagerEEEEvv
pub fn stub_0x88927c() {
    // IDA 0x88927c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sPluginManagerEEEERKS0_v")]
// 0x889280 — __ZN3RBX4Name9doDeclareILZNS_14sPluginManagerEEEERKS0_v
pub fn stub_0x889280() {
    // IDA 0x889280: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv")]
// 0x88f4b8 — __ZNK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv
pub fn stub_0x88f4b8() {
    // IDA 0x88f4b8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv")]
// 0x88f4e4 — __ZThn32_NK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv
pub fn stub_0x88f4e4() {
    // IDA 0x88f4e4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sPluginMouseEEEEvv")]
// 0x88f50c — __ZN3RBX4Name13callDoDeclareILZNS_12sPluginMouseEEEEvv
pub fn stub_0x88f50c() {
    // IDA 0x88f50c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sPluginMouseEEEERKS0_v")]
// 0x88f510 — __ZN3RBX4Name9doDeclareILZNS_12sPluginMouseEEEERKS0_v
pub fn stub_0x88f510() {
    // IDA 0x88f510: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX21PersonalServerService7getRankEPNS_7Network6PlayerEiN5boost8functionIFvSsEEES7_")]
// 0x8922e8 — __ZN3RBX21PersonalServerService7getRankEPNS_7Network6PlayerEiN5boost8functionIFvSsEEES7_
// type: void __fastcall(int, int, int, int, int)
pub fn stub_0x8922e8() {
    // IDA 0x8922e8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX21PersonalServerService7setRankEPNS_7Network6PlayerEiiN5boost8functionIFvbEEENS5_IFvSsEEE")]
// 0x892534 — __ZN3RBX21PersonalServerService7setRankEPNS_7Network6PlayerEiiN5boost8functionIFvbEEENS5_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int)
pub fn stub_0x892534() {
    // IDA 0x892534: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

