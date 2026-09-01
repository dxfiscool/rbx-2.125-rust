//! core shard it — 100 core stubs EA-sorted, 0x2b7568..0x50b4c8 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 after 0x2b7568 prior 3463 remaining).
//! Source: `ida/export.json` filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sWorkspaceEEEERKS0_v")]
// 0x2b7568 — __ZN3RBX4Name7declareILZNS_10sWorkspaceEEEERKS0_v
pub fn stub_0x2b7568() -> ! {
    todo!("0x2b7568 __ZN3RBX4Name7declareILZNS_10sWorkspaceEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sWorkspaceEEEERKS0_v")]
// 0x2b75b0 — __ZN3RBX4Name9doDeclareILZNS_10sWorkspaceEEEERKS0_v
pub fn stub_0x2b75b0() -> ! {
    todo!("0x2b75b0 __ZN3RBX4Name9doDeclareILZNS_10sWorkspaceEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_10sCloneToolEEE7getNameEv")]
// 0x2dbf88 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_10sCloneToolEEE7getNameEv
pub fn stub_0x2dbf88() -> ! {
    todo!("0x2dbf88 __ZNK3RBX5NamedINS_12MouseCommandELZNS_10sCloneToolEEE7getNameEv")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGameToolEEE7getNameEv")]
// 0x2e33ec — __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGameToolEEE7getNameEv
pub fn stub_0x2e33ec() -> ! {
    todo!("0x2e33ec __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGameToolEEE7getNameEv")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGrabToolEEE7getNameEv")]
// 0x2e3c60 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGrabToolEEE7getNameEv
pub fn stub_0x2e3c60() -> ! {
    todo!("0x2e3c60 __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGrabToolEEE7getNameEv")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_11sHammerToolEEE7getNameEv")]
// 0x2e4b34 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_11sHammerToolEEE7getNameEv
pub fn stub_0x2e4b34() -> ! {
    todo!("0x2e4b34 __ZNK3RBX5NamedINS_12MouseCommandELZNS_11sHammerToolEEE7getNameEv")
}

#[doc(alias = "RBX::ClickDetector::fireMouseClick(float,RBX::Network::Player *)")]
// 0x3f1114 — __ZN3RBX13ClickDetector14fireMouseClickEfPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, float, RBX::Network::Player *)
pub fn stub_0x3f1114() -> ! {
    todo!("0x3f1114 __ZN3RBX13ClickDetector14fireMouseClickEfPNS_7Network6PlayerE")
}

#[doc(alias = "RBX::ClickDetector::fireMouseHover(RBX::Network::Player *)")]
// 0x3f130c — __ZN3RBX13ClickDetector14fireMouseHoverEPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
pub fn stub_0x3f130c() -> ! {
    todo!("0x3f130c __ZN3RBX13ClickDetector14fireMouseHoverEPNS_7Network6PlayerE")
}

#[doc(alias = "RBX::ClickDetector::fireMouseHoverLeave(RBX::Network::Player *)")]
// 0x3f1410 — __ZN3RBX13ClickDetector19fireMouseHoverLeaveEPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
pub fn stub_0x3f1410() -> ! {
    todo!("0x3f1410 __ZN3RBX13ClickDetector19fireMouseHoverLeaveEPNS_7Network6PlayerE")
}

#[doc(alias = "RBX::NetworkStatsCommand::doIt(RBX::IDataState *)")]
// 0x3f7f80 — __ZN3RBX19NetworkStatsCommand4doItEPNS_10IDataStateE
// type: void __fastcall(int, int, int, const void *)
pub fn stub_0x3f7f80() -> ! {
    todo!("0x3f7f80 __ZN3RBX19NetworkStatsCommand4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::NetworkStatsCommand::isEnabled(void)const")]
// 0x3f8268 — __ZNK3RBX19NetworkStatsCommand9isEnabledEv
// type: bool __fastcall(RBX::NetworkStatsCommand *this)
pub fn stub_0x3f8268() -> ! {
    todo!("0x3f8268 __ZNK3RBX19NetworkStatsCommand9isEnabledEv")
}

#[doc(alias = "RBX::NetworkStatsCommand::isChecked(void)const")]
// 0x3f83e4 — __ZNK3RBX19NetworkStatsCommand9isCheckedEv
// type: int __fastcall(RBX::NetworkStatsCommand *this)
pub fn stub_0x3f83e4() -> ! {
    todo!("0x3f83e4 __ZNK3RBX19NetworkStatsCommand9isCheckedEv")
}

#[doc(alias = "RBX::NetworkStatsCommand::~NetworkStatsCommand()")]
// 0x3fe628 — __ZN3RBX19NetworkStatsCommandD1Ev
// type: void __fastcall(RBX::NetworkStatsCommand *__hidden this)
pub fn stub_0x3fe628() -> ! {
    todo!("0x3fe628 __ZN3RBX19NetworkStatsCommandD1Ev")
}

#[doc(alias = "RBX::NetworkStatsCommand::~NetworkStatsCommand()")]
// 0x3fe62c — __ZN3RBX19NetworkStatsCommandD0Ev
// type: void __fastcall(RBX::NetworkStatsCommand *__hidden this)
pub fn stub_0x3fe62c() -> ! {
    todo!("0x3fe62c __ZN3RBX19NetworkStatsCommandD0Ev")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v")]
// 0x401d58 — __ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v
pub fn stub_0x401d58() -> ! {
    todo!("0x401d58 __ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v")]
// 0x401d9c — __ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v
// type: int()
pub fn stub_0x401d9c() -> ! {
    todo!("0x401d9c __ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v")
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev")]
// 0x48424c — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev
pub fn stub_0x48424c() -> ! {
    todo!("0x48424c __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv")]
// 0x4912ac — __ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv
pub fn stub_0x4912ac() -> ! {
    todo!("0x4912ac __ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v")]
// 0x4912b0 — __ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v
pub fn stub_0x4912b0() -> ! {
    todo!("0x4912b0 __ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sDecalEEEEvv")]
// 0x49148c — __ZN3RBX4Name13callDoDeclareILZNS_6sDecalEEEEvv
pub fn stub_0x49148c() -> ! {
    todo!("0x49148c __ZN3RBX4Name13callDoDeclareILZNS_6sDecalEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v")]
// 0x491490 — __ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v
pub fn stub_0x491490() -> ! {
    todo!("0x491490 __ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sDialogChoiceEEEEvv")]
// 0x494950 — __ZN3RBX4Name13callDoDeclareILZNS_13sDialogChoiceEEEEvv
pub fn stub_0x494950() -> ! {
    todo!("0x494950 __ZN3RBX4Name13callDoDeclareILZNS_13sDialogChoiceEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v")]
// 0x494954 — __ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v
pub fn stub_0x494954() -> ! {
    todo!("0x494954 __ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sDialogRootEEEEvv")]
// 0x49757c — __ZN3RBX4Name13callDoDeclareILZNS_11sDialogRootEEEEvv
pub fn stub_0x49757c() -> ! {
    todo!("0x49757c __ZN3RBX4Name13callDoDeclareILZNS_11sDialogRootEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v")]
// 0x497580 — __ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v
pub fn stub_0x497580() -> ! {
    todo!("0x497580 __ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sForceFieldEEEEvv")]
// 0x4a18d0 — __ZN3RBX4Name13callDoDeclareILZNS_11sForceFieldEEEEvv
pub fn stub_0x4a18d0() -> ! {
    todo!("0x4a18d0 __ZN3RBX4Name13callDoDeclareILZNS_11sForceFieldEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sExplosionEEEEvv")]
// 0x4a1d78 — __ZN3RBX4Name13callDoDeclareILZNS_10sExplosionEEEEvv
pub fn stub_0x4a1d78() -> ! {
    todo!("0x4a1d78 __ZN3RBX4Name13callDoDeclareILZNS_10sExplosionEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v")]
// 0x4a1d7c — __ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v
pub fn stub_0x4a1d7c() -> ! {
    todo!("0x4a1d7c __ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sExtrudedPartEEEEvv")]
// 0x4a8190 — __ZN3RBX4Name13callDoDeclareILZNS_13sExtrudedPartEEEEvv
pub fn stub_0x4a8190() -> ! {
    todo!("0x4a8190 __ZN3RBX4Name13callDoDeclareILZNS_13sExtrudedPartEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sExtrudedPartEEEERKS0_v")]
// 0x4a8194 — __ZN3RBX4Name9doDeclareILZNS_13sExtrudedPartEEEERKS0_v
pub fn stub_0x4a8194() -> ! {
    todo!("0x4a8194 __ZN3RBX4Name9doDeclareILZNS_13sExtrudedPartEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sBindableEventEEEERKS0_v")]
// 0x4ac23c — __ZN3RBX4Name7declareILZNS_14sBindableEventEEEERKS0_v
pub fn stub_0x4ac23c() -> ! {
    todo!("0x4ac23c __ZN3RBX4Name7declareILZNS_14sBindableEventEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sBindableEventEEEEvv")]
// 0x4ac280 — __ZN3RBX4Name13callDoDeclareILZNS_14sBindableEventEEEEvv
pub fn stub_0x4ac280() -> ! {
    todo!("0x4ac280 __ZN3RBX4Name13callDoDeclareILZNS_14sBindableEventEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sBindableEventEEEERKS0_v")]
// 0x4ac284 — __ZN3RBX4Name9doDeclareILZNS_14sBindableEventEEEERKS0_v
pub fn stub_0x4ac284() -> ! {
    todo!("0x4ac284 __ZN3RBX4Name9doDeclareILZNS_14sBindableEventEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_17sBindableFunctionEEEERKS0_v")]
// 0x4ada84 — __ZN3RBX4Name7declareILZNS_17sBindableFunctionEEEERKS0_v
pub fn stub_0x4ada84() -> ! {
    todo!("0x4ada84 __ZN3RBX4Name7declareILZNS_17sBindableFunctionEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sBindableFunctionEEEEvv")]
// 0x4adac8 — __ZN3RBX4Name13callDoDeclareILZNS_17sBindableFunctionEEEEvv
pub fn stub_0x4adac8() -> ! {
    todo!("0x4adac8 __ZN3RBX4Name13callDoDeclareILZNS_17sBindableFunctionEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sBindableFunctionEEEERKS0_v")]
// 0x4adacc — __ZN3RBX4Name9doDeclareILZNS_17sBindableFunctionEEEERKS0_v
// type: int()
pub fn stub_0x4adacc() -> ! {
    todo!("0x4adacc __ZN3RBX4Name9doDeclareILZNS_17sBindableFunctionEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sAnimationEEEERKS0_v")]
// 0x4ae3d0 — __ZN3RBX4Name7declareILZNS_10sAnimationEEEERKS0_v
pub fn stub_0x4ae3d0() -> ! {
    todo!("0x4ae3d0 __ZN3RBX4Name7declareILZNS_10sAnimationEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sAnimationEEEEvv")]
// 0x4ae414 — __ZN3RBX4Name13callDoDeclareILZNS_10sAnimationEEEEvv
// type: int()
pub fn stub_0x4ae414() -> ! {
    todo!("0x4ae414 __ZN3RBX4Name13callDoDeclareILZNS_10sAnimationEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sAnimationEEEERKS0_v")]
// 0x4ae418 — __ZN3RBX4Name9doDeclareILZNS_10sAnimationEEEERKS0_v
pub fn stub_0x4ae418() -> ! {
    todo!("0x4ae418 __ZN3RBX4Name9doDeclareILZNS_10sAnimationEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sSparklesEEEERKS0_v")]
// 0x4af45c — __ZN3RBX4Name7declareILZNS_9sSparklesEEEERKS0_v
pub fn stub_0x4af45c() -> ! {
    todo!("0x4af45c __ZN3RBX4Name7declareILZNS_9sSparklesEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sSparklesEEEEvv")]
// 0x4af4a0 — __ZN3RBX4Name13callDoDeclareILZNS_9sSparklesEEEEvv
pub fn stub_0x4af4a0() -> ! {
    todo!("0x4af4a0 __ZN3RBX4Name13callDoDeclareILZNS_9sSparklesEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sSparklesEEEERKS0_v")]
// 0x4af4a4 — __ZN3RBX4Name9doDeclareILZNS_9sSparklesEEEERKS0_v
pub fn stub_0x4af4a4() -> ! {
    todo!("0x4af4a4 __ZN3RBX4Name9doDeclareILZNS_9sSparklesEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sBasicPartEEEERKS0_v")]
// 0x4afdac — __ZN3RBX4Name7declareILZNS_10sBasicPartEEEERKS0_v
pub fn stub_0x4afdac() -> ! {
    todo!("0x4afdac __ZN3RBX4Name7declareILZNS_10sBasicPartEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sBasicPartEEEEvv")]
// 0x4afdf0 — __ZN3RBX4Name13callDoDeclareILZNS_10sBasicPartEEEEvv
pub fn stub_0x4afdf0() -> ! {
    todo!("0x4afdf0 __ZN3RBX4Name13callDoDeclareILZNS_10sBasicPartEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBasicPartEEEERKS0_v")]
// 0x4afdf4 — __ZN3RBX4Name9doDeclareILZNS_10sBasicPartEEEERKS0_v
pub fn stub_0x4afdf4() -> ! {
    todo!("0x4afdf4 __ZN3RBX4Name9doDeclareILZNS_10sBasicPartEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sForceFieldEEEERKS0_v")]
// 0x4b0454 — __ZN3RBX4Name9doDeclareILZNS_11sForceFieldEEEERKS0_v
pub fn stub_0x4b0454() -> ! {
    todo!("0x4b0454 __ZN3RBX4Name9doDeclareILZNS_11sForceFieldEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sCustomEventEEEERKS0_v")]
// 0x4b1c80 — __ZN3RBX4Name7declareILZNS_12sCustomEventEEEERKS0_v
pub fn stub_0x4b1c80() -> ! {
    todo!("0x4b1c80 __ZN3RBX4Name7declareILZNS_12sCustomEventEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sCustomEventEEEEvv")]
// 0x4b1cc4 — __ZN3RBX4Name13callDoDeclareILZNS_12sCustomEventEEEEvv
pub fn stub_0x4b1cc4() -> ! {
    todo!("0x4b1cc4 __ZN3RBX4Name13callDoDeclareILZNS_12sCustomEventEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sCustomEventEEEERKS0_v")]
// 0x4b1cc8 — __ZN3RBX4Name9doDeclareILZNS_12sCustomEventEEEERKS0_v
// type: int()
pub fn stub_0x4b1cc8() -> ! {
    todo!("0x4b1cc8 __ZN3RBX4Name9doDeclareILZNS_12sCustomEventEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_20sCustomEventReceiverEEEERKS0_v")]
// 0x4b3964 — __ZN3RBX4Name7declareILZNS_20sCustomEventReceiverEEEERKS0_v
pub fn stub_0x4b3964() -> ! {
    todo!("0x4b3964 __ZN3RBX4Name7declareILZNS_20sCustomEventReceiverEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sCustomEventReceiverEEEEvv")]
// 0x4b39a8 — __ZN3RBX4Name13callDoDeclareILZNS_20sCustomEventReceiverEEEEvv
pub fn stub_0x4b39a8() -> ! {
    todo!("0x4b39a8 __ZN3RBX4Name13callDoDeclareILZNS_20sCustomEventReceiverEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sCustomEventReceiverEEEERKS0_v")]
// 0x4b39ac — __ZN3RBX4Name9doDeclareILZNS_20sCustomEventReceiverEEEERKS0_v
pub fn stub_0x4b39ac() -> ! {
    todo!("0x4b39ac __ZN3RBX4Name9doDeclareILZNS_20sCustomEventReceiverEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sVelocityMotorEEEEvv")]
// 0x4e7874 — __ZN3RBX4Name13callDoDeclareILZNS_14sVelocityMotorEEEEvv
pub fn stub_0x4e7874() -> ! {
    todo!("0x4e7874 __ZN3RBX4Name13callDoDeclareILZNS_14sVelocityMotorEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sVelocityMotorEEEERKS0_v")]
// 0x4e7878 — __ZN3RBX4Name9doDeclareILZNS_14sVelocityMotorEEEERKS0_v
pub fn stub_0x4e7878() -> ! {
    todo!("0x4e7878 __ZN3RBX4Name9doDeclareILZNS_14sVelocityMotorEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sMotorFeatureEEEEvv")]
// 0x4e8224 — __ZN3RBX4Name13callDoDeclareILZNS_13sMotorFeatureEEEEvv
pub fn stub_0x4e8224() -> ! {
    todo!("0x4e8224 __ZN3RBX4Name13callDoDeclareILZNS_13sMotorFeatureEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sMotorFeatureEEEERKS0_v")]
// 0x4e8228 — __ZN3RBX4Name9doDeclareILZNS_13sMotorFeatureEEEERKS0_v
pub fn stub_0x4e8228() -> ! {
    todo!("0x4e8228 __ZN3RBX4Name9doDeclareILZNS_13sMotorFeatureEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sHoleEEEEvv")]
// 0x4e8bd4 — __ZN3RBX4Name13callDoDeclareILZNS_5sHoleEEEEvv
pub fn stub_0x4e8bd4() -> ! {
    todo!("0x4e8bd4 __ZN3RBX4Name13callDoDeclareILZNS_5sHoleEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sHoleEEEERKS0_v")]
// 0x4e8bd8 — __ZN3RBX4Name9doDeclareILZNS_5sHoleEEEERKS0_v
pub fn stub_0x4e8bd8() -> ! {
    todo!("0x4e8bd8 __ZN3RBX4Name9doDeclareILZNS_5sHoleEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sFeatureEEEEvv")]
// 0x4e8f70 — __ZN3RBX4Name13callDoDeclareILZNS_8sFeatureEEEEvv
pub fn stub_0x4e8f70() -> ! {
    todo!("0x4e8f70 __ZN3RBX4Name13callDoDeclareILZNS_8sFeatureEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sFeatureEEEERKS0_v")]
// 0x4e8f74 — __ZN3RBX4Name9doDeclareILZNS_8sFeatureEEEERKS0_v
pub fn stub_0x4e8f74() -> ! {
    todo!("0x4e8f74 __ZN3RBX4Name9doDeclareILZNS_8sFeatureEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sFireEEEEvv")]
// 0x4f03b0 — __ZN3RBX4Name13callDoDeclareILZNS_5sFireEEEEvv
pub fn stub_0x4f03b0() -> ! {
    todo!("0x4f03b0 __ZN3RBX4Name13callDoDeclareILZNS_5sFireEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sFireEEEERKS0_v")]
// 0x4f03b4 — __ZN3RBX4Name9doDeclareILZNS_5sFireEEEERKS0_v
pub fn stub_0x4f03b4() -> ! {
    todo!("0x4f03b4 __ZN3RBX4Name9doDeclareILZNS_5sFireEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sFlagEEEEvv")]
// 0x4f266c — __ZN3RBX4Name13callDoDeclareILZNS_5sFlagEEEEvv
pub fn stub_0x4f266c() -> ! {
    todo!("0x4f266c __ZN3RBX4Name13callDoDeclareILZNS_5sFlagEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sFlagEEEERKS0_v")]
// 0x4f2670 — __ZN3RBX4Name9doDeclareILZNS_5sFlagEEEERKS0_v
pub fn stub_0x4f2670() -> ! {
    todo!("0x4f2670 __ZN3RBX4Name9doDeclareILZNS_5sFlagEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sFlagStandEEEEvv")]
// 0x4f58d4 — __ZN3RBX4Name13callDoDeclareILZNS_10sFlagStandEEEEvv
pub fn stub_0x4f58d4() -> ! {
    todo!("0x4f58d4 __ZN3RBX4Name13callDoDeclareILZNS_10sFlagStandEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sFlagStandEEEERKS0_v")]
// 0x4f58d8 — __ZN3RBX4Name9doDeclareILZNS_10sFlagStandEEEERKS0_v
pub fn stub_0x4f58d8() -> ! {
    todo!("0x4f58d8 __ZN3RBX4Name9doDeclareILZNS_10sFlagStandEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sFlagStandServiceEEEEvv")]
// 0x4f5c70 — __ZN3RBX4Name13callDoDeclareILZNS_17sFlagStandServiceEEEEvv
pub fn stub_0x4f5c70() -> ! {
    todo!("0x4f5c70 __ZN3RBX4Name13callDoDeclareILZNS_17sFlagStandServiceEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sFlagStandServiceEEEERKS0_v")]
// 0x4f5c74 — __ZN3RBX4Name9doDeclareILZNS_17sFlagStandServiceEEEERKS0_v
pub fn stub_0x4f5c74() -> ! {
    todo!("0x4f5c74 __ZN3RBX4Name9doDeclareILZNS_17sFlagStandServiceEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEE9singletonEv")]
// 0x4fc998 — __ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
pub fn stub_0x4fc998() -> ! {
    todo!("0x4fc998 __ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEE9singletonEv")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sGameSettingsEEEERKS0_v")]
// 0x4ff25c — __ZN3RBX4Name7declareILZNS_13sGameSettingsEEEERKS0_v
pub fn stub_0x4ff25c() -> ! {
    todo!("0x4ff25c __ZN3RBX4Name7declareILZNS_13sGameSettingsEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sGameSettingsEEEEvv")]
// 0x4ff2a0 — __ZN3RBX4Name13callDoDeclareILZNS_13sGameSettingsEEEEvv
pub fn stub_0x4ff2a0() -> ! {
    todo!("0x4ff2a0 __ZN3RBX4Name13callDoDeclareILZNS_13sGameSettingsEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sGameSettingsEEEERKS0_v")]
// 0x4ff2a4 — __ZN3RBX4Name9doDeclareILZNS_13sGameSettingsEEEERKS0_v
pub fn stub_0x4ff2a4() -> ! {
    todo!("0x4ff2a4 __ZN3RBX4Name9doDeclareILZNS_13sGameSettingsEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEEC2Ev")]
// 0x501608 — __ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEEC2Ev
pub fn stub_0x501608() -> ! {
    todo!("0x501608 __ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEEC2Ev")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev")]
// 0x502174 — __ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev
pub fn stub_0x502174() -> ! {
    todo!("0x502174 __ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")]
// 0x5021b4 — __ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev
pub fn stub_0x5021b4() -> ! {
    todo!("0x5021b4 __ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev")]
// 0x502294 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev
pub fn stub_0x502294() -> ! {
    todo!("0x502294 __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")]
// 0x5022d8 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev
pub fn stub_0x5022d8() -> ! {
    todo!("0x5022d8 __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev")]
// 0x5022e0 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev
pub fn stub_0x5022e0() -> ! {
    todo!("0x5022e0 __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")]
// 0x502324 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev
pub fn stub_0x502324() -> ! {
    todo!("0x502324 __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sGeometryServiceEEEERKS0_v")]
// 0x506440 — __ZN3RBX4Name9doDeclareILZNS_16sGeometryServiceEEEERKS0_v
pub fn stub_0x506440() -> ! {
    todo!("0x506440 __ZN3RBX4Name9doDeclareILZNS_16sGeometryServiceEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv")]
// 0x5092dc — __ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv
pub fn stub_0x5092dc() -> ! {
    todo!("0x5092dc __ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv")]
// 0x509418 — __ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv
pub fn stub_0x509418() -> ! {
    todo!("0x509418 __ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv")]
// 0x509554 — __ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv
pub fn stub_0x509554() -> ! {
    todo!("0x509554 __ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv")]
// 0x50957c — __ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv
pub fn stub_0x50957c() -> ! {
    todo!("0x50957c __ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv")]
// 0x509850 — __ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv
pub fn stub_0x509850() -> ! {
    todo!("0x509850 __ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv")]
// 0x509b40 — __ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv
pub fn stub_0x509b40() -> ! {
    todo!("0x509b40 __ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sGlobalBasicSettingsEEEEvv")]
// 0x509e3c — __ZN3RBX4Name13callDoDeclareILZNS_20sGlobalBasicSettingsEEEEvv
pub fn stub_0x509e3c() -> ! {
    todo!("0x509e3c __ZN3RBX4Name13callDoDeclareILZNS_20sGlobalBasicSettingsEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sGlobalBasicSettingsEEEERKS0_v")]
// 0x509e40 — __ZN3RBX4Name9doDeclareILZNS_20sGlobalBasicSettingsEEEERKS0_v
pub fn stub_0x509e40() -> ! {
    todo!("0x509e40 __ZN3RBX4Name9doDeclareILZNS_20sGlobalBasicSettingsEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_23sGlobalAdvancedSettingsEEEEvv")]
// 0x509f20 — __ZN3RBX4Name13callDoDeclareILZNS_23sGlobalAdvancedSettingsEEEEvv
pub fn stub_0x509f20() -> ! {
    todo!("0x509f20 __ZN3RBX4Name13callDoDeclareILZNS_23sGlobalAdvancedSettingsEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_23sGlobalAdvancedSettingsEEEERKS0_v")]
// 0x509f24 — __ZN3RBX4Name9doDeclareILZNS_23sGlobalAdvancedSettingsEEEERKS0_v
pub fn stub_0x509f24() -> ! {
    todo!("0x509f24 __ZN3RBX4Name9doDeclareILZNS_23sGlobalAdvancedSettingsEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sSettingsEEEEvv")]
// 0x50a004 — __ZN3RBX4Name13callDoDeclareILZNS_9sSettingsEEEEvv
pub fn stub_0x50a004() -> ! {
    todo!("0x50a004 __ZN3RBX4Name13callDoDeclareILZNS_9sSettingsEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sSettingsEEEERKS0_v")]
// 0x50a008 — __ZN3RBX4Name9doDeclareILZNS_9sSettingsEEEERKS0_v
pub fn stub_0x50a008() -> ! {
    todo!("0x50a008 __ZN3RBX4Name9doDeclareILZNS_9sSettingsEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev")]
// 0x50ad20 — __ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev
pub fn stub_0x50ad20() -> ! {
    todo!("0x50ad20 __ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev")
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev")]
// 0x50ad5c — __ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev
pub fn stub_0x50ad5c() -> ! {
    todo!("0x50ad5c __ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev")]
// 0x50ae2c — __ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev
pub fn stub_0x50ae2c() -> ! {
    todo!("0x50ae2c __ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev")]
// 0x50ae68 — __ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev
pub fn stub_0x50ae68() -> ! {
    todo!("0x50ae68 __ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev")]
// 0x50af3c — __ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev
pub fn stub_0x50af3c() -> ! {
    todo!("0x50af3c __ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev")]
// 0x50af78 — __ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x50af78() -> ! {
    todo!("0x50af78 __ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sSelectionEEEERKS0_v")]
// 0x50b480 — __ZN3RBX4Name7declareILZNS_10sSelectionEEEERKS0_v
pub fn stub_0x50b480() -> ! {
    todo!("0x50b480 __ZN3RBX4Name7declareILZNS_10sSelectionEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sSelectionEEEERKS0_v")]
// 0x50b4c8 — __ZN3RBX4Name9doDeclareILZNS_10sSelectionEEEERKS0_v
pub fn stub_0x50b4c8() -> ! {
    todo!("0x50b4c8 __ZN3RBX4Name9doDeclareILZNS_10sSelectionEEEERKS0_v")
}
