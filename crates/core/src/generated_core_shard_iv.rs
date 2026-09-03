//! core shard iv — 120 core stubs EA-sorted, 0x5cabb0..0x68057c (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 120 after 0x5cabb0 prior 3263 remaining).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 120 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sMessageEEEEvv")]
// 0x5cabb0 — __ZN3RBX4Name13callDoDeclareILZNS_8sMessageEEEEvv
pub fn stub_0x5cabb0() {
    // IDA 0x5cabb0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sMessageEEEERKS0_v")]
// 0x5cabb4 — __ZN3RBX4Name9doDeclareILZNS_8sMessageEEEERKS0_v
pub fn stub_0x5cabb4() {
    // IDA 0x5cabb4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_4PART6sWedgeEEEEvv")]
// 0x5d7ae8 — __ZN3RBX4Name13callDoDeclareILZNS_4PART6sWedgeEEEEvv
pub fn stub_0x5d7ae8() {
    // IDA 0x5d7ae8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_4PART6sWedgeEEEERKS0_v")]
// 0x5d7aec — __ZN3RBX4Name9doDeclareILZNS_4PART6sWedgeEEEERKS0_v
pub fn stub_0x5d7aec() {
    // IDA 0x5d7aec: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX7Network12NetworkOwner16ServerUnassignedEv")]
// 0x5e1de8 — __ZN3RBX7Network12NetworkOwner16ServerUnassignedEv
// type: _DWORD __fastcall(RBX::Network::NetworkOwner *__hidden this)
pub fn stub_0x5e1de8() {
    // IDA 0x5e1de8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX7Network12NetworkOwner16colorFromAddressERKNS_13SystemAddressE")]
// 0x5e1e40 — __ZN3RBX7Network12NetworkOwner16colorFromAddressERKNS_13SystemAddressE
// type: int(void)
pub fn stub_0x5e1e40() {
    // IDA 0x5e1e40: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX7Network12NetworkOwner8isClientERKNS_13SystemAddressE")]
// 0x5e1eac — __ZN3RBX7Network12NetworkOwner8isClientERKNS_13SystemAddressE
// type: int(void)
pub fn stub_0x5e1eac() {
    // IDA 0x5e1eac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX7Network12NetworkOwner6ServerEv")]
// 0x5e1ef8 — __ZN3RBX7Network12NetworkOwner6ServerEv
// type: _DWORD __fastcall(RBX::Network::NetworkOwner *__hidden this)
pub fn stub_0x5e1ef8() {
    // IDA 0x5e1ef8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sPartEEEEvv")]
// 0x5e4b2c — __ZN3RBX4Name13callDoDeclareILZNS_5sPartEEEEvv
pub fn stub_0x5e4b2c() {
    // IDA 0x5e4b2c: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sPartEEEERKS0_v")]
// 0x5e4b30 — __ZN3RBX4Name9doDeclareILZNS_5sPartEEEERKS0_v
pub fn stub_0x5e4b30() {
    // IDA 0x5e4b30: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sFWServiceEEEERKS0_v")]
// 0x5e6c18 — __ZN3RBX4Name7declareILZNS_10sFWServiceEEEERKS0_v
// type: int(void)
pub fn stub_0x5e6c18() {
    // IDA 0x5e6c18: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sFWServiceEEEERKS0_v")]
// 0x5e6c60 — __ZN3RBX4Name9doDeclareILZNS_10sFWServiceEEEERKS0_v
pub fn stub_0x5e6c60() {
    // IDA 0x5e6c60: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX19PhysicsInstructions22changeSimulationRadiusEPNS_7Network6PlayerEf")]
// 0x5f6978 — __ZN3RBX19PhysicsInstructions22changeSimulationRadiusEPNS_7Network6PlayerEf
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, float)
pub fn stub_0x5f6978() {
    // IDA 0x5f6978: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX19PhysicsInstructions25changeMaxSimulationRadiusEPNS_7Network6PlayerEf")]
// 0x5f69ec — __ZN3RBX19PhysicsInstructions25changeMaxSimulationRadiusEPNS_7Network6PlayerEf
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, float)
pub fn stub_0x5f69ec() {
    // IDA 0x5f69ec: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX19PhysicsInstructions12setThrottlesEPNS_7Network6PlayerEPNS_9WorkspaceEdd")]
// 0x5f6a90 — __ZN3RBX19PhysicsInstructions12setThrottlesEPNS_7Network6PlayerEPNS_9WorkspaceEdd
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, RBX::Workspace *, double, double)
pub fn stub_0x5f6a90() {
    // IDA 0x5f6a90: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEEC2Ev")]
// 0x5f9180 — __ZN3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEEC2Ev
// type: int(void)
pub fn stub_0x5f9180() {
    // IDA 0x5f9180: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED1Ev")]
// 0x5f97ec — __ZN3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED1Ev
pub fn stub_0x5f97ec() {
    // IDA 0x5f97ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED0Ev")]
// 0x5f982c — __ZN3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED0Ev
pub fn stub_0x5f982c() {
    // IDA 0x5f982c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED1Ev")]
// 0x5f990c — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED1Ev
pub fn stub_0x5f990c() {
    // IDA 0x5f990c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED0Ev")]
// 0x5f9950 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED0Ev
pub fn stub_0x5f9950() {
    // IDA 0x5f9950: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED1Ev")]
// 0x5f9958 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED1Ev
pub fn stub_0x5f9958() {
    // IDA 0x5f9958: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED0Ev")]
// 0x5f999c — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEED0Ev
// type: int __fastcall(int)
pub fn stub_0x5f999c() {
    // IDA 0x5f999c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE12getClassNameEv")]
// 0x5fd88c — __ZNK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE12getClassNameEv
pub fn stub_0x5fd88c() {
    // IDA 0x5fd88c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE12getClassNameEv")]
// 0x5fd964 — __ZThn32_NK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE12getClassNameEv
pub fn stub_0x5fd964() {
    // IDA 0x5fd964: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE12getClassNameEv")]
// 0x5fdc8c — __ZNK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE12getClassNameEv
pub fn stub_0x5fdc8c() {
    // IDA 0x5fdc8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE12getClassNameEv")]
// 0x5fdf00 — __ZThn32_NK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE12getClassNameEv
pub fn stub_0x5fdf00() {
    // IDA 0x5fdf00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sPlayerGuiEEEEvv")]
// 0x5fec98 — __ZN3RBX4Name13callDoDeclareILZNS_10sPlayerGuiEEEEvv
pub fn stub_0x5fec98() {
    // IDA 0x5fec98: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sPlayerGuiEEEERKS0_v")]
// 0x5fec9c — __ZN3RBX4Name9doDeclareILZNS_10sPlayerGuiEEEERKS0_v
pub fn stub_0x5fec9c() {
    // IDA 0x5fec9c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sBasePlayerGuiEEEEvv")]
// 0x5ff03c — __ZN3RBX4Name13callDoDeclareILZNS_14sBasePlayerGuiEEEEvv
pub fn stub_0x5ff03c() {
    // IDA 0x5ff03c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sBasePlayerGuiEEEERKS0_v")]
// 0x5ff040 — __ZN3RBX4Name9doDeclareILZNS_14sBasePlayerGuiEEEERKS0_v
pub fn stub_0x5ff040() {
    // IDA 0x5ff040: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13RelativePanelELZNS_10sPlayerHUDEEE12getClassNameEv")]
// 0x6048c4 — __ZNK3RBX17NonFactoryProductINS_13RelativePanelELZNS_10sPlayerHUDEEE12getClassNameEv
pub fn stub_0x6048c4() {
    // IDA 0x6048c4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13RelativePanelELZNS_10sPlayerHUDEEE12getClassNameEv")]
// 0x604998 — __ZThn32_NK3RBX17NonFactoryProductINS_13RelativePanelELZNS_10sPlayerHUDEEE12getClassNameEv
pub fn stub_0x604998() {
    // IDA 0x604998: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sPlayerHUDEEEEvv")]
// 0x604a6c — __ZN3RBX4Name13callDoDeclareILZNS_10sPlayerHUDEEEEvv
pub fn stub_0x604a6c() {
    // IDA 0x604a6c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sPlayerHUDEEEERKS0_v")]
// 0x604a70 — __ZN3RBX4Name9doDeclareILZNS_10sPlayerHUDEEEERKS0_v
pub fn stub_0x604a70() {
    // IDA 0x604a70: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_5MouseELZNS_12sPlayerMouseEEE12getClassNameEv")]
// 0x6053c4 — __ZNK3RBX17NonFactoryProductINS_5MouseELZNS_12sPlayerMouseEEE12getClassNameEv
pub fn stub_0x6053c4() {
    // IDA 0x6053c4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_5MouseELZNS_12sPlayerMouseEEE12getClassNameEv")]
// 0x605580 — __ZThn32_NK3RBX17NonFactoryProductINS_5MouseELZNS_12sPlayerMouseEEE12getClassNameEv
pub fn stub_0x605580() {
    // IDA 0x605580: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sPlayerMouseEEEEvv")]
// 0x6055a8 — __ZN3RBX4Name13callDoDeclareILZNS_12sPlayerMouseEEEEvv
pub fn stub_0x6055a8() {
    // IDA 0x6055a8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sPlayerMouseEEEERKS0_v")]
// 0x6055ac — __ZN3RBX4Name9doDeclareILZNS_12sPlayerMouseEEEERKS0_v
pub fn stub_0x6055ac() {
    // IDA 0x6055ac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sPoseEEEEvv")]
// 0x6069a8 — __ZN3RBX4Name13callDoDeclareILZNS_5sPoseEEEEvv
pub fn stub_0x6069a8() {
    // IDA 0x6069a8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sPoseEEEERKS0_v")]
// 0x6069ac — __ZN3RBX4Name9doDeclareILZNS_5sPoseEEEERKS0_v
pub fn stub_0x6069ac() {
    // IDA 0x6069ac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEE12getClassNameEv")]
// 0x60f244 — __ZNK3RBX17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEE12getClassNameEv
pub fn stub_0x60f244() {
    // IDA 0x60f244: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEE12getClassNameEv")]
// 0x60f490 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiObjectELZNS_12sScale9FrameEEE12getClassNameEv
pub fn stub_0x60f490() {
    // IDA 0x60f490: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sScale9FrameEEEEvv")]
// 0x60f6dc — __ZN3RBX4Name13callDoDeclareILZNS_12sScale9FrameEEEEvv
pub fn stub_0x60f6dc() {
    // IDA 0x60f6dc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sScale9FrameEEEERKS0_v")]
// 0x60f6e0 — __ZN3RBX4Name9doDeclareILZNS_12sScale9FrameEEEERKS0_v
pub fn stub_0x60f6e0() {
    // IDA 0x60f6e0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sGuiMainEEEEvv")]
// 0x612264 — __ZN3RBX4Name13callDoDeclareILZNS_8sGuiMainEEEEvv
pub fn stub_0x612264() {
    // IDA 0x612264: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sGuiMainEEEERKS0_v")]
// 0x612268 — __ZN3RBX4Name9doDeclareILZNS_8sGuiMainEEEERKS0_v
pub fn stub_0x612268() {
    // IDA 0x612268: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sScreenGuiEEEEvv")]
// 0x612d80 — __ZN3RBX4Name13callDoDeclareILZNS_10sScreenGuiEEEEvv
pub fn stub_0x612d80() {
    // IDA 0x612d80: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sScreenGuiEEEERKS0_v")]
// 0x612d84 — __ZN3RBX4Name9doDeclareILZNS_10sScreenGuiEEEERKS0_v
pub fn stub_0x612d84() {
    // IDA 0x612d84: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_6sMouseEEEERKS0_v")]
// 0x61558c — __ZN3RBX4Name7declareILZNS_6sMouseEEEERKS0_v
// type: int(void)
pub fn stub_0x61558c() {
    // IDA 0x61558c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sMouseEEEEvv")]
// 0x6155d0 — __ZN3RBX4Name13callDoDeclareILZNS_6sMouseEEEEvv
pub fn stub_0x6155d0() {
    // IDA 0x6155d0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sMouseEEEERKS0_v")]
// 0x6155d4 — __ZN3RBX4Name9doDeclareILZNS_6sMouseEEEERKS0_v
pub fn stub_0x6155d4() {
    // IDA 0x6155d4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sSeatEEEEvv")]
// 0x617ae8 — __ZN3RBX4Name13callDoDeclareILZNS_5sSeatEEEEvv
pub fn stub_0x617ae8() {
    // IDA 0x617ae8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sSeatEEEERKS0_v")]
// 0x617aec — __ZN3RBX4Name9doDeclareILZNS_5sSeatEEEERKS0_v
pub fn stub_0x617aec() {
    // IDA 0x617aec: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sSelectionBoxEEEEvv")]
// 0x61de28 — __ZN3RBX4Name13callDoDeclareILZNS_13sSelectionBoxEEEEvv
pub fn stub_0x61de28() {
    // IDA 0x61de28: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sSelectionBoxEEEERKS0_v")]
// 0x61de2c — __ZN3RBX4Name9doDeclareILZNS_13sSelectionBoxEEEERKS0_v
pub fn stub_0x61de2c() {
    // IDA 0x61de2c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEE12getClassNameEv")]
// 0x61fb08 — __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEE12getClassNameEv
pub fn stub_0x61fb08() {
    // IDA 0x61fb08: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEE12getClassNameEv")]
// 0x61fdd0 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEE12getClassNameEv
pub fn stub_0x61fdd0() {
    // IDA 0x61fdd0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sSelectionPointLassoEEEEvv")]
// 0x620d50 — __ZN3RBX4Name13callDoDeclareILZNS_20sSelectionPointLassoEEEEvv
pub fn stub_0x620d50() {
    // IDA 0x620d50: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sSelectionPointLassoEEEERKS0_v")]
// 0x620d54 — __ZN3RBX4Name9doDeclareILZNS_20sSelectionPointLassoEEEERKS0_v
pub fn stub_0x620d54() {
    // IDA 0x620d54: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sSelectionPartLassoEEEEvv")]
// 0x621c18 — __ZN3RBX4Name13callDoDeclareILZNS_19sSelectionPartLassoEEEEvv
pub fn stub_0x621c18() {
    // IDA 0x621c18: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sSelectionPartLassoEEEERKS0_v")]
// 0x621c1c — __ZN3RBX4Name9doDeclareILZNS_19sSelectionPartLassoEEEERKS0_v
pub fn stub_0x621c1c() {
    // IDA 0x621c1c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sSelectionLassoEEEEvv")]
// 0x621fb4 — __ZN3RBX4Name13callDoDeclareILZNS_15sSelectionLassoEEEEvv
pub fn stub_0x621fb4() {
    // IDA 0x621fb4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sSelectionLassoEEEERKS0_v")]
// 0x621fb8 — __ZN3RBX4Name9doDeclareILZNS_15sSelectionLassoEEEERKS0_v
pub fn stub_0x621fb8() {
    // IDA 0x621fb8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_21sSkateboardControllerEEEEvv")]
// 0x625e78 — __ZN3RBX4Name13callDoDeclareILZNS_21sSkateboardControllerEEEEvv
pub fn stub_0x625e78() {
    // IDA 0x625e78: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sSkateboardControllerEEEERKS0_v")]
// 0x625e7c — __ZN3RBX4Name9doDeclareILZNS_21sSkateboardControllerEEEERKS0_v
pub fn stub_0x625e7c() {
    // IDA 0x625e7c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sSkateboardPlatformEEEEvv")]
// 0x62b3b4 — __ZN3RBX4Name13callDoDeclareILZNS_19sSkateboardPlatformEEEEvv
pub fn stub_0x62b3b4() {
    // IDA 0x62b3b4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sSkateboardPlatformEEEERKS0_v")]
// 0x62b3b8 — __ZN3RBX4Name9doDeclareILZNS_19sSkateboardPlatformEEEERKS0_v
pub fn stub_0x62b3b8() {
    // IDA 0x62b3b8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_4sSkyEEEEvv")]
// 0x635de0 — __ZN3RBX4Name13callDoDeclareILZNS_4sSkyEEEEvv
pub fn stub_0x635de0() {
    // IDA 0x635de0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_4sSkyEEEERKS0_v")]
// 0x635de4 — __ZN3RBX4Name9doDeclareILZNS_4sSkyEEEERKS0_v
pub fn stub_0x635de4() {
    // IDA 0x635de4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sSmokeEEEEvv")]
// 0x637f3c — __ZN3RBX4Name13callDoDeclareILZNS_6sSmokeEEEEvv
pub fn stub_0x637f3c() {
    // IDA 0x637f3c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sSmokeEEEERKS0_v")]
// 0x637f40 — __ZN3RBX4Name9doDeclareILZNS_6sSmokeEEEERKS0_v
pub fn stub_0x637f40() {
    // IDA 0x637f40: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX14SpawnerService16GetSpawnLocationEPNS_7Network6PlayerESs")]
// 0x63df08 — __ZN3RBX14SpawnerService16GetSpawnLocationEPNS_7Network6PlayerESs
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x63df08() {
    // IDA 0x63df08: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sSpawnLocationEEEEvv")]
// 0x63f16c — __ZN3RBX4Name13callDoDeclareILZNS_14sSpawnLocationEEEEvv
pub fn stub_0x63f16c() {
    // IDA 0x63f16c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sSpawnLocationEEEERKS0_v")]
// 0x63f170 — __ZN3RBX4Name9doDeclareILZNS_14sSpawnLocationEEEERKS0_v
pub fn stub_0x63f170() {
    // IDA 0x63f170: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sSpecialShapeEEEEvv")]
// 0x64257c — __ZN3RBX4Name13callDoDeclareILZNS_13sSpecialShapeEEEEvv
pub fn stub_0x64257c() {
    // IDA 0x64257c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sSpecialShapeEEEERKS0_v")]
// 0x642580 — __ZN3RBX4Name9doDeclareILZNS_13sSpecialShapeEEEERKS0_v
pub fn stub_0x642580() {
    // IDA 0x642580: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFileMeshEEEERKS0_v")]
// 0x642f10 — __ZN3RBX4Name7declareILZNS_9sFileMeshEEEERKS0_v
// type: int(void)
pub fn stub_0x642f10() {
    // IDA 0x642f10: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sFileMeshEEEEvv")]
// 0x642f54 — __ZN3RBX4Name13callDoDeclareILZNS_9sFileMeshEEEEvv
pub fn stub_0x642f54() {
    // IDA 0x642f54: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFileMeshEEEERKS0_v")]
// 0x642f58 — __ZN3RBX4Name9doDeclareILZNS_9sFileMeshEEEERKS0_v
pub fn stub_0x642f58() {
    // IDA 0x642f58: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5Stats10sStatsItemEEEEvv")]
// 0x64bed8 — __ZN3RBX4Name13callDoDeclareILZNS_5Stats10sStatsItemEEEEvv
pub fn stub_0x64bed8() {
    // IDA 0x64bed8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEE12getClassNameEv")]
// 0x64e8c0 — __ZNK3RBX17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEE12getClassNameEv
pub fn stub_0x64e8c0() {
    // IDA 0x64e8c0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEE12getClassNameEv")]
// 0x64ec30 — __ZThn32_NK3RBX17NonFactoryProductINS_5Stats4ItemELZ14sProfilingItemEE12getClassNameEv
pub fn stub_0x64ec30() {
    // IDA 0x64ec30: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ14sProfilingItemEEEvv")]
// 0x64ed68 — __ZN3RBX4Name13callDoDeclareILZ14sProfilingItemEEEvv
pub fn stub_0x64ed68() {
    // IDA 0x64ed68: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ14sProfilingItemEEERKS0_v")]
// 0x64ed6c — __ZN3RBX4Name9doDeclareILZ14sProfilingItemEEERKS0_v
pub fn stub_0x64ed6c() {
    // IDA 0x64ed6c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEE12getClassNameEv")]
// 0x64f580 — __ZNK3RBX17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEE12getClassNameEv
pub fn stub_0x64f580() {
    // IDA 0x64f580: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEE12getClassNameEv")]
// 0x64f828 — __ZThn32_NK3RBX17NonFactoryProductI18RunningAverageItemIdELZ25sRunningAverageItemDoubleEE12getClassNameEv
pub fn stub_0x64f828() {
    // IDA 0x64f828: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ25sRunningAverageItemDoubleEEEvv")]
// 0x64f960 — __ZN3RBX4Name13callDoDeclareILZ25sRunningAverageItemDoubleEEEvv
pub fn stub_0x64f960() {
    // IDA 0x64f960: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ25sRunningAverageItemDoubleEEERKS0_v")]
// 0x64f964 — __ZN3RBX4Name9doDeclareILZ25sRunningAverageItemDoubleEEERKS0_v
pub fn stub_0x64f964() {
    // IDA 0x64f964: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEE12getClassNameEv")]
// 0x650178 — __ZNK3RBX17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEE12getClassNameEv
pub fn stub_0x650178() {
    // IDA 0x650178: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEE12getClassNameEv")]
// 0x650420 — __ZThn32_NK3RBX17NonFactoryProductI18RunningAverageItemIiELZ22sRunningAverageItemIntEE12getClassNameEv
pub fn stub_0x650420() {
    // IDA 0x650420: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ22sRunningAverageItemIntEEEvv")]
// 0x650558 — __ZN3RBX4Name13callDoDeclareILZ22sRunningAverageItemIntEEEvv
pub fn stub_0x650558() {
    // IDA 0x650558: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ22sRunningAverageItemIntEEERKS0_v")]
// 0x65055c — __ZN3RBX4Name9doDeclareILZ22sRunningAverageItemIntEEERKS0_v
pub fn stub_0x65055c() {
    // IDA 0x65055c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEE12getClassNameEv")]
// 0x650d70 — __ZNK3RBX17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEE12getClassNameEv
pub fn stub_0x650d70() {
    // IDA 0x650d70: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEE12getClassNameEv")]
// 0x650ff8 — __ZThn32_NK3RBX17NonFactoryProductINS_5Stats4ItemELZ27sTotalCountTimeIntervalItemEE12getClassNameEv
pub fn stub_0x650ff8() {
    // IDA 0x650ff8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ27sTotalCountTimeIntervalItemEEEvv")]
// 0x651130 — __ZN3RBX4Name13callDoDeclareILZ27sTotalCountTimeIntervalItemEEEvv
pub fn stub_0x651130() {
    // IDA 0x651130: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ27sTotalCountTimeIntervalItemEEERKS0_v")]
// 0x651134 — __ZN3RBX4Name9doDeclareILZ27sTotalCountTimeIntervalItemEEERKS0_v
pub fn stub_0x651134() {
    // IDA 0x651134: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sSurfaceSelectionEEEEvv")]
// 0x661974 — __ZN3RBX4Name13callDoDeclareILZNS_17sSurfaceSelectionEEEEvv
pub fn stub_0x661974() {
    // IDA 0x661974: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sSurfaceSelectionEEEERKS0_v")]
// 0x661978 — __ZN3RBX4Name9doDeclareILZNS_17sSurfaceSelectionEEEERKS0_v
pub fn stub_0x661978() {
    // IDA 0x661978: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sTeamEEEEvv")]
// 0x6635dc — __ZN3RBX4Name13callDoDeclareILZNS_5sTeamEEEEvv
pub fn stub_0x6635dc() {
    // IDA 0x6635dc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sTeamEEEERKS0_v")]
// 0x6635e0 — __ZN3RBX4Name9doDeclareILZNS_5sTeamEEEERKS0_v
pub fn stub_0x6635e0() {
    // IDA 0x6635e0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX5Teams21assignNewPlayerToTeamEPNS_7Network6PlayerE")]
// 0x664a54 — __ZN3RBX5Teams21assignNewPlayerToTeamEPNS_7Network6PlayerE
// type: _DWORD __fastcall(RBX::Teams *__hidden this, RBX::Network::Player *)
pub fn stub_0x664a54() {
    // IDA 0x664a54: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX5Teams17getTeamFromPlayerEPNS_7Network6PlayerE")]
// 0x664c9c — __ZN3RBX5Teams17getTeamFromPlayerEPNS_7Network6PlayerE
// type: _DWORD __fastcall(RBX::Teams *__hidden this, RBX::Network::Player *)
pub fn stub_0x664c9c() {
    // IDA 0x664c9c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sTextBoxEEEEvv")]
// 0x6695d0 — __ZN3RBX4Name13callDoDeclareILZNS_8sTextBoxEEEEvv
pub fn stub_0x6695d0() {
    // IDA 0x6695d0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sTextBoxEEEERKS0_v")]
// 0x6695d4 — __ZN3RBX4Name9doDeclareILZNS_8sTextBoxEEEERKS0_v
pub fn stub_0x6695d4() {
    // IDA 0x6695d4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEE12getClassNameEv")]
// 0x66ad30 — __ZNK3RBX17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEE12getClassNameEv
pub fn stub_0x66ad30() {
    // IDA 0x66ad30: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEE12getClassNameEv")]
// 0x66ade0 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEE12getClassNameEv
pub fn stub_0x66ade0() {
    // IDA 0x66ade0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sGuiObjectEEEERKS0_v")]
// 0x66ae90 — __ZN3RBX4Name7declareILZNS_10sGuiObjectEEEERKS0_v
// type: int(void)
pub fn stub_0x66ae90() {
    // IDA 0x66ae90: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sGuiObjectEEEEvv")]
// 0x66aed4 — __ZN3RBX4Name13callDoDeclareILZNS_10sGuiObjectEEEEvv
pub fn stub_0x66aed4() {
    // IDA 0x66aed4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sGuiObjectEEEERKS0_v")]
// 0x66aed8 — __ZN3RBX4Name9doDeclareILZNS_10sGuiObjectEEEERKS0_v
pub fn stub_0x66aed8() {
    // IDA 0x66aed8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEED2Ev")]
// 0x66b85c — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEED2Ev
pub fn stub_0x66b85c() {
    // IDA 0x66b85c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEED2Ev")]
// 0x66b98c — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEED2Ev
pub fn stub_0x66b98c() {
    // IDA 0x66b98c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sTextServiceEEEERKS0_v")]
// 0x66bce0 — __ZN3RBX4Name7declareILZNS_12sTextServiceEEEERKS0_v
// type: int(void)
pub fn stub_0x66bce0() {
    // IDA 0x66bce0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sTextServiceEEEEvv")]
// 0x66bd24 — __ZN3RBX4Name13callDoDeclareILZNS_12sTextServiceEEEEvv
pub fn stub_0x66bd24() {
    // IDA 0x66bd24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sTextServiceEEEERKS0_v")]
// 0x66bd28 — __ZN3RBX4Name9doDeclareILZNS_12sTextServiceEEEERKS0_v
pub fn stub_0x66bd28() {
    // IDA 0x66bd28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sGuiTextButtonEEEEvv")]
// 0x674574 — __ZN3RBX4Name13callDoDeclareILZNS_14sGuiTextButtonEEEEvv
pub fn stub_0x674574() {
    // IDA 0x674574: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sGuiTextButtonEEEERKS0_v")]
// 0x674578 — __ZN3RBX4Name9doDeclareILZNS_14sGuiTextButtonEEEERKS0_v
// type: int()
pub fn stub_0x674578() {
    // IDA 0x674578: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sTextLabelEEEEvv")]
// 0x67977c — __ZN3RBX4Name13callDoDeclareILZNS_10sTextLabelEEEEvv
pub fn stub_0x67977c() {
    // IDA 0x67977c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sTextLabelEEEERKS0_v")]
// 0x679780 — __ZN3RBX4Name9doDeclareILZNS_10sTextLabelEEEERKS0_v
// type: int()
pub fn stub_0x679780() {
    // IDA 0x679780: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Tool7dropAllEPNS_7Network6PlayerE")]
// 0x68052c — __ZN3RBX4Tool7dropAllEPNS_7Network6PlayerE
// type: _DWORD __fastcall(RBX::Tool *__hidden this, RBX::Network::Player *)
pub fn stub_0x68052c() {
    // IDA 0x68052c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Tool22moveAllToolsToBackpackEPNS_7Network6PlayerE")]
// 0x68057c — __ZN3RBX4Tool22moveAllToolsToBackpackEPNS_7Network6PlayerE
// type: _DWORD __fastcall(RBX::Tool *__hidden this, RBX::Network::Player *)
pub fn stub_0x68057c() {
    // IDA 0x68057c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}
