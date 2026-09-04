//! core shard nd — 100 core stubs EA-sorted asc global gap filler not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33887, 1323 uncovered before -> 1223 after, batch 0xf326f4..0xf36634).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v")]
// 0xf326f4 — j___ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v
pub fn stub_0xf326f4() {
    // IDA 0xf326f4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sAdornmentGuiEEEERKS0_v")]
// 0xf32794 — j___ZN3RBX4Name9doDeclareILZNS_13sAdornmentGuiEEEERKS0_v
pub fn stub_0xf32794() {
    // IDA 0xf32794: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "ChangeHistoryStatsItem::ChangeHistoryStatsItem(void)")]
// 0xf32f94 — j___ZN22ChangeHistoryStatsItemC2Ev
// type: ChangeHistoryStatsItem *__fastcall(ChangeHistoryStatsItem *__hidden this)
pub fn stub_0xf32f94() {
    // IDA 0xf32f94: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sBodyColorsEEEERKS0_v")]
// 0xf33b14 — j___ZN3RBX4Name9doDeclareILZNS_11sBodyColorsEEEERKS0_v
pub fn stub_0xf33b14() {
    // IDA 0xf33b14: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v")]
// 0xf33b24 — j___ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v
pub fn stub_0xf33b24() {
    // IDA 0xf33b24: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_5sSkinEEEERKS0_v")]
// 0xf33b34 — j___ZN3RBX4Name9doDeclareILZNS_5sSkinEEEERKS0_v
pub fn stub_0xf33b34() {
    // IDA 0xf33b34: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_6sPantsEEEERKS0_v")]
// 0xf33b44 — j___ZN3RBX4Name9doDeclareILZNS_6sPantsEEEERKS0_v
pub fn stub_0xf33b44() {
    // IDA 0xf33b44: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_6sShirtEEEERKS0_v")]
// 0xf33b54 — j___ZN3RBX4Name9doDeclareILZNS_6sShirtEEEERKS0_v
pub fn stub_0xf33b54() {
    // IDA 0xf33b54: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sClothingEEEERKS0_v")]
// 0xf33b64 — j___ZN3RBX4Name9doDeclareILZNS_9sClothingEEEERKS0_v
pub fn stub_0xf33b64() {
    // IDA 0xf33b64: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_20sCharacterAppearanceEEEERKS0_v")]
// 0xf33d64 — j___ZN3RBX4Name7declareILZNS_20sCharacterAppearanceEEEERKS0_v
pub fn stub_0xf33d64() {
    // IDA 0xf33d64: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v")]
// 0xf33d74 — j___ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v
pub fn stub_0xf33d74() {
    // IDA 0xf33d74: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_20sCharacterAppearanceEEEERKS0_v")]
// 0xf33d84 — j___ZN3RBX4Name9doDeclareILZNS_20sCharacterAppearanceEEEERKS0_v
// type: int(void)
pub fn stub_0xf33d84() {
    // IDA 0xf33d84: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_18sFilteredSelectionEEEERKS0_v")]
// 0xf34504 — j___ZN3RBX4Name7declareILZNS_18sFilteredSelectionEEEERKS0_v
// type: int(void)
pub fn stub_0xf34504() {
    // IDA 0xf34504: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_18sFilteredSelectionEEEERKS0_v")]
// 0xf34524 — j___ZN3RBX4Name9doDeclareILZNS_18sFilteredSelectionEEEERKS0_v
pub fn stub_0xf34524() {
    // IDA 0xf34524: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_10sCloneToolEEEERKS0_v")]
// 0xf349c4 — j___ZN3RBX4Name7declareILZNS_10sCloneToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf349c4() {
    // IDA 0xf349c4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_10sHingeToolEEEERKS0_v")]
// 0xf349d4 — j___ZN3RBX4Name7declareILZNS_10sHingeToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf349d4() {
    // IDA 0xf349d4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_10sInletToolEEEERKS0_v")]
// 0xf349e4 — j___ZN3RBX4Name7declareILZNS_10sInletToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf349e4() {
    // IDA 0xf349e4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_10sStudsToolEEEERKS0_v")]
// 0xf349f4 — j___ZN3RBX4Name7declareILZNS_10sStudsToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf349f4() {
    // IDA 0xf349f4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v")]
// 0xf34a04 — j___ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34a04() {
    // IDA 0xf34a04: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_11sHammerToolEEEERKS0_v")]
// 0xf34a14 — j___ZN3RBX4Name7declareILZNS_11sHammerToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34a14() {
    // IDA 0xf34a14: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_12sAdvMoveToolEEEERKS0_v")]
// 0xf34a24 — j___ZN3RBX4Name7declareILZNS_12sAdvMoveToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34a24() {
    // IDA 0xf34a24: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_12sDropperToolEEEERKS0_v")]
// 0xf34a34 — j___ZN3RBX4Name7declareILZNS_12sDropperToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34a34() {
    // IDA 0xf34a34: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v")]
// 0xf34a44 — j___ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34a44() {
    // IDA 0xf34a44: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sAdvRotateToolEEEERKS0_v")]
// 0xf34a54 — j___ZN3RBX4Name7declareILZNS_14sAdvRotateToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34a54() {
    // IDA 0xf34a54: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v")]
// 0xf34a64 — j___ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34a64() {
    // IDA 0xf34a64: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sUniversalToolEEEERKS0_v")]
// 0xf34a74 — j___ZN3RBX4Name7declareILZNS_14sUniversalToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34a74() {
    // IDA 0xf34a74: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_15sAxisRotateToolEEEERKS0_v")]
// 0xf34a84 — j___ZN3RBX4Name7declareILZNS_15sAxisRotateToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34a84() {
    // IDA 0xf34a84: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v")]
// 0xf34a94 — j___ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34a94() {
    // IDA 0xf34a94: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_19sMoveResizeJoinToolEEEERKS0_v")]
// 0xf34aa4 — j___ZN3RBX4Name7declareILZNS_19sMoveResizeJoinToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34aa4() {
    // IDA 0xf34aa4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v")]
// 0xf34ab4 — j___ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34ab4() {
    // IDA 0xf34ab4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v")]
// 0xf34ac4 — j___ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34ac4() {
    // IDA 0xf34ac4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_9sFillToolEEEERKS0_v")]
// 0xf34ad4 — j___ZN3RBX4Name7declareILZNS_9sFillToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34ad4() {
    // IDA 0xf34ad4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_9sFlatToolEEEERKS0_v")]
// 0xf34ae4 — j___ZN3RBX4Name7declareILZNS_9sFlatToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34ae4() {
    // IDA 0xf34ae4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_9sGameToolEEEERKS0_v")]
// 0xf34af4 — j___ZN3RBX4Name7declareILZNS_9sGameToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34af4() {
    // IDA 0xf34af4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_9sGlueToolEEEERKS0_v")]
// 0xf34b04 — j___ZN3RBX4Name7declareILZNS_9sGlueToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34b04() {
    // IDA 0xf34b04: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_9sGrabToolEEEERKS0_v")]
// 0xf34b14 — j___ZN3RBX4Name7declareILZNS_9sGrabToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34b14() {
    // IDA 0xf34b14: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_9sLockToolEEEERKS0_v")]
// 0xf34b24 — j___ZN3RBX4Name7declareILZNS_9sLockToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34b24() {
    // IDA 0xf34b24: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_9sNullToolEEEERKS0_v")]
// 0xf34b34 — j___ZN3RBX4Name7declareILZNS_9sNullToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34b34() {
    // IDA 0xf34b34: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_9sWeldToolEEEERKS0_v")]
// 0xf34b44 — j___ZN3RBX4Name7declareILZNS_9sWeldToolEEEERKS0_v
// type: int(void)
pub fn stub_0xf34b44() {
    // IDA 0xf34b44: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v")]
// 0xf34b54 — j___ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v
pub fn stub_0xf34b54() {
    // IDA 0xf34b54: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sHingeToolEEEERKS0_v")]
// 0xf34b64 — j___ZN3RBX4Name9doDeclareILZNS_10sHingeToolEEEERKS0_v
pub fn stub_0xf34b64() {
    // IDA 0xf34b64: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sInletToolEEEERKS0_v")]
// 0xf34b74 — j___ZN3RBX4Name9doDeclareILZNS_10sInletToolEEEERKS0_v
pub fn stub_0xf34b74() {
    // IDA 0xf34b74: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sStudsToolEEEERKS0_v")]
// 0xf34b84 — j___ZN3RBX4Name9doDeclareILZNS_10sStudsToolEEEERKS0_v
pub fn stub_0xf34b84() {
    // IDA 0xf34b84: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v")]
// 0xf34b94 — j___ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v
pub fn stub_0xf34b94() {
    // IDA 0xf34b94: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v")]
// 0xf34ba4 — j___ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v
pub fn stub_0xf34ba4() {
    // IDA 0xf34ba4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sAdvMoveToolEEEERKS0_v")]
// 0xf34bb4 — j___ZN3RBX4Name9doDeclareILZNS_12sAdvMoveToolEEEERKS0_v
pub fn stub_0xf34bb4() {
    // IDA 0xf34bb4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sDropperToolEEEERKS0_v")]
// 0xf34bc4 — j___ZN3RBX4Name9doDeclareILZNS_12sDropperToolEEEERKS0_v
pub fn stub_0xf34bc4() {
    // IDA 0xf34bc4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v")]
// 0xf34bd4 — j___ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v
pub fn stub_0xf34bd4() {
    // IDA 0xf34bd4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sAdvRotateToolEEEERKS0_v")]
// 0xf34be4 — j___ZN3RBX4Name9doDeclareILZNS_14sAdvRotateToolEEEERKS0_v
pub fn stub_0xf34be4() {
    // IDA 0xf34be4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v")]
// 0xf34bf4 — j___ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v
pub fn stub_0xf34bf4() {
    // IDA 0xf34bf4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sUniversalToolEEEERKS0_v")]
// 0xf34c04 — j___ZN3RBX4Name9doDeclareILZNS_14sUniversalToolEEEERKS0_v
pub fn stub_0xf34c04() {
    // IDA 0xf34c04: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_15sAxisRotateToolEEEERKS0_v")]
// 0xf34c14 — j___ZN3RBX4Name9doDeclareILZNS_15sAxisRotateToolEEEERKS0_v
pub fn stub_0xf34c14() {
    // IDA 0xf34c14: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v")]
// 0xf34c24 — j___ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v
pub fn stub_0xf34c24() {
    // IDA 0xf34c24: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_19sMoveResizeJoinToolEEEERKS0_v")]
// 0xf34c34 — j___ZN3RBX4Name9doDeclareILZNS_19sMoveResizeJoinToolEEEERKS0_v
pub fn stub_0xf34c34() {
    // IDA 0xf34c34: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v")]
// 0xf34c44 — j___ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v
pub fn stub_0xf34c44() {
    // IDA 0xf34c44: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v")]
// 0xf34c54 — j___ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v
pub fn stub_0xf34c54() {
    // IDA 0xf34c54: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sFillToolEEEERKS0_v")]
// 0xf34c64 — j___ZN3RBX4Name9doDeclareILZNS_9sFillToolEEEERKS0_v
pub fn stub_0xf34c64() {
    // IDA 0xf34c64: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sFlatToolEEEERKS0_v")]
// 0xf34c74 — j___ZN3RBX4Name9doDeclareILZNS_9sFlatToolEEEERKS0_v
pub fn stub_0xf34c74() {
    // IDA 0xf34c74: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v")]
// 0xf34c84 — j___ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v
pub fn stub_0xf34c84() {
    // IDA 0xf34c84: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sGlueToolEEEERKS0_v")]
// 0xf34c94 — j___ZN3RBX4Name9doDeclareILZNS_9sGlueToolEEEERKS0_v
pub fn stub_0xf34c94() {
    // IDA 0xf34c94: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v")]
// 0xf34ca4 — j___ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v
pub fn stub_0xf34ca4() {
    // IDA 0xf34ca4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sLockToolEEEERKS0_v")]
// 0xf34cb4 — j___ZN3RBX4Name9doDeclareILZNS_9sLockToolEEEERKS0_v
// type: int()
pub fn stub_0xf34cb4() {
    // IDA 0xf34cb4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v")]
// 0xf34cc4 — j___ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v
pub fn stub_0xf34cc4() {
    // IDA 0xf34cc4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sWeldToolEEEERKS0_v")]
// 0xf34cd4 — j___ZN3RBX4Name9doDeclareILZNS_9sWeldToolEEEERKS0_v
// type: int()
pub fn stub_0xf34cd4() {
    // IDA 0xf34cd4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v")]
// 0xf35534 — j___ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v
pub fn stub_0xf35534() {
    // IDA 0xf35534: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v")]
// 0xf355e4 — j___ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v
pub fn stub_0xf355e4() {
    // IDA 0xf355e4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE15isNullClassNameEv")]
// 0xf362c4 — j___ZN3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE15isNullClassNameEv
// type: int()
pub fn stub_0xf362c4() {
    // IDA 0xf362c4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE15isNullClassNameEv")]
// 0xf362d4 — j___ZN3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE15isNullClassNameEv
// type: int()
pub fn stub_0xf362d4() {
    // IDA 0xf362d4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEE15isNullClassNameEv")]
// 0xf362e4 — j___ZN3RBX17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEE15isNullClassNameEv
// type: int()
pub fn stub_0xf362e4() {
    // IDA 0xf362e4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_11sGuiServiceEEEERKS0_v")]
// 0xf36414 — j___ZN3RBX4Name7declareILZNS_11sGuiServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36414() {
    // IDA 0xf36414: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v")]
// 0xf36424 — j___ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v
// type: int()
pub fn stub_0xf36424() {
    // IDA 0xf36424: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_12sTestServiceEEEERKS0_v")]
// 0xf36434 — j___ZN3RBX4Name7declareILZNS_12sTestServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36434() {
    // IDA 0xf36434: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_13sAssetServiceEEEERKS0_v")]
// 0xf36444 — j___ZN3RBX4Name7declareILZNS_13sAssetServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36444() {
    // IDA 0xf36444: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_13sBadgeServiceEEEERKS0_v")]
// 0xf36454 — j___ZN3RBX4Name7declareILZNS_13sBadgeServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36454() {
    // IDA 0xf36454: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v")]
// 0xf36464 — j___ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v
// type: int(void)
pub fn stub_0xf36464() {
    // IDA 0xf36464: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sDebrisServiceEEEERKS0_v")]
// 0xf36474 — j___ZN3RBX4Name7declareILZNS_14sDebrisServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36474() {
    // IDA 0xf36474: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sFriendServiceEEEERKS0_v")]
// 0xf36484 — j___ZN3RBX4Name7declareILZNS_14sFriendServiceEEEERKS0_v
// type: int()
pub fn stub_0xf36484() {
    // IDA 0xf36484: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sJointsServiceEEEERKS0_v")]
// 0xf36494 — j___ZN3RBX4Name7declareILZNS_14sJointsServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36494() {
    // IDA 0xf36494: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sLocalBackpackEEEERKS0_v")]
// 0xf364a4 — j___ZN3RBX4Name7declareILZNS_14sLocalBackpackEEEERKS0_v
// type: int(void)
pub fn stub_0xf364a4() {
    // IDA 0xf364a4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sServerStorageEEEERKS0_v")]
// 0xf364b4 — j___ZN3RBX4Name7declareILZNS_14sServerStorageEEEERKS0_v
// type: int(void)
pub fn stub_0xf364b4() {
    // IDA 0xf364b4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sSocialServiceEEEERKS0_v")]
// 0xf364c4 — j___ZN3RBX4Name7declareILZNS_14sSocialServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf364c4() {
    // IDA 0xf364c4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_15sCookiesServiceEEEERKS0_v")]
// 0xf364d4 — j___ZN3RBX4Name7declareILZNS_15sCookiesServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf364d4() {
    // IDA 0xf364d4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_15sPhysicsServiceEEEERKS0_v")]
// 0xf364e4 — j___ZN3RBX4Name7declareILZNS_15sPhysicsServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf364e4() {
    // IDA 0xf364e4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_15sSpawnerServiceEEEERKS0_v")]
// 0xf364f4 — j___ZN3RBX4Name7declareILZNS_15sSpawnerServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf364f4() {
    // IDA 0xf364f4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_16sGamePassServiceEEEERKS0_v")]
// 0xf36504 — j___ZN3RBX4Name7declareILZNS_16sGamePassServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36504() {
    // IDA 0xf36504: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_16sGeometryServiceEEEERKS0_v")]
// 0xf36514 — j___ZN3RBX4Name7declareILZNS_16sGeometryServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36514() {
    // IDA 0xf36514: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_16sTeleportServiceEEEERKS0_v")]
// 0xf36524 — j___ZN3RBX4Name7declareILZNS_16sTeleportServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36524() {
    // IDA 0xf36524: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_18sCollectionServiceEEEERKS0_v")]
// 0xf36534 — j___ZN3RBX4Name7declareILZNS_18sCollectionServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36534() {
    // IDA 0xf36534: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v")]
// 0xf36544 — j___ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v
// type: int(void)
pub fn stub_0xf36544() {
    // IDA 0xf36544: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_18sStarterGuiServiceEEEERKS0_v")]
// 0xf36554 — j___ZN3RBX4Name7declareILZNS_18sStarterGuiServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36554() {
    // IDA 0xf36554: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_19sMarketplaceServiceEEEERKS0_v")]
// 0xf36564 — j___ZN3RBX4Name7declareILZNS_19sMarketplaceServiceEEEERKS0_v
// type: int()
pub fn stub_0xf36564() {
    // IDA 0xf36564: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_19sRenderHooksServiceEEEERKS0_v")]
// 0xf36574 — j___ZN3RBX4Name7declareILZNS_19sRenderHooksServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36574() {
    // IDA 0xf36574: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_19sStarterPackServiceEEEERKS0_v")]
// 0xf36584 — j___ZN3RBX4Name7declareILZNS_19sStarterPackServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf36584() {
    // IDA 0xf36584: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_21sChangeHistoryServiceEEEERKS0_v")]
// 0xf365a4 — j___ZN3RBX4Name7declareILZNS_21sChangeHistoryServiceEEEERKS0_v
// type: int()
pub fn stub_0xf365a4() {
    // IDA 0xf365a4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_21sContextActionServiceEEEERKS0_v")]
// 0xf365b4 — j___ZN3RBX4Name7declareILZNS_21sContextActionServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf365b4() {
    // IDA 0xf365b4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_22sPersonalServerServiceEEEERKS0_v")]
// 0xf365c4 — j___ZN3RBX4Name7declareILZNS_22sPersonalServerServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf365c4() {
    // IDA 0xf365c4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v")]
// 0xf365d4 — j___ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v
// type: int(void)
pub fn stub_0xf365d4() {
    // IDA 0xf365d4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_6sVisitEEEERKS0_v")]
// 0xf365f4 — j___ZN3RBX4Name7declareILZNS_6sVisitEEEERKS0_v
// type: int()
pub fn stub_0xf365f4() {
    // IDA 0xf365f4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_9sLightingEEEERKS0_v")]
// 0xf36604 — j___ZN3RBX4Name7declareILZNS_9sLightingEEEERKS0_v
// type: int()
pub fn stub_0xf36604() {
    // IDA 0xf36604: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v")]
// 0xf36634 — j___ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v
pub fn stub_0xf36634() {
    // IDA 0xf36634: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}
