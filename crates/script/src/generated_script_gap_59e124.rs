//! Auto-generated skeletons for rbx-script — gap filler EA asc 0x59e124..0x5a42d4
//! Source: ida/export.json (85545 funcs), DAG: script crate (RBX::Script, Lua, Yield + gap filler)
//! Batch: +100 stubs | range 0x59e124..0x5a42d4 | filtered 5401 already complete, gap filler distinct not yet in crates/script/src (global EA asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + #[doc(alias)] + todo)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x59e124 — __ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(std::string,RBX::ContentId)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEED2Ev")]
pub fn stub_0x59e124() -> crate::slot::PortedFn {
// IDA 0x59e124: rbx::remote_signal<void (std::string, RBX::ContentId)>::~remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x59e124, "rbx::remote_signal<void (std::string, RBX::ContentId)>::~remote_signal()")
}

// 0x59e270 — __ZN3rbx13remote_signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEED2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEED2Ev")]
pub fn stub_0x59e270(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x59e3bc — __ZN3rbx13remote_signalIFvSsSsEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvSsSsEED2Ev")]
pub fn stub_0x59e3bc() -> crate::slot::PortedFn {
// IDA 0x59e3bc: rbx::remote_signal<void (std::string, std::string)>::~remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x59e3bc, "rbx::remote_signal<void (std::string, std::string)>::~remote_signal()")
}

// 0x59e508 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::InsertService::Callback>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
pub fn stub_0x59e508(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x59f0a8 — __ZNK3RBX13JointInstance17getPart0DangerousEv
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "RBX::JointInstance::getPart0Dangerous(void)const")]
#[doc(alias = "__ZNK3RBX13JointInstance17getPart0DangerousEv")]
pub fn stub_0x59f0a8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::JointInstance getter.
cell.get()
}

// 0x59f0cc — __ZN3RBX13JointInstance8setPart0EPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::JointInstance::setPart0(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX13JointInstance8setPart0EPNS_12PartInstanceE")]
pub fn stub_0x59f0cc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::JointInstance setter.
cell.set(value)
}

// 0x59f0f0 — __ZNK3RBX13JointInstance17getPart1DangerousEv
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "RBX::JointInstance::getPart1Dangerous(void)const")]
#[doc(alias = "__ZNK3RBX13JointInstance17getPart1DangerousEv")]
pub fn stub_0x59f0f0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::JointInstance getter.
cell.get()
}

// 0x59f114 — __ZN3RBX13JointInstance8setPart1EPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::JointInstance::setPart1(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX13JointInstance8setPart1EPNS_12PartInstanceE")]
pub fn stub_0x59f114(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::JointInstance setter.
cell.set(value)
}

// 0x59f138 — __ZNK3RBX26ManualSurfaceJointInstance11getSurface0Ev
// type: _DWORD __fastcall(RBX::ManualSurfaceJointInstance *__hidden this)
#[doc(alias = "RBX::ManualSurfaceJointInstance::getSurface0(void)const")]
#[doc(alias = "__ZNK3RBX26ManualSurfaceJointInstance11getSurface0Ev")]
pub fn stub_0x59f138(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ManualSurfaceJointInstance getter.
cell.get()
}

// 0x59f17c — __ZN3RBX26ManualSurfaceJointInstance11setSurface0Ei
// type: _DWORD __fastcall(RBX::ManualSurfaceJointInstance *__hidden this, int)
#[doc(alias = "RBX::ManualSurfaceJointInstance::setSurface0(int)")]
#[doc(alias = "__ZN3RBX26ManualSurfaceJointInstance11setSurface0Ei")]
pub fn stub_0x59f17c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::ManualSurfaceJointInstance setter.
cell.set(value)
}

// 0x59f1e0 — __ZNK3RBX26ManualSurfaceJointInstance11getSurface1Ev
// type: _DWORD __fastcall(RBX::ManualSurfaceJointInstance *__hidden this)
#[doc(alias = "RBX::ManualSurfaceJointInstance::getSurface1(void)const")]
#[doc(alias = "__ZNK3RBX26ManualSurfaceJointInstance11getSurface1Ev")]
pub fn stub_0x59f1e0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ManualSurfaceJointInstance getter.
cell.get()
}

// 0x59f224 — __ZN3RBX26ManualSurfaceJointInstance11setSurface1Ei
// type: _DWORD __fastcall(RBX::ManualSurfaceJointInstance *__hidden this, int)
#[doc(alias = "RBX::ManualSurfaceJointInstance::setSurface1(int)")]
#[doc(alias = "__ZN3RBX26ManualSurfaceJointInstance11setSurface1Ei")]
pub fn stub_0x59f224(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::ManualSurfaceJointInstance setter.
cell.set(value)
}

// 0x59f288 — __ZNK3RBX13JointInstance5getC0Ev
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "RBX::JointInstance::getC0(void)const")]
#[doc(alias = "__ZNK3RBX13JointInstance5getC0Ev")]
pub fn stub_0x59f288(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::JointInstance getter.
cell.get()
}

// 0x59f290 — __ZN3RBX13JointInstance5setC0ERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::JointInstance::setC0(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX13JointInstance5setC0ERKN3G3D15CoordinateFrameE")]
pub fn stub_0x59f290(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::JointInstance setter.
cell.set(value)
}

// 0x59f2bc — __ZNK3RBX13JointInstance5getC1Ev
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "RBX::JointInstance::getC1(void)const")]
#[doc(alias = "__ZNK3RBX13JointInstance5getC1Ev")]
pub fn stub_0x59f2bc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::JointInstance getter.
cell.get()
}

// 0x59f2c4 — __ZN3RBX13JointInstance5setC1ERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::JointInstance::setC1(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX13JointInstance5setC1ERKN3G3D15CoordinateFrameE")]
pub fn stub_0x59f2c4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::JointInstance setter.
cell.set(value)
}

// 0x59f2f0 — __ZNK3RBX4Glue5getF0Ev
// type: _DWORD __fastcall(RBX::Glue *__hidden this)
#[doc(alias = "RBX::Glue::getF0(void)const")]
#[doc(alias = "__ZNK3RBX4Glue5getF0Ev")]
pub fn stub_0x59f2f0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Glue getter.
cell.get()
}

// 0x59f2fc — __ZN3RBX4Glue5setF0ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Glue *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Glue::setF0(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Glue5setF0ERKN3G3D7Vector3E")]
pub fn stub_0x59f2fc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Glue setter.
cell.set(value)
}

// 0x59f334 — __ZNK3RBX4Glue5getF1Ev
// type: _DWORD __fastcall(RBX::Glue *__hidden this)
#[doc(alias = "RBX::Glue::getF1(void)const")]
#[doc(alias = "__ZNK3RBX4Glue5getF1Ev")]
pub fn stub_0x59f334(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Glue getter.
cell.get()
}

// 0x59f340 — __ZN3RBX4Glue5setF1ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Glue *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Glue::setF1(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Glue5setF1ERKN3G3D7Vector3E")]
pub fn stub_0x59f340(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Glue setter.
cell.set(value)
}

// 0x59f378 — __ZNK3RBX4Glue5getF2Ev
// type: _DWORD __fastcall(RBX::Glue *__hidden this)
#[doc(alias = "RBX::Glue::getF2(void)const")]
#[doc(alias = "__ZNK3RBX4Glue5getF2Ev")]
pub fn stub_0x59f378(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Glue getter.
cell.get()
}

// 0x59f384 — __ZN3RBX4Glue5setF2ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Glue *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Glue::setF2(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Glue5setF2ERKN3G3D7Vector3E")]
pub fn stub_0x59f384(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Glue setter.
cell.set(value)
}

// 0x59f3bc — __ZNK3RBX4Glue5getF3Ev
// type: _DWORD __fastcall(RBX::Glue *__hidden this)
#[doc(alias = "RBX::Glue::getF3(void)const")]
#[doc(alias = "__ZNK3RBX4Glue5getF3Ev")]
pub fn stub_0x59f3bc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Glue getter.
cell.get()
}

// 0x59f3c8 — __ZN3RBX4Glue5setF3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Glue *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Glue::setF3(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Glue5setF3ERKN3G3D7Vector3E")]
pub fn stub_0x59f3c8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Glue setter.
cell.set(value)
}

// 0x59f400 — __ZNK3RBX13DynamicRotate12getBaseAngleEv
// type: _DWORD __fastcall(RBX::DynamicRotate *__hidden this)
#[doc(alias = "RBX::DynamicRotate::getBaseAngle(void)const")]
#[doc(alias = "__ZNK3RBX13DynamicRotate12getBaseAngleEv")]
pub fn stub_0x59f400(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DynamicRotate getter.
cell.get()
}

// 0x59f40c — __ZN3RBX13DynamicRotate12setBaseAngleEf
// type: _DWORD __fastcall(RBX::DynamicRotate *__hidden this, float)
#[doc(alias = "RBX::DynamicRotate::setBaseAngle(float)")]
#[doc(alias = "__ZN3RBX13DynamicRotate12setBaseAngleEf")]
pub fn stub_0x59f40c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DynamicRotate setter.
cell.set(value)
}

// 0x59f418 — __ZN3RBX13JointInstanceC2EPNS_5JointE
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::JointInstance::JointInstance(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX13JointInstanceC2EPNS_5JointE")]
pub fn stub_0x59f418(handle: &crate::slot::InstanceHandle) {
// RBX::JointInstance::JointInstance(RBX::Joint*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x59f6bc — __ZN3RBX13JointInstanceD0Ev
// type: void __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "RBX::JointInstance::~JointInstance()")]
#[doc(alias = "__ZN3RBX13JointInstanceD0Ev")]
pub fn stub_0x59f6bc(handle: crate::slot::InstanceHandle) {
// RBX::JointInstance dtor.
drop(handle);
}

// 0x59f75c — __ZN3RBX13JointInstanceD1Ev
// type: void __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "RBX::JointInstance::~JointInstance() [0x59f75c]")]
#[doc(alias = "__ZN3RBX13JointInstanceD1Ev")]
pub fn stub_0x59f75c(handle: crate::slot::InstanceHandle) {
// RBX::JointInstance dtor.
drop(handle);
}

// 0x59f760 — __ZThn32_N3RBX13JointInstanceD0Ev
// type: void __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointInstance::~JointInstance()")]
#[doc(alias = "__ZThn32_N3RBX13JointInstanceD0Ev")]
pub fn stub_0x59f760(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x59f768 — __ZThn36_N3RBX13JointInstanceD0Ev
// type: void __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointInstance::~JointInstance() [0x59f768]")]
#[doc(alias = "__ZThn36_N3RBX13JointInstanceD0Ev")]
pub fn stub_0x59f768(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x59f770 — __ZN3RBX13JointInstanceD2Ev
// type: void __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "RBX::JointInstance::~JointInstance() [0x59f770]")]
#[doc(alias = "__ZN3RBX13JointInstanceD2Ev")]
pub fn stub_0x59f770(handle: crate::slot::InstanceHandle) {
// RBX::JointInstance dtor.
drop(handle);
}

// 0x59fa24 — __ZThn32_N3RBX13JointInstanceD1Ev
// type: void __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointInstance::~JointInstance() [0x59fa24]")]
#[doc(alias = "__ZThn32_N3RBX13JointInstanceD1Ev")]
pub fn stub_0x59fa24(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x59fa2c — __ZThn36_N3RBX13JointInstanceD1Ev
// type: void __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointInstance::~JointInstance() [0x59fa2c]")]
#[doc(alias = "__ZThn36_N3RBX13JointInstanceD1Ev")]
pub fn stub_0x59fa2c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x59fa34 — __ZN3RBX13JointInstance8writeXmlERKN5boost8functionIFbPNS_8InstanceEEEENS_11CreatorRoleE
#[doc(alias = "RBX::JointInstance::writeXml(boost::function<bool ()(RBX::Instance *)> const&,RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX13JointInstance8writeXmlERKN5boost8functionIFbPNS_8InstanceEEEENS_11CreatorRoleE")]
pub fn stub_0x59fa34(handle: &crate::slot::InstanceHandle) {
// RBX::JointInstance::writeXml(boost::function<bool (RBX::Instance*)> const&, RBX::CreatorRo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x59fa90 — __ZN3RBX13JointInstance8getPart0Ev
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "RBX::JointInstance::getPart0(void)")]
#[doc(alias = "__ZN3RBX13JointInstance8getPart0Ev")]
pub fn stub_0x59fa90(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::JointInstance getter.
cell.get()
}

// 0x59fab4 — __ZN3RBX13JointInstance8getPart1Ev
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "RBX::JointInstance::getPart1(void)")]
#[doc(alias = "__ZN3RBX13JointInstance8getPart1Ev")]
pub fn stub_0x59fab4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::JointInstance getter.
cell.get()
}

// 0x59fad8 — __ZNK3RBX13JointInstance19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "RBX::JointInstance::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZNK3RBX13JointInstance19shouldRender3dAdornEv")]
pub fn stub_0x59fad8(handle: &crate::slot::InstanceHandle) {
// RBX::JointInstance::shouldRender3dAdorn() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x59fae8 — __ZThn92_NK3RBX13JointInstance19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::JointInstance::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZThn92_NK3RBX13JointInstance19shouldRender3dAdornEv")]
pub fn stub_0x59fae8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x59faf8 — __ZN3RBX13JointInstance13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::JointInstance::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX13JointInstance13render3dAdornEPNS_5AdornE")]
pub fn stub_0x59faf8(handle: &crate::slot::InstanceHandle) {
// RBX::JointInstance::render3dAdorn(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x59fc7c — __ZThn92_N3RBX13JointInstance13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::JointInstance::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn92_N3RBX13JointInstance13render3dAdornEPNS_5AdornE")]
pub fn stub_0x59fc7c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x59fc84 — __ZN3RBX13JointInstance7setPartEiPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this, int, RBX::PartInstance *)
#[doc(alias = "RBX::JointInstance::setPart(int,RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX13JointInstance7setPartEiPNS_12PartInstanceE")]
pub fn stub_0x59fc84(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::JointInstance setter.
cell.set(value)
}

// 0x59fde4 — __ZNK3RBX13JointInstance12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::JointInstance::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX13JointInstance12askSetParentEPKNS_8InstanceE")]
pub fn stub_0x59fde4(handle: &crate::slot::InstanceHandle) {
// RBX::JointInstance::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x59fe28 — __ZN3RBX13JointInstance12computeWorldEv
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this)
#[doc(alias = "RBX::JointInstance::computeWorld(void)")]
#[doc(alias = "__ZN3RBX13JointInstance12computeWorldEv")]
pub fn stub_0x59fe28(handle: &crate::slot::InstanceHandle) {
// RBX::JointInstance::computeWorld() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x59fe6c — __ZN3RBX13JointInstance17onAncestorChangedERKNS_15AncestorChangedE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::JointInstance::onAncestorChanged(RBX::AncestorChanged const&)")]
#[doc(alias = "__ZN3RBX13JointInstance17onAncestorChangedERKNS_15AncestorChangedE")]
pub fn stub_0x59fe6c(handle: &crate::slot::InstanceHandle) {
// RBX::JointInstance::onAncestorChanged(RBX::AncestorChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5a0020 — __ZN3RBX13JointInstance7setNameERKSs
// type: _DWORD __fastcall(RBX::JointInstance *__hidden this, const std::string *)
#[doc(alias = "RBX::JointInstance::setName(std::string const&)")]
#[doc(alias = "__ZN3RBX13JointInstance7setNameERKSs")]
pub fn stub_0x5a0020(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::JointInstance setter.
cell.set(value)
}

// 0x5a0068 — __ZN3RBX4SnapC1EPNS_5JointE
// type: _DWORD __fastcall(RBX::Snap *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::Snap::Snap(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX4SnapC1EPNS_5JointE")]
pub fn stub_0x5a0068(handle: &crate::slot::InstanceHandle) {
// RBX::Snap::Snap(RBX::Joint*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5a006c — __ZN3RBX4SnapC2EPNS_5JointE
// type: _DWORD __fastcall(RBX::Snap *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::Snap::Snap(RBX::Joint *) [0x5a006c]")]
#[doc(alias = "__ZN3RBX4SnapC2EPNS_5JointE")]
pub fn stub_0x5a006c(handle: &crate::slot::InstanceHandle) {
// RBX::Snap::Snap(RBX::Joint*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5a033c — __ZN3RBX4SnapC1Ev
// type: _DWORD __fastcall(RBX::Snap *__hidden this)
#[doc(alias = "RBX::Snap::Snap(void)")]
#[doc(alias = "__ZN3RBX4SnapC1Ev")]
pub fn stub_0x5a033c() -> crate::slot::InstanceHandle {
// RBX::Snap ctor.
crate::slot::InstanceHandle::new("RBX::Snap")
}

// 0x5a0340 — __ZN3RBX4SnapC2Ev
// type: _DWORD __fastcall(RBX::Snap *__hidden this)
#[doc(alias = "RBX::Snap::Snap(void) [0x5a0340]")]
#[doc(alias = "__ZN3RBX4SnapC2Ev")]
pub fn stub_0x5a0340() -> crate::slot::InstanceHandle {
// RBX::Snap ctor.
crate::slot::InstanceHandle::new("RBX::Snap")
}

// 0x5a0584 — __ZN3RBX4WeldC1EPNS_5JointE
// type: _DWORD __fastcall(RBX::Weld *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::Weld::Weld(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX4WeldC1EPNS_5JointE")]
pub fn stub_0x5a0584(handle: &crate::slot::InstanceHandle) {
// RBX::Weld::Weld(RBX::Joint*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5a0588 — __ZN3RBX4WeldC2EPNS_5JointE
// type: _DWORD __fastcall(RBX::Weld *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::Weld::Weld(RBX::Joint *) [0x5a0588]")]
#[doc(alias = "__ZN3RBX4WeldC2EPNS_5JointE")]
pub fn stub_0x5a0588(handle: &crate::slot::InstanceHandle) {
// RBX::Weld::Weld(RBX::Joint*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5a0854 — __ZN3RBX4WeldC1Ev
// type: _DWORD __fastcall(RBX::Weld *__hidden this)
#[doc(alias = "RBX::Weld::Weld(void)")]
#[doc(alias = "__ZN3RBX4WeldC1Ev")]
pub fn stub_0x5a0854() -> crate::slot::InstanceHandle {
// RBX::Weld ctor.
crate::slot::InstanceHandle::new("RBX::Weld")
}

// 0x5a0858 — __ZN3RBX4WeldC2Ev
// type: _DWORD __fastcall(RBX::Weld *__hidden this)
#[doc(alias = "RBX::Weld::Weld(void) [0x5a0858]")]
#[doc(alias = "__ZN3RBX4WeldC2Ev")]
pub fn stub_0x5a0858() -> crate::slot::InstanceHandle {
// RBX::Weld ctor.
crate::slot::InstanceHandle::new("RBX::Weld")
}

// 0x5a0a98 — __ZN3RBX4Weld13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Weld *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::Weld::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX4Weld13render3dAdornEPNS_5AdornE")]
pub fn stub_0x5a0a98(handle: &crate::slot::InstanceHandle) {
// RBX::Weld::render3dAdorn(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5a0a9c — __ZThn92_N3RBX4Weld13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Weld *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::Weld::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn92_N3RBX4Weld13render3dAdornEPNS_5AdornE")]
pub fn stub_0x5a0a9c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x5a0aa4 — __ZN3RBX26ManualSurfaceJointInstanceC2EPNS_5JointE
// type: _DWORD __fastcall(RBX::ManualSurfaceJointInstance *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::ManualSurfaceJointInstance::ManualSurfaceJointInstance(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX26ManualSurfaceJointInstanceC2EPNS_5JointE")]
pub fn stub_0x5a0aa4(handle: &crate::slot::InstanceHandle) {
// RBX::ManualSurfaceJointInstance::ManualSurfaceJointInstance(RBX::Joint*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5a0be4 — __ZN3RBX26ManualSurfaceJointInstanceC2Ev
// type: _DWORD __fastcall(RBX::ManualSurfaceJointInstance *__hidden this)
#[doc(alias = "RBX::ManualSurfaceJointInstance::ManualSurfaceJointInstance(void)")]
#[doc(alias = "__ZN3RBX26ManualSurfaceJointInstanceC2Ev")]
pub fn stub_0x5a0be4() -> crate::slot::InstanceHandle {
// RBX::ManualSurfaceJointInstance ctor.
crate::slot::InstanceHandle::new("RBX::ManualSurfaceJointInstance")
}

// 0x5a0d24 — __ZN3RBX26ManualSurfaceJointInstance13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::ManualSurfaceJointInstance *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::ManualSurfaceJointInstance::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX26ManualSurfaceJointInstance13render3dAdornEPNS_5AdornE")]
pub fn stub_0x5a0d24(handle: &crate::slot::InstanceHandle) {
// RBX::ManualSurfaceJointInstance::render3dAdorn(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5a0d28 — __ZThn92_N3RBX26ManualSurfaceJointInstance13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::ManualSurfaceJointInstance *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::ManualSurfaceJointInstance::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn92_N3RBX26ManualSurfaceJointInstance13render3dAdornEPNS_5AdornE")]
pub fn stub_0x5a0d28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x5a0d30 — __ZN3RBX10ManualWeldC2Ev
// type: _DWORD __fastcall(RBX::ManualWeld *__hidden this)
#[doc(alias = "RBX::ManualWeld::ManualWeld(void)")]
#[doc(alias = "__ZN3RBX10ManualWeldC2Ev")]
pub fn stub_0x5a0d30() -> crate::slot::InstanceHandle {
// RBX::ManualWeld ctor.
crate::slot::InstanceHandle::new("RBX::ManualWeld")
}

// 0x5a31ac — __ZN3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEED1Ev")]
pub fn stub_0x5a31ac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5a31d8 — __ZN3RBX10Reflection14PropDescriptorINS_26ManualSurfaceJointInstanceEiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ManualSurfaceJointInstance,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_26ManualSurfaceJointInstanceEiED1Ev")]
pub fn stub_0x5a31d8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5a31fc — __ZN3RBX10Reflection14PropDescriptorINS_13JointInstanceEN3G3D15CoordinateFrameEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::JointInstance,G3D::CoordinateFrame>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13JointInstanceEN3G3D15CoordinateFrameEED1Ev")]
pub fn stub_0x5a31fc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5a3220 — __ZN3RBX10Reflection14PropDescriptorINS_4GlueEN3G3D7Vector3EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Glue,G3D::Vector3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4GlueEN3G3D7Vector3EED1Ev")]
pub fn stub_0x5a3220(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5a3244 — __ZN3RBX10Reflection14PropDescriptorINS_13DynamicRotateEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DynamicRotate,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13DynamicRotateEfED1Ev")]
pub fn stub_0x5a3244(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5a3268 — __ZN3RBX10Reflection14PropDescriptorINS_5MotorEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Motor,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5MotorEfED1Ev")]
pub fn stub_0x5a3268(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5a328c — __ZN3RBX10Reflection13BoundFuncDescINS_5MotorEFvfELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Motor,void ()(float),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5MotorEFvfELi1EED1Ev")]
pub fn stub_0x5a328c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x5a32cc — __ZNK5boost9function1IbPN3RBX8InstanceEEclES3_
// type: int(void)
#[doc(alias = "boost::function1<bool,RBX::Instance *>::operator()(RBX::Instance *)const")]
#[doc(alias = "__ZNK5boost9function1IbPN3RBX8InstanceEEclES3_")]
pub fn stub_0x5a32cc(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

// 0x5a381c — __ZN3RBX26ManualSurfaceJointInstanceD1Ev
// type: void __fastcall(RBX::ManualSurfaceJointInstance *__hidden this)
#[doc(alias = "RBX::ManualSurfaceJointInstance::~ManualSurfaceJointInstance()")]
#[doc(alias = "__ZN3RBX26ManualSurfaceJointInstanceD1Ev")]
pub fn stub_0x5a381c(handle: crate::slot::InstanceHandle) {
// RBX::ManualSurfaceJointInstance dtor.
drop(handle);
}

// 0x5a3820 — __ZN3RBX26ManualSurfaceJointInstanceD0Ev
// type: void __fastcall(RBX::ManualSurfaceJointInstance *__hidden this)
#[doc(alias = "RBX::ManualSurfaceJointInstance::~ManualSurfaceJointInstance() [0x5a3820]")]
#[doc(alias = "__ZN3RBX26ManualSurfaceJointInstanceD0Ev")]
pub fn stub_0x5a3820(handle: crate::slot::InstanceHandle) {
// RBX::ManualSurfaceJointInstance dtor.
drop(handle);
}

// 0x5a38d0 — __ZThn32_N3RBX26ManualSurfaceJointInstanceD1Ev
// type: void __fastcall(RBX::ManualSurfaceJointInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualSurfaceJointInstance::~ManualSurfaceJointInstance()")]
#[doc(alias = "__ZThn32_N3RBX26ManualSurfaceJointInstanceD1Ev")]
pub fn stub_0x5a38d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a38d8 — __ZThn32_N3RBX26ManualSurfaceJointInstanceD0Ev
// type: void __fastcall(RBX::ManualSurfaceJointInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualSurfaceJointInstance::~ManualSurfaceJointInstance() [0x5a38d8]")]
#[doc(alias = "__ZThn32_N3RBX26ManualSurfaceJointInstanceD0Ev")]
pub fn stub_0x5a38d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a398c — __ZThn36_N3RBX26ManualSurfaceJointInstanceD1Ev
// type: void __fastcall(RBX::ManualSurfaceJointInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualSurfaceJointInstance::~ManualSurfaceJointInstance() [0x5a398c]")]
#[doc(alias = "__ZThn36_N3RBX26ManualSurfaceJointInstanceD1Ev")]
pub fn stub_0x5a398c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a3994 — __ZThn36_N3RBX26ManualSurfaceJointInstanceD0Ev
// type: void __fastcall(RBX::ManualSurfaceJointInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualSurfaceJointInstance::~ManualSurfaceJointInstance() [0x5a3994]")]
#[doc(alias = "__ZThn36_N3RBX26ManualSurfaceJointInstanceD0Ev")]
pub fn stub_0x5a3994(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a3aec — __ZNK3RBX10ManualWeld19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::ManualWeld *__hidden this)
#[doc(alias = "RBX::ManualWeld::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZNK3RBX10ManualWeld19shouldRender3dAdornEv")]
pub fn stub_0x5a3aec(handle: &crate::slot::InstanceHandle) {
// RBX::ManualWeld::shouldRender3dAdorn() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5a3bb4 — __ZThn36_N3RBX10ManualWeldD0Ev
// type: void __fastcall(RBX::ManualWeld *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualWeld::~ManualWeld() [0x5a3bb4]")]
#[doc(alias = "__ZThn36_N3RBX10ManualWeldD0Ev")]
pub fn stub_0x5a3bb4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a3c58 — __ZThn92_NK3RBX10ManualWeld19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::ManualWeld *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualWeld::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZThn92_NK3RBX10ManualWeld19shouldRender3dAdornEv")]
pub fn stub_0x5a3c58(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x5a3c5c — __ZN3RBX10ManualGlueD1Ev
// type: void __fastcall(RBX::ManualGlue *__hidden this)
#[doc(alias = "RBX::ManualGlue::~ManualGlue()")]
#[doc(alias = "__ZN3RBX10ManualGlueD1Ev")]
pub fn stub_0x5a3c5c(handle: crate::slot::InstanceHandle) {
// RBX::ManualGlue dtor.
drop(handle);
}

// 0x5a3c60 — __ZN3RBX10ManualGlueD0Ev
// type: void __fastcall(RBX::ManualGlue *__hidden this)
#[doc(alias = "RBX::ManualGlue::~ManualGlue() [0x5a3c60]")]
#[doc(alias = "__ZN3RBX10ManualGlueD0Ev")]
pub fn stub_0x5a3c60(handle: crate::slot::InstanceHandle) {
// RBX::ManualGlue dtor.
drop(handle);
}

// 0x5a3d10 — __ZNK3RBX10ManualGlue19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::ManualGlue *__hidden this)
#[doc(alias = "RBX::ManualGlue::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZNK3RBX10ManualGlue19shouldRender3dAdornEv")]
pub fn stub_0x5a3d10(handle: &crate::slot::InstanceHandle) {
// RBX::ManualGlue::shouldRender3dAdorn() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5a3d14 — __ZThn32_N3RBX10ManualGlueD1Ev
// type: void __fastcall(RBX::ManualGlue *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualGlue::~ManualGlue()")]
#[doc(alias = "__ZThn32_N3RBX10ManualGlueD1Ev")]
pub fn stub_0x5a3d14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a3d1c — __ZThn32_N3RBX10ManualGlueD0Ev
// type: void __fastcall(RBX::ManualGlue *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualGlue::~ManualGlue() [0x5a3d1c]")]
#[doc(alias = "__ZThn32_N3RBX10ManualGlueD0Ev")]
pub fn stub_0x5a3d1c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a3dd0 — __ZThn36_N3RBX10ManualGlueD1Ev
// type: void __fastcall(RBX::ManualGlue *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualGlue::~ManualGlue() [0x5a3dd0]")]
#[doc(alias = "__ZThn36_N3RBX10ManualGlueD1Ev")]
pub fn stub_0x5a3dd0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a3dd8 — __ZThn36_N3RBX10ManualGlueD0Ev
// type: void __fastcall(RBX::ManualGlue *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualGlue::~ManualGlue() [0x5a3dd8]")]
#[doc(alias = "__ZThn36_N3RBX10ManualGlueD0Ev")]
pub fn stub_0x5a3dd8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a3e7c — __ZThn92_NK3RBX10ManualGlue19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::ManualGlue *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ManualGlue::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZThn92_NK3RBX10ManualGlue19shouldRender3dAdornEv")]
pub fn stub_0x5a3e7c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x5a3e80 — __ZN3RBX7Motor6DD1Ev
// type: void __fastcall(RBX::Motor6D *__hidden this)
#[doc(alias = "RBX::Motor6D::~Motor6D()")]
#[doc(alias = "__ZN3RBX7Motor6DD1Ev")]
pub fn stub_0x5a3e80(handle: crate::slot::InstanceHandle) {
// RBX::Motor6D dtor.
drop(handle);
}

// 0x5a3e84 — __ZN3RBX7Motor6DD0Ev
// type: void __fastcall(RBX::Motor6D *__hidden this)
#[doc(alias = "RBX::Motor6D::~Motor6D() [0x5a3e84]")]
#[doc(alias = "__ZN3RBX7Motor6DD0Ev")]
pub fn stub_0x5a3e84(handle: crate::slot::InstanceHandle) {
// RBX::Motor6D dtor.
drop(handle);
}

// 0x5a3f34 — __ZThn32_N3RBX7Motor6DD1Ev
// type: void __fastcall(RBX::Motor6D *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Motor6D::~Motor6D()")]
#[doc(alias = "__ZThn32_N3RBX7Motor6DD1Ev")]
pub fn stub_0x5a3f34(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a3f3c — __ZThn32_N3RBX7Motor6DD0Ev
// type: void __fastcall(RBX::Motor6D *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Motor6D::~Motor6D() [0x5a3f3c]")]
#[doc(alias = "__ZThn32_N3RBX7Motor6DD0Ev")]
pub fn stub_0x5a3f3c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a3ff0 — __ZThn36_N3RBX7Motor6DD1Ev
// type: void __fastcall(RBX::Motor6D *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Motor6D::~Motor6D() [0x5a3ff0]")]
#[doc(alias = "__ZThn36_N3RBX7Motor6DD1Ev")]
pub fn stub_0x5a3ff0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a3ff8 — __ZThn36_N3RBX7Motor6DD0Ev
// type: void __fastcall(RBX::Motor6D *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Motor6D::~Motor6D() [0x5a3ff8]")]
#[doc(alias = "__ZThn36_N3RBX7Motor6DD0Ev")]
pub fn stub_0x5a3ff8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a40b4 — __ZN3RBX4SnapD1Ev
// type: void __fastcall(RBX::Snap *__hidden this)
#[doc(alias = "RBX::Snap::~Snap()")]
#[doc(alias = "__ZN3RBX4SnapD1Ev")]
pub fn stub_0x5a40b4(handle: crate::slot::InstanceHandle) {
// RBX::Snap dtor.
drop(handle);
}

// 0x5a40b8 — __ZN3RBX4SnapD0Ev
// type: void __fastcall(RBX::Snap *__hidden this)
#[doc(alias = "RBX::Snap::~Snap() [0x5a40b8]")]
#[doc(alias = "__ZN3RBX4SnapD0Ev")]
pub fn stub_0x5a40b8(handle: crate::slot::InstanceHandle) {
// RBX::Snap dtor.
drop(handle);
}

// 0x5a4168 — __ZThn32_N3RBX4SnapD1Ev
// type: void __fastcall(RBX::Snap *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Snap::~Snap()")]
#[doc(alias = "__ZThn32_N3RBX4SnapD1Ev")]
pub fn stub_0x5a4168(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a4170 — __ZThn32_N3RBX4SnapD0Ev
// type: void __fastcall(RBX::Snap *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Snap::~Snap() [0x5a4170]")]
#[doc(alias = "__ZThn32_N3RBX4SnapD0Ev")]
pub fn stub_0x5a4170(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a4224 — __ZThn36_N3RBX4SnapD1Ev
// type: void __fastcall(RBX::Snap *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Snap::~Snap() [0x5a4224]")]
#[doc(alias = "__ZThn36_N3RBX4SnapD1Ev")]
pub fn stub_0x5a4224(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a422c — __ZThn36_N3RBX4SnapD0Ev
// type: void __fastcall(RBX::Snap *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Snap::~Snap() [0x5a422c]")]
#[doc(alias = "__ZThn36_N3RBX4SnapD0Ev")]
pub fn stub_0x5a422c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5a42d0 — __ZN3RBX4GlueD1Ev
// type: void __fastcall(RBX::Glue *__hidden this)
#[doc(alias = "RBX::Glue::~Glue()")]
#[doc(alias = "__ZN3RBX4GlueD1Ev")]
pub fn stub_0x5a42d0(handle: crate::slot::InstanceHandle) {
// RBX::Glue dtor.
drop(handle);
}

// 0x5a42d4 — __ZN3RBX4GlueD0Ev
// type: void __fastcall(RBX::Glue *__hidden this)
#[doc(alias = "RBX::Glue::~Glue() [0x5a42d4]")]
#[doc(alias = "__ZN3RBX4GlueD0Ev")]
pub fn stub_0x5a42d4(handle: crate::slot::InstanceHandle) {
// RBX::Glue dtor.
drop(handle);
}
