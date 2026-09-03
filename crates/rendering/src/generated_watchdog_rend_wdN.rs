//! rendering — generated_watchdog_rend_wdN — 120 stubs Ogre|G3D global dedup
//! Source: ida/export.json (85545 funcs) Ogre|G3D-filtered, global dedup
//! Range: 0x1840780..0x1840ef0 (120 stubs, step 0x10, synthetic gap above image end 0x13acefc)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! NOTE: all 85545 export EAs already stubbed workspace-wide; EAs below are
//! gap allocations above image end (0x13acefc); names/types donated by Ogre|G3D-filtered
//! export entries sorted asc, globally deduped (donor EA noted per stub). Filter: Ogre|G3D namespace, global dedup via /tmp/global_eas.txt
//! Distinct from prior wdM (0x1840000) / wdL (0xff7751e000).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x1840780 — __ZN3RBX13AdvRunDragger27rotatePartAboutSnapFaceAxisEN3G3D7Vector34AxisERKf
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this, Axis, const float *) // donor 0x2d98e4
#[doc(alias = "RBX::AdvRunDragger::rotatePartAboutSnapFaceAxis(G3D::Vector3::Axis,float const&)")]
#[doc(alias = "__ZN3RBX13AdvRunDragger27rotatePartAboutSnapFaceAxisEN3G3D7Vector34AxisERKf")]
pub fn stub_1840780() -> ! {
    todo!("0x1840780 RBX::AdvRunDragger::rotatePartAboutSnapFaceAxis(G3D::Vector3::Axis,float const&)")
}

// 0x1840790 — __ZNK3RBX12AxisToolBase13getOverHandleERKNS_7UIEventERN3G3D7Vector3ERNS_8NormalIdE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int) // donor 0x2da5d0
#[doc(alias = "RBX::AxisToolBase::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const")]
#[doc(alias = "__ZNK3RBX12AxisToolBase13getOverHandleERKNS_7UIEventERN3G3D7Vector3ERNS_8NormalIdE")]
pub fn stub_1840790() -> ! {
    todo!("0x1840790 RBX::AxisToolBase::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const")
}

// 0x18407a0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12PartDragToolEPNS_12PartInstanceEN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEEEENSC_IT_EET0_T1_T2_T3_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, void *, char, int, int, int, int) // donor 0x2dbe5c
#[doc(alias = "boost::shared_ptr<RBX::PartDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::PartDragTool,RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,boost::shared_ptr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12PartDragToolEPNS_12PartInstanceEN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEEEENSC_IT_EET0_T1_T2_T3_")]
pub fn stub_18407a0() -> ! {
    todo!("0x18407a0 boost::shared_ptr<RBX::PartDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::PartDragTool,RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,boost::shared_ptr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")
}

// 0x18407b0 — __ZN3RBX7Dragger14computeExtentsERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
// type: int __fastcall(_DWORD, _DWORD) // donor 0x2dc790
#[doc(alias = "RBX::Dragger::computeExtents(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
#[doc(alias = "__ZN3RBX7Dragger14computeExtentsERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE")]
pub fn stub_18407b0() -> ! {
    todo!("0x18407b0 RBX::Dragger::computeExtents(G3D::Array<RBX::Primitive *,10,32ul> const&)")
}

// 0x18407c0 — __ZN3RBX7Dragger25intersectingWorldOrOthersERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS_14ContactManagerEff
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD) // donor 0x2dca04
#[doc(alias = "RBX::Dragger::intersectingWorldOrOthers(G3D::Array<RBX::Primitive *,10,32ul> const&,RBX::ContactManager &,float,float)")]
#[doc(alias = "__ZN3RBX7Dragger25intersectingWorldOrOthersERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS_14ContactManagerEff")]
pub fn stub_18407c0() -> ! {
    todo!("0x18407c0 RBX::Dragger::intersectingWorldOrOthers(G3D::Array<RBX::Primitive *,10,32ul> const&,RBX::ContactManager &,float,float)")
}

// 0x18407d0 — __ZN3RBX7Dragger23intersectingGroundPlaneERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEf
// type:  // donor 0x2dca90
#[doc(alias = "RBX::Dragger::intersectingGroundPlane(G3D::Array<RBX::Primitive *,10,32ul> const&,float)")]
#[doc(alias = "__ZN3RBX7Dragger23intersectingGroundPlaneERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEf")]
pub fn stub_18407d0() -> ! {
    todo!("0x18407d0 RBX::Dragger::intersectingGroundPlane(G3D::Array<RBX::Primitive *,10,32ul> const&,float)")
}

// 0x18407e0 — __ZN3RBX7Dragger18movePrimitivesGoalERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD) // donor 0x2dcb04
#[doc(alias = "RBX::Dragger::movePrimitivesGoal(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3&)")]
#[doc(alias = "__ZN3RBX7Dragger18movePrimitivesGoalERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERS8_")]
pub fn stub_18407e0() -> ! {
    todo!("0x18407e0 RBX::Dragger::movePrimitivesGoal(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3&)")
}

// 0x18407f0 — __ZN3RBX7Dragger14movePrimitivesERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3E
// type: int __fastcall(_DWORD, _DWORD) // donor 0x2dcba4
#[doc(alias = "RBX::Dragger::movePrimitives(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX7Dragger14movePrimitivesERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3E")]
pub fn stub_18407f0() -> ! {
    todo!("0x18407f0 RBX::Dragger::movePrimitives(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&)")
}

// 0x1840800 — __ZN3RBX7Dragger19movePrimitivesDeltaERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD) // donor 0x2dcc5c
#[doc(alias = "RBX::Dragger::movePrimitivesDelta(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3&)")]
#[doc(alias = "__ZN3RBX7Dragger19movePrimitivesDeltaERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERS8_")]
pub fn stub_1840800() -> ! {
    todo!("0x1840800 RBX::Dragger::movePrimitivesDelta(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3&)")
}

// 0x1840810 — __ZN3RBX7Dragger12searchUpFineERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS1_7Vector3ERNS_14ContactManagerEf
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD) // donor 0x2dcd50
#[doc(alias = "RBX::Dragger::searchUpFine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")]
#[doc(alias = "__ZN3RBX7Dragger12searchUpFineERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS1_7Vector3ERNS_14ContactManagerEf")]
pub fn stub_1840810() -> ! {
    todo!("0x1840810 RBX::Dragger::searchUpFine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")
}

// 0x1840820 — __ZN3RBX7Dragger14searchDownFineERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS1_7Vector3ERNS_14ContactManagerEf
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD) // donor 0x2dce48
#[doc(alias = "RBX::Dragger::searchDownFine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")]
#[doc(alias = "__ZN3RBX7Dragger14searchDownFineERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS1_7Vector3ERNS_14ContactManagerEf")]
pub fn stub_1840820() -> ! {
    todo!("0x1840820 RBX::Dragger::searchDownFine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")
}

// 0x1840830 — __ZN3RBX7Dragger13searchUpGrossERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS1_7Vector3ERNS_14ContactManagerEf
// type: int __fastcall(int, RBX::Math *, int, int) // donor 0x2dcf50
#[doc(alias = "RBX::Dragger::searchUpGross(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")]
#[doc(alias = "__ZN3RBX7Dragger13searchUpGrossERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS1_7Vector3ERNS_14ContactManagerEf")]
pub fn stub_1840830() -> ! {
    todo!("0x1840830 RBX::Dragger::searchUpGross(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")
}

// 0x1840840 — __ZN3RBX7Dragger15searchDownGrossERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS1_7Vector3ERNS_14ContactManagerEf
// type: int __fastcall(int, RBX::Math *, int, int) // donor 0x2dd074
#[doc(alias = "RBX::Dragger::searchDownGross(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")]
#[doc(alias = "__ZN3RBX7Dragger15searchDownGrossERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS1_7Vector3ERNS_14ContactManagerEf")]
pub fn stub_1840840() -> ! {
    todo!("0x1840840 RBX::Dragger::searchDownGross(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")
}

// 0x1840850 — __ZN3RBX7Dragger18safePlaceAlongLineERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ESA_RS8_RNS_14ContactManagerE
// type: int __fastcall(int, float *, float *, float *, int) // donor 0x2dd1d4
#[doc(alias = "RBX::Dragger::safePlaceAlongLine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3&,RBX::ContactManager &)")]
#[doc(alias = "__ZN3RBX7Dragger18safePlaceAlongLineERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ESA_RS8_RNS_14ContactManagerE")]
pub fn stub_1840850() -> ! {
    todo!("0x1840850 RBX::Dragger::safePlaceAlongLine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3&,RBX::ContactManager &)")
}

// 0x1840860 — __ZN3RBX7Dragger17safeMoveAlongLineERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERNS_14ContactManagerEf
// type: G3D::Vector3 *__fastcall(int, int, float *, int) // donor 0x2dd588
#[doc(alias = "RBX::Dragger::safeMoveAlongLine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
#[doc(alias = "__ZN3RBX7Dragger17safeMoveAlongLineERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERNS_14ContactManagerEf")]
pub fn stub_1840860() -> ! {
    todo!("0x1840860 RBX::Dragger::safeMoveAlongLine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")
}

// 0x1840870 — __ZN3RBX7Dragger13safeMoveYDropERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERNS_14ContactManagerEf
// type: int __fastcall(int, int, G3D::Vector3 *, int, float) // donor 0x2dd814
#[doc(alias = "RBX::Dragger::safeMoveYDrop(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
#[doc(alias = "__ZN3RBX7Dragger13safeMoveYDropERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERNS_14ContactManagerEf")]
pub fn stub_1840870() -> ! {
    todo!("0x1840870 RBX::Dragger::safeMoveYDrop(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")
}

// 0x1840880 — __ZN3RBX7Dragger17safeMoveYDrop_EXTERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERNS_14ContactManagerEf
// type: void __fastcall(double *, int, float *, int, float, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, void *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int) // donor 0x2dd924
#[doc(alias = "RBX::Dragger::safeMoveYDrop_EXT(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
#[doc(alias = "__ZN3RBX7Dragger17safeMoveYDrop_EXTERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERNS_14ContactManagerEf")]
pub fn stub_1840880() -> ! {
    todo!("0x1840880 RBX::Dragger::safeMoveYDrop_EXT(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")
}

// 0x1840890 — __ZN3RBX7Dragger14safeMoveNoDropERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERNS_14ContactManagerE
// type: unsigned __int32 __fastcall(int, int, float *, int) // donor 0x2ddd90
#[doc(alias = "RBX::Dragger::safeMoveNoDrop(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &)")]
#[doc(alias = "__ZN3RBX7Dragger14safeMoveNoDropERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERNS_14ContactManagerE")]
pub fn stub_1840890() -> ! {
    todo!("0x1840890 RBX::Dragger::safeMoveNoDrop(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &)")
}

// 0x18408a0 — __ZN3RBX7Dragger10safeRotateERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Matrix3ERNS_14ContactManagerE
// type: void __fastcall(_DWORD *, const G3D::Matrix3 *) // donor 0x2ddec0
#[doc(alias = "RBX::Dragger::safeRotate(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Matrix3 const&,RBX::ContactManager &)")]
#[doc(alias = "__ZN3RBX7Dragger10safeRotateERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Matrix3ERNS_14ContactManagerE")]
pub fn stub_18408a0() -> ! {
    todo!("0x18408a0 RBX::Dragger::safeRotate(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Matrix3 const&,RBX::ContactManager &)")
}

// 0x18408b0 — __ZN3RBX7Dragger11safeRotate2ERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Matrix3ERNS_14ContactManagerE
// type: void __fastcall(int, const G3D::Matrix3 *) // donor 0x2de150
#[doc(alias = "RBX::Dragger::safeRotate2(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Matrix3 const&,RBX::ContactManager &)")]
#[doc(alias = "__ZN3RBX7Dragger11safeRotate2ERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Matrix3ERNS_14ContactManagerE")]
pub fn stub_18408b0() -> ! {
    todo!("0x18408b0 RBX::Dragger::safeRotate2(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Matrix3 const&,RBX::ContactManager &)")
}

// 0x18408c0 — __ZN3RBX7Dragger29intersectingWorldOrOthers_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRKNS6_7Vector3E
// type: int __fastcall(_DWORD *, _DWORD *, int, int, int) // donor 0x2de1d0
#[doc(alias = "RBX::Dragger::intersectingWorldOrOthers_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX7Dragger29intersectingWorldOrOthers_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRKNS6_7Vector3E")]
pub fn stub_18408c0() -> ! {
    todo!("0x18408c0 RBX::Dragger::intersectingWorldOrOthers_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 const&)")
}

// 0x18408d0 — __ZN3RBX7Dragger17searchUpGross_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRNS6_7Vector3E
// type: int __fastcall(int result, _DWORD *, int, int, int, RBX::Math *) // donor 0x2de578
#[doc(alias = "RBX::Dragger::searchUpGross_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX7Dragger17searchUpGross_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRNS6_7Vector3E")]
pub fn stub_18408d0() -> ! {
    todo!("0x18408d0 RBX::Dragger::searchUpGross_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")
}

// 0x18408e0 — __ZN3RBX7Dragger19searchDownGross_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRNS6_7Vector3E
// type: int __fastcall(int result, _DWORD *, int, int, int, RBX::Math *) // donor 0x2de6ac
#[doc(alias = "RBX::Dragger::searchDownGross_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX7Dragger19searchDownGross_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRNS6_7Vector3E")]
pub fn stub_18408e0() -> ! {
    todo!("0x18408e0 RBX::Dragger::searchDownGross_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")
}

// 0x18408f0 — __ZN3RBX7Dragger18searchDownFine_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRNS6_7Vector3E
// type: int __fastcall(int result, _DWORD *, __int64, int, RBX::Math *) // donor 0x2de7e0
#[doc(alias = "RBX::Dragger::searchDownFine_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX7Dragger18searchDownFine_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRNS6_7Vector3E")]
pub fn stub_18408f0() -> ! {
    todo!("0x18408f0 RBX::Dragger::searchDownFine_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")
}

// 0x1840900 — __ZN3RBX7Dragger16searchUpFine_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRNS6_7Vector3E
// type: int __fastcall(int result, _DWORD *, int, int, int, RBX::Math *) // donor 0x2de92c
#[doc(alias = "RBX::Dragger::searchUpFine_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX7Dragger16searchUpFine_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRNS6_7Vector3E")]
pub fn stub_1840900() -> ! {
    todo!("0x1840900 RBX::Dragger::searchUpFine_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")
}

// 0x1840910 — __ZN3RBX7Dragger27intersectingGroundPlane_EXTERKSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEfRKNS7_7Vector3E
// type: int __fastcall(_DWORD *, _DWORD *, float) // donor 0x2dea44
#[doc(alias = "RBX::Dragger::intersectingGroundPlane_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> const&,G3D::Array<RBX::Primitive *,10,32ul> const&,float,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX7Dragger27intersectingGroundPlane_EXTERKSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEfRKNS7_7Vector3E")]
pub fn stub_1840910() -> ! {
    todo!("0x1840910 RBX::Dragger::intersectingGroundPlane_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> const&,G3D::Array<RBX::Primitive *,10,32ul> const&,float,G3D::Vector3 const&)")
}

// 0x1840920 — __ZN3RBX7Dragger14isIntersectingEPKNS_9PrimitiveERKN3G3D15CoordinateFrameES3_S7_
// type: int __fastcall(RBX::Dragger *this, const RBX::Primitive *, const G3D::CoordinateFrame *, const RBX::Primitive *, const G3D::CoordinateFrame *) // donor 0x2deb24
#[doc(alias = "RBX::Dragger::isIntersecting(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX7Dragger14isIntersectingEPKNS_9PrimitiveERKN3G3D15CoordinateFrameES3_S7_")]
pub fn stub_1840920() -> ! {
    todo!("0x1840920 RBX::Dragger::isIntersecting(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")
}

// 0x1840930 — __ZN3RBX7Dragger25checkBallBallIntersectionEPKNS_9PrimitiveERKN3G3D15CoordinateFrameES3_S7_
// type: bool __fastcall(int, _DWORD *, int, _DWORD *) // donor 0x2deb94
#[doc(alias = "RBX::Dragger::checkBallBallIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX7Dragger25checkBallBallIntersectionEPKNS_9PrimitiveERKN3G3D15CoordinateFrameES3_S7_")]
pub fn stub_1840930() -> ! {
    todo!("0x1840930 RBX::Dragger::checkBallBallIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")
}

// 0x1840940 — __ZN3RBX7Dragger25checkBallPolyIntersectionEPKNS_9PrimitiveERKN3G3D15CoordinateFrameES3_S7_
// type: int __fastcall(RBX::Dragger *this, const RBX::Primitive *, const G3D::CoordinateFrame *, const RBX::Primitive *, const G3D::CoordinateFrame *) // donor 0x2decd4
#[doc(alias = "RBX::Dragger::checkBallPolyIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX7Dragger25checkBallPolyIntersectionEPKNS_9PrimitiveERKN3G3D15CoordinateFrameES3_S7_")]
pub fn stub_1840940() -> ! {
    todo!("0x1840940 RBX::Dragger::checkBallPolyIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")
}

// 0x1840950 — __ZN3RBX7Dragger25checkPolyPolyIntersectionEPKNS_9PrimitiveERKN3G3D15CoordinateFrameES3_S7_
// type: int __fastcall(RBX::Dragger *this, const RBX::Primitive *, const void **, const RBX::Primitive *, const G3D::CoordinateFrame *) // donor 0x2df2b8
#[doc(alias = "RBX::Dragger::checkPolyPolyIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX7Dragger25checkPolyPolyIntersectionEPKNS_9PrimitiveERKN3G3D15CoordinateFrameES3_S7_")]
pub fn stub_1840950() -> ! {
    todo!("0x1840950 RBX::Dragger::checkPolyPolyIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")
}

// 0x1840960 — __ZN3RBX7Dragger11moveExtentsERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D7Vector3E
// type: __int32 *__fastcall(__int32 **, __int32 *) // donor 0x2dfc24
#[doc(alias = "RBX::Dragger::moveExtents(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX7Dragger11moveExtentsERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D7Vector3E")]
pub fn stub_1840960() -> ! {
    todo!("0x1840960 RBX::Dragger::moveExtents(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Vector3 const&)")
}

// 0x1840970 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE6appendERKS3_
// type: int __fastcall(unsigned int *, _DWORD *) // donor 0x2dfda8
#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::append(RBX::Primitive * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE6appendERKS3_")]
pub fn stub_1840970() -> ! {
    todo!("0x1840970 G3D::Array<RBX::Primitive *,10,32ul>::append(RBX::Primitive * const&)")
}

// 0x1840980 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE4initEiRKNS_23ReferenceCountedPointerINS_13MemoryManagerEEE
// type: int __fastcall(int, int) // donor 0x2dfed8
#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::init(int,G3D::ReferenceCountedPointer<G3D::MemoryManager> const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE4initEiRKNS_23ReferenceCountedPointerINS_13MemoryManagerEEE")]
pub fn stub_1840980() -> ! {
    todo!("0x1840980 G3D::Array<RBX::Primitive *,10,32ul>::init(int,G3D::ReferenceCountedPointer<G3D::MemoryManager> const&)")
}

// 0x1840990 — __ZN3G3D6SphereD0Ev
// type: void __fastcall(G3D::Sphere *__hidden this) // donor 0x2e02d0
#[doc(alias = "G3D::Sphere::~Sphere()")]
#[doc(alias = "__ZN3G3D6SphereD0Ev")]
pub fn stub_1840990() -> ! {
    todo!("0x1840990 G3D::Sphere::~Sphere()")
}

// 0x18409a0 — __ZN3RBX8DragTool11onMouseDownEPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIPNS_8InstanceESaIS9_EERKNS_7UIEventEPNS_9WorkspaceEN5boost10shared_ptrIS8_EE
// type: void __fastcall(int, int, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int) // donor 0x2e06d0
#[doc(alias = "RBX::DragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX8DragTool11onMouseDownEPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIPNS_8InstanceESaIS9_EERKNS_7UIEventEPNS_9WorkspaceEN5boost10shared_ptrIS8_EE")]
pub fn stub_18409a0() -> ! {
    todo!("0x18409a0 RBX::DragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")
}

// 0x18409b0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11LuaDragToolEPNS_12PartInstanceEN3G3D7Vector3ESt6vectorIN5boost8weak_ptrIS5_EESaISC_EEPNS_9WorkspaceENSA_10shared_ptrINS_8InstanceEEEEENSH_IT_EET0_T1_T2_T3_T4_
// type: void __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, const shared_count *, void *, char, int, int, int, int) // donor 0x2e08bc
#[doc(alias = "boost::shared_ptr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11LuaDragToolEPNS_12PartInstanceEN3G3D7Vector3ESt6vectorIN5boost8weak_ptrIS5_EESaISC_EEPNS_9WorkspaceENSA_10shared_ptrINS_8InstanceEEEEENSH_IT_EET0_T1_T2_T3_T4_")]
pub fn stub_18409b0() -> ! {
    todo!("0x18409b0 boost::shared_ptr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")
}

// 0x18409c0 — __ZN3RBX13DragUtilities13safeMoveYDropERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKN3G3D7Vector3ERNS_14ContactManagerEf
// type: void __fastcall(int, int, struct _Unwind_Exception *, int, float, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int) // donor 0x2e0f38
#[doc(alias = "RBX::DragUtilities::safeMoveYDrop(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
#[doc(alias = "__ZN3RBX13DragUtilities13safeMoveYDropERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKN3G3D7Vector3ERNS_14ContactManagerEf")]
pub fn stub_18409c0() -> ! {
    todo!("0x18409c0 RBX::DragUtilities::safeMoveYDrop(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")
}

// 0x18409d0 — __ZN3RBX13DragUtilities17partsToPrimitivesERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
// type: int __fastcall(__int64 *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int) // donor 0x2e10d8
#[doc(alias = "RBX::DragUtilities::partsToPrimitives(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::Array<RBX::Primitive *,10,32ul> &)")]
#[doc(alias = "__ZN3RBX13DragUtilities17partsToPrimitivesERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE")]
pub fn stub_18409d0() -> ! {
    todo!("0x18409d0 RBX::DragUtilities::partsToPrimitives(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::Array<RBX::Primitive *,10,32ul> &)")
}

// 0x18409e0 — __ZN3RBX13DragUtilities16hitObjectOrPlaneERKNS_14ContactManagerERKNS_6RbxRayEPKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS7_7Vector3Eb
// type: int __fastcall(int, const RBX::RbxRay *, int, const G3D::Vector3 *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int) // donor 0x2e13f0
#[doc(alias = "RBX::DragUtilities::hitObjectOrPlane(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)")]
#[doc(alias = "__ZN3RBX13DragUtilities16hitObjectOrPlaneERKNS_14ContactManagerERKNS_6RbxRayEPKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS7_7Vector3Eb")]
pub fn stub_18409e0() -> ! {
    todo!("0x18409e0 RBX::DragUtilities::hitObjectOrPlane(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)")
}

// 0x18409f0 — __ZN3RBX13DragUtilities9hitObjectERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKNS_6RbxRayERKNS_14ContactManagerERN3G3D7Vector3Eb
// type: int __fastcall(__int64 *, int, struct _Unwind_Exception *lpuexcpt, int, int, struct _Unwind_Exception *lpuexcpta, char, int, int, int, int, int, int, int) // donor 0x2e1628
#[doc(alias = "RBX::DragUtilities::hitObject(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,G3D::Vector3 &,bool)")]
#[doc(alias = "__ZN3RBX13DragUtilities9hitObjectERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKNS_6RbxRayERKNS_14ContactManagerERN3G3D7Vector3Eb")]
pub fn stub_18409f0() -> ! {
    todo!("0x18409f0 RBX::DragUtilities::hitObject(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,G3D::Vector3 &,bool)")
}

// 0x1840a00 — __ZN3RBX13DragUtilities9hitObjectERKNS_14ContactManagerERKNS_6RbxRayEPKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS7_7Vector3Eb
// type: int __fastcall(int, RBX::RbxRay *, int, const G3D::Vector3 *, int) // donor 0x2e1708
#[doc(alias = "RBX::DragUtilities::hitObject(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)")]
#[doc(alias = "__ZN3RBX13DragUtilities9hitObjectERKNS_14ContactManagerERKNS_6RbxRayEPKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS7_7Vector3Eb")]
pub fn stub_1840a00() -> ! {
    todo!("0x1840a00 RBX::DragUtilities::hitObject(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)")
}

// 0x1840a10 — __ZN3RBX13DragUtilities12moveAndCleanEPNS_12PartInstanceERKN3G3D7Vector3E
// type: int __fastcall(RBX::DragUtilities *this, RBX::PartInstance *, const G3D::Vector3 *) // donor 0x2e2300
#[doc(alias = "RBX::DragUtilities::moveAndClean(RBX::PartInstance *,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX13DragUtilities12moveAndCleanEPNS_12PartInstanceERKN3G3D7Vector3E")]
pub fn stub_1840a10() -> ! {
    todo!("0x1840a10 RBX::DragUtilities::moveAndClean(RBX::PartInstance *,G3D::Vector3 const&)")
}

// 0x1840a20 — __ZN3RBX13DragUtilities4moveERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EEN3G3D15CoordinateFrameESB_
// type: void __fastcall(int *, _DWORD *, int) // donor 0x2e24f0
#[doc(alias = "RBX::DragUtilities::move(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::CoordinateFrame,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX13DragUtilities4moveERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EEN3G3D15CoordinateFrameESB_")]
pub fn stub_1840a20() -> ! {
    todo!("0x1840a20 RBX::DragUtilities::move(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::CoordinateFrame,G3D::CoordinateFrame)")
}

// 0x1840a30 — __ZN3RBX13DragUtilities6toGridERKN3G3D7Vector3ES4_
// type: int __fastcall(RBX::DragUtilities *this, const G3D::Vector3 *, const G3D::Vector3 *) // donor 0x2e26d4
#[doc(alias = "RBX::DragUtilities::toGrid(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX13DragUtilities6toGridERKN3G3D7Vector3ES4_")]
pub fn stub_1840a30() -> ! {
    todo!("0x1840a30 RBX::DragUtilities::toGrid(G3D::Vector3 const&,G3D::Vector3 const&)")
}

// 0x1840a40 — __ZNK3RBX8GameTool13draggablePartEPKNS_12PartInstanceERKN3G3D7Vector3E
// type: int __fastcall(RBX::GameTool *this, const RBX::PartInstance *, const G3D::Vector3 *) // donor 0x2e304c
#[doc(alias = "RBX::GameTool::draggablePart(RBX::PartInstance const*,G3D::Vector3 const&)const")]
#[doc(alias = "__ZNK3RBX8GameTool13draggablePartEPKNS_12PartInstanceERKN3G3D7Vector3E")]
pub fn stub_1840a40() -> ! {
    todo!("0x1840a40 RBX::GameTool::draggablePart(RBX::PartInstance const*,G3D::Vector3 const&)const")
}

// 0x1840a50 — __ZN3RBX10LuaDragger15mouseDownPublicEN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS2_IKSt6vectorIS4_SaIS4_EEEE
// type: void __fastcall(int, int *, int, int, int, int) // donor 0x2e51d0
#[doc(alias = "RBX::LuaDragger::mouseDownPublic(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>)")]
#[doc(alias = "__ZN3RBX10LuaDragger15mouseDownPublicEN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS2_IKSt6vectorIS4_SaIS4_EEEE")]
pub fn stub_1840a50() -> ! {
    todo!("0x1840a50 RBX::LuaDragger::mouseDownPublic(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>)")
}

// 0x1840a60 — __ZN3RBX10LuaDragger10axisRotateEN3G3D7Vector34AxisE
// type: _DWORD __fastcall(RBX::LuaDragger *__hidden this, Axis) // donor 0x2e5b88
#[doc(alias = "RBX::LuaDragger::axisRotate(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3RBX10LuaDragger10axisRotateEN3G3D7Vector34AxisE")]
pub fn stub_1840a60() -> ! {
    todo!("0x1840a60 RBX::LuaDragger::axisRotate(G3D::Vector3::Axis)")
}

// 0x1840a70 — __ZN3RBX10LuaDragger9mouseDownEN5boost10shared_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorINS1_8weak_ptrIS3_EESaISB_EE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD) // donor 0x2e6070
#[doc(alias = "RBX::LuaDragger::mouseDown(boost::shared_ptr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>)")]
#[doc(alias = "__ZN3RBX10LuaDragger9mouseDownEN5boost10shared_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorINS1_8weak_ptrIS3_EESaISB_EE")]
pub fn stub_1840a70() -> ! {
    todo!("0x1840a70 RBX::LuaDragger::mouseDown(boost::shared_ptr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>)")
}

// 0x1840a80 — __ZN3RBX10LuaDragger15getSnapHitPointEPNS_12PartInstanceERKNS_6RbxRayERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::LuaDragger *__hidden this, RBX::PartInstance *, const RBX::RbxRay *, G3D::Vector3 *) // donor 0x2e67a4
#[doc(alias = "RBX::LuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX10LuaDragger15getSnapHitPointEPNS_12PartInstanceERKNS_6RbxRayERN3G3D7Vector3E")]
pub fn stub_1840a80() -> ! {
    todo!("0x1840a80 RBX::LuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)")
}

// 0x1840a90 — __ZN3RBX10LuaDragger16rotateOnSnapFaceEN3G3D7Vector34AxisERKNS1_7Matrix3E
// type: _DWORD __fastcall(RBX::LuaDragger *__hidden this, Axis, const G3D::Matrix3 *) // donor 0x2e6b88
#[doc(alias = "RBX::LuaDragger::rotateOnSnapFace(G3D::Vector3::Axis,G3D::Matrix3 const&)")]
#[doc(alias = "__ZN3RBX10LuaDragger16rotateOnSnapFaceEN3G3D7Vector34AxisERKNS1_7Matrix3E")]
pub fn stub_1840a90() -> ! {
    todo!("0x1840a90 RBX::LuaDragger::rotateOnSnapFace(G3D::Vector3::Axis,G3D::Matrix3 const&)")
}

// 0x1840aa0 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED1Ev
// type:  // donor 0x2e700c
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED1Ev")]
pub fn stub_1840aa0() -> ! {
    todo!("0x1840aa0 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")
}

// 0x1840ab0 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EED1Ev
// type:  // donor 0x2e712c
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EED1Ev")]
pub fn stub_1840ab0() -> ! {
    todo!("0x1840ab0 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::~BoundFuncDesc()")
}

// 0x1840ac0 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EEC2EMS2_FvS5_EPKcSB_S5_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type:  // donor 0x2e8528
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(G3D::Vector3::Axis),char const*,char const*,G3D::Vector3::Axis,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EEC2EMS2_FvS5_EPKcSB_S5_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_1840ac0() -> ! {
    todo!("0x1840ac0 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(G3D::Vector3::Axis),char const*,char const*,G3D::Vector3::Axis,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x1840ad0 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EE16declareSignatureEPKcNS0_7VariantE
// type:  // donor 0x2e86d4
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_1840ad0() -> ! {
    todo!("0x1840ad0 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x1840ae0 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EED0Ev
// type:  // donor 0x2e8704
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EED0Ev")]
pub fn stub_1840ae0() -> ! {
    todo!("0x1840ae0 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::~BoundFuncDesc()")
}

// 0x1840af0 — __ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type:  // donor 0x2e87d8
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_1840af0() -> ! {
    todo!("0x1840af0 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x1840b00 — __ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector34AxisELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS6_EEPNSA_10disable_ifINSA_7is_sameIS6_NSA_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type:  // donor 0x2e880c
#[doc(alias = "G3D::Vector3::Axis RBX::Reflection::ArgHelper::getArg<G3D::Vector3::Axis,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3::Axis> const&,boost::disable_if<boost::is_same<G3D::Vector3::Axis,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector34AxisELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS6_EEPNSA_10disable_ifINSA_7is_sameIS6_NSA_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_1840b00() -> ! {
    todo!("0x1840b00 G3D::Vector3::Axis RBX::Reflection::ArgHelper::getArg<G3D::Vector3::Axis,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3::Axis> const&,boost::disable_if<boost::is_same<G3D::Vector3::Axis,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x1840b10 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1EN3G3D7Vector34AxisEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSB_7is_enumIS9_EEvE4typeE
// type:  // donor 0x2e899c
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,G3D::Vector3::Axis>(RBX::Reflection::FunctionDescriptor::Arguments &,G3D::Vector3::Axis &,boost::enable_if<boost::is_enum<G3D::Vector3::Axis>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi1EN3G3D7Vector34AxisEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSB_7is_enumIS9_EEvE4typeE")]
pub fn stub_1840b10() -> ! {
    todo!("0x1840b10 bool RBX::Reflection::ArgHelper::try_enum<1,G3D::Vector3::Axis>(RBX::Reflection::FunctionDescriptor::Arguments &,G3D::Vector3::Axis &,boost::enable_if<boost::is_enum<G3D::Vector3::Axis>,void>::type *)")
}

// 0x1840b20 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EEC2EMS2_FvS6_S8_SD_EPKcSJ_SJ_SJ_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type:  // donor 0x2e8ee8
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::BoundFuncDesc(void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EEC2EMS2_FvS6_S8_SD_EPKcSJ_SJ_SJ_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_1840b20() -> ! {
    todo!("0x1840b20 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::BoundFuncDesc(void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x1840b30 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EE16declareSignatureEPKcNS0_7VariantESH_SI_SH_SI_
// type:  // donor 0x2e9144
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EE16declareSignatureEPKcNS0_7VariantESH_SI_SH_SI_")]
pub fn stub_1840b30() -> ! {
    todo!("0x1840b30 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x1840b40 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED0Ev
// type: int __fastcall(int, int, int, int, int) // donor 0x2e91ac
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED0Ev")]
pub fn stub_1840b40() -> ! {
    todo!("0x1840b40 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")
}

// 0x1840b50 — __ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int) // donor 0x2e924c
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_1840b50() -> ! {
    todo!("0x1840b50 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x1840b60 — __ZN3RBX10Reflection11Call3HelperINS_10LuaDraggerEMS2_FvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEES6_S8_SD_vE4callEPS2_SF_RNS0_7VariantERKS6_RKS8_RKSD_
// type: int __fastcall(int, int, int, int, char, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int) // donor 0x2e9388
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::LuaDragger,void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,void>::call(RBX::LuaDragger*,void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,G3D::Vector3 const&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_10LuaDraggerEMS2_FvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEES6_S8_SD_vE4callEPS2_SF_RNS0_7VariantERKS6_RKS8_RKSD_")]
pub fn stub_1840b60() -> ! {
    todo!("0x1840b60 RBX::Reflection::Call3Helper<RBX::LuaDragger,void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,void>::call(RBX::LuaDragger*,void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,G3D::Vector3 const&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&)")
}

// 0x1840b70 — __ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector3ELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type:  // donor 0x2e94dc
#[doc(alias = "G3D::Vector3 RBX::Reflection::ArgHelper::getArg<G3D::Vector3,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3> const&,boost::disable_if<boost::is_same<G3D::Vector3,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector3ELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_1840b70() -> ! {
    todo!("0x1840b70 G3D::Vector3 RBX::Reflection::ArgHelper::getArg<G3D::Vector3,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3> const&,boost::disable_if<boost::is_same<G3D::Vector3,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x1840b80 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED2Ev
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, void *, int, int, int, int) // donor 0x2e998c
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED2Ev")]
pub fn stub_1840b80() -> ! {
    todo!("0x1840b80 RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")
}

// 0x1840b90 — __ZN3RBX11LuaDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD) // donor 0x2e9f80
#[doc(alias = "RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX11LuaDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE")]
pub fn stub_1840b90() -> ! {
    todo!("0x1840b90 RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")
}

// 0x1840ba0 — __ZN3RBX11LuaDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int, int, int, Workspace *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, RBX::MouseCommand *, int, int, int, int) // donor 0x2e9f84
#[doc(alias = "RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX11LuaDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE")]
pub fn stub_1840ba0() -> ! {
    todo!("0x1840ba0 RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")
}

// 0x1840bb0 — __ZN3RBX11MegaDragger13safeMoveYDropERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this, const G3D::Vector3 *) // donor 0x2eb604
#[doc(alias = "RBX::MegaDragger::safeMoveYDrop(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX11MegaDragger13safeMoveYDropERKN3G3D7Vector3E")]
pub fn stub_1840bb0() -> ! {
    todo!("0x1840bb0 RBX::MegaDragger::safeMoveYDrop(G3D::Vector3 const&)")
}

// 0x1840bc0 — __ZN3RBX11MegaDragger15getPartsForDragERN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
// type: int __fastcall(int, int) // donor 0x2eb680
#[doc(alias = "RBX::MegaDragger::getPartsForDrag(G3D::Array<RBX::Primitive *,10,32ul> &)")]
#[doc(alias = "__ZN3RBX11MegaDragger15getPartsForDragERN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE")]
pub fn stub_1840bc0() -> ! {
    todo!("0x1840bc0 RBX::MegaDragger::getPartsForDrag(G3D::Array<RBX::Primitive *,10,32ul> &)")
}

// 0x1840bd0 — __ZN3RBX11MegaDragger14safeMoveNoDropERKN3G3D7Vector3E
// type: void __fastcall(RBX::MegaDragger *this, const G3D::Vector3 *, float *) // donor 0x2eb734
#[doc(alias = "RBX::MegaDragger::safeMoveNoDrop(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX11MegaDragger14safeMoveNoDropERKN3G3D7Vector3E")]
pub fn stub_1840bd0() -> ! {
    todo!("0x1840bd0 RBX::MegaDragger::safeMoveNoDrop(G3D::Vector3 const&)")
}

// 0x1840be0 — __ZN3RBX11MegaDragger17safeMoveAlongLineERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this, const G3D::Vector3 *) // donor 0x2eb87c
#[doc(alias = "RBX::MegaDragger::safeMoveAlongLine(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX11MegaDragger17safeMoveAlongLineERKN3G3D7Vector3E")]
pub fn stub_1840be0() -> ! {
    todo!("0x1840be0 RBX::MegaDragger::safeMoveAlongLine(G3D::Vector3 const&)")
}

// 0x1840bf0 — __ZN3RBX11MegaDragger22moveSafePlaceAlongLineERKN3G3D7Vector3E
// type: _DWORD __fastcall(struct _Unwind_Exception *lpuexcpt, const G3D::Vector3 *) // donor 0x2eba30
#[doc(alias = "RBX::MegaDragger::moveSafePlaceAlongLine(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX11MegaDragger22moveSafePlaceAlongLineERKN3G3D7Vector3E")]
pub fn stub_1840bf0() -> ! {
    todo!("0x1840bf0 RBX::MegaDragger::moveSafePlaceAlongLine(G3D::Vector3 const&)")
}

// 0x1840c00 — __ZN3RBX11MegaDragger13moveAlongLineERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this, const G3D::Vector3 *) // donor 0x2ebc38
#[doc(alias = "RBX::MegaDragger::moveAlongLine(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX11MegaDragger13moveAlongLineERKN3G3D7Vector3E")]
pub fn stub_1840c00() -> ! {
    todo!("0x1840c00 RBX::MegaDragger::moveAlongLine(G3D::Vector3 const&)")
}

// 0x1840c10 — __ZN3RBX11MegaDragger19safeRotateAlongLineERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this, const G3D::Vector3 *) // donor 0x2ebc44
#[doc(alias = "RBX::MegaDragger::safeRotateAlongLine(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX11MegaDragger19safeRotateAlongLineERKN3G3D7Vector3E")]
pub fn stub_1840c10() -> ! {
    todo!("0x1840c10 RBX::MegaDragger::safeRotateAlongLine(G3D::Vector3 const&)")
}

// 0x1840c20 — __ZN3RBX11MegaDragger10safeRotateERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this, const G3D::Matrix3 *) // donor 0x2ebd7c
#[doc(alias = "RBX::MegaDragger::safeRotate(G3D::Matrix3 const&)")]
#[doc(alias = "__ZN3RBX11MegaDragger10safeRotateERKN3G3D7Matrix3E")]
pub fn stub_1840c20() -> ! {
    todo!("0x1840c20 RBX::MegaDragger::safeRotate(G3D::Matrix3 const&)")
}

// 0x1840c30 — __ZN3RBX11MegaDragger15rotateDragPartsERKN3G3D7Matrix3Eb
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this, const G3D::Matrix3 *, G3D::Matrix3 *) // donor 0x2ebf24
#[doc(alias = "RBX::MegaDragger::rotateDragParts(G3D::Matrix3 const&,bool)")]
#[doc(alias = "__ZN3RBX11MegaDragger15rotateDragPartsERKN3G3D7Matrix3Eb")]
pub fn stub_1840c30() -> ! {
    todo!("0x1840c30 RBX::MegaDragger::rotateDragParts(G3D::Matrix3 const&,bool)")
}

// 0x1840c40 — __ZN3RBX11NewNullTool16getIndicatedPartERKNS_7UIEventERKbPPNS_12PartInstanceEPbPN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this, const RBX::UIEvent *, const bool *, RBX::PartInstance **, bool *, G3D::Vector3 *) // donor 0x2ef364
#[doc(alias = "RBX::NewNullTool::getIndicatedPart(RBX::UIEvent const&,bool const&,RBX::PartInstance **,bool *,G3D::Vector3 *)")]
#[doc(alias = "__ZN3RBX11NewNullTool16getIndicatedPartERKNS_7UIEventERKbPPNS_12PartInstanceEPbPN3G3D7Vector3E")]
pub fn stub_1840c40() -> ! {
    todo!("0x1840c40 RBX::NewNullTool::getIndicatedPart(RBX::UIEvent const&,bool const&,RBX::PartInstance **,bool *,G3D::Vector3 *)")
}

// 0x1840c50 — __ZN3RBX12PartDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEE
// type:  // donor 0x2f0948
#[doc(alias = "RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX12PartDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_1840c50() -> ! {
    todo!("0x1840c50 RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")
}

// 0x1840c60 — __ZN3RBX12PartDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int, int, Workspace *, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, RBX::MouseCommand *, int, int, void *, void *, int, int, int, int) // donor 0x2f094c
#[doc(alias = "RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX12PartDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_1840c60() -> ! {
    todo!("0x1840c60 RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")
}

// 0x1840c70 — __ZN3RBX10RunDragger9initLocalEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3E
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int) // donor 0x2f2bf0
#[doc(alias = "RBX::RunDragger::initLocal(RBX::Workspace *,boost::weak_ptr<RBX::PartInstance>,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX10RunDragger9initLocalEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3E")]
pub fn stub_1840c70() -> ! {
    todo!("0x1840c70 RBX::RunDragger::initLocal(RBX::Workspace *,boost::weak_ptr<RBX::PartInstance>,G3D::Vector3 const&)")
}

// 0x1840c80 — __ZN3RBX10RunDragger4initEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3E
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int) // donor 0x2f2ff8
#[doc(alias = "RBX::RunDragger::init(RBX::Workspace *,boost::weak_ptr<RBX::PartInstance>,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX10RunDragger4initEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3E")]
pub fn stub_1840c80() -> ! {
    todo!("0x1840c80 RBX::RunDragger::init(RBX::Workspace *,boost::weak_ptr<RBX::PartInstance>,G3D::Vector3 const&)")
}

// 0x1840c90 — __ZN3RBX10RunDragger17createSnapSurfaceEPNS_9PrimitiveEPN3G3D5ArrayImLi10ELm32EEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int) // donor 0x2f33e0
#[doc(alias = "RBX::RunDragger::createSnapSurface(RBX::Primitive *,G3D::Array<unsigned long,10,32ul> *)")]
#[doc(alias = "__ZN3RBX10RunDragger17createSnapSurfaceEPNS_9PrimitiveEPN3G3D5ArrayImLi10ELm32EEE")]
pub fn stub_1840c90() -> ! {
    todo!("0x1840c90 RBX::RunDragger::createSnapSurface(RBX::Primitive *,G3D::Array<unsigned long,10,32ul> *)")
}

// 0x1840ca0 — __ZN3RBX10RunDragger8notTriedEPNS_9PrimitiveERKN3G3D5ArrayIS2_Li10ELm32EEE
// type:  // donor 0x2f4630
#[doc(alias = "RBX::RunDragger::notTried(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> const&)")]
#[doc(alias = "__ZN3RBX10RunDragger8notTriedEPNS_9PrimitiveERKN3G3D5ArrayIS2_Li10ELm32EEE")]
pub fn stub_1840ca0() -> ! {
    todo!("0x1840ca0 RBX::RunDragger::notTried(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> const&)")
}

// 0x1840cb0 — __ZN3RBX10RunDragger11rayHitsPartERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEb
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int) // donor 0x2f4700
#[doc(alias = "RBX::RunDragger::rayHitsPart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool)")]
#[doc(alias = "__ZN3RBX10RunDragger11rayHitsPartERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEb")]
pub fn stub_1840cb0() -> ! {
    todo!("0x1840cb0 RBX::RunDragger::rayHitsPart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool)")
}

// 0x1840cc0 — __ZN3RBX10RunDragger17bestProximatePartERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEMNS_7ContactEFbfE
// type: int __fastcall(int, int, int, int, int) // donor 0x2f495c
#[doc(alias = "RBX::RunDragger::bestProximatePart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool (RBX::Contact::*)(float))")]
#[doc(alias = "__ZN3RBX10RunDragger17bestProximatePartERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEMNS_7ContactEFbfE")]
pub fn stub_1840cc0() -> ! {
    todo!("0x1840cc0 RBX::RunDragger::bestProximatePart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool (RBX::Contact::*)(float))")
}

// 0x1840cd0 — __ZN3RBX10RunDragger8findSnapERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
// type:  // donor 0x2f4eac
#[doc(alias = "RBX::RunDragger::findSnap(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
#[doc(alias = "__ZN3RBX10RunDragger8findSnapERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE")]
pub fn stub_1840cd0() -> ! {
    todo!("0x1840cd0 RBX::RunDragger::findSnap(G3D::Array<RBX::Primitive *,10,32ul> const&)")
}

// 0x1840ce0 — __ZN3RBX10RunDragger18findNoSnapPositionERKN3G3D15CoordinateFrameE
// type: void __fastcall(RBX::RunDragger *this, const G3D::CoordinateFrame *) // donor 0x2f5018
#[doc(alias = "RBX::RunDragger::findNoSnapPosition(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX10RunDragger18findNoSnapPositionERKN3G3D15CoordinateFrameE")]
pub fn stub_1840ce0() -> ! {
    todo!("0x1840ce0 RBX::RunDragger::findNoSnapPosition(G3D::CoordinateFrame const&)")
}

// 0x1840cf0 — __ZN3RBX10RunDragger32rotatePart90DegAboutSnapFaceAxisEN3G3D7Vector34AxisE
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this, Axis) // donor 0x2f53f4
#[doc(alias = "RBX::RunDragger::rotatePart90DegAboutSnapFaceAxis(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3RBX10RunDragger32rotatePart90DegAboutSnapFaceAxisEN3G3D7Vector34AxisE")]
pub fn stub_1840cf0() -> ! {
    todo!("0x1840cf0 RBX::RunDragger::rotatePart90DegAboutSnapFaceAxis(G3D::Vector3::Axis)")
}

// 0x1840d00 — __ZN3RBX10RunDragger27rotatePartAboutSnapFaceAxisEN3G3D7Vector34AxisERKf
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this, Axis, const float *) // donor 0x2f5410
#[doc(alias = "RBX::RunDragger::rotatePartAboutSnapFaceAxis(G3D::Vector3::Axis,float const&)")]
#[doc(alias = "__ZN3RBX10RunDragger27rotatePartAboutSnapFaceAxisEN3G3D7Vector34AxisERKf")]
pub fn stub_1840d00() -> ! {
    todo!("0x1840d00 RBX::RunDragger::rotatePartAboutSnapFaceAxis(G3D::Vector3::Axis,float const&)")
}

// 0x1840d10 — __ZN3G3D5ArrayImLi10ELm32EE6appendERKm
// type:  // donor 0x2f587c
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::append(unsigned long const&)")]
#[doc(alias = "__ZN3G3D5ArrayImLi10ELm32EE6appendERKm")]
pub fn stub_1840d10() -> ! {
    todo!("0x1840d10 G3D::Array<unsigned long,10,32ul>::append(unsigned long const&)")
}

// 0x1840d20 — __ZN3G3D5ArrayImLi10ELm32EE6resizeEib
// type:  // donor 0x2f58d8
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayImLi10ELm32EE6resizeEib")]
pub fn stub_1840d20() -> ! {
    todo!("0x1840d20 G3D::Array<unsigned long,10,32ul>::resize(int,bool)")
}

// 0x1840d30 — __ZN3G3D5ArrayImLi10ELm32EE7reallocEi
// type:  // donor 0x2f5990
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayImLi10ELm32EE7reallocEi")]
pub fn stub_1840d30() -> ! {
    todo!("0x1840d30 G3D::Array<unsigned long,10,32ul>::realloc(int)")
}

// 0x1840d40 — __ZN3G3D5ArrayImLi10ELm32EED2Ev
// type:  // donor 0x2f5b78
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayImLi10ELm32EED2Ev")]
pub fn stub_1840d40() -> ! {
    todo!("0x1840d40 G3D::Array<unsigned long,10,32ul>::~Array()")
}

// 0x1840d50 — __ZN3G3D5ArrayImLi10ELm32EEC2Ev
// type:  // donor 0x2f5c4c
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayImLi10ELm32EEC2Ev")]
pub fn stub_1840d50() -> ! {
    todo!("0x1840d50 G3D::Array<unsigned long,10,32ul>::Array(void)")
}

// 0x1840d60 — __ZN3RBX16BoxSelectCommand17getMouseInstancesERSt3setIN5boost10shared_ptrINS_8InstanceEEESt4lessIS5_ESaIS5_EERKNS_7UIEventERN3G3D6Rect2DE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int) // donor 0x2f75e8
#[doc(alias = "RBX::BoxSelectCommand::getMouseInstances(std::set<boost::shared_ptr<RBX::Instance>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>> &,RBX::UIEvent const&,G3D::Rect2D &)")]
#[doc(alias = "__ZN3RBX16BoxSelectCommand17getMouseInstancesERSt3setIN5boost10shared_ptrINS_8InstanceEEESt4lessIS5_ESaIS5_EERKNS_7UIEventERN3G3D6Rect2DE")]
pub fn stub_1840d60() -> ! {
    todo!("0x1840d60 RBX::BoxSelectCommand::getMouseInstances(std::set<boost::shared_ptr<RBX::Instance>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>> &,RBX::UIEvent const&,G3D::Rect2D &)")
}

// 0x1840d70 — __ZN3RBX4Axes14axisToNormalIdEN3G3D7Vector34AxisE
// type: _DWORD __fastcall(RBX::Axes *__hidden this, Axis) // donor 0x302ed8
#[doc(alias = "RBX::Axes::axisToNormalId(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3RBX4Axes14axisToNormalIdEN3G3D7Vector34AxisE")]
pub fn stub_1840d70() -> ! {
    todo!("0x1840d70 RBX::Axes::axisToNormalId(G3D::Vector3::Axis)")
}

// 0x1840d80 — __ZN3RBX4Axes10axisToMaskEN3G3D7Vector34AxisE
// type: _DWORD __fastcall(RBX::Axes *__hidden this, Axis) // donor 0x302ee0
#[doc(alias = "RBX::Axes::axisToMask(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3RBX4Axes10axisToMaskEN3G3D7Vector34AxisE")]
pub fn stub_1840d80() -> ! {
    todo!("0x1840d80 RBX::Axes::axisToMask(G3D::Vector3::Axis)")
}

// 0x1840d90 — __ZNK3RBX4Axes7getAxisEN3G3D7Vector34AxisE
// type: _DWORD __fastcall(RBX::Axes *__hidden this, Axis) // donor 0x302f1c
#[doc(alias = "RBX::Axes::getAxis(G3D::Vector3::Axis)const")]
#[doc(alias = "__ZNK3RBX4Axes7getAxisEN3G3D7Vector34AxisE")]
pub fn stub_1840d90() -> ! {
    todo!("0x1840d90 RBX::Axes::getAxis(G3D::Vector3::Axis)const")
}

// 0x1840da0 — __ZN3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEEC1Ev
// type: int(void) // donor 0x303124
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEEC1Ev")]
pub fn stub_1840da0() -> ! {
    todo!("0x1840da0 RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::EnumDesc(void)")
}

// 0x1840db0 — __ZN3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEEC2Ev
// type:  // donor 0x303128
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEEC2Ev")]
pub fn stub_1840db0() -> ! {
    todo!("0x1840db0 RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::EnumDesc(void)")
}

// 0x1840dc0 — __ZN3RBX10Reflection7Variant7convertIN3G3D7Vector34AxisEEERT_v
// type:  // donor 0x303300
#[doc(alias = "G3D::Vector3::Axis & RBX::Reflection::Variant::convert<G3D::Vector3::Axis>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant7convertIN3G3D7Vector34AxisEEERT_v")]
pub fn stub_1840dc0() -> ! {
    todo!("0x1840dc0 G3D::Vector3::Axis & RBX::Reflection::Variant::convert<G3D::Vector3::Axis>(void)")
}

// 0x1840dd0 — __ZN3RBX15StringConverterIN3G3D7Vector34AxisEE14convertToValueERKSsRS3_
// type:  // donor 0x303304
#[doc(alias = "RBX::StringConverter<G3D::Vector3::Axis>::convertToValue(std::string const&,G3D::Vector3::Axis&)")]
#[doc(alias = "__ZN3RBX15StringConverterIN3G3D7Vector34AxisEE14convertToValueERKSsRS3_")]
pub fn stub_1840dd0() -> ! {
    todo!("0x1840dd0 RBX::StringConverter<G3D::Vector3::Axis>::convertToValue(std::string const&,G3D::Vector3::Axis&)")
}

// 0x1840de0 — __ZN3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEE7addPairES4_PKc
// type: int(void) // donor 0x30367c
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::addPair(G3D::Vector3::Axis,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEE7addPairES4_PKc")]
pub fn stub_1840de0() -> ! {
    todo!("0x1840de0 RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::addPair(G3D::Vector3::Axis,char const*)")
}

// 0x1840df0 — __ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector34AxisEEERT_v
// type: int(void) // donor 0x3039dc
#[doc(alias = "G3D::Vector3::Axis & RBX::Reflection::Variant::genericConvert<G3D::Vector3::Axis>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector34AxisEEERT_v")]
pub fn stub_1840df0() -> ! {
    todo!("0x1840df0 G3D::Vector3::Axis & RBX::Reflection::Variant::genericConvert<G3D::Vector3::Axis>(void)")
}

// 0x1840e00 — __ZN3rbx8any_castIN3G3D7Vector34AxisEN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
// type:  // donor 0x303bc8
#[doc(alias = "G3D::Vector3::Axis * rbx::any_cast<G3D::Vector3::Axis,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D7Vector34AxisEN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_1840e00() -> ! {
    todo!("0x1840e00 G3D::Vector3::Axis * rbx::any_cast<G3D::Vector3::Axis,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x1840e10 — __ZN3rbx8any_castIRN3G3D7Vector34AxisEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void) // donor 0x303c20
#[doc(alias = "G3D::Vector3::Axis & rbx::any_cast<G3D::Vector3::Axis &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3G3D7Vector34AxisEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_1840e10() -> ! {
    todo!("0x1840e10 G3D::Vector3::Axis & rbx::any_cast<G3D::Vector3::Axis &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x1840e20 — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int) // donor 0x303d10
#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::resize(unsigned long,G3D::Vector3::Axis)")]
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE6resizeEmS2_")]
pub fn stub_1840e20() -> ! {
    todo!("0x1840e20 std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::resize(unsigned long,G3D::Vector3::Axis)")
}

// 0x1840e30 — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *) // donor 0x303d44
#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::push_back(G3D::Vector3::Axis const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE9push_backERKS2_")]
pub fn stub_1840e30() -> ! {
    todo!("0x1840e30 std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::push_back(G3D::Vector3::Axis const&)")
}

// 0x1840e40 — __ZNSt3mapIPKN3RBX4NameEN3G3D7Vector34AxisESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_
// type: int(void) // donor 0x303d6c
#[doc(alias = "std::map<RBX::Name const*,G3D::Vector3::Axis,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameEN3G3D7Vector34AxisESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_")]
pub fn stub_1840e40() -> ! {
    todo!("0x1840e40 std::map<RBX::Name const*,G3D::Vector3::Axis,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::operator[](RBX::Name const* const&)")
}

// 0x1840e50 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, _Rb_tree_node_base *) // donor 0x303dc4
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")]
pub fn stub_1840e50() -> ! {
    todo!("0x1840e50 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)")
}

// 0x1840e60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: int(void) // donor 0x303e78
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")]
pub fn stub_1840e60() -> ! {
    todo!("0x1840e60 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)")
}

// 0x1840e70 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int(void) // donor 0x303ed0
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert_unique(std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_")]
pub fn stub_1840e70() -> ! {
    todo!("0x1840e70 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert_unique(std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)")
}

// 0x1840e80 — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void) // donor 0x303f38
#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector3::Axis*,std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>>,G3D::Vector3::Axis const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_1840e80() -> ! {
    todo!("0x1840e80 std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector3::Axis*,std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>>,G3D::Vector3::Axis const&)")
}

// 0x1840e90 — __ZNSt12_Vector_baseIN3G3D7Vector34AxisESaIS2_EE11_M_allocateEm
// type: int(void) // donor 0x30401c
#[doc(alias = "std::_Vector_base<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3G3D7Vector34AxisESaIS2_EE11_M_allocateEm")]
pub fn stub_1840e90() -> ! {
    todo!("0x1840e90 std::_Vector_base<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_allocate(unsigned long)")
}

// 0x1840ea0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Vector34AxisES6_EET0_T_S8_S7_
// type: int(void) // donor 0x304034
#[doc(alias = "G3D::Vector3::Axis * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3::Axis *,G3D::Vector3::Axis *>(G3D::Vector3::Axis *,G3D::Vector3::Axis *,G3D::Vector3::Axis *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Vector34AxisES6_EET0_T_S8_S7_")]
pub fn stub_1840ea0() -> ! {
    todo!("0x1840ea0 G3D::Vector3::Axis * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3::Axis *,G3D::Vector3::Axis *>(G3D::Vector3::Axis *,G3D::Vector3::Axis *,G3D::Vector3::Axis *)")
}

// 0x1840eb0 — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(_DWORD) // donor 0x304070
#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Vector3::Axis*,std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>>,unsigned long,G3D::Vector3::Axis const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_1840eb0() -> ! {
    todo!("0x1840eb0 std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Vector3::Axis*,std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>>,unsigned long,G3D::Vector3::Axis const&)")
}

// 0x1840ec0 — __ZN3RBX10BrickColor7closestEN3G3D6Color3E
// type: int __fastcall(_DWORD, _DWORD) // donor 0x3044a0
#[doc(alias = "RBX::BrickColor::closest(G3D::Color3)")]
#[doc(alias = "__ZN3RBX10BrickColor7closestEN3G3D6Color3E")]
pub fn stub_1840ec0() -> ! {
    todo!("0x1840ec0 RBX::BrickColor::closest(G3D::Color3)")
}

// 0x1840ed0 — __ZN3RBX10BrickColor7closestEN3G3D6Color4E
// type: int __fastcall(_DWORD, _DWORD) // donor 0x3044c4
#[doc(alias = "RBX::BrickColor::closest(G3D::Color4)")]
#[doc(alias = "__ZN3RBX10BrickColor7closestEN3G3D6Color4E")]
pub fn stub_1840ed0() -> ! {
    todo!("0x1840ed0 RBX::BrickColor::closest(G3D::Color4)")
}

// 0x1840ee0 — __ZN3RBX13CameraSubject11doOcclusionERN3G3D7Vector3ERNS1_15CoordinateFrameEf
// type: _DWORD __fastcall(RBX::CameraSubject *__hidden this, Vector3 *, G3D::CoordinateFrame *, float) // donor 0x30dd94
#[doc(alias = "RBX::CameraSubject::doOcclusion(G3D::Vector3 &,G3D::CoordinateFrame &,float)")]
#[doc(alias = "__ZN3RBX13CameraSubject11doOcclusionERN3G3D7Vector3ERNS1_15CoordinateFrameEf")]
pub fn stub_1840ee0() -> ! {
    todo!("0x1840ee0 RBX::CameraSubject::doOcclusion(G3D::Vector3 &,G3D::CoordinateFrame &,float)")
}

// 0x1840ef0 — __ZN3RBX13CameraSubject13testOcclusionERKN3G3D7Vector3ERKNS1_15CoordinateFrameERf
// type: _DWORD __fastcall(RBX::CameraSubject *__hidden this, const G3D::Vector3 *, const G3D::CoordinateFrame *, float *) // donor 0x30de2c
#[doc(alias = "RBX::CameraSubject::testOcclusion(G3D::Vector3 const&,G3D::CoordinateFrame const&,float &)")]
#[doc(alias = "__ZN3RBX13CameraSubject13testOcclusionERKN3G3D7Vector3ERKNS1_15CoordinateFrameERf")]
pub fn stub_1840ef0() -> ! {
    todo!("0x1840ef0 RBX::CameraSubject::testOcclusion(G3D::Vector3 const&,G3D::CoordinateFrame const&,float &)")
}
