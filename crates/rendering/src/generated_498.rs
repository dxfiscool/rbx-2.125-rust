//! rendering — generated_498 — 100 stubs global dedup (rendering filtered, EA-sorted asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) NOT in /tmp/global_eas.txt — next 100 uncovered EA-sorted asc 0xd4be40..0xd4e4f4 (3966 candidates remaining, 89813 global EAs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr). Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::ogre::{CompareFunction, CullingMode, ManualCullingMode, Pass, PolygonMode, SceneBlendOperation, ShadeOptions};

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xd4be40 — __ZNK4Ogre4Pass24hasSeparateSceneBlendingEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::hasSeparateSceneBlending(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass24hasSeparateSceneBlendingEv")]
// was: Ogre::Pass::hasSeparateSceneBlending(void)const
pub fn stub_0xd4be40(pass: &Pass) -> bool {
    pass.has_separate_scene_blending() // IDA 0xd4be40: LDRB.W R0,[R0,#0x70]
}

// 0xd4be48 — __ZN4Ogre4Pass25setSceneBlendingOperationENS_19SceneBlendOperationE
#[doc(alias = "Ogre::Pass::setSceneBlendingOperation(Ogre::SceneBlendOperation)")]
#[doc(alias = "__ZN4Ogre4Pass25setSceneBlendingOperationENS_19SceneBlendOperationE")]
// was: Ogre::Pass::setSceneBlendingOperation(Ogre::SceneBlendOperation)
pub fn stub_0xd4be48(pass: &mut Pass, op: SceneBlendOperation) {
    pass.set_scene_blending_operation(op) // IDA 0xd4be48: STR R1,[R0,#0x74]; STRB #0,[R0,#0x7C]
}

// 0xd4be54 — __ZN4Ogre4Pass33setSeparateSceneBlendingOperationENS_19SceneBlendOperationES1_
#[doc(alias = "Ogre::Pass::setSeparateSceneBlendingOperation(Ogre::SceneBlendOperation,Ogre::SceneBlendOperation)")]
#[doc(alias = "__ZN4Ogre4Pass33setSeparateSceneBlendingOperationENS_19SceneBlendOperationES1_")]
// was: Ogre::Pass::setSeparateSceneBlendingOperation(Ogre::SceneBlendOperation,Ogre::SceneBlendOperation)
pub fn stub_0xd4be54(pass: &mut Pass, colour_op: SceneBlendOperation, alpha_op: SceneBlendOperation) {
    pass.set_separate_scene_blending_operation(colour_op, alpha_op) // IDA 0xd4be54: STRD.W R1,R2,[R0,#0x74]; STRB #1,[R0,#0x7C]
}

// 0xd4be64 — __ZNK4Ogre4Pass25getSceneBlendingOperationEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getSceneBlendingOperation(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass25getSceneBlendingOperationEv")]
// was: Ogre::Pass::getSceneBlendingOperation(void)const
pub fn stub_0xd4be64(pass: &Pass) -> SceneBlendOperation {
    pass.scene_blending_operation() // IDA 0xd4be64: LDR R0,[R0,#0x74]
}

// 0xd4be68 — __ZNK4Ogre4Pass30getSceneBlendingOperationAlphaEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getSceneBlendingOperationAlpha(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass30getSceneBlendingOperationAlphaEv")]
// was: Ogre::Pass::getSceneBlendingOperationAlpha(void)const
pub fn stub_0xd4be68(pass: &Pass) -> SceneBlendOperation {
    pass.scene_blending_operation_alpha() // IDA 0xd4be68: LDR R0,[R0,#0x78]
}

// 0xd4be6c — __ZNK4Ogre4Pass34hasSeparateSceneBlendingOperationsEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::hasSeparateSceneBlendingOperations(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass34hasSeparateSceneBlendingOperationsEv")]
// was: Ogre::Pass::hasSeparateSceneBlendingOperations(void)const
pub fn stub_0xd4be6c(pass: &Pass) -> bool {
    pass.has_separate_scene_blending_operations() // IDA 0xd4be6c: LDRB.W R0,[R0,#0x7C]
}

// 0xd4be74 — __ZNK4Ogre4Pass13isTransparentEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::isTransparent(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass13isTransparentEv")]
// was: Ogre::Pass::isTransparent(void)const
pub fn stub_0xd4be74(pass: &Pass) -> bool {
    pass.is_transparent() // IDA 0xd4be74: dest(+0x64)!=1 -> true; else (0x55 >> (src(+0x60)-2)) & 1
}

// 0xd4be9c — __ZN4Ogre4Pass20setDepthCheckEnabledEb
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, bool)
#[doc(alias = "Ogre::Pass::setDepthCheckEnabled(bool)")]
#[doc(alias = "__ZN4Ogre4Pass20setDepthCheckEnabledEb")]
// was: Ogre::Pass::setDepthCheckEnabled(bool)
pub fn stub_0xd4be9c(pass: &mut Pass, enabled: bool) {
    pass.set_depth_check_enabled(enabled) // IDA 0xd4be9c: STRB.W R1,[R0,#0x7D]
}

// 0xd4bea4 — __ZNK4Ogre4Pass20getDepthCheckEnabledEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getDepthCheckEnabled(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass20getDepthCheckEnabledEv")]
// was: Ogre::Pass::getDepthCheckEnabled(void)const
pub fn stub_0xd4bea4(pass: &Pass) -> bool {
    pass.depth_check_enabled() // IDA 0xd4bea4: LDRB.W R0,[R0,#0x7D]
}

// 0xd4beac — __ZN4Ogre4Pass20setDepthWriteEnabledEb
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, bool)
#[doc(alias = "Ogre::Pass::setDepthWriteEnabled(bool)")]
#[doc(alias = "__ZN4Ogre4Pass20setDepthWriteEnabledEb")]
// was: Ogre::Pass::setDepthWriteEnabled(bool)
pub fn stub_0xd4beac(pass: &mut Pass, enabled: bool) {
    pass.set_depth_write_enabled(enabled) // IDA 0xd4beac: STRB.W R1,[R0,#0x7E]
}

// 0xd4beb4 — __ZNK4Ogre4Pass20getDepthWriteEnabledEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getDepthWriteEnabled(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass20getDepthWriteEnabledEv")]
// was: Ogre::Pass::getDepthWriteEnabled(void)const
pub fn stub_0xd4beb4(pass: &Pass) -> bool {
    pass.depth_write_enabled() // IDA 0xd4beb4: LDRB.W R0,[R0,#0x7E]
}

// 0xd4bebc — __ZN4Ogre4Pass16setDepthFunctionENS_15CompareFunctionE
#[doc(alias = "Ogre::Pass::setDepthFunction(Ogre::CompareFunction)")]
#[doc(alias = "__ZN4Ogre4Pass16setDepthFunctionENS_15CompareFunctionE")]
// was: Ogre::Pass::setDepthFunction(Ogre::CompareFunction)
pub fn stub_0xd4bebc(pass: &mut Pass, func: CompareFunction) {
    pass.set_depth_function(func) // IDA 0xd4bebc: STR.W R1,[R0,#0x80]
}

// 0xd4bec4 — __ZNK4Ogre4Pass16getDepthFunctionEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getDepthFunction(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass16getDepthFunctionEv")]
// was: Ogre::Pass::getDepthFunction(void)const
pub fn stub_0xd4bec4(pass: &Pass) -> CompareFunction {
    pass.depth_function() // IDA 0xd4bec4: LDR.W R0,[R0,#0x80]
}

// 0xd4becc — __ZN4Ogre4Pass22setAlphaRejectSettingsENS_15CompareFunctionEhb
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "Ogre::Pass::setAlphaRejectSettings(Ogre::CompareFunction,unsigned char,bool)")]
#[doc(alias = "__ZN4Ogre4Pass22setAlphaRejectSettingsENS_15CompareFunctionEhb")]
// was: Ogre::Pass::setAlphaRejectSettings(Ogre::CompareFunction,unsigned char,bool)
pub fn stub_0xd4becc(pass: &mut Pass, func: CompareFunction, value: u8, alpha_to_coverage: bool) {
    pass.set_alpha_reject_settings(func, value, alpha_to_coverage) // IDA 0xd4becc: STR.W R1,[R0,#0x94]; STRB R2,[R0,#0x98]; STRB R3,[R0,#0x99]
}
// 0xd4bedc — __ZN4Ogre4Pass22setAlphaRejectFunctionENS_15CompareFunctionE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::Pass::setAlphaRejectFunction(Ogre::CompareFunction)")]
#[doc(alias = "__ZN4Ogre4Pass22setAlphaRejectFunctionENS_15CompareFunctionE")]
// was: Ogre::Pass::setAlphaRejectFunction(Ogre::CompareFunction)
pub fn stub_0xd4bedc(pass: &mut Pass, func: CompareFunction) {
    pass.set_alpha_reject_function(func) // IDA 0xd4bedc: STR.W R1,[R0,#0x94]
}
// 0xd4bee4 — __ZN4Ogre4Pass25setAlphaToCoverageEnabledEb
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, bool)
#[doc(alias = "Ogre::Pass::setAlphaToCoverageEnabled(bool)")]
#[doc(alias = "__ZN4Ogre4Pass25setAlphaToCoverageEnabledEb")]
// was: Ogre::Pass::setAlphaToCoverageEnabled(bool)
pub fn stub_0xd4bee4(pass: &mut Pass, enabled: bool) {
    pass.set_alpha_to_coverage_enabled(enabled) // IDA 0xd4bee4: STRB.W R1,[R0,#0x99]
}
// 0xd4beec — __ZN4Ogre4Pass28setTransparentSortingEnabledEb
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, bool)
#[doc(alias = "Ogre::Pass::setTransparentSortingEnabled(bool)")]
#[doc(alias = "__ZN4Ogre4Pass28setTransparentSortingEnabledEb")]
// was: Ogre::Pass::setTransparentSortingEnabled(bool)
pub fn stub_0xd4beec(pass: &mut Pass, enabled: bool) {
    pass.set_transparent_sorting_enabled(enabled) // IDA 0xd4beec: STRB.W R1,[R0,#0x9A]
}

// 0xd4bef4 — __ZNK4Ogre4Pass28getTransparentSortingEnabledEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getTransparentSortingEnabled(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass28getTransparentSortingEnabledEv")]
// was: Ogre::Pass::getTransparentSortingEnabled(void)const
pub fn stub_0xd4bef4(pass: &Pass) -> bool {
    pass.transparent_sorting_enabled() // IDA 0xd4bef4: LDRB.W R0,[R0,#0x9A]
}

// 0xd4befc — __ZN4Ogre4Pass27setTransparentSortingForcedEb
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, bool)
#[doc(alias = "Ogre::Pass::setTransparentSortingForced(bool)")]
#[doc(alias = "__ZN4Ogre4Pass27setTransparentSortingForcedEb")]
// was: Ogre::Pass::setTransparentSortingForced(bool)
pub fn stub_0xd4befc(pass: &mut Pass, enabled: bool) {
    pass.set_transparent_sorting_forced(enabled) // IDA 0xd4befc: STRB.W R1,[R0,#0x9B]
}

// 0xd4bf04 — __ZNK4Ogre4Pass27getTransparentSortingForcedEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getTransparentSortingForced(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass27getTransparentSortingForcedEv")]
// was: Ogre::Pass::getTransparentSortingForced(void)const
pub fn stub_0xd4bf04(pass: &Pass) -> bool {
    pass.transparent_sorting_forced() // IDA 0xd4bf04: LDRB.W R0,[R0,#0x9B]
}

// 0xd4bf0c — __ZN4Ogre4Pass21setColourWriteEnabledEb
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, bool)
#[doc(alias = "Ogre::Pass::setColourWriteEnabled(bool)")]
#[doc(alias = "__ZN4Ogre4Pass21setColourWriteEnabledEb")]
// was: Ogre::Pass::setColourWriteEnabled(bool)
pub fn stub_0xd4bf0c(pass: &mut Pass, enabled: bool) {
    pass.set_colour_write_enabled(enabled) // IDA 0xd4bf0c: STRB.W R1,[R0,#0x90]
}

// 0xd4bf14 — __ZNK4Ogre4Pass21getColourWriteEnabledEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getColourWriteEnabled(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass21getColourWriteEnabledEv")]
// was: Ogre::Pass::getColourWriteEnabled(void)const
pub fn stub_0xd4bf14(pass: &Pass) -> bool {
    pass.colour_write_enabled() // IDA 0xd4bf14: LDRB.W R0,[R0,#0x90]
}

// 0xd4bf1c — __ZN4Ogre4Pass14setCullingModeENS_11CullingModeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::Pass::setCullingMode(Ogre::CullingMode)")]
#[doc(alias = "__ZN4Ogre4Pass14setCullingModeENS_11CullingModeE")]
// was: Ogre::Pass::setCullingMode(Ogre::CullingMode)
pub fn stub_0xd4bf1c(pass: &mut Pass, mode: CullingMode) {
    pass.set_culling_mode(mode) // IDA 0xd4bf1c: STR.W R1,[R0,#0x9C]
}

// 0xd4bf24 — __ZNK4Ogre4Pass14getCullingModeEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getCullingMode(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass14getCullingModeEv")]
// was: Ogre::Pass::getCullingMode(void)const
pub fn stub_0xd4bf24(pass: &Pass) -> CullingMode {
    pass.culling_mode() // IDA 0xd4bf24: LDR.W R0,[R0,#0x9C]
}

// 0xd4bf2c — __ZN4Ogre4Pass18setLightingEnabledEb
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, bool)
#[doc(alias = "Ogre::Pass::setLightingEnabled(bool)")]
#[doc(alias = "__ZN4Ogre4Pass18setLightingEnabledEb")]
// was: Ogre::Pass::setLightingEnabled(bool)
pub fn stub_0xd4bf2c(pass: &mut Pass, enabled: bool) {
    pass.set_lighting_enabled(enabled) // IDA 0xd4bf2c: STRB.W R1,[R0,#0xA4]
}

// 0xd4bf34 — __ZNK4Ogre4Pass18getLightingEnabledEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getLightingEnabled(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass18getLightingEnabledEv")]
// was: Ogre::Pass::getLightingEnabled(void)const
pub fn stub_0xd4bf34(pass: &Pass) -> bool {
    pass.lighting_enabled() // IDA 0xd4bf34: LDRB.W R0,[R0,#0xA4]
}

// 0xd4bf3c — __ZN4Ogre4Pass24setMaxSimultaneousLightsEt
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::Pass::setMaxSimultaneousLights(unsigned short)")]
#[doc(alias = "__ZN4Ogre4Pass24setMaxSimultaneousLightsEt")]
// was: Ogre::Pass::setMaxSimultaneousLights(unsigned short)
pub fn stub_0xd4bf3c(pass: &mut Pass, count: u16) {
    pass.set_max_simultaneous_lights(count) // IDA 0xd4bf3c: STRH.W R1,[R0,#0xA6]
}

// 0xd4bf44 — __ZNK4Ogre4Pass24getMaxSimultaneousLightsEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getMaxSimultaneousLights(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass24getMaxSimultaneousLightsEv")]
// was: Ogre::Pass::getMaxSimultaneousLights(void)const
pub fn stub_0xd4bf44(pass: &Pass) -> u16 {
    pass.max_simultaneous_lights() // IDA 0xd4bf44: LDRH.W R0,[R0,#0xA6]
}

// 0xd4bf4c — __ZN4Ogre4Pass13setStartLightEt
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::Pass::setStartLight(unsigned short)")]
#[doc(alias = "__ZN4Ogre4Pass13setStartLightEt")]
// was: Ogre::Pass::setStartLight(unsigned short)
pub fn stub_0xd4bf4c(pass: &mut Pass, index: u16) {
    pass.set_start_light(index) // IDA 0xd4bf4c: STRH.W R1,[R0,#0xA8]
}

// 0xd4bf54 — __ZNK4Ogre4Pass13getStartLightEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getStartLight(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass13getStartLightEv")]
// was: Ogre::Pass::getStartLight(void)const
pub fn stub_0xd4bf54(pass: &Pass) -> u16 {
    pass.start_light() // IDA 0xd4bf54: LDRH.W R0,[R0,#0xA8]
}

// 0xd4bf5c — __ZN4Ogre4Pass12setLightMaskEj
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, unsigned int)
#[doc(alias = "Ogre::Pass::setLightMask(unsigned int)")]
#[doc(alias = "__ZN4Ogre4Pass12setLightMaskEj")]
// was: Ogre::Pass::setLightMask(unsigned int)
pub fn stub_0xd4bf5c(pass: &mut Pass, mask: u32) {
    pass.set_light_mask(mask) // IDA 0xd4bf5c: STR.W R1,[R0,#0xB4]
}

// 0xd4bf64 — __ZNK4Ogre4Pass12getLightMaskEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getLightMask(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass12getLightMaskEv")]
// was: Ogre::Pass::getLightMask(void)const
pub fn stub_0xd4bf64(pass: &Pass) -> u32 {
    pass.light_mask() // IDA 0xd4bf64: LDR.W R0,[R0,#0xB4]
}

// 0xd4bf6c — __ZN4Ogre4Pass25setLightCountPerIterationEt
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::Pass::setLightCountPerIteration(unsigned short)")]
#[doc(alias = "__ZN4Ogre4Pass25setLightCountPerIterationEt")]
// was: Ogre::Pass::setLightCountPerIteration(unsigned short)
pub fn stub_0xd4bf6c(pass: &mut Pass, count: u16) {
    pass.set_light_count_per_iteration(count) // IDA 0xd4bf6c: STRH.W R1,[R0,#0xAC]
}

// 0xd4bf74 — __ZNK4Ogre4Pass25getLightCountPerIterationEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getLightCountPerIteration(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass25getLightCountPerIterationEv")]
// was: Ogre::Pass::getLightCountPerIteration(void)const
pub fn stub_0xd4bf74(pass: &Pass) -> u16 {
    pass.light_count_per_iteration() // IDA 0xd4bf74: LDRH.W R0,[R0,#0xAC]
}

// 0xd4bf7c — __ZN4Ogre4Pass18setIteratePerLightEbbNS_5Light10LightTypesE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "Ogre::Pass::setIteratePerLight(bool,bool,Ogre::Light::LightTypes)")]
#[doc(alias = "__ZN4Ogre4Pass18setIteratePerLightEbbNS_5Light10LightTypesE")]
// was: Ogre::Pass::setIteratePerLight(bool,bool,Ogre::Light::LightTypes)
pub fn stub_0xd4bf7c() -> ! {
    todo!("0xd4bf7c Ogre::Pass::setIteratePerLight(bool,bool,Ogre::Light::LightTypes)")
}

// 0xd4bf8c — __ZN4Ogre4Pass14setShadingModeENS_12ShadeOptionsE
#[doc(alias = "Ogre::Pass::setShadingMode(Ogre::ShadeOptions)")]
#[doc(alias = "__ZN4Ogre4Pass14setShadingModeENS_12ShadeOptionsE")]
// was: Ogre::Pass::setShadingMode(Ogre::ShadeOptions)
pub fn stub_0xd4bf8c(pass: &mut Pass, mode: ShadeOptions) {
    pass.set_shading_mode(mode) // IDA 0xd4bf8c: STR.W R1,[R0,#0xB8]
}

// 0xd4bf94 — __ZNK4Ogre4Pass14getShadingModeEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadingMode(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass14getShadingModeEv")]
// was: Ogre::Pass::getShadingMode(void)const
pub fn stub_0xd4bf94(pass: &Pass) -> ShadeOptions {
    pass.shading_mode() // IDA 0xd4bf94: LDR.W R0,[R0,#0xB8]
}

// 0xd4bf9c — __ZN4Ogre4Pass14setPolygonModeENS_11PolygonModeE
#[doc(alias = "Ogre::Pass::setPolygonMode(Ogre::PolygonMode)")]
#[doc(alias = "__ZN4Ogre4Pass14setPolygonModeENS_11PolygonModeE")]
// was: Ogre::Pass::setPolygonMode(Ogre::PolygonMode)
pub fn stub_0xd4bf9c(pass: &mut Pass, mode: PolygonMode) {
    pass.set_polygon_mode(mode) // IDA 0xd4bf9c: STR.W R1,[R0,#0xBC]
}

// 0xd4bfa4 — __ZNK4Ogre4Pass14getPolygonModeEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getPolygonMode(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass14getPolygonModeEv")]
// was: Ogre::Pass::getPolygonMode(void)const
pub fn stub_0xd4bfa4(pass: &Pass) -> PolygonMode {
    pass.polygon_mode() // IDA 0xd4bfa4: LDR.W R0,[R0,#0xBC]
}

// 0xd4bfac — __ZN4Ogre4Pass20setManualCullingModeENS_17ManualCullingModeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::Pass::setManualCullingMode(Ogre::ManualCullingMode)")]
#[doc(alias = "__ZN4Ogre4Pass20setManualCullingModeENS_17ManualCullingModeE")]
// was: Ogre::Pass::setManualCullingMode(Ogre::ManualCullingMode)
pub fn stub_0xd4bfac(pass: &mut Pass, mode: ManualCullingMode) {
    pass.set_manual_culling_mode(mode) // IDA 0xd4bfac: STR.W R1,[R0,#0xA0]
}

// 0xd4bfb4 — __ZNK4Ogre4Pass20getManualCullingModeEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getManualCullingMode(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass20getManualCullingModeEv")]
// was: Ogre::Pass::getManualCullingMode(void)const
pub fn stub_0xd4bfb4(pass: &Pass) -> ManualCullingMode {
    pass.manual_culling_mode() // IDA 0xd4bfb4: LDR.W R0,[R0,#0xA0]
}

// 0xd4bfbc — __ZN4Ogre4Pass6setFogEbNS_7FogModeERKNS_11ColourValueEfff
// type: int __fastcall(int, int, int, int, float, float, float)
#[doc(alias = "Ogre::Pass::setFog(bool,Ogre::FogMode,Ogre::ColourValue const&,float,float,float)")]
#[doc(alias = "__ZN4Ogre4Pass6setFogEbNS_7FogModeERKNS_11ColourValueEfff")]
// was: Ogre::Pass::setFog(bool,Ogre::FogMode,Ogre::ColourValue const&,float,float,float)
pub fn stub_0xd4bfbc() -> ! {
    todo!("0xd4bfbc Ogre::Pass::setFog(bool,Ogre::FogMode,Ogre::ColourValue const&,float,float,float)")
}

// 0xd4bff0 — __ZNK4Ogre4Pass14getFogOverrideEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getFogOverride(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass14getFogOverrideEv")]
// was: Ogre::Pass::getFogOverride(void)const
pub fn stub_0xd4bff0() -> ! {
    todo!("0xd4bff0 Ogre::Pass::getFogOverride(void)const")
}

// 0xd4bff8 — __ZNK4Ogre4Pass10getFogModeEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getFogMode(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass10getFogModeEv")]
// was: Ogre::Pass::getFogMode(void)const
pub fn stub_0xd4bff8() -> ! {
    todo!("0xd4bff8 Ogre::Pass::getFogMode(void)const")
}

// 0xd4c000 — __ZNK4Ogre4Pass12getFogColourEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getFogColour(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass12getFogColourEv")]
// was: Ogre::Pass::getFogColour(void)const
pub fn stub_0xd4c000() -> ! {
    todo!("0xd4c000 Ogre::Pass::getFogColour(void)const")
}

// 0xd4c004 — __ZNK4Ogre4Pass11getFogStartEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getFogStart(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass11getFogStartEv")]
// was: Ogre::Pass::getFogStart(void)const
pub fn stub_0xd4c004() -> ! {
    todo!("0xd4c004 Ogre::Pass::getFogStart(void)const")
}

// 0xd4c00c — __ZNK4Ogre4Pass9getFogEndEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getFogEnd(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass9getFogEndEv")]
// was: Ogre::Pass::getFogEnd(void)const
pub fn stub_0xd4c00c() -> ! {
    todo!("0xd4c00c Ogre::Pass::getFogEnd(void)const")
}

// 0xd4c014 — __ZNK4Ogre4Pass13getFogDensityEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getFogDensity(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass13getFogDensityEv")]
// was: Ogre::Pass::getFogDensity(void)const
pub fn stub_0xd4c014() -> ! {
    todo!("0xd4c014 Ogre::Pass::getFogDensity(void)const")
}

// 0xd4c01c — __ZN4Ogre4Pass12setDepthBiasEff
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, float, float)
#[doc(alias = "Ogre::Pass::setDepthBias(float,float)")]
#[doc(alias = "__ZN4Ogre4Pass12setDepthBiasEff")]
// was: Ogre::Pass::setDepthBias(float,float)
pub fn stub_0xd4c01c() -> ! {
    todo!("0xd4c01c Ogre::Pass::setDepthBias(float,float)")
}

// 0xd4c028 — __ZNK4Ogre4Pass20getDepthBiasConstantEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getDepthBiasConstant(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass20getDepthBiasConstantEv")]
// was: Ogre::Pass::getDepthBiasConstant(void)const
pub fn stub_0xd4c028() -> ! {
    todo!("0xd4c028 Ogre::Pass::getDepthBiasConstant(void)const")
}

// 0xd4c030 — __ZNK4Ogre4Pass22getDepthBiasSlopeScaleEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getDepthBiasSlopeScale(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass22getDepthBiasSlopeScaleEv")]
// was: Ogre::Pass::getDepthBiasSlopeScale(void)const
pub fn stub_0xd4c030() -> ! {
    todo!("0xd4c030 Ogre::Pass::getDepthBiasSlopeScale(void)const")
}

// 0xd4c038 — __ZN4Ogre4Pass21setIterationDepthBiasEf
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, float)
#[doc(alias = "Ogre::Pass::setIterationDepthBias(float)")]
#[doc(alias = "__ZN4Ogre4Pass21setIterationDepthBiasEf")]
// was: Ogre::Pass::setIterationDepthBias(float)
pub fn stub_0xd4c038() -> ! {
    todo!("0xd4c038 Ogre::Pass::setIterationDepthBias(float)")
}

// 0xd4c040 — __ZNK4Ogre4Pass21getIterationDepthBiasEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getIterationDepthBias(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass21getIterationDepthBiasEv")]
// was: Ogre::Pass::getIterationDepthBias(void)const
pub fn stub_0xd4c040() -> ! {
    todo!("0xd4c040 Ogre::Pass::getIterationDepthBias(void)const")
}

// 0xd4c048 — __ZN4Ogre4Pass6_splitEt
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::Pass::_split(unsigned short)")]
#[doc(alias = "__ZN4Ogre4Pass6_splitEt")]
// was: Ogre::Pass::_split(unsigned short)
pub fn stub_0xd4c048() -> ! {
    todo!("0xd4c048 Ogre::Pass::_split(unsigned short)")
}

// 0xd4c388 — __ZN4Ogre4Pass12_notifyIndexEt
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::Pass::_notifyIndex(unsigned short)")]
#[doc(alias = "__ZN4Ogre4Pass12_notifyIndexEt")]
// was: Ogre::Pass::_notifyIndex(unsigned short)
pub fn stub_0xd4c388() -> ! {
    todo!("0xd4c388 Ogre::Pass::_notifyIndex(unsigned short)")
}

// 0xd4c3d4 — __ZN4Ogre4Pass8_prepareEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::_prepare(void)")]
#[doc(alias = "__ZN4Ogre4Pass8_prepareEv")]
// was: Ogre::Pass::_prepare(void)
pub fn stub_0xd4c3d4() -> ! {
    todo!("0xd4c3d4 Ogre::Pass::_prepare(void)")
}

// 0xd4c3f0 — __ZN4Ogre4Pass10_unprepareEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::_unprepare(void)")]
#[doc(alias = "__ZN4Ogre4Pass10_unprepareEv")]
// was: Ogre::Pass::_unprepare(void)
pub fn stub_0xd4c3f0() -> ! {
    todo!("0xd4c3f0 Ogre::Pass::_unprepare(void)")
}

// 0xd4c40c — __ZN4Ogre4Pass5_loadEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::_load(void)")]
#[doc(alias = "__ZN4Ogre4Pass5_loadEv")]
// was: Ogre::Pass::_load(void)
pub fn stub_0xd4c40c() -> ! {
    todo!("0xd4c40c Ogre::Pass::_load(void)")
}

// 0xd4c4c0 — __ZN4Ogre4Pass7_unloadEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::_unload(void)")]
#[doc(alias = "__ZN4Ogre4Pass7_unloadEv")]
// was: Ogre::Pass::_unload(void)
pub fn stub_0xd4c4c0() -> ! {
    todo!("0xd4c4c0 Ogre::Pass::_unload(void)")
}

// 0xd4c4dc — __ZN4Ogre4Pass16setVertexProgramERKSsb
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, const std::string *, bool)
#[doc(alias = "Ogre::Pass::setVertexProgram(std::string const&,bool)")]
#[doc(alias = "__ZN4Ogre4Pass16setVertexProgramERKSsb")]
// was: Ogre::Pass::setVertexProgram(std::string const&,bool)
pub fn stub_0xd4c4dc() -> ! {
    todo!("0xd4c4dc Ogre::Pass::setVertexProgram(std::string const&,bool)")
}

// 0xd4c698 — __ZNK4Ogre4Pass20getVertexProgramNameEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getVertexProgramName(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass20getVertexProgramNameEv")]
// was: Ogre::Pass::getVertexProgramName(void)const
pub fn stub_0xd4c698() -> ! {
    todo!("0xd4c698 Ogre::Pass::getVertexProgramName(void)const")
}

// 0xd4c920 — __ZN4Ogre4Pass18setFragmentProgramERKSsb
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, const std::string *, bool)
#[doc(alias = "Ogre::Pass::setFragmentProgram(std::string const&,bool)")]
#[doc(alias = "__ZN4Ogre4Pass18setFragmentProgramERKSsb")]
// was: Ogre::Pass::setFragmentProgram(std::string const&,bool)
pub fn stub_0xd4c920() -> ! {
    todo!("0xd4c920 Ogre::Pass::setFragmentProgram(std::string const&,bool)")
}

// 0xd4cadc — __ZNK4Ogre4Pass22getFragmentProgramNameEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getFragmentProgramName(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass22getFragmentProgramNameEv")]
// was: Ogre::Pass::getFragmentProgramName(void)const
pub fn stub_0xd4cadc() -> ! {
    todo!("0xd4cadc Ogre::Pass::getFragmentProgramName(void)const")
}

// 0xd4cd64 — __ZN4Ogre4Pass18setGeometryProgramERKSsb
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, const std::string *, bool)
#[doc(alias = "Ogre::Pass::setGeometryProgram(std::string const&,bool)")]
#[doc(alias = "__ZN4Ogre4Pass18setGeometryProgramERKSsb")]
// was: Ogre::Pass::setGeometryProgram(std::string const&,bool)
pub fn stub_0xd4cd64() -> ! {
    todo!("0xd4cd64 Ogre::Pass::setGeometryProgram(std::string const&,bool)")
}

// 0xd4cf20 — __ZNK4Ogre4Pass22getGeometryProgramNameEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getGeometryProgramName(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass22getGeometryProgramNameEv")]
// was: Ogre::Pass::getGeometryProgramName(void)const
pub fn stub_0xd4cf20() -> ! {
    todo!("0xd4cf20 Ogre::Pass::getGeometryProgramName(void)const")
}

// 0xd4cf44 — __ZNK4Ogre4Pass26getVertexProgramParametersEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getVertexProgramParameters(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass26getVertexProgramParametersEv")]
// was: Ogre::Pass::getVertexProgramParameters(void)const
pub fn stub_0xd4cf44() -> ! {
    todo!("0xd4cf44 Ogre::Pass::getVertexProgramParameters(void)const")
}

// 0xd4d128 — __ZNK4Ogre4Pass16getVertexProgramEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getVertexProgram(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass16getVertexProgramEv")]
// was: Ogre::Pass::getVertexProgram(void)const
pub fn stub_0xd4d128() -> ! {
    todo!("0xd4d128 Ogre::Pass::getVertexProgram(void)const")
}

// 0xd4d130 — __ZNK4Ogre4Pass28getFragmentProgramParametersEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getFragmentProgramParameters(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass28getFragmentProgramParametersEv")]
// was: Ogre::Pass::getFragmentProgramParameters(void)const
pub fn stub_0xd4d130() -> ! {
    todo!("0xd4d130 Ogre::Pass::getFragmentProgramParameters(void)const")
}

// 0xd4d140 — __ZNK4Ogre4Pass18getFragmentProgramEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getFragmentProgram(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass18getFragmentProgramEv")]
// was: Ogre::Pass::getFragmentProgram(void)const
pub fn stub_0xd4d140() -> ! {
    todo!("0xd4d140 Ogre::Pass::getFragmentProgram(void)const")
}

// 0xd4d148 — __ZNK4Ogre4Pass28getGeometryProgramParametersEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getGeometryProgramParameters(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass28getGeometryProgramParametersEv")]
// was: Ogre::Pass::getGeometryProgramParameters(void)const
pub fn stub_0xd4d148() -> ! {
    todo!("0xd4d148 Ogre::Pass::getGeometryProgramParameters(void)const")
}

// 0xd4d158 — __ZNK4Ogre4Pass18getGeometryProgramEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getGeometryProgram(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass18getGeometryProgramEv")]
// was: Ogre::Pass::getGeometryProgram(void)const
pub fn stub_0xd4d158() -> ! {
    todo!("0xd4d158 Ogre::Pass::getGeometryProgram(void)const")
}

// 0xd4d160 — __ZNK4Ogre4Pass8isLoadedEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::isLoaded(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass8isLoadedEv")]
// was: Ogre::Pass::isLoaded(void)const
pub fn stub_0xd4d160() -> ! {
    todo!("0xd4d160 Ogre::Pass::isLoaded(void)const")
}

// 0xd4d16c — __ZN4Ogre4Pass21_notifyNeedsRecompileEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::_notifyNeedsRecompile(void)")]
#[doc(alias = "__ZN4Ogre4Pass21_notifyNeedsRecompileEv")]
// was: Ogre::Pass::_notifyNeedsRecompile(void)
pub fn stub_0xd4d16c() -> ! {
    todo!("0xd4d16c Ogre::Pass::_notifyNeedsRecompile(void)")
}

// 0xd4d178 — __ZNK4Ogre4Pass17_updateAutoParamsEPKNS_19AutoParamDataSourceEt
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, const Ogre::AutoParamDataSource *, unsigned __int16)
#[doc(alias = "Ogre::Pass::_updateAutoParams(Ogre::AutoParamDataSource const*,unsigned short)const")]
#[doc(alias = "__ZNK4Ogre4Pass17_updateAutoParamsEPKNS_19AutoParamDataSourceEt")]
// was: Ogre::Pass::_updateAutoParams(Ogre::AutoParamDataSource const*,unsigned short)const
pub fn stub_0xd4d178() -> ! {
    todo!("0xd4d178 Ogre::Pass::_updateAutoParams(Ogre::AutoParamDataSource const*,unsigned short)const")
}

// 0xd4d37c — __ZN4Ogre4Pass25processPendingPassUpdatesEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::processPendingPassUpdates(void)")]
#[doc(alias = "__ZN4Ogre4Pass25processPendingPassUpdatesEv")]
// was: Ogre::Pass::processPendingPassUpdates(void)
pub fn stub_0xd4d37c() -> ! {
    todo!("0xd4d37c Ogre::Pass::processPendingPassUpdates(void)")
}

// 0xd4d508 — __ZN4Ogre4Pass16queueForDeletionEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::queueForDeletion(void)")]
#[doc(alias = "__ZN4Ogre4Pass16queueForDeletionEv")]
// was: Ogre::Pass::queueForDeletion(void)
pub fn stub_0xd4d508() -> ! {
    todo!("0xd4d508 Ogre::Pass::queueForDeletion(void)")
}

// 0xd4d608 — __ZNK4Ogre4Pass13isAmbientOnlyEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::isAmbientOnly(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass13isAmbientOnlyEv")]
// was: Ogre::Pass::isAmbientOnly(void)const
pub fn stub_0xd4d608() -> ! {
    todo!("0xd4d608 Ogre::Pass::isAmbientOnly(void)const")
}

// 0xd4d64c — __ZN4Ogre4Pass28setShadowCasterVertexProgramERKSs
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, const std::string *)
#[doc(alias = "Ogre::Pass::setShadowCasterVertexProgram(std::string const&)")]
#[doc(alias = "__ZN4Ogre4Pass28setShadowCasterVertexProgramERKSs")]
// was: Ogre::Pass::setShadowCasterVertexProgram(std::string const&)
pub fn stub_0xd4d64c() -> ! {
    todo!("0xd4d64c Ogre::Pass::setShadowCasterVertexProgram(std::string const&)")
}

// 0xd4d764 — __ZNK4Ogre4Pass32getShadowCasterVertexProgramNameEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadowCasterVertexProgramName(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass32getShadowCasterVertexProgramNameEv")]
// was: Ogre::Pass::getShadowCasterVertexProgramName(void)const
pub fn stub_0xd4d764() -> ! {
    todo!("0xd4d764 Ogre::Pass::getShadowCasterVertexProgramName(void)const")
}

// 0xd4d788 — __ZNK4Ogre4Pass38getShadowCasterVertexProgramParametersEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadowCasterVertexProgramParameters(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass38getShadowCasterVertexProgramParametersEv")]
// was: Ogre::Pass::getShadowCasterVertexProgramParameters(void)const
pub fn stub_0xd4d788() -> ! {
    todo!("0xd4d788 Ogre::Pass::getShadowCasterVertexProgramParameters(void)const")
}

// 0xd4d96c — __ZNK4Ogre4Pass28getShadowCasterVertexProgramEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadowCasterVertexProgram(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass28getShadowCasterVertexProgramEv")]
// was: Ogre::Pass::getShadowCasterVertexProgram(void)const
pub fn stub_0xd4d96c() -> ! {
    todo!("0xd4d96c Ogre::Pass::getShadowCasterVertexProgram(void)const")
}

// 0xd4d974 — __ZN4Ogre4Pass30setShadowCasterFragmentProgramERKSs
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, const std::string *)
#[doc(alias = "Ogre::Pass::setShadowCasterFragmentProgram(std::string const&)")]
#[doc(alias = "__ZN4Ogre4Pass30setShadowCasterFragmentProgramERKSs")]
// was: Ogre::Pass::setShadowCasterFragmentProgram(std::string const&)
pub fn stub_0xd4d974() -> ! {
    todo!("0xd4d974 Ogre::Pass::setShadowCasterFragmentProgram(std::string const&)")
}

// 0xd4da8c — __ZNK4Ogre4Pass34getShadowCasterFragmentProgramNameEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadowCasterFragmentProgramName(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass34getShadowCasterFragmentProgramNameEv")]
// was: Ogre::Pass::getShadowCasterFragmentProgramName(void)const
pub fn stub_0xd4da8c() -> ! {
    todo!("0xd4da8c Ogre::Pass::getShadowCasterFragmentProgramName(void)const")
}

// 0xd4dab0 — __ZNK4Ogre4Pass40getShadowCasterFragmentProgramParametersEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadowCasterFragmentProgramParameters(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass40getShadowCasterFragmentProgramParametersEv")]
// was: Ogre::Pass::getShadowCasterFragmentProgramParameters(void)const
pub fn stub_0xd4dab0() -> ! {
    todo!("0xd4dab0 Ogre::Pass::getShadowCasterFragmentProgramParameters(void)const")
}

// 0xd4dcbc — __ZNK4Ogre4Pass30getShadowCasterFragmentProgramEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadowCasterFragmentProgram(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass30getShadowCasterFragmentProgramEv")]
// was: Ogre::Pass::getShadowCasterFragmentProgram(void)const
pub fn stub_0xd4dcbc() -> ! {
    todo!("0xd4dcbc Ogre::Pass::getShadowCasterFragmentProgram(void)const")
}

// 0xd4dcc4 — __ZN4Ogre4Pass30setShadowReceiverVertexProgramERKSs
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, const std::string *)
#[doc(alias = "Ogre::Pass::setShadowReceiverVertexProgram(std::string const&)")]
#[doc(alias = "__ZN4Ogre4Pass30setShadowReceiverVertexProgramERKSs")]
// was: Ogre::Pass::setShadowReceiverVertexProgram(std::string const&)
pub fn stub_0xd4dcc4() -> ! {
    todo!("0xd4dcc4 Ogre::Pass::setShadowReceiverVertexProgram(std::string const&)")
}

// 0xd4dddc — __ZNK4Ogre4Pass34getShadowReceiverVertexProgramNameEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadowReceiverVertexProgramName(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass34getShadowReceiverVertexProgramNameEv")]
// was: Ogre::Pass::getShadowReceiverVertexProgramName(void)const
pub fn stub_0xd4dddc() -> ! {
    todo!("0xd4dddc Ogre::Pass::getShadowReceiverVertexProgramName(void)const")
}

// 0xd4de00 — __ZNK4Ogre4Pass40getShadowReceiverVertexProgramParametersEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadowReceiverVertexProgramParameters(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass40getShadowReceiverVertexProgramParametersEv")]
// was: Ogre::Pass::getShadowReceiverVertexProgramParameters(void)const
pub fn stub_0xd4de00() -> ! {
    todo!("0xd4de00 Ogre::Pass::getShadowReceiverVertexProgramParameters(void)const")
}

// 0xd4dfe4 — __ZNK4Ogre4Pass30getShadowReceiverVertexProgramEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadowReceiverVertexProgram(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass30getShadowReceiverVertexProgramEv")]
// was: Ogre::Pass::getShadowReceiverVertexProgram(void)const
pub fn stub_0xd4dfe4() -> ! {
    todo!("0xd4dfe4 Ogre::Pass::getShadowReceiverVertexProgram(void)const")
}

// 0xd4dfec — __ZN4Ogre4Pass32setShadowReceiverFragmentProgramERKSs
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, const std::string *)
#[doc(alias = "Ogre::Pass::setShadowReceiverFragmentProgram(std::string const&)")]
#[doc(alias = "__ZN4Ogre4Pass32setShadowReceiverFragmentProgramERKSs")]
// was: Ogre::Pass::setShadowReceiverFragmentProgram(std::string const&)
pub fn stub_0xd4dfec() -> ! {
    todo!("0xd4dfec Ogre::Pass::setShadowReceiverFragmentProgram(std::string const&)")
}

// 0xd4e104 — __ZNK4Ogre4Pass36getShadowReceiverFragmentProgramNameEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadowReceiverFragmentProgramName(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass36getShadowReceiverFragmentProgramNameEv")]
// was: Ogre::Pass::getShadowReceiverFragmentProgramName(void)const
pub fn stub_0xd4e104() -> ! {
    todo!("0xd4e104 Ogre::Pass::getShadowReceiverFragmentProgramName(void)const")
}

// 0xd4e128 — __ZNK4Ogre4Pass42getShadowReceiverFragmentProgramParametersEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadowReceiverFragmentProgramParameters(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass42getShadowReceiverFragmentProgramParametersEv")]
// was: Ogre::Pass::getShadowReceiverFragmentProgramParameters(void)const
pub fn stub_0xd4e128() -> ! {
    todo!("0xd4e128 Ogre::Pass::getShadowReceiverFragmentProgramParameters(void)const")
}

// 0xd4e30c — __ZNK4Ogre4Pass32getShadowReceiverFragmentProgramEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getShadowReceiverFragmentProgram(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass32getShadowReceiverFragmentProgramEv")]
// was: Ogre::Pass::getShadowReceiverFragmentProgram(void)const
pub fn stub_0xd4e30c() -> ! {
    todo!("0xd4e30c Ogre::Pass::getShadowReceiverFragmentProgram(void)const")
}

// 0xd4e314 — __ZNK4Ogre4Pass16getResourceGroupEv
// type: _DWORD __fastcall(Ogre::Pass *__hidden this)
#[doc(alias = "Ogre::Pass::getResourceGroup(void)const")]
#[doc(alias = "__ZNK4Ogre4Pass16getResourceGroupEv")]
// was: Ogre::Pass::getResourceGroup(void)const
pub fn stub_0xd4e314() -> ! {
    todo!("0xd4e314 Ogre::Pass::getResourceGroup(void)const")
}

// 0xd4e320 — __ZNK4Ogre4Pass19applyTextureAliasesERKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb
#[doc(alias = "Ogre::Pass::applyTextureAliases(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,bool)const")]
#[doc(alias = "__ZNK4Ogre4Pass19applyTextureAliasesERKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb")]
// was: Ogre::Pass::applyTextureAliases(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,bool)const
pub fn stub_0xd4e320() -> ! {
    todo!("0xd4e320 Ogre::Pass::applyTextureAliases(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,bool)const")
}

// 0xd4e358 — __ZNK4Ogre4Pass35_getTextureUnitWithContentTypeIndexENS_16TextureUnitState11ContentTypeEt
#[doc(alias = "Ogre::Pass::_getTextureUnitWithContentTypeIndex(Ogre::TextureUnitState::ContentType,unsigned short)const")]
#[doc(alias = "__ZNK4Ogre4Pass35_getTextureUnitWithContentTypeIndexENS_16TextureUnitState11ContentTypeEt")]
// was: Ogre::Pass::_getTextureUnitWithContentTypeIndex(Ogre::TextureUnitState::ContentType,unsigned short)const
pub fn stub_0xd4e358() -> ! {
    todo!("0xd4e358 Ogre::Pass::_getTextureUnitWithContentTypeIndex(Ogre::TextureUnitState::ContentType,unsigned short)const")
}

// 0xd4e458 — __ZN4Ogre29MinTextureStateChangeHashFuncD1Ev
// type: void __fastcall(Ogre::MinTextureStateChangeHashFunc *__hidden this)
#[doc(alias = "Ogre::MinTextureStateChangeHashFunc::~MinTextureStateChangeHashFunc()")]
#[doc(alias = "__ZN4Ogre29MinTextureStateChangeHashFuncD1Ev")]
// was: Ogre::MinTextureStateChangeHashFunc::~MinTextureStateChangeHashFunc()
pub fn stub_0xd4e458() -> ! {
    todo!("0xd4e458 Ogre::MinTextureStateChangeHashFunc::~MinTextureStateChangeHashFunc()")
}

// 0xd4e45c — __ZN4Ogre27MinGpuProgramChangeHashFuncD1Ev
// type: void __fastcall(Ogre::MinGpuProgramChangeHashFunc *__hidden this)
#[doc(alias = "Ogre::MinGpuProgramChangeHashFunc::~MinGpuProgramChangeHashFunc()")]
#[doc(alias = "__ZN4Ogre27MinGpuProgramChangeHashFuncD1Ev")]
// was: Ogre::MinGpuProgramChangeHashFunc::~MinGpuProgramChangeHashFunc()
pub fn stub_0xd4e45c() -> ! {
    todo!("0xd4e45c Ogre::MinGpuProgramChangeHashFunc::~MinGpuProgramChangeHashFunc()")
}

// 0xd4e460 — __ZNSt3setIPN4Ogre4PassESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(alias = "std::set<Ogre::Pass *,std::less<Ogre::Pass *>,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~set()")]
#[doc(alias = "__ZNSt3setIPN4Ogre4PassESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev")]
// was: std::set<Ogre::Pass *,std::less<Ogre::Pass *>,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~set()
pub fn stub_0xd4e460() -> ! {
    todo!("0xd4e460 std::set<Ogre::Pass *,std::less<Ogre::Pass *>,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~set()")
}

// 0xd4e4f4 — __ZN4Ogre4Pass26setPolygonModeOverrideableEb
// type: _DWORD __fastcall(Ogre::Pass *__hidden this, bool)
#[doc(alias = "Ogre::Pass::setPolygonModeOverrideable(bool)")]
#[doc(alias = "__ZN4Ogre4Pass26setPolygonModeOverrideableEb")]
// was: Ogre::Pass::setPolygonModeOverrideable(bool)
pub fn stub_0xd4e4f4() -> ! {
    todo!("0xd4e4f4 Ogre::Pass::setPolygonModeOverrideable(bool)")
}
