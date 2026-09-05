// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel|Workspace (10215) complete — fallback global gap filler lowest uncovered EA asc not yet in datamodel
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x278c4c..0x2811cc | datamodel distinct 31164->31264 global uncovered 54381->54281, lowest gap EA-sorted asc
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias where needed

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_224::CellId;
use crate::model::RbxRay;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x278c4c — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_tostring(RBX::Faces const&,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x278c4c as stub_0x278c4c;

// 0x278d70 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_tostring(RBX::Axes const&,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x278d70 as stub_0x278d70;

// 0x278e94 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_tostring(RBX::CellID const&,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x278e94 as stub_0x278e94;

// 0x278fb8 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_tostring(RBX::InputObject const&,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x278fb8 as stub_0x278fb8;

// 0x2790dc — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x2790dc as stub_0x2790dc;

// 0x2791f0 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x2791f0 as stub_0x2791f0;

// 0x27920c — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27920c as stub_0x27920c;

// 0x279248 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279248 as stub_0x279248;

// 0x279270 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x279270 as stub_0x279270;

// 0x279384 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279384 as stub_0x279384;

// 0x2793a0 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x2793a0 as stub_0x2793a0;

// 0x2793e0 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x2793e0 as stub_0x2793e0;

// 0x279408 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x279408 as stub_0x279408;

// 0x27951c — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27951c as stub_0x27951c;

// 0x27953c — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27953c as stub_0x27953c;

// 0x279578 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279578 as stub_0x279578;

// 0x2795a0 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x2795a0 as stub_0x2795a0;

// 0x2796b4 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x2796b4 as stub_0x2796b4;

// 0x2796d0 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x2796d0 as stub_0x2796d0;

// 0x279710 — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279710 as stub_0x279710;

// 0x279738 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x279738 as stub_0x279738;

// 0x27984c — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27984c as stub_0x27984c;

// 0x279868 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279868 as stub_0x279868;

// 0x2798a8 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x2798a8 as stub_0x2798a8;

// 0x2798d0 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x2798d0 as stub_0x2798d0;

// 0x2799e4 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x2799e4 as stub_0x2799e4;

// 0x279a08 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279a08 as stub_0x279a08;

// 0x279a44 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279a44 as stub_0x279a44;

// 0x279a6c — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x279a6c as stub_0x279a6c;

// 0x279b80 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279b80 as stub_0x279b80;

// 0x279b9c — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279b9c as stub_0x279b9c;

// 0x279c54 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279c54 as stub_0x279c54;

// 0x279c7c — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x279c7c as stub_0x279c7c;

// 0x279d90 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279d90 as stub_0x279d90;

// 0x279dac — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279dac as stub_0x279dac;

// 0x279e1c — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279e1c as stub_0x279e1c;

// 0x279e44 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x279e44 as stub_0x279e44;

// 0x279f58 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279f58 as stub_0x279f58;

// 0x279f74 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279f74 as stub_0x279f74;

// 0x279fe4 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x279fe4 as stub_0x279fe4;

// 0x27a00c — __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE13registerClassEP9lua_StatePFiS9_ESB_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<boost::intrusive_ptr<RBX::Lua::WeakThreadRef::Node>,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
pub use rbx_core::generated_core_shard_a::stub_0x27a00c as stub_0x27a00c;

// 0x27a120 — __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<boost::intrusive_ptr<RBX::Lua::WeakThreadRef::Node>,true>::on_gc(lua_State *)
pub use rbx_core::generated_core_shard_a::stub_0x27a120 as stub_0x27a120;

// 0x27a148 — __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<boost::intrusive_ptr<RBX::Lua::WeakThreadRef::Node>,true>::on_eq(lua_State *)
pub use rbx_core::generated_core_shard_a::stub_0x27a148 as stub_0x27a148;

// 0x27a188 — __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<boost::intrusive_ptr<RBX::Lua::WeakThreadRef::Node>,true>::on_tostring(lua_State *)
pub use rbx_core::generated_core_shard_a::stub_0x27a188 as stub_0x27a188;

// 0x27a1b0 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE13registerClassEP9lua_StatePFiSE_ESG_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_27a1b0 as stub_0x27a1b0;

// 0x27a2c4 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,true>::on_gc(lua_State *)
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_27a2c4 as stub_0x27a2c4;

// 0x27a2ec — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,true>::on_eq(lua_State *)
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_27a2ec as stub_0x27a2ec;

// 0x27a32c — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,true>::on_tostring(lua_State *)
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_27a32c as stub_0x27a32c;

// 0x27a354 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE13registerClassEP9lua_StatePFiSI_ESK_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_27a354 as stub_0x27a354;

// 0x27a468 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_gc(lua_State *)
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_27a468 as stub_0x27a468;

// 0x27a490 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_eq(lua_State *)
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_27a490 as stub_0x27a490;

// 0x27a4d0 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_tostring(lua_State *)
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_27a4d0 as stub_0x27a4d0;

// 0x27a630 — __ZN3RBX3Lua6BridgeIN3rbx7signals10connectionELb1EE13registerClassEP9lua_StatePFiS7_ES9_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<rbx::signals::connection,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_d::stub_27a630 as stub_0x27a630;

// 0x27a744 — __ZN3RBX3Lua6BridgeIN3rbx7signals10connectionELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx::signals::connection,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_d::stub_27a744 as stub_0x27a744;

// 0x27a76c — __ZN3RBX3Lua6BridgeIN3rbx7signals10connectionELb1EE5on_eqEP9lua_State
// type: void
#[doc(alias = "RBX::Lua::Bridge<rbx::signals::connection,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_d::stub_27a76c as stub_0x27a76c;

// 0x27a7a8 — __ZN3RBX3Lua6BridgeIN3rbx7signals10connectionELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<rbx::signals::connection,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_d::stub_27a7a8 as stub_0x27a7a8;

// 0x27a7d0 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE13registerClassEP9lua_StatePFiS5_ES7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x27a7d0 as stub_0x27a7d0;

// 0x27a8e4 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27a8e4 as stub_0x27a8e4;

// 0x27a904 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27a904 as stub_0x27a904;

// 0x27a948 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27a948 as stub_0x27a948;

// 0x27ab10 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE13registerClassEP9lua_StatePFiS7_ES9_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x27ab10 as stub_0x27ab10;

// 0x27ac24 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27ac24 as stub_0x27ac24;

// 0x27ac40 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27ac40 as stub_0x27ac40;

// 0x27ac7c — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27ac7c as stub_0x27ac7c;

// 0x27aca4 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE13registerClassEP9lua_StatePFiS8_ESA_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x27aca4 as stub_0x27aca4;

// 0x27adb8 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27adb8 as stub_0x27adb8;

// 0x27add4 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27add4 as stub_0x27add4;

// 0x27ae10 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27ae10 as stub_0x27ae10;

// 0x27ae38 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE13registerClassEP9lua_StatePFiS9_ESB_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
pub use rbx_core::generated_core_shard_nv::stub_0x27ae38 as stub_0x27ae38;

// 0x27af4c — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE5on_gcEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_gc(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27af4c as stub_0x27af4c;

// 0x27af68 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE5on_eqEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_eq(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27af68 as stub_0x27af68;

// 0x27afa4 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_tostring(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27afa4 as stub_0x27afa4;

// 0x27b0cc — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_tostring(RBX::Lua::Library const&,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27b0cc as stub_0x27b0cc;

// 0x27b1f0 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_tostring(RBX::Region3int16 const&,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27b1f0 as stub_0x27b1f0;

// 0x27b314 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE11on_tostringERKS2_P9lua_State
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_tostring(RBX::Region3 const&,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27b314 as stub_0x27b314;

// 0x27b438 — __ZNK3RBX6RbxRayeqERKS0_
// type: bool __fastcall(float *, float *)
#[doc(alias = "RBX::RbxRay::operator==(RBX::RbxRay const&)const")]
pub fn stub_0x27b438(first: &RbxRay, second: &RbxRay) -> bool {
    // IDA 0x27b438 (decompiled): chained exact `==` over words `a1[1..=6]`
    // (offsets +4..+24: the origin + direction floats); offset +0 (the vtable
    // word, which has no Rust counterpart) is skipped, miss returns 0 (0x27b4b2).
    // `Vector3: PartialEq` is the same float equality.
    first.origin == second.origin && first.direction == second.direction
}

// 0x27b4b4 — __ZNK3RBX6CellIDeqERKS0_
// type: bool __fastcall(unsigned __int8 *, unsigned __int8 *)
#[doc(alias = "RBX::CellID::operator==(RBX::CellID const&)const")]
pub fn stub_0x27b4b4(first: &CellId, second: &CellId) -> bool {
    // IDA 0x27b4b4 (decompiled): byte at +0, exact float `==` at +4/+8/+12,
    // dword at +16 — the link `px` word only (the count word at +20 is NOT
    // compared, 0x27b504); miss returns 0 (0x27b508). `Arc::ptr_eq` is the
    // same `px` compare; `CellId` layout (`flag`, `coords`, `link`) is 0x897568.
    first.flag == second.flag && first.coords == second.coords
        && SharedPtr::ptr_eq(&first.link, &second.link)
}

// 0x27b50c — __GLOBAL__I_a_65
// type: void
#[doc(alias = "global constructor keyed to_a_65")]
// was: global constructor keyed to_a_65
pub fn stub_0x27b50c() {
    // IDA 0x27b50c (`__GLOBAL__I_a_65`, disasm 0x27b50c..): stores
    // `boost::system::generic_category()` / `system_category()` into the
    // `__MergedGlobals_95` slots plus a `std::ios_base::Init` construct.
    // Process-static init; the `__cxa_guard` once-init collapses into static
    // init (cf. instance.rs `INSTANCE_SIGNAL_MUTEX`). No observable body remains.
}

// 0x27b6a4 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE8on_indexERKS4_PKcP9lua_State
// type: int __fastcall(int, const char *, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_index(RBX::Lua::AllEnumDescriptors const* const&,char const*,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27b6a4 as stub_0x27b6a4;

// 0x27b798 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE11on_newindexERS4_PKcP9lua_State
// type: void __noreturn()
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_newindex(RBX::Lua::AllEnumDescriptors const*&,char const*,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27b798 as stub_0x27b798;

// 0x27b84c — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE8on_indexERKS5_PKcP9lua_State
// type: int __fastcall(_DWORD *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_index(RBX::Reflection::EnumDescriptor const* const&,char const*,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27b84c as stub_0x27b84c;

// 0x27b974 — __ZN3RBX3LuaL12pushEnumListEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushEnumList(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27b974 as stub_0x27b974;

// 0x27ba4c — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE11on_newindexERS5_PKcP9lua_State
// type: void __noreturn()
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_newindex(RBX::Reflection::EnumDescriptor const*&,char const*,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27ba4c as stub_0x27ba4c;

// 0x27bb00 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE8on_indexERKS6_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_index(RBX::Reflection::EnumDescriptor::Item const* const&,char const*,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27bb00 as stub_0x27bb00;

// 0x27bc34 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE11on_newindexERS6_PKcP9lua_State
// type: void __noreturn()
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_newindex(RBX::Reflection::EnumDescriptor::Item const*&,char const*,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27bc34 as stub_0x27bc34;

// 0x27bce8 — __ZN3RBX3Lua5Enums15declareAllEnumsEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Enums::declareAllEnums(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27bce8 as stub_0x27bce8;

// 0x27bd1c — __ZN3RBX3Lua5Enums8getValueEP9lua_StatejRNS_10Reflection7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::Enums::getValue(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27bd1c as stub_0x27bd1c;

// 0x27bd44 — __ZN3RBX3Lua6BridgeIPKNS0_18AllEnumDescriptorsELb1EE11on_tostringERKS4_P9lua_State
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_tostring(RBX::Lua::AllEnumDescriptors const* const&,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27bd44 as stub_0x27bd44;

// 0x27bd60 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptorELb1EE11on_tostringERKS5_P9lua_State
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::on_tostring(RBX::Reflection::EnumDescriptor const* const&,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27bd60 as stub_0x27bd60;

// 0x27bd78 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE11on_tostringERKS6_P9lua_State
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::on_tostring(RBX::Reflection::EnumDescriptor::Item const* const&,lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27bd78 as stub_0x27bd78;

// 0x27bea8 — __ZN3RBX10Reflection14EnumDescriptor16lookupDescriptorERKNS_4NameE
// type: int __fastcall(RBX::Reflection::EnumDescriptor *this, const Name *)
#[doc(alias = "RBX::Reflection::EnumDescriptor::lookupDescriptor(RBX::Name const&)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27bea8 as stub_0x27bea8;

// 0x27bef0 — __GLOBAL__I_a_66
// type: void
#[doc(alias = "global constructor keyed to_a_66")]
// was: global constructor keyed to_a_66
pub fn stub_0x27bef0() {
    // IDA 0x27bef0 (`__GLOBAL__I_a_66`, disasm 0x27bef0..): same
    // `__MergedGlobals_96` + `std::ios_base::Init` static-init shape as 0x27b50c.
    // Same collapse; no observable body remains.
}

// 0x27bfb8 — __ZN3RBX3Lua12newweaktableEP9lua_StatePKc
// type: int __fastcall(int, char *)
#[doc(alias = "RBX::Lua::newweaktable(lua_State *,char const*)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27bfb8 as stub_0x27bfb8;

// 0x27ddf0 — __ZN3RBX3Lua12ObjectBridge23callMemberYieldFunctionEP9lua_State
// type: RBX::Reflection::MemberDescriptor *__fastcall(int)
#[doc(alias = "RBX::Lua::ObjectBridge::callMemberYieldFunction(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27ddf0 as stub_0x27ddf0;

// 0x27e110 — __ZN3RBX3Lua12ObjectBridge18callMemberFunctionEP9lua_State
// type: RBX::Reflection::MemberDescriptor *__fastcall(int)
#[doc(alias = "RBX::Lua::ObjectBridge::callMemberFunction(lua_State *)")]
pub use rbx_core::generated_core_shard_nv::stub_0x27e110 as stub_0x27e110;

// 0x27e448 — __ZN3RBX3Lua12callCallbackENS0_15WeakFunctionRefEN5boost10shared_ptrIKNS_10Reflection5TupleEEENS2_13intrusive_ptrINS0_13WeakThreadRefEEE
// type: void __fastcall(boost::detail::sp_counted_base **, int, boost::detail::sp_counted_base ***, boost::detail::sp_counted_base *)
#[doc(alias = "RBX::Lua::callCallback(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>)")]
// was: RBX::Lua::callCallback(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>)
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_27e448 as stub_0x27e448;

// 0x280d34 — __ZN24YieldFunctionStateObject7executeEv
// type: int __fastcall(YieldFunctionStateObject *this)
#[doc(alias = "YieldFunctionStateObject::execute(void)")]
pub use rbx_core::generated_core_shard_jt::stub_280d34 as stub_0x280d34;

// 0x281000 — __ZN5boost10shared_ptrI24YieldFunctionStateObjectEC2IS1_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<YieldFunctionStateObject>::shared_ptr<YieldFunctionStateObject>(YieldFunctionStateObject *)")]
// was: boost::shared_ptr<YieldFunctionStateObject>::shared_ptr<YieldFunctionStateObject>(YieldFunctionStateObject *)
pub use rbx_core::generated_core_shard_a::stub_0x281000 as stub_0x281000;

// 0x2810e8 — __ZNK5boost23enable_shared_from_thisI24YieldFunctionStateObjectE22_internal_accept_ownerIS1_S1_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<YieldFunctionStateObject>::_internal_accept_owner<YieldFunctionStateObject,YieldFunctionStateObject>(rbx_core::SharedPtr<YieldFunctionStateObject> const*,YieldFunctionStateObject *)const")]
// was: void boost::enable_shared_from_this<YieldFunctionStateObject>::_internal_accept_owner<YieldFunctionStateObject,YieldFunctionStateObject>(boost::shared_ptr<YieldFunctionStateObject> const*,YieldFunctionStateObject *)const
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x2810e8 as stub_0x2810e8;

// 0x2811cc — __ZN5boost6detail12shared_countC2I24YieldFunctionStateObjectEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<YieldFunctionStateObject>(YieldFunctionStateObject *)")]
pub use rbx_core::generated_core_shard_a::stub_0x2811cc as stub_0x2811cc;


#[cfg(test)]
mod ray_cellid_tests {
    use super::*;
    use crate::instance::Vector3;

    fn ray(ox: f32, oy: f32, oz: f32, dx: f32, dy: f32, dz: f32) -> RbxRay {
        RbxRay {
            origin: Vector3 { x: ox, y: oy, z: oz },
            direction: Vector3 { x: dx, y: dy, z: dz },
        }
    }

    #[test]
    fn ray_eq_compares_all_six_floats() {
        let a = ray(1.0, 2.0, 3.0, 0.0, 0.0, 1.0);
        assert!(stub_0x27b438(&a, &a));
        assert!(stub_0x27b438(&a, &ray(1.0, 2.0, 3.0, 0.0, 0.0, 1.0)));
        assert!(!stub_0x27b438(&a, &ray(9.0, 2.0, 3.0, 0.0, 0.0, 1.0)));
        assert!(!stub_0x27b438(&a, &ray(1.0, 2.0, 3.0, 0.0, 0.0, 0.0)));
    }

    #[test]
    fn cellid_eq_compares_flag_coords_link_ptr() {
        let link = SharedPtr::new(crate::generated_05::Instance::default());
        let a = CellId { flag: true, coords: [1.0, 2.0, 3.0], link: SharedPtr::clone(&link) };
        let same = CellId { flag: true, coords: [1.0, 2.0, 3.0], link: SharedPtr::clone(&link) };
        assert!(stub_0x27b4b4(&a, &same));
        let other_link = CellId {
            flag: true,
            coords: [1.0, 2.0, 3.0],
            link: SharedPtr::new(crate::generated_05::Instance::default()),
        };
        assert!(!stub_0x27b4b4(&a, &other_link));
        let other_flag = CellId { flag: false, coords: [1.0, 2.0, 3.0], link: SharedPtr::clone(&link) };
        assert!(!stub_0x27b4b4(&a, &other_flag));
        let other_coord = CellId { flag: true, coords: [1.0, 2.0, 4.0], link: SharedPtr::clone(&link) };
        assert!(!stub_0x27b4b4(&a, &other_coord));
    }

    #[test]
    fn global_ctors_are_noops() {
        stub_0x27b50c();
        stub_0x27bef0();
    }
}
