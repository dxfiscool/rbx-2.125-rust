//! core shard ix — 120 core stubs EA-sorted, 0x74ae14..0x892534 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 120 after 0x74ae14 prior 3023 remaining).
//! Source: `ida/export.json` filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 120 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN3RBX9Primitive20setNetworkIsSleepingEb")]
// 0x74ae14 — __ZN3RBX9Primitive20setNetworkIsSleepingEb
// type: int __fastcall(int this, int)
pub fn stub_0x74ae14() -> ! {
    todo!("0x74ae14 __ZN3RBX9Primitive20setNetworkIsSleepingEb")
}

#[doc(alias = "__ZN3RBX23updateNetworkIsSleepingEPNS_8AssemblyE")]
// 0x75be00 — __ZN3RBX23updateNetworkIsSleepingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Assembly *)
pub fn stub_0x75be00() -> ! {
    todo!("0x75be00 __ZN3RBX23updateNetworkIsSleepingEPNS_8AssemblyE")
}

#[doc(alias = "__ZN3RBX10ChatOutput19onPlayerChatMessageERKNS_7Network11ChatMessageE")]
// 0x7a0ee4 — __ZN3RBX10ChatOutput19onPlayerChatMessageERKNS_7Network11ChatMessageE
// type: void __fastcall(RBX::ChatOutput *this, const RBX::Network::ChatMessage *)
pub fn stub_0x7a0ee4() -> ! {
    todo!("0x7a0ee4 __ZN3RBX10ChatOutput19onPlayerChatMessageERKNS_7Network11ChatMessageE")
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6insertEPNS8_4slotE")]
// 0x7a8b34 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x7a8b34() -> ! {
    todo!("0x7a8b34 __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6insertEPNS8_4slotE")
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE24safe_static_do_get_mutexEv")]
// 0x7a8d64 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE24safe_static_do_get_mutexEv
pub fn stub_0x7a8d64() -> ! {
    todo!("0x7a8d64 __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot10disconnectEv")]
// 0x7a8f5c — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot10disconnectEv
pub fn stub_0x7a8f5c() -> ! {
    todo!("0x7a8f5c __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot10disconnectEv")
}

#[doc(alias = "__ZNK3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot9connectedEv")]
// 0x7a906c — __ZNK3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot9connectedEv
pub fn stub_0x7a906c() -> ! {
    todo!("0x7a906c __ZNK3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot9connectedEv")
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6removeEPNS8_4slotE")]
// 0x7a90a0 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x7a90a0() -> ! {
    todo!("0x7a90a0 __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6removeEPNS8_4slotE")
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot22safe_static_init_mutexEv")]
// 0x7a9190 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot22safe_static_init_mutexEv
pub fn stub_0x7a9190() -> ! {
    todo!("0x7a9190 __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot24safe_static_do_get_mutexEv")]
// 0x7a9194 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x7a9194() -> ! {
    todo!("0x7a9194 __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD1Ev")]
// 0x7a9284 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD1Ev
pub fn stub_0x7a9284() -> ! {
    todo!("0x7a9284 __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD1Ev")
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD0Ev")]
// 0x7a92b0 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD0Ev
pub fn stub_0x7a92b0() -> ! {
    todo!("0x7a92b0 __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD0Ev")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sGuiItemEEEEvv")]
// 0x7af944 — __ZN3RBX4Name13callDoDeclareILZNS_8sGuiItemEEEEvv
pub fn stub_0x7af944() -> ! {
    todo!("0x7af944 __ZN3RBX4Name13callDoDeclareILZNS_8sGuiItemEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sGuiItemEEEERKS0_v")]
// 0x7af948 — __ZN3RBX4Name9doDeclareILZNS_8sGuiItemEEEERKS0_v
pub fn stub_0x7af948() -> ! {
    todo!("0x7af948 __ZN3RBX4Name9doDeclareILZNS_8sGuiItemEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_5sDeadEEE7getNameEv")]
// 0x7b42bc — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_5sDeadEEE7getNameEv
pub fn stub_0x7b42bc() -> ! {
    todo!("0x7b42bc __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_5sDeadEEE7getNameEv")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_12sFallingDownEEE7getNameEv")]
// 0x7b443c — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_12sFallingDownEEE7getNameEv
pub fn stub_0x7b443c() -> ! {
    todo!("0x7b443c __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_12sFallingDownEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN12sFallingDownEEEEvv")]
// 0x7b45bc — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN12sFallingDownEEEEvv
pub fn stub_0x7b45bc() -> ! {
    todo!("0x7b45bc __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN12sFallingDownEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN12sFallingDownEEEERKS0_v")]
// 0x7b45c0 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN12sFallingDownEEEERKS0_v
pub fn stub_0x7b45c0() -> ! {
    todo!("0x7b45c0 __ZN3RBX4Name9doDeclareILZNS_5HUMAN12sFallingDownEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN5sDeadEEEEvv")]
// 0x7b46a0 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN5sDeadEEEEvv
pub fn stub_0x7b46a0() -> ! {
    todo!("0x7b46a0 __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN5sDeadEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN5sDeadEEEERKS0_v")]
// 0x7b46a4 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN5sDeadEEEERKS0_v
pub fn stub_0x7b46a4() -> ! {
    todo!("0x7b46a4 __ZN3RBX4Name9doDeclareILZNS_5HUMAN5sDeadEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_7sFlyingEEE7getNameEv")]
// 0x7b4a50 — __ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_7sFlyingEEE7getNameEv
pub fn stub_0x7b4a50() -> ! {
    todo!("0x7b4a50 __ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_7sFlyingEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sFlyingEEEEvv")]
// 0x7b4bcc — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sFlyingEEEEvv
pub fn stub_0x7b4bcc() -> ! {
    todo!("0x7b4bcc __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sFlyingEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sFlyingEEEERKS0_v")]
// 0x7b4bd0 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sFlyingEEEERKS0_v
pub fn stub_0x7b4bd0() -> ! {
    todo!("0x7b4bd0 __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sFlyingEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_9sFreefallEEE7getNameEv")]
// 0x7b5814 — __ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_9sFreefallEEE7getNameEv
pub fn stub_0x7b5814() -> ! {
    todo!("0x7b5814 __ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_9sFreefallEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sFreefallEEEEvv")]
// 0x7b5848 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sFreefallEEEEvv
pub fn stub_0x7b5848() -> ! {
    todo!("0x7b5848 __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sFreefallEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN9sFreefallEEEERKS0_v")]
// 0x7b584c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sFreefallEEEERKS0_v
pub fn stub_0x7b584c() -> ! {
    todo!("0x7b584c __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sFreefallEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_10sGettingUpEEE7getNameEv")]
// 0x7b5c40 — __ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_10sGettingUpEEE7getNameEv
pub fn stub_0x7b5c40() -> ! {
    todo!("0x7b5c40 __ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_10sGettingUpEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN10sGettingUpEEEEvv")]
// 0x7b5dc4 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN10sGettingUpEEEEvv
pub fn stub_0x7b5dc4() -> ! {
    todo!("0x7b5dc4 __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN10sGettingUpEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN10sGettingUpEEEERKS0_v")]
// 0x7b5dc8 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN10sGettingUpEEEERKS0_v
pub fn stub_0x7b5dc8() -> ! {
    todo!("0x7b5dc8 __ZN3RBX4Name9doDeclareILZNS_5HUMAN10sGettingUpEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sHumanoidEEEEvv")]
// 0x7bfce8 — __ZN3RBX4Name13callDoDeclareILZNS_9sHumanoidEEEEvv
pub fn stub_0x7bfce8() -> ! {
    todo!("0x7bfce8 __ZN3RBX4Name13callDoDeclareILZNS_9sHumanoidEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sHumanoidEEEERKS0_v")]
// 0x7bfcec — __ZN3RBX4Name9doDeclareILZNS_9sHumanoidEEEERKS0_v
pub fn stub_0x7bfcec() -> ! {
    todo!("0x7bfcec __ZN3RBX4Name9doDeclareILZNS_9sHumanoidEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_9sClimbingEEE7getNameEv")]
// 0x7d1f38 — __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_9sClimbingEEE7getNameEv
pub fn stub_0x7d1f38() -> ! {
    todo!("0x7d1f38 __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_9sClimbingEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5HUMAN9sClimbingEEEERKS0_v")]
// 0x7d1f50 — __ZN3RBX4Name7declareILZNS_5HUMAN9sClimbingEEEERKS0_v
pub fn stub_0x7d1f50() -> ! {
    todo!("0x7d1f50 __ZN3RBX4Name7declareILZNS_5HUMAN9sClimbingEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sClimbingEEEEvv")]
// 0x7d1f94 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sClimbingEEEEvv
pub fn stub_0x7d1f94() -> ! {
    todo!("0x7d1f94 __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sClimbingEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN9sClimbingEEEERKS0_v")]
// 0x7d1f98 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sClimbingEEEERKS0_v
pub fn stub_0x7d1f98() -> ! {
    todo!("0x7d1f98 __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sClimbingEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN6FlyingELZNS1_8sJumpingEEE7getNameEv")]
// 0x7d2d78 — __ZNK3RBX5NamedINS_5HUMAN6FlyingELZNS1_8sJumpingEEE7getNameEv
pub fn stub_0x7d2d78() -> ! {
    todo!("0x7d2d78 __ZNK3RBX5NamedINS_5HUMAN6FlyingELZNS1_8sJumpingEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sJumpingEEEEvv")]
// 0x7d2f00 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sJumpingEEEEvv
pub fn stub_0x7d2f00() -> ! {
    todo!("0x7d2f00 __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sJumpingEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN8sJumpingEEEERKS0_v")]
// 0x7d2f04 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sJumpingEEEERKS0_v
pub fn stub_0x7d2f04() -> ! {
    todo!("0x7d2f04 __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sJumpingEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_20sMovingNoPhysicsBaseEEE7getNameEv")]
// 0x7d3efc — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_20sMovingNoPhysicsBaseEEE7getNameEv
pub fn stub_0x7d3efc() -> ! {
    todo!("0x7d3efc __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_20sMovingNoPhysicsBaseEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEEvv")]
// 0x7d3f28 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEEvv
pub fn stub_0x7d3f28() -> ! {
    todo!("0x7d3f28 __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEERKS0_v")]
// 0x7d3f2c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEERKS0_v
pub fn stub_0x7d3f2c() -> ! {
    todo!("0x7d3f2c __ZN3RBX4Name9doDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_8sRunningEEE7getNameEv")]
// 0x7d4810 — __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_8sRunningEEE7getNameEv
pub fn stub_0x7d4810() -> ! {
    todo!("0x7d4810 __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_8sRunningEEE7getNameEv")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN7RunningELZNS1_13sRunningSlaveEEE7getNameEv")]
// 0x7d498c — __ZNK3RBX5NamedINS_5HUMAN7RunningELZNS1_13sRunningSlaveEEE7getNameEv
pub fn stub_0x7d498c() -> ! {
    todo!("0x7d498c __ZNK3RBX5NamedINS_5HUMAN7RunningELZNS1_13sRunningSlaveEEE7getNameEv")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_7sLandedEEE7getNameEv")]
// 0x7d4b04 — __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_7sLandedEEE7getNameEv
pub fn stub_0x7d4b04() -> ! {
    todo!("0x7d4b04 __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_7sLandedEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sLandedEEEEvv")]
// 0x7d4c80 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sLandedEEEEvv
pub fn stub_0x7d4c80() -> ! {
    todo!("0x7d4c80 __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sLandedEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sLandedEEEERKS0_v")]
// 0x7d4c84 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sLandedEEEERKS0_v
pub fn stub_0x7d4c84() -> ! {
    todo!("0x7d4c84 __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sLandedEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN13sRunningSlaveEEEEvv")]
// 0x7d4d64 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN13sRunningSlaveEEEEvv
pub fn stub_0x7d4d64() -> ! {
    todo!("0x7d4d64 __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN13sRunningSlaveEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN13sRunningSlaveEEEERKS0_v")]
// 0x7d4d68 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN13sRunningSlaveEEEERKS0_v
pub fn stub_0x7d4d68() -> ! {
    todo!("0x7d4d68 __ZN3RBX4Name9doDeclareILZNS_5HUMAN13sRunningSlaveEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sRunningEEEEvv")]
// 0x7d4e48 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sRunningEEEEvv
pub fn stub_0x7d4e48() -> ! {
    todo!("0x7d4e48 __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sRunningEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRunningEEEERKS0_v")]
// 0x7d4e4c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRunningEEEERKS0_v
pub fn stub_0x7d4e4c() -> ! {
    todo!("0x7d4e4c __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRunningEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_17sRunningNoPhysicsEEE7getNameEv")]
// 0x7d6760 — __ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_17sRunningNoPhysicsEEE7getNameEv
pub fn stub_0x7d6760() -> ! {
    todo!("0x7d6760 __ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_17sRunningNoPhysicsEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEEvv")]
// 0x7d68e4 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEEvv
pub fn stub_0x7d68e4() -> ! {
    todo!("0x7d68e4 __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEERKS0_v")]
// 0x7d68e8 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEERKS0_v
pub fn stub_0x7d68e8() -> ! {
    todo!("0x7d68e8 __ZN3RBX4Name9doDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_7sSeatedEEE7getNameEv")]
// 0x7d7104 — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_7sSeatedEEE7getNameEv
pub fn stub_0x7d7104() -> ! {
    todo!("0x7d7104 __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_7sSeatedEEE7getNameEv")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_17sPlatformStandingEEE7getNameEv")]
// 0x7d713c — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_17sPlatformStandingEEE7getNameEv
pub fn stub_0x7d713c() -> ! {
    todo!("0x7d713c __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_17sPlatformStandingEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sPlatformStandingEEEEvv")]
// 0x7d7174 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sPlatformStandingEEEEvv
pub fn stub_0x7d7174() -> ! {
    todo!("0x7d7174 __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sPlatformStandingEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN17sPlatformStandingEEEERKS0_v")]
// 0x7d7178 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN17sPlatformStandingEEEERKS0_v
pub fn stub_0x7d7178() -> ! {
    todo!("0x7d7178 __ZN3RBX4Name9doDeclareILZNS_5HUMAN17sPlatformStandingEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sSeatedEEEEvv")]
// 0x7d7258 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sSeatedEEEEvv
pub fn stub_0x7d7258() -> ! {
    todo!("0x7d7258 __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sSeatedEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sSeatedEEEERKS0_v")]
// 0x7d725c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sSeatedEEEERKS0_v
pub fn stub_0x7d725c() -> ! {
    todo!("0x7d725c __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sSeatedEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_18sStrafingNoPhysicsEEE7getNameEv")]
// 0x7d75a0 — __ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_18sStrafingNoPhysicsEEE7getNameEv
pub fn stub_0x7d75a0() -> ! {
    todo!("0x7d75a0 __ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_18sStrafingNoPhysicsEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEEvv")]
// 0x7d771c — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEEvv
pub fn stub_0x7d771c() -> ! {
    todo!("0x7d771c __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEERKS0_v")]
// 0x7d7720 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEERKS0_v
pub fn stub_0x7d7720() -> ! {
    todo!("0x7d7720 __ZN3RBX4Name9doDeclareILZNS_5HUMAN18sStrafingNoPhysicsEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE12getClassNameEv")]
// 0x7e6b64 — __ZNK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE12getClassNameEv
pub fn stub_0x7e6b64() -> ! {
    todo!("0x7e6b64 __ZNK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE12getClassNameEv")]
// 0x7e6b9c — __ZThn32_NK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE12getClassNameEv
pub fn stub_0x7e6b9c() -> ! {
    todo!("0x7e6b9c __ZThn32_NK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE12getClassNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sMeshContentProviderEEEEvv")]
// 0x7e6be4 — __ZN3RBX4Name13callDoDeclareILZNS_20sMeshContentProviderEEEEvv
pub fn stub_0x7e6be4() -> ! {
    todo!("0x7e6be4 __ZN3RBX4Name13callDoDeclareILZNS_20sMeshContentProviderEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sMeshContentProviderEEEERKS0_v")]
// 0x7e6be8 — __ZN3RBX4Name9doDeclareILZNS_20sMeshContentProviderEEEERKS0_v
pub fn stub_0x7e6be8() -> ! {
    todo!("0x7e6be8 __ZN3RBX4Name9doDeclareILZNS_20sMeshContentProviderEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE12getClassNameEv")]
// 0x7e808c — __ZNK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE12getClassNameEv
pub fn stub_0x7e808c() -> ! {
    todo!("0x7e808c __ZNK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE12getClassNameEv")]
// 0x7e80c4 — __ZThn32_NK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE12getClassNameEv
pub fn stub_0x7e80c4() -> ! {
    todo!("0x7e80c4 __ZThn32_NK3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE12getClassNameEv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_23sTextureContentProviderEEEERKS0_v")]
// 0x7e810c — __ZN3RBX4Name9doDeclareILZNS_23sTextureContentProviderEEEERKS0_v
pub fn stub_0x7e810c() -> ! {
    todo!("0x7e810c __ZN3RBX4Name9doDeclareILZNS_23sTextureContentProviderEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v")]
// 0x7f10c8 — __ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v
pub fn stub_0x7f10c8() -> ! {
    todo!("0x7f10c8 __ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sFunctionalTestEEEEvv")]
// 0x8058c4 — __ZN3RBX4Name13callDoDeclareILZNS_15sFunctionalTestEEEEvv
// type: int()
pub fn stub_0x8058c4() -> ! {
    todo!("0x8058c4 __ZN3RBX4Name13callDoDeclareILZNS_15sFunctionalTestEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sFunctionalTestEEEERKS0_v")]
// 0x8058c8 — __ZN3RBX4Name9doDeclareILZNS_15sFunctionalTestEEEERKS0_v
pub fn stub_0x8058c8() -> ! {
    todo!("0x8058c8 __ZN3RBX4Name9doDeclareILZNS_15sFunctionalTestEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sNotificationBoxEEEEvv")]
// 0x83618c — __ZN3RBX4Name13callDoDeclareILZNS_16sNotificationBoxEEEEvv
pub fn stub_0x83618c() -> ! {
    todo!("0x83618c __ZN3RBX4Name13callDoDeclareILZNS_16sNotificationBoxEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sNotificationBoxEEEERKS0_v")]
// 0x836190 — __ZN3RBX4Name9doDeclareILZNS_16sNotificationBoxEEEERKS0_v
pub fn stub_0x836190() -> ! {
    todo!("0x836190 __ZN3RBX4Name9doDeclareILZNS_16sNotificationBoxEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sNotificationObjectEEEEvv")]
// 0x837a7c — __ZN3RBX4Name13callDoDeclareILZNS_19sNotificationObjectEEEEvv
pub fn stub_0x837a7c() -> ! {
    todo!("0x837a7c __ZN3RBX4Name13callDoDeclareILZNS_19sNotificationObjectEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sNotificationObjectEEEERKS0_v")]
// 0x837a80 — __ZN3RBX4Name9doDeclareILZNS_19sNotificationObjectEEEERKS0_v
pub fn stub_0x837a80() -> ! {
    todo!("0x837a80 __ZN3RBX4Name9doDeclareILZNS_19sNotificationObjectEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_6sFrameEEEERKS0_v")]
// 0x838064 — __ZN3RBX4Name7declareILZNS_6sFrameEEEERKS0_v
// type: int(void)
pub fn stub_0x838064() -> ! {
    todo!("0x838064 __ZN3RBX4Name7declareILZNS_6sFrameEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sFrameEEEEvv")]
// 0x8380a8 — __ZN3RBX4Name13callDoDeclareILZNS_6sFrameEEEEvv
pub fn stub_0x8380a8() -> ! {
    todo!("0x8380a8 __ZN3RBX4Name13callDoDeclareILZNS_6sFrameEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sFrameEEEERKS0_v")]
// 0x8380ac — __ZN3RBX4Name9doDeclareILZNS_6sFrameEEEERKS0_v
pub fn stub_0x8380ac() -> ! {
    todo!("0x8380ac __ZN3RBX4Name9doDeclareILZNS_6sFrameEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEEC2Ev")]
// 0x84a824 — __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEEC2Ev
pub fn stub_0x84a824() -> ! {
    todo!("0x84a824 __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEEC2Ev")
}

#[doc(alias = "__ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEE13resetSettingsEv")]
// 0x84acd0 — __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEE13resetSettingsEv
pub fn stub_0x84acd0() -> ! {
    todo!("0x84acd0 __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEE13resetSettingsEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_18sGameBasicSettingsEEEEvv")]
// 0x84b0a8 — __ZN3RBX4Name13callDoDeclareILZNS_18sGameBasicSettingsEEEEvv
pub fn stub_0x84b0a8() -> ! {
    todo!("0x84b0a8 __ZN3RBX4Name13callDoDeclareILZNS_18sGameBasicSettingsEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sGameBasicSettingsEEEERKS0_v")]
// 0x84b0ac — __ZN3RBX4Name9doDeclareILZNS_18sGameBasicSettingsEEEERKS0_v
pub fn stub_0x84b0ac() -> ! {
    todo!("0x84b0ac __ZN3RBX4Name9doDeclareILZNS_18sGameBasicSettingsEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev")]
// 0x84b760 — __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev
pub fn stub_0x84b760() -> ! {
    todo!("0x84b760 __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev")
}

#[doc(alias = "__ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev")]
// 0x84b7a0 — __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev
pub fn stub_0x84b7a0() -> ! {
    todo!("0x84b7a0 __ZN3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev")]
// 0x84b884 — __ZThn32_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev
pub fn stub_0x84b884() -> ! {
    todo!("0x84b884 __ZThn32_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev")]
// 0x84b8c8 — __ZThn32_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev
pub fn stub_0x84b8c8() -> ! {
    todo!("0x84b8c8 __ZThn32_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev")]
// 0x84b8d0 — __ZThn36_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev
pub fn stub_0x84b8d0() -> ! {
    todo!("0x84b8d0 __ZThn36_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev")]
// 0x84b914 — __ZThn36_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev
pub fn stub_0x84b914() -> ! {
    todo!("0x84b914 __ZThn36_N3RBX23GlobalBasicSettingsItemINS_17GameBasicSettingsELZNS_18sGameBasicSettingsEEED0Ev")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEE9singletonEv")]
// 0x8560dc — __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
pub fn stub_0x8560dc() -> ! {
    todo!("0x8560dc __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEE9singletonEv")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEEC2Ev")]
// 0x856758 — __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEEC2Ev
pub fn stub_0x856758() -> ! {
    todo!("0x856758 __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEEC2Ev")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev")]
// 0x856dc0 — __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev
pub fn stub_0x856dc0() -> ! {
    todo!("0x856dc0 __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev")]
// 0x856e00 — __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev
pub fn stub_0x856e00() -> ! {
    todo!("0x856e00 __ZN3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev")]
// 0x856ee0 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev
pub fn stub_0x856ee0() -> ! {
    todo!("0x856ee0 __ZThn32_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev")]
// 0x856f24 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev
pub fn stub_0x856f24() -> ! {
    todo!("0x856f24 __ZThn32_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev")]
// 0x856f2c — __ZThn36_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev
pub fn stub_0x856f2c() -> ! {
    todo!("0x856f2c __ZThn36_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev")]
// 0x856f70 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev
pub fn stub_0x856f70() -> ! {
    todo!("0x856f70 __ZThn36_N3RBX26GlobalAdvancedSettingsItemIN4FLog19FastLogSettingsItemELZNS_16sFastLogSettingsEEED0Ev")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sFastLogSettingsEEEEvv")]
// 0x8575cc — __ZN3RBX4Name13callDoDeclareILZNS_16sFastLogSettingsEEEEvv
pub fn stub_0x8575cc() -> ! {
    todo!("0x8575cc __ZN3RBX4Name13callDoDeclareILZNS_16sFastLogSettingsEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sFastLogSettingsEEEERKS0_v")]
// 0x8575d0 — __ZN3RBX4Name9doDeclareILZNS_16sFastLogSettingsEEEERKS0_v
pub fn stub_0x8575d0() -> ! {
    todo!("0x8575d0 __ZN3RBX4Name9doDeclareILZNS_16sFastLogSettingsEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sTextureTrailEEEEvv")]
// 0x85f604 — __ZN3RBX4Name13callDoDeclareILZNS_13sTextureTrailEEEEvv
pub fn stub_0x85f604() -> ! {
    todo!("0x85f604 __ZN3RBX4Name13callDoDeclareILZNS_13sTextureTrailEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sTextureTrailEEEERKS0_v")]
// 0x85f608 — __ZN3RBX4Name9doDeclareILZNS_13sTextureTrailEEEERKS0_v
pub fn stub_0x85f608() -> ! {
    todo!("0x85f608 __ZN3RBX4Name9doDeclareILZNS_13sTextureTrailEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sTeleportServiceEEEEvv")]
// 0x862efc — __ZN3RBX4Name13callDoDeclareILZNS_16sTeleportServiceEEEEvv
pub fn stub_0x862efc() -> ! {
    todo!("0x862efc __ZN3RBX4Name13callDoDeclareILZNS_16sTeleportServiceEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sFloorWireEEEEvv")]
// 0x8697e8 — __ZN3RBX4Name13callDoDeclareILZNS_10sFloorWireEEEEvv
pub fn stub_0x8697e8() -> ! {
    todo!("0x8697e8 __ZN3RBX4Name13callDoDeclareILZNS_10sFloorWireEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sFloorWireEEEERKS0_v")]
// 0x8697ec — __ZN3RBX4Name9doDeclareILZNS_10sFloorWireEEEERKS0_v
pub fn stub_0x8697ec() -> ! {
    todo!("0x8697ec __ZN3RBX4Name9doDeclareILZNS_10sFloorWireEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sMegaClusterEEEEvv")]
// 0x872a18 — __ZN3RBX4Name13callDoDeclareILZNS_12sMegaClusterEEEEvv
pub fn stub_0x872a18() -> ! {
    todo!("0x872a18 __ZN3RBX4Name13callDoDeclareILZNS_12sMegaClusterEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sMegaClusterEEEERKS0_v")]
// 0x872a1c — __ZN3RBX4Name9doDeclareILZNS_12sMegaClusterEEEERKS0_v
pub fn stub_0x872a1c() -> ! {
    todo!("0x872a1c __ZN3RBX4Name9doDeclareILZNS_12sMegaClusterEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sToolbarEEEEvv")]
// 0x888590 — __ZN3RBX4Name13callDoDeclareILZNS_8sToolbarEEEEvv
pub fn stub_0x888590() -> ! {
    todo!("0x888590 __ZN3RBX4Name13callDoDeclareILZNS_8sToolbarEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sToolbarEEEERKS0_v")]
// 0x888594 — __ZN3RBX4Name9doDeclareILZNS_8sToolbarEEEERKS0_v
pub fn stub_0x888594() -> ! {
    todo!("0x888594 __ZN3RBX4Name9doDeclareILZNS_8sToolbarEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sButtonEEEEvv")]
// 0x888770 — __ZN3RBX4Name13callDoDeclareILZNS_7sButtonEEEEvv
pub fn stub_0x888770() -> ! {
    todo!("0x888770 __ZN3RBX4Name13callDoDeclareILZNS_7sButtonEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sButtonEEEERKS0_v")]
// 0x888774 — __ZN3RBX4Name9doDeclareILZNS_7sButtonEEEERKS0_v
pub fn stub_0x888774() -> ! {
    todo!("0x888774 __ZN3RBX4Name9doDeclareILZNS_7sButtonEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sPluginEEEEvv")]
// 0x888abc — __ZN3RBX4Name13callDoDeclareILZNS_7sPluginEEEEvv
pub fn stub_0x888abc() -> ! {
    todo!("0x888abc __ZN3RBX4Name13callDoDeclareILZNS_7sPluginEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sPluginEEEERKS0_v")]
// 0x888ac0 — __ZN3RBX4Name9doDeclareILZNS_7sPluginEEEERKS0_v
pub fn stub_0x888ac0() -> ! {
    todo!("0x888ac0 __ZN3RBX4Name9doDeclareILZNS_7sPluginEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sPluginManagerEEEEvv")]
// 0x88927c — __ZN3RBX4Name13callDoDeclareILZNS_14sPluginManagerEEEEvv
pub fn stub_0x88927c() -> ! {
    todo!("0x88927c __ZN3RBX4Name13callDoDeclareILZNS_14sPluginManagerEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sPluginManagerEEEERKS0_v")]
// 0x889280 — __ZN3RBX4Name9doDeclareILZNS_14sPluginManagerEEEERKS0_v
pub fn stub_0x889280() -> ! {
    todo!("0x889280 __ZN3RBX4Name9doDeclareILZNS_14sPluginManagerEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv")]
// 0x88f4b8 — __ZNK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv
pub fn stub_0x88f4b8() -> ! {
    todo!("0x88f4b8 __ZNK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv")]
// 0x88f4e4 — __ZThn32_NK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv
pub fn stub_0x88f4e4() -> ! {
    todo!("0x88f4e4 __ZThn32_NK3RBX17NonFactoryProductINS_5MouseELZNS_12sPluginMouseEEE12getClassNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sPluginMouseEEEEvv")]
// 0x88f50c — __ZN3RBX4Name13callDoDeclareILZNS_12sPluginMouseEEEEvv
pub fn stub_0x88f50c() -> ! {
    todo!("0x88f50c __ZN3RBX4Name13callDoDeclareILZNS_12sPluginMouseEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sPluginMouseEEEERKS0_v")]
// 0x88f510 — __ZN3RBX4Name9doDeclareILZNS_12sPluginMouseEEEERKS0_v
pub fn stub_0x88f510() -> ! {
    todo!("0x88f510 __ZN3RBX4Name9doDeclareILZNS_12sPluginMouseEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX21PersonalServerService7getRankEPNS_7Network6PlayerEiN5boost8functionIFvSsEEES7_")]
// 0x8922e8 — __ZN3RBX21PersonalServerService7getRankEPNS_7Network6PlayerEiN5boost8functionIFvSsEEES7_
// type: void __fastcall(int, int, int, int, int)
pub fn stub_0x8922e8() -> ! {
    todo!("0x8922e8 __ZN3RBX21PersonalServerService7getRankEPNS_7Network6PlayerEiN5boost8functionIFvSsEEES7_")
}

#[doc(alias = "__ZN3RBX21PersonalServerService7setRankEPNS_7Network6PlayerEiiN5boost8functionIFvbEEENS5_IFvSsEEE")]
// 0x892534 — __ZN3RBX21PersonalServerService7setRankEPNS_7Network6PlayerEiiN5boost8functionIFvbEEENS5_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int)
pub fn stub_0x892534() -> ! {
    todo!("0x892534 __ZN3RBX21PersonalServerService7setRankEPNS_7Network6PlayerEiiN5boost8functionIFvbEEENS5_IFvSsEEE")
}

