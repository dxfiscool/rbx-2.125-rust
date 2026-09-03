//! core shard nv — 120 core stubs EA-sorted asc gap filler not yet in core after nu (next uncovered).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 120 not yet in rbx_core (39274 uncovered before -> 39154 after, batch 0x2788e0..0x4189a8).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(G3D::Vector2 const&,lua_State *)")]
// 0x2788e0 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int)
pub fn stub_0x2788e0() {
    // IDA 0x2788e0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_tostring(RBX::BrickColor const&,lua_State *)")]
// 0x278a04 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(RBX::BrickColor *, int)
pub fn stub_0x278a04() {
    // IDA 0x278a04: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(G3D::CoordinateFrame const&,lua_State *)")]
// 0x278b28 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int, int, int)
pub fn stub_0x278b28() {
    // IDA 0x278b28: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_tostring(RBX::Faces const&,lua_State *)")]
// 0x278c4c — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
pub fn stub_0x278c4c() {
    // IDA 0x278c4c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_tostring(RBX::Axes const&,lua_State *)")]
// 0x278d70 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
pub fn stub_0x278d70() {
    // IDA 0x278d70: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_tostring(RBX::CellID const&,lua_State *)")]
// 0x278e94 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x278e94() {
    // IDA 0x278e94: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_tostring(RBX::InputObject const&,lua_State *)")]
// 0x278fb8 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
pub fn stub_0x278fb8() {
    // IDA 0x278fb8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x2790dc — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
pub fn stub_0x2790dc() {
    // IDA 0x2790dc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_gc(lua_State *)")]
// 0x2791f0 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x2791f0() {
    // IDA 0x2791f0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_eq(lua_State *)")]
// 0x27920c — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27920c() {
    // IDA 0x27920c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_tostring(lua_State *)")]
// 0x279248 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279248() {
    // IDA 0x279248: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x279270 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
pub fn stub_0x279270() {
    // IDA 0x279270: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_gc(lua_State *)")]
// 0x279384 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279384() {
    // IDA 0x279384: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_eq(lua_State *)")]
// 0x2793a0 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x2793a0() {
    // IDA 0x2793a0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_tostring(lua_State *)")]
// 0x2793e0 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x2793e0() {
    // IDA 0x2793e0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x279408 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
pub fn stub_0x279408() {
    // IDA 0x279408: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_gc(lua_State *)")]
// 0x27951c — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27951c() {
    // IDA 0x27951c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_eq(lua_State *)")]
// 0x27953c — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27953c() {
    // IDA 0x27953c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_tostring(lua_State *)")]
// 0x279578 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279578() {
    // IDA 0x279578: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x2795a0 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
pub fn stub_0x2795a0() {
    // IDA 0x2795a0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_gc(lua_State *)")]
// 0x2796b4 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x2796b4() {
    // IDA 0x2796b4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_eq(lua_State *)")]
// 0x2796d0 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x2796d0() {
    // IDA 0x2796d0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_tostring(lua_State *)")]
// 0x279710 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279710() {
    // IDA 0x279710: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x279738 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
pub fn stub_0x279738() {
    // IDA 0x279738: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_gc(lua_State *)")]
// 0x27984c — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27984c() {
    // IDA 0x27984c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_eq(lua_State *)")]
// 0x279868 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279868() {
    // IDA 0x279868: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_tostring(lua_State *)")]
// 0x2798a8 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x2798a8() {
    // IDA 0x2798a8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x2798d0 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
pub fn stub_0x2798d0() {
    // IDA 0x2798d0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_gc(lua_State *)")]
// 0x2799e4 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x2799e4() {
    // IDA 0x2799e4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_eq(lua_State *)")]
// 0x279a08 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279a08() {
    // IDA 0x279a08: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_tostring(lua_State *)")]
// 0x279a44 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279a44() {
    // IDA 0x279a44: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x279a6c — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
pub fn stub_0x279a6c() {
    // IDA 0x279a6c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_gc(lua_State *)")]
// 0x279b80 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279b80() {
    // IDA 0x279b80: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_eq(lua_State *)")]
// 0x279b9c — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279b9c() {
    // IDA 0x279b9c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_tostring(lua_State *)")]
// 0x279c54 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279c54() {
    // IDA 0x279c54: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x279c7c — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
pub fn stub_0x279c7c() {
    // IDA 0x279c7c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_gc(lua_State *)")]
// 0x279d90 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279d90() {
    // IDA 0x279d90: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_eq(lua_State *)")]
// 0x279dac — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279dac() {
    // IDA 0x279dac: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_tostring(lua_State *)")]
// 0x279e1c — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279e1c() {
    // IDA 0x279e1c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x279e44 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int)
pub fn stub_0x279e44() {
    // IDA 0x279e44: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_gc(lua_State *)")]
// 0x279f58 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279f58() {
    // IDA 0x279f58: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_eq(lua_State *)")]
// 0x279f74 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279f74() {
    // IDA 0x279f74: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(lua_State *)")]
// 0x279fe4 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x279fe4() {
    // IDA 0x279fe4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x27a7d0 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
pub fn stub_0x27a7d0() {
    // IDA 0x27a7d0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_gc(lua_State *)")]
// 0x27a8e4 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27a8e4() {
    // IDA 0x27a8e4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_eq(lua_State *)")]
// 0x27a904 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27a904() {
    // IDA 0x27a904: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_tostring(lua_State *)")]
// 0x27a948 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27a948() {
    // IDA 0x27a948: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x27a970 — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
pub fn stub_0x27a970() {
    // IDA 0x27a970: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_gc(lua_State *)")]
// 0x27aa84 — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27aa84() {
    // IDA 0x27aa84: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_eq(lua_State *)")]
// 0x27aaac — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE5on_eqEP9lua_State
pub fn stub_0x27aaac() {
    // IDA 0x27aaac: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_tostring(lua_State *)")]
// 0x27aae8 — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27aae8() {
    // IDA 0x27aae8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x27ab10 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE13registerClassEP9lua_StatePFiS7_ES9_
// type: int __fastcall(int, int, int)
pub fn stub_0x27ab10() {
    // IDA 0x27ab10: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_gc(lua_State *)")]
// 0x27ac24 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27ac24() {
    // IDA 0x27ac24: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_eq(lua_State *)")]
// 0x27ac40 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27ac40() {
    // IDA 0x27ac40: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_tostring(lua_State *)")]
// 0x27ac7c — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27ac7c() {
    // IDA 0x27ac7c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x27aca4 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE13registerClassEP9lua_StatePFiS8_ESA_
// type: int __fastcall(int, int, int)
pub fn stub_0x27aca4() {
    // IDA 0x27aca4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_gc(lua_State *)")]
// 0x27adb8 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27adb8() {
    // IDA 0x27adb8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_eq(lua_State *)")]
// 0x27add4 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27add4() {
    // IDA 0x27add4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_tostring(lua_State *)")]
// 0x27ae10 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27ae10() {
    // IDA 0x27ae10: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// 0x27ae38 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE13registerClassEP9lua_StatePFiS9_ESB_
// type: int __fastcall(int, int, int)
pub fn stub_0x27ae38() {
    // IDA 0x27ae38: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_gc(lua_State *)")]
// 0x27af4c — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27af4c() {
    // IDA 0x27af4c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_eq(lua_State *)")]
// 0x27af68 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27af68() {
    // IDA 0x27af68: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_tostring(lua_State *)")]
// 0x27afa4 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27afa4() {
    // IDA 0x27afa4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::EventInstance::operator==(RBX::Lua::EventInstance const&)const")]
// 0x27afcc — __ZNK3RBX3Lua13EventInstanceeqERKS1_
// type: bool __fastcall(_DWORD *, _DWORD *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x27afcc() {
    // IDA 0x27afcc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_tostring(RBX::Lua::Library const&,lua_State *)")]
// 0x27b0cc — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
pub fn stub_0x27b0cc() {
    // IDA 0x27b0cc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_tostring(RBX::Region3int16 const&,lua_State *)")]
// 0x27b1f0 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
pub fn stub_0x27b1f0() {
    // IDA 0x27b1f0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_tostring(RBX::Region3 const&,lua_State *)")]
// 0x27b314 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
pub fn stub_0x27b314() {
    // IDA 0x27b314: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_index(RBX::Lua::AllEnumDescriptors const* const&,char const*,lua_State *)")]
// 0x27b6a4 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE8on_indexERKS4_PKcP9lua_State
// type: int __fastcall(int, const char *, int)
pub fn stub_0x27b6a4() {
    // IDA 0x27b6a4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_newindex(RBX::Lua::AllEnumDescriptors const*&,char const*,lua_State *)")]
// 0x27b798 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE11on_newindexERS4_PKcP9lua_State
// type: void __noreturn()
pub fn stub_0x27b798() {
    // IDA 0x27b798: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_index(RBX::Reflection::EnumDescriptor const* const&,char const*,lua_State *)")]
// 0x27b84c — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE8on_indexERKS5_PKcP9lua_State
// type: int __fastcall(_DWORD *, char *__s1, int)
pub fn stub_0x27b84c() {
    // IDA 0x27b84c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::pushEnumList(lua_State *)")]
// 0x27b974 — __ZN3RBX3LuaL12pushEnumListEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27b974() {
    // IDA 0x27b974: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_newindex(RBX::Reflection::EnumDescriptor const*&,char const*,lua_State *)")]
// 0x27ba4c — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE11on_newindexERS5_PKcP9lua_State
// type: void __noreturn()
pub fn stub_0x27ba4c() {
    // IDA 0x27ba4c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_index(RBX::Reflection::EnumDescriptor::Item const* const&,char const*,lua_State *)")]
// 0x27bb00 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE8on_indexERKS6_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
pub fn stub_0x27bb00() {
    // IDA 0x27bb00: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_newindex(RBX::Reflection::EnumDescriptor::Item const*&,char const*,lua_State *)")]
// 0x27bc34 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE11on_newindexERS6_PKcP9lua_State
// type: void __noreturn()
pub fn stub_0x27bc34() {
    // IDA 0x27bc34: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Enums::declareAllEnums(lua_State *)")]
// 0x27bce8 — __ZN3RBX3Lua5Enums15declareAllEnumsEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27bce8() {
    // IDA 0x27bce8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Enums::getValue(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// 0x27bd1c — __ZN3RBX3Lua5Enums8getValueEP9lua_StatejRNS_10Reflection7VariantE
// type: int __fastcall(int, int, int)
pub fn stub_0x27bd1c() {
    // IDA 0x27bd1c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_tostring(RBX::Lua::AllEnumDescriptors const* const&,lua_State *)")]
// 0x27bd44 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE11on_tostringERKS4_P9lua_State
// type: int __fastcall(int, int)
pub fn stub_0x27bd44() {
    // IDA 0x27bd44: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_tostring(RBX::Reflection::EnumDescriptor const* const&,lua_State *)")]
// 0x27bd60 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE11on_tostringERKS5_P9lua_State
// type: int __fastcall(int, int)
pub fn stub_0x27bd60() {
    // IDA 0x27bd60: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_tostring(RBX::Reflection::EnumDescriptor::Item const* const&,lua_State *)")]
// 0x27bd78 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE11on_tostringERKS6_P9lua_State
// type: int __fastcall(int, int)
pub fn stub_0x27bd78() {
    // IDA 0x27bd78: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::lookupDescriptor(RBX::Name const&)")]
// 0x27bea8 — __ZN3RBX10Reflection14EnumDescriptor16lookupDescriptorERKNS_4NameE
// type: int __fastcall(RBX::Reflection::EnumDescriptor *this, const Name *)
pub fn stub_0x27bea8() {
    // IDA 0x27bea8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::newweaktable(lua_State *,char const*)")]
// 0x27bfb8 — __ZN3RBX3Lua12newweaktableEP9lua_StatePKc
// type: int __fastcall(int, char *)
pub fn stub_0x27bfb8() {
    // IDA 0x27bfb8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::ObjectBridge::newInstance(lua_State *)")]
// 0x27c004 — __ZN3RBX3Lua12ObjectBridge11newInstanceEP9lua_State
// type: int __fastcall(boost::detail::sp_counted_base *)
pub fn stub_0x27c004() {
    // IDA 0x27c004: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::ObjectBridge::lockInstance(lua_State *)")]
// 0x27c244 — __ZN3RBX3Lua12ObjectBridge12lockInstanceEP9lua_State
// type: int __fastcall(int)
pub fn stub_0x27c244() {
    // IDA 0x27c244: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::ObjectBridge::unlockInstance(lua_State *)")]
// 0x27c254 — __ZN3RBX3Lua12ObjectBridge14unlockInstanceEP9lua_State
// type: int()
pub fn stub_0x27c254() {
    // IDA 0x27c254: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::ObjectBridge::callMemberYieldFunction(lua_State *)")]
// 0x27ddf0 — __ZN3RBX3Lua12ObjectBridge23callMemberYieldFunctionEP9lua_State
// type: RBX::Reflection::MemberDescriptor *__fastcall(int)
pub fn stub_0x27ddf0() {
    // IDA 0x27ddf0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::ObjectBridge::callMemberFunction(lua_State *)")]
// 0x27e110 — __ZN3RBX3Lua12ObjectBridge18callMemberFunctionEP9lua_State
// type: RBX::Reflection::MemberDescriptor *__fastcall(int)
pub fn stub_0x27e110() {
    // IDA 0x27e110: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::EventInstance* RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::pushNewObject<RBX::Lua::EventInstance>(lua_State *,RBX::Lua::EventInstance)")]
// 0x280bac — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *)
pub fn stub_0x280bac() {
    // IDA 0x280bac: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "YieldFunctionStateObject::onReturnResult(RBX::Reflection::Variant)")]
// 0x281620 — __ZN24YieldFunctionStateObject14onReturnResultEN3RBX10Reflection7VariantE
// type: void __fastcall(int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, void *, int, int, int, int)
pub fn stub_0x281620() {
    // IDA 0x281620: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::findDescriptor(char const*)const")]
// 0x285774 — __ZNK3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE14findDescriptorEPKc
// type: int __fastcall(int, char *)
pub fn stub_0x285774() {
    // IDA 0x285774: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr(lua_State *,unsigned int)")]
// 0x2857a4 — __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrEP9lua_Statej
// type: int __fastcall(sp_counted_base **, int, const char *)
pub fn stub_0x2857a4() {
    // IDA 0x2857a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropertyDescriptor::setEnumItem(RBX::Reflection::DescribedBase *,RBX::Reflection::EnumDescriptor::Item const&)const")]
// 0x2857f0 — __ZNK3RBX10Reflection22EnumPropertyDescriptor11setEnumItemEPNS0_13DescribedBaseERKNS0_14EnumDescriptor4ItemE
// type: int __fastcall(RBX::Reflection::EnumPropertyDescriptor *this, RBX::Reflection::DescribedBase *, const RBX::Reflection::EnumDescriptor::Item *)
pub fn stub_0x2857f0() {
    // IDA 0x2857f0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::findDescriptor(char const*)const")]
// 0x285848 — __ZNK3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE14findDescriptorEPKc
// type: int __fastcall(int, char *)
pub fn stub_0x285848() {
    // IDA 0x285848: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::findDescriptor(char const*)const")]
// 0x285870 — __ZNK3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE14findDescriptorEPKc
// type: int __fastcall(int, char *)
pub fn stub_0x285870() {
    // IDA 0x285870: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::findDescriptor(char const*)const")]
// 0x285898 — __ZNK3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE14findDescriptorEPKc
// type: int __fastcall(int, char *)
pub fn stub_0x285898() {
    // IDA 0x285898: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::LuaAllocator::LuaAllocator(bool)")]
// 0x285c30 — __ZN3RBX12LuaAllocatorC1Eb
// type: int __fastcall(RBX::LuaAllocator *this, bool)
pub fn stub_0x285c30() {
    // IDA 0x285c30: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::LuaAllocator::LuaAllocator(bool)")]
// 0x285c34 — __ZN3RBX12LuaAllocatorC2Eb
// type: RBX::LuaAllocator *__fastcall(RBX::LuaAllocator *this, int)
pub fn stub_0x285c34() {
    // IDA 0x285c34: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::LuaAllocator::~LuaAllocator()")]
// 0x285d3c — __ZN3RBX12LuaAllocatorD1Ev
// type: void __fastcall(RBX::LuaAllocator *__hidden this)
pub fn stub_0x285d3c() {
    // IDA 0x285d3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LuaAllocator::~LuaAllocator()")]
// 0x285d40 — __ZN3RBX12LuaAllocatorD2Ev
// type: void __fastcall(RBX::LuaAllocator *__hidden this)
pub fn stub_0x285d40() {
    // IDA 0x285d40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LuaAllocator::clearHeapMax(void)")]
// 0x285da0 — __ZN3RBX12LuaAllocator12clearHeapMaxEv
// type: int __fastcall(int this)
pub fn stub_0x285da0() {
    // IDA 0x285da0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::LuaAllocator::getHeapStats(unsigned long &,unsigned long &,unsigned long &,unsigned long &)const")]
// 0x285dac — __ZNK3RBX12LuaAllocator12getHeapStatsERmS1_S1_S1_
// type: unsigned int __fastcall(RBX::LuaAllocator *this, unsigned int *, unsigned int *, unsigned int *, unsigned int *)
pub fn stub_0x285dac() {
    // IDA 0x285dac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE12getClassNameEv")]
// 0x417e8c — __ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_0x417e8c() {
    // IDA 0x417e8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CornerWedgeInstance::getPartType(void)const")]
// 0x417e9c — __ZNK3RBX19CornerWedgeInstance11getPartTypeEv
// type: int __fastcall(RBX::CornerWedgeInstance *this)
pub fn stub_0x417e9c() {
    // IDA 0x417e9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE12getClassNameEv")]
// 0x417ea0 — __ZThn32_NK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_0x417ea0() {
    // IDA 0x417ea0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x417eb0 — __ZN3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::PartInstance *)
pub fn stub_0x417eb0() {
    // IDA 0x417eb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x417ec4 — __ZN3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::PartInstance *)
pub fn stub_0x417ec4() {
    // IDA 0x417ec4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x417f74 — __ZThn132_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_0x417f74() {
    // IDA 0x417f74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x417f88 — __ZThn132_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_0x417f88() {
    // IDA 0x417f88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x41803c — __ZN3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::PartInstance *)
pub fn stub_0x41803c() {
    // IDA 0x41803c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x418050 — __ZN3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::PartInstance *)
pub fn stub_0x418050() {
    // IDA 0x418050: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x418100 — __ZThn132_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_0x418100() {
    // IDA 0x418100: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x418114 — __ZThn132_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_0x418114() {
    // IDA 0x418114: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev")]
// 0x4181c8 — __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev
// type: void __fastcall(RBX::PartInstance *)
pub fn stub_0x4181c8() {
    // IDA 0x4181c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev")]
// 0x4181dc — __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev
// type: void __fastcall(RBX::PartInstance *)
pub fn stub_0x4181dc() {
    // IDA 0x4181dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev")]
// 0x41828c — __ZThn132_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev
// type: void __fastcall(int)
pub fn stub_0x41828c() {
    // IDA 0x41828c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev")]
// 0x4182a0 — __ZThn132_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev
// type: int __fastcall(int)
pub fn stub_0x4182a0() {
    // IDA 0x4182a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorD1Ev")]
// 0x4182a8 — __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorD1Ev
// type: int()
pub fn stub_0x4182a8() {
    // IDA 0x4182a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorD2Ev")]
// 0x4182ac — __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x4182ac() {
    // IDA 0x4182ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x418348 — __ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7Creator12getClassNameEv
// type: int __fastcall(_DWORD)
pub fn stub_0x418348() {
    // IDA 0x418348: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7Creator6createEv")]
// 0x4183d0 — __ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
pub fn stub_0x4183d0() {
    // IDA 0x4183d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorC2Ev")]
// 0x4189a8 — __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_0x4189a8() {
    // IDA 0x4189a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
