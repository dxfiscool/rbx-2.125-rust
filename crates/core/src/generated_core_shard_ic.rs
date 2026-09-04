//! core shard IC — 100 core stubs EA-sorted, continuation after IB 0x406f78 (EA-sorted ascending, next 100 uncovered).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted, next 100 uncovered after 0x406f78.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "__ZNK3RBX5NamedINS_8PartToolELZNS_12sDropperToolEEE7getNameEv")]
// 0x407570 — __ZNK3RBX5NamedINS_8PartToolELZNS_12sDropperToolEEE7getNameEv
pub fn stub_407570() {
    // IDA 0x407570: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sDropperToolEEEERKS0_v")]
// 0x407868 — __ZN3RBX4Name7declareILZNS_12sDropperToolEEEERKS0_v
pub fn stub_407868() {
    // IDA 0x407868: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sDropperToolEEEEvv")]
// 0x4078ac — __ZN3RBX4Name13callDoDeclareILZNS_12sDropperToolEEEEvv
pub fn stub_4078ac() {
    // IDA 0x4078ac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sDropperToolEEEERKS0_v")]
// 0x4078b0 — __ZN3RBX4Name9doDeclareILZNS_12sDropperToolEEEERKS0_v
pub fn stub_4078b0() {
    // IDA 0x4078b0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_8PartToolELZNS_13sMaterialToolEEE7getNameEv")]
// 0x407ea8 — __ZNK3RBX5NamedINS_8PartToolELZNS_13sMaterialToolEEE7getNameEv
pub fn stub_407ea8() {
    // IDA 0x407ea8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v")]
// 0x408268 — __ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v
pub fn stub_408268() {
    // IDA 0x408268: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sMaterialToolEEEEvv")]
// 0x4082ac — __ZN3RBX4Name13callDoDeclareILZNS_13sMaterialToolEEEEvv
pub fn stub_4082ac() {
    // IDA 0x4082ac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v")]
// 0x4082b0 — __ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v
pub fn stub_4082b0() {
    // IDA 0x4082b0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_8PartToolELZNS_9sFillToolEEE7getNameEv")]
// 0x4088a8 — __ZNK3RBX5NamedINS_8PartToolELZNS_9sFillToolEEE7getNameEv
pub fn stub_4088a8() {
    // IDA 0x4088a8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFillToolEEEERKS0_v")]
// 0x408c68 — __ZN3RBX4Name7declareILZNS_9sFillToolEEEERKS0_v
pub fn stub_408c68() {
    // IDA 0x408c68: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sFillToolEEEEvv")]
// 0x408cac — __ZN3RBX4Name13callDoDeclareILZNS_9sFillToolEEEEvv
pub fn stub_408cac() {
    // IDA 0x408cac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFillToolEEEERKS0_v")]
// 0x408cb0 — __ZN3RBX4Name9doDeclareILZNS_9sFillToolEEEERKS0_v
pub fn stub_408cb0() {
    // IDA 0x408cb0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_9ModelToolELZNS_9sLockToolEEE7getNameEv")]
// 0x4092a8 — __ZNK3RBX5NamedINS_9ModelToolELZNS_9sLockToolEEE7getNameEv
pub fn stub_4092a8() {
    // IDA 0x4092a8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sLockToolEEEERKS0_v")]
// 0x4097b4 — __ZN3RBX4Name7declareILZNS_9sLockToolEEEERKS0_v
pub fn stub_4097b4() {
    // IDA 0x4097b4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sLockToolEEEEvv")]
// 0x4097f8 — __ZN3RBX4Name13callDoDeclareILZNS_9sLockToolEEEEvv
pub fn stub_4097f8() {
    // IDA 0x4097f8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sLockToolEEEERKS0_v")]
// 0x4097fc — __ZN3RBX4Name9doDeclareILZNS_9sLockToolEEEERKS0_v
pub fn stub_4097fc() {
    // IDA 0x4097fc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_9ModelToolELZNS_11sAnchorToolEEE7getNameEv")]
// 0x409dfc — __ZNK3RBX5NamedINS_9ModelToolELZNS_11sAnchorToolEEE7getNameEv
pub fn stub_409dfc() {
    // IDA 0x409dfc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v")]
// 0x40a308 — __ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v
pub fn stub_40a308() {
    // IDA 0x40a308: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sAnchorToolEEEEvv")]
// 0x40a34c — __ZN3RBX4Name13callDoDeclareILZNS_11sAnchorToolEEEEvv
pub fn stub_40a34c() {
    // IDA 0x40a34c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v")]
// 0x40a350 — __ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v
pub fn stub_40a350() {
    // IDA 0x40a350: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_21sSmoothNoOutlinesToolEEE7getNameEv")]
// 0x40a948 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_21sSmoothNoOutlinesToolEEE7getNameEv
pub fn stub_40a948() {
    // IDA 0x40a948: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v")]
// 0x40ad08 — __ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v
pub fn stub_40ad08() {
    // IDA 0x40ad08: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_21sSmoothNoOutlinesToolEEEEvv")]
// 0x40ad4c — __ZN3RBX4Name13callDoDeclareILZNS_21sSmoothNoOutlinesToolEEEEvv
pub fn stub_40ad4c() {
    // IDA 0x40ad4c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v")]
// 0x40ad50 — __ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v
pub fn stub_40ad50() {
    // IDA 0x40ad50: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_19sOscillateMotorToolEEE7getNameEv")]
// 0x40b348 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_19sOscillateMotorToolEEE7getNameEv
pub fn stub_40b348() {
    // IDA 0x40b348: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v")]
// 0x40b640 — __ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v
pub fn stub_40b640() {
    // IDA 0x40b640: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sOscillateMotorToolEEEEvv")]
// 0x40b684 — __ZN3RBX4Name13callDoDeclareILZNS_19sOscillateMotorToolEEEEvv
pub fn stub_40b684() {
    // IDA 0x40b684: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v")]
// 0x40b688 — __ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v
pub fn stub_40b688() {
    // IDA 0x40b688: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sLeftMotorToolEEE7getNameEv")]
// 0x40bc80 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sLeftMotorToolEEE7getNameEv
pub fn stub_40bc80() {
    // IDA 0x40bc80: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v")]
// 0x40bf78 — __ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v
pub fn stub_40bf78() {
    // IDA 0x40bf78: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sLeftMotorToolEEEEvv")]
// 0x40bfbc — __ZN3RBX4Name13callDoDeclareILZNS_14sLeftMotorToolEEEEvv
pub fn stub_40bfbc() {
    // IDA 0x40bfbc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v")]
// 0x40bfc0 — __ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v
pub fn stub_40bfc0() {
    // IDA 0x40bfc0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_15sRightMotorToolEEE7getNameEv")]
// 0x40c5b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_15sRightMotorToolEEE7getNameEv
pub fn stub_40c5b8() {
    // IDA 0x40c5b8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v")]
// 0x40c978 — __ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v
pub fn stub_40c978() {
    // IDA 0x40c978: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sRightMotorToolEEEEvv")]
// 0x40c9bc — __ZN3RBX4Name13callDoDeclareILZNS_15sRightMotorToolEEEEvv
pub fn stub_40c9bc() {
    // IDA 0x40c9bc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v")]
// 0x40c9c0 — __ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v
pub fn stub_40c9c0() {
    // IDA 0x40c9c0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sHingeToolEEE7getNameEv")]
// 0x40cfb8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sHingeToolEEE7getNameEv
pub fn stub_40cfb8() {
    // IDA 0x40cfb8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sHingeToolEEEERKS0_v")]
// 0x40d378 — __ZN3RBX4Name7declareILZNS_10sHingeToolEEEERKS0_v
pub fn stub_40d378() {
    // IDA 0x40d378: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sHingeToolEEEEvv")]
// 0x40d3bc — __ZN3RBX4Name13callDoDeclareILZNS_10sHingeToolEEEEvv
pub fn stub_40d3bc() {
    // IDA 0x40d3bc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sHingeToolEEEERKS0_v")]
// 0x40d3c0 — __ZN3RBX4Name9doDeclareILZNS_10sHingeToolEEEERKS0_v
pub fn stub_40d3c0() {
    // IDA 0x40d3c0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sUniversalToolEEE7getNameEv")]
// 0x40d9b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sUniversalToolEEE7getNameEv
pub fn stub_40d9b8() {
    // IDA 0x40d9b8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sUniversalToolEEEERKS0_v")]
// 0x40dd78 — __ZN3RBX4Name7declareILZNS_14sUniversalToolEEEERKS0_v
pub fn stub_40dd78() {
    // IDA 0x40dd78: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sUniversalToolEEEEvv")]
// 0x40ddbc — __ZN3RBX4Name13callDoDeclareILZNS_14sUniversalToolEEEEvv
pub fn stub_40ddbc() {
    // IDA 0x40ddbc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sUniversalToolEEEERKS0_v")]
// 0x40ddc0 — __ZN3RBX4Name9doDeclareILZNS_14sUniversalToolEEEERKS0_v
pub fn stub_40ddc0() {
    // IDA 0x40ddc0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sInletToolEEE7getNameEv")]
// 0x40e3b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sInletToolEEE7getNameEv
pub fn stub_40e3b8() {
    // IDA 0x40e3b8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sInletToolEEEERKS0_v")]
// 0x40e778 — __ZN3RBX4Name7declareILZNS_10sInletToolEEEERKS0_v
pub fn stub_40e778() {
    // IDA 0x40e778: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sInletToolEEEEvv")]
// 0x40e7bc — __ZN3RBX4Name13callDoDeclareILZNS_10sInletToolEEEEvv
pub fn stub_40e7bc() {
    // IDA 0x40e7bc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sInletToolEEEERKS0_v")]
// 0x40e7c0 — __ZN3RBX4Name9doDeclareILZNS_10sInletToolEEEERKS0_v
pub fn stub_40e7c0() {
    // IDA 0x40e7c0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sStudsToolEEE7getNameEv")]
// 0x40edb8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sStudsToolEEE7getNameEv
pub fn stub_40edb8() {
    // IDA 0x40edb8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sStudsToolEEEERKS0_v")]
// 0x40f178 — __ZN3RBX4Name7declareILZNS_10sStudsToolEEEERKS0_v
pub fn stub_40f178() {
    // IDA 0x40f178: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sStudsToolEEEEvv")]
// 0x40f1bc — __ZN3RBX4Name13callDoDeclareILZNS_10sStudsToolEEEEvv
pub fn stub_40f1bc() {
    // IDA 0x40f1bc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sStudsToolEEEERKS0_v")]
// 0x40f1c0 — __ZN3RBX4Name9doDeclareILZNS_10sStudsToolEEEERKS0_v
pub fn stub_40f1c0() {
    // IDA 0x40f1c0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sWeldToolEEE7getNameEv")]
// 0x40f7b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sWeldToolEEE7getNameEv
pub fn stub_40f7b8() {
    // IDA 0x40f7b8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sWeldToolEEEERKS0_v")]
// 0x40fb78 — __ZN3RBX4Name7declareILZNS_9sWeldToolEEEERKS0_v
pub fn stub_40fb78() {
    // IDA 0x40fb78: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sWeldToolEEEEvv")]
// 0x40fbbc — __ZN3RBX4Name13callDoDeclareILZNS_9sWeldToolEEEEvv
pub fn stub_40fbbc() {
    // IDA 0x40fbbc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sWeldToolEEEERKS0_v")]
// 0x40fbc0 — __ZN3RBX4Name9doDeclareILZNS_9sWeldToolEEEERKS0_v
pub fn stub_40fbc0() {
    // IDA 0x40fbc0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sGlueToolEEE7getNameEv")]
// 0x4101b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sGlueToolEEE7getNameEv
pub fn stub_4101b8() {
    // IDA 0x4101b8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sGlueToolEEEERKS0_v")]
// 0x410578 — __ZN3RBX4Name7declareILZNS_9sGlueToolEEEERKS0_v
pub fn stub_410578() {
    // IDA 0x410578: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sGlueToolEEEEvv")]
// 0x4105bc — __ZN3RBX4Name13callDoDeclareILZNS_9sGlueToolEEEEvv
pub fn stub_4105bc() {
    // IDA 0x4105bc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGlueToolEEEERKS0_v")]
// 0x4105c0 — __ZN3RBX4Name9doDeclareILZNS_9sGlueToolEEEERKS0_v
pub fn stub_4105c0() {
    // IDA 0x4105c0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sFlatToolEEE7getNameEv")]
// 0x410bb8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sFlatToolEEE7getNameEv
pub fn stub_410bb8() {
    // IDA 0x410bb8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFlatToolEEEERKS0_v")]
// 0x410f78 — __ZN3RBX4Name7declareILZNS_9sFlatToolEEEERKS0_v
pub fn stub_410f78() {
    // IDA 0x410f78: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sFlatToolEEEEvv")]
// 0x410fbc — __ZN3RBX4Name13callDoDeclareILZNS_9sFlatToolEEEEvv
pub fn stub_410fbc() {
    // IDA 0x410fbc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFlatToolEEEERKS0_v")]
// 0x410fc0 — __ZN3RBX4Name9doDeclareILZNS_9sFlatToolEEEERKS0_v
pub fn stub_410fc0() {
    // IDA 0x410fc0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_14sAdvRotateToolEEE7getNameEv")]
// 0x4119fc — __ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_14sAdvRotateToolEEE7getNameEv
pub fn stub_4119fc() {
    // IDA 0x4119fc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sAdvRotateToolEEEERKS0_v")]
// 0x41208c — __ZN3RBX4Name7declareILZNS_14sAdvRotateToolEEEERKS0_v
pub fn stub_41208c() {
    // IDA 0x41208c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sAdvRotateToolEEEEvv")]
// 0x4120d0 — __ZN3RBX4Name13callDoDeclareILZNS_14sAdvRotateToolEEEEvv
pub fn stub_4120d0() {
    // IDA 0x4120d0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sAdvRotateToolEEEERKS0_v")]
// 0x4120d4 — __ZN3RBX4Name9doDeclareILZNS_14sAdvRotateToolEEEERKS0_v
pub fn stub_4120d4() {
    // IDA 0x4120d4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_12sAdvMoveToolEEE7getNameEv")]
// 0x4126e4 — __ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_12sAdvMoveToolEEE7getNameEv
pub fn stub_4126e4() {
    // IDA 0x4126e4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sAdvMoveToolEEEERKS0_v")]
// 0x412b58 — __ZN3RBX4Name7declareILZNS_12sAdvMoveToolEEEERKS0_v
pub fn stub_412b58() {
    // IDA 0x412b58: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sAdvMoveToolEEEEvv")]
// 0x412b9c — __ZN3RBX4Name13callDoDeclareILZNS_12sAdvMoveToolEEEEvv
pub fn stub_412b9c() {
    // IDA 0x412b9c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sAdvMoveToolEEEERKS0_v")]
// 0x412ba0 — __ZN3RBX4Name9doDeclareILZNS_12sAdvMoveToolEEEERKS0_v
pub fn stub_412ba0() {
    // IDA 0x412ba0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_12AdvArrowToolELZNS_19sMoveResizeJoinToolEEE7getNameEv")]
// 0x4132b0 — __ZNK3RBX5NamedINS_12AdvArrowToolELZNS_19sMoveResizeJoinToolEEE7getNameEv
pub fn stub_4132b0() {
    // IDA 0x4132b0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sMoveResizeJoinToolEEEERKS0_v")]
// 0x41366c — __ZN3RBX4Name7declareILZNS_19sMoveResizeJoinToolEEEERKS0_v
pub fn stub_41366c() {
    // IDA 0x41366c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sMoveResizeJoinToolEEEEvv")]
// 0x4136b0 — __ZN3RBX4Name13callDoDeclareILZNS_19sMoveResizeJoinToolEEEEvv
pub fn stub_4136b0() {
    // IDA 0x4136b0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sMoveResizeJoinToolEEEERKS0_v")]
// 0x4136b4 — __ZN3RBX4Name9doDeclareILZNS_19sMoveResizeJoinToolEEEERKS0_v
pub fn stub_4136b4() {
    // IDA 0x4136b4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX5NamedINS_12AxisToolBaseELZNS_15sAxisRotateToolEEE7getNameEv")]
// 0x413ca8 — __ZNK3RBX5NamedINS_12AxisToolBaseELZNS_15sAxisRotateToolEEE7getNameEv
pub fn stub_413ca8() {
    // IDA 0x413ca8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sAxisRotateToolEEEERKS0_v")]
// 0x4147e8 — __ZN3RBX4Name7declareILZNS_15sAxisRotateToolEEEERKS0_v
pub fn stub_4147e8() {
    // IDA 0x4147e8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sAxisRotateToolEEEEvv")]
// 0x41482c — __ZN3RBX4Name13callDoDeclareILZNS_15sAxisRotateToolEEEEvv
pub fn stub_41482c() {
    // IDA 0x41482c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAxisRotateToolEEEERKS0_v")]
// 0x414830 — __ZN3RBX4Name9doDeclareILZNS_15sAxisRotateToolEEEERKS0_v
pub fn stub_414830() {
    // IDA 0x414830: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_174")]
// 0x4160e4 — __GLOBAL__I_a_174
pub fn stub_4160e4() {
    // IDA 0x4160e4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sConfigurationEEEEvv")]
// 0x4171ac — __ZN3RBX4Name13callDoDeclareILZNS_14sConfigurationEEEEvv
pub fn stub_4171ac() {
    // IDA 0x4171ac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v")]
// 0x4171b0 — __ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v
pub fn stub_4171b0() {
    // IDA 0x4171b0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_175")]
// 0x417744 — __GLOBAL__I_a_175
pub fn stub_417744() {
    // IDA 0x417744: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sCornerWedgeEEEEvv")]
// 0x4188c4 — __ZN3RBX4Name13callDoDeclareILZNS_12sCornerWedgeEEEEvv
pub fn stub_4188c4() {
    // IDA 0x4188c4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v")]
// 0x4188c8 — __ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v
pub fn stub_4188c8() {
    // IDA 0x4188c8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_176")]
// 0x419024 — __GLOBAL__I_a_176
pub fn stub_419024() {
    // IDA 0x419024: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_177")]
// 0x419344 — __GLOBAL__I_a_177
pub fn stub_419344() {
    // IDA 0x419344: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sCoreGuiServiceEEEEvv")]
// 0x43b798 — __ZN3RBX4Name13callDoDeclareILZNS_15sCoreGuiServiceEEEEvv
pub fn stub_43b798() {
    // IDA 0x43b798: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5Stats6sStatsEEEEvv")]
// 0x440ac8 — __ZN3RBX4Name13callDoDeclareILZNS_5Stats6sStatsEEEEvv
pub fn stub_440ac8() {
    // IDA 0x440ac8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sTestServiceEEEERKS0_v")]
// 0x440b8c — __ZN3RBX4Name7declareILZNS_12sTestServiceEEEERKS0_v
pub fn stub_440b8c() {
    // IDA 0x440b8c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sTestServiceEEEEvv")]
// 0x440bd0 — __ZN3RBX4Name13callDoDeclareILZNS_12sTestServiceEEEEvv
pub fn stub_440bd0() {
    // IDA 0x440bd0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sTestServiceEEEERKS0_v")]
// 0x440bd4 — __ZN3RBX4Name9doDeclareILZNS_12sTestServiceEEEERKS0_v
pub fn stub_440bd4() {
    // IDA 0x440bd4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sJointsServiceEEEERKS0_v")]
// 0x441210 — __ZN3RBX4Name7declareILZNS_14sJointsServiceEEEERKS0_v
pub fn stub_441210() {
    // IDA 0x441210: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sJointsServiceEEEERKS0_v")]
// 0x441258 — __ZN3RBX4Name9doDeclareILZNS_14sJointsServiceEEEERKS0_v
pub fn stub_441258() {
    // IDA 0x441258: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sSpawnerServiceEEEERKS0_v")]
// 0x44162c — __ZN3RBX4Name7declareILZNS_15sSpawnerServiceEEEERKS0_v
pub fn stub_44162c() {
    // IDA 0x44162c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sSpawnerServiceEEEEvv")]
// 0x441670 — __ZN3RBX4Name13callDoDeclareILZNS_15sSpawnerServiceEEEEvv
pub fn stub_441670() {
    // IDA 0x441670: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sSpawnerServiceEEEERKS0_v")]
// 0x441674 — __ZN3RBX4Name9doDeclareILZNS_15sSpawnerServiceEEEERKS0_v
pub fn stub_441674() {
    // IDA 0x441674: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sGuiServiceEEEERKS0_v")]
// 0x442888 — __ZN3RBX4Name7declareILZNS_11sGuiServiceEEEERKS0_v
pub fn stub_442888() {
    // IDA 0x442888: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v")]
// 0x4428d0 — __ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v
pub fn stub_4428d0() {
    // IDA 0x4428d0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}
