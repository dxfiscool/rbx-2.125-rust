//! rendering generated — next 150 stubs
//! Filter: Ogre|Gfx|Render|G3D (15058 total, 5350 prior +150 this batch = 5500 total) — 0xca1580..0xcb64e0 after 0xc8a3d4
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(
    non_snake_case,
    dead_code,
    unused_variables,
    unused_imports,
    clippy::all
)]

use rbx_core::SharedPtr;

// 0xc6e6f0 — __ZNK4Ogre13MovableObject16isParentTagPointEv
#[doc(alias = "Ogre::MovableObject::isParentTagPoint(void)const")]
// was: Ogre::MovableObject::isParentTagPoint(void)const
// IDA 0xc6e6f0: LDRB [R0,#0x18], BX LR — bool field load; recovered: &MovableObject -> bool.
pub fn stub_c6e6f0(obj: &crate::movable::MovableObject) -> bool {
    obj.is_parent_tag_point()
}

// 0xc6e6f4 — __ZN4Ogre13MovableObject20setRenderingDistanceEf
#[doc(alias = "Ogre::MovableObject::setRenderingDistance(float)")]
// was: Ogre::MovableObject::setRenderingDistance(float)
// IDA 0xc6e6f4: STR [R0,#0x1C] + VMUL.F32 D0,D16,D16 / VSTR [R0,#0x20] — stores distance and cached square; recovered: &mut MovableObject, f32.
pub fn stub_c6e6f4(obj: &mut crate::movable::MovableObject, distance: f32) {
    obj.set_rendering_distance(distance)
}

// 0xc6e708 — __ZNK4Ogre13MovableObject20getRenderingDistanceEv
#[doc(alias = "Ogre::MovableObject::getRenderingDistance(void)const")]
// was: Ogre::MovableObject::getRenderingDistance(void)const
// IDA 0xc6e708: LDR R0,[R0,#0x1C], BX LR — float field load; recovered: &MovableObject -> f32.
pub fn stub_c6e708(obj: &crate::movable::MovableObject) -> f32 {
    obj.rendering_distance()
}

// 0xc6e70c — __ZN4Ogre13MovableObject24setRenderingMinPixelSizeEf
#[doc(alias = "Ogre::MovableObject::setRenderingMinPixelSize(float)")]
// was: Ogre::MovableObject::setRenderingMinPixelSize(float)
// IDA 0xc6e70c: STR R1,[R0,#0x24], BX LR — float field store; recovered: &mut MovableObject, f32.
pub fn stub_c6e70c(obj: &mut crate::movable::MovableObject, size: f32) {
    obj.set_rendering_min_pixel_size(size)
}

// 0xc6e710 — __ZNK4Ogre13MovableObject24getRenderingMinPixelSizeEv
#[doc(alias = "Ogre::MovableObject::getRenderingMinPixelSize(void)const")]
// was: Ogre::MovableObject::getRenderingMinPixelSize(void)const
// IDA 0xc6e710: LDR R0,[R0,#0x24], BX LR — float field load; recovered: &MovableObject -> f32.
pub fn stub_c6e710(obj: &crate::movable::MovableObject) -> f32 {
    obj.rendering_min_pixel_size()
}

// 0xc6e714 — __ZN4Ogre13MovableObject10setUserAnyERKNS_3AnyE
#[doc(alias = "Ogre::MovableObject::setUserAny(Ogre::Any const&)")]
// was: Ogre::MovableObject::setUserAny(Ogre::Any const&)
// IDA 0xc6e714: ADDS R0,#0x2C + BL Ogre::UserObjectBindings::setUserAny — delegates to bindings at this+0x2C; recovered: &mut MovableObject, UserAny.
pub fn stub_c6e714(obj: &mut crate::movable::MovableObject, any: crate::movable::UserAny) {
    obj.set_user_any(any)
}

// 0xc6e720 — __ZNK4Ogre13MovableObject10getUserAnyEv
#[doc(alias = "Ogre::MovableObject::getUserAny(void)const")]
// was: Ogre::MovableObject::getUserAny(void)const
// IDA 0xc6e720: ADDS R0,#0x2C + BL Ogre::UserObjectBindings::getUserAny — delegates to bindings at this+0x2C; recovered: &MovableObject -> &UserAny.
pub fn stub_c6e720(obj: &crate::movable::MovableObject) -> &crate::movable::UserAny {
    obj.user_any()
}

// 0xc6e72c — __ZN4Ogre13MovableObject13setQueryFlagsEj
#[doc(alias = "Ogre::MovableObject::setQueryFlags(unsigned int)")]
// was: Ogre::MovableObject::setQueryFlags(unsigned int)
// IDA 0xc6e72c: STR R1,[R0,#0x3C], BX LR — u32 field store; recovered: &mut MovableObject, u32.
pub fn stub_c6e72c(obj: &mut crate::movable::MovableObject, flags: u32) {
    obj.set_query_flags(flags)
}

// 0xc6e730 — __ZN4Ogre13MovableObject13addQueryFlagsEj
#[doc(alias = "Ogre::MovableObject::addQueryFlags(unsigned int)")]
// was: Ogre::MovableObject::addQueryFlags(unsigned int)
// IDA 0xc6e730: LDR R2,[R0,#0x3C] / ORRS R1,R2 / STR — flags |= mask; recovered: &mut MovableObject, u32.
pub fn stub_c6e730(obj: &mut crate::movable::MovableObject, flags: u32) {
    obj.add_query_flags(flags)
}

// 0xc6e738 — __ZN4Ogre13MovableObject16removeQueryFlagsEj
#[doc(alias = "Ogre::MovableObject::removeQueryFlags(unsigned int)")]
// was: Ogre::MovableObject::removeQueryFlags(unsigned int)
// IDA 0xc6e738: LDR R2,[R0,#0x3C] / BIC.W R1,R2,R1 / STR — flags &= ~mask; recovered: &mut MovableObject, u32.
pub fn stub_c6e738(obj: &mut crate::movable::MovableObject, flags: u32) {
    obj.remove_query_flags(flags)
}

// 0xc6e744 — __ZNK4Ogre13MovableObject13getQueryFlagsEv
#[doc(alias = "Ogre::MovableObject::getQueryFlags(void)const")]
// was: Ogre::MovableObject::getQueryFlags(void)const
// IDA 0xc6e744: LDR R0,[R0,#0x3C], BX LR — u32 field load; recovered: &MovableObject -> u32.
pub fn stub_c6e744(obj: &crate::movable::MovableObject) -> u32 {
    obj.query_flags()
}

// 0xc6e748 — __ZN4Ogre13MovableObject18setVisibilityFlagsEj
#[doc(alias = "Ogre::MovableObject::setVisibilityFlags(unsigned int)")]
// was: Ogre::MovableObject::setVisibilityFlags(unsigned int)
// IDA 0xc6e748: STR R1,[R0,#0x40], BX LR — u32 field store; recovered: &mut MovableObject, u32.
pub fn stub_c6e748(obj: &mut crate::movable::MovableObject, flags: u32) {
    obj.set_visibility_flags(flags)
}

// 0xc6e74c — __ZN4Ogre13MovableObject18addVisibilityFlagsEj
#[doc(alias = "Ogre::MovableObject::addVisibilityFlags(unsigned int)")]
// was: Ogre::MovableObject::addVisibilityFlags(unsigned int)
// IDA 0xc6e74c: LDR R2,[R0,#0x40] / ORRS R1,R2 / STR — flags |= mask; recovered: &mut MovableObject, u32.
pub fn stub_c6e74c(obj: &mut crate::movable::MovableObject, flags: u32) {
    obj.add_visibility_flags(flags)
}

// 0xc6e754 — __ZN4Ogre13MovableObject21removeVisibilityFlagsEj
#[doc(alias = "Ogre::MovableObject::removeVisibilityFlags(unsigned int)")]
// was: Ogre::MovableObject::removeVisibilityFlags(unsigned int)
// IDA 0xc6e754: LDR R2,[R0,#0x40] / BIC.W R1,R2,R1 / STR — flags &= ~mask; recovered: &mut MovableObject, u32.
pub fn stub_c6e754(obj: &mut crate::movable::MovableObject, flags: u32) {
    obj.remove_visibility_flags(flags)
}

// 0xc6e760 — __ZNK4Ogre13MovableObject18getVisibilityFlagsEv
#[doc(alias = "Ogre::MovableObject::getVisibilityFlags(void)const")]
// was: Ogre::MovableObject::getVisibilityFlags(void)const
// IDA 0xc6e760: LDR R0,[R0,#0x40], BX LR — u32 field load; recovered: &MovableObject -> u32.
pub fn stub_c6e760(obj: &crate::movable::MovableObject) -> u32 {
    obj.visibility_flags()
}

// 0xc6e764 — __ZN4Ogre13MovableObject11setListenerEPNS0_8ListenerE
#[doc(alias = "Ogre::MovableObject::setListener(Ogre::MovableObject::Listener *)")]
// was: Ogre::MovableObject::setListener(Ogre::MovableObject::Listener *)
// IDA 0xc6e764: STR.W R1,[R0,#0x98], BX LR — listener pointer store; recovered: &mut MovableObject, Option<usize>.
pub fn stub_c6e764(obj: &mut crate::movable::MovableObject, listener: Option<usize>) {
    obj.set_listener(listener)
}

// 0xc6e76c — __ZNK4Ogre13MovableObject11getListenerEv
#[doc(alias = "Ogre::MovableObject::getListener(void)const")]
// was: Ogre::MovableObject::getListener(void)const
// IDA 0xc6e76c: LDR.W R0,[R0,#0x98], BX LR — listener pointer load; recovered: &MovableObject -> Option<usize>.
pub fn stub_c6e76c(obj: &crate::movable::MovableObject) -> Option<usize> {
    obj.listener()
}

// 0xc6e774 — __ZNK4Ogre13MovableObject12getLightMaskEv
#[doc(alias = "Ogre::MovableObject::getLightMask(void)const")]
// was: Ogre::MovableObject::getLightMask(void)const
// IDA 0xc6e774: LDR.W R0,[R0,#0xB8], BX LR — u32 field load; recovered: &MovableObject -> u32.
pub fn stub_c6e774(obj: &crate::movable::MovableObject) -> u32 {
    obj.light_mask()
}

// 0xc6e77c — __ZN4Ogre13MovableObject13_getLightListEv
#[doc(alias = "Ogre::MovableObject::_getLightList(void)")]
// was: Ogre::MovableObject::_getLightList(void)
// IDA 0xc6e77c: ADDS R0,#0x9C, BX LR — returns embedded light-list head pointer; recovered: &MovableObject -> &[u32].
pub fn stub_c6e77c(obj: &crate::movable::MovableObject) -> &[u32] {
    obj.light_list()
}

// 0xc6e780 — __ZN4Ogre13MovableObject22setDebugDisplayEnabledEb
#[doc(alias = "Ogre::MovableObject::setDebugDisplayEnabled(bool)")]
// was: Ogre::MovableObject::setDebugDisplayEnabled(bool)
// IDA 0xc6e780: STRB R1,[R0,#0x1A], BX LR — bool field store; recovered: &mut MovableObject, bool.
pub fn stub_c6e780(obj: &mut crate::movable::MovableObject, enabled: bool) {
    obj.set_debug_display_enabled(enabled)
}

// 0xc6e784 — __ZNK4Ogre13MovableObject21isDebugDisplayEnabledEv
#[doc(alias = "Ogre::MovableObject::isDebugDisplayEnabled(void)const")]
// was: Ogre::MovableObject::isDebugDisplayEnabled(void)const
// IDA 0xc6e784: LDRB R0,[R0,#0x1A], BX LR — bool field load; recovered: &MovableObject -> bool.
pub fn stub_c6e784(obj: &crate::movable::MovableObject) -> bool {
    obj.is_debug_display_enabled()
}

// 0xc6e788 — __ZNK4Ogre7Frustum25isCustomViewMatrixEnabledEv
#[doc(alias = "Ogre::Frustum::isCustomViewMatrixEnabled(void)const")]
// was: Ogre::Frustum::isCustomViewMatrixEnabled(void)const
// IDA 0xc6e788: LDRB.W R0,[R0,#0x291], BX LR — Frustum bool field load; recovered: &Frustum -> bool.
pub fn stub_c6e788(obj: &crate::movable::Frustum) -> bool {
    obj.is_custom_view_matrix_enabled()
}

// 0xc6e790 — __ZNK4Ogre7Frustum31isCustomProjectionMatrixEnabledEv
#[doc(alias = "Ogre::Frustum::isCustomProjectionMatrixEnabled(void)const")]
// was: Ogre::Frustum::isCustomProjectionMatrixEnabled(void)const
// IDA 0xc6e790: LDRB.W R0,[R0,#0x292], BX LR — Frustum bool field load; recovered: &Frustum -> bool.
pub fn stub_c6e790(obj: &crate::movable::Frustum) -> bool {
    obj.is_custom_projection_matrix_enabled()
}

// 0xc6e798 — __ZNK4Ogre7Frustum11isReflectedEv
#[doc(alias = "Ogre::Frustum::isReflected(void)const")]
// was: Ogre::Frustum::isReflected(void)const
// IDA 0xc6e798: LDRB.W R0,[R0,#0x374], BX LR — Frustum bool field load; recovered: &Frustum -> bool.
pub fn stub_c6e798(obj: &crate::movable::Frustum) -> bool {
    obj.is_reflected()
}

// 0xc6e7a0 — __ZNK4Ogre7Frustum19getReflectionMatrixEv
#[doc(alias = "Ogre::Frustum::getReflectionMatrix(void)const")]
// was: Ogre::Frustum::getReflectionMatrix(void)const
// IDA 0xc6e7a0: ADD.W R0,R0,#0x378, BX LR — returns embedded Matrix4; recovered: &Frustum -> &[f32; 16].
pub fn stub_c6e7a0(obj: &crate::movable::Frustum) -> &[f32; 16] {
    obj.reflection_matrix()
}

// 0xc6e7a8 — __ZNK4Ogre7Frustum18getReflectionPlaneEv
#[doc(alias = "Ogre::Frustum::getReflectionPlane(void)const")]
// was: Ogre::Frustum::getReflectionPlane(void)const
// IDA 0xc6e7a8: ADD.W R0,R0,#0x3B8, BX LR — returns embedded Plane; recovered: &Frustum -> &[f32; 4].
pub fn stub_c6e7a8(obj: &crate::movable::Frustum) -> &[f32; 4] {
    obj.reflection_plane()
}

// 0xc6e7b0 — __ZNK4Ogre7Frustum28isCustomNearClipPlaneEnabledEv
#[doc(alias = "Ogre::Frustum::isCustomNearClipPlaneEnabled(void)const")]
// was: Ogre::Frustum::isCustomNearClipPlaneEnabled(void)const
// IDA 0xc6e7b0: LDRB.W R0,[R0,#0x3DC], BX LR — Frustum bool field load; recovered: &Frustum -> bool.
pub fn stub_c6e7b0(obj: &crate::movable::Frustum) -> bool {
    obj.is_custom_near_clip_plane_enabled()
}

// 0xc6e7b8 — __ZNK4Ogre6Camera11isWindowSetEv
#[doc(alias = "Ogre::Camera::isWindowSet(void)const")]
// was: Ogre::Camera::isWindowSet(void)const
// IDA 0xc6e7b8: LDRB.W R0,[R0,#0x4A0], BX LR — Camera bool field load; recovered: &Camera -> bool.
pub fn stub_c6e7b8(obj: &crate::movable::Camera) -> bool {
    obj.is_window_set()
}

// 0xc6e7c0 — __ZN4Ogre6Camera23setUseRenderingDistanceEb
#[doc(alias = "Ogre::Camera::setUseRenderingDistance(bool)")]
// was: Ogre::Camera::setUseRenderingDistance(bool)
// IDA 0xc6e7c0: STRB.W R1,[R0,#0x4C4], BX LR — Camera bool field store; recovered: &mut Camera, bool.
pub fn stub_c6e7c0(obj: &mut crate::movable::Camera, use_it: bool) {
    obj.set_use_rendering_distance(use_it)
}

// 0xc6e7c8 — __ZNK4Ogre6Camera23getUseRenderingDistanceEv
#[doc(alias = "Ogre::Camera::getUseRenderingDistance(void)const")]
// was: Ogre::Camera::getUseRenderingDistance(void)const
// IDA 0xc6e7c8: LDRB.W R0,[R0,#0x4C4], BX LR — Camera bool field load; recovered: &Camera -> bool.
pub fn stub_c6e7c8(obj: &crate::movable::Camera) -> bool {
    obj.use_rendering_distance()
}

// 0xc6e7d0 — __ZNK4Ogre14AnimableObject25getAnimableDictionaryNameEv
#[doc(alias = "Ogre::AnimableObject::getAnimableDictionaryName(void)const")]
// was: Ogre::AnimableObject::getAnimableDictionaryName(void)const
// IDA 0xc6e7d0: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6e7d0() {
}

// 0xc6e7e0 — __ZNK4Ogre14AnimableObject28initialiseAnimableDictionaryERSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(
    alias = "Ogre::AnimableObject::initialiseAnimableDictionary(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const"
)]
// was: Ogre::AnimableObject::initialiseAnimableDictionary(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const
// IDA 0xc6e7e0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c6e7e0() {
}

// 0xc6e7e4 — __ZN4Ogre14AnimableObject19createAnimableValueERKSs
#[doc(alias = "Ogre::AnimableObject::createAnimableValue(std::string const&)")]
// was: Ogre::AnimableObject::createAnimableValue(std::string const&)
// IDA 0xc6e7e4: 236 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6e7e4() {
}

// 0xc6ea98 — __ZN4Ogre10Renderable9preRenderEPNS_12SceneManagerEPNS_12RenderSystemE
#[doc(alias = "Ogre::Renderable::preRender(Ogre::SceneManager *,Ogre::RenderSystem *)")]
// was: Ogre::Renderable::preRender(Ogre::SceneManager *,Ogre::RenderSystem *)
// IDA 0xc6ea98: MOVS R0,#1, BX LR — default Renderable::preRender returns true; recovered: &mut Renderable -> bool.
pub fn stub_c6ea98(obj: &mut crate::movable::Renderable) -> bool {
    obj.pre_render()
}

// 0xc6eaa0 — __ZNK4Ogre10Renderable21getNumWorldTransformsEv
#[doc(alias = "Ogre::Renderable::getNumWorldTransforms(void)const")]
// was: Ogre::Renderable::getNumWorldTransforms(void)const
// IDA 0xc6eaa0: MOVS R0,#1, BX LR — base Renderable has one world transform; recovered: &Renderable -> u16.
pub fn stub_c6eaa0(obj: &crate::movable::Renderable) -> u16 {
    obj.num_world_transforms()
}

// 0xc6eaa8 — __ZNK4Ogre10Renderable25_updateCustomGpuParameterERKNS_20GpuProgramParameters17AutoConstantEntryEPS1_
#[doc(
    alias = "Ogre::Renderable::_updateCustomGpuParameter(Ogre::GpuProgramParameters::AutoConstantEntry const&,Ogre::GpuProgramParameters*)const"
)]
// was: Ogre::Renderable::_updateCustomGpuParameter(Ogre::GpuProgramParameters::AutoConstantEntry const&,Ogre::GpuProgramParameters*)const
// IDA 0xc6eaa8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6eaa8() {
}

// 0xc6eb08 — __ZNK4Ogre10Renderable10getUserAnyEv
#[doc(alias = "Ogre::Renderable::getUserAny(void)const")]
// was: Ogre::Renderable::getUserAny(void)const
// IDA 0xc6eb08: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6eb08() {
}

// 0xc6eb18 — __ZNK4Ogre10Renderable19setRenderSystemDataEPNS0_16RenderSystemDataE
#[doc(alias = "Ogre::Renderable::setRenderSystemData(Ogre::Renderable::RenderSystemData *)const")]
// was: Ogre::Renderable::setRenderSystemData(Ogre::Renderable::RenderSystemData *)const
// IDA 0xc6eb18: STR R1,[R0,#0x2C], BX LR — RenderSystemData pointer store (const setter mutates mutable cell); recovered: &mut Renderable, Option<usize>.
pub fn stub_c6eb18(obj: &mut crate::movable::Renderable, data: Option<usize>) {
    obj.set_render_system_data(data)
}

// 0xc6eb1c — __ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(
    alias = "std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector4*,std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Vector4 const&)"
)]
// was: std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector4*,std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Vector4 const&)
// IDA 0xc6eb1c: 115 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6eb1c() {
}

// 0xc6ec68 — __ZNSt12_Vector_baseIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc6ec68: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c6ec68() {
}

// 0xc6ec6c — __ZNSt12_Vector_baseIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc6ec6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c6ec6c() {
}

// 0xc6ec78 — __ZNSt6vectorIN4Ogre5PlaneENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(
    alias = "std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Plane*,std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Plane const&)"
)]
// was: std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Plane*,std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Plane const&)
// IDA 0xc6ec78: 246 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6ec78() {
}

// 0xc6eef4 — __ZNSt6vectorIPN4Ogre6Camera8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(
    alias = "std::vector<Ogre::Camera::Listener *,Ogre::STLAllocator<Ogre::Camera::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Camera::Listener **,std::vector<Ogre::Camera::Listener *,Ogre::STLAllocator<Ogre::Camera::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Camera::Listener * const&)"
)]
// was: std::vector<Ogre::Camera::Listener *,Ogre::STLAllocator<Ogre::Camera::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Camera::Listener **,std::vector<Ogre::Camera::Listener *,Ogre::STLAllocator<Ogre::Camera::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Camera::Listener * const&)
// IDA 0xc6eef4: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6eef4() {
}

// 0xc6efec — __ZNSt12_Vector_baseIPN4Ogre6Camera8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::Camera::Listener *,Ogre::STLAllocator<Ogre::Camera::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::Camera::Listener *,Ogre::STLAllocator<Ogre::Camera::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc6efec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c6efec() {
}

// 0xc6eff0 — __ZNSt12_Vector_baseIPN4Ogre6Camera8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::Camera::Listener *,Ogre::STLAllocator<Ogre::Camera::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::Camera::Listener *,Ogre::STLAllocator<Ogre::Camera::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc6eff0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c6eff0() {
}

// 0xc6f000 — __ZNSt12_Vector_baseIN4Ogre5PlaneENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc6f000: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c6f000() {
}

// 0xc6f010 — __ZN4Ogre9ExceptionD2Ev
#[doc(alias = "Ogre::Exception::~Exception()")]
// was: Ogre::Exception::~Exception()
// IDA 0xc6f010: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c6f010() {
}

// 0xc6f148 — __ZN4Ogre9ExceptionD0Ev
#[doc(alias = "Ogre::Exception::~Exception()")]
// was: Ogre::Exception::~Exception()
// IDA 0xc6f148: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c6f148() {
}

// 0xc6f160 — __ZNK4Ogre9Exception9getSourceEv
#[doc(alias = "Ogre::Exception::getSource(void)const")]
// was: Ogre::Exception::getSource(void)const
// IDA 0xc6f160: ADDS R0,#0x14, BX LR — returns embedded source string; recovered: &OgreException -> &str.
pub fn stub_c6f160(obj: &crate::movable::OgreException) -> &str {
    obj.source()
}

// 0xc6f168 — __ZNK4Ogre9Exception7getLineEv
#[doc(alias = "Ogre::Exception::getLine(void)const")]
// was: Ogre::Exception::getLine(void)const
// IDA 0xc6f168: LDR R0,[R0,#4], BX LR — i32 field load; recovered: &OgreException -> i32.
pub fn stub_c6f168(obj: &crate::movable::OgreException) -> i32 {
    obj.line()
}

// 0xc6f170 — __ZN4Ogre21ItemIdentityExceptionD0Ev
#[doc(alias = "Ogre::ItemIdentityException::~ItemIdentityException()")]
// was: Ogre::ItemIdentityException::~ItemIdentityException()
// IDA 0xc6f170: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c6f170() {
}

// 0xc6f1f0 — __ZN4Ogre5CodecD2Ev
#[doc(alias = "Ogre::Codec::~Codec()")]
// was: Ogre::Codec::~Codec()
// IDA 0xc6f1f0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c6f1f0() {
}

// 0xc6f1f4 — __ZN4Ogre5Codec13getExtensionsEv
#[doc(alias = "Ogre::Codec::getExtensions(void)")]
// was: Ogre::Codec::getExtensions(void)
// IDA 0xc6f1f4: 156 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6f1f4() {
}

// 0xc6f3a0 — __ZN4Ogre5Codec8getCodecERKSs
#[doc(alias = "Ogre::Codec::getCodec(std::string const&)")]
// was: Ogre::Codec::getCodec(std::string const&)
// IDA 0xc6f3a0: 722 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6f3a0() {
}

// 0xc6fbcc — __ZN4Ogre5Codec8getCodecEPcm
#[doc(alias = "Ogre::Codec::getCodec(char *,unsigned long)")]
// was: Ogre::Codec::getCodec(char *,unsigned long)
// IDA 0xc6fbcc: 184 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6fbcc() {
}

// 0xc6fdc8 — __ZNSt3mapISsPN4Ogre5CodecESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(
    alias = "std::map<std::string,Ogre::Codec *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Codec *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~map()"
)]
// was: std::map<std::string,Ogre::Codec *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Codec *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~map()
// IDA 0xc6fdc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c6fdc8() {
}

// 0xc6fe5c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre5CodecEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Codec *>,std::_Select1st<std::pair<std::string const,Ogre::Codec *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Codec *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Codec *>,std::_Select1st<std::pair<std::string const,Ogre::Codec *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Codec *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc6fe5c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c6fe5c() {
}

// 0xc6fe60 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre5CodecEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Codec *>,std::_Select1st<std::pair<std::string const,Ogre::Codec *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Codec *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Codec *>,std::_Select1st<std::pair<std::string const,Ogre::Codec *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Codec *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc6fe60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c6fe60() {
}

// 0xc6fee4 — __ZNK4Ogre11ColourValue9getAsRGBAEv
#[doc(alias = "Ogre::ColourValue::getAsRGBA(void)const")]
// was: Ogre::ColourValue::getAsRGBA(void)const
// IDA 0xc6fee4: 22 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6fee4() {
}

// 0xc6ff3c — __ZNK4Ogre11ColourValue9getAsARGBEv
#[doc(alias = "Ogre::ColourValue::getAsARGB(void)const")]
// was: Ogre::ColourValue::getAsARGB(void)const
// IDA 0xc6ff3c: 22 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6ff3c() {
}

// 0xc6ff94 — __ZNK4Ogre11ColourValue9getAsABGREv
#[doc(alias = "Ogre::ColourValue::getAsABGR(void)const")]
// was: Ogre::ColourValue::getAsABGR(void)const
// IDA 0xc6ff94: 22 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6ff94() {
}

// 0xc6ffec — __ZNK4Ogre11ColourValueeqERKS0_
#[doc(alias = "Ogre::ColourValue::operator==(Ogre::ColourValue const&)const")]
// was: Ogre::ColourValue::operator==(Ogre::ColourValue const&)const
// IDA 0xc6ffec: 26 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6ffec() {
}

// 0xc70040 — __ZNK4Ogre11ColourValueneERKS0_
#[doc(alias = "Ogre::ColourValue::operator!=(Ogre::ColourValue const&)const")]
// was: Ogre::ColourValue::operator!=(Ogre::ColourValue const&)const
// IDA 0xc70040: 27 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70040() {
}

// 0xc70150 — __ZN4Ogre8FastHashEPKcij
#[doc(alias = "Ogre::FastHash(char const*,int,unsigned int)")]
// was: Ogre::FastHash(char const*,int,unsigned int)
// IDA 0xc70150: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70150() {
}

// 0xc70228 — __ZN4Ogre15CompositionPassC1EPNS_21CompositionTargetPassE
#[doc(alias = "Ogre::CompositionPass::CompositionPass(Ogre::CompositionTargetPass *)")]
// was: Ogre::CompositionPass::CompositionPass(Ogre::CompositionTargetPass *)
// IDA 0xc70228: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70228() {
}

// 0xc70234 — __ZN4Ogre15CompositionPassC2EPNS_21CompositionTargetPassE
#[doc(alias = "Ogre::CompositionPass::CompositionPass(Ogre::CompositionTargetPass *)")]
// was: Ogre::CompositionPass::CompositionPass(Ogre::CompositionTargetPass *)
// IDA 0xc70234: 267 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70234() {
}

// 0xc70504 — __ZN4Ogre15CompositionPassD1Ev
#[doc(alias = "Ogre::CompositionPass::~CompositionPass()")]
// was: Ogre::CompositionPass::~CompositionPass()
// IDA 0xc70504: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c70504() {
}

// 0xc70510 — __ZN4Ogre15CompositionPassD2Ev
#[doc(alias = "Ogre::CompositionPass::~CompositionPass()")]
// was: Ogre::CompositionPass::~CompositionPass()
// IDA 0xc70510: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c70510() {
}

// 0xc706dc — __ZN4Ogre15CompositionPass7setTypeENS0_8PassTypeE
#[doc(alias = "Ogre::CompositionPass::setType(Ogre::CompositionPass::PassType)")]
// was: Ogre::CompositionPass::setType(Ogre::CompositionPass::PassType)
// IDA 0xc706dc: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c706dc() {
}

// 0xc706e0 — __ZN4Ogre15CompositionPass13setIdentifierEj
#[doc(alias = "Ogre::CompositionPass::setIdentifier(unsigned int)")]
// was: Ogre::CompositionPass::setIdentifier(unsigned int)
// IDA 0xc706e0: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c706e0() {
}

// 0xc706e4 — __ZN4Ogre15CompositionPass15setMaterialNameERKSs
#[doc(alias = "Ogre::CompositionPass::setMaterialName(std::string const&)")]
// was: Ogre::CompositionPass::setMaterialName(std::string const&)
// IDA 0xc706e4: 183 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c706e4() {
}

// 0xc708b8 — __ZN4Ogre15CompositionPass15setClearBuffersEj
#[doc(alias = "Ogre::CompositionPass::setClearBuffers(unsigned int)")]
// was: Ogre::CompositionPass::setClearBuffers(unsigned int)
// IDA 0xc708b8: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c708b8() {
}

// 0xc708bc — __ZN4Ogre15CompositionPass14setClearColourENS_11ColourValueE
#[doc(alias = "Ogre::CompositionPass::setClearColour(Ogre::ColourValue)")]
// was: Ogre::CompositionPass::setClearColour(Ogre::ColourValue)
// IDA 0xc708bc: 4 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c708bc() {
}

// 0xc708cc — __ZN4Ogre15CompositionPass8setInputEmRKSsm
#[doc(alias = "Ogre::CompositionPass::setInput(unsigned long,std::string const&,unsigned long)")]
// was: Ogre::CompositionPass::setInput(unsigned long,std::string const&,unsigned long)
// IDA 0xc708cc: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c708cc() {
}

// 0xc709fc — __ZN4Ogre15CompositionPass19setFirstRenderQueueEh
#[doc(alias = "Ogre::CompositionPass::setFirstRenderQueue(unsigned char)")]
// was: Ogre::CompositionPass::setFirstRenderQueue(unsigned char)
// IDA 0xc709fc: 2 insns (STRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c709fc() {
}

// 0xc70a00 — __ZN4Ogre15CompositionPass18setLastRenderQueueEh
#[doc(alias = "Ogre::CompositionPass::setLastRenderQueue(unsigned char)")]
// was: Ogre::CompositionPass::setLastRenderQueue(unsigned char)
// IDA 0xc70a00: 2 insns (STRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a00() {
}

// 0xc70a04 — __ZN4Ogre15CompositionPass17setMaterialSchemeERKSs
#[doc(alias = "Ogre::CompositionPass::setMaterialScheme(std::string const&)")]
// was: Ogre::CompositionPass::setMaterialScheme(std::string const&)
// IDA 0xc70a04: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a04() {
}

// 0xc70a10 — __ZN4Ogre15CompositionPass13setClearDepthEf
#[doc(alias = "Ogre::CompositionPass::setClearDepth(float)")]
// was: Ogre::CompositionPass::setClearDepth(float)
// IDA 0xc70a10: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a10() {
}

// 0xc70a14 — __ZN4Ogre15CompositionPass15setClearStencilEj
#[doc(alias = "Ogre::CompositionPass::setClearStencil(unsigned int)")]
// was: Ogre::CompositionPass::setClearStencil(unsigned int)
// IDA 0xc70a14: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a14() {
}

// 0xc70a18 — __ZN4Ogre15CompositionPass15setStencilCheckEb
#[doc(alias = "Ogre::CompositionPass::setStencilCheck(bool)")]
// was: Ogre::CompositionPass::setStencilCheck(bool)
// IDA 0xc70a18: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a18() {
}

// 0xc70a20 — __ZN4Ogre15CompositionPass14setStencilFuncENS_15CompareFunctionE
#[doc(alias = "Ogre::CompositionPass::setStencilFunc(Ogre::CompareFunction)")]
// was: Ogre::CompositionPass::setStencilFunc(Ogre::CompareFunction)
// IDA 0xc70a20: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a20() {
}

// 0xc70a28 — __ZN4Ogre15CompositionPass18setStencilRefValueEj
#[doc(alias = "Ogre::CompositionPass::setStencilRefValue(unsigned int)")]
// was: Ogre::CompositionPass::setStencilRefValue(unsigned int)
// IDA 0xc70a28: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a28() {
}

// 0xc70a30 — __ZN4Ogre15CompositionPass14setStencilMaskEj
#[doc(alias = "Ogre::CompositionPass::setStencilMask(unsigned int)")]
// was: Ogre::CompositionPass::setStencilMask(unsigned int)
// IDA 0xc70a30: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a30() {
}

// 0xc70a38 — __ZN4Ogre15CompositionPass16setStencilFailOpENS_16StencilOperationE
#[doc(alias = "Ogre::CompositionPass::setStencilFailOp(Ogre::StencilOperation)")]
// was: Ogre::CompositionPass::setStencilFailOp(Ogre::StencilOperation)
// IDA 0xc70a38: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a38() {
}

// 0xc70a40 — __ZN4Ogre15CompositionPass21setStencilDepthFailOpENS_16StencilOperationE
#[doc(alias = "Ogre::CompositionPass::setStencilDepthFailOp(Ogre::StencilOperation)")]
// was: Ogre::CompositionPass::setStencilDepthFailOp(Ogre::StencilOperation)
// IDA 0xc70a40: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a40() {
}

// 0xc70a48 — __ZN4Ogre15CompositionPass16setStencilPassOpENS_16StencilOperationE
#[doc(alias = "Ogre::CompositionPass::setStencilPassOp(Ogre::StencilOperation)")]
// was: Ogre::CompositionPass::setStencilPassOp(Ogre::StencilOperation)
// IDA 0xc70a48: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a48() {
}

// 0xc70a50 — __ZN4Ogre15CompositionPass27setStencilTwoSidedOperationEb
#[doc(alias = "Ogre::CompositionPass::setStencilTwoSidedOperation(bool)")]
// was: Ogre::CompositionPass::setStencilTwoSidedOperation(bool)
// IDA 0xc70a50: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a50() {
}

// 0xc70a58 — __ZN4Ogre15CompositionPass17setQuadFarCornersEbb
#[doc(alias = "Ogre::CompositionPass::setQuadFarCorners(bool,bool)")]
// was: Ogre::CompositionPass::setQuadFarCorners(bool,bool)
// IDA 0xc70a58: 3 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a58() {
}

// 0xc70a64 — __ZN4Ogre15CompositionPass13setCustomTypeERKSs
#[doc(alias = "Ogre::CompositionPass::setCustomType(std::string const&)")]
// was: Ogre::CompositionPass::setCustomType(std::string const&)
// IDA 0xc70a64: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a64() {
}

// 0xc70a70 — __ZN4Ogre15CompositionPass12_isSupportedEv
#[doc(alias = "Ogre::CompositionPass::_isSupported(void)")]
// was: Ogre::CompositionPass::_isSupported(void)
// IDA 0xc70a70: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a70() {
}

// 0xc70ad8 — __ZN4Ogre21CompositionTargetPassC1EPNS_20CompositionTechniqueE
#[doc(alias = "Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")]
// was: Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)
// IDA 0xc70ad8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70ad8() {
}

// 0xc70ae4 — __ZN4Ogre21CompositionTargetPassC2EPNS_20CompositionTechniqueE
#[doc(alias = "Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")]
// was: Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)
// IDA 0xc70ae4: 168 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70ae4() {
}

// 0xc70cb8 — __ZN4Ogre21CompositionTargetPassD1Ev
#[doc(alias = "Ogre::CompositionTargetPass::~CompositionTargetPass()")]
// was: Ogre::CompositionTargetPass::~CompositionTargetPass()
// IDA 0xc70cb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c70cb8() {
}

// 0xc70cc4 — __ZN4Ogre21CompositionTargetPassD2Ev
#[doc(alias = "Ogre::CompositionTargetPass::~CompositionTargetPass()")]
// was: Ogre::CompositionTargetPass::~CompositionTargetPass()
// IDA 0xc70cc4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c70cc4() {
}

// 0xc70e08 — __ZN4Ogre21CompositionTargetPass12setInputModeENS0_9InputModeE
#[doc(alias = "Ogre::CompositionTargetPass::setInputMode(Ogre::CompositionTargetPass::InputMode)")]
// was: Ogre::CompositionTargetPass::setInputMode(Ogre::CompositionTargetPass::InputMode)
// IDA 0xc70e08: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e08() {
}

// 0xc70e0c — __ZN4Ogre21CompositionTargetPass13setOutputNameERKSs
#[doc(alias = "Ogre::CompositionTargetPass::setOutputName(std::string const&)")]
// was: Ogre::CompositionTargetPass::setOutputName(std::string const&)
// IDA 0xc70e0c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e0c() {
}

// 0xc70e18 — __ZN4Ogre21CompositionTargetPass14setOnlyInitialEb
#[doc(alias = "Ogre::CompositionTargetPass::setOnlyInitial(bool)")]
// was: Ogre::CompositionTargetPass::setOnlyInitial(bool)
// IDA 0xc70e18: 2 insns (STRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e18() {
}

// 0xc70e1c — __ZN4Ogre21CompositionTargetPass17setVisibilityMaskEj
#[doc(alias = "Ogre::CompositionTargetPass::setVisibilityMask(unsigned int)")]
// was: Ogre::CompositionTargetPass::setVisibilityMask(unsigned int)
// IDA 0xc70e1c: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e1c() {
}

// 0xc70e20 — __ZN4Ogre21CompositionTargetPass10setLodBiasEf
#[doc(alias = "Ogre::CompositionTargetPass::setLodBias(float)")]
// was: Ogre::CompositionTargetPass::setLodBias(float)
// IDA 0xc70e20: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e20() {
}

// 0xc70e24 — __ZN4Ogre21CompositionTargetPass17setMaterialSchemeERKSs
#[doc(alias = "Ogre::CompositionTargetPass::setMaterialScheme(std::string const&)")]
// was: Ogre::CompositionTargetPass::setMaterialScheme(std::string const&)
// IDA 0xc70e24: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e24() {
}

// 0xc70e30 — __ZN4Ogre21CompositionTargetPass17setShadowsEnabledEb
#[doc(alias = "Ogre::CompositionTargetPass::setShadowsEnabled(bool)")]
// was: Ogre::CompositionTargetPass::setShadowsEnabled(bool)
// IDA 0xc70e30: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e30() {
}

// 0xc70e38 — __ZN4Ogre21CompositionTargetPass10createPassEv
#[doc(alias = "Ogre::CompositionTargetPass::createPass(void)")]
// was: Ogre::CompositionTargetPass::createPass(void)
// IDA 0xc70e38: 91 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e38() {
}

// 0xc70f2c — __ZN4Ogre21CompositionTargetPass12_isSupportedEv
#[doc(alias = "Ogre::CompositionTargetPass::_isSupported(void)")]
// was: Ogre::CompositionTargetPass::_isSupported(void)
// IDA 0xc70f2c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70f2c() {
}

// 0xc70f4c — __ZNSt6vectorIPN4Ogre15CompositionPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(
    alias = "std::vector<Ogre::CompositionPass *,Ogre::STLAllocator<Ogre::CompositionPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::CompositionPass **,std::vector<Ogre::CompositionPass *,Ogre::STLAllocator<Ogre::CompositionPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CompositionPass * const&)"
)]
// was: std::vector<Ogre::CompositionPass *,Ogre::STLAllocator<Ogre::CompositionPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::CompositionPass **,std::vector<Ogre::CompositionPass *,Ogre::STLAllocator<Ogre::CompositionPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CompositionPass * const&)
// IDA 0xc70f4c: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70f4c() {
}

// 0xc71044 — __ZNSt12_Vector_baseIPN4Ogre15CompositionPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::CompositionPass *,Ogre::STLAllocator<Ogre::CompositionPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::CompositionPass *,Ogre::STLAllocator<Ogre::CompositionPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc71044: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c71044() {
}

// 0xc71048 — __ZNSt12_Vector_baseIPN4Ogre15CompositionPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::CompositionPass *,Ogre::STLAllocator<Ogre::CompositionPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::CompositionPass *,Ogre::STLAllocator<Ogre::CompositionPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc71048: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c71048() {
}

// 0xc71088 — __ZN4Ogre20CompositionTechniqueC1EPNS_10CompositorE
#[doc(alias = "Ogre::CompositionTechnique::CompositionTechnique(Ogre::Compositor *)")]
// was: Ogre::CompositionTechnique::CompositionTechnique(Ogre::Compositor *)
// IDA 0xc71088: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c71088() {
}

// 0xc71094 — __ZN4Ogre20CompositionTechniqueC2EPNS_10CompositorE
#[doc(alias = "Ogre::CompositionTechnique::CompositionTechnique(Ogre::Compositor *)")]
// was: Ogre::CompositionTechnique::CompositionTechnique(Ogre::Compositor *)
// IDA 0xc71094: 168 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c71094() {
}

// 0xc71260 — __ZN4Ogre20CompositionTechniqueD0Ev
#[doc(alias = "Ogre::CompositionTechnique::~CompositionTechnique()")]
// was: Ogre::CompositionTechnique::~CompositionTechnique()
// IDA 0xc71260: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c71260() {
}

// 0xc712f0 — __ZN4Ogre20CompositionTechniqueD1Ev
#[doc(alias = "Ogre::CompositionTechnique::~CompositionTechnique()")]
// was: Ogre::CompositionTechnique::~CompositionTechnique()
// IDA 0xc712f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c712f0() {
}

// 0xc712fc — __ZN4Ogre20CompositionTechniqueD2Ev
#[doc(alias = "Ogre::CompositionTechnique::~CompositionTechnique()")]
// was: Ogre::CompositionTechnique::~CompositionTechnique()
// IDA 0xc712fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c712fc() {
}

// 0xc71474 — __ZN4Ogre20CompositionTechnique27removeAllTextureDefinitionsEv
#[doc(alias = "Ogre::CompositionTechnique::removeAllTextureDefinitions(void)")]
// was: Ogre::CompositionTechnique::removeAllTextureDefinitions(void)
// IDA 0xc71474: 131 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c71474() {
}

// 0xc715e0 — __ZN4Ogre20CompositionTechnique23createTextureDefinitionERKSs
#[doc(alias = "Ogre::CompositionTechnique::createTextureDefinition(std::string const&)")]
// was: Ogre::CompositionTechnique::createTextureDefinition(std::string const&)
// IDA 0xc715e0: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c715e0() {
}

// 0xc71688 — __ZN4Ogre20CompositionTechnique28getTextureDefinitionIteratorEv
#[doc(alias = "Ogre::CompositionTechnique::getTextureDefinitionIterator(void)")]
// was: Ogre::CompositionTechnique::getTextureDefinitionIterator(void)
// IDA 0xc71688: 6 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c71688() {
}

// 0xc71694 — __ZN4Ogre20CompositionTechnique16createTargetPassEv
#[doc(alias = "Ogre::CompositionTechnique::createTargetPass(void)")]
// was: Ogre::CompositionTechnique::createTargetPass(void)
// IDA 0xc71694: 91 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c71694() {
}

// 0xc71788 — __ZN4Ogre20CompositionTechnique19getOutputTargetPassEv
#[doc(alias = "Ogre::CompositionTechnique::getOutputTargetPass(void)")]
// was: Ogre::CompositionTechnique::getOutputTargetPass(void)
// IDA 0xc71788: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c71788() {
}

// 0xc7178c — __ZN4Ogre20CompositionTechnique11isSupportedEb
#[doc(alias = "Ogre::CompositionTechnique::isSupported(bool)")]
// was: Ogre::CompositionTechnique::isSupported(bool)
// IDA 0xc7178c: 119 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7178c() {
}

// 0xc718d0 — __ZN4Ogre20CompositionTechnique13setSchemeNameERKSs
#[doc(alias = "Ogre::CompositionTechnique::setSchemeName(std::string const&)")]
// was: Ogre::CompositionTechnique::setSchemeName(std::string const&)
// IDA 0xc718d0: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c718d0() {
}

// 0xc718dc — __ZNSt6vectorIPN4Ogre21CompositionTargetPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(
    alias = "std::vector<Ogre::CompositionTargetPass *,Ogre::STLAllocator<Ogre::CompositionTargetPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::CompositionTargetPass **,std::vector<Ogre::CompositionTargetPass *,Ogre::STLAllocator<Ogre::CompositionTargetPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CompositionTargetPass * const&)"
)]
// was: std::vector<Ogre::CompositionTargetPass *,Ogre::STLAllocator<Ogre::CompositionTargetPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::CompositionTargetPass **,std::vector<Ogre::CompositionTargetPass *,Ogre::STLAllocator<Ogre::CompositionTargetPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CompositionTargetPass * const&)
// IDA 0xc718dc: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c718dc() {
}

// 0xc719d4 — __ZNSt6vectorIPN4Ogre20CompositionTechnique17TextureDefinitionENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(
    alias = "std::vector<Ogre::CompositionTechnique::TextureDefinition *,Ogre::STLAllocator<Ogre::CompositionTechnique::TextureDefinition *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::CompositionTechnique::TextureDefinition **,std::vector<Ogre::CompositionTechnique::TextureDefinition *,Ogre::STLAllocator<Ogre::CompositionTechnique::TextureDefinition *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CompositionTechnique::TextureDefinition * const&)"
)]
// was: std::vector<Ogre::CompositionTechnique::TextureDefinition *,Ogre::STLAllocator<Ogre::CompositionTechnique::TextureDefinition *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::CompositionTechnique::TextureDefinition **,std::vector<Ogre::CompositionTechnique::TextureDefinition *,Ogre::STLAllocator<Ogre::CompositionTechnique::TextureDefinition *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CompositionTechnique::TextureDefinition * const&)
// IDA 0xc719d4: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c719d4() {
}

// 0xc71acc — __ZNSt12_Vector_baseIPN4Ogre21CompositionTargetPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::CompositionTargetPass *,Ogre::STLAllocator<Ogre::CompositionTargetPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::CompositionTargetPass *,Ogre::STLAllocator<Ogre::CompositionTargetPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc71acc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c71acc() {
}

// 0xc71ad0 — __ZNSt12_Vector_baseIPN4Ogre20CompositionTechnique17TextureDefinitionENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::CompositionTechnique::TextureDefinition *,Ogre::STLAllocator<Ogre::CompositionTechnique::TextureDefinition *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::CompositionTechnique::TextureDefinition *,Ogre::STLAllocator<Ogre::CompositionTechnique::TextureDefinition *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc71ad0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c71ad0() {
}

// 0xc71ad4 — __ZNSt12_Vector_baseIPN4Ogre21CompositionTargetPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::CompositionTargetPass *,Ogre::STLAllocator<Ogre::CompositionTargetPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::CompositionTargetPass *,Ogre::STLAllocator<Ogre::CompositionTargetPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc71ad4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c71ad4() {
}

// 0xc71ae0 — __ZNSt12_Vector_baseIPN4Ogre20CompositionTechnique17TextureDefinitionENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::CompositionTechnique::TextureDefinition *,Ogre::STLAllocator<Ogre::CompositionTechnique::TextureDefinition *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::CompositionTechnique::TextureDefinition *,Ogre::STLAllocator<Ogre::CompositionTechnique::TextureDefinition *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc71ae0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c71ae0() {
}

// 0xc71aec — __ZNSt12_Vector_baseIN4Ogre11PixelFormatENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc71aec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c71aec() {
}

// 0xc71af0 — __ZNSt12_Vector_baseIN4Ogre11PixelFormatENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc71af0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c71af0() {
}

// 0xc71b30 — __ZN4Ogre10CompositorC1EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(
    alias = "Ogre::Compositor::Compositor(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)"
)]
// was: Ogre::Compositor::Compositor(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xc71b30: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c71b30() {
}

// 0xc71be0 — __ZN4Ogre10CompositorD0Ev
#[doc(alias = "Ogre::Compositor::~Compositor()")]
// was: Ogre::Compositor::~Compositor()
// IDA 0xc71be0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c71be0() {
}

// 0xc71c70 — __ZN4Ogre10CompositorD1Ev
#[doc(alias = "Ogre::Compositor::~Compositor()")]
// was: Ogre::Compositor::~Compositor()
// IDA 0xc71c70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c71c70() {
}

// 0xc71c7c — __ZN4Ogre10CompositorD2Ev
#[doc(alias = "Ogre::Compositor::~Compositor()")]
// was: Ogre::Compositor::~Compositor()
// IDA 0xc71c7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c71c7c() {
}

// 0xc71e04 — __ZN4Ogre10Compositor19removeAllTechniquesEv
#[doc(alias = "Ogre::Compositor::removeAllTechniques(void)")]
// was: Ogre::Compositor::removeAllTechniques(void)
// IDA 0xc71e04: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c71e04() {
}

// 0xc71e34 — __ZN4Ogre10Compositor15createTechniqueEv
#[doc(alias = "Ogre::Compositor::createTechnique(void)")]
// was: Ogre::Compositor::createTechnique(void)
// IDA 0xc71e34: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c71e34() {
}

// 0xc71f30 — __ZN4Ogre10Compositor8loadImplEv
#[doc(alias = "Ogre::Compositor::loadImpl(void)")]
// was: Ogre::Compositor::loadImpl(void)
// IDA 0xc71f30: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c71f30() {
}

// 0xc71f4c — __ZN4Ogre10Compositor7compileEv
#[doc(alias = "Ogre::Compositor::compile(void)")]
// was: Ogre::Compositor::compile(void)
// IDA 0xc71f4c: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c71f4c() {
}

// 0xc71ff0 — __ZN4Ogre10Compositor20createGlobalTexturesEv
#[doc(alias = "Ogre::Compositor::createGlobalTextures(void)")]
// was: Ogre::Compositor::createGlobalTextures(void)
// IDA 0xc71ff0: 2534 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c71ff0() {
}

// 0xc73b0c — __ZN4Ogre10Compositor10unloadImplEv
#[doc(alias = "Ogre::Compositor::unloadImpl(void)")]
// was: Ogre::Compositor::unloadImpl(void)
// IDA 0xc73b0c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c73b0c() {
}

// 0xc73b18 — __ZN4Ogre10Compositor18freeGlobalTexturesEv
#[doc(alias = "Ogre::Compositor::freeGlobalTextures(void)")]
// was: Ogre::Compositor::freeGlobalTextures(void)
// IDA 0xc73b18: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c73b18() {
}

// 0xc73bc4 — __ZNK4Ogre10Compositor13calculateSizeEv
#[doc(alias = "Ogre::Compositor::calculateSize(void)const")]
// was: Ogre::Compositor::calculateSize(void)const
// IDA 0xc73bc4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c73bc4() {
}

// 0xc73bc8 — __ZN4Ogre18getMRTTexLocalNameERKSsm
#[doc(alias = "Ogre::getMRTTexLocalName(std::string const&,unsigned long)")]
// was: Ogre::getMRTTexLocalName(std::string const&,unsigned long)
// IDA 0xc73bc8: 199 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c73bc8() {
}

// 0xc73e08 — __ZNSt3mapISsPN4Ogre17MultiRenderTargetESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(
    alias = "std::map<std::string,Ogre::MultiRenderTarget *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)"
)]
// was: std::map<std::string,Ogre::MultiRenderTarget *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xc73e08: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c73e08() {
}

// 0xc73fc4 — __ZNSt3mapISsN4Ogre10TexturePtrESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
#[doc(
    alias = "std::map<std::string,Ogre::TexturePtr,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)"
)]
// was: std::map<std::string,Ogre::TexturePtr,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xc73fc4: 259 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c73fc4() {
}

// 0xc74270 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xc74270: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c74270() {
}

// 0xc74314 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::MultiRenderTarget *>> *)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::MultiRenderTarget *>> *)
// IDA 0xc74314: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c74314() {
}

// 0xc7438c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::TexturePtr>> *)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::TexturePtr>> *)
// IDA 0xc7438c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7438c() {
}

// 0xc743bc — __ZN4Ogre12STLAllocatorISt4pairIKSsNS_10TexturePtrEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS4_
#[doc(
    alias = "Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::TexturePtr>*)"
)]
// was: Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::TexturePtr>*)
// IDA 0xc743bc: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c743bc() {
}

// 0xc744ec — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::TexturePtr>>,std::pair<std::string const,Ogre::TexturePtr> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::TexturePtr>>,std::pair<std::string const,Ogre::TexturePtr> const&)
// IDA 0xc744ec: 341 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c744ec() {
}

// 0xc74834 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::TexturePtr> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::TexturePtr> const&)
// IDA 0xc74834: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c74834() {
}

// 0xc748a8 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::TexturePtr> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::TexturePtr> const&)
// IDA 0xc748a8: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c748a8() {
}

// 0xc7498c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS4_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::TexturePtr> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::TexturePtr> const&)
// IDA 0xc7498c: 112 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7498c() {
}

// 0xc74ab8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::pair<std::string const,Ogre::MultiRenderTarget *> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::pair<std::string const,Ogre::MultiRenderTarget *> const&)
// IDA 0xc74ab8: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c74ab8() {
}

// 0xc74c98 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::MultiRenderTarget *> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::MultiRenderTarget *> const&)
// IDA 0xc74c98: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c74c98() {
}

// 0xc74dec — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::MultiRenderTarget *> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::MultiRenderTarget *> const&)
// IDA 0xc74dec: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c74dec() {
}

// 0xc74ed0 — __ZNSt6vectorIPN4Ogre20CompositionTechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(
    alias = "std::vector<Ogre::CompositionTechnique *,Ogre::STLAllocator<Ogre::CompositionTechnique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::CompositionTechnique **,std::vector<Ogre::CompositionTechnique *,Ogre::STLAllocator<Ogre::CompositionTechnique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CompositionTechnique * const&)"
)]
// was: std::vector<Ogre::CompositionTechnique *,Ogre::STLAllocator<Ogre::CompositionTechnique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::CompositionTechnique **,std::vector<Ogre::CompositionTechnique *,Ogre::STLAllocator<Ogre::CompositionTechnique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CompositionTechnique * const&)
// IDA 0xc74ed0: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c74ed0() {
}

// 0xc74fc8 — __ZNSt12_Vector_baseIPN4Ogre20CompositionTechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::CompositionTechnique *,Ogre::STLAllocator<Ogre::CompositionTechnique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::CompositionTechnique *,Ogre::STLAllocator<Ogre::CompositionTechnique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc74fc8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c74fc8() {
}

// 0xc74fcc — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc74fcc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c74fcc() {
}

// 0xc74fd0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17MultiRenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MultiRenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::MultiRenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MultiRenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc74fd0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c74fd0() {
}

// 0xc74fdc — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc74fdc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c74fdc() {
}

// 0xc74fe0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre10TexturePtrEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::TexturePtr>,std::_Select1st<std::pair<std::string const,Ogre::TexturePtr>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc74fe0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c74fe0() {
}

// 0xc74fec — __ZNSt12_Vector_baseIPN4Ogre20CompositionTechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::CompositionTechnique *,Ogre::STLAllocator<Ogre::CompositionTechnique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::CompositionTechnique *,Ogre::STLAllocator<Ogre::CompositionTechnique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc74fec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c74fec() {
}

// 0xc7502c — __ZN4Ogre15CompositorChain13getCompositorERKSs
#[doc(alias = "Ogre::CompositorChain::getCompositor(std::string const&)")]
// was: Ogre::CompositorChain::getCompositor(std::string const&)
// IDA 0xc7502c: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7502c() {
}

// 0xc75094 — __ZN4Ogre20RenderTargetListener13viewportAddedERKNS_25RenderTargetViewportEventE
#[doc(alias = "Ogre::RenderTargetListener::viewportAdded(Ogre::RenderTargetViewportEvent const&)")]
// was: Ogre::RenderTargetListener::viewportAdded(Ogre::RenderTargetViewportEvent const&)
// IDA 0xc75094: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c75094() {
}

// 0xc75098 — __ZN4Ogre20RenderTargetListener15viewportRemovedERKNS_25RenderTargetViewportEventE
#[doc(
    alias = "Ogre::RenderTargetListener::viewportRemoved(Ogre::RenderTargetViewportEvent const&)"
)]
// was: Ogre::RenderTargetListener::viewportRemoved(Ogre::RenderTargetViewportEvent const&)
// IDA 0xc75098: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c75098() {
}

// 0xc750d0 — __ZN4Ogre18CompositorInstance13getCompositorEv
#[doc(alias = "Ogre::CompositorInstance::getCompositor(void)")]
// was: Ogre::CompositorInstance::getCompositor(void)
// IDA 0xc750d0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c750d0() {
}

// 0xc750d4 — __ZN4Ogre18CompositorInstance18getTextureInstanceERKSsm
#[doc(alias = "Ogre::CompositorInstance::getTextureInstance(std::string const&,unsigned long)")]
// was: Ogre::CompositorInstance::getTextureInstance(std::string const&,unsigned long)
// IDA 0xc750d4: 102 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c750d4() {
}

// 0xc751e8 — __ZN4Ogre18CompositorInstance18getMRTTexLocalNameERKSsm
#[doc(alias = "Ogre::CompositorInstance::getMRTTexLocalName(std::string const&,unsigned long)")]
// was: Ogre::CompositorInstance::getMRTTexLocalName(std::string const&,unsigned long)
// IDA 0xc751e8: 200 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c751e8() {
}

// 0xc75460 — __ZN4Ogre17CompositorManager12getSingletonEv
#[doc(alias = "Ogre::CompositorManager::getSingleton(void)")]
// was: Ogre::CompositorManager::getSingleton(void)
// IDA 0xc75460: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c75460() {
}

// 0xc75470 — __ZN4Ogre17CompositorManagerC1Ev
#[doc(alias = "Ogre::CompositorManager::CompositorManager(void)")]
// was: Ogre::CompositorManager::CompositorManager(void)
// IDA 0xc75470: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c75470() {
}

// 0xc7547c — __ZN4Ogre17CompositorManagerC2Ev
#[doc(alias = "Ogre::CompositorManager::CompositorManager(void)")]
// was: Ogre::CompositorManager::CompositorManager(void)
// IDA 0xc7547c: 231 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7547c() {
}

// 0xc75724 — __ZN4Ogre17CompositorManagerD0Ev
#[doc(alias = "Ogre::CompositorManager::~CompositorManager()")]
// was: Ogre::CompositorManager::~CompositorManager()
// IDA 0xc75724: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c75724() {
}

// 0xc757b4 — __ZN4Ogre17CompositorManagerD1Ev
#[doc(alias = "Ogre::CompositorManager::~CompositorManager()")]
// was: Ogre::CompositorManager::~CompositorManager()
// IDA 0xc757b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c757b4() {
}

// 0xc757c0 — __ZN4Ogre17CompositorManagerD2Ev
#[doc(alias = "Ogre::CompositorManager::~CompositorManager()")]
// was: Ogre::CompositorManager::~CompositorManager()
// IDA 0xc757c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c757c0() {
}

// 0xc75a20 — __ZN4Ogre17CompositorManager18freePooledTexturesEb
#[doc(alias = "Ogre::CompositorManager::freePooledTextures(bool)")]
// was: Ogre::CompositorManager::freePooledTextures(bool)
// IDA 0xc75a20: 256 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c75a20() {
}

// 0xc75cdc — __ZN4Ogre17CompositorManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(
    alias = "Ogre::CompositorManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)"
)]
// was: Ogre::CompositorManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xc75cdc: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c75cdc() {
}

// 0xc75db0 — __ZN4Ogre17CompositorManager11parseScriptERNS_9SharedPtrINS_10DataStreamEEERKSs
#[doc(
    alias = "Ogre::CompositorManager::parseScript(Ogre::SharedPtr<Ogre::DataStream> &,std::string const&)"
)]
// was: Ogre::CompositorManager::parseScript(Ogre::SharedPtr<Ogre::DataStream> &,std::string const&)
// IDA 0xc75db0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c75db0() {
}

// 0xc75dc8 — __ZN4Ogre17CompositorManager9removeAllEv
#[doc(alias = "Ogre::CompositorManager::removeAll(void)")]
// was: Ogre::CompositorManager::removeAll(void)
// IDA 0xc75dc8: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c75dc8() {
}

// 0xc75e18 — __ZN4Ogre17CompositorManager23_getTexturedRectangle2DEv
#[doc(alias = "Ogre::CompositorManager::_getTexturedRectangle2D(void)")]
// was: Ogre::CompositorManager::_getTexturedRectangle2D(void)
// IDA 0xc75e18: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c75e18() {
}

// 0xc75ffc — __ZNSt8_Rb_treeISt4pairISsSsES0_IKS1_St3mapIN4Ogre17CompositorManager10TextureDefENS4_10TexturePtrENS5_14TextureDefLessENS4_12STLAllocatorIS0_IKS6_S7_ENS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEEESt10_Select1stISH_ESt4lessIS1_ESaISH_EE8_M_eraseEPSt13_Rb_tree_nodeISH_E
#[doc(
    alias = "std::_Rb_tree<std::pair<std::string,std::string>,std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::pair<std::string,std::string>>,std::allocator<std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)"
)]
// was: std::_Rb_tree<std::pair<std::string,std::string>,std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::pair<std::string,std::string>>,std::allocator<std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)
// IDA 0xc75ffc: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c75ffc() {
}

// 0xc76024 — __ZNSt8_Rb_treeISt4pairISsSsES0_IKS1_St3mapIN4Ogre17CompositorManager10TextureDefENS4_10TexturePtrENS5_14TextureDefLessENS4_12STLAllocatorIS0_IKS6_S7_ENS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEEESt10_Select1stISH_ESt4lessIS1_ESaISH_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISH_E
#[doc(
    alias = "std::_Rb_tree<std::pair<std::string,std::string>,std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::pair<std::string,std::string>>,std::allocator<std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)"
)]
// was: std::_Rb_tree<std::pair<std::string,std::string>,std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::pair<std::string,std::string>>,std::allocator<std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::pair<std::string,std::string> const,std::map<Ogre::CompositorManager::TextureDef,Ogre::TexturePtr,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)
// IDA 0xc76024: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c76024() {
}

// 0xc76134 — __ZNSt8_Rb_treeIN4Ogre17CompositorManager10TextureDefESt4pairIKS2_NS0_10TexturePtrEESt10_Select1stIS6_ENS1_14TextureDefLessENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(
    alias = "std::_Rb_tree<Ogre::CompositorManager::TextureDef,std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,std::_Select1st<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>>,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>> *)"
)]
// was: std::_Rb_tree<Ogre::CompositorManager::TextureDef,std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,std::_Select1st<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>>,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>> *)
// IDA 0xc76134: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c76134() {
}

// 0xc76164 — __ZN4Ogre17CompositorManager14TextureDefLessD1Ev
#[doc(alias = "Ogre::CompositorManager::TextureDefLess::~TextureDefLess()")]
// was: Ogre::CompositorManager::TextureDefLess::~TextureDefLess()
// IDA 0xc76164: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c76164() {
}

// 0xc76168 — __ZN4Ogre12STLAllocatorISt4pairIKNS_17CompositorManager10TextureDefENS_10TexturePtrEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_
#[doc(
    alias = "Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>*)"
)]
// was: Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<Ogre::CompositorManager::TextureDef const,Ogre::TexturePtr>*)
// IDA 0xc76168: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c76168() {
}

// 0xc76298 — __ZNSt8_Rb_treeIN4Ogre17CompositorManager10TextureDefESt4pairIKS2_PSt6vectorINS0_10TexturePtrENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ENS1_14TextureDefLessENS7_ISE_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E
#[doc(
    alias = "std::_Rb_tree<Ogre::CompositorManager::TextureDef,std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)"
)]
// was: std::_Rb_tree<Ogre::CompositorManager::TextureDef,std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)
// IDA 0xc76298: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c76298() {
}

// 0xc76310 — __ZNSt8_Rb_treeIPN4Ogre7TextureES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(
    alias = "std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Texture * const&)"
)]
// was: std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Texture * const&)
// IDA 0xc76310: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c76310() {
}

// 0xc76408 — __ZNSt6vectorIN4Ogre10TexturePtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(
    alias = "std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::TexturePtr const&)"
)]
// was: std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::TexturePtr*,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::TexturePtr const&)
// IDA 0xc76408: 376 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c76408() {
}

// 0xc76828 — __ZN4Ogre17CompositorManager14TextureDefLessD0Ev
#[doc(alias = "Ogre::CompositorManager::TextureDefLess::~TextureDefLess()")]
// was: Ogre::CompositorManager::TextureDefLess::~TextureDefLess()
// IDA 0xc76828: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c76828() {
}

// 0xc76834 — __ZNSt8_Rb_treeIPN4Ogre8ViewportESt4pairIKS2_PNS0_15CompositorChainEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(
    alias = "std::_Rb_tree<Ogre::Viewport *,std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>,std::_Select1st<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>>,std::less<Ogre::Viewport *>,Ogre::STLAllocator<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>> *)"
)]
// was: std::_Rb_tree<Ogre::Viewport *,std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>,std::_Select1st<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>>,std::less<Ogre::Viewport *>,Ogre::STLAllocator<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>> *)
// IDA 0xc76834: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c76834() {
}

// 0xc7685c — __ZNSt12_Vector_baseIPN4Ogre18CompositorInstanceENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::CompositorInstance *,Ogre::STLAllocator<Ogre::CompositorInstance *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::CompositorInstance *,Ogre::STLAllocator<Ogre::CompositorInstance *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc7685c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c7685c() {
}

// 0xc76860 — __ZNSt8_Rb_treeIN4Ogre17CompositorManager10TextureDefESt4pairIKS2_PSt6vectorINS0_10TexturePtrENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ENS1_14TextureDefLessENS7_ISE_SA_EEE13_Rb_tree_implISH_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<Ogre::CompositorManager::TextureDef,std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<Ogre::CompositorManager::TextureDefLess,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<Ogre::CompositorManager::TextureDef,std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<Ogre::CompositorManager::TextureDefLess,false>::~_Rb_tree_impl()
// IDA 0xc76860: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c76860() {
}

// 0xc76864 — __ZNSt8_Rb_treeIN4Ogre17CompositorManager10TextureDefESt4pairIKS2_PSt6vectorINS0_10TexturePtrENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ENS1_14TextureDefLessENS7_ISE_SA_EEE13_Rb_tree_implISH_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<Ogre::CompositorManager::TextureDef,std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<Ogre::CompositorManager::TextureDefLess,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<Ogre::CompositorManager::TextureDef,std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::CompositorManager::TextureDefLess,Ogre::STLAllocator<std::pair<Ogre::CompositorManager::TextureDef const,std::vector<Ogre::TexturePtr,Ogre::STLAllocator<Ogre::TexturePtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<Ogre::CompositorManager::TextureDefLess,false>::~_Rb_tree_impl()
// IDA 0xc76864: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c76864() {
}

// 0xc76870 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21CustomCompositionPassEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::CustomCompositionPass *>,std::_Select1st<std::pair<std::string const,Ogre::CustomCompositionPass *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::CustomCompositionPass *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::CustomCompositionPass *>,std::_Select1st<std::pair<std::string const,Ogre::CustomCompositionPass *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::CustomCompositionPass *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc76870: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c76870() {
}

// 0xc76874 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21CustomCompositionPassEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::CustomCompositionPass *>,std::_Select1st<std::pair<std::string const,Ogre::CustomCompositionPass *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::CustomCompositionPass *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::CustomCompositionPass *>,std::_Select1st<std::pair<std::string const,Ogre::CustomCompositionPass *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::CustomCompositionPass *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc76874: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c76874() {
}

// 0xc76880 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15CompositorLogicEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::CompositorLogic *>,std::_Select1st<std::pair<std::string const,Ogre::CompositorLogic *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::CompositorLogic *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::CompositorLogic *>,std::_Select1st<std::pair<std::string const,Ogre::CompositorLogic *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::CompositorLogic *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc76880: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c76880() {
}

// 0xc76884 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15CompositorLogicEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::CompositorLogic *>,std::_Select1st<std::pair<std::string const,Ogre::CompositorLogic *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::CompositorLogic *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::CompositorLogic *>,std::_Select1st<std::pair<std::string const,Ogre::CompositorLogic *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::CompositorLogic *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc76884: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c76884() {
}

// 0xc76890 — __ZNSt12_Vector_baseIPN4Ogre18CompositorInstanceENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::CompositorInstance *,Ogre::STLAllocator<Ogre::CompositorInstance *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::CompositorInstance *,Ogre::STLAllocator<Ogre::CompositorInstance *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc76890: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c76890() {
}

// 0xc7689c — __ZNSt8_Rb_treeIPN4Ogre8ViewportESt4pairIKS2_PNS0_15CompositorChainEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<Ogre::Viewport *,std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>,std::_Select1st<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>>,std::less<Ogre::Viewport *>,Ogre::STLAllocator<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Viewport *>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<Ogre::Viewport *,std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>,std::_Select1st<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>>,std::less<Ogre::Viewport *>,Ogre::STLAllocator<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Viewport *>,false>::~_Rb_tree_impl()
// IDA 0xc7689c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c7689c() {
}

// 0xc768a0 — __ZNSt8_Rb_treeIPN4Ogre8ViewportESt4pairIKS2_PNS0_15CompositorChainEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<Ogre::Viewport *,std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>,std::_Select1st<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>>,std::less<Ogre::Viewport *>,Ogre::STLAllocator<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Viewport *>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<Ogre::Viewport *,std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>,std::_Select1st<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>>,std::less<Ogre::Viewport *>,Ogre::STLAllocator<std::pair<Ogre::Viewport * const,Ogre::CompositorChain *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Viewport *>,false>::~_Rb_tree_impl()
// IDA 0xc768a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c768a0() {
}

// 0xc768ac — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15CompositorLogicEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::CompositorLogic *>,std::_Select1st<std::pair<std::string const,Ogre::CompositorLogic *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::CompositorLogic *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::CompositorLogic *>> *)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::CompositorLogic *>,std::_Select1st<std::pair<std::string const,Ogre::CompositorLogic *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::CompositorLogic *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::CompositorLogic *>> *)
// IDA 0xc768ac: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c768ac() {
}

// 0xc76924 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21CustomCompositionPassEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::CustomCompositionPass *>,std::_Select1st<std::pair<std::string const,Ogre::CustomCompositionPass *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::CustomCompositionPass *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::CustomCompositionPass *>> *)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::CustomCompositionPass *>,std::_Select1st<std::pair<std::string const,Ogre::CustomCompositionPass *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::CustomCompositionPass *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::CustomCompositionPass *>> *)
// IDA 0xc76924: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c76924() {
}

// 0xc769d0 — __ZN4Ogre10ConfigFileC1Ev
#[doc(alias = "Ogre::ConfigFile::ConfigFile(void)")]
// was: Ogre::ConfigFile::ConfigFile(void)
// IDA 0xc769d0: 19 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c769d0() {
}

// 0xc76a04 — __ZN4Ogre10ConfigFileD0Ev
#[doc(alias = "Ogre::ConfigFile::~ConfigFile()")]
// was: Ogre::ConfigFile::~ConfigFile()
// IDA 0xc76a04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c76a04() {
}

// 0xc76a94 — __ZN4Ogre10ConfigFileD1Ev
#[doc(alias = "Ogre::ConfigFile::~ConfigFile()")]
// was: Ogre::ConfigFile::~ConfigFile()
// IDA 0xc76a94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c76a94() {
}

// 0xc76aa0 — __ZN4Ogre10ConfigFileD2Ev
#[doc(alias = "Ogre::ConfigFile::~ConfigFile()")]
// was: Ogre::ConfigFile::~ConfigFile()
// IDA 0xc76aa0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c76aa0() {
}

// 0xc76bac — __ZN4Ogre10ConfigFile5clearEv
#[doc(alias = "Ogre::ConfigFile::clear(void)")]
// was: Ogre::ConfigFile::clear(void)
// IDA 0xc76bac: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c76bac() {
}

// 0xc76c8c — __ZN4Ogre10ConfigFile4loadERKSsS2_b
#[doc(alias = "Ogre::ConfigFile::load(std::string const&,std::string const&,bool)")]
// was: Ogre::ConfigFile::load(std::string const&,std::string const&,bool)
// IDA 0xc76c8c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c76c8c() {
}

// 0xc76c98 — __ZN4Ogre10ConfigFile10loadDirectERKSsS2_b
#[doc(alias = "Ogre::ConfigFile::loadDirect(std::string const&,std::string const&,bool)")]
// was: Ogre::ConfigFile::loadDirect(std::string const&,std::string const&,bool)
// IDA 0xc76c98: 301 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c76c98() {
}

// 0xc7727c — __ZN4Ogre10ConfigFile4loadERKNS_9SharedPtrINS_10DataStreamEEERKSsb
#[doc(
    alias = "Ogre::ConfigFile::load(Ogre::SharedPtr<Ogre::DataStream> const&,std::string const&,bool)"
)]
// was: Ogre::ConfigFile::load(Ogre::SharedPtr<Ogre::DataStream> const&,std::string const&,bool)
// IDA 0xc7727c: 653 insns (PUSH..TBH.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7727c() {
}

// 0xc77cb4 — __ZNK4Ogre10ConfigFile10getSettingERKSsS2_S2_
#[doc(
    alias = "Ogre::ConfigFile::getSetting(std::string const&,std::string const&,std::string const&)const"
)]
// was: Ogre::ConfigFile::getSetting(std::string const&,std::string const&,std::string const&)const
// IDA 0xc77cb4: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c77cb4() {
}

// 0xc77d00 — __ZNK4Ogre10ConfigFile15getMultiSettingERKSsS2_
#[doc(alias = "Ogre::ConfigFile::getMultiSetting(std::string const&,std::string const&)const")]
// was: Ogre::ConfigFile::getMultiSetting(std::string const&,std::string const&)const
// IDA 0xc77d00: 181 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c77d00() {
}

// 0xc77ee0 — __ZN4Ogre10ConfigFile19getSettingsIteratorERKSs
#[doc(alias = "Ogre::ConfigFile::getSettingsIterator(std::string const&)")]
// was: Ogre::ConfigFile::getSettingsIterator(std::string const&)
// IDA 0xc77ee0: 202 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c77ee0() {
}

// 0xc78128 — __ZNSt3mapISsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIKSsSsENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEES2_NS4_IS5_IS6_SD_ESA_EEEixERS6_
#[doc(
    alias = "std::map<std::string,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)"
)]
// was: std::map<std::string,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xc78128: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c78128() {
}

// 0xc782e4 — __ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPSsS6_EERKSs
#[doc(
    alias = "std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::string const&)"
)]
// was: std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::string const&)
// IDA 0xc782e4: 311 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c782e4() {
}

// 0xc78630 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xc78630: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c78630() {
}

// 0xc786d4 — __ZNKSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE4findERS1_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const
// IDA 0xc786d4: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c786d4() {
}

// 0xc78778 — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE4findERS1_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xc78778: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c78778() {
}

// 0xc7881c — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISE_ERKSE_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)
// IDA 0xc7881c: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7881c() {
}

// 0xc789fc — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE9_M_insertEPSt18_Rb_tree_node_baseSK_RKSE_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)
// IDA 0xc789fc: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c789fc() {
}

// 0xc78b50 — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE16_M_insert_uniqueERKSE_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)
// IDA 0xc78b50: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c78b50() {
}

// 0xc78c34 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc78c34: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c78c34() {
}

// 0xc78c38 — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)
// IDA 0xc78c38: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c78c38() {
}

// 0xc78cb0 — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE13_Rb_tree_implIS4_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc78cb0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c78cb0() {
}

// 0xc78cb4 — __ZNSt8_Rb_treeISsSt4pairIKSsPSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorIS0_IS1_SsENS5_22CategorisedAllocPolicyILNS5_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ES4_NS6_ISE_SA_EEE13_Rb_tree_implIS4_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc78cb4: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c78cb4() {
}

// 0xc78cf4 — __ZN4Ogre17ControllerManager12getSingletonEv
#[doc(alias = "Ogre::ControllerManager::getSingleton(void)")]
// was: Ogre::ControllerManager::getSingleton(void)
// IDA 0xc78cf4: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c78cf4() {
}

// 0xc78d04 — __ZN4Ogre17ControllerManagerC1Ev
#[doc(alias = "Ogre::ControllerManager::ControllerManager(void)")]
// was: Ogre::ControllerManager::ControllerManager(void)
// IDA 0xc78d04: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c78d04() {
}

// 0xc78d10 — __ZN4Ogre17ControllerManagerC2Ev
#[doc(alias = "Ogre::ControllerManager::ControllerManager(void)")]
// was: Ogre::ControllerManager::ControllerManager(void)
// IDA 0xc78d10: 242 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c78d10() {
}

// 0xc78f70 — __ZN4Ogre17ControllerManagerD1Ev
#[doc(alias = "Ogre::ControllerManager::~ControllerManager()")]
// was: Ogre::ControllerManager::~ControllerManager()
// IDA 0xc78f70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c78f70() {
}

// 0xc78f7c — __ZN4Ogre17ControllerManagerD2Ev
#[doc(alias = "Ogre::ControllerManager::~ControllerManager()")]
// was: Ogre::ControllerManager::~ControllerManager()
// IDA 0xc78f7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c78f7c() {
}

// 0xc79268 — __ZN4Ogre17ControllerManager36createFrameTimePassthroughControllerERKNS_9SharedPtrINS_15ControllerValueIfEEEE
#[doc(
    alias = "Ogre::ControllerManager::createFrameTimePassthroughController(Ogre::SharedPtr<Ogre::ControllerValue<float>> const&)"
)]
// was: Ogre::ControllerManager::createFrameTimePassthroughController(Ogre::SharedPtr<Ogre::ControllerValue<float>> const&)
// IDA 0xc79268: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c79268() {
}

// 0xc79344 — __ZNK4Ogre17ControllerManager18getFrameTimeSourceEv
#[doc(alias = "Ogre::ControllerManager::getFrameTimeSource(void)const")]
// was: Ogre::ControllerManager::getFrameTimeSource(void)const
// IDA 0xc79344: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c79344() {
}

// 0xc79348 — __ZN4Ogre17ControllerManager20updateAllControllersEv
#[doc(alias = "Ogre::ControllerManager::updateAllControllers(void)")]
// was: Ogre::ControllerManager::updateAllControllers(void)
// IDA 0xc79348: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c79348() {
}

// 0xc793b8 — __ZN4Ogre17ControllerManager21createTextureAnimatorEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureAnimator(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureAnimator(Ogre::TextureUnitState *,float)
// IDA 0xc793b8: 295 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c793b8() {
}

// 0xc7967c — __ZN4Ogre17ControllerManager23createTextureUVScrollerEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureUVScroller(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureUVScroller(Ogre::TextureUnitState *,float)
// IDA 0xc7967c: 327 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7967c() {
}

// 0xc7998c — __ZN4Ogre17ControllerManager22createTextureUScrollerEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureUScroller(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureUScroller(Ogre::TextureUnitState *,float)
// IDA 0xc7998c: 327 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7998c() {
}

// 0xc79c9c — __ZN4Ogre17ControllerManager22createTextureVScrollerEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureVScroller(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureVScroller(Ogre::TextureUnitState *,float)
// IDA 0xc79c9c: 327 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c79c9c() {
}

// 0xc79fac — __ZN4Ogre17ControllerManager20createTextureRotaterEPNS_16TextureUnitStateEf
#[doc(alias = "Ogre::ControllerManager::createTextureRotater(Ogre::TextureUnitState *,float)")]
// was: Ogre::ControllerManager::createTextureRotater(Ogre::TextureUnitState *,float)
// IDA 0xc79fac: 324 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c79fac() {
}

// 0xc7a2b8 — __ZN4Ogre17ControllerManager28createTextureWaveTransformerEPNS_16TextureUnitStateENS1_20TextureTransformTypeENS_12WaveformTypeEffff
#[doc(
    alias = "Ogre::ControllerManager::createTextureWaveTransformer(Ogre::TextureUnitState *,Ogre::TextureUnitState::TextureTransformType,Ogre::WaveformType,float,float,float,float)"
)]
// was: Ogre::ControllerManager::createTextureWaveTransformer(Ogre::TextureUnitState *,Ogre::TextureUnitState::TextureTransformType,Ogre::WaveformType,float,float,float,float)
// IDA 0xc7a2b8: 531 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7a2b8() {
}

// 0xc7a7a8 — __ZN4Ogre17ControllerManager17destroyControllerEPNS_10ControllerIfEE
#[doc(alias = "Ogre::ControllerManager::destroyController(Ogre::Controller<float> *)")]
// was: Ogre::ControllerManager::destroyController(Ogre::Controller<float> *)
// IDA 0xc7a7a8: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7a7a8() {
}

// 0xc7a804 — __ZNK4Ogre17ControllerManager14getElapsedTimeEv
#[doc(alias = "Ogre::ControllerManager::getElapsedTime(void)const")]
// was: Ogre::ControllerManager::getElapsedTime(void)const
// IDA 0xc7a804: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7a804() {
}

// 0xc7a810 — __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::ControllerValue<float>>::~SharedPtr()
// IDA 0xc7a810: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7a810() {
}

// 0xc7a900 — __ZN4Ogre9SharedPtrINS_18ControllerFunctionIfEEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerFunction<float>>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::ControllerFunction<float>>::~SharedPtr()
// IDA 0xc7a900: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7a900() {
}

// 0xc7a9f0 — __ZN4Ogre9SharedPtrINS_18ControllerFunctionIfEEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerFunction<float>>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::ControllerFunction<float>>::~SharedPtr()
// IDA 0xc7a9f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7a9f0() {
}

// 0xc7aae4 — __ZN4Ogre9SharedPtrINS_18ControllerFunctionIfEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerFunction<float>>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::ControllerFunction<float>>::destroy(void)
// IDA 0xc7aae4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7aae4() {
}

// 0xc7ab1c — __ZN4Ogre9SharedPtrINS_18ControllerFunctionIfEEE4swapERS3_
#[doc(
    alias = "Ogre::SharedPtr<Ogre::ControllerFunction<float>>::swap(Ogre::SharedPtr<Ogre::ControllerFunction<float>>&)"
)]
// was: Ogre::SharedPtr<Ogre::ControllerFunction<float>>::swap(Ogre::SharedPtr<Ogre::ControllerFunction<float>>&)
// IDA 0xc7ab1c: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ab1c() {
}

// 0xc7ab38 — __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::ControllerValue<float>>::~SharedPtr()
// IDA 0xc7ab38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7ab38() {
}

// 0xc7ac2c — __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::ControllerValue<float>>::destroy(void)
// IDA 0xc7ac2c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ac2c() {
}

// 0xc7ac64 — __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEE4swapERS3_
#[doc(
    alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::swap(Ogre::SharedPtr<Ogre::ControllerValue<float>>&)"
)]
// was: Ogre::SharedPtr<Ogre::ControllerValue<float>>::swap(Ogre::SharedPtr<Ogre::ControllerValue<float>>&)
// IDA 0xc7ac64: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ac64() {
}

// 0xc7ac80 — __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E
#[doc(
    alias = "std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Controller<float> *> *)"
)]
// was: std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Controller<float> *> *)
// IDA 0xc7ac80: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ac80() {
}

// 0xc7aca8 — __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS3_
#[doc(
    alias = "std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Controller<float> * const&)"
)]
// was: std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Controller<float> * const&)
// IDA 0xc7aca8: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7aca8() {
}

// 0xc7ada0 — __ZN4Ogre10ControllerIfEC2ERKNS_9SharedPtrINS_15ControllerValueIfEEEES7_RKNS2_INS_18ControllerFunctionIfEEEE
#[doc(
    alias = "Ogre::Controller<float>::Controller(Ogre::SharedPtr<Ogre::ControllerValue<float>> const&,Ogre::SharedPtr<Ogre::ControllerValue<float>> const&,Ogre::SharedPtr<Ogre::ControllerFunction<float>> const&)"
)]
// was: Ogre::Controller<float>::Controller(Ogre::SharedPtr<Ogre::ControllerValue<float>> const&,Ogre::SharedPtr<Ogre::ControllerValue<float>> const&,Ogre::SharedPtr<Ogre::ControllerFunction<float>> const&)
// IDA 0xc7ada0: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ada0() {
}

// 0xc7ae5c — __ZN4Ogre10ControllerIfED1Ev
#[doc(alias = "Ogre::Controller<float>::~Controller()")]
// was: Ogre::Controller<float>::~Controller()
// IDA 0xc7ae5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7ae5c() {
}

// 0xc7ae68 — __ZN4Ogre10ControllerIfED0Ev
#[doc(alias = "Ogre::Controller<float>::~Controller()")]
// was: Ogre::Controller<float>::~Controller()
// IDA 0xc7ae68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7ae68() {
}

// 0xc7aef8 — __ZN4Ogre10ControllerIfED2Ev
#[doc(alias = "Ogre::Controller<float>::~Controller()")]
// was: Ogre::Controller<float>::~Controller()
// IDA 0xc7aef8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7aef8() {
}

// 0xc7b0cc — __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS7_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Controller<float> *>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Controller<float> *>,false>::~_Rb_tree_impl()
// IDA 0xc7b0cc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c7b0cc() {
}

// 0xc7b0d0 — __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS7_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Controller<float> *>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Controller<float> *>,false>::~_Rb_tree_impl()
// IDA 0xc7b0d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7b0d0() {
}

// 0xc7b110 — __ZN4Ogre10ConvexBody15_initialisePoolEv
#[doc(alias = "Ogre::ConvexBody::_initialisePool(void)")]
// was: Ogre::ConvexBody::_initialisePool(void)
// IDA 0xc7b110: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b110() {
}

// 0xc7b16c — __ZN4Ogre10ConvexBody12_destroyPoolEv
#[doc(alias = "Ogre::ConvexBody::_destroyPool(void)")]
// was: Ogre::ConvexBody::_destroyPool(void)
// IDA 0xc7b16c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b16c() {
}

// 0xc7b1a0 — __ZNSt6vectorIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(
    alias = "std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()"
)]
// was: std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()
// IDA 0xc7b1a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7b1a0() {
}

// 0xc7b234 — __ZNSt6vectorIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(
    alias = "std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Polygon **,std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Polygon * const&)"
)]
// was: std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Polygon **,std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Polygon * const&)
// IDA 0xc7b234: 159 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b234() {
}

// 0xc7b3dc — __ZNSt12_Vector_baseIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc7b3dc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c7b3dc() {
}

// 0xc7b3e0 — __ZNSt12_Vector_baseIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc7b3e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7b3e0() {
}

// 0xc7b458 — __ZN4Ogre10DataStream7getLineEb
#[doc(alias = "Ogre::DataStream::getLine(bool)")]
// was: Ogre::DataStream::getLine(bool)
// IDA 0xc7b458: 148 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b458() {
}

// 0xc7b630 — __ZN4Ogre10DataStream8readLineEPcmRKSs
#[doc(alias = "Ogre::DataStream::readLine(char *,unsigned long,std::string const&)")]
// was: Ogre::DataStream::readLine(char *,unsigned long,std::string const&)
// IDA 0xc7b630: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b630() {
}

// 0xc7b710 — __ZN4Ogre10DataStream8skipLineERKSs
#[doc(alias = "Ogre::DataStream::skipLine(std::string const&)")]
// was: Ogre::DataStream::skipLine(std::string const&)
// IDA 0xc7b710: 48 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b710() {
}

// 0xc7b78c — __ZN4Ogre10DataStream11getAsStringEv
#[doc(alias = "Ogre::DataStream::getAsString(void)")]
// was: Ogre::DataStream::getAsString(void)
// IDA 0xc7b78c: 116 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b78c() {
}

// 0xc7b8c8 — __ZN4Ogre16MemoryDataStreamC1EPvmbb
#[doc(alias = "Ogre::MemoryDataStream::MemoryDataStream(void *,unsigned long,bool,bool)")]
// was: Ogre::MemoryDataStream::MemoryDataStream(void *,unsigned long,bool,bool)
// IDA 0xc7b8c8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b8c8() {
}

// 0xc7b918 — __ZN4Ogre16MemoryDataStreamC1ERNS_9SharedPtrINS_10DataStreamEEEbb
#[doc(
    alias = "Ogre::MemoryDataStream::MemoryDataStream(Ogre::SharedPtr<Ogre::DataStream> &,bool,bool)"
)]
// was: Ogre::MemoryDataStream::MemoryDataStream(Ogre::SharedPtr<Ogre::DataStream> &,bool,bool)
// IDA 0xc7b918: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b918() {
}

// 0xc7b924 — __ZN4Ogre16MemoryDataStreamC2ERNS_9SharedPtrINS_10DataStreamEEEbb
#[doc(
    alias = "Ogre::MemoryDataStream::MemoryDataStream(Ogre::SharedPtr<Ogre::DataStream> &,bool,bool)"
)]
// was: Ogre::MemoryDataStream::MemoryDataStream(Ogre::SharedPtr<Ogre::DataStream> &,bool,bool)
// IDA 0xc7b924: 210 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b924() {
}

// 0xc7bb48 — __ZN4Ogre16MemoryDataStreamC1ERKSsRKNS_9SharedPtrINS_10DataStreamEEEbb
#[doc(
    alias = "Ogre::MemoryDataStream::MemoryDataStream(std::string const&,Ogre::SharedPtr<Ogre::DataStream> const&,bool,bool)"
)]
// was: Ogre::MemoryDataStream::MemoryDataStream(std::string const&,Ogre::SharedPtr<Ogre::DataStream> const&,bool,bool)
// IDA 0xc7bb48: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7bb48() {
}

// 0xc7bb60 — __ZN4Ogre16MemoryDataStreamC2ERKSsRKNS_9SharedPtrINS_10DataStreamEEEbb
#[doc(
    alias = "Ogre::MemoryDataStream::MemoryDataStream(std::string const&,Ogre::SharedPtr<Ogre::DataStream> const&,bool,bool)"
)]
// was: Ogre::MemoryDataStream::MemoryDataStream(std::string const&,Ogre::SharedPtr<Ogre::DataStream> const&,bool,bool)
// IDA 0xc7bb60: 217 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7bb60() {
}

// 0xc7bd9c — __ZN4Ogre16MemoryDataStreamC1Embb
#[doc(alias = "Ogre::MemoryDataStream::MemoryDataStream(unsigned long,bool,bool)")]
// was: Ogre::MemoryDataStream::MemoryDataStream(unsigned long,bool,bool)
// IDA 0xc7bd9c: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7bd9c() {
}

// 0xc7bed4 — __ZN4Ogre16MemoryDataStreamC1ERKSsmbb
#[doc(
    alias = "Ogre::MemoryDataStream::MemoryDataStream(std::string const&,unsigned long,bool,bool)"
)]
// was: Ogre::MemoryDataStream::MemoryDataStream(std::string const&,unsigned long,bool,bool)
// IDA 0xc7bed4: 114 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7bed4() {
}

// 0xc7c010 — __ZN4Ogre16MemoryDataStreamD0Ev
#[doc(alias = "Ogre::MemoryDataStream::~MemoryDataStream()")]
// was: Ogre::MemoryDataStream::~MemoryDataStream()
// IDA 0xc7c010: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7c010() {
}

// 0xc7c0a0 — __ZN4Ogre16MemoryDataStreamD1Ev
#[doc(alias = "Ogre::MemoryDataStream::~MemoryDataStream()")]
// was: Ogre::MemoryDataStream::~MemoryDataStream()
// IDA 0xc7c0a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7c0a0() {
}

// 0xc7c0ac — __ZN4Ogre16MemoryDataStreamD2Ev
#[doc(alias = "Ogre::MemoryDataStream::~MemoryDataStream()")]
// was: Ogre::MemoryDataStream::~MemoryDataStream()
// IDA 0xc7c0ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7c0ac() {
}

// 0xc7c200 — __ZN4Ogre16MemoryDataStream4readEPvm
#[doc(alias = "Ogre::MemoryDataStream::read(void *,unsigned long)")]
// was: Ogre::MemoryDataStream::read(void *,unsigned long)
// IDA 0xc7c200: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c200() {
}

// 0xc7c234 — __ZN4Ogre16MemoryDataStream5writeEPKvm
#[doc(alias = "Ogre::MemoryDataStream::write(void const*,unsigned long)")]
// was: Ogre::MemoryDataStream::write(void const*,unsigned long)
// IDA 0xc7c234: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c234() {
}

// 0xc7c278 — __ZN4Ogre16MemoryDataStream8readLineEPcmRKSs
#[doc(alias = "Ogre::MemoryDataStream::readLine(char *,unsigned long,std::string const&)")]
// was: Ogre::MemoryDataStream::readLine(char *,unsigned long,std::string const&)
// IDA 0xc7c278: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c278() {
}

// 0xc7c308 — __ZN4Ogre16MemoryDataStream8skipLineERKSs
#[doc(alias = "Ogre::MemoryDataStream::skipLine(std::string const&)")]
// was: Ogre::MemoryDataStream::skipLine(std::string const&)
// IDA 0xc7c308: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c308() {
}

// 0xc7c338 — __ZN4Ogre16MemoryDataStream4skipEl
#[doc(alias = "Ogre::MemoryDataStream::skip(long)")]
// was: Ogre::MemoryDataStream::skip(long)
// IDA 0xc7c338: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c338() {
}

// 0xc7c340 — __ZN4Ogre16MemoryDataStream4seekEm
#[doc(alias = "Ogre::MemoryDataStream::seek(unsigned long)")]
// was: Ogre::MemoryDataStream::seek(unsigned long)
// IDA 0xc7c340: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c340() {
}

// 0xc7c348 — __ZNK4Ogre16MemoryDataStream4tellEv
#[doc(alias = "Ogre::MemoryDataStream::tell(void)const")]
// was: Ogre::MemoryDataStream::tell(void)const
// IDA 0xc7c348: 3 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c348() {
}

// 0xc7c350 — __ZNK4Ogre16MemoryDataStream3eofEv
#[doc(alias = "Ogre::MemoryDataStream::eof(void)const")]
// was: Ogre::MemoryDataStream::eof(void)const
// IDA 0xc7c350: 6 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c350() {
}

// 0xc7c360 — __ZN4Ogre16MemoryDataStream5closeEv
#[doc(alias = "Ogre::MemoryDataStream::close(void)")]
// was: Ogre::MemoryDataStream::close(void)
// IDA 0xc7c360: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c360() {
}

// 0xc7c37c — __ZN4Ogre20FileStreamDataStreamC1ERKSsPSt14basic_ifstreamIcSt11char_traitsIcEEb
#[doc(
    alias = "Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_ifstream<char,std::char_traits<char>> *,bool)"
)]
// was: Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_ifstream<char,std::char_traits<char>> *,bool)
// IDA 0xc7c37c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c37c() {
}

// 0xc7c388 — __ZN4Ogre20FileStreamDataStreamC2ERKSsPSt14basic_ifstreamIcSt11char_traitsIcEEb
#[doc(
    alias = "Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_ifstream<char,std::char_traits<char>> *,bool)"
)]
// was: Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_ifstream<char,std::char_traits<char>> *,bool)
// IDA 0xc7c388: 152 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c388() {
}

// 0xc7c520 — __ZN4Ogre20FileStreamDataStreamC1ERKSsPSt14basic_ifstreamIcSt11char_traitsIcEEmb
#[doc(
    alias = "Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_ifstream<char,std::char_traits<char>> *,unsigned long,bool)"
)]
// was: Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_ifstream<char,std::char_traits<char>> *,unsigned long,bool)
// IDA 0xc7c520: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c520() {
}

// 0xc7c56c — __ZN4Ogre20FileStreamDataStreamC1ERKSsPSt13basic_fstreamIcSt11char_traitsIcEEmb
#[doc(
    alias = "Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_fstream<char,std::char_traits<char>> *,unsigned long,bool)"
)]
// was: Ogre::FileStreamDataStream::FileStreamDataStream(std::string const&,std::basic_fstream<char,std::char_traits<char>> *,unsigned long,bool)
// IDA 0xc7c56c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c56c() {
}

// 0xc7c5b8 — __ZN4Ogre20FileStreamDataStreamD0Ev
#[doc(alias = "Ogre::FileStreamDataStream::~FileStreamDataStream()")]
// was: Ogre::FileStreamDataStream::~FileStreamDataStream()
// IDA 0xc7c5b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7c5b8() {
}

// 0xc7c648 — __ZN4Ogre20FileStreamDataStreamD1Ev
#[doc(alias = "Ogre::FileStreamDataStream::~FileStreamDataStream()")]
// was: Ogre::FileStreamDataStream::~FileStreamDataStream()
// IDA 0xc7c648: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7c648() {
}

// 0xc7c654 — __ZN4Ogre20FileStreamDataStreamD2Ev
#[doc(alias = "Ogre::FileStreamDataStream::~FileStreamDataStream()")]
// was: Ogre::FileStreamDataStream::~FileStreamDataStream()
// IDA 0xc7c654: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7c654() {
}

// 0xc7c798 — __ZN4Ogre20FileStreamDataStream4readEPvm
#[doc(alias = "Ogre::FileStreamDataStream::read(void *,unsigned long)")]
// was: Ogre::FileStreamDataStream::read(void *,unsigned long)
// IDA 0xc7c798: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c798() {
}

// 0xc7c7ac — __ZN4Ogre20FileStreamDataStream5writeEPKvm
#[doc(alias = "Ogre::FileStreamDataStream::write(void const*,unsigned long)")]
// was: Ogre::FileStreamDataStream::write(void const*,unsigned long)
// IDA 0xc7c7ac: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c7ac() {
}

// 0xc7c7e0 — __ZN4Ogre20FileStreamDataStream8readLineEPcmRKSs
#[doc(alias = "Ogre::FileStreamDataStream::readLine(char *,unsigned long,std::string const&)")]
// was: Ogre::FileStreamDataStream::readLine(char *,unsigned long,std::string const&)
// IDA 0xc7c7e0: 393 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7c7e0() {
}

// 0xc7cc5c — __ZN4Ogre20FileStreamDataStream4skipEl
#[doc(alias = "Ogre::FileStreamDataStream::skip(long)")]
// was: Ogre::FileStreamDataStream::skip(long)
// IDA 0xc7cc5c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7cc5c() {
}

// 0xc7cc84 — __ZN4Ogre20FileStreamDataStream4seekEm
#[doc(alias = "Ogre::FileStreamDataStream::seek(unsigned long)")]
// was: Ogre::FileStreamDataStream::seek(unsigned long)
// IDA 0xc7cc84: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7cc84() {
}

// 0xc7ccac — __ZNK4Ogre20FileStreamDataStream4tellEv
#[doc(alias = "Ogre::FileStreamDataStream::tell(void)const")]
// was: Ogre::FileStreamDataStream::tell(void)const
// IDA 0xc7ccac: 26 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ccac() {
}

// 0xc7ccf0 — __ZNK4Ogre20FileStreamDataStream3eofEv
#[doc(alias = "Ogre::FileStreamDataStream::eof(void)const")]
// was: Ogre::FileStreamDataStream::eof(void)const
// IDA 0xc7ccf0: 8 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ccf0() {
}

// 0xc7cd04 — __ZN4Ogre20FileStreamDataStream5closeEv
#[doc(alias = "Ogre::FileStreamDataStream::close(void)")]
// was: Ogre::FileStreamDataStream::close(void)
// IDA 0xc7cd04: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7cd04() {
}

// 0xc7cdc4 — __ZN4Ogre8DDSCodec7startupEv
#[doc(alias = "Ogre::DDSCodec::startup(void)")]
// was: Ogre::DDSCodec::startup(void)
// IDA 0xc7cdc4: 162 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7cdc4() {
}

// 0xc7cf9c — __ZN4Ogre8DDSCodec8shutdownEv
#[doc(alias = "Ogre::DDSCodec::shutdown(void)")]
// was: Ogre::DDSCodec::shutdown(void)
// IDA 0xc7cf9c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7cf9c() {
}

// 0xc7cfc8 — __ZNK4Ogre8DDSCodec4codeERNS_9SharedPtrINS_16MemoryDataStreamEEERNS1_INS_5Codec9CodecDataEEE
#[doc(
    alias = "Ogre::DDSCodec::code(Ogre::SharedPtr<Ogre::MemoryDataStream> &,Ogre::SharedPtr<Ogre::Codec::CodecData> &)const"
)]
// was: Ogre::DDSCodec::code(Ogre::SharedPtr<Ogre::MemoryDataStream> &,Ogre::SharedPtr<Ogre::Codec::CodecData> &)const
// IDA 0xc7cfc8: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7cfc8() {
}

// 0xc7d178 — __ZNK4Ogre8DDSCodec10codeToFileERNS_9SharedPtrINS_16MemoryDataStreamEEERKSsRNS1_INS_5Codec9CodecDataEEE
#[doc(
    alias = "Ogre::DDSCodec::codeToFile(Ogre::SharedPtr<Ogre::MemoryDataStream> &,std::string const&,Ogre::SharedPtr<Ogre::Codec::CodecData> &)const"
)]
// was: Ogre::DDSCodec::codeToFile(Ogre::SharedPtr<Ogre::MemoryDataStream> &,std::string const&,Ogre::SharedPtr<Ogre::Codec::CodecData> &)const
// IDA 0xc7d178: 466 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7d178() {
}

// 0xc7d8b8 — __ZNK4Ogre8DDSCodec18convertPixelFormatEjjjjj
#[doc(
    alias = "Ogre::DDSCodec::convertPixelFormat(unsigned int,unsigned int,unsigned int,unsigned int,unsigned int)const"
)]
// was: Ogre::DDSCodec::convertPixelFormat(unsigned int,unsigned int,unsigned int,unsigned int,unsigned int)const
// IDA 0xc7d8b8: 150 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7d8b8() {
}

// 0xc7db10 — __ZNK4Ogre8DDSCodec15unpackDXTColourENS_11PixelFormatERKNS_14DXTColourBlockEPNS_11ColourValueE
#[doc(
    alias = "Ogre::DDSCodec::unpackDXTColour(Ogre::PixelFormat,Ogre::DXTColourBlock const&,Ogre::ColourValue *)const"
)]
// was: Ogre::DDSCodec::unpackDXTColour(Ogre::PixelFormat,Ogre::DXTColourBlock const&,Ogre::ColourValue *)const
// IDA 0xc7db10: 219 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7db10() {
}

// 0xc7ddc4 — __ZNK4Ogre8DDSCodec14unpackDXTAlphaERKNS_21DXTExplicitAlphaBlockEPNS_11ColourValueE
#[doc(
    alias = "Ogre::DDSCodec::unpackDXTAlpha(Ogre::DXTExplicitAlphaBlock const&,Ogre::ColourValue *)const"
)]
// was: Ogre::DDSCodec::unpackDXTAlpha(Ogre::DXTExplicitAlphaBlock const&,Ogre::ColourValue *)const
// IDA 0xc7ddc4: 86 insns (LDRH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ddc4() {
}

// 0xc7df0c — __ZNK4Ogre8DDSCodec14unpackDXTAlphaERKNS_25DXTInterpolatedAlphaBlockEPNS_11ColourValueE
#[doc(
    alias = "Ogre::DDSCodec::unpackDXTAlpha(Ogre::DXTInterpolatedAlphaBlock const&,Ogre::ColourValue *)const"
)]
// was: Ogre::DDSCodec::unpackDXTAlpha(Ogre::DXTInterpolatedAlphaBlock const&,Ogre::ColourValue *)const
// IDA 0xc7df0c: 119 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7df0c() {
}

// 0xc7e0d4 — __ZNK4Ogre8DDSCodec6decodeERNS_9SharedPtrINS_10DataStreamEEE
#[doc(alias = "Ogre::DDSCodec::decode(Ogre::SharedPtr<Ogre::DataStream> &)const")]
// was: Ogre::DDSCodec::decode(Ogre::SharedPtr<Ogre::DataStream> &)const
// IDA 0xc7e0d4: 1397 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7e0d4() {
}

// 0xc7eea8 — __ZNK4Ogre8DDSCodec7getTypeEv
#[doc(alias = "Ogre::DDSCodec::getType(void)const")]
// was: Ogre::DDSCodec::getType(void)const
// IDA 0xc7eea8: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7eea8() {
}

// 0xc7eeb4 — __ZNK4Ogre8DDSCodec20magicNumberToFileExtEPKcm
#[doc(alias = "Ogre::DDSCodec::magicNumberToFileExt(char const*,unsigned long)const")]
// was: Ogre::DDSCodec::magicNumberToFileExt(char const*,unsigned long)const
// IDA 0xc7eeb4: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7eeb4() {
}

// 0xc7eef4 — __ZN4Ogre8DDSCodecD1Ev
#[doc(alias = "Ogre::DDSCodec::~DDSCodec()")]
// was: Ogre::DDSCodec::~DDSCodec()
// IDA 0xc7eef4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7eef4() {
}

// 0xc7ef54 — __ZN4Ogre8DDSCodecD0Ev
#[doc(alias = "Ogre::DDSCodec::~DDSCodec()")]
// was: Ogre::DDSCodec::~DDSCodec()
// IDA 0xc7ef54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7ef54() {
}

// 0xc7f068 — __ZN4Ogre27DefaultHardwareVertexBufferC1EPNS_25HardwareBufferManagerBaseEmmNS_14HardwareBuffer5UsageE
#[doc(
    alias = "Ogre::DefaultHardwareVertexBuffer::DefaultHardwareVertexBuffer(Ogre::HardwareBufferManagerBase *,unsigned long,unsigned long,Ogre::HardwareBuffer::Usage)"
)]
// was: Ogre::DefaultHardwareVertexBuffer::DefaultHardwareVertexBuffer(Ogre::HardwareBufferManagerBase *,unsigned long,unsigned long,Ogre::HardwareBuffer::Usage)
// IDA 0xc7f068: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f068() {
}

// 0xc7f134 — __ZN4Ogre27DefaultHardwareVertexBufferD0Ev
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::~DefaultHardwareVertexBuffer()")]
// was: Ogre::DefaultHardwareVertexBuffer::~DefaultHardwareVertexBuffer()
// IDA 0xc7f134: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f134() {
}

// 0xc7f1f8 — __ZN4Ogre27DefaultHardwareVertexBufferD1Ev
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::~DefaultHardwareVertexBuffer()")]
// was: Ogre::DefaultHardwareVertexBuffer::~DefaultHardwareVertexBuffer()
// IDA 0xc7f1f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f1f8() {
}

// 0xc7f2ac — __ZN4Ogre27DefaultHardwareVertexBuffer8lockImplEmmNS_14HardwareBuffer11LockOptionsE
#[doc(
    alias = "Ogre::DefaultHardwareVertexBuffer::lockImpl(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)"
)]
// was: Ogre::DefaultHardwareVertexBuffer::lockImpl(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)
// IDA 0xc7f2ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f2ac() {
}

// 0xc7f2b4 — __ZN4Ogre27DefaultHardwareVertexBuffer10unlockImplEv
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::unlockImpl(void)")]
// was: Ogre::DefaultHardwareVertexBuffer::unlockImpl(void)
// IDA 0xc7f2b4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c7f2b4() {
}

// 0xc7f2b8 — __ZN4Ogre27DefaultHardwareVertexBuffer4lockEmmNS_14HardwareBuffer11LockOptionsE
#[doc(
    alias = "Ogre::DefaultHardwareVertexBuffer::lock(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)"
)]
// was: Ogre::DefaultHardwareVertexBuffer::lock(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)
// IDA 0xc7f2b8: 5 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f2b8() {
}

// 0xc7f2c4 — __ZN4Ogre27DefaultHardwareVertexBuffer6unlockEv
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::unlock(void)")]
// was: Ogre::DefaultHardwareVertexBuffer::unlock(void)
// IDA 0xc7f2c4: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f2c4() {
}

// 0xc7f2cc — __ZN4Ogre27DefaultHardwareVertexBuffer8readDataEmmPv
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::readData(unsigned long,unsigned long,void *)")]
// was: Ogre::DefaultHardwareVertexBuffer::readData(unsigned long,unsigned long,void *)
// IDA 0xc7f2cc: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f2cc() {
}

// 0xc7f2dc — __ZN4Ogre27DefaultHardwareVertexBuffer9writeDataEmmPKvb
#[doc(
    alias = "Ogre::DefaultHardwareVertexBuffer::writeData(unsigned long,unsigned long,void const*,bool)"
)]
// was: Ogre::DefaultHardwareVertexBuffer::writeData(unsigned long,unsigned long,void const*,bool)
// IDA 0xc7f2dc: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f2dc() {
}

// 0xc7f2ec — __ZN4Ogre26DefaultHardwareIndexBufferC1ENS_19HardwareIndexBuffer9IndexTypeEmNS_14HardwareBuffer5UsageE
#[doc(
    alias = "Ogre::DefaultHardwareIndexBuffer::DefaultHardwareIndexBuffer(Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage)"
)]
// was: Ogre::DefaultHardwareIndexBuffer::DefaultHardwareIndexBuffer(Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage)
// IDA 0xc7f2ec: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f2ec() {
}

// 0xc7f3bc — __ZN4Ogre26DefaultHardwareIndexBufferD0Ev
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::~DefaultHardwareIndexBuffer()")]
// was: Ogre::DefaultHardwareIndexBuffer::~DefaultHardwareIndexBuffer()
// IDA 0xc7f3bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f3bc() {
}

// 0xc7f47c — __ZN4Ogre26DefaultHardwareIndexBufferD1Ev
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::~DefaultHardwareIndexBuffer()")]
// was: Ogre::DefaultHardwareIndexBuffer::~DefaultHardwareIndexBuffer()
// IDA 0xc7f47c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f47c() {
}

// 0xc7f530 — __ZN4Ogre26DefaultHardwareIndexBuffer8lockImplEmmNS_14HardwareBuffer11LockOptionsE
#[doc(
    alias = "Ogre::DefaultHardwareIndexBuffer::lockImpl(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)"
)]
// was: Ogre::DefaultHardwareIndexBuffer::lockImpl(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)
// IDA 0xc7f530: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f530() {
}

// 0xc7f538 — __ZN4Ogre26DefaultHardwareIndexBuffer10unlockImplEv
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::unlockImpl(void)")]
// was: Ogre::DefaultHardwareIndexBuffer::unlockImpl(void)
// IDA 0xc7f538: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c7f538() {
}

// 0xc7f53c — __ZN4Ogre26DefaultHardwareIndexBuffer4lockEmmNS_14HardwareBuffer11LockOptionsE
#[doc(
    alias = "Ogre::DefaultHardwareIndexBuffer::lock(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)"
)]
// was: Ogre::DefaultHardwareIndexBuffer::lock(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)
// IDA 0xc7f53c: 5 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f53c() {
}

// 0xc7f548 — __ZN4Ogre26DefaultHardwareIndexBuffer6unlockEv
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::unlock(void)")]
// was: Ogre::DefaultHardwareIndexBuffer::unlock(void)
// IDA 0xc7f548: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f548() {
}

// 0xc7f550 — __ZN4Ogre26DefaultHardwareIndexBuffer8readDataEmmPv
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::readData(unsigned long,unsigned long,void *)")]
// was: Ogre::DefaultHardwareIndexBuffer::readData(unsigned long,unsigned long,void *)
// IDA 0xc7f550: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f550() {
}

// 0xc7f560 — __ZN4Ogre26DefaultHardwareIndexBuffer9writeDataEmmPKvb
#[doc(
    alias = "Ogre::DefaultHardwareIndexBuffer::writeData(unsigned long,unsigned long,void const*,bool)"
)]
// was: Ogre::DefaultHardwareIndexBuffer::writeData(unsigned long,unsigned long,void const*,bool)
// IDA 0xc7f560: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f560() {
}

// 0xc7f5a4 — __ZN4Ogre29DefaultIntersectionSceneQueryC1EPNS_12SceneManagerE
#[doc(
    alias = "Ogre::DefaultIntersectionSceneQuery::DefaultIntersectionSceneQuery(Ogre::SceneManager *)"
)]
// was: Ogre::DefaultIntersectionSceneQuery::DefaultIntersectionSceneQuery(Ogre::SceneManager *)
// IDA 0xc7f5a4: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f5a4() {
}

// 0xc7f66c — __ZN4Ogre29DefaultIntersectionSceneQueryD0Ev
#[doc(alias = "Ogre::DefaultIntersectionSceneQuery::~DefaultIntersectionSceneQuery()")]
// was: Ogre::DefaultIntersectionSceneQuery::~DefaultIntersectionSceneQuery()
// IDA 0xc7f66c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f66c() {
}

// 0xc7f6fc — __ZN4Ogre29DefaultIntersectionSceneQueryD1Ev
#[doc(alias = "Ogre::DefaultIntersectionSceneQuery::~DefaultIntersectionSceneQuery()")]
// was: Ogre::DefaultIntersectionSceneQuery::~DefaultIntersectionSceneQuery()
// IDA 0xc7f6fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f6fc() {
}

// 0xc7f708 — __ZThn48_N4Ogre29DefaultIntersectionSceneQueryD0Ev
#[doc(
    alias = "non-virtual thunk toOgre::DefaultIntersectionSceneQuery::~DefaultIntersectionSceneQuery()"
)]
// was: non-virtual thunk toOgre::DefaultIntersectionSceneQuery::~DefaultIntersectionSceneQuery()
// IDA 0xc7f708: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f708() {
}

// 0xc7f79c — __ZThn48_N4Ogre29DefaultIntersectionSceneQueryD1Ev
#[doc(
    alias = "non-virtual thunk toOgre::DefaultIntersectionSceneQuery::~DefaultIntersectionSceneQuery()"
)]
// was: non-virtual thunk toOgre::DefaultIntersectionSceneQuery::~DefaultIntersectionSceneQuery()
// IDA 0xc7f79c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f79c() {
}

// 0xc7f7a8 — __ZN4Ogre29DefaultIntersectionSceneQuery7executeEPNS_30IntersectionSceneQueryListenerE
#[doc(
    alias = "Ogre::DefaultIntersectionSceneQuery::execute(Ogre::IntersectionSceneQueryListener *)"
)]
// was: Ogre::DefaultIntersectionSceneQuery::execute(Ogre::IntersectionSceneQueryListener *)
// IDA 0xc7f7a8: 253 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f7a8() {
}

// 0xc7fa48 — __ZN4Ogre31DefaultAxisAlignedBoxSceneQueryC1EPNS_12SceneManagerE
#[doc(
    alias = "Ogre::DefaultAxisAlignedBoxSceneQuery::DefaultAxisAlignedBoxSceneQuery(Ogre::SceneManager *)"
)]
// was: Ogre::DefaultAxisAlignedBoxSceneQuery::DefaultAxisAlignedBoxSceneQuery(Ogre::SceneManager *)
// IDA 0xc7fa48: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7fa48() {
}

// 0xc7fb10 — __ZN4Ogre31DefaultAxisAlignedBoxSceneQueryD0Ev
#[doc(alias = "Ogre::DefaultAxisAlignedBoxSceneQuery::~DefaultAxisAlignedBoxSceneQuery()")]
// was: Ogre::DefaultAxisAlignedBoxSceneQuery::~DefaultAxisAlignedBoxSceneQuery()
// IDA 0xc7fb10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fb10() {
}

// 0xc7fba0 — __ZN4Ogre31DefaultAxisAlignedBoxSceneQueryD1Ev
#[doc(alias = "Ogre::DefaultAxisAlignedBoxSceneQuery::~DefaultAxisAlignedBoxSceneQuery()")]
// was: Ogre::DefaultAxisAlignedBoxSceneQuery::~DefaultAxisAlignedBoxSceneQuery()
// IDA 0xc7fba0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fba0() {
}

// 0xc7fbac — __ZThn48_N4Ogre31DefaultAxisAlignedBoxSceneQueryD0Ev
#[doc(
    alias = "non-virtual thunk toOgre::DefaultAxisAlignedBoxSceneQuery::~DefaultAxisAlignedBoxSceneQuery()"
)]
// was: non-virtual thunk toOgre::DefaultAxisAlignedBoxSceneQuery::~DefaultAxisAlignedBoxSceneQuery()
// IDA 0xc7fbac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fbac() {
}

// 0xc7fc40 — __ZThn48_N4Ogre31DefaultAxisAlignedBoxSceneQueryD1Ev
#[doc(
    alias = "non-virtual thunk toOgre::DefaultAxisAlignedBoxSceneQuery::~DefaultAxisAlignedBoxSceneQuery()"
)]
// was: non-virtual thunk toOgre::DefaultAxisAlignedBoxSceneQuery::~DefaultAxisAlignedBoxSceneQuery()
// IDA 0xc7fc40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fc40() {
}

// 0xc7fc4c — __ZN4Ogre31DefaultAxisAlignedBoxSceneQuery7executeEPNS_18SceneQueryListenerE
#[doc(alias = "Ogre::DefaultAxisAlignedBoxSceneQuery::execute(Ogre::SceneQueryListener *)")]
// was: Ogre::DefaultAxisAlignedBoxSceneQuery::execute(Ogre::SceneQueryListener *)
// IDA 0xc7fc4c: 117 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7fc4c() {
}

// 0xc7fd80 — __ZN4Ogre20DefaultRaySceneQueryC1EPNS_12SceneManagerE
#[doc(alias = "Ogre::DefaultRaySceneQuery::DefaultRaySceneQuery(Ogre::SceneManager *)")]
// was: Ogre::DefaultRaySceneQuery::DefaultRaySceneQuery(Ogre::SceneManager *)
// IDA 0xc7fd80: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7fd80() {
}

// 0xc7fe48 — __ZN4Ogre20DefaultRaySceneQueryD0Ev
#[doc(alias = "Ogre::DefaultRaySceneQuery::~DefaultRaySceneQuery()")]
// was: Ogre::DefaultRaySceneQuery::~DefaultRaySceneQuery()
// IDA 0xc7fe48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fe48() {
}

// 0xc7fed8 — __ZN4Ogre20DefaultRaySceneQueryD1Ev
#[doc(alias = "Ogre::DefaultRaySceneQuery::~DefaultRaySceneQuery()")]
// was: Ogre::DefaultRaySceneQuery::~DefaultRaySceneQuery()
// IDA 0xc7fed8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fed8() {
}

// 0xc7fee4 — __ZThn48_N4Ogre20DefaultRaySceneQueryD0Ev
#[doc(alias = "non-virtual thunk toOgre::DefaultRaySceneQuery::~DefaultRaySceneQuery()")]
// was: non-virtual thunk toOgre::DefaultRaySceneQuery::~DefaultRaySceneQuery()
// IDA 0xc7fee4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fee4() {
}

// 0xc7ff78 — __ZThn48_N4Ogre20DefaultRaySceneQueryD1Ev
#[doc(alias = "non-virtual thunk toOgre::DefaultRaySceneQuery::~DefaultRaySceneQuery()")]
// was: non-virtual thunk toOgre::DefaultRaySceneQuery::~DefaultRaySceneQuery()
// IDA 0xc7ff78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7ff78() {
}

// 0xc7ff84 — __ZN4Ogre20DefaultRaySceneQuery7executeEPNS_21RaySceneQueryListenerE
#[doc(alias = "Ogre::DefaultRaySceneQuery::execute(Ogre::RaySceneQueryListener *)")]
// was: Ogre::DefaultRaySceneQuery::execute(Ogre::RaySceneQueryListener *)
// IDA 0xc7ff84: 86 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ff84() {
}

// 0xc8004c — __ZN4Ogre23DefaultSphereSceneQueryC1EPNS_12SceneManagerE
#[doc(alias = "Ogre::DefaultSphereSceneQuery::DefaultSphereSceneQuery(Ogre::SceneManager *)")]
// was: Ogre::DefaultSphereSceneQuery::DefaultSphereSceneQuery(Ogre::SceneManager *)
// IDA 0xc8004c: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8004c() {
}

// 0xc80114 — __ZN4Ogre23DefaultSphereSceneQueryD0Ev
#[doc(alias = "Ogre::DefaultSphereSceneQuery::~DefaultSphereSceneQuery()")]
// was: Ogre::DefaultSphereSceneQuery::~DefaultSphereSceneQuery()
// IDA 0xc80114: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c80114() {
}

// 0xc801a4 — __ZN4Ogre23DefaultSphereSceneQueryD1Ev
#[doc(alias = "Ogre::DefaultSphereSceneQuery::~DefaultSphereSceneQuery()")]
// was: Ogre::DefaultSphereSceneQuery::~DefaultSphereSceneQuery()
// IDA 0xc801a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c801a4() {
}

// 0xc801b0 — __ZThn48_N4Ogre23DefaultSphereSceneQueryD0Ev
#[doc(alias = "non-virtual thunk toOgre::DefaultSphereSceneQuery::~DefaultSphereSceneQuery()")]
// was: non-virtual thunk toOgre::DefaultSphereSceneQuery::~DefaultSphereSceneQuery()
// IDA 0xc801b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c801b0() {
}

// 0xc80244 — __ZThn48_N4Ogre23DefaultSphereSceneQueryD1Ev
#[doc(alias = "non-virtual thunk toOgre::DefaultSphereSceneQuery::~DefaultSphereSceneQuery()")]
// was: non-virtual thunk toOgre::DefaultSphereSceneQuery::~DefaultSphereSceneQuery()
// IDA 0xc80244: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c80244() {
}

// 0xc80250 — __ZN4Ogre23DefaultSphereSceneQuery7executeEPNS_18SceneQueryListenerE
#[doc(alias = "Ogre::DefaultSphereSceneQuery::execute(Ogre::SceneQueryListener *)")]
// was: Ogre::DefaultSphereSceneQuery::execute(Ogre::SceneQueryListener *)
// IDA 0xc80250: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80250() {
}

// 0xc80384 — __ZN4Ogre39DefaultPlaneBoundedVolumeListSceneQueryC1EPNS_12SceneManagerE
#[doc(
    alias = "Ogre::DefaultPlaneBoundedVolumeListSceneQuery::DefaultPlaneBoundedVolumeListSceneQuery(Ogre::SceneManager *)"
)]
// was: Ogre::DefaultPlaneBoundedVolumeListSceneQuery::DefaultPlaneBoundedVolumeListSceneQuery(Ogre::SceneManager *)
// IDA 0xc80384: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80384() {
}

// 0xc8044c — __ZN4Ogre39DefaultPlaneBoundedVolumeListSceneQueryD0Ev
#[doc(
    alias = "Ogre::DefaultPlaneBoundedVolumeListSceneQuery::~DefaultPlaneBoundedVolumeListSceneQuery()"
)]
// was: Ogre::DefaultPlaneBoundedVolumeListSceneQuery::~DefaultPlaneBoundedVolumeListSceneQuery()
// IDA 0xc8044c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8044c() {
}

// 0xc804dc — __ZN4Ogre39DefaultPlaneBoundedVolumeListSceneQueryD1Ev
#[doc(
    alias = "Ogre::DefaultPlaneBoundedVolumeListSceneQuery::~DefaultPlaneBoundedVolumeListSceneQuery()"
)]
// was: Ogre::DefaultPlaneBoundedVolumeListSceneQuery::~DefaultPlaneBoundedVolumeListSceneQuery()
// IDA 0xc804dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c804dc() {
}

// 0xc804e8 — __ZThn48_N4Ogre39DefaultPlaneBoundedVolumeListSceneQueryD0Ev
#[doc(
    alias = "non-virtual thunk toOgre::DefaultPlaneBoundedVolumeListSceneQuery::~DefaultPlaneBoundedVolumeListSceneQuery()"
)]
// was: non-virtual thunk toOgre::DefaultPlaneBoundedVolumeListSceneQuery::~DefaultPlaneBoundedVolumeListSceneQuery()
// IDA 0xc804e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c804e8() {
}

// 0xc8057c — __ZThn48_N4Ogre39DefaultPlaneBoundedVolumeListSceneQueryD1Ev
#[doc(
    alias = "non-virtual thunk toOgre::DefaultPlaneBoundedVolumeListSceneQuery::~DefaultPlaneBoundedVolumeListSceneQuery()"
)]
// was: non-virtual thunk toOgre::DefaultPlaneBoundedVolumeListSceneQuery::~DefaultPlaneBoundedVolumeListSceneQuery()
// IDA 0xc8057c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8057c() {
}

// 0xc80588 — __ZN4Ogre39DefaultPlaneBoundedVolumeListSceneQuery7executeEPNS_18SceneQueryListenerE
#[doc(alias = "Ogre::DefaultPlaneBoundedVolumeListSceneQuery::execute(Ogre::SceneQueryListener *)")]
// was: Ogre::DefaultPlaneBoundedVolumeListSceneQuery::execute(Ogre::SceneQueryListener *)
// IDA 0xc80588: 150 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80588() {
}

// 0xc80728 — __ZNSt8_Rb_treeIN4Ogre10SceneQuery17WorldFragmentTypeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(
    alias = "std::_Rb_tree<Ogre::SceneQuery::WorldFragmentType,Ogre::SceneQuery::WorldFragmentType,std::_Identity<Ogre::SceneQuery::WorldFragmentType>,std::less<Ogre::SceneQuery::WorldFragmentType>,Ogre::STLAllocator<Ogre::SceneQuery::WorldFragmentType,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::SceneQuery::WorldFragmentType const&)"
)]
// was: std::_Rb_tree<Ogre::SceneQuery::WorldFragmentType,Ogre::SceneQuery::WorldFragmentType,std::_Identity<Ogre::SceneQuery::WorldFragmentType>,std::less<Ogre::SceneQuery::WorldFragmentType>,Ogre::STLAllocator<Ogre::SceneQuery::WorldFragmentType,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::SceneQuery::WorldFragmentType const&)
// IDA 0xc80728: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80728() {
}

// 0xc80854 — __ZN4Ogre19DistanceLodStrategy15getSingletonPtrEv
#[doc(alias = "Ogre::DistanceLodStrategy::getSingletonPtr(void)")]
// was: Ogre::DistanceLodStrategy::getSingletonPtr(void)
// IDA 0xc80854: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80854() {
}

// 0xc80864 — __ZN4Ogre19DistanceLodStrategyC1Ev
#[doc(alias = "Ogre::DistanceLodStrategy::DistanceLodStrategy(void)")]
// was: Ogre::DistanceLodStrategy::DistanceLodStrategy(void)
// IDA 0xc80864: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80864() {
}

// 0xc80870 — __ZN4Ogre19DistanceLodStrategyC2Ev
#[doc(alias = "Ogre::DistanceLodStrategy::DistanceLodStrategy(void)")]
// was: Ogre::DistanceLodStrategy::DistanceLodStrategy(void)
// IDA 0xc80870: 116 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80870() {
}

// 0xc809cc — __ZNK4Ogre19DistanceLodStrategy12getValueImplEPKNS_13MovableObjectEPKNS_6CameraE
#[doc(
    alias = "Ogre::DistanceLodStrategy::getValueImpl(Ogre::MovableObject const*,Ogre::Camera const*)const"
)]
// was: Ogre::DistanceLodStrategy::getValueImpl(Ogre::MovableObject const*,Ogre::Camera const*)const
// IDA 0xc809cc: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c809cc() {
}

// 0xc80ad8 — __ZNK4Ogre19DistanceLodStrategy12getBaseValueEv
#[doc(alias = "Ogre::DistanceLodStrategy::getBaseValue(void)const")]
// was: Ogre::DistanceLodStrategy::getBaseValue(void)const
// IDA 0xc80ad8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80ad8() {
}

// 0xc80adc — __ZNK4Ogre19DistanceLodStrategy13transformBiasEf
#[doc(alias = "Ogre::DistanceLodStrategy::transformBias(float)const")]
// was: Ogre::DistanceLodStrategy::transformBias(float)const
// IDA 0xc80adc: 5 insns (VMOV.F32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80adc() {
}

// 0xc80af0 — __ZNK4Ogre19DistanceLodStrategy18transformUserValueEf
#[doc(alias = "Ogre::DistanceLodStrategy::transformUserValue(float)const")]
// was: Ogre::DistanceLodStrategy::transformUserValue(float)const
// IDA 0xc80af0: 4 insns (VMOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80af0() {
}

// 0xc80b00 — __ZNK4Ogre19DistanceLodStrategy8getIndexEfRKSt6vectorINS_12MeshLodUsageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(
    alias = "Ogre::DistanceLodStrategy::getIndex(float,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)const"
)]
// was: Ogre::DistanceLodStrategy::getIndex(float,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)const
// IDA 0xc80b00: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80b00() {
}

// 0xc80b10 — __ZNK4Ogre19DistanceLodStrategy8getIndexEfRKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(
    alias = "Ogre::DistanceLodStrategy::getIndex(float,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)const"
)]
// was: Ogre::DistanceLodStrategy::getIndex(float,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)const
// IDA 0xc80b10: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80b10() {
}

// 0xc80b20 — __ZNK4Ogre19DistanceLodStrategy8isSortedERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(
    alias = "Ogre::DistanceLodStrategy::isSorted(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)const"
)]
// was: Ogre::DistanceLodStrategy::isSorted(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)const
// IDA 0xc80b20: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80b20() {
}

// 0xc80b2c — __ZNK4Ogre19DistanceLodStrategy4sortERSt6vectorINS_12MeshLodUsageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(
    alias = "Ogre::DistanceLodStrategy::sort(std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const"
)]
// was: Ogre::DistanceLodStrategy::sort(std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const
// IDA 0xc80b2c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80b2c() {
}

// 0xc80b38 — __ZN4Ogre19DistanceLodStrategyD1Ev
#[doc(alias = "Ogre::DistanceLodStrategy::~DistanceLodStrategy()")]
// was: Ogre::DistanceLodStrategy::~DistanceLodStrategy()
// IDA 0xc80b38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c80b38() {
}

// 0xc80b50 — __ZN4Ogre19DistanceLodStrategyD0Ev
#[doc(alias = "Ogre::DistanceLodStrategy::~DistanceLodStrategy()")]
// was: Ogre::DistanceLodStrategy::~DistanceLodStrategy()
// IDA 0xc80b50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c80b50() {
}

// 0xc80c24 — __ZN4Ogre6DynLibC1ERKSs
#[doc(alias = "Ogre::DynLib::DynLib(std::string const&)")]
// was: Ogre::DynLib::DynLib(std::string const&)
// IDA 0xc80c24: 82 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80c24() {
}

// 0xc80d10 — __ZN4Ogre6DynLibD1Ev
#[doc(alias = "Ogre::DynLib::~DynLib()")]
// was: Ogre::DynLib::~DynLib()
// IDA 0xc80d10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c80d10() {
}

// 0xc80d5c — __ZN4Ogre6DynLib4loadEv
#[doc(alias = "Ogre::DynLib::load(void)")]
// was: Ogre::DynLib::load(void)
// IDA 0xc80d5c: 223 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80d5c() {
}

// 0xc812a0 — __ZN4Ogre6DynLib6unloadEv
#[doc(alias = "Ogre::DynLib::unload(void)")]
// was: Ogre::DynLib::unload(void)
// IDA 0xc812a0: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c812a0() {
}

// 0xc8141c — __ZNK4Ogre6DynLib9getSymbolERKSs
#[doc(alias = "Ogre::DynLib::getSymbol(std::string const&)const")]
// was: Ogre::DynLib::getSymbol(std::string const&)const
// IDA 0xc8141c: 51 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8141c() {
}

// 0xc814e4 — __ZN4Ogre13DynLibManager12getSingletonEv
#[doc(alias = "Ogre::DynLibManager::getSingleton(void)")]
// was: Ogre::DynLibManager::getSingleton(void)
// IDA 0xc814e4: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c814e4() {
}

// 0xc814f4 — __ZN4Ogre13DynLibManagerC1Ev
#[doc(alias = "Ogre::DynLibManager::DynLibManager(void)")]
// was: Ogre::DynLibManager::DynLibManager(void)
// IDA 0xc814f4: 22 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c814f4() {
}

// 0xc81538 — __ZN4Ogre13DynLibManager4loadERKSs
#[doc(alias = "Ogre::DynLibManager::load(std::string const&)")]
// was: Ogre::DynLibManager::load(std::string const&)
// IDA 0xc81538: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c81538() {
}

// 0xc8162c — __ZN4Ogre13DynLibManager6unloadEPNS_6DynLibE
#[doc(alias = "Ogre::DynLibManager::unload(Ogre::DynLib *)")]
// was: Ogre::DynLibManager::unload(Ogre::DynLib *)
// IDA 0xc8162c: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8162c() {
}

// 0xc81740 — __ZN4Ogre13DynLibManagerD0Ev
#[doc(alias = "Ogre::DynLibManager::~DynLibManager()")]
// was: Ogre::DynLibManager::~DynLibManager()
// IDA 0xc81740: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c81740() {
}

// 0xc817d0 — __ZN4Ogre13DynLibManagerD1Ev
#[doc(alias = "Ogre::DynLibManager::~DynLibManager()")]
// was: Ogre::DynLibManager::~DynLibManager()
// IDA 0xc817d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c817d0() {
}

// 0xc817dc — __ZN4Ogre13DynLibManagerD2Ev
#[doc(alias = "Ogre::DynLibManager::~DynLibManager()")]
// was: Ogre::DynLibManager::~DynLibManager()
// IDA 0xc817dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c817dc() {
}

// 0xc8193c — __ZNSt3mapISsPN4Ogre6DynLibESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(
    alias = "std::map<std::string,Ogre::DynLib *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)"
)]
// was: std::map<std::string,Ogre::DynLib *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xc8193c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8193c() {
}

// 0xc81af8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::DynLib *>> *)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::DynLib *>> *)
// IDA 0xc81af8: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c81af8() {
}

// 0xc81b70 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::DynLib *>>,std::pair<std::string const,Ogre::DynLib *> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::DynLib *>>,std::pair<std::string const,Ogre::DynLib *> const&)
// IDA 0xc81b70: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c81b70() {
}

// 0xc81d50 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::DynLib *> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::DynLib *> const&)
// IDA 0xc81d50: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c81d50() {
}

// 0xc81ea4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::DynLib *> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::DynLib *> const&)
// IDA 0xc81ea4: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c81ea4() {
}

// 0xc81f88 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xc81f88: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c81f88() {
}

// 0xc8202c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc8202c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8202c() {
}

// 0xc82030 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre6DynLibEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::DynLib *>,std::_Select1st<std::pair<std::string const,Ogre::DynLib *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::DynLib *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc82030: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c82030() {
}

// 0xc82070 — __ZN4Ogre15EdgeListBuilderC1Ev
#[doc(alias = "Ogre::EdgeListBuilder::EdgeListBuilder(void)")]
// was: Ogre::EdgeListBuilder::EdgeListBuilder(void)
// IDA 0xc82070: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c82070() {
}

// 0xc82118 — __ZN4Ogre15EdgeListBuilderD0Ev
#[doc(alias = "Ogre::EdgeListBuilder::~EdgeListBuilder()")]
// was: Ogre::EdgeListBuilder::~EdgeListBuilder()
// IDA 0xc82118: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c82118() {
}

// 0xc8212c — __ZN4Ogre15EdgeListBuilderD1Ev
#[doc(alias = "Ogre::EdgeListBuilder::~EdgeListBuilder()")]
// was: Ogre::EdgeListBuilder::~EdgeListBuilder()
// IDA 0xc8212c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8212c() {
}

// 0xc82138 — __ZN4Ogre15EdgeListBuilderD2Ev
#[doc(alias = "Ogre::EdgeListBuilder::~EdgeListBuilder()")]
// was: Ogre::EdgeListBuilder::~EdgeListBuilder()
// IDA 0xc82138: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c82138() {
}

// 0xc82224 — __ZN4Ogre15EdgeListBuilder13addVertexDataEPKNS_10VertexDataE
#[doc(alias = "Ogre::EdgeListBuilder::addVertexData(Ogre::VertexData const*)")]
// was: Ogre::EdgeListBuilder::addVertexData(Ogre::VertexData const*)
// IDA 0xc82224: 174 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c82224() {
}

// 0xc82420 — __ZN4Ogre15EdgeListBuilder12addIndexDataEPKNS_9IndexDataEmNS_15RenderOperation13OperationTypeE
#[doc(
    alias = "Ogre::EdgeListBuilder::addIndexData(Ogre::IndexData const*,unsigned long,Ogre::RenderOperation::OperationType)"
)]
// was: Ogre::EdgeListBuilder::addIndexData(Ogre::IndexData const*,unsigned long,Ogre::RenderOperation::OperationType)
// IDA 0xc82420: 183 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c82420() {
}

// 0xc82634 — __ZN4Ogre15EdgeListBuilder5buildEv
#[doc(alias = "Ogre::EdgeListBuilder::build(void)")]
// was: Ogre::EdgeListBuilder::build(void)
// IDA 0xc82634: 248 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c82634() {
}

// 0xc828dc — __ZN4Ogre15EdgeListBuilder19buildTrianglesEdgesERKNS0_8GeometryE
#[doc(alias = "Ogre::EdgeListBuilder::buildTrianglesEdges(Ogre::EdgeListBuilder::Geometry const&)")]
// was: Ogre::EdgeListBuilder::buildTrianglesEdges(Ogre::EdgeListBuilder::Geometry const&)
// IDA 0xc828dc: 529 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c828dc() {
}

// 0xc82db8 — __ZN4Ogre15EdgeListBuilder24findOrCreateCommonVertexERKNS_7Vector3Emmm
#[doc(
    alias = "Ogre::EdgeListBuilder::findOrCreateCommonVertex(Ogre::Vector3 const&,unsigned long,unsigned long,unsigned long)"
)]
// was: Ogre::EdgeListBuilder::findOrCreateCommonVertex(Ogre::Vector3 const&,unsigned long,unsigned long,unsigned long)
// IDA 0xc82db8: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c82db8() {
}

// 0xc82e68 — __ZN4Ogre15EdgeListBuilder19connectOrCreateEdgeEmmmmmm
#[doc(
    alias = "Ogre::EdgeListBuilder::connectOrCreateEdge(unsigned long,unsigned long,unsigned long,unsigned long,unsigned long,unsigned long)"
)]
// was: Ogre::EdgeListBuilder::connectOrCreateEdge(unsigned long,unsigned long,unsigned long,unsigned long,unsigned long,unsigned long)
// IDA 0xc82e68: 108 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c82e68() {
}

// 0xc82f80 — __ZN4Ogre8EdgeData25updateTriangleLightFacingERKNS_7Vector4E
#[doc(alias = "Ogre::EdgeData::updateTriangleLightFacing(Ogre::Vector4 const&)")]
// was: Ogre::EdgeData::updateTriangleLightFacing(Ogre::Vector4 const&)
// IDA 0xc82f80: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c82f80() {
}

// 0xc82fc4 — __ZN4Ogre8EdgeData17updateFaceNormalsEmRKNS_29HardwareVertexBufferSharedPtrE
#[doc(
    alias = "Ogre::EdgeData::updateFaceNormals(unsigned long,Ogre::HardwareVertexBufferSharedPtr const&)"
)]
// was: Ogre::EdgeData::updateFaceNormals(unsigned long,Ogre::HardwareVertexBufferSharedPtr const&)
// IDA 0xc82fc4: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c82fc4() {
}

// 0xc83028 — __ZNSt6vectorIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm
#[doc(
    alias = "std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)"
)]
// was: std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)
// IDA 0xc83028: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c83028() {
}

// 0xc830a4 — __ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE7reserveEm
#[doc(
    alias = "std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::reserve(unsigned long)"
)]
// was: std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::reserve(unsigned long)
// IDA 0xc830a4: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c830a4() {
}

// 0xc83120 — __ZNSt6vectorIN4Ogre15EdgeListBuilder12CommonVertexENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(
    alias = "std::vector<Ogre::EdgeListBuilder::CommonVertex,Ogre::STLAllocator<Ogre::EdgeListBuilder::CommonVertex,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::CommonVertex*,std::vector<Ogre::EdgeListBuilder::CommonVertex,Ogre::STLAllocator<Ogre::EdgeListBuilder::CommonVertex,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::CommonVertex const&)"
)]
// was: std::vector<Ogre::EdgeListBuilder::CommonVertex,Ogre::STLAllocator<Ogre::EdgeListBuilder::CommonVertex,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::CommonVertex*,std::vector<Ogre::EdgeListBuilder::CommonVertex,Ogre::STLAllocator<Ogre::EdgeListBuilder::CommonVertex,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::CommonVertex const&)
// IDA 0xc83120: 150 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c83120() {
}

// 0xc832f0 — __ZNSt8_Rb_treeIN4Ogre7Vector3ESt4pairIKS1_mESt10_Select1stIS4_ENS0_15EdgeListBuilder10vectorLessENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(
    alias = "std::_Rb_tree<Ogre::Vector3,std::pair<Ogre::Vector3 const,unsigned long>,std::_Select1st<std::pair<Ogre::Vector3 const,unsigned long>>,Ogre::EdgeListBuilder::vectorLess,Ogre::STLAllocator<std::pair<Ogre::Vector3 const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Vector3 const,unsigned long> const&)"
)]
// was: std::_Rb_tree<Ogre::Vector3,std::pair<Ogre::Vector3 const,unsigned long>,std::_Select1st<std::pair<Ogre::Vector3 const,unsigned long>>,Ogre::EdgeListBuilder::vectorLess,Ogre::STLAllocator<std::pair<Ogre::Vector3 const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Vector3 const,unsigned long> const&)
// IDA 0xc832f0: 167 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c832f0() {
}

// 0xc834f4 — __ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(
    alias = "std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EdgeData::Edge*,std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeData::Edge const&)"
)]
// was: std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EdgeData::Edge*,std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeData::Edge const&)
// IDA 0xc834f4: 145 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c834f4() {
}

// 0xc836b8 — __ZNSt8_Rb_treeISt4pairImmES0_IKS1_S1_ESt10_Select1stIS3_ESt4lessIS1_EN4Ogre12STLAllocatorIS3_NS8_22CategorisedAllocPolicyILNS8_14MemoryCategoryE0EEEEEE15_M_insert_equalERKS3_
#[doc(
    alias = "std::_Rb_tree<std::pair<unsigned long,unsigned long>,std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,std::_Select1st<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>>,std::less<std::pair<unsigned long,unsigned long>>,Ogre::STLAllocator<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_equal(std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>> const&)"
)]
// was: std::_Rb_tree<std::pair<unsigned long,unsigned long>,std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,std::_Select1st<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>>,std::less<std::pair<unsigned long,unsigned long>>,Ogre::STLAllocator<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_equal(std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>> const&)
// IDA 0xc836b8: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c836b8() {
}

// 0xc83758 — __ZNSt6vectorIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(
    alias = "std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EdgeData::Triangle*,std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeData::Triangle const&)"
)]
// was: std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EdgeData::Triangle*,std::vector<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeData::Triangle const&)
// IDA 0xc83758: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c83758() {
}

// 0xc838e8 — __ZNSt6vectorIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(
    alias = "std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector4*,std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>>,Ogre::Vector4 const&)"
)]
// was: std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector4*,std::vector<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>>,Ogre::Vector4 const&)
// IDA 0xc838e8: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c838e8() {
}

// 0xc83a30 — __ZNSt6vectorIcN4Ogre12STLAllocatorIcNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPcS6_EEmRKc
#[doc(
    alias = "std::vector<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<char *,std::vector<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,char const&)"
)]
// was: std::vector<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<char *,std::vector<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,char const&)
// IDA 0xc83a30: 146 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c83a30() {
}

// 0xc83ba4 — __ZNSt6vectorIN4Ogre8EdgeData9EdgeGroupENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(
    alias = "std::vector<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::EdgeData::EdgeGroup*,std::vector<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::EdgeData::EdgeGroup const&)"
)]
// was: std::vector<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::EdgeData::EdgeGroup*,std::vector<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::EdgeData::EdgeGroup const&)
// IDA 0xc83ba4: 291 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c83ba4() {
}

// 0xc83f40 — __ZSt22__uninitialized_copy_aIPN4Ogre8EdgeData9EdgeGroupES3_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_SA_S9_T1_
#[doc(
    alias = "Ogre::EdgeData::EdgeGroup * std::__uninitialized_copy_a<Ogre::EdgeData::EdgeGroup *,Ogre::EdgeData::EdgeGroup *,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::EdgeData::EdgeGroup *,Ogre::EdgeData::EdgeGroup *,Ogre::EdgeData::EdgeGroup *,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)"
)]
// was: Ogre::EdgeData::EdgeGroup * std::__uninitialized_copy_a<Ogre::EdgeData::EdgeGroup *,Ogre::EdgeData::EdgeGroup *,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::EdgeData::EdgeGroup *,Ogre::EdgeData::EdgeGroup *,Ogre::EdgeData::EdgeGroup *,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
// IDA 0xc83f40: 75 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c83f40() {
}

// 0xc84078 — __ZN4Ogre12STLAllocatorINS_8EdgeData9EdgeGroupENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev
#[doc(
    alias = "Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()"
)]
// was: Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xc84078: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c84078() {
}

// 0xc8407c — __ZSt24__uninitialized_fill_n_aIPN4Ogre8EdgeData9EdgeGroupEmS2_NS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEvT_T0_RKT1_T2_
#[doc(
    alias = "void std::__uninitialized_fill_n_a<Ogre::EdgeData::EdgeGroup *,unsigned long,Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::EdgeData::EdgeGroup *,unsigned long,Ogre::EdgeData::EdgeGroup const&,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)"
)]
// was: void std::__uninitialized_fill_n_a<Ogre::EdgeData::EdgeGroup *,unsigned long,Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::EdgeData::EdgeGroup *,unsigned long,Ogre::EdgeData::EdgeGroup const&,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
// IDA 0xc8407c: 66 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8407c() {
}

// 0xc84198 — __ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
#[doc(
    alias = "std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)"
)]
// was: std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xc84198: 111 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c84198() {
}

// 0xc842ec — __ZN4Ogre12STLAllocatorINS_8EdgeData9EdgeGroupENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(
    alias = "Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()"
)]
// was: Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xc842ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c842ec() {
}

// 0xc842f8 — __ZNSt6vectorIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_
#[doc(
    alias = "std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)"
)]
// was: std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xc842f8: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c842f8() {
}

// 0xc8438c — __ZNSt12_Vector_baseIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc8438c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8438c() {
}

// 0xc84390 — __ZNSt12_Vector_baseIN4Ogre8EdgeData4EdgeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::EdgeData::Edge,Ogre::STLAllocator<Ogre::EdgeData::Edge,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc84390: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c84390() {
}

// 0xc8439c — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS3_12geometryLessEEvT_SF_T0_T1_
#[doc(
    alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::EdgeListBuilder::geometryLess)"
)]
// was: void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::EdgeListBuilder::geometryLess)
// IDA 0xc8439c: 142 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8439c() {
}

// 0xc8450c — __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS3_12geometryLessEEvT_SF_T0_
#[doc(
    alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess)"
)]
// was: void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess)
// IDA 0xc8450c: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8450c() {
}

// 0xc84580 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS3_12geometryLessEEvT_SF_T0_
#[doc(
    alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess)"
)]
// was: void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess)
// IDA 0xc84580: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c84580() {
}

// 0xc84614 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS3_12geometryLessEEvT_SF_SF_T0_
#[doc(
    alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess)"
)]
// was: void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::geometryLess)
// IDA 0xc84614: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c84614() {
}

// 0xc846b0 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre15EdgeListBuilder8GeometryESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS4_NS3_12geometryLessEEvT_T0_SG_T1_T2_
#[doc(
    alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::EdgeListBuilder::Geometry,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::EdgeListBuilder::Geometry,Ogre::EdgeListBuilder::geometryLess)"
)]
// was: void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::EdgeListBuilder::Geometry,Ogre::EdgeListBuilder::geometryLess>(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry *,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::EdgeListBuilder::Geometry,Ogre::EdgeListBuilder::geometryLess)
// IDA 0xc846b0: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c846b0() {
}

// 0xc84790 — __ZNSt6vectorIN4Ogre15EdgeListBuilder8GeometryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(
    alias = "std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry*,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::Geometry const&)"
)]
// was: std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EdgeListBuilder::Geometry*,std::vector<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EdgeListBuilder::Geometry const&)
// IDA 0xc84790: 106 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c84790() {
}

// 0xc848b4 — __ZNSt6vectorIPKN4Ogre10VertexDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(
    alias = "std::vector<Ogre::VertexData const*,Ogre::STLAllocator<Ogre::VertexData const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::VertexData const**,std::vector<Ogre::VertexData const*,Ogre::STLAllocator<Ogre::VertexData const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::VertexData const* const&)"
)]
// was: std::vector<Ogre::VertexData const*,Ogre::STLAllocator<Ogre::VertexData const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::VertexData const**,std::vector<Ogre::VertexData const*,Ogre::STLAllocator<Ogre::VertexData const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::VertexData const* const&)
// IDA 0xc848b4: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c848b4() {
}

// 0xc849ac — __ZNSt12_Vector_baseIN4Ogre15EdgeListBuilder12CommonVertexENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::EdgeListBuilder::CommonVertex,Ogre::STLAllocator<Ogre::EdgeListBuilder::CommonVertex,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::EdgeListBuilder::CommonVertex,Ogre::STLAllocator<Ogre::EdgeListBuilder::CommonVertex,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc849ac: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c849ac() {
}

// 0xc849b0 — __ZNSt12_Vector_baseIPKN4Ogre10VertexDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::VertexData const*,Ogre::STLAllocator<Ogre::VertexData const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::VertexData const*,Ogre::STLAllocator<Ogre::VertexData const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc849b0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c849b0() {
}

// 0xc849b4 — __ZNSt12_Vector_baseIN4Ogre15EdgeListBuilder8GeometryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc849b4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c849b4() {
}

// 0xc849b8 — __ZNSt8_Rb_treeISt4pairImmES0_IKS1_S1_ESt10_Select1stIS3_ESt4lessIS1_EN4Ogre12STLAllocatorIS3_NS8_22CategorisedAllocPolicyILNS8_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS7_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<std::pair<unsigned long,unsigned long>,std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,std::_Select1st<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>>,std::less<std::pair<unsigned long,unsigned long>>,Ogre::STLAllocator<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::pair<unsigned long,unsigned long>>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::pair<unsigned long,unsigned long>,std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,std::_Select1st<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>>,std::less<std::pair<unsigned long,unsigned long>>,Ogre::STLAllocator<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::pair<unsigned long,unsigned long>>,false>::~_Rb_tree_impl()
// IDA 0xc849b8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c849b8() {
}

// 0xc849bc — __ZNSt8_Rb_treeISt4pairImmES0_IKS1_S1_ESt10_Select1stIS3_ESt4lessIS1_EN4Ogre12STLAllocatorIS3_NS8_22CategorisedAllocPolicyILNS8_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS7_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<std::pair<unsigned long,unsigned long>,std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,std::_Select1st<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>>,std::less<std::pair<unsigned long,unsigned long>>,Ogre::STLAllocator<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::pair<unsigned long,unsigned long>>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::pair<unsigned long,unsigned long>,std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,std::_Select1st<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>>,std::less<std::pair<unsigned long,unsigned long>>,Ogre::STLAllocator<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::pair<unsigned long,unsigned long>>,false>::~_Rb_tree_impl()
// IDA 0xc849bc: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c849bc() {
}

// 0xc849c8 — __ZNSt8_Rb_treeIN4Ogre7Vector3ESt4pairIKS1_mESt10_Select1stIS4_ENS0_15EdgeListBuilder10vectorLessENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<Ogre::Vector3,std::pair<Ogre::Vector3 const,unsigned long>,std::_Select1st<std::pair<Ogre::Vector3 const,unsigned long>>,Ogre::EdgeListBuilder::vectorLess,Ogre::STLAllocator<std::pair<Ogre::Vector3 const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<Ogre::EdgeListBuilder::vectorLess,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<Ogre::Vector3,std::pair<Ogre::Vector3 const,unsigned long>,std::_Select1st<std::pair<Ogre::Vector3 const,unsigned long>>,Ogre::EdgeListBuilder::vectorLess,Ogre::STLAllocator<std::pair<Ogre::Vector3 const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<Ogre::EdgeListBuilder::vectorLess,false>::~_Rb_tree_impl()
// IDA 0xc849c8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c849c8() {
}

// 0xc849cc — __ZNSt8_Rb_treeIN4Ogre7Vector3ESt4pairIKS1_mESt10_Select1stIS4_ENS0_15EdgeListBuilder10vectorLessENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<Ogre::Vector3,std::pair<Ogre::Vector3 const,unsigned long>,std::_Select1st<std::pair<Ogre::Vector3 const,unsigned long>>,Ogre::EdgeListBuilder::vectorLess,Ogre::STLAllocator<std::pair<Ogre::Vector3 const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<Ogre::EdgeListBuilder::vectorLess,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<Ogre::Vector3,std::pair<Ogre::Vector3 const,unsigned long>,std::_Select1st<std::pair<Ogre::Vector3 const,unsigned long>>,Ogre::EdgeListBuilder::vectorLess,Ogre::STLAllocator<std::pair<Ogre::Vector3 const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<Ogre::EdgeListBuilder::vectorLess,false>::~_Rb_tree_impl()
// IDA 0xc849cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c849cc() {
}

// 0xc849d8 — __ZNSt12_Vector_baseIN4Ogre15EdgeListBuilder12CommonVertexENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::EdgeListBuilder::CommonVertex,Ogre::STLAllocator<Ogre::EdgeListBuilder::CommonVertex,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::EdgeListBuilder::CommonVertex,Ogre::STLAllocator<Ogre::EdgeListBuilder::CommonVertex,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc849d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c849d8() {
}

// 0xc849e4 — __ZNSt12_Vector_baseIPKN4Ogre10VertexDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::VertexData const*,Ogre::STLAllocator<Ogre::VertexData const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::VertexData const*,Ogre::STLAllocator<Ogre::VertexData const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc849e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c849e4() {
}

// 0xc849f0 — __ZNSt12_Vector_baseIN4Ogre15EdgeListBuilder8GeometryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::EdgeListBuilder::Geometry,Ogre::STLAllocator<Ogre::EdgeListBuilder::Geometry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc849f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c849f0() {
}

// 0xc849fc — __ZNSt12_Vector_baseIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc849fc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c849fc() {
}

// 0xc84a00 — __ZNSt12_Vector_baseIN4Ogre8EdgeData9EdgeGroupENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc84a00: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c84a00() {
}

// 0xc84a04 — __ZNSt12_Vector_baseIcN4Ogre12STLAllocatorIcNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc84a04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c84a04() {
}

// 0xc84a10 — __ZNSt12_Vector_baseIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc84a10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c84a10() {
}

// 0xc84a1c — __ZNSt8_Rb_treeISt4pairImmES0_IKS1_S1_ESt10_Select1stIS3_ESt4lessIS1_EN4Ogre12STLAllocatorIS3_NS8_22CategorisedAllocPolicyILNS8_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E
#[doc(
    alias = "std::_Rb_tree<std::pair<unsigned long,unsigned long>,std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,std::_Select1st<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>>,std::less<std::pair<unsigned long,unsigned long>>,Ogre::STLAllocator<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>> *)"
)]
// was: std::_Rb_tree<std::pair<unsigned long,unsigned long>,std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,std::_Select1st<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>>,std::less<std::pair<unsigned long,unsigned long>>,Ogre::STLAllocator<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::pair<unsigned long,unsigned long> const,std::pair<unsigned long,unsigned long>>> *)
// IDA 0xc84a1c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c84a1c() {
}

// 0xc84a44 — __ZNSt8_Rb_treeIN4Ogre7Vector3ESt4pairIKS1_mESt10_Select1stIS4_ENS0_15EdgeListBuilder10vectorLessENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(
    alias = "std::_Rb_tree<Ogre::Vector3,std::pair<Ogre::Vector3 const,unsigned long>,std::_Select1st<std::pair<Ogre::Vector3 const,unsigned long>>,Ogre::EdgeListBuilder::vectorLess,Ogre::STLAllocator<std::pair<Ogre::Vector3 const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Vector3 const,unsigned long>> *)"
)]
// was: std::_Rb_tree<Ogre::Vector3,std::pair<Ogre::Vector3 const,unsigned long>,std::_Select1st<std::pair<Ogre::Vector3 const,unsigned long>>,Ogre::EdgeListBuilder::vectorLess,Ogre::STLAllocator<std::pair<Ogre::Vector3 const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Vector3 const,unsigned long>> *)
// IDA 0xc84a44: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c84a44() {
}

// 0xc84aa0 — __ZN4Ogre6EntityC2Ev
#[doc(alias = "Ogre::Entity::Entity(void)")]
// was: Ogre::Entity::Entity(void)
// IDA 0xc84aa0: 183 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c84aa0() {
}

// 0xc84d5c — __ZN4Ogre6EntityC2ERKSsRKNS_7MeshPtrE
#[doc(alias = "Ogre::Entity::Entity(std::string const&,Ogre::MeshPtr const&)")]
// was: Ogre::Entity::Entity(std::string const&,Ogre::MeshPtr const&)
// IDA 0xc84d5c: 446 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c84d5c() {
}

// 0xc85294 — __ZN4Ogre6Entity11_initialiseEb
#[doc(alias = "Ogre::Entity::_initialise(bool)")]
// was: Ogre::Entity::_initialise(bool)
// IDA 0xc85294: 530 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c85294() {
}

// 0xc8585c — __ZN4Ogre6Entity25backgroundLoadingCompleteEPNS_8ResourceE
#[doc(alias = "Ogre::Entity::backgroundLoadingComplete(Ogre::Resource *)")]
// was: Ogre::Entity::backgroundLoadingComplete(Ogre::Resource *)
// IDA 0xc8585c: `if (*(this+49) == resource) _initialise(this, false)` —
// decompile shows the pending-mesh compare at `0xc85866` and the
// `_initialise(entity, 0)` call at `0xc8586c`; a mismatch returns as-is.
pub fn stub_c8585c(entity: &mut crate::movable::Entity, resource_mesh: &str) -> bool {
    entity.background_loading_complete(resource_mesh)
}

// 0xc85874 — __ZThn188_N4Ogre6Entity25backgroundLoadingCompleteEPNS_8ResourceE
#[doc(alias = "non-virtual thunk toOgre::Entity::backgroundLoadingComplete(Ogre::Resource *)")]
// was: non-virtual thunk to Ogre::Entity::backgroundLoadingComplete(Ogre::Resource *)
// IDA 0xc85874: non-virtual thunk — `this - 188` adjustor (`0xc85880`), then
// the same pending-resource check and `_initialise(entity, false)` at
// `0xc85884`. Rust has no `Resource` base subobject, so the adjustor is a
// documented no-op and control flow matches the decompiled branches.
pub fn stub_c85874(entity: &mut crate::movable::Entity, resource_mesh: &str) -> bool {
    entity.background_loading_complete(resource_mesh)
}

// 0xc8588c — __ZN4Ogre6Entity13_deinitialiseEv
#[doc(alias = "Ogre::Entity::_deinitialise(void)")]
// was: Ogre::Entity::_deinitialise(void)
// IDA 0xc8588c: 271 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8588c() {
}

// 0xc85b98 — __ZN4Ogre6Entity18buildSubEntityListERNS_7MeshPtrEPSt6vectorIPNS_9SubEntityENS_12STLAllocatorIS5_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(
    alias = "Ogre::Entity::buildSubEntityList(Ogre::MeshPtr &,std::vector<Ogre::SubEntity *,Ogre::STLAllocator<Ogre::SubEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *)"
)]
// was: Ogre::Entity::buildSubEntityList(Ogre::MeshPtr &,std::vector<Ogre::SubEntity *,Ogre::STLAllocator<Ogre::SubEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *)
// IDA 0xc85b98: 134 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c85b98() {
}

// 0xc85cf8 — __ZN4Ogre6Entity23prepareTempBlendBuffersEv
#[doc(alias = "Ogre::Entity::prepareTempBlendBuffers(void)")]
// was: Ogre::Entity::prepareTempBlendBuffers(void)
// IDA 0xc85cf8: 139 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c85cf8() {
}

// 0xc85e98 — __ZN4Ogre6Entity26reevaluateVertexProcessingEv
#[doc(alias = "Ogre::Entity::reevaluateVertexProcessing(void)")]
// was: Ogre::Entity::reevaluateVertexProcessing(void)
// IDA 0xc85e98: `_M_erase` of the `ushort -> bool` request map at `+424`
// (`0xc85ea6`), then root/sentinel reset (`0xc85eb0..0xc85ec0`).
pub fn stub_c85e98(entity: &mut crate::movable::Entity) {
    entity.reevaluate_vertex_processing()
}

// 0xc85ec4 — __ZN4Ogre6Entity20detachAllObjectsImplEv
#[doc(alias = "Ogre::Entity::detachAllObjectsImpl(void)")]
// was: Ogre::Entity::detachAllObjectsImpl(void)
// IDA 0xc85ec4: walk the child map (`0xc85ed2..0xc85ef6`, one
// `freeTagPoint` per entry), `_M_erase` the map at `+584` and reset its
// sentinels (`0xc85f0c..0xc85f1e`).
pub fn stub_c85ec4(entity: &mut crate::movable::Entity) {
    entity.detach_all_objects_impl()
}

// 0xc85f28 — __ZN4Ogre6Entity27stopSharingSkeletonInstanceEv
#[doc(alias = "Ogre::Entity::stopSharingSkeletonInstance(void)")]
// was: Ogre::Entity::stopSharingSkeletonInstance(void)
// IDA 0xc85f28: 307 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c85f28() {
}

// 0xc86288 — __ZN4Ogre6EntityD0Ev
#[doc(alias = "Ogre::Entity::~Entity()")]
// was: Ogre::Entity::~Entity()
// IDA 0xc86288: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c86288() {
}

// 0xc86318 — __ZN4Ogre6EntityD1Ev
#[doc(alias = "Ogre::Entity::~Entity()")]
// was: Ogre::Entity::~Entity()
// IDA 0xc86318: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c86318() {
}

// 0xc86324 — __ZThn4_N4Ogre6EntityD0Ev
#[doc(alias = "non-virtual thunk toOgre::Entity::~Entity()")]
// was: non-virtual thunk to Ogre::Entity::~Entity()
// IDA 0xc86324: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c86324() {
}

// 0xc863b8 — __ZThn188_N4Ogre6EntityD0Ev
#[doc(alias = "non-virtual thunk toOgre::Entity::~Entity()")]
// was: non-virtual thunk to Ogre::Entity::~Entity()
// IDA 0xc863b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c863b8() {
}

// 0xc8644c — __ZN4Ogre6EntityD2Ev
#[doc(alias = "Ogre::Entity::~Entity()")]
// was: Ogre::Entity::~Entity()
// IDA 0xc8644c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8644c() {
}

// 0xc86750 — __ZThn4_N4Ogre6EntityD1Ev
#[doc(alias = "non-virtual thunk toOgre::Entity::~Entity()")]
// was: non-virtual thunk to Ogre::Entity::~Entity()
// IDA 0xc86750: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c86750() {
}

// 0xc8675c — __ZThn188_N4Ogre6EntityD1Ev
#[doc(alias = "non-virtual thunk toOgre::Entity::~Entity()")]
// was: non-virtual thunk to Ogre::Entity::~Entity()
// IDA 0xc8675c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8675c() {
}

// 0xc86768 — __ZNK4Ogre6Entity7getMeshEv
// IDA 0xc86768: `ADDS R0,#0xC0; BX LR` — returns the mesh word at `+192`
// (disasm confirms the single-add offset past the `MovableObject` base).
pub fn stub_c86768(entity: &crate::movable::Entity) -> &str {
    entity.mesh_name()
}

// 0xc8676c — __ZNK4Ogre6Entity12getSubEntityEj
// IDA 0xc8676c: `(end - begin) >> 2 <= index` guard (`0xc867c2`); in range
// returns `begin[index]` (`0xc867e4`), else throws
// `InvalidParametersException` ("Index out of bounds.",
// `Entity::getSubEntity`, `OgreEntity.cpp:324` via `__cxa_throw`).
pub fn stub_c8676c(
    entity: &crate::movable::Entity,
    index: u32,
) -> Result<&crate::movable::SubEntity, crate::movable::OgreException> {
    entity.sub_entity(index)
}

// 0xc86950 — __ZNK4Ogre6Entity17getNumSubEntitiesEv
// IDA 0xc86958: `(end - begin) >> 2` — word-count of the sub-entity vector
// at `+212`.
pub fn stub_c86950(entity: &crate::movable::Entity) -> usize {
    entity.num_sub_entities()
}

// 0xc8695c — __ZN4Ogre6Entity15setMaterialNameERKSsS2_
// IDA 0xc86964..0xc86988: iterate `[begin, end)` at `+212` and call
// `SubEntity::setMaterialName` on each slot (`0xc8697e`).
pub fn stub_c8695c(entity: &mut crate::movable::Entity, name: &str, group: &str) {
    entity.set_material_name(name, group)
}

// 0xc86990 — __ZN4Ogre6Entity20_notifyCurrentCameraEPNS_6CameraE
#[doc(alias = "Ogre::Entity::_notifyCurrentCamera(Ogre::Camera *)")]
// was: Ogre::Entity::_notifyCurrentCamera(Ogre::Camera *)
// IDA 0xc86990: 126 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c86990() {
}

// 0xc86b10 — __ZNK4Ogre6Entity14getBoundingBoxEv
#[doc(alias = "Ogre::Entity::getBoundingBox(void)const")]
// was: Ogre::Entity::getBoundingBox(void)const
// IDA 0xc86b10: when the mesh is bounds-aware (`0xc86b6c`), copy
// `Mesh::getBounds` into the cache at `+612` (`0xc86ba4..0xc86bc2`), fold
// in `getChildObjectsBoundingBox` (`0xc86bd0`) with a component-wise
// min/max merge (`0xc86bf8..0xc86c90`).
pub fn stub_c86b10(entity: &mut crate::movable::Entity) -> crate::movable::AxisAlignedBox {
    entity.bounding_box()
}

// 0xc86d08 — __ZNK4Ogre6Entity26getChildObjectsBoundingBoxEv
#[doc(alias = "Ogre::Entity::getChildObjectsBoundingBox(void)const")]
// was: Ogre::Entity::getChildObjectsBoundingBox(void)const
// IDA 0xc86d08: seed a null box (`0xc86d98..0xc86da6`), walk the child map
// (`0xc86de8`), transform each child box by its tag-point matrix
// (`0xc86e40..0xc86e48`) and merge (`0xc86ed0..0xc86f3e`); null when empty.
pub fn stub_c86d08(entity: &crate::movable::Entity) -> crate::movable::AxisAlignedBox {
    entity.child_objects_bounding_box()
}

// 0xc86fbc — __ZNK4Ogre6Entity19getWorldBoundingBoxEb
#[doc(alias = "Ogre::Entity::getWorldBoundingBox(bool)const")]
// was: Ogre::Entity::getWorldBoundingBox(bool)const
// IDA 0xc86fbc: with `derive=true`, refresh every child object first
// (`0xc86fce..0xc86fec` update loop), then take the `MovableObject` world
// box (`0xc86ffa` tail call).
pub fn stub_c86fbc(entity: &mut crate::movable::Entity, derive: bool) -> crate::movable::AxisAlignedBox {
    entity.world_bounding_box(derive)
}

// 0xc86ffc — __ZNK4Ogre6Entity22getWorldBoundingSphereEb
#[doc(alias = "Ogre::Entity::getWorldBoundingSphere(bool)const")]
// was: Ogre::Entity::getWorldBoundingSphere(bool)const
// IDA 0xc86ffc: sphere twin of `getWorldBoundingBox` — same
// derive-refresh loop (`0xc8700e..0xc8702c`), then the `MovableObject`
// world sphere (`0xc8703a` tail call).
pub fn stub_c86ffc(entity: &mut crate::movable::Entity, derive: bool) -> crate::movable::BoundingSphere {
    entity.world_bounding_sphere(derive)
}

// 0xc8703c — __ZN4Ogre6Entity18_updateRenderQueueEPNS_11RenderQueueE
#[doc(alias = "Ogre::Entity::_updateRenderQueue(Ogre::RenderQueue *)")]
// was: Ogre::Entity::_updateRenderQueue(Ogre::RenderQueue *)
// IDA 0xc8703c: 277 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8703c() {
}

// 0xc87320 — __ZN4Ogre6Entity15updateAnimationEv
#[doc(alias = "Ogre::Entity::updateAnimation(void)")]
// was: Ogre::Entity::updateAnimation(void)
// IDA 0xc87320: 512 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c87320() {
}

// 0xc8790c — __ZNK4Ogre6Entity21getAllAnimationStatesEv
#[doc(alias = "Ogre::Entity::getAllAnimationStates(void)const")]
// was: Ogre::Entity::getAllAnimationStates(void)const
// IDA 0xc8790c: `LDR.W R0,[R0,#0xE0]; BX LR` — animation-state set word at
// `+224`.
pub fn stub_c8790c(entity: &crate::movable::Entity) -> &[String] {
    entity.all_animation_states()
}

// 0xc87914 — __ZNK4Ogre6Entity14getMovableTypeEv
#[doc(alias = "Ogre::Entity::getMovableType(void)const")]
// was: Ogre::Entity::getMovableType(void)const
// IDA 0xc87914: `MOV R0,=FACTORY_TYPE_NAME; ADD R0,PC` (disasm confirms
// the PC-relative load of `EntityFactory::FACTORY_TYPE_NAME`).
pub fn stub_c87914() -> &'static str {
    crate::movable::Entity::movable_type()
}

// 0xc87920 — __ZNK4Ogre6Entity26tempVertexAnimBuffersBoundEv
#[doc(alias = "Ogre::Entity::tempVertexAnimBuffersBound(void)const")]
// was: Ogre::Entity::tempVertexAnimBuffersBound(void)const
// IDA 0xc87920: skeleton-level checkout first (`0xc8792c..0xc87954`), then
// per-sub-entity: skip unanimated slots, fail when an animated slot lacks
// its checkout (`0xc87960..0xc879ca`).
pub fn stub_c87920(entity: &crate::movable::Entity) -> bool {
    entity.temp_vertex_anim_buffers_bound()
}

// 0xc879cc — __ZN4Ogre6Entity26isHardwareAnimationEnabledEv
#[doc(alias = "Ogre::Entity::isHardwareAnimationEnabled(void)")]
// was: Ogre::Entity::isHardwareAnimationEnabled(void)
// IDA 0xc879cc: scheme-map lookup against the material manager value
// (`0xc879d4..0xc87a00`); on a miss re-run `calcVertexProcessing`
// (`0xc87a16`) and record a fresh entry (`0xc87a2a`), then return the
// cached enable byte (`0xc87a34`).
pub fn stub_c879cc(entity: &mut crate::movable::Entity) -> bool {
    entity.is_hardware_animation_enabled()
}

// 0xc87a38 — __ZN4Ogre6Entity20applyVertexAnimationEbb
#[doc(alias = "Ogre::Entity::applyVertexAnimation(bool,bool)")]
// was: Ogre::Entity::applyVertexAnimation(bool,bool)
// IDA 0xc87a38: 1064 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c87a38() {
}

// 0xc884f0 — __ZNK4Ogre6Entity19_isSkeletonAnimatedEv
#[doc(alias = "Ogre::Entity::_isSkeletonAnimated(void)const")]
// was: Ogre::Entity::_isSkeletonAnimated(void)const
// IDA 0xc884f0: no skeleton yields 0 (`0xc884fc`); with one the answer is
// 1 unless the state set is empty, in which case the skeleton's own
// vtable predicate decides (`0xc8850a..0xc88514`).
pub fn stub_c884f0(entity: &crate::movable::Entity) -> bool {
    entity.is_skeleton_animated()
}

// 0xc8851c — __ZN4Ogre6Entity24initialisePoseVertexDataEPKNS_10VertexDataEPS1_b
#[doc(
    alias = "Ogre::Entity::initialisePoseVertexData(Ogre::VertexData const*,Ogre::VertexData*,bool)"
)]
// was: Ogre::Entity::initialisePoseVertexData(Ogre::VertexData const*,Ogre::VertexData*,bool)
// IDA 0xc8851c: 471 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8851c() {
}

// 0xc88950 — __ZN4Ogre6Entity32restoreBuffersForUnusedAnimationEb
#[doc(alias = "Ogre::Entity::restoreBuffersForUnusedAnimation(bool)")]
// was: Ogre::Entity::restoreBuffersForUnusedAnimation(bool)
// IDA 0xc88950: 241 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c88950() {
}

// 0xc88bc4 — __ZN4Ogre6Entity19finalisePoseNormalsEPKNS_10VertexDataEPS1_
#[doc(alias = "Ogre::Entity::finalisePoseNormals(Ogre::VertexData const*,Ogre::VertexData*)")]
// was: Ogre::Entity::finalisePoseNormals(Ogre::VertexData const*,Ogre::VertexData*)
// IDA 0xc88bc4: 408 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c88bc4() {
}

// 0xc88fc4 — __ZN4Ogre6Entity28_markBuffersUsedForAnimationEv
#[doc(alias = "Ogre::Entity::_markBuffersUsedForAnimation(void)")]
// was: Ogre::Entity::_markBuffersUsedForAnimation(void)
// IDA 0xc88fc4: `MOVS R1,#1; STRB.W R1,[R0,#0x188]; BX LR` (disasm
// confirms the byte store at `+392`).
pub fn stub_c88fc4(entity: &mut crate::movable::Entity) {
    entity.mark_buffers_used_for_animation()
}

// 0xc88fcc — __ZN4Ogre6Entity30bindMissingHardwarePoseBuffersEPKNS_10VertexDataEPS1_
#[doc(
    alias = "Ogre::Entity::bindMissingHardwarePoseBuffers(Ogre::VertexData const*,Ogre::VertexData*)"
)]
// was: Ogre::Entity::bindMissingHardwarePoseBuffers(Ogre::VertexData const*,Ogre::VertexData*)
// IDA 0xc88fcc: 195 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c88fcc() {
}

// 0xc891a4 — __ZNK4Ogre6Entity32_getSoftwareVertexAnimVertexDataEv
#[doc(alias = "Ogre::Entity::_getSoftwareVertexAnimVertexData(void)const")]
// was: Ogre::Entity::_getSoftwareVertexAnimVertexData(void)const
// IDA 0xc891a4: `LDR.W R0,[R0,#0x180]; BX LR` — vertex-data word at `+384`.
pub fn stub_c891a4(entity: &crate::movable::Entity) -> Option<usize> {
    entity.software_vertex_anim_data()
}

// 0xc891ac — __ZNK4Ogre6Entity32_getHardwareVertexAnimVertexDataEv
#[doc(alias = "Ogre::Entity::_getHardwareVertexAnimVertexData(void)const")]
// was: Ogre::Entity::_getHardwareVertexAnimVertexData(void)const
// IDA 0xc891ac: `LDR.W R0,[R0,#0x184]; BX LR` — vertex-data word at `+388`.
pub fn stub_c891ac(entity: &crate::movable::Entity) -> Option<usize> {
    entity.hardware_vertex_anim_data()
}

// 0xc891b4 — __ZN4Ogre6Entity20detachObjectFromBoneEPNS_13MovableObjectE
#[doc(alias = "Ogre::Entity::detachObjectFromBone(Ogre::MovableObject *)")]
// was: Ogre::Entity::detachObjectFromBone(Ogre::MovableObject *)
// IDA 0xc891b4: 69 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c891b4() {
}

// 0xc8926c — __ZNK4Ogre6Entity17getBoundingRadiusEv
#[doc(alias = "Ogre::Entity::getBoundingRadius(void)const")]
// was: Ogre::Entity::getBoundingRadius(void)const
// IDA 0xc8926c: tail call `Mesh::getBoundingSphereRadius(*(this + 49))`
// (`0xc89278`) — the radius lives on the mesh, not the entity.
pub fn stub_c8926c(entity: &crate::movable::Entity) -> f32 {
    entity.bounding_radius()
}

// 0xc8927c — __ZN4Ogre6Entity21extractTempBufferInfoEPNS_10VertexDataEPNS_21TempBlendedBufferInfoE
#[doc(
    alias = "Ogre::Entity::extractTempBufferInfo(Ogre::VertexData *,Ogre::TempBlendedBufferInfo *)"
)]
// was: Ogre::Entity::extractTempBufferInfo(Ogre::VertexData *,Ogre::TempBlendedBufferInfo *)
// IDA 0xc8927c: single tail call `TempBlendedBufferInfo::extractFrom(a3,
// a2)` (`0xc89286`); `this` is unused.
pub fn stub_c8927c(info: &mut crate::movable::TempBlendedBufferInfo, vertex_data_id: usize) {
    info.extract_from(vertex_data_id)
}

// 0xc89288 — __ZN4Ogre6Entity30cloneVertexDataRemoveBlendInfoEPKNS_10VertexDataE
#[doc(alias = "Ogre::Entity::cloneVertexDataRemoveBlendInfo(Ogre::VertexData const*)")]
// was: Ogre::Entity::cloneVertexDataRemoveBlendInfo(Ogre::VertexData const*)
// IDA 0xc89288: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c89288() {
}

// 0xc8933c — __ZN4Ogre6Entity11getEdgeListEv
#[doc(alias = "Ogre::Entity::getEdgeList(void)")]
// was: Ogre::Entity::getEdgeList(void)
// IDA 0xc8933c: `Mesh::getEdgeList(mesh, lodIndex)` with the LOD word at
// `+470` (`0xc8934c`); null when the mesh has no edge list.
pub fn stub_c8933c(entity: &crate::movable::Entity) -> Option<u16> {
    entity.edge_list_lod()
}

// 0xc89350 — __ZN4Ogre6Entity11hasEdgeListEv
#[doc(alias = "Ogre::Entity::hasEdgeList(void)")]
// was: Ogre::Entity::hasEdgeList(void)
// IDA 0xc89350: `Mesh::getEdgeList(...) != 0` (`0xc89366`).
pub fn stub_c89350(entity: &crate::movable::Entity) -> bool {
    entity.has_edge_list()
}

// 0xc89368 — __ZN4Ogre6Entity20calcVertexProcessingEv
#[doc(alias = "Ogre::Entity::calcVertexProcessing(void)")]
// was: Ogre::Entity::calcVertexProcessing(void)
// IDA 0xc89368 (306 insns): decides hardware vs. software vertex
// processing from the skeleton binding and per-sub-mesh animation types;
// morph slots force the software path.
pub fn stub_c89368(entity: &mut crate::movable::Entity) {
    entity.calc_vertex_processing()
}

// 0xc89684 — __ZN4Ogre6Entity33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm
#[doc(
    alias = "Ogre::Entity::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)"
)]
// was: Ogre::Entity::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)
// IDA 0xc89684: 553 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c89684() {
}

// 0xc89c68 — __ZN4Ogre6Entity21findBlendedVertexDataEPKNS_10VertexDataE
#[doc(alias = "Ogre::Entity::findBlendedVertexData(Ogre::VertexData const*)")]
// was: Ogre::Entity::findBlendedVertexData(Ogre::VertexData const*)
// IDA 0xc89c68: 188 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c89c68() {
}

// 0xc89e9c — __ZN4Ogre6Entity15_notifyAttachedEPNS_4NodeEb
#[doc(alias = "Ogre::Entity::_notifyAttached(Ogre::Node *,bool)")]
// was: Ogre::Entity::_notifyAttached(Ogre::Node *,bool)
// IDA 0xc89e9c: `MovableObject::_notifyAttached(this, node, attached)`
// first (`0xc89eaa`), then propagate to every object in the child map
// (`0xc89eb8..0xc89ed0` vtable call `+88` per entry).
pub fn stub_c89e9c(entity: &mut crate::movable::Entity, attached: bool) {
    entity.notify_attached(attached)
}

// 0xc89ed4 — __ZN4Ogre6Entity22EntityShadowRenderableC2EPS0_PNS_28HardwareIndexBufferSharedPtrEPKNS_10VertexDataEbPNS_9SubEntityEb
#[doc(
    alias = "Ogre::Entity::EntityShadowRenderable::EntityShadowRenderable(Ogre::Entity*,Ogre::HardwareIndexBufferSharedPtr *,Ogre::VertexData const*,bool,Ogre::SubEntity *,bool)"
)]
// was: Ogre::Entity::EntityShadowRenderable::EntityShadowRenderable(Ogre::Entity*,Ogre::HardwareIndexBufferSharedPtr *,Ogre::VertexData const*,bool,Ogre::SubEntity *,bool)
// IDA 0xc89ed4: 463 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c89ed4() {
}

// 0xc8a344 — __ZN4Ogre6Entity22EntityShadowRenderableD0Ev
#[doc(alias = "Ogre::Entity::EntityShadowRenderable::~EntityShadowRenderable()")]
// was: Ogre::Entity::EntityShadowRenderable::~EntityShadowRenderable()
// IDA 0xc8a344: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8a344() {
}

// 0xc8a3d4 — __ZN4Ogre6Entity22EntityShadowRenderableD1Ev
#[doc(alias = "Ogre::Entity::EntityShadowRenderable::~EntityShadowRenderable()")]
// was: Ogre::Entity::EntityShadowRenderable::~EntityShadowRenderable()
// IDA 0xc8a3d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8a3d4() {
}

// 0xca1580 — __ZN4Ogre17GpuProgramManager26canGetCompiledShaderBufferEv
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this)
#[doc(alias = "Ogre::GpuProgramManager::canGetCompiledShaderBuffer(void)")]
// was: Ogre::GpuProgramManager::canGetCompiledShaderBuffer(void)
// IDA 0xca1580: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca1580() {
}

// 0xca159c — __ZN4Ogre17GpuProgramManager21addRenderSystemToNameERKSs
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgramManager::addRenderSystemToName(std::string const&)")]
// was: Ogre::GpuProgramManager::addRenderSystemToName(std::string const&)
// IDA 0xca159c: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca159c() {
}

// 0xca1764 — __ZNK4Ogre17GpuProgramManager27isMicrocodeAvailableInCacheERKSs
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgramManager::isMicrocodeAvailableInCache(std::string const&)const")]
// was: Ogre::GpuProgramManager::isMicrocodeAvailableInCache(std::string const&)const
// IDA 0xca1764: 41 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca1764() {
}

// 0xca17d8 — __ZNK4Ogre17GpuProgramManager21getMicrocodeFromCacheERKSs
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgramManager::getMicrocodeFromCache(std::string const&)const")]
// was: Ogre::GpuProgramManager::getMicrocodeFromCache(std::string const&)const
// IDA 0xca17d8: 35 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca17d8() {
}

// 0xca183c — __ZNK4Ogre17GpuProgramManager15createMicrocodeEm
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this, unsigned int)
#[doc(alias = "Ogre::GpuProgramManager::createMicrocode(unsigned long)const")]
// was: Ogre::GpuProgramManager::createMicrocode(unsigned long)const
// IDA 0xca183c: 96 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca183c() {
}

// 0xca193c — __ZN4Ogre17GpuProgramManager19addMicrocodeToCacheERKSsRKNS_9SharedPtrINS_16MemoryDataStreamEEE
#[doc(
    alias = "Ogre::GpuProgramManager::addMicrocodeToCache(std::string const&,Ogre::SharedPtr<Ogre::MemoryDataStream> const&)"
)]
// was: Ogre::GpuProgramManager::addMicrocodeToCache(std::string const&,Ogre::SharedPtr<Ogre::MemoryDataStream> const&)
// IDA 0xca193c: 562 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca193c() {
}

// 0xca1ef0 — __ZNK4Ogre17GpuProgramManager18saveMicrocodeCacheENS_9SharedPtrINS_10DataStreamEEE
#[doc(
    alias = "Ogre::GpuProgramManager::saveMicrocodeCache(Ogre::SharedPtr<Ogre::DataStream>)const"
)]
// was: Ogre::GpuProgramManager::saveMicrocodeCache(Ogre::SharedPtr<Ogre::DataStream>)const
// IDA 0xca1ef0: 253 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca1ef0() {
}

// 0xca21a8 — __ZN4Ogre17GpuProgramManager18loadMicrocodeCacheENS_9SharedPtrINS_10DataStreamEEE
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, char, char, char, char, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::GpuProgramManager::loadMicrocodeCache(Ogre::SharedPtr<Ogre::DataStream>)")]
// was: Ogre::GpuProgramManager::loadMicrocodeCache(Ogre::SharedPtr<Ogre::DataStream>)
// IDA 0xca21a8: 663 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca21a8() {
}

// 0xca2830 — __ZNSt3mapISsN4Ogre9SharedPtrINS0_19GpuSharedParametersEEESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
#[doc(
    alias = "std::map<std::string,Ogre::SharedPtr<Ogre::GpuSharedParameters>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)"
)]
// was: std::map<std::string,Ogre::SharedPtr<Ogre::GpuSharedParameters>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xca2830: 256 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca2830() {
}

// 0xca2ad0 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(
    alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::operator=(Ogre::SharedPtr<Ogre::GpuSharedParameters> const&)"
)]
// was: Ogre::SharedPtr<Ogre::GpuSharedParameters>::operator=(Ogre::SharedPtr<Ogre::GpuSharedParameters> const&)
// IDA 0xca2ad0: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca2ad0() {
}

// 0xca2c50 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>> *)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>> *)
// IDA 0xca2c50: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca2c50() {
}

// 0xca2c80 — __ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_16MemoryDataStreamEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_
#[doc(
    alias = "Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>*)"
)]
// was: Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>*)
// IDA 0xca2c80: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca2c80() {
}

// 0xca2db0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: int __fastcall(int, int, int)
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>> const&)
// IDA 0xca2db0: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca2db0() {
}

// 0xca2e94 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
// type: int __fastcall(int, int, int, int)
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>> const&)
// IDA 0xca2e94: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca2e94() {
}

// 0xca2f08 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS6_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>> const&)
// IDA 0xca2f08: 106 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca2f08() {
}

// 0xca3020 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xca3020: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca3020() {
}

// 0xca30c4 — __ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const
// IDA 0xca30c4: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca30c4() {
}

// 0xca3168 — __ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const
// IDA 0xca3168: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca3168() {
}

// 0xca320c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// type: int __fastcall(int)
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> const&)
// IDA 0xca320c: 341 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca320c() {
}

// 0xca3554 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
// type: int __fastcall(int, int, int, int)
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> const&)
// IDA 0xca3554: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca3554() {
}

// 0xca35c8 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: int __fastcall(int, int, int)
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> const&)
// IDA 0xca35c8: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca35c8() {
}

// 0xca36ac — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS6_
// type: _DWORD *__fastcall(int, const std::string *, int, int, void *, int)
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> const&)
// IDA 0xca36ac: 106 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca36ac() {
}

// 0xca37c4 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xca37c4: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca37c4() {
}

// 0xca3868 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xca3868: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ca3868() {
}

// 0xca386c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xca386c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ca386c() {
}

// 0xca3878 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xca3878: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ca3878() {
}

// 0xca387c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xca387c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ca387c() {
}

// 0xca3888 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>> *)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>> *)
// IDA 0xca3888: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca3888() {
}

// 0xca38b8 — __ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_19GpuSharedParametersEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_
#[doc(
    alias = "Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>*)"
)]
// was: Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>*)
// IDA 0xca38b8: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca38b8() {
}

// 0xca3a1c — __ZN4Ogre17GpuNamedConstants38generateConstantDefinitionArrayEntriesERKSsRKNS_21GpuConstantDefinitionE
#[doc(
    alias = "Ogre::GpuNamedConstants::generateConstantDefinitionArrayEntries(std::string const&,Ogre::GpuConstantDefinition const&)"
)]
// was: Ogre::GpuNamedConstants::generateConstantDefinitionArrayEntries(std::string const&,Ogre::GpuConstantDefinition const&)
// IDA 0xca3a1c: 458 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca3a1c() {
}

// 0xca3f10 — __ZN4Ogre27GpuNamedConstantsSerializerD1Ev
// type: void __fastcall(Ogre::GpuNamedConstantsSerializer *__hidden this)
#[doc(alias = "Ogre::GpuNamedConstantsSerializer::~GpuNamedConstantsSerializer()")]
// was: Ogre::GpuNamedConstantsSerializer::~GpuNamedConstantsSerializer()
// IDA 0xca3f10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ca3f10() {
}

// 0xca3f1c — __ZN4Ogre17GpuNamedConstants4loadERNS_9SharedPtrINS_10DataStreamEEE
#[doc(alias = "Ogre::GpuNamedConstants::load(Ogre::SharedPtr<Ogre::DataStream> &)")]
// was: Ogre::GpuNamedConstants::load(Ogre::SharedPtr<Ogre::DataStream> &)
// IDA 0xca3f1c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca3f1c() {
}

// 0xca3ff0 — __ZN4Ogre27GpuNamedConstantsSerializer20importNamedConstantsERNS_9SharedPtrINS_10DataStreamEEEPNS_17GpuNamedConstantsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(
    alias = "Ogre::GpuNamedConstantsSerializer::importNamedConstants(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::GpuNamedConstants *)"
)]
// was: Ogre::GpuNamedConstantsSerializer::importNamedConstants(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::GpuNamedConstants *)
// IDA 0xca3ff0: 214 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca3ff0() {
}

// 0xca4228 — __ZN4Ogre27GpuNamedConstantsSerializerD0Ev
// type: void __fastcall(Ogre::GpuNamedConstantsSerializer *__hidden this)
#[doc(alias = "Ogre::GpuNamedConstantsSerializer::~GpuNamedConstantsSerializer()")]
// was: Ogre::GpuNamedConstantsSerializer::~GpuNamedConstantsSerializer()
// IDA 0xca4228: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ca4228() {
}

// 0xca42b8 — __ZN4Ogre19GpuSharedParametersC1ERKSs
// type: _DWORD __fastcall(Ogre::GpuSharedParameters *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuSharedParameters::GpuSharedParameters(std::string const&)")]
// was: Ogre::GpuSharedParameters::GpuSharedParameters(std::string const&)
// IDA 0xca42b8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca42b8() {
}

// 0xca42c4 — __ZN4Ogre19GpuSharedParametersC2ERKSs
// type: _DWORD __fastcall(Ogre::GpuSharedParameters *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuSharedParameters::GpuSharedParameters(std::string const&)")]
// was: Ogre::GpuSharedParameters::GpuSharedParameters(std::string const&)
// IDA 0xca42c4: 192 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca42c4() {
}

// 0xca44c8 — __ZN4Ogre19GpuSharedParametersD0Ev
// type: void __fastcall(Ogre::GpuSharedParameters *__hidden this)
#[doc(alias = "Ogre::GpuSharedParameters::~GpuSharedParameters()")]
// was: Ogre::GpuSharedParameters::~GpuSharedParameters()
// IDA 0xca44c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ca44c8() {
}

// 0xca4558 — __ZN4Ogre19GpuSharedParametersD1Ev
// type: void __fastcall(Ogre::GpuSharedParameters *__hidden this)
#[doc(alias = "Ogre::GpuSharedParameters::~GpuSharedParameters()")]
// was: Ogre::GpuSharedParameters::~GpuSharedParameters()
// IDA 0xca4558: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ca4558() {
}

// 0xca4564 — __ZN4Ogre19GpuSharedParametersD2Ev
// type: void __fastcall(Ogre::GpuSharedParameters *__hidden this)
#[doc(alias = "Ogre::GpuSharedParameters::~GpuSharedParameters()")]
// was: Ogre::GpuSharedParameters::~GpuSharedParameters()
// IDA 0xca4564: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ca4564() {
}

// 0xca46a4 — __ZN4Ogre19GpuSharedParameters21addConstantDefinitionERKSsNS_15GpuConstantTypeEm
#[doc(
    alias = "Ogre::GpuSharedParameters::addConstantDefinition(std::string const&,Ogre::GpuConstantType,unsigned long)"
)]
// was: Ogre::GpuSharedParameters::addConstantDefinition(std::string const&,Ogre::GpuConstantType,unsigned long)
// IDA 0xca46a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ca46a4() {
}

// 0xca4a64 — __ZN4Ogre19GpuSharedParameters16setNamedConstantERKSsPKfm
// type: _DWORD __fastcall(Ogre::GpuSharedParameters *__hidden this, const std::string *, const float *, unsigned int)
#[doc(
    alias = "Ogre::GpuSharedParameters::setNamedConstant(std::string const&,float const*,unsigned long)"
)]
// was: Ogre::GpuSharedParameters::setNamedConstant(std::string const&,float const*,unsigned long)
// IDA 0xca4a64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ca4a64() {
}

// 0xca4aa8 — __ZN4Ogre19GpuSharedParameters16setNamedConstantERKSsPKim
// type: _DWORD __fastcall(Ogre::GpuSharedParameters *__hidden this, const std::string *, const int *, unsigned int)
#[doc(
    alias = "Ogre::GpuSharedParameters::setNamedConstant(std::string const&,int const*,unsigned long)"
)]
// was: Ogre::GpuSharedParameters::setNamedConstant(std::string const&,int const*,unsigned long)
// IDA 0xca4aa8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca4aa8() {
}

// 0xca4aec — __ZN4Ogre24GpuSharedParametersUsageC2ENS_9SharedPtrINS_19GpuSharedParametersEEEPNS_20GpuProgramParametersE
#[doc(
    alias = "Ogre::GpuSharedParametersUsage::GpuSharedParametersUsage(Ogre::SharedPtr<Ogre::GpuSharedParameters>,Ogre::GpuProgramParameters *)"
)]
// was: Ogre::GpuSharedParametersUsage::GpuSharedParametersUsage(Ogre::SharedPtr<Ogre::GpuSharedParameters>,Ogre::GpuProgramParameters *)
// IDA 0xca4aec: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca4aec() {
}

// 0xca4c9c — __ZN4Ogre24GpuSharedParametersUsage12initCopyDataEv
// type: _DWORD __fastcall(Ogre::GpuSharedParametersUsage *__hidden this)
#[doc(alias = "Ogre::GpuSharedParametersUsage::initCopyData(void)")]
// was: Ogre::GpuSharedParametersUsage::initCopyData(void)
// IDA 0xca4c9c: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca4c9c() {
}

// 0xca4d34 — __ZNK4Ogre20GpuProgramParameters28_findNamedConstantDefinitionERKSsb
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const std::string *, bool)
#[doc(
    alias = "Ogre::GpuProgramParameters::_findNamedConstantDefinition(std::string const&,bool)const"
)]
// was: Ogre::GpuProgramParameters::_findNamedConstantDefinition(std::string const&,bool)const
// IDA 0xca4d34: 383 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca4d34() {
}

// 0xca51ac — __ZN4Ogre24GpuSharedParametersUsage31_copySharedParamsToTargetParamsEv
// type: _DWORD __fastcall(Ogre::GpuSharedParametersUsage *__hidden this)
#[doc(alias = "Ogre::GpuSharedParametersUsage::_copySharedParamsToTargetParams(void)")]
// was: Ogre::GpuSharedParametersUsage::_copySharedParamsToTargetParams(void)
// IDA 0xca51ac: 169 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca51ac() {
}

// 0xca5348 — __ZN4Ogre20GpuProgramParametersC1Ev
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this)
#[doc(alias = "Ogre::GpuProgramParameters::GpuProgramParameters(void)")]
// was: Ogre::GpuProgramParameters::GpuProgramParameters(void)
// IDA 0xca5348: 72 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca5348() {
}

// 0xca541c — __ZN4Ogre20GpuProgramParametersC1ERKS0_
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const Ogre::GpuProgramParameters *)
#[doc(
    alias = "Ogre::GpuProgramParameters::GpuProgramParameters(Ogre::GpuProgramParameters const&)"
)]
// was: Ogre::GpuProgramParameters::GpuProgramParameters(Ogre::GpuProgramParameters const&)
// IDA 0xca541c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca541c() {
}

// 0xca5428 — __ZN4Ogre20GpuProgramParametersC2ERKS0_
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const Ogre::GpuProgramParameters *)
#[doc(
    alias = "Ogre::GpuProgramParameters::GpuProgramParameters(Ogre::GpuProgramParameters const&)"
)]
// was: Ogre::GpuProgramParameters::GpuProgramParameters(Ogre::GpuProgramParameters const&)
// IDA 0xca5428: 319 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca5428() {
}

// 0xca5750 — __ZN4Ogre20GpuProgramParameters23copySharedParamSetUsageERKSt6vectorINS_24GpuSharedParametersUsageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, int, int, char, int, int, int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(
    alias = "Ogre::GpuProgramParameters::copySharedParamSetUsage(std::vector<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)"
)]
// was: Ogre::GpuProgramParameters::copySharedParamSetUsage(std::vector<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xca5750: 224 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca5750() {
}

// 0xca5970 — __ZN4Ogre20GpuProgramParameters18_setNamedConstantsERKNS_9SharedPtrINS_17GpuNamedConstantsEEE
#[doc(
    alias = "Ogre::GpuProgramParameters::_setNamedConstants(Ogre::SharedPtr<Ogre::GpuNamedConstants> const&)"
)]
// was: Ogre::GpuProgramParameters::_setNamedConstants(Ogre::SharedPtr<Ogre::GpuNamedConstants> const&)
// IDA 0xca5970: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca5970() {
}

// 0xca59c8 — __ZN4Ogre20GpuProgramParameters18_setLogicalIndexesERKNS_9SharedPtrINS_22GpuLogicalBufferStructEEES5_
#[doc(
    alias = "Ogre::GpuProgramParameters::_setLogicalIndexes(Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct> const&,Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct> const&)"
)]
// was: Ogre::GpuProgramParameters::_setLogicalIndexes(Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct> const&,Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct> const&)
// IDA 0xca59c8: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca59c8() {
}

// 0xca5a30 — __ZN4Ogre20GpuProgramParameters11setConstantEmPKfm
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, unsigned int, const float *, unsigned int)
#[doc(alias = "Ogre::GpuProgramParameters::setConstant(unsigned long,float const*,unsigned long)")]
// was: Ogre::GpuProgramParameters::setConstant(unsigned long,float const*,unsigned long)
// IDA 0xca5a30: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca5a30() {
}

// 0xca5a5c — __ZN4Ogre20GpuProgramParameters11setConstantEmRKNS_7Matrix4E
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, unsigned int, const Ogre::Matrix4 *)
#[doc(alias = "Ogre::GpuProgramParameters::setConstant(unsigned long,Ogre::Matrix4 const&)")]
// was: Ogre::GpuProgramParameters::setConstant(unsigned long,Ogre::Matrix4 const&)
// IDA 0xca5a5c: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca5a5c() {
}

// 0xca5b78 — __ZN4Ogre20GpuProgramParameters11setConstantEmPKim
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, unsigned int, const int *, unsigned int)
#[doc(alias = "Ogre::GpuProgramParameters::setConstant(unsigned long,int const*,unsigned long)")]
// was: Ogre::GpuProgramParameters::setConstant(unsigned long,int const*,unsigned long)
// IDA 0xca5b78: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca5b78() {
}

// 0xca5ba4 — __ZN4Ogre20GpuProgramParameters17_writeRawConstantEmRKNS_7Vector4Em
// type: int __fastcall(int, int, void *__src)
#[doc(
    alias = "Ogre::GpuProgramParameters::_writeRawConstant(unsigned long,Ogre::Vector4 const&,unsigned long)"
)]
// was: Ogre::GpuProgramParameters::_writeRawConstant(unsigned long,Ogre::Vector4 const&,unsigned long)
// IDA 0xca5ba4: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca5ba4() {
}

// 0xca5bc4 — __ZN4Ogre20GpuProgramParameters17_writeRawConstantEmRKNS_7Matrix4Em
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, unsigned int, const Ogre::Matrix4 *__src, unsigned int)
#[doc(
    alias = "Ogre::GpuProgramParameters::_writeRawConstant(unsigned long,Ogre::Matrix4 const&,unsigned long)"
)]
// was: Ogre::GpuProgramParameters::_writeRawConstant(unsigned long,Ogre::Matrix4 const&,unsigned long)
// IDA 0xca5bc4: 67 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca5bc4() {
}

// 0xca5ca4 — __ZN4Ogre20GpuProgramParameters17_writeRawConstantEmPKNS_7Matrix4Em
// type: int __fastcall(int, int, void *__src, size_t __n)
#[doc(
    alias = "Ogre::GpuProgramParameters::_writeRawConstant(unsigned long,Ogre::Matrix4 const*,unsigned long)"
)]
// was: Ogre::GpuProgramParameters::_writeRawConstant(unsigned long,Ogre::Matrix4 const*,unsigned long)
// IDA 0xca5ca4: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca5ca4() {
}

// 0xca5d6c — __ZN4Ogre20GpuProgramParameters32_getFloatConstantLogicalIndexUseEmmt
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, unsigned int, unsigned int, unsigned __int16)
#[doc(
    alias = "Ogre::GpuProgramParameters::_getFloatConstantLogicalIndexUse(unsigned long,unsigned long,unsigned short)"
)]
// was: Ogre::GpuProgramParameters::_getFloatConstantLogicalIndexUse(unsigned long,unsigned long,unsigned short)
// IDA 0xca5d6c: 174 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca5d6c() {
}

// 0xca5f30 — __ZN4Ogre20GpuProgramParameters30_getIntConstantLogicalIndexUseEmmt
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, unsigned int, unsigned int, unsigned __int16)
#[doc(
    alias = "Ogre::GpuProgramParameters::_getIntConstantLogicalIndexUse(unsigned long,unsigned long,unsigned short)"
)]
// was: Ogre::GpuProgramParameters::_getIntConstantLogicalIndexUse(unsigned long,unsigned long,unsigned short)
// IDA 0xca5f30: 328 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca5f30() {
}

// 0xca62d4 — __ZNK4Ogre20GpuProgramParameters29getConstantDefinitionIteratorEv
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this)
#[doc(alias = "Ogre::GpuProgramParameters::getConstantDefinitionIterator(void)const")]
// was: Ogre::GpuProgramParameters::getConstantDefinitionIterator(void)const
// IDA 0xca62d4: 161 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca62d4() {
}

// 0xca64b4 — __ZNK4Ogre20GpuProgramParameters21getConstantDefinitionERKSs
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgramParameters::getConstantDefinition(std::string const&)const")]
// was: Ogre::GpuProgramParameters::getConstantDefinition(std::string const&)const
// IDA 0xca64b4: 164 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca64b4() {
}

// 0xca669c — __ZN4Ogre20GpuProgramParameters15setAutoConstantEmNS0_16AutoConstantTypeEm
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(
    alias = "Ogre::GpuProgramParameters::setAutoConstant(unsigned long,Ogre::GpuProgramParameters::AutoConstantType,unsigned long)"
)]
// was: Ogre::GpuProgramParameters::setAutoConstant(unsigned long,Ogre::GpuProgramParameters::AutoConstantType,unsigned long)
// IDA 0xca669c: 307 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca669c() {
}

// 0xca6b2c — __ZN4Ogre20GpuProgramParameters17clearAutoConstantEm
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, unsigned int)
#[doc(alias = "Ogre::GpuProgramParameters::clearAutoConstant(unsigned long)")]
// was: Ogre::GpuProgramParameters::clearAutoConstant(unsigned long)
// IDA 0xca6b2c: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca6b2c() {
}

// 0xca6b90 — __ZN4Ogre20GpuProgramParameters22clearNamedAutoConstantERKSs
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgramParameters::clearNamedAutoConstant(std::string const&)")]
// was: Ogre::GpuProgramParameters::clearNamedAutoConstant(std::string const&)
// IDA 0xca6b90: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca6b90() {
}

// 0xca6c1c — __ZNK4Ogre20GpuProgramParameters23getAutoConstantIteratorEv
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this)
#[doc(alias = "Ogre::GpuProgramParameters::getAutoConstantIterator(void)const")]
// was: Ogre::GpuProgramParameters::getAutoConstantIterator(void)const
// IDA 0xca6c1c: 6 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca6c1c() {
}

// 0xca6c28 — __ZN4Ogre20GpuProgramParameters19setAutoConstantRealEmNS0_16AutoConstantTypeEf
#[doc(
    alias = "Ogre::GpuProgramParameters::setAutoConstantReal(unsigned long,Ogre::GpuProgramParameters::AutoConstantType,float)"
)]
// was: Ogre::GpuProgramParameters::setAutoConstantReal(unsigned long,Ogre::GpuProgramParameters::AutoConstantType,float)
// IDA 0xca6c28: 305 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca6c28() {
}

// 0xca70b4 — __ZN4Ogre20GpuProgramParameters17_updateAutoParamsEPKNS_19AutoParamDataSourceEt
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const Ogre::AutoParamDataSource *, unsigned __int16)
#[doc(
    alias = "Ogre::GpuProgramParameters::_updateAutoParams(Ogre::AutoParamDataSource const*,unsigned short)"
)]
// was: Ogre::GpuProgramParameters::_updateAutoParams(Ogre::AutoParamDataSource const*,unsigned short)
// IDA 0xca70b4: 2850 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca70b4() {
}

// 0xca96ec — __ZN4Ogre20GpuProgramParameters16setNamedConstantERKSsf
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const std::string *, float)
#[doc(alias = "Ogre::GpuProgramParameters::setNamedConstant(std::string const&,float)")]
// was: Ogre::GpuProgramParameters::setNamedConstant(std::string const&,float)
// IDA 0xca96ec: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca96ec() {
}

// 0xca9724 — __ZN4Ogre20GpuProgramParameters16setNamedConstantERKSsRKNS_7Vector4E
// type: int __fastcall(Ogre::GpuProgramParameters *this)
#[doc(
    alias = "Ogre::GpuProgramParameters::setNamedConstant(std::string const&,Ogre::Vector4 const&)"
)]
// was: Ogre::GpuProgramParameters::setNamedConstant(std::string const&,Ogre::Vector4 const&)
// IDA 0xca9724: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca9724() {
}

// 0xca9760 — __ZN4Ogre20GpuProgramParameters16setNamedConstantERKSsRKNS_7Matrix4E
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const std::string *, const Ogre::Matrix4 *)
#[doc(
    alias = "Ogre::GpuProgramParameters::setNamedConstant(std::string const&,Ogre::Matrix4 const&)"
)]
// was: Ogre::GpuProgramParameters::setNamedConstant(std::string const&,Ogre::Matrix4 const&)
// IDA 0xca9760: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca9760() {
}

// 0xca9790 — __ZN4Ogre20GpuProgramParameters16setNamedConstantERKSsPKfmm
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const std::string *, const float *, unsigned int, unsigned int)
#[doc(
    alias = "Ogre::GpuProgramParameters::setNamedConstant(std::string const&,float const*,unsigned long,unsigned long)"
)]
// was: Ogre::GpuProgramParameters::setNamedConstant(std::string const&,float const*,unsigned long,unsigned long)
// IDA 0xca9790: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca9790() {
}

// 0xca97c8 — __ZN4Ogre20GpuProgramParameters16setNamedConstantERKSsRKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const std::string *, const Ogre::ColourValue *)
#[doc(
    alias = "Ogre::GpuProgramParameters::setNamedConstant(std::string const&,Ogre::ColourValue const&)"
)]
// was: Ogre::GpuProgramParameters::setNamedConstant(std::string const&,Ogre::ColourValue const&)
// IDA 0xca97c8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca97c8() {
}

// 0xca9804 — __ZN4Ogre20GpuProgramParameters16setNamedConstantERKSsPKimm
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const std::string *, const int *, unsigned int, unsigned int)
#[doc(
    alias = "Ogre::GpuProgramParameters::setNamedConstant(std::string const&,int const*,unsigned long,unsigned long)"
)]
// was: Ogre::GpuProgramParameters::setNamedConstant(std::string const&,int const*,unsigned long,unsigned long)
// IDA 0xca9804: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca9804() {
}

// 0xca983c — __ZN4Ogre20GpuProgramParameters20setNamedAutoConstantERKSsNS0_16AutoConstantTypeEm
// type: int __fastcall(Ogre::GpuProgramParameters *this)
#[doc(
    alias = "Ogre::GpuProgramParameters::setNamedAutoConstant(std::string const&,Ogre::GpuProgramParameters::AutoConstantType,unsigned long)"
)]
// was: Ogre::GpuProgramParameters::setNamedAutoConstant(std::string const&,Ogre::GpuProgramParameters::AutoConstantType,unsigned long)
// IDA 0xca983c: 86 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca983c() {
}

// 0xca9a24 — __ZN4Ogre20GpuProgramParameters24setNamedAutoConstantRealERKSsNS0_16AutoConstantTypeEf
// type: int __fastcall(Ogre::GpuProgramParameters *this)
#[doc(
    alias = "Ogre::GpuProgramParameters::setNamedAutoConstantReal(std::string const&,Ogre::GpuProgramParameters::AutoConstantType,float)"
)]
// was: Ogre::GpuProgramParameters::setNamedAutoConstantReal(std::string const&,Ogre::GpuProgramParameters::AutoConstantType,float)
// IDA 0xca9a24: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca9a24() {
}

// 0xca9c20 — __ZN4Ogre20GpuProgramParameters30_findRawAutoConstantEntryFloatEm
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, unsigned int)
#[doc(alias = "Ogre::GpuProgramParameters::_findRawAutoConstantEntryFloat(unsigned long)")]
// was: Ogre::GpuProgramParameters::_findRawAutoConstantEntryFloat(unsigned long)
// IDA 0xca9c20: 11 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca9c20() {
}

// 0xca9c3c — __ZN4Ogre20GpuProgramParameters17copyConstantsFromERKS0_
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const Ogre::GpuProgramParameters *)
#[doc(alias = "Ogre::GpuProgramParameters::copyConstantsFrom(Ogre::GpuProgramParameters const&)")]
// was: Ogre::GpuProgramParameters::copyConstantsFrom(Ogre::GpuProgramParameters const&)
// IDA 0xca9c3c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca9c3c() {
}

// 0xca9c74 — __ZN4Ogre20GpuProgramParameters30copyMatchingNamedConstantsFromERKS0_
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const Ogre::GpuProgramParameters *)
#[doc(
    alias = "Ogre::GpuProgramParameters::copyMatchingNamedConstantsFrom(Ogre::GpuProgramParameters const&)"
)]
// was: Ogre::GpuProgramParameters::copyMatchingNamedConstantsFrom(Ogre::GpuProgramParameters const&)
// IDA 0xca9c74: 422 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ca9c74() {
}

// 0xcaa0a4 — __ZN4Ogre20GpuProgramParameters19addSharedParametersENS_9SharedPtrINS_19GpuSharedParametersEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, int, int, char, int, int, int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(
    alias = "Ogre::GpuProgramParameters::addSharedParameters(Ogre::SharedPtr<Ogre::GpuSharedParameters>)"
)]
// was: Ogre::GpuProgramParameters::addSharedParameters(Ogre::SharedPtr<Ogre::GpuSharedParameters>)
// IDA 0xcaa0a4: 226 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caa0a4() {
}

// 0xcaa2dc — __ZN4Ogre20GpuProgramParameters25getAutoConstantDefinitionERKSs
#[doc(alias = "Ogre::GpuProgramParameters::getAutoConstantDefinition(std::string const&)")]
// was: Ogre::GpuProgramParameters::getAutoConstantDefinition(std::string const&)
// IDA 0xcaa2dc: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caa2dc() {
}

// 0xcaa364 — __ZN4Ogre20GpuProgramParameters22incPassIterationNumberEv
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this)
#[doc(alias = "Ogre::GpuProgramParameters::incPassIterationNumber(void)")]
// was: Ogre::GpuProgramParameters::incPassIterationNumber(void)
// IDA 0xcaa364: 11 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caa364() {
}

// 0xcaa388 — __ZN4Ogre20GpuProgramParameters19addSharedParametersERKSs
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgramParameters::addSharedParameters(std::string const&)")]
// was: Ogre::GpuProgramParameters::addSharedParameters(std::string const&)
// IDA 0xcaa388: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caa388() {
}

// 0xcaa500 — __ZN4Ogre20GpuProgramParameters17_copySharedParamsEv
// type: _DWORD __fastcall(Ogre::GpuProgramParameters *__hidden this)
#[doc(alias = "Ogre::GpuProgramParameters::_copySharedParams(void)")]
// was: Ogre::GpuProgramParameters::_copySharedParams(void)
// IDA 0xcaa500: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caa500() {
}

// 0xcaa520 — __ZNSt3mapISsN4Ogre21GpuConstantDefinitionESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
#[doc(
    alias = "std::map<std::string,Ogre::GpuConstantDefinition,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)"
)]
// was: std::map<std::string,Ogre::GpuConstantDefinition,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xcaa520: 163 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caa520() {
}

// 0xcaa6e8 — __ZN4Ogre12STLAllocatorINS_24GpuSharedParametersUsageENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev
#[doc(
    alias = "Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()"
)]
// was: Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xcaa6e8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_caa6e8() {
}

// 0xcaa6ec — __ZNSt6vectorIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_
#[doc(
    alias = "std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)"
)]
// was: std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xcaa6ec: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caa6ec() {
}

// 0xcaa7ac — __ZNSt6vectorIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS6_
#[doc(
    alias = "std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)"
)]
// was: std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xcaa7ac: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caa7ac() {
}

// 0xcaa86c — __ZNSt6vectorIN4Ogre20GpuProgramParameters17AutoConstantEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
#[doc(
    alias = "std::vector<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)"
)]
// was: std::vector<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xcaa86c: 103 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caa86c() {
}

// 0xcaab68 — __ZNSt6vectorIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
#[doc(
    alias = "std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)"
)]
// was: std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xcaab68: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caab68() {
}

// 0xcaaeac — __ZNSt6vectorIN4Ogre20GpuProgramParameters17AutoConstantEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(
    alias = "std::vector<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GpuProgramParameters::AutoConstantEntry*,std::vector<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GpuProgramParameters::AutoConstantEntry const&)"
)]
// was: std::vector<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GpuProgramParameters::AutoConstantEntry*,std::vector<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GpuProgramParameters::AutoConstantEntry const&)
// IDA 0xcaaeac: 130 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caaeac() {
}

// 0xcab01c — __ZNSt6vectorIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPiS6_EEmRKi
// type: int __fastcall(int, void *__src)
#[doc(
    alias = "std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<int *,std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,int const&)"
)]
// was: std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<int *,std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,int const&)
// IDA 0xcab01c: 159 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cab01c() {
}

// 0xcab1b8 — __ZNSt6vectorIN4Ogre24GpuSharedParametersUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: int __fastcall(int, int, int)
#[doc(
    alias = "std::vector<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GpuSharedParametersUsage*,std::vector<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GpuSharedParametersUsage const&)"
)]
// was: std::vector<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GpuSharedParametersUsage*,std::vector<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GpuSharedParametersUsage const&)
// IDA 0xcab1b8: 266 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cab1b8() {
}

// 0xcab460 — __ZSt22__uninitialized_copy_aIPN4Ogre24GpuSharedParametersUsageES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
// type: int __fastcall(int, int, int, int, Ogre::GpuSharedParametersUsage *, void *, int, int, int, int)
#[doc(
    alias = "Ogre::GpuSharedParametersUsage * std::__uninitialized_copy_a<Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)"
)]
// was: Ogre::GpuSharedParametersUsage * std::__uninitialized_copy_a<Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
// IDA 0xcab460: 64 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cab460() {
}

// 0xcab578 — __ZN4Ogre12STLAllocatorINS_24GpuSharedParametersUsageENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(
    alias = "Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()"
)]
// was: Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xcab578: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cab578() {
}

// 0xcab584 — __ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN4Ogre24GpuSharedParametersUsageES4_EET0_T_S6_S5_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(
    alias = "Ogre::GpuSharedParametersUsage * std::__copy_backward_normal<false,false>::__copy_b_n<Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *>(Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *)"
)]
// was: Ogre::GpuSharedParametersUsage * std::__copy_backward_normal<false,false>::__copy_b_n<Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *>(Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *,Ogre::GpuSharedParametersUsage *)
// IDA 0xcab584: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cab584() {
}

// 0xcab6ec — __ZN4Ogre24GpuSharedParametersUsageC2ERKS0_
// type: _DWORD __fastcall(Ogre::GpuSharedParametersUsage *__hidden this, const Ogre::GpuSharedParametersUsage *)
#[doc(
    alias = "Ogre::GpuSharedParametersUsage::GpuSharedParametersUsage(Ogre::GpuSharedParametersUsage const&)"
)]
// was: Ogre::GpuSharedParametersUsage::GpuSharedParametersUsage(Ogre::GpuSharedParametersUsage const&)
// IDA 0xcab6ec: 170 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cab6ec() {
}

// 0xcab890 — __ZNSt6vectorIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_
#[doc(
    alias = "std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)"
)]
// was: std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xcab890: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cab890() {
}

// 0xcab910 — __ZNSt12_Vector_baseIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcab910: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cab910() {
}

// 0xcab914 — __ZNSt12_Vector_baseIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcab914: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cab914() {
}

// 0xcab920 — __ZNSt12_Vector_baseIN4Ogre24GpuSharedParametersUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcab920: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cab920() {
}

// 0xcab924 — __ZNSt12_Vector_baseIN4Ogre24GpuSharedParametersUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcab924: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cab924() {
}

// 0xcab930 — __ZNSt12_Vector_baseIN4Ogre20GpuProgramParameters17AutoConstantEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcab930: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cab930() {
}

// 0xcab934 — __ZNSt12_Vector_baseIN4Ogre20GpuProgramParameters17AutoConstantEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::STLAllocator<Ogre::GpuProgramParameters::AutoConstantEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcab934: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cab934() {
}

// 0xcab940 — __ZNSt6vectorIN4Ogre24GpuSharedParametersUsage13CopyDataEntryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(
    alias = "std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GpuSharedParametersUsage::CopyDataEntry*,std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GpuSharedParametersUsage::CopyDataEntry const&)"
)]
// was: std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GpuSharedParametersUsage::CopyDataEntry*,std::vector<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage::CopyDataEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GpuSharedParametersUsage::CopyDataEntry const&)
// IDA 0xcab940: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cab940() {
}

// 0xcaba5c — __ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const
// IDA 0xcaba5c: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_caba5c() {
}

// 0xcabb00 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xcabb00: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cabb00() {
}

// 0xcabba4 — __ZNSt12_Vector_baseIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(
    alias = "std::_Vector_base<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcabba4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cabba4() {
}

// 0xcabba8 — __ZNSt12_Vector_baseIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(
    alias = "std::_Vector_base<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()"
)]
// was: std::_Vector_base<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcabba8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cabba8() {
}

// 0xcabbb4 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::pair<std::string const,Ogre::GpuConstantDefinition> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::pair<std::string const,Ogre::GpuConstantDefinition> const&)
// IDA 0xcabbb4: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cabbb4() {
}

// 0xcabd94 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::GpuConstantDefinition> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::GpuConstantDefinition> const&)
// IDA 0xcabd94: 131 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cabd94() {
}

// 0xcabef8 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::GpuConstantDefinition> const&)"
)]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::GpuConstantDefinition> const&)
// IDA 0xcabef8: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cabef8() {
}

// 0xcb4364 — __ZN4Ogre15GpuProgramUsageC1ENS_14GpuProgramTypeEPNS_4PassE
#[doc(alias = "Ogre::GpuProgramUsage::GpuProgramUsage(Ogre::GpuProgramType,Ogre::Pass *)")]
// was: Ogre::GpuProgramUsage::GpuProgramUsage(Ogre::GpuProgramType,Ogre::Pass *)
// IDA 0xcb4364: 26 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb4364() {
}

// 0xcb43b8 — __ZN4Ogre15GpuProgramUsageC1ERKS0_PNS_4PassE
// type: _DWORD __fastcall(Ogre::GpuProgramUsage *__hidden this, const Ogre::GpuProgramUsage *, Ogre::Pass *)
#[doc(alias = "Ogre::GpuProgramUsage::GpuProgramUsage(Ogre::GpuProgramUsage const&,Ogre::Pass *)")]
// was: Ogre::GpuProgramUsage::GpuProgramUsage(Ogre::GpuProgramUsage const&,Ogre::Pass *)
// IDA 0xcb43b8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb43b8() {
}

// 0xcb43c4 — __ZN4Ogre15GpuProgramUsageC2ERKS0_PNS_4PassE
// type: _DWORD __fastcall(Ogre::GpuProgramUsage *__hidden this, const Ogre::GpuProgramUsage *, Ogre::Pass *)
#[doc(alias = "Ogre::GpuProgramUsage::GpuProgramUsage(Ogre::GpuProgramUsage const&,Ogre::Pass *)")]
// was: Ogre::GpuProgramUsage::GpuProgramUsage(Ogre::GpuProgramUsage const&,Ogre::Pass *)
// IDA 0xcb43c4: 194 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb43c4() {
}

// 0xcb45ac — __ZN4Ogre15GpuProgramUsageD0Ev
// type: void __fastcall(Ogre::GpuProgramUsage *__hidden this)
#[doc(alias = "Ogre::GpuProgramUsage::~GpuProgramUsage()")]
// was: Ogre::GpuProgramUsage::~GpuProgramUsage()
// IDA 0xcb45ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb45ac() {
}

// 0xcb463c — __ZN4Ogre15GpuProgramUsageD1Ev
// type: void __fastcall(Ogre::GpuProgramUsage *__hidden this)
#[doc(alias = "Ogre::GpuProgramUsage::~GpuProgramUsage()")]
// was: Ogre::GpuProgramUsage::~GpuProgramUsage()
// IDA 0xcb463c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb463c() {
}

// 0xcb4648 — __ZN4Ogre15GpuProgramUsageD2Ev
// type: void __fastcall(Ogre::GpuProgramUsage *__hidden this)
#[doc(alias = "Ogre::GpuProgramUsage::~GpuProgramUsage()")]
// was: Ogre::GpuProgramUsage::~GpuProgramUsage()
// IDA 0xcb4648: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb4648() {
}

// 0xcb4840 — __ZN4Ogre15GpuProgramUsage14setProgramNameERKSsb
// type: _DWORD __fastcall(Ogre::GpuProgramUsage *__hidden this, const std::string *, bool)
#[doc(alias = "Ogre::GpuProgramUsage::setProgramName(std::string const&,bool)")]
// was: Ogre::GpuProgramUsage::setProgramName(std::string const&,bool)
// IDA 0xcb4840: 593 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb4840() {
}

// 0xcb4ed0 — __ZN4Ogre15GpuProgramUsage18recreateParametersEv
// type: _DWORD __fastcall(Ogre::GpuProgramUsage *__hidden this)
#[doc(alias = "Ogre::GpuProgramUsage::recreateParameters(void)")]
// was: Ogre::GpuProgramUsage::recreateParameters(void)
// IDA 0xcb4ed0: 145 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb4ed0() {
}

// 0xcb5040 — __ZN4Ogre15GpuProgramUsage13setParametersENS_9SharedPtrINS_20GpuProgramParametersEEE
#[doc(alias = "Ogre::GpuProgramUsage::setParameters(Ogre::SharedPtr<Ogre::GpuProgramParameters>)")]
// was: Ogre::GpuProgramUsage::setParameters(Ogre::SharedPtr<Ogre::GpuProgramParameters>)
// IDA 0xcb5040: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb5040() {
}

// 0xcb504c — __ZN4Ogre15GpuProgramUsage13getParametersEv
// type: _DWORD __fastcall(Ogre::GpuProgramUsage *__hidden this)
#[doc(alias = "Ogre::GpuProgramUsage::getParameters(void)")]
// was: Ogre::GpuProgramUsage::getParameters(void)
// IDA 0xcb504c: 170 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb504c() {
}

// 0xcb5244 — __ZN4Ogre15GpuProgramUsage5_loadEv
// type: _DWORD __fastcall(Ogre::GpuProgramUsage *__hidden this)
#[doc(alias = "Ogre::GpuProgramUsage::_load(void)")]
// was: Ogre::GpuProgramUsage::_load(void)
// IDA 0xcb5244: 557 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb5244() {
}

// 0xcb5894 — __ZN4Ogre15GpuProgramUsage17unloadingCompleteEPNS_8ResourceE
// type: _DWORD __fastcall(Ogre::GpuProgramUsage *__hidden this, Ogre::Resource *)
#[doc(alias = "Ogre::GpuProgramUsage::unloadingComplete(Ogre::Resource *)")]
// was: Ogre::GpuProgramUsage::unloadingComplete(Ogre::Resource *)
// IDA 0xcb5894: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb5894() {
}

// 0xcb589c — __ZN4Ogre15GpuProgramUsage15loadingCompleteEPNS_8ResourceE
// type: _DWORD __fastcall(Ogre::GpuProgramUsage *__hidden this, Ogre::Resource *)
#[doc(alias = "Ogre::GpuProgramUsage::loadingComplete(Ogre::Resource *)")]
// was: Ogre::GpuProgramUsage::loadingComplete(Ogre::Resource *)
// IDA 0xcb589c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb589c() {
}

// 0xcb58b0 — __ZN4Ogre13GpuProgramPtrD1Ev
// type: void __fastcall(Ogre::GpuProgramPtr *__hidden this)
#[doc(alias = "Ogre::GpuProgramPtr::~GpuProgramPtr()")]
// was: Ogre::GpuProgramPtr::~GpuProgramPtr()
// IDA 0xcb58b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb58b0() {
}

// 0xcb59a0 — __ZN4Ogre8Resource8Listener25backgroundLoadingCompleteEPS0_
// type: _DWORD __fastcall(Ogre::Resource::Listener *__hidden this, Ogre::Resource *)
#[doc(alias = "Ogre::Resource::Listener::backgroundLoadingComplete(Ogre::Resource*)")]
// was: Ogre::Resource::Listener::backgroundLoadingComplete(Ogre::Resource*)
// IDA 0xcb59a0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb59a0() {
}

// 0xcb59a4 — __ZN4Ogre9SharedPtrINS_10GpuProgramEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(
    alias = "Ogre::SharedPtr<Ogre::GpuProgram>::operator=(Ogre::SharedPtr<Ogre::GpuProgram> const&)"
)]
// was: Ogre::SharedPtr<Ogre::GpuProgram>::operator=(Ogre::SharedPtr<Ogre::GpuProgram> const&)
// IDA 0xcb59a4: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb59a4() {
}

// 0xcb5b24 — __ZN4Ogre13GpuProgramPtrD0Ev
// type: void __fastcall(Ogre::GpuProgramPtr *__hidden this)
#[doc(alias = "Ogre::GpuProgramPtr::~GpuProgramPtr()")]
// was: Ogre::GpuProgramPtr::~GpuProgramPtr()
// IDA 0xcb5b24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb5b24() {
}

// 0xcb5c4c — __ZN4Ogre21HardwareBufferManager15getSingletonPtrEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManager *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManager::getSingletonPtr(void)")]
// was: Ogre::HardwareBufferManager::getSingletonPtr(void)
// IDA 0xcb5c4c: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb5c4c() {
}

// 0xcb5c5c — __ZN4Ogre21HardwareBufferManager12getSingletonEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManager *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManager::getSingleton(void)")]
// was: Ogre::HardwareBufferManager::getSingleton(void)
// IDA 0xcb5c5c: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb5c5c() {
}

// 0xcb5c6c — __ZN4Ogre21HardwareBufferManagerC2EPNS_25HardwareBufferManagerBaseE
#[doc(
    alias = "Ogre::HardwareBufferManager::HardwareBufferManager(Ogre::HardwareBufferManagerBase *)"
)]
// was: Ogre::HardwareBufferManager::HardwareBufferManager(Ogre::HardwareBufferManagerBase *)
// IDA 0xcb5c6c: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb5c6c() {
}

// 0xcb5d70 — __ZN4Ogre25HardwareBufferManagerBaseC2Ev
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::HardwareBufferManagerBase(void)")]
// was: Ogre::HardwareBufferManagerBase::HardwareBufferManagerBase(void)
// IDA 0xcb5d70: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb5d70() {
}

// 0xcb5e60 — __ZN4Ogre25HardwareBufferManagerBaseD2Ev
// type: void __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::~HardwareBufferManagerBase()")]
// was: Ogre::HardwareBufferManagerBase::~HardwareBufferManagerBase()
// IDA 0xcb5e60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb5e60() {
}

// 0xcb605c — __ZN4Ogre21HardwareBufferManagerD0Ev
// type: void __fastcall(Ogre::HardwareBufferManager *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManager::~HardwareBufferManager()")]
// was: Ogre::HardwareBufferManager::~HardwareBufferManager()
// IDA 0xcb605c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb605c() {
}

// 0xcb60fc — __ZN4Ogre21HardwareBufferManagerD1Ev
// type: void __fastcall(Ogre::HardwareBufferManager *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManager::~HardwareBufferManager()")]
// was: Ogre::HardwareBufferManager::~HardwareBufferManager()
// IDA 0xcb60fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb60fc() {
}

// 0xcb6114 — __ZN4Ogre21HardwareBufferManagerD2Ev
// type: void __fastcall(Ogre::HardwareBufferManager *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManager::~HardwareBufferManager()")]
// was: Ogre::HardwareBufferManager::~HardwareBufferManager()
// IDA 0xcb6114: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb6114() {
}

// 0xcb612c — __ZN4Ogre25HardwareBufferManagerBaseD0Ev
// type: void __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::~HardwareBufferManagerBase()")]
// was: Ogre::HardwareBufferManagerBase::~HardwareBufferManagerBase()
// IDA 0xcb612c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb612c() {
}

// 0xcb61bc — __ZN4Ogre25HardwareBufferManagerBaseD1Ev
// type: void __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::~HardwareBufferManagerBase()")]
// was: Ogre::HardwareBufferManagerBase::~HardwareBufferManagerBase()
// IDA 0xcb61bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb61bc() {
}

// 0xcb61c8 — __ZN4Ogre25HardwareBufferManagerBase23createVertexDeclarationEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::createVertexDeclaration(void)")]
// was: Ogre::HardwareBufferManagerBase::createVertexDeclaration(void)
// IDA 0xcb61c8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb61c8() {
}

// 0xcb61ec — __ZN4Ogre25HardwareBufferManagerBase24destroyVertexDeclarationEPNS_17VertexDeclarationE
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this, Ogre::VertexDeclaration *)
#[doc(
    alias = "Ogre::HardwareBufferManagerBase::destroyVertexDeclaration(Ogre::VertexDeclaration *)"
)]
// was: Ogre::HardwareBufferManagerBase::destroyVertexDeclaration(Ogre::VertexDeclaration *)
// IDA 0xcb61ec: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb61ec() {
}

// 0xcb624c — __ZN4Ogre25HardwareBufferManagerBase25createVertexBufferBindingEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::createVertexBufferBinding(void)")]
// was: Ogre::HardwareBufferManagerBase::createVertexBufferBinding(void)
// IDA 0xcb624c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb624c() {
}

// 0xcb6270 — __ZN4Ogre25HardwareBufferManagerBase26destroyVertexBufferBindingEPNS_19VertexBufferBindingE
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this, Ogre::VertexBufferBinding *)
#[doc(
    alias = "Ogre::HardwareBufferManagerBase::destroyVertexBufferBinding(Ogre::VertexBufferBinding *)"
)]
// was: Ogre::HardwareBufferManagerBase::destroyVertexBufferBinding(Ogre::VertexBufferBinding *)
// IDA 0xcb6270: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb6270() {
}

// 0xcb62d0 — __ZN4Ogre25HardwareBufferManagerBase27createVertexDeclarationImplEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::createVertexDeclarationImpl(void)")]
// was: Ogre::HardwareBufferManagerBase::createVertexDeclarationImpl(void)
// IDA 0xcb62d0: 65 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb62d0() {
}

// 0xcb6388 — __ZN4Ogre25HardwareBufferManagerBase28destroyVertexDeclarationImplEPNS_17VertexDeclarationE
#[doc(
    alias = "Ogre::HardwareBufferManagerBase::destroyVertexDeclarationImpl(Ogre::VertexDeclaration *)"
)]
// was: Ogre::HardwareBufferManagerBase::destroyVertexDeclarationImpl(Ogre::VertexDeclaration *)
// IDA 0xcb6388: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb6388() {
}

// 0xcb639c — __ZN4Ogre25HardwareBufferManagerBase29createVertexBufferBindingImplEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::createVertexBufferBindingImpl(void)")]
// was: Ogre::HardwareBufferManagerBase::createVertexBufferBindingImpl(void)
// IDA 0xcb639c: 65 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb639c() {
}

// 0xcb6454 — __ZN4Ogre25HardwareBufferManagerBase30destroyVertexBufferBindingImplEPNS_19VertexBufferBindingE
#[doc(
    alias = "Ogre::HardwareBufferManagerBase::destroyVertexBufferBindingImpl(Ogre::VertexBufferBinding *)"
)]
// was: Ogre::HardwareBufferManagerBase::destroyVertexBufferBindingImpl(Ogre::VertexBufferBinding *)
// IDA 0xcb6454: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb6454() {
}

// 0xcb6468 — __ZN4Ogre25HardwareBufferManagerBase22destroyAllDeclarationsEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::destroyAllDeclarations(void)")]
// was: Ogre::HardwareBufferManagerBase::destroyAllDeclarations(void)
// IDA 0xcb6468: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb6468() {
}

// 0xcb64a4 — __ZN4Ogre25HardwareBufferManagerBase18destroyAllBindingsEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::destroyAllBindings(void)")]
// was: Ogre::HardwareBufferManagerBase::destroyAllBindings(void)
// IDA 0xcb64a4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb64a4() {
}

// 0xcb64e0 — __ZN4Ogre25HardwareBufferManagerBase33registerVertexBufferSourceAndCopyERKNS_29HardwareVertexBufferSharedPtrES3_
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this, const Ogre::HardwareVertexBufferSharedPtr *, const Ogre::HardwareVertexBufferSharedPtr *)
#[doc(
    alias = "Ogre::HardwareBufferManagerBase::registerVertexBufferSourceAndCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareVertexBufferSharedPtr const&)"
)]
// was: Ogre::HardwareBufferManagerBase::registerVertexBufferSourceAndCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareVertexBufferSharedPtr const&)
// IDA 0xcb64e0: 222 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb64e0() {
}

// 0x37068 — __ZN10RobloxView37requestStopRenderingForBackgroundModeEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::requestStopRenderingForBackgroundMode(void)")]
// was: RobloxView::requestStopRenderingForBackgroundMode(void)
// IDA 0x37068: 296 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37068() {
}

// 0x37378 — __ZN10RobloxView22requestResumeRenderingEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::requestResumeRendering(void)")]
// was: RobloxView::requestResumeRendering(void)
// IDA 0x37378: 220 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37378() {
}

// 0x39d7c — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEE5resetEv
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::reset(void)")]
// was: boost::shared_ptr<RobloxView::RenderJob>::reset(void)
// IDA 0x39d7c: 51 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39d7c() {
}

// 0x3a030 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEaSEOS3_
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::operator=(rbx_core::SharedPtr<RobloxView::RenderJob>&&)")]
// was: boost::shared_ptr<RobloxView::RenderJob>::operator=(boost::shared_ptr<RobloxView::RenderJob>&&)
// IDA 0x3a030: 55 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a030() {
}

// 0x3a0d4 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEC1IS2_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::shared_ptr<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
// was: boost::shared_ptr<RobloxView::RenderJob>::shared_ptr<RobloxView::RenderJob>(RobloxView::RenderJob *)
// IDA 0x3a0d4: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a0d4() {
}

// 0x3dc60 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(rbx_core::SharedPtr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const")]
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(boost::shared_ptr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const
// IDA 0x3dc60: 77 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dc60() {
}

// 0x3dd34 — __ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
// was: boost::detail::shared_count::shared_count<RobloxView::RenderJob>(RobloxView::RenderJob *)
// IDA 0x3dd34: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dd34() {
}

// 0x3de28 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()
// IDA 0x3de28: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3de28() {
}

// 0x3de2c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()
// IDA 0x3de2c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3de2c() {
}

// 0x3de30 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::dispose(void)")]
// was: boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::dispose(void)
// IDA 0x3de30: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3de30() {
}

// 0x3de40 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_deleter(std::type_info const&)
// IDA 0x3de40: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3de40() {
}

// 0x3de44 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_untyped_deleter(void)
// IDA 0x3de44: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3de44() {
}

// 0x3ecf0 — __ZN10RobloxView9RenderJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerEN5boost10shared_ptrINS1_9DataModelEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
#[doc(alias = "RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)")]
// was: RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,boost::shared_ptr<RBX::DataModel>)
// IDA 0x3ecf0: 143 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ecf0() {
}

// 0x3ee80 — __ZN10RobloxView9RenderJobD1Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "RobloxView::RenderJob::~RenderJob()")]
// was: RobloxView::RenderJob::~RenderJob()
// IDA 0x3ee80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ee80() {
}

// 0x3ef40 — __ZN10RobloxView9RenderJobD0Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "RobloxView::RenderJob::~RenderJob()")]
// was: RobloxView::RenderJob::~RenderJob()
// IDA 0x3ef40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ef40() {
}

// 0x3f008 — __ZN10RobloxView9RenderJob9sleepTimeERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// was: RobloxView::RenderJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// IDA 0x3f008: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f008() {
}

// 0x3f058 — __ZN10RobloxView9RenderJob5errorERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// was: RobloxView::RenderJob::error(RBX::TaskScheduler::Job::Stats const&)
// IDA 0x3f058: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f058() {
}

// 0x3f094 — __ZN10RobloxView9RenderJob16stepDataModelJobERKN3RBX13TaskScheduler3Job5StatsE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
// was: RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)
// IDA 0x3f094: 477 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f094() {
}

// 0x3f598 — __ZNK10RobloxView9RenderJob14getMetricValueERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "RobloxView::RenderJob::getMetricValue(std::string const&)const")]
// was: RobloxView::RenderJob::getMetricValue(std::string const&)const
// IDA 0x3f598: 115 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f598() {
}

// 0x3f700 — __ZNK10RobloxView9RenderJob9getMetricERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "RobloxView::RenderJob::getMetric(std::string const&)const")]
// was: RobloxView::RenderJob::getMetric(std::string const&)const
// IDA 0x3f700: 180 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f700() {
}

// 0x3f904 — __ZThn480_N10RobloxView9RenderJobD1Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::~RenderJob()")]
// was: non-virtual thunk to RobloxView::RenderJob::~RenderJob()
// IDA 0x3f904: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3f904() {
}

// 0x3f9c8 — __ZThn480_N10RobloxView9RenderJobD0Ev
// type: void __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::~RenderJob()")]
// was: non-virtual thunk to RobloxView::RenderJob::~RenderJob()
// IDA 0x3f9c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3f9c8() {
}

// 0x3fa94 — __ZThn480_NK10RobloxView9RenderJob9getMetricERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::getMetric(std::string const&)const")]
// was: non-virtual thunk to RobloxView::RenderJob::getMetric(std::string const&)const
// IDA 0x3fa94: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fa94() {
}

// 0x3faa4 — __ZThn480_NK10RobloxView9RenderJob14getMetricValueERKSs
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, const std::string *)
#[doc(alias = "non-virtual thunk toRobloxView::RenderJob::getMetricValue(std::string const&)const")]
// was: non-virtual thunk to RobloxView::RenderJob::getMetricValue(std::string const&)const
// IDA 0x3faa4: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3faa4() {
}

// 0x3faac — __ZN10RobloxView9RenderJob21scheduleRenderPrepareEPS0_PN3RBX8ViewBaseE
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, RenderJob *, ViewBase *)
#[doc(alias = "RobloxView::RenderJob::scheduleRenderPrepare(RobloxView::RenderJob*,RBX::ViewBase *)")]
// was: RobloxView::RenderJob::scheduleRenderPrepare(RobloxView::RenderJob*,RBX::ViewBase *)
// IDA 0x3faac: 10 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3faac() {
}

// 0x3fac4 — __ZN10RobloxView9RenderJob21scheduleRenderPerformEPS0_PN3RBX8ViewBaseEd
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this, RobloxView::RenderJob *, RBX::ViewBase *, double)
#[doc(alias = "RobloxView::RenderJob::scheduleRenderPerform(RobloxView::RenderJob*,RBX::ViewBase *,double)")]
// was: RobloxView::RenderJob::scheduleRenderPerform(RobloxView::RenderJob*,RBX::ViewBase *,double)
// IDA 0x3fac4: 77 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fac4() {
}

// 0x3fb9c — __ZN10RobloxView9RenderJob4wakeEv
// type: _DWORD __fastcall(RobloxView::RenderJob *__hidden this)
#[doc(alias = "RobloxView::RenderJob::wake(void)")]
// was: RobloxView::RenderJob::wake(void)
// IDA 0x3fb9c: 123 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb9c() {
}

// 0x40160 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x40160: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40160() {
}

// 0x401dc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEdENS3_5list3INS3_5valueIS7_EENSE_ISA_EENSE_IdEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *,double),boost::_bi::list3<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)
// IDA 0x401dc: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_401dc() {
}

// 0x401f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x401f0: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_401f0() {
}

// 0x40270 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX8ViewBaseEPNS7_7IMetricEdEENS3_5list3INS3_5valueIPS8_EENSD_IPN10RobloxView9RenderJobEEENSD_IdEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list3<boost::_bi::value<RBX::ViewBase*>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)
// IDA 0x40270: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40270() {
}

// 0x4027c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double> &,boost::_bi::list0 &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<RBX::ViewBase *>,boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ViewBase,RBX::IMetric *,double> &,boost::_bi::list0 &,int)
// IDA 0x4027c: 15 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4027c() {
}

// 0x402a8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x402a8: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_402a8() {
}

// 0x40308 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvPN10RobloxView9RenderJobEPN3RBX8ViewBaseEENS3_5list2INS3_5valueIS7_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView::RenderJob *,RBX::ViewBase *),boost::_bi::list2<boost::_bi::value<RobloxView::RenderJob *>,boost::_bi::value<RBX::ViewBase *>>>,void>::invoke(boost::detail::function::function_buffer &)
// IDA 0x40308: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40308() {
}

// 0xd2e00 — __ZN4FMOD15OutputCoreAudio12updateRenderEmP15AudioBufferList
// type: _DWORD __fastcall(FMOD::OutputCoreAudio *__hidden this, unsigned int, AudioBufferList *)
#[doc(alias = "FMOD::OutputCoreAudio::updateRender(unsigned long,AudioBufferList *)")]
// was: FMOD::OutputCoreAudio::updateRender(unsigned long,AudioBufferList *)
// IDA 0xd2e00: 51 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2e00() {
}

// 0x1d9a80 — _FT_Lookup_Renderer
#[doc(alias = "_FT_Lookup_Renderer")]
// was: _FT_Lookup_Renderer
// IDA 0x1d9a80: 23 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1d9a80() {
}

// 0x1dcc68 — _FT_Set_Renderer
#[doc(alias = "_FT_Set_Renderer")]
// was: _FT_Set_Renderer
// IDA 0x1dcc68: 89 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1dcc68() {
}

// 0x1dcdd0 — _FT_Render_Glyph_Internal
#[doc(alias = "_FT_Render_Glyph_Internal")]
// was: _FT_Render_Glyph_Internal
// IDA 0x1dcdd0: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1dcdd0() {
}

// 0x1dcec0 — _FT_Render_Glyph
#[doc(alias = "_FT_Render_Glyph")]
// was: _FT_Render_Glyph
// IDA 0x1dcec0: 10 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1dcec0() {
}

// 0x20b5dc — _Render_Single_Pass
#[doc(alias = "_Render_Single_Pass")]
// was: _Render_Single_Pass
// IDA 0x20b5dc: 725 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_20b5dc() {
}

// 0x20c130 — _Render_Glyph
#[doc(alias = "_Render_Glyph")]
// was: _Render_Glyph
// IDA 0x20c130: 96 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_20c130() {
}

// 0x253f70 — __ZN3RBX10RbxDbgInfo14SetGfxCardNameEPKc
// type: int __fastcall(RBX::RbxDbgInfo *this, const char *)
#[doc(alias = "RBX::RbxDbgInfo::SetGfxCardName(char const*)")]
// was: RBX::RbxDbgInfo::SetGfxCardName(char const*)
// IDA 0x253f70: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_253f70() {
}

// 0x253f94 — __ZN3RBX10RbxDbgInfo23SetGfxCardDriverVersionEPKc
// type: int __fastcall(RBX::RbxDbgInfo *this, const char *)
#[doc(alias = "RBX::RbxDbgInfo::SetGfxCardDriverVersion(char const*)")]
// was: RBX::RbxDbgInfo::SetGfxCardDriverVersion(char const*)
// IDA 0x253f94: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_253f94() {
}

// 0x253fb8 — __ZN3RBX10RbxDbgInfo16SetGfxCardVendorEPKc
// type: int __fastcall(RBX::RbxDbgInfo *this, const char *)
#[doc(alias = "RBX::RbxDbgInfo::SetGfxCardVendor(char const*)")]
// was: RBX::RbxDbgInfo::SetGfxCardVendor(char const*)
// IDA 0x253fb8: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_253fb8() {
}

// 0x2d448c — __ZN3RBX15AdvMoveToolBase8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::AdvMoveToolBase::render2d(RBX::Adorn *)")]
// was: RBX::AdvMoveToolBase::render2d(RBX::Adorn *)
// IDA 0x2d448c: 95 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d448c() {
}

// 0x2d470c — __ZThn4_N3RBX15AdvMoveToolBase8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk to RBX::AdvMoveToolBase::render2d(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::AdvMoveToolBase::render2d(RBX::Adorn *)
// IDA 0x2d470c: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d470c() {
}

// 0x2d4714 — __ZN3RBX15AdvMoveToolBase13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::AdvMoveToolBase::render3dAdorn(RBX::Adorn *)")]
// was: RBX::AdvMoveToolBase::render3dAdorn(RBX::Adorn *)
// IDA 0x2d4714: 111 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d4714() {
}

// 0x2d4874 — __ZThn4_N3RBX15AdvMoveToolBase13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk to RBX::AdvMoveToolBase::render3dAdorn(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::AdvMoveToolBase::render3dAdorn(RBX::Adorn *)
// IDA 0x2d4874: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d4874() {
}

// 0x2d4ac4 — __ZN3RBX11AdvMoveTool8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AdvMoveTool *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::AdvMoveTool::render2d(RBX::Adorn *)")]
// was: RBX::AdvMoveTool::render2d(RBX::Adorn *)
// IDA 0x2d4ac4: 221 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d4ac4() {
}

// 0x2d504c — __ZThn4_N3RBX11AdvMoveTool8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AdvMoveTool *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk to RBX::AdvMoveTool::render2d(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::AdvMoveTool::render2d(RBX::Adorn *)
// IDA 0x2d504c: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d504c() {
}

// 0x2d51bc — __ZN3RBX9DrawAdorn11resizeColorEv
// type: _DWORD __fastcall(RBX::DrawAdorn *__hidden this)
#[doc(alias = "RBX::DrawAdorn::resizeColor(void)")]
// was: RBX::DrawAdorn::resizeColor(void)
// IDA 0x2d51bc: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d51bc() {
}

// 0x2d5890 — __ZN3RBX13AdvRotateTool8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AdvRotateTool *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::AdvRotateTool::render2d(RBX::Adorn *)")]
// was: RBX::AdvRotateTool::render2d(RBX::Adorn *)
// IDA 0x2d5890: 139 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d5890() {
}

// 0x2d5a28 — __ZThn4_N3RBX13AdvRotateTool8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AdvRotateTool *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk to RBX::AdvRotateTool::render2d(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::AdvRotateTool::render2d(RBX::Adorn *)
// IDA 0x2d5a28: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d5a28() {
}

// 0x2d5a30 — __ZN3RBX13AdvRotateTool13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AdvRotateTool *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::AdvRotateTool::render3dAdorn(RBX::Adorn *)")]
// was: RBX::AdvRotateTool::render3dAdorn(RBX::Adorn *)
// IDA 0x2d5a30: 303 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d5a30() {
}

// 0x2d5d98 — __ZThn4_N3RBX13AdvRotateTool13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AdvRotateTool *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk to RBX::AdvRotateTool::render3dAdorn(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::AdvRotateTool::render3dAdorn(RBX::Adorn *)
// IDA 0x2d5d98: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d5d98() {
}

// 0x2d9d4c — __ZNK3RBX13AdvRunDragger19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::shouldRender3dAdorn(void)const")]
// was: RBX::AdvRunDragger::shouldRender3dAdorn(void)const
// IDA 0x2d9d4c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d9d4c() {
}

// 0x2dac5c — __ZN3RBX12AxisToolBase8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AxisToolBase *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::AxisToolBase::render2d(RBX::Adorn *)")]
// was: RBX::AxisToolBase::render2d(RBX::Adorn *)
// IDA 0x2dac5c: 101 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dac5c() {
}

// 0x2daee8 — __ZThn4_N3RBX12AxisToolBase8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AxisToolBase *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk to RBX::AxisToolBase::render2d(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::AxisToolBase::render2d(RBX::Adorn *)
// IDA 0x2daee8: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2daee8() {
}

// 0x2daef0 — __ZN3RBX12AxisToolBase13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AxisToolBase *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::AxisToolBase::render3dAdorn(RBX::Adorn *)")]
// was: RBX::AxisToolBase::render3dAdorn(RBX::Adorn *)
// IDA 0x2daef0: 116 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2daef0() {
}

// 0x2db050 — __ZThn4_N3RBX12AxisToolBase13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::AxisToolBase *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk to RBX::AxisToolBase::render3dAdorn(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::AxisToolBase::render3dAdorn(RBX::Adorn *)
// IDA 0x2db050: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2db050() {
}

// 0x2e4a5c — __ZN3RBX10HammerTool13render3dAdornEPNS_5AdornE
#[doc(alias = "RBX::HammerTool::render3dAdorn(RBX::Adorn *)")]
// was: RBX::HammerTool::render3dAdorn(RBX::Adorn *)
// IDA 0x2e4a5c: 8 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4a5c() {
}

// 0x2e4a70 — __ZThn4_N3RBX10HammerTool13render3dAdornEPNS_5AdornE
#[doc(alias = "non-virtual thunk to RBX::HammerTool::render3dAdorn(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::HammerTool::render3dAdorn(RBX::Adorn *)
// IDA 0x2e4a70: 8 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e4a70() {
}

// 0x2ec558 — __ZN3RBX18MoveResizeJoinTool13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::MoveResizeJoinTool *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::MoveResizeJoinTool::render3dAdorn(RBX::Adorn *)")]
// was: RBX::MoveResizeJoinTool::render3dAdorn(RBX::Adorn *)
// IDA 0x2ec558: 249 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ec558() {
}

// 0x2ec7e4 — __ZThn4_N3RBX18MoveResizeJoinTool13render3dAdornEPNS_5AdornE
// type: int __fastcall(RBX::MoveResizeJoinTool *this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk to RBX::MoveResizeJoinTool::render3dAdorn(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::MoveResizeJoinTool::render3dAdorn(RBX::Adorn *)
// IDA 0x2ec7e4: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ec7e4() {
}

// 0x2ec7ec — __ZN3RBX18MoveResizeJoinTool8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::MoveResizeJoinTool *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::MoveResizeJoinTool::render2d(RBX::Adorn *)")]
// was: RBX::MoveResizeJoinTool::render2d(RBX::Adorn *)
// IDA 0x2ec7ec: 1457 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ec7ec() {
}

// 0x2ed9d4 — __ZThn4_N3RBX18MoveResizeJoinTool8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::MoveResizeJoinTool *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk to RBX::MoveResizeJoinTool::render2d(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::MoveResizeJoinTool::render2d(RBX::Adorn *)
// IDA 0x2ed9d4: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ed9d4() {
}

// 0x2efef0 — __ZN3RBX11NewNullTool13render3dAdornEPNS_5AdornE
#[doc(alias = "RBX::NewNullTool::render3dAdorn(RBX::Adorn *)")]
// was: RBX::NewNullTool::render3dAdorn(RBX::Adorn *)
// IDA 0x2efef0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2efef0() {
}

// 0x2efef4 — __ZThn4_N3RBX11NewNullTool13render3dAdornEPNS_5AdornE
#[doc(alias = "non-virtual thunk to RBX::NewNullTool::render3dAdorn(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::NewNullTool::render3dAdorn(RBX::Adorn *)
// IDA 0x2efef4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2efef4() {
}

// 0x2f0410 — __ZNK3RBX11NewNullTool19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this)
#[doc(alias = "RBX::NewNullTool::shouldRender3dAdorn(void)const")]
// was: RBX::NewNullTool::shouldRender3dAdorn(void)const
// IDA 0x2f0410: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0410() {
}

// 0x2f0414 — __ZThn4_NK3RBX11NewNullTool19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::NewNullTool::shouldRender3dAdorn(void)const")]
// was: non-virtual thunk to RBX::NewNullTool::shouldRender3dAdorn(void)const
// IDA 0x2f0414: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0414() {
}

// 0x2f12c0 — __ZN3RBX12PartDragTool13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::PartDragTool *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::PartDragTool::render3dAdorn(RBX::Adorn *)")]
// was: RBX::PartDragTool::render3dAdorn(RBX::Adorn *)
// IDA 0x2f12c0: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f12c0() {
}

// 0x2f13d0 — __ZThn4_N3RBX12PartDragTool13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::PartDragTool *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk to RBX::PartDragTool::render3dAdorn(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::PartDragTool::render3dAdorn(RBX::Adorn *)
// IDA 0x2f13d0: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f13d0() {
}

// 0x2f6850 — __ZN3RBX13ArrowToolBase13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::ArrowToolBase::render3dAdorn(RBX::Adorn *)")]
// was: RBX::ArrowToolBase::render3dAdorn(RBX::Adorn *)
// IDA 0x2f6850: 2 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f6850() {
}

// 0x2f6858 — __ZN3RBX13ArrowToolBase15renderHoverOverEPNS_5AdornEb
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, RBX::Adorn *, bool)
#[doc(alias = "RBX::ArrowToolBase::renderHoverOver(RBX::Adorn *,bool)")]
// was: RBX::ArrowToolBase::renderHoverOver(RBX::Adorn *,bool)
// IDA 0x2f6858: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f6858() {
}

// 0x2f68c8 — __ZThn4_N3RBX13ArrowToolBase13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk to RBX::ArrowToolBase::render3dAdorn(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::ArrowToolBase::render3dAdorn(RBX::Adorn *)
// IDA 0x2f68c8: 3 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f68c8() {
}

// 0x2f7818 — __ZN3RBX16BoxSelectCommand8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::BoxSelectCommand *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::BoxSelectCommand::render2d(RBX::Adorn *)")]
// was: RBX::BoxSelectCommand::render2d(RBX::Adorn *)
// IDA 0x2f7818: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f7818() {
}

// 0x2f78d0 — __ZThn4_N3RBX16BoxSelectCommand8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::BoxSelectCommand *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk to RBX::BoxSelectCommand::render2d(RBX::Adorn *)")]
// was: non-virtual thunk to RBX::BoxSelectCommand::render2d(RBX::Adorn *)
// IDA 0x2f78d0: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f78d0() {
}

// 0x3047f0 — __ZN3RBX10BrickColor8BrickMap32setRenderingSupportedPaletteSizeEm
// type: _DWORD __fastcall(RBX::BrickColor::BrickMap *__hidden this, unsigned int)
#[doc(alias = "RBX::BrickColor::BrickMap::setRenderingSupportedPaletteSize(unsigned long)")]
// was: RBX::BrickColor::BrickMap::setRenderingSupportedPaletteSize(unsigned long)
// IDA 0x3047f0: 346 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3047f0() {
}

// 0x38c6b4 — __ZN3RBX13UserInputBase13getGameCursorEPNS_5AdornE
// type: void __fastcall(RBX::UserInputBase *this, const shared_count *, int)
#[doc(alias = "RBX::UserInputBase::getGameCursor(RBX::Adorn *)")]
// was: RBX::UserInputBase::getGameCursor(RBX::Adorn *)
// IDA 0x38c6b4: 229 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38c6b4() {
}

// 0x38c928 — __ZN3RBX13UserInputBase11setCursorIdEPNS_5AdornERKNS_9TextureIdE
// type: int __fastcall(RBX::UserInputBase *this, RBX::Adorn *, const RBX::TextureId *)
#[doc(alias = "RBX::UserInputBase::setCursorId(RBX::Adorn *,RBX::TextureId const&)")]
// was: RBX::UserInputBase::setCursorId(RBX::Adorn *,RBX::TextureId const&)
// IDA 0x38c928: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38c928() {
}

// 0x38c974 — __ZN3RBX13UserInputBase16renderGameCursorEPNS_5AdornE
// type: void __fastcall(RBX::UserInputBase *this, RBX::Adorn *)
#[doc(alias = "RBX::UserInputBase::renderGameCursor(RBX::Adorn *)")]
// was: RBX::UserInputBase::renderGameCursor(RBX::Adorn *)
// IDA 0x38c974: 191 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38c974() {
}

// 0x38efa0 — __ZN3RBX12Accoutrement14render3dSelectEPNS_5AdornENS_11SelectStateE
// type: unsigned int __fastcall(RBX::Instance *, int, int)
#[doc(alias = "RBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
// was: RBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)
// IDA 0x38efa0: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38efa0() {
}

// 0x38f014 — __ZThn104_N3RBX12Accoutrement14render3dSelectEPNS_5AdornENS_11SelectStateE
// type: unsigned int __fastcall(int, int, int)
#[doc(alias = "non-virtual thunk to RBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
// was: non-virtual thunk to RBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)
// IDA 0x38f014: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f014() {
}

// 0x39066c — __ZN3RBX12Accoutrement17getRenderLocationEv
// type: int __fastcall(RBX::Accoutrement *this, int)
#[doc(alias = "RBX::Accoutrement::getRenderLocation(void)")]
// was: RBX::Accoutrement::getRenderLocation(void)
// IDA 0x39066c: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39066c() {
}

// 0x39067c — __ZN3RBX12Accoutrement13getRenderSizeEv
// type: int __fastcall(RBX::Accoutrement *this, RBX::Accoutrement *)
#[doc(alias = "RBX::Accoutrement::getRenderSize(void)")]
// was: RBX::Accoutrement::getRenderSize(void)
// IDA 0x39067c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39067c() {
}

// 0x3906b4 — __ZThn128_N3RBX12Accoutrement17getRenderLocationEv
// type: int __fastcall(RBX::Accoutrement *this, int)
#[doc(alias = "non-virtual thunk to RBX::Accoutrement::getRenderLocation(void)")]
// was: non-virtual thunk to RBX::Accoutrement::getRenderLocation(void)
// IDA 0x3906b4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3906b4() {
}

// 0x3906c4 — __ZThn128_N3RBX12Accoutrement13getRenderSizeEv
// type: int __fastcall(RBX::Accoutrement *this)
#[doc(alias = "non-virtual thunk to RBX::Accoutrement::getRenderSize(void)")]
// was: non-virtual thunk to RBX::Accoutrement::getRenderSize(void)
// IDA 0x3906c4: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3906c4() {
}

// 0x393b34 — __ZN3RBX13PartAdornment10setAdorneeEPNS_12PartInstanceE
// type: void __fastcall(RBX::PartAdornment *this, RBX::PartInstance *)
#[doc(alias = "RBX::PartAdornment::setAdornee(RBX::PartInstance *)")]
// was: RBX::PartAdornment::setAdornee(RBX::PartInstance *)
// IDA 0x393b34: 94 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_393b34() {
}

// 0x393c44 — __ZN3RBX13PartAdornmentC2EPKc
// type: RBX::GuiBase3d *__fastcall(RBX::PartAdornment *this, const char *)
#[doc(alias = "RBX::PartAdornment::PartAdornment(char const*)")]
// was: RBX::PartAdornment::PartAdornment(char const*)
// IDA 0x393c44: 137 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_393c44() {
}

// 0x393dd0 — __ZN3RBX11PVAdornment10setAdorneeEPNS_10PVInstanceE
// type: void __fastcall(RBX::PVAdornment *this, RBX::PVInstance *)
#[doc(alias = "RBX::PVAdornment::setAdornee(RBX::PVInstance *)")]
// was: RBX::PVAdornment::setAdornee(RBX::PVInstance *)
// IDA 0x393dd0: 94 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_393dd0() {
}

// 0x393ee0 — __ZN3RBX11PVAdornmentC2EPKc
// type: RBX::GuiBase3d *__fastcall(RBX::PVAdornment *this, const char *)
#[doc(alias = "RBX::PVAdornment::PVAdornment(char const*)")]
// was: RBX::PVAdornment::PVAdornment(char const*)
// IDA 0x393ee0: 137 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_393ee0() {
}

// 0x39406c — __ZNK3RBX13PartAdornment19getAdorneeDangerousEv
// type: int __fastcall(RBX::PartAdornment *this)
#[doc(alias = "RBX::PartAdornment::getAdorneeDangerous(void)const")]
// was: RBX::PartAdornment::getAdorneeDangerous(void)const
// IDA 0x39406c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39406c() {
}

// 0x394090 — __ZN3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::~RefPropDescriptor()")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::~RefPropDescriptor()
// IDA 0x394090: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_394090() {
}

// 0x3940bc — __ZNK3RBX11PVAdornment19getAdorneeDangerousEv
// type: int __fastcall(RBX::PVAdornment *this)
#[doc(alias = "RBX::PVAdornment::getAdorneeDangerous(void)const")]
// was: RBX::PVAdornment::getAdorneeDangerous(void)const
// IDA 0x3940bc: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3940bc() {
}

// 0x3940e0 — __ZN3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::~RefPropDescriptor()")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::~RefPropDescriptor()
// IDA 0x3940e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3940e0() {
}

// 0x39427c — __ZN3RBX11PVAdornmentD1Ev
// type: void __fastcall(RBX::PVAdornment *__hidden this)
#[doc(alias = "RBX::PVAdornment::~PVAdornment()")]
// was: RBX::PVAdornment::~PVAdornment()
// IDA 0x39427c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_39427c() {
}

// 0x3943c4 — __ZN3RBX11PVAdornmentD0Ev
// type: void __fastcall(RBX::PVAdornment *__hidden this)
#[doc(alias = "RBX::PVAdornment::~PVAdornment()")]
// was: RBX::PVAdornment::~PVAdornment()
// IDA 0x3943c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3943c4() {
}

// 0x394464 — __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv
// IDA 0x394464: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_394464() {
}

// 0x39448c — __ZThn32_N3RBX11PVAdornmentD1Ev
// type: void __fastcall(RBX::PVAdornment *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::PVAdornment::~PVAdornment()")]
// was: non-virtual thunk to RBX::PVAdornment::~PVAdornment()
// IDA 0x39448c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_39448c() {
}

// 0x3945d4 — __ZThn32_N3RBX11PVAdornmentD0Ev
// type: void __fastcall(RBX::PVAdornment *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::PVAdornment::~PVAdornment()")]
// was: non-virtual thunk to RBX::PVAdornment::~PVAdornment()
// IDA 0x3945d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3945d4() {
}

// 0x394730 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv
// IDA 0x394730: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_394730() {
}

// 0x394758 — __ZThn36_N3RBX11PVAdornmentD1Ev
// type: void __fastcall(RBX::PVAdornment *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::PVAdornment::~PVAdornment()")]
// was: non-virtual thunk to RBX::PVAdornment::~PVAdornment()
// IDA 0x394758: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_394758() {
}

// 0x3948a0 — __ZThn36_N3RBX11PVAdornmentD0Ev
// type: void __fastcall(RBX::PVAdornment *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::PVAdornment::~PVAdornment()")]
// was: non-virtual thunk to RBX::PVAdornment::~PVAdornment()
// IDA 0x3948a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3948a0() {
}

// 0x3949fc — __ZN3RBX4Name13callDoDeclareILZNS_12sPVAdornmentEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sPVAdornmentEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_12sPVAdornmentEEEEvv
// IDA 0x3949fc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3949fc() {
}

// 0x394a00 — __ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v
// IDA 0x394a00: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_394a00() {
}

// 0x394ae0 — __ZN3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x394ae0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_394ae0() {
}

// 0x394b9c — __ZN3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x394b9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_394b9c() {
}

// 0x394c68 — __ZThn32_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x394c68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_394c68() {
}

// 0x394d20 — __ZThn32_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x394d20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_394d20() {
}

// 0x394df0 — __ZThn36_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x394df0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_394df0() {
}

// 0x394ea8 — __ZThn36_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_11PVAdornmentELZNS_12sPVAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x394ea8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_394ea8() {
}

// 0x394f78 — __ZN3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, char, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::RefPropDescriptor<RBX::PVInstance* (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance*)>(char const*,char const*,RBX::PVInstance* (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::RefPropDescriptor<RBX::PVInstance* (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance*)>(char const*,char const*,RBX::PVInstance* (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x394f78: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_394f78() {
}

// 0x395114 — __ZN3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::~RefPropDescriptor()")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::~RefPropDescriptor()
// IDA 0x395114: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_395114() {
}

// 0x395144 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::isReadOnly(void)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::isReadOnly(void)const
// IDA 0x395144: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395144() {
}

// 0x395154 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::isWriteOnly(void)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::isWriteOnly(void)const
// IDA 0x395154: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395154() {
}

// 0x395164 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x395164: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395164() {
}

// 0x39518c — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x39518c: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39518c() {
}

// 0x3952a4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x3952a4: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3952a4() {
}

// 0x39536c — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x39536c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39536c() {
}

// 0x395390 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// IDA 0x395390: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395390() {
}

// 0x395464 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// IDA 0x395464: 15 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395464() {
}

// 0x395488 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE11getRefValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::getRefValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x395488: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395488() {
}

// 0x39549c — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const
// IDA 0x39549c: 41 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39549c() {
}

// 0x395518 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const
// IDA 0x395518: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395518() {
}

// 0x395538 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: void __fastcall(int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const
// IDA 0x395538: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395538() {
}

// 0x395618 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_11PVAdornmentENS_10PVInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk to RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: non-virtual thunk to RBX::Reflection::RefPropDescriptor<RBX::PVAdornment,RBX::PVInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const
// IDA 0x395618: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395618() {
}

// 0x395620 — __ZNK3RBX10Reflection14PropDescriptorINS_11PVAdornmentEPNS_10PVInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVAdornment,RBX::PVInstance *>::GetSetImpl<RBX::PVInstance * (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance *)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PVAdornment,RBX::PVInstance *>::GetSetImpl<RBX::PVInstance * (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance *)>::isReadOnly(void)const
// IDA 0x395620: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395620() {
}

// 0x395624 — __ZNK3RBX10Reflection14PropDescriptorINS_11PVAdornmentEPNS_10PVInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVAdornment,RBX::PVInstance *>::GetSetImpl<RBX::PVInstance * (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance *)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PVAdornment,RBX::PVInstance *>::GetSetImpl<RBX::PVInstance * (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance *)>::isWriteOnly(void)const
// IDA 0x395624: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395624() {
}

// 0x395628 — __ZNK3RBX10Reflection14PropDescriptorINS_11PVAdornmentEPNS_10PVInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVAdornment,RBX::PVInstance *>::GetSetImpl<RBX::PVInstance * (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PVAdornment,RBX::PVInstance *>::GetSetImpl<RBX::PVInstance * (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x395628: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395628() {
}

// 0x395648 — __ZNK3RBX10Reflection14PropDescriptorINS_11PVAdornmentEPNS_10PVInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PVAdornment,RBX::PVInstance *>::GetSetImpl<RBX::PVInstance * (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PVInstance * const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PVAdornment,RBX::PVInstance *>::GetSetImpl<RBX::PVInstance * (RBX::PVAdornment::*)(void)const,void (RBX::PVAdornment::*)(RBX::PVInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PVInstance * const&)const
// IDA 0x395648: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395648() {
}

// 0x395720 — __ZN3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x395720: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_395720() {
}

// 0x3957dc — __ZN3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3957dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3957dc() {
}

// 0x3958a8 — __ZThn32_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3958a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3958a8() {
}

// 0x395960 — __ZThn32_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x395960: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_395960() {
}

// 0x395a30 — __ZThn36_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x395a30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_395a30() {
}

// 0x395ae8 — __ZThn36_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13PartAdornmentELZNS_14sPartAdornmentEENS_17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x395ae8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_395ae8() {
}

// 0x395bb8 — __ZN3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, char, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::PartAdornment::*)(void)const,void (RBX::PartAdornment::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x395bb8: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395bb8() {
}

// 0x395c5c — __ZN3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::~RefPropDescriptor()")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::~RefPropDescriptor()
// IDA 0x395c5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_395c5c() {
}

// 0x395c8c — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::isReadOnly(void)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::isReadOnly(void)const
// IDA 0x395c8c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395c8c() {
}

// 0x395c9c — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::isWriteOnly(void)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::isWriteOnly(void)const
// IDA 0x395c9c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395c9c() {
}

// 0x395cac — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x395cac: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395cac() {
}

// 0x395cd4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x395cd4: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395cd4() {
}

// 0x395dec — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x395dec: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395dec() {
}

// 0x395eb4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x395eb4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395eb4() {
}

// 0x395ed8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// IDA 0x395ed8: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395ed8() {
}

// 0x395fac — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// IDA 0x395fac: 15 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395fac() {
}

// 0x395fd0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE11getRefValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::getRefValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x395fd0: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395fd0() {
}

// 0x395fe4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const
// IDA 0x395fe4: 41 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_395fe4() {
}

// 0x396060 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const
// IDA 0x396060: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_396060() {
}

// 0x396080 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13PartAdornmentENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: void __fastcall(int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: RBX::Reflection::RefPropDescriptor<RBX::PartAdornment,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const
// IDA 0x396080: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_396080() {
}