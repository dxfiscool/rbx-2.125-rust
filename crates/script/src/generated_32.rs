// Auto-generated skeletons for rbx-script — filler EA-sorted after 0x278574 (next 120) [filler EA-sorted ascending earliest gap]
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x278698..0x287d0c | existing 3411 -> 3531 total (filler after 0x278574, EA-sorted ascending)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x278698 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(G3D::Vector2int16 const&,lua_State *)")]
pub fn stub_0x278698(value: &crate::lua::LuaVector2i16, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Vector2int16>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}", value.x, value.y)));
1
}

// 0x2787bc — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(G3D::Vector3 const&,lua_State *)")]
pub fn stub_0x2787bc(value: &crate::lua::LuaVector3, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Vector3>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}", value.x, value.y, value.z)));
1
}

// 0x2788e0 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(G3D::Vector2 const&,lua_State *)")]
pub fn stub_0x2788e0(value: &crate::lua::LuaVector2, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Vector2>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}", value.x, value.y)));
1
}

// 0x278a04 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(RBX::BrickColor *, int)
// was: int __fastcall(RBX::BrickColor *, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_tostring(RBX::BrickColor const&,lua_State *)")]
pub fn stub_0x278a04(value: &crate::lua::LuaBrickColor, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<BrickColor>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(crate::lua::brickcolor_name(value.number).to_owned()));
1
}

// 0x278b28 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int, int, int)
// was: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(G3D::CoordinateFrame const&,lua_State *)")]
pub fn stub_0x278b28(value: &crate::lua::LuaCoordinateFrame, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<CoordinateFrame>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", value.position.x, value.position.y, value.position.z, value.rotation[0][0], value.rotation[0][1], value.rotation[0][2], value.rotation[1][0], value.rotation[1][1], value.rotation[1][2], value.rotation[2][0], value.rotation[2][1], value.rotation[2][2])));
1
}

// 0x278c4c — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_tostring(RBX::Faces const&,lua_State *)")]
pub fn stub_0x278c4c(value: &crate::lua::LuaFaces, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Faces>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("Faces({})", value.bits)));
1
}

// 0x278d70 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_tostring(RBX::Axes const&,lua_State *)")]
pub fn stub_0x278d70(value: &crate::lua::LuaAxes, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Axes>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("Axes({})", value.bits)));
1
}

// 0x278e94 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(struct _Unwind_Exception *lpuexcpt, int)
// was: int __fastcall(struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_tostring(RBX::CellID const&,lua_State *)")]
pub fn stub_0x278e94(value: &crate::lua::LuaCellId, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<CellID>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}", value.x, value.y, value.z)));
1
}

// 0x278fb8 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_tostring(RBX::InputObject const&,lua_State *)")]
pub fn stub_0x278fb8(value: &crate::lua::LuaInputObject, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<InputObject>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("InputObject({})", value.kind)));
1
}

// 0x2790dc — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x2790dc(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x2790dc: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x2791f0 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_gc(lua_State *)")]
pub fn stub_0x2791f0(value: crate::lua::LuaInputObject) {
// Bridge<InputObject>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x27920c — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_eq(lua_State *)")]
pub fn stub_0x27920c(a: &crate::lua::LuaInputObject, b: &crate::lua::LuaInputObject) -> bool {
// Bridge<InputObject>::on_eq — value equality on the payload.
a == b
}

// 0x279248 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_tostring(lua_State *)")]
pub fn stub_0x279248(value: &crate::lua::LuaInputObject, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<InputObject>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("InputObject({})", value.kind)));
1
}

// 0x279270 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x279270(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x279270: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x279384 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_gc(lua_State *)")]
pub fn stub_0x279384(value: crate::lua::LuaAxes) {
// Bridge<Axes>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x2793a0 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_eq(lua_State *)")]
pub fn stub_0x2793a0(a: &crate::lua::LuaAxes, b: &crate::lua::LuaAxes) -> bool {
// Bridge<Axes>::on_eq — value equality on the payload.
a == b
}

// 0x2793e0 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_tostring(lua_State *)")]
pub fn stub_0x2793e0(value: &crate::lua::LuaAxes, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Axes>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("Axes({})", value.bits)));
1
}

// 0x279408 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x279408(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x279408: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x27951c — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_gc(lua_State *)")]
pub fn stub_0x27951c(value: crate::lua::LuaCellId) {
// Bridge<CellID>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x27953c — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_eq(lua_State *)")]
pub fn stub_0x27953c(a: &crate::lua::LuaCellId, b: &crate::lua::LuaCellId) -> bool {
// Bridge<CellID>::on_eq — value equality on the payload.
a == b
}

// 0x279578 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_tostring(lua_State *)")]
pub fn stub_0x279578(value: &crate::lua::LuaCellId, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<CellID>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}", value.x, value.y, value.z)));
1
}

// 0x2795a0 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x2795a0(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x2795a0: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x2796b4 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_gc(lua_State *)")]
pub fn stub_0x2796b4(value: crate::lua::LuaFaces) {
// Bridge<Faces>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x2796d0 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_eq(lua_State *)")]
pub fn stub_0x2796d0(a: &crate::lua::LuaFaces, b: &crate::lua::LuaFaces) -> bool {
// Bridge<Faces>::on_eq — value equality on the payload.
a == b
}

// 0x279710 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_tostring(lua_State *)")]
pub fn stub_0x279710(value: &crate::lua::LuaFaces, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Faces>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("Faces({})", value.bits)));
1
}

// 0x279738 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x279738(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x279738: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x27984c — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_gc(lua_State *)")]
pub fn stub_0x27984c(value: crate::lua::LuaBrickColor) {
// Bridge<BrickColor>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x279868 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_eq(lua_State *)")]
pub fn stub_0x279868(a: &crate::lua::LuaBrickColor, b: &crate::lua::LuaBrickColor) -> bool {
// Bridge<BrickColor>::on_eq — value equality on the payload.
a == b
}

// 0x2798a8 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_tostring(lua_State *)")]
pub fn stub_0x2798a8(value: &crate::lua::LuaBrickColor, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<BrickColor>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(crate::lua::brickcolor_name(value.number).to_owned()));
1
}

// 0x2798d0 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x2798d0(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x2798d0: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x2799e4 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_gc(lua_State *)")]
pub fn stub_0x2799e4(value: crate::lua::LuaRbxRay) {
// Bridge<RbxRay>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x279a08 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_eq(lua_State *)")]
pub fn stub_0x279a08(a: &crate::lua::LuaRbxRay, b: &crate::lua::LuaRbxRay) -> bool {
// Bridge<RbxRay>::on_eq — value equality on the payload.
a == b
}

// 0x279a44 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_tostring(lua_State *)")]
pub fn stub_0x279a44(value: &crate::lua::LuaRbxRay, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<RbxRay>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {} | {}, {}, {}", value.origin.x, value.origin.y, value.origin.z, value.direction.x, value.direction.y, value.direction.z)));
1
}

// 0x279a6c — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x279a6c(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x279a6c: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x279b80 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_gc(lua_State *)")]
pub fn stub_0x279b80(value: crate::lua::LuaRegion3) {
// Bridge<Region3>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x279b9c — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_eq(lua_State *)")]
pub fn stub_0x279b9c(a: &crate::lua::LuaRegion3, b: &crate::lua::LuaRegion3) -> bool {
// Bridge<Region3>::on_eq — value equality on the payload.
a == b
}

// 0x279c54 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_tostring(lua_State *)")]
pub fn stub_0x279c54(value: &crate::lua::LuaRegion3, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Region3>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {} - {}, {}, {}", value.min.x, value.min.y, value.min.z, value.max.x, value.max.y, value.max.z)));
1
}

// 0x279c7c — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x279c7c(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x279c7c: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x279d90 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_gc(lua_State *)")]
pub fn stub_0x279d90(value: crate::lua::LuaRegion3i16) {
// Bridge<Region3int16>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x279dac — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_eq(lua_State *)")]
pub fn stub_0x279dac(a: &crate::lua::LuaRegion3i16, b: &crate::lua::LuaRegion3i16) -> bool {
// Bridge<Region3int16>::on_eq — value equality on the payload.
a == b
}

// 0x279e1c — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_tostring(lua_State *)")]
pub fn stub_0x279e1c(value: &crate::lua::LuaRegion3i16, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Region3int16>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {} - {}, {}, {}", value.min.x, value.min.y, value.min.z, value.max.x, value.max.y, value.max.z)));
1
}

// 0x279e44 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x279e44(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x279e44: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x279f58 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_gc(lua_State *)")]
pub fn stub_0x279f58(value: crate::lua::LuaColor3) {
// Bridge<Color3>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x279f74 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_eq(lua_State *)")]
pub fn stub_0x279f74(a: &crate::lua::LuaColor3, b: &crate::lua::LuaColor3) -> bool {
// Bridge<Color3>::on_eq — value equality on the payload.
a == b
}

// 0x279fe4 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(lua_State *)")]
pub fn stub_0x279fe4(value: &crate::lua::LuaColor3, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Color3>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {}", value.r, value.g, value.b)));
1
}

// 0x27a4f8 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13registerClassEP9lua_StatePFiS8_ESA_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x27a4f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x27a5e0 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_gc(lua_State *)")]
pub fn stub_0x27a5e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x27a608 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_tostring(lua_State *)")]
pub fn stub_0x27a608() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x27a630 — __ZN3RBX3Lua6BridgeIN3rbx7signals10connectionELb1EE13registerClassEP9lua_StatePFiS7_ES9_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<rbx::signals::connection,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x27a630() -> crate::slot::SlotConnection {
// IDA 0x27a630: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x27a744 — __ZN3RBX3Lua6BridgeIN3rbx7signals10connectionELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx::signals::connection,true>::on_gc(lua_State *)")]
pub fn stub_0x27a744() -> crate::slot::SlotConnection {
// IDA 0x27a744: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Lua::Bridge<rbx::signals::connection,true>::on_eq(lua_State *)")]
pub fn stub_0x27a76c() -> crate::slot::SlotConnection {
// IDA 0x27a76c: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x27a7a8 — __ZN3RBX3Lua6BridgeIN3rbx7signals10connectionELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx::signals::connection,true>::on_tostring(lua_State *)")]
pub fn stub_0x27a7a8() -> crate::slot::SlotConnection {
// IDA 0x27a7a8: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x27a7d0 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x27a7d0(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x27a7d0: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x27a8e4 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_gc(lua_State *)")]
pub fn stub_0x27a8e4(value: crate::slot::InstanceHandle) {
// Bridge<Library>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x27a904 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_eq(lua_State *)")]
pub fn stub_0x27a904(a: &crate::slot::InstanceHandle, b: &crate::slot::InstanceHandle) -> bool {
// Bridge<Library>::on_eq — value equality on the payload.
a == b
}

// 0x27a948 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_tostring(lua_State *)")]
pub fn stub_0x27a948(value: &crate::slot::InstanceHandle, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Library>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(value.class.to_owned()));
1
}

// 0x27a970 — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x27a970(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x27a970: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x27aa84 — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_gc(lua_State *)")]
pub fn stub_0x27aa84(value: Option<rbx_core::SharedPtr<crate::lua::LuaInstanceHandle>>) {
// Bridge<Instance>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_eq(lua_State *)")]
pub fn stub_0x27aaac(a: &Option<rbx_core::SharedPtr<crate::lua::LuaInstanceHandle>>, b: &Option<rbx_core::SharedPtr<crate::lua::LuaInstanceHandle>>) -> bool {
// Bridge<Instance>::on_eq — value equality on the payload.
a == b
}

// 0x27aae8 — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_tostring(lua_State *)")]
pub fn stub_0x27aae8(value: &Option<rbx_core::SharedPtr<crate::lua::LuaInstanceHandle>>, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Instance>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(value.as_ref().map(|h| h.name.clone()).unwrap_or_else(|| "nil".to_owned())));
1
}

// 0x27ab10 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE13registerClassEP9lua_StatePFiS7_ES9_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x27ab10(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x27ab10: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

// 0x27ac24 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_gc(lua_State *)")]
pub fn stub_0x27ac24(value: crate::lua::LuaEnumDescriptor) {
// Bridge<EnumDesc>::on_gc — releases the one host ref the
// userdata held (cf. CellID temp dtor, 0x26e17c).
drop(value);
}

// 0x27ac40 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_eq(lua_State *)")]
pub fn stub_0x27ac40(a: &crate::lua::LuaEnumDescriptor, b: &crate::lua::LuaEnumDescriptor) -> bool {
// Bridge<EnumDesc>::on_eq — value equality on the payload.
a == b
}

// 0x27ac7c — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_tostring(lua_State *)")]
pub fn stub_0x27ac7c(value: &crate::lua::LuaEnumDescriptor, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<EnumDesc>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(value.name.clone()));
1
}

// 0x27afcc — __ZNK3RBX3Lua13EventInstanceeqERKS1_
// type: bool __fastcall(_DWORD *, _DWORD *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool __fastcall(_DWORD *, _DWORD *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Lua::EventInstance::operator==(RBX::Lua::EventInstance const&)const")]
pub fn stub_0x27afcc(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::EventInstance::operator==(RBX::Lua::EventInstance const&)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x27b0cc — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_tostring(RBX::Lua::Library const&,lua_State *)")]
pub fn stub_0x27b0cc(value: &crate::slot::InstanceHandle, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Library>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(value.class.to_owned()));
1
}

// 0x27b1f0 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_tostring(RBX::Region3int16 const&,lua_State *)")]
pub fn stub_0x27b1f0(value: &crate::lua::LuaRegion3i16, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Region3int16>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {} - {}, {}, {}", value.min.x, value.min.y, value.min.z, value.max.x, value.max.y, value.max.z)));
1
}

// 0x27b314 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_tostring(RBX::Region3 const&,lua_State *)")]
pub fn stub_0x27b314(value: &crate::lua::LuaRegion3, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Region3>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(format!("{}, {}, {} - {}, {}, {}", value.min.x, value.min.y, value.min.z, value.max.x, value.max.y, value.max.z)));
1
}

// 0x27b6a4 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE8on_indexERKS4_PKcP9lua_State
// type: int __fastcall(int, const char *, int)
// was: int __fastcall(int, const char *, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_index(RBX::Lua::AllEnumDescriptors const* const&,char const*,lua_State *)")]
pub fn stub_0x27b6a4(value: &crate::lua::LuaEnumDescriptor, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<EnumDescriptor>::on_index — ordinal lookup by key.
match key.parse::<usize>() {
    Ok(i) if i < value.values.len() => {
        thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: value.name.clone(), value: value.values[i] }) }));
    }
    _ => panic!("{key} is not a valid member"),
}
1
}

// 0x27b798 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE11on_newindexERS4_PKcP9lua_State
// type: void __noreturn()
// was: void __noreturn()
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_newindex(RBX::Lua::AllEnumDescriptors const*&,char const*,lua_State *)")]
pub fn stub_0x27b798(key: &str) -> ! {
// Bridge<EnumDesc>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

// 0x27b974 — __ZN3RBX3LuaL12pushEnumListEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushEnumList(lua_State *)")]
pub fn stub_0x27b974(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::pushEnumList(lua_State *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x27bce8 — __ZN3RBX3Lua5Enums15declareAllEnumsEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::Enums::declareAllEnums(lua_State *)")]
pub fn stub_0x27bce8(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::Enums::declareAllEnums(lua_State *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x27bd44 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE11on_tostringERKS4_P9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_tostring(RBX::Lua::AllEnumDescriptors const* const&,lua_State *)")]
pub fn stub_0x27bd44(value: &crate::lua::LuaEnumDescriptor, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<EnumDesc>::on_tostring — pushes the display string.
thread.push(crate::lua::LuaStackValue::String(value.name.clone()));
1
}

// 0x27bfb8 — __ZN3RBX3Lua12newweaktableEP9lua_StatePKc
// type: int __fastcall(int, char *)
// was: int __fastcall(int, char *)
#[doc(alias = "RBX::Lua::newweaktable(lua_State *,char const*)")]
pub fn stub_0x27bfb8(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::newweaktable(lua_State *,char const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x27c004 — __ZN3RBX3Lua12ObjectBridge11newInstanceEP9lua_State
// type: int __fastcall(boost::detail::sp_counted_base *)
// was: int __fastcall(boost::detail::sp_counted_base *)
#[doc(alias = "RBX::Lua::ObjectBridge::newInstance(lua_State *)")]
pub fn stub_0x27c004(_class_name: &str) -> crate::slot::InstanceHandle {
// ObjectBridge::newInstance — constructs the named
// Instance (engine-side); the host keeps its identity.
crate::slot::InstanceHandle::new("Instance")
}

// 0x27c244 — __ZN3RBX3Lua12ObjectBridge12lockInstanceEP9lua_State
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::Lua::ObjectBridge::lockInstance(lua_State *)")]
pub fn stub_0x27c244(handle: &crate::slot::InstanceHandle) {
// ObjectBridge::lockInstance — the lock flag is engine-side.
let _ = handle;
}

// 0x27c254 — __ZN3RBX3Lua12ObjectBridge14unlockInstanceEP9lua_State
// type: int()
// was: int()
#[doc(alias = "RBX::Lua::ObjectBridge::unlockInstance(lua_State *)")]
pub fn stub_0x27c254(handle: &crate::slot::InstanceHandle) {
// ObjectBridge::unlockInstance — the lock flag is engine-side.
let _ = handle;
}

// 0x27c258 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexERKS5_PKcP9lua_State
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_index(rbx_core::SharedPtr<RBX::Instance> const&,char const*,lua_State *)")]
pub fn stub_0x27c258() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x27dbc8 — __ZL22PropertyNameCorrectionRKN5boost10shared_ptrIN3RBX8InstanceEEEPKcP9lua_State
// type: const char *__fastcall(int, const char *, int)
// was: const char *__fastcall(int, const char *, int)
#[doc(alias = "PropertyNameCorrection(rbx_core::SharedPtr<RBX::Instance> const&,char const*,lua_State *)")]
pub fn stub_0x27dbc8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x27ddf0 — __ZN3RBX3Lua12ObjectBridge23callMemberYieldFunctionEP9lua_State
// type: RBX::Reflection::MemberDescriptor *__fastcall(int)
// was: RBX::Reflection::MemberDescriptor *__fastcall(int)
#[doc(alias = "RBX::Lua::ObjectBridge::callMemberYieldFunction(lua_State *)")]
pub fn stub_0x27ddf0(handle: &crate::slot::InstanceHandle, _member: &str) -> crate::slot::PortedFn {
// ObjectBridge::callMemberYieldFunction — dispatched through reflection
// engine-side; the entry preserves the call linkage.
let _ = handle;
crate::slot::PortedFn::new(0x27ddf0, "ObjectBridge::callMemberYieldFunction")
}

// 0x27e110 — __ZN3RBX3Lua12ObjectBridge18callMemberFunctionEP9lua_State
// type: RBX::Reflection::MemberDescriptor *__fastcall(int)
// was: RBX::Reflection::MemberDescriptor *__fastcall(int)
#[doc(alias = "RBX::Lua::ObjectBridge::callMemberFunction(lua_State *)")]
pub fn stub_0x27e110(handle: &crate::slot::InstanceHandle, _member: &str) -> crate::slot::PortedFn {
// ObjectBridge::callMemberFunction — dispatched through reflection
// engine-side; the entry preserves the call linkage.
let _ = handle;
crate::slot::PortedFn::new(0x27e110, "ObjectBridge::callMemberFunction")
}

// 0x27ef18 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_newindexERS5_PKcP9lua_State
// type: void __fastcall(RBX::Security::Context *, const char *, int)
// was: void __fastcall(RBX::Security::Context *, const char *, int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_newindex(rbx_core::SharedPtr<RBX::Instance>&,char const*,lua_State *)")]
pub fn stub_0x27ef18() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x280b90 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringERKS5_P9lua_State
// type: int __fastcall(RBX::Instance **, int)
// was: int __fastcall(RBX::Instance **, int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_tostring(rbx_core::SharedPtr<RBX::Instance> const&,lua_State *)")]
pub fn stub_0x280b90() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x280bac — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *)
// was: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "RBX::Lua::EventInstance* RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::pushNewObject<RBX::Lua::EventInstance>(lua_State *,RBX::Lua::EventInstance)")]
pub fn stub_0x280bac(thread: &mut crate::lua::LuaThreadState, value: &Option<rbx_core::SharedPtr<crate::lua::LuaInstanceHandle>>) -> Option<rbx_core::SharedPtr<crate::lua::LuaInstanceHandle>> {
// SharedPtrBridge::pushNewObject<Instance> — pushes the
// Instance-class userdata (or nil) and returns the ref.
thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::INSTANCE.to_owned(), payload: crate::lua::LuaUserdataPayload::Instance(value.clone()) }));
value.clone()
}

// 0x280c4c — __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrIN5boost10shared_ptrIS2_EEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Instance> &)")]
pub fn stub_0x280c4c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x280d34 — __ZN24YieldFunctionStateObject7executeEv
// type: int __fastcall(YieldFunctionStateObject *this)
// was: int __fastcall(YieldFunctionStateObject *this)
#[doc(alias = "YieldFunctionStateObject::execute(void)")]
pub fn stub_0x280d34() -> crate::slot::PortedFn {
// IDA 0x280d34: YieldFunctionStateObject::execute(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x280d34, "YieldFunctionStateObject::execute(void)")
}

// 0x281494 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueIS5_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Instance> &)")]
pub fn stub_0x281494() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x281c0c — __ZN24YieldFunctionStateObject16onRaiseExceptionESs
// type: void __fastcall(int, const std::string *, int)
// was: void __fastcall(int, const std::string *, int)
#[doc(alias = "YieldFunctionStateObject::onRaiseException(std::string)")]
pub fn stub_0x281c0c() -> crate::slot::PortedFn {
// IDA 0x281c0c: YieldFunctionStateObject::onRaiseException(std::string).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x281c0c, "YieldFunctionStateObject::onRaiseException(std::string)")
}

// 0x282734 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS7_5list2INS7_5valueISC_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
// was: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS7_5list2INS7_5valueISC_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x282734() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

// 0x2828bc — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS6_5list2INS6_5valueISB_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
// was: _DWORD *__fastcall(_DWORD *, int *)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS6_5list2INS6_5valueISB_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2828bc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

// 0x2857a4 — __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrEP9lua_Statej
// type: int __fastcall(sp_counted_base **, int, const char *)
// was: int __fastcall(sp_counted_base **, int, const char *)
#[doc(alias = "RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr(lua_State *,unsigned int)")]
pub fn stub_0x2857a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x285c30 — __ZN3RBX12LuaAllocatorC1Eb
// type: int __fastcall(RBX::LuaAllocator *this, bool)
// was: int __fastcall(RBX::LuaAllocator *this, bool)
#[doc(alias = "RBX::LuaAllocator::LuaAllocator(bool)")]
pub fn stub_0x285c30() -> crate::slot::InstanceHandle {
// RBX::LuaAllocator ctor.
crate::slot::InstanceHandle::new("RBX::LuaAllocator")
}

// 0x285c34 — __ZN3RBX12LuaAllocatorC2Eb
// type: RBX::LuaAllocator *__fastcall(RBX::LuaAllocator *this, int)
// was: RBX::LuaAllocator *__fastcall(RBX::LuaAllocator *this, int)
#[doc(alias = "RBX::LuaAllocator::LuaAllocator(bool) [0x285c34]")]
pub fn stub_0x285c34() -> crate::slot::InstanceHandle {
// RBX::LuaAllocator ctor.
crate::slot::InstanceHandle::new("RBX::LuaAllocator")
}

// 0x285d3c — __ZN3RBX12LuaAllocatorD1Ev
// type: void __fastcall(RBX::LuaAllocator *__hidden this)
// was: void __fastcall(RBX::LuaAllocator *__hidden this)
#[doc(alias = "RBX::LuaAllocator::~LuaAllocator()")]
pub fn stub_0x285d3c(handle: crate::slot::InstanceHandle) {
// RBX::LuaAllocator dtor.
drop(handle);
}

// 0x285d40 — __ZN3RBX12LuaAllocatorD2Ev
// type: void __fastcall(RBX::LuaAllocator *__hidden this)
// was: void __fastcall(RBX::LuaAllocator *__hidden this)
#[doc(alias = "RBX::LuaAllocator::~LuaAllocator() [0x285d40]")]
pub fn stub_0x285d40(handle: crate::slot::InstanceHandle) {
// RBX::LuaAllocator dtor.
drop(handle);
}

// 0x285da0 — __ZN3RBX12LuaAllocator12clearHeapMaxEv
// type: int __fastcall(int this)
// was: int __fastcall(int this)
#[doc(alias = "RBX::LuaAllocator::clearHeapMax(void)")]
pub fn stub_0x285da0(handle: &crate::slot::InstanceHandle) {
// RBX::LuaAllocator::clearHeapMax(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x285dac — __ZNK3RBX12LuaAllocator12getHeapStatsERmS1_S1_S1_
// type: unsigned int __fastcall(RBX::LuaAllocator *this, unsigned int *, unsigned int *, unsigned int *, unsigned int *)
// was: unsigned int __fastcall(RBX::LuaAllocator *this, unsigned int *, unsigned int *, unsigned int *, unsigned int *)
#[doc(alias = "RBX::LuaAllocator::getHeapStats(unsigned long &,unsigned long &,unsigned long &,unsigned long &)const")]
pub fn stub_0x285dac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::LuaAllocator getter.
cell.get()
}

// 0x285dc8 — __ZN3RBX12LuaAllocator5allocEPvS1_mm
// type: int __fastcall(RBX::LuaAllocator *this, void *, void *, unsigned int, unsigned int)
// was: int __fastcall(RBX::LuaAllocator *this, void *, void *, unsigned int, unsigned int)
#[doc(alias = "RBX::LuaAllocator::alloc(void *,void *,unsigned long,unsigned long)")]
pub fn stub_0x285dc8(handle: &crate::slot::InstanceHandle) {
// RBX::LuaAllocator::alloc(void *,void *,unsigned long,unsigned long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x285dd4 — __ZN3RBX12LuaAllocator8hasSpaceEl
// type: bool __fastcall(RBX::LuaAllocator *this, int)
// was: bool __fastcall(RBX::LuaAllocator *this, int)
#[doc(alias = "RBX::LuaAllocator::hasSpace(long)")]
pub fn stub_0x285dd4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::LuaAllocator getter.
cell.get()
}

// 0x285e14 — __ZN3RBX12LuaAllocator5allocEPvmm
// type: _DWORD __fastcall(RBX::LuaAllocator *__hidden this, void *, unsigned int, unsigned int)
// was: _DWORD __fastcall(RBX::LuaAllocator *__hidden this, void *, unsigned int, unsigned int)
#[doc(alias = "RBX::LuaAllocator::alloc(void *,unsigned long,unsigned long)")]
pub fn stub_0x285e14(handle: &crate::slot::InstanceHandle) {
// RBX::LuaAllocator::alloc(void *,unsigned long,unsigned long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x286330 — __ZN3RBX11LuaSettingsC1Ev
// type: _DWORD __fastcall(RBX::LuaSettings *__hidden this)
// was: _DWORD __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "RBX::LuaSettings::LuaSettings(void)")]
pub fn stub_0x286330() -> crate::slot::InstanceHandle {
// RBX::LuaSettings ctor.
crate::slot::InstanceHandle::new("RBX::LuaSettings")
}

// 0x286334 — __ZN3RBX11LuaSettingsC2Ev
// type: _DWORD __fastcall(RBX::LuaSettings *__hidden this)
// was: _DWORD __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "RBX::LuaSettings::LuaSettings(void) [0x286334]")]
pub fn stub_0x286334() -> crate::slot::InstanceHandle {
// RBX::LuaSettings ctor.
crate::slot::InstanceHandle::new("RBX::LuaSettings")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEEC2Ev")]
pub fn stub_0x28652c() -> crate::slot::InstanceHandle {
// settings-item ctor.
crate::slot::InstanceHandle::new("RBX::GlobalAdvancedSettingsItem")
}

// 0x28679c — __ZN3RBX11LuaSettingsD1Ev
// type: void __fastcall(RBX::LuaSettings *__hidden this)
// was: void __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "RBX::LuaSettings::~LuaSettings()")]
pub fn stub_0x28679c(handle: crate::slot::InstanceHandle) {
// RBX::LuaSettings dtor.
drop(handle);
}

// 0x2867dc — __ZN3RBX11LuaSettingsD0Ev
// type: void __fastcall(RBX::LuaSettings *__hidden this)
// was: void __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "RBX::LuaSettings::~LuaSettings() [0x2867dc]")]
pub fn stub_0x2867dc(handle: crate::slot::InstanceHandle) {
// RBX::LuaSettings dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x2868bc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaSettings"
}

// 0x2868cc — __ZThn32_N3RBX11LuaSettingsD1Ev
// type: void __fastcall(RBX::LuaSettings *__hidden this)
// was: void __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaSettings::~LuaSettings()")]
pub fn stub_0x2868cc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x286910 — __ZThn32_N3RBX11LuaSettingsD0Ev
// type: void __fastcall(RBX::LuaSettings *__hidden this)
// was: void __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaSettings::~LuaSettings() [0x286910]")]
pub fn stub_0x286910(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x2869f0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaSettings"
}

// 0x286a00 — __ZThn36_N3RBX11LuaSettingsD1Ev
// type: void __fastcall(RBX::LuaSettings *__hidden this)
// was: void __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaSettings::~LuaSettings() [0x286a00]")]
pub fn stub_0x286a00(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x286a44 — __ZThn36_N3RBX11LuaSettingsD0Ev
// type: void __fastcall(RBX::LuaSettings *__hidden this)
// was: void __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaSettings::~LuaSettings() [0x286a44]")]
pub fn stub_0x286a44(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x286b24() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaSettings"
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED1Ev")]
pub fn stub_0x286b98(handle: crate::slot::InstanceHandle) {
// settings-item dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev")]
pub fn stub_0x286bd8(handle: crate::slot::InstanceHandle) {
// settings-item dtor.
drop(handle);
}

// 0x286cb8 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED1Ev
// type: void __fastcall(_QWORD *)
// was: void __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED1Ev")]
pub fn stub_0x286cb8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev")]
pub fn stub_0x286cfc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED1Ev")]
pub fn stub_0x286d04(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev")]
pub fn stub_0x286d48(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

// 0x287acc — __ZN3RBX3Lua6BridgeINS0_13EventInstanceELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1)
// was: int __fastcall(int, char *__s1)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_index(RBX::Lua::EventInstance const&,char const*,lua_State *)")]
pub fn stub_0x287acc(value: &Option<rbx_core::SharedPtr<crate::lua::LuaInstanceHandle>>, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// SharedPtrBridge<Instance>::on_index — reflection property
// lookup through the instance bag; missing members push nil.
let _ = value;
if key == "Connect" || key == "Wait" {
    thread.push(crate::lua::LuaStackValue::Function(crate::lua::method_fn_id(key)));
} else {
    panic!("{key} is not a valid member");
}
1
}

// 0x287d0c — __ZN3RBX3Lua11EventBridge7connectEP9lua_State
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Lua::EventBridge::connect(lua_State *)")]
pub fn stub_0x287d0c(handle: &crate::slot::InstanceHandle) -> crate::slot::SlotConnection {
// EventBridge::connect — links the handler, returns the
// RBXScriptConnection (cf. signal::connect, 0x39d700).
let _ = handle;
crate::slot::SlotConnection::new()
}
