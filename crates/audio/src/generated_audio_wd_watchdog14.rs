//! audio generated_audio_wd_watchdog14 — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x06589e8 | rbx_core::SharedPtr not boost
//! Range 0x06589f4..0x065a8ac | existing 36403 -> 36503 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
use crate::generated::flog_asserts;
use crate::generated_audio_wd_watchdog13::SurfaceState;
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

/// `NormalId` face count behind the `*Static` selectors (IDA
/// 0x658be0-0x658d90): faces 0..=5 index the six per-face statics;
/// anything else hits the `ReleaseAssert` (Surface.cpp lines
/// 411/432/453/474).
pub const SURFACE_FACE_COUNT: u32 = 6;
/// Per-face surface slot: the type/input ids plus the float params.
/// The live data behind the six static descriptor families (IDA
/// 0x658be0: type, 0x658c70: input, 0x658d00: param A, 0x658d90:
/// param B). Raw ids — no variant names are grounded in this range.
#[derive(Debug, Clone, Copy, Default)]
pub struct FaceSurface {
    pub surface_type: u32,
    pub surface_input: u32,
    pub param_a: f32,
    pub param_b: f32,
}
/// Float slot selected by a `SurfacePropDescriptor`'s member-pointer
/// pair (IDA 0x6594c4/0x6594e4: the face is baked in as `, 2`, the
/// member selects ParamA vs ParamB).
#[derive(Debug, Clone, Copy)]
pub enum FaceFloatSlot {
    ParamA,
    ParamB,
}
impl FaceSurface {
    pub fn float_slot(&self, slot: FaceFloatSlot) -> f32 {
        match slot {
            FaceFloatSlot::ParamA => self.param_a,
            FaceFloatSlot::ParamB => self.param_b,
        }
    }
    pub fn set_float_slot(&mut self, slot: FaceFloatSlot, value: f32) {
        match slot {
            FaceFloatSlot::ParamA => self.param_a = value,
            FaceFloatSlot::ParamB => self.param_b = value,
        }
    }
}
/// The six per-face slots of a part, indexed by `NormalId`.
#[derive(Debug, Clone, Default)]
pub struct PartSurfaceData {
    pub faces: [FaceSurface; SURFACE_FACE_COUNT as usize],
}
/// `RBX::Reflection::Variant` payload read by the surface
/// `genericConvert` cutovers (IDA 0x658e24/0x658f8c): either the enum
/// value or something else (the string cleanup at 0x658ebc-0x659002
/// still throws).
#[derive(Debug, Clone, Copy)]
pub enum SurfaceVariant {
    SurfaceType(u32),
    SurfaceInput(u32),
    Other,
}
/// Descriptor family behind the four `*Static` selector groups (IDA
/// 0x658be0/0x658c70/0x658d00/0x658d90).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceDescriptorFamily {
    SurfaceType,
    SurfaceInput,
    ParamA,
    ParamB,
}
/// Static descriptor id: family + face. `isSurfaceDescriptor` (IDA
/// 0x658b70) compares against the six `SurfaceType` statics — the
/// same six targets as the 0x658be0 switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceDescriptor {
    pub family: SurfaceDescriptorFamily,
    pub face: u32,
}
/// Shared body of the four `*Static` face selectors (IDA
/// 0x658be0/0x658c70/0x658d00/0x658d90): faces 0..=5 return their
/// static; anything else hits the `ReleaseAssert` (host seam) then
/// falls back to the face-1 static. `line`/`ea` identify the
/// selector for the assert message.
fn surface_static_descriptor(
    family: SurfaceDescriptorFamily,
    face: u32,
    line: u32,
    ea: u32,
) -> SurfaceDescriptor {
    if face >= SURFACE_FACE_COUNT {
        if flog_asserts() {
            panic!(
                "false file: Client/App/v8datamodel/Surface.cpp line: {} (IDA {:#x})",
                line, ea
            );
        }
        return SurfaceDescriptor { family, face: 1 };
    }
    SurfaceDescriptor { family, face }
}
/// `RBX::SurfacePropDescriptor<2, float>` cutover (IDA 0x65937c):
/// face baked in, getter/setter member-pointer pair folds into the
/// slot selector (same shape as `StudioToolBoolProp` at 0x6579d0).
#[derive(Debug, Clone)]
pub struct FaceFloatProp {
    pub face: u32,
    pub slot: FaceFloatSlot,
    pub name: String,
    pub category: String,
    pub functionality: u32,
    pub permissions: u32,
}
impl FaceFloatProp {
    pub fn new(
        face: u32,
        slot: FaceFloatSlot,
        name: &str,
        category: &str,
        functionality: u32,
        permissions: u32,
    ) -> Self {
        Self {
            face,
            slot,
            name: name.to_owned(),
            category: category.to_owned(),
            functionality,
            permissions,
        }
    }
}
/// `RBX::SurfaceEnumPropDescriptor<2, LegacyController::InputType>`
/// cutover (IDA 0x659508): face baked in, member pair folds away;
/// the inner `GetSet` (+44) reads the live face slot.
#[derive(Debug, Clone)]
pub struct FaceInputProp {
    pub face: u32,
    pub name: String,
    pub category: String,
    pub functionality: u32,
}
impl FaceInputProp {
    pub fn new(face: u32, name: &str, category: &str, functionality: u32) -> Self {
        Self {
            face,
            name: name.to_owned(),
            category: category.to_owned(),
            functionality,
        }
    }
}
// 0x06589f4 — __ZN3RBX7SurfaceC1Ev
// demangled: RBX::Surface::Surface(void)
// type: _DWORD __fastcall(RBX::Surface *__hidden this)
#[doc(alias = "RBX::Surface::Surface(void)")]
#[doc(alias = "__ZN3RBX7SurfaceC1Ev")]
pub fn stub_06589f4() -> SurfaceState {
    // IDA 0x6589f4 (`RBX::Surface::Surface(void)`): zeroes the
    // 8-byte pair (0x6589f8). Host: the cleared cutover.
    SurfaceState {
        part_present: false,
        normal: 0,
    }
}

// 0x0658a00 — __ZN3RBX7Surface14setSurfaceTypeENS_11SurfaceTypeE
// demangled: RBX::Surface::setSurfaceType(RBX::SurfaceType)
#[doc(alias = "RBX::Surface::setSurfaceType(RBX::SurfaceType)")]
#[doc(alias = "__ZN3RBX7Surface14setSurfaceTypeENS_11SurfaceTypeE")]
pub fn stub_0658a00(
    part: &mut PartSurfaceData,
    surface: &SurfaceState,
    surface_type: u32,
) {
    // IDA 0x658a00 (`RBX::Surface::setSurfaceType`, disasm `MOV R2,
    // R1; LDRD.W R0, R1, [R0]; B`): forwards the pair's part/normal
    // plus the type to `PartInstance::setSurfaceType`. The returned
    // `SurfaceData*` (chained by `flat` at 0x658a3c) folds into the
    // slot store.
    part.faces[surface.normal as usize].surface_type = surface_type;
}

// 0x0658a0c — __ZN3RBX7Surface15setSurfaceInputENS_16LegacyController9InputTypeE
// demangled: RBX::Surface::setSurfaceInput(RBX::LegacyController::InputType)
#[doc(alias = "RBX::Surface::setSurfaceInput(RBX::LegacyController::InputType)")]
#[doc(alias = "__ZN3RBX7Surface15setSurfaceInputENS_16LegacyController9InputTypeE")]
pub fn stub_0658a0c(part: &mut PartSurfaceData, surface: &SurfaceState, input: u32) {
    // IDA 0x658a0c (`RBX::Surface::setSurfaceInput`): forwards the
    // pair's part/normal plus the input to
    // `PartInstance::setSurfaceInput` (same shape as 0x658a00).
    part.faces[surface.normal as usize].surface_input = input;
}

// 0x0658a18 — __ZN3RBX7Surface9setParamAEf
// demangled: RBX::Surface::setParamA(float)
// type: _DWORD __fastcall(RBX::Surface *__hidden this, float)
#[doc(alias = "RBX::Surface::setParamA(float)")]
#[doc(alias = "__ZN3RBX7Surface9setParamAEf")]
pub fn stub_0658a18(part: &mut PartSurfaceData, surface: &SurfaceState, value: f32) {
    // IDA 0x658a18 (`RBX::Surface::setParamA`, disasm `MOV R2, R1;
    // LDRD.W R0, R1, [R0]; B` — the decompiler drops the forwarded
    // float): forwards the pair's part/normal plus the value to
    // `PartInstance::setParamA(NormalId, float)`.
    part.faces[surface.normal as usize].param_a = value;
}

// 0x0658a24 — __ZN3RBX7Surface9setParamBEf
// demangled: RBX::Surface::setParamB(float)
// type: _DWORD __fastcall(RBX::Surface *__hidden this, float)
#[doc(alias = "RBX::Surface::setParamB(float)")]
#[doc(alias = "__ZN3RBX7Surface9setParamBEf")]
pub fn stub_0658a24(part: &mut PartSurfaceData, surface: &SurfaceState, value: f32) {
    // IDA 0x658a24 (`RBX::Surface::setParamB`): forwards the pair's
    // part/normal plus the value to `PartInstance::setParamB`
    // (same shape as 0x658a18).
    part.faces[surface.normal as usize].param_b = value;
}

// 0x0658a30 — __ZN3RBX7Surface4flatEv
// demangled: RBX::Surface::flat(void)
#[doc(alias = "RBX::Surface::flat(void)")]
#[doc(alias = "__ZN3RBX7Surface4flatEv")]
pub fn stub_0658a30(part: &mut PartSurfaceData, surface: &SurfaceState) {
    // IDA 0x658a30 (`RBX::Surface::flat`): `setSurfaceType` with 0
    // (0x658a34-0x658a3c) + `SurfaceData::empty` (0x658a40), then the
    // input/paramA/paramB sets drawing their values from
    // `SurfaceData::empty`'s static (0x658a44-0x658a70) each followed
    // by `empty`, with `setParamB` tail-called (0x658a72-0x658a76).
    // Net effect: the face slot ends empty. Host: reset the slot.
    part.faces[surface.normal as usize] = FaceSurface::default();
}

// 0x0658a7c — __ZN3RBX10Reflection4Type12getSingletonINS_7SurfaceEEERKS1_v
// demangled: RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Surface>(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Surface>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_7SurfaceEEERKS1_v")]
pub fn stub_0658a7c() {
    // IDA 0x658a7c (`Type::getSingleton<Surface>`): `__cxa_guard`
    // once-init publishing the static (0x658ada-0x658b3e). Host
    // statics initialize on use — carrier no-op (same shape as
    // 0x1a5d0).
}

// 0x0658b6c — __ZN3RBX7Surface26registerSurfaceDescriptorsEv
// demangled: RBX::Surface::registerSurfaceDescriptors(void)
// type: _DWORD __fastcall(RBX::Surface *__hidden this)
#[doc(alias = "RBX::Surface::registerSurfaceDescriptors(void)")]
#[doc(alias = "__ZN3RBX7Surface26registerSurfaceDescriptorsEv")]
pub fn stub_0658b6c() {
    // IDA 0x658b6c (`RBX::Surface::registerSurfaceDescriptors`):
    // empty body — registration folds into the host statics.
}

// 0x0658b70 — __ZN3RBX7Surface19isSurfaceDescriptorERKNS_10Reflection18PropertyDescriptorE
// demangled: RBX::Surface::isSurfaceDescriptor(RBX::Reflection::PropertyDescriptor const&)
// type: _DWORD __fastcall(RBX::Surface *__hidden this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Surface::isSurfaceDescriptor(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX7Surface19isSurfaceDescriptorERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_0658b70(descriptor: &SurfaceDescriptor) -> bool {
    // IDA 0x658b70 (`RBX::Surface::isSurfaceDescriptor`): address
    // compare against the six `SurfaceType` statics (0x658bdc) — the
    // same six targets as the 0x658be0 switch. Host: family + range
    // check.
    descriptor.family == SurfaceDescriptorFamily::SurfaceType
        && descriptor.face < SURFACE_FACE_COUNT
}

// 0x0658be0 — __ZN3RBX7Surface20getSurfaceTypeStaticENS_8NormalIdE
// demangled: RBX::Surface::getSurfaceTypeStatic(RBX::NormalId)
#[doc(alias = "RBX::Surface::getSurfaceTypeStatic(RBX::NormalId)")]
#[doc(alias = "__ZN3RBX7Surface20getSurfaceTypeStaticENS_8NormalIdE")]
pub fn stub_0658be0(face: u32) -> SurfaceDescriptor {
    // IDA 0x658be0 (`getSurfaceTypeStatic`): switch 0..=5 over the
    // six statics (0x658c54-0x658c68); default hits the
    // `ReleaseAssert` (Surface.cpp line 411, host seam) then falls
    // back to the face-1 static (0x658bdc-0x658c06). Host:
    // descriptor id = face.
    surface_static_descriptor(SurfaceDescriptorFamily::SurfaceType, face, 411, 0x658be0)
}

// 0x0658c70 — __ZN3RBX7Surface21getSurfaceInputStaticENS_8NormalIdE
// demangled: RBX::Surface::getSurfaceInputStatic(RBX::NormalId)
#[doc(alias = "RBX::Surface::getSurfaceInputStatic(RBX::NormalId)")]
#[doc(alias = "__ZN3RBX7Surface21getSurfaceInputStaticENS_8NormalIdE")]
pub fn stub_0658c70(face: u32) -> SurfaceDescriptor {
    // IDA 0x658c70 (`getSurfaceInputStatic`): same switch shape over
    // its six statics (0x658ce4-0x658cf8); default asserts
    // (Surface.cpp line 432) then falls back to face 1.
    surface_static_descriptor(SurfaceDescriptorFamily::SurfaceInput, face, 432, 0x658c70)
}

// 0x0658d00 — __ZN3RBX7Surface15getParamAStaticENS_8NormalIdE
// demangled: RBX::Surface::getParamAStatic(RBX::NormalId)
#[doc(alias = "RBX::Surface::getParamAStatic(RBX::NormalId)")]
#[doc(alias = "__ZN3RBX7Surface15getParamAStaticENS_8NormalIdE")]
pub fn stub_0658d00(face: u32) -> SurfaceDescriptor {
    // IDA 0x658d00 (`getParamAStatic`): same switch shape
    // (0x658d74-0x658d88); default asserts (Surface.cpp line 453)
    // then falls back to face 1.
    surface_static_descriptor(SurfaceDescriptorFamily::ParamA, face, 453, 0x658d00)
}

// 0x0658d90 — __ZN3RBX7Surface15getParamBStaticENS_8NormalIdE
// demangled: RBX::Surface::getParamBStatic(RBX::NormalId)
#[doc(alias = "RBX::Surface::getParamBStatic(RBX::NormalId)")]
#[doc(alias = "__ZN3RBX7Surface15getParamBStaticENS_8NormalIdE")]
pub fn stub_0658d90(face: u32) -> SurfaceDescriptor {
    // IDA 0x658d90 (`getParamBStatic`): same switch shape
    // (0x658e04-0x658e18); default asserts (Surface.cpp line 474)
    // then falls back to face 1.
    surface_static_descriptor(SurfaceDescriptorFamily::ParamB, face, 474, 0x658d90)
}

// 0x0658e20 — __ZN3RBX10Reflection5TTypeINS_7SurfaceEED1Ev
// demangled: RBX::Reflection::TType<RBX::Surface>::~TType()
#[doc(alias = "RBX::Reflection::TType<RBX::Surface>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_7SurfaceEED1Ev")]
pub fn stub_0658e20() {
    // IDA 0x0658e20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0658e24 — __ZN3RBX10Reflection7Variant14genericConvertINS_11SurfaceTypeEEERT_v
// demangled: RBX::SurfaceType & RBX::Reflection::Variant::genericConvert<RBX::SurfaceType>(void)
// type: int(void)
#[doc(alias = "RBX::SurfaceType & RBX::Reflection::Variant::genericConvert<RBX::SurfaceType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_11SurfaceTypeEEERT_v")]
pub fn stub_0658e24(variant: &SurfaceVariant) -> u32 {
    // IDA 0x658e24 (`Variant::genericConvert<SurfaceType>`):
    // `any_cast<SurfaceType>` hit returns the value
    // (0x658e78-0x658e96); a miss (after the string cleanup at
    // 0x658ebc-0x659002) throws `runtime_error("Unable to cast %s to
    // %s")` (0x658f0c-0x658f64, host: panic).
    match *variant {
        SurfaceVariant::SurfaceType(value) => value,
        _ => panic!("Unable to cast variant to SurfaceType (IDA 0x658e24)"),
    }
}

// 0x0658f8c — __ZN3RBX10Reflection7Variant14genericConvertINS_16LegacyController9InputTypeEEERT_v
// demangled: RBX::LegacyController::InputType & RBX::Reflection::Variant::genericConvert<RBX::LegacyController::InputType>(void)
// type: int(void)
#[doc(alias = "RBX::LegacyController::InputType & RBX::Reflection::Variant::genericConvert<RBX::LegacyController::InputType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_16LegacyController9InputTypeEEERT_v")]
pub fn stub_0658f8c(variant: &SurfaceVariant) -> u32 {
    // IDA 0x658f8c (`Variant::genericConvert<InputType>`): same
    // any-cast shape (0x658fe0-0x658ffe), throwing on a miss
    // (0x659074-0x6590cc, host: panic).
    match *variant {
        SurfaceVariant::SurfaceInput(value) => value,
        _ => panic!("Unable to cast variant to SurfaceInput (IDA 0x658f8c)"),
    }
}

// 0x06590f4 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEED1Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEED1Ev")]
pub fn stub_06590f4() {
    // IDA 0x06590f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0659118 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED1Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED1Ev")]
pub fn stub_0659118() {
    // IDA 0x0659118: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065913c — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfED1Ev
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::~SurfacePropDescriptor()
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::~SurfacePropDescriptor()")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfED1Ev")]
pub fn stub_065913c() {
    // IDA 0x065913c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0659160 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEED1Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEED1Ev")]
pub fn stub_0659160() {
    // IDA 0x0659160: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0659184 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED1Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED1Ev")]
pub fn stub_0659184() {
    // IDA 0x0659184: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06591a8 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfED1Ev
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)4,float>::~SurfacePropDescriptor()
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)4,float>::~SurfacePropDescriptor()")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfED1Ev")]
pub fn stub_06591a8() {
    // IDA 0x06591a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06591cc — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEED1Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEED1Ev")]
pub fn stub_06591cc() {
    // IDA 0x06591cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06591f0 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEED1Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEED1Ev")]
pub fn stub_06591f0() {
    // IDA 0x06591f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0659214 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE3EfED1Ev
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)3,float>::~SurfacePropDescriptor()
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)3,float>::~SurfacePropDescriptor()")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE3EfED1Ev")]
pub fn stub_0659214() {
    // IDA 0x0659214: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0659238 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEED1Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEED1Ev")]
pub fn stub_0659238() {
    // IDA 0x0659238: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065925c — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEED1Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEED1Ev")]
pub fn stub_065925c() {
    // IDA 0x065925c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0659280 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE0EfED1Ev
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)0,float>::~SurfacePropDescriptor()
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)0,float>::~SurfacePropDescriptor()")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE0EfED1Ev")]
pub fn stub_0659280() {
    // IDA 0x0659280: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06592a4 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEED1Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEED1Ev")]
pub fn stub_06592a4() {
    // IDA 0x06592a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06592c8 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED1Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED1Ev")]
pub fn stub_06592c8() {
    // IDA 0x06592c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06592ec — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE5EfED1Ev
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)5,float>::~SurfacePropDescriptor()
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)5,float>::~SurfacePropDescriptor()")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE5EfED1Ev")]
pub fn stub_06592ec() {
    // IDA 0x06592ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0659310 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEED1Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEED1Ev")]
pub fn stub_0659310() {
    // IDA 0x0659310: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0659334 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED1Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED1Ev")]
pub fn stub_0659334() {
    // IDA 0x0659334: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0659358 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE2EfED1Ev
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)2,float>::~SurfacePropDescriptor()
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)2,float>::~SurfacePropDescriptor()")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE2EfED1Ev")]
pub fn stub_0659358() {
    // IDA 0x0659358: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065937c — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE2EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)2,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)2,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE2EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE")]
pub fn stub_065937c(
    name: &str,
    category: &str,
    slot: FaceFloatSlot,
    functionality: u32,
    permissions: u32,
) -> FaceFloatProp {
    // IDA 0x65937c (`SurfacePropDescriptor<2, float>` ctor): the
    // `PartInstance` `classDescriptor` call (0x6593a2) +
    // `operator new(0x14)` impl holding the vtable and the
    // getter/setter member-pointer pair (0x6593a8-0x6593e6), then the
    // `TypedPropertyDescriptor<float>` base init with
    // name/category/functionality/permissions (0x659422-0x659440).
    // The face is baked in (template `NormalId` 2); the member pair
    // folds into the slot selector.
    FaceFloatProp::new(2, slot, name, category, functionality, permissions)
}

// 0x0659490 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE2EfED0Ev
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)2,float>::~SurfacePropDescriptor()
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)2,float>::~SurfacePropDescriptor()")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE2EfED0Ev")]
pub fn stub_0659490() {
    // IDA 0x0659490: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06594bc — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)2,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv")]
pub fn stub_06594bc() -> bool {
    // IDA 0x6594bc (`SurfaceGetSet<2, float>::isReadOnly`): `MOVS
    // R0, #0; BX LR` — always readable.
    false
}

// 0x06594c0 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)2,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv")]
pub fn stub_06594c0() -> bool {
    // IDA 0x6594c0 (`SurfaceGetSet<2, float>::isWriteOnly`): `MOVS
    // R0, #0; BX LR` — always writable.
    false
}

// 0x06594c4 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)2,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_06594c4(part: &PartSurfaceData, slot: FaceFloatSlot) -> f32 {
    // IDA 0x6594c4 (`SurfaceGetSet<2, float>::getValue`): the
    // member-pointer resolve (0x6594c4-0x6594de) tail-calling the
    // getter with the face baked in (`v3)(v5, 2)`, 0x6594e0). The
    // member selects the slot; the pointer folds away.
    part.faces[2].float_slot(slot)
}

// 0x06594e4 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)2,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf")]
pub fn stub_06594e4(part: &mut PartSurfaceData, slot: FaceFloatSlot, value: f32) {
    // IDA 0x6594e4 (`SurfaceGetSet<2, float>::setValue`): the
    // member-pointer resolve (0x6594e4-0x659500) tail-calling the
    // setter with `(instance, 2, value)` (0x659502). The member
    // selects the slot; the pointer folds away.
    part.faces[2].set_float_slot(slot, value);
}

// 0x0659508 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
pub fn stub_0659508(name: &str, category: &str, functionality: u32) -> FaceInputProp {
    // IDA 0x659508 (`SurfaceEnumPropDescriptor<2, InputType>` ctor):
    // the `PartInstance` `classDescriptor` call (0x65951a), the
    // `EnumDesc<InputType>` singleton once-init (0x659536-0x65953a),
    // the `PropertyDescriptor` base init (0x65956e-0x659582) and
    // `operator new(0x14)` impl holding the getter/setter pair
    // (0x659586-0x6595a4). The face is baked in; the member pair
    // folds into the face slot.
    FaceInputProp::new(2, name, category, functionality)
}

// 0x06595b4 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED0Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED0Ev")]
pub fn stub_06595b4() {
    // IDA 0x06595b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06595e0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10isReadOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10isReadOnlyEv")]
pub fn stub_06595e0() -> bool {
    // IDA 0x6595e0 (`SurfaceEnumPropDescriptor<2, InputType>::
    // isReadOnly`): delegates to the inner `GetSet` at +44
    // (0x6595ec, host: stub_06594bc) — always readable.
    false
}

// 0x06595f0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11isWriteOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")]
pub fn stub_06595f0() -> bool {
    // IDA 0x6595f0 (`SurfaceEnumPropDescriptor<2, InputType>::
    // isWriteOnly`): delegates to the inner `GetSet` at +44
    // (0x6595fc, host: stub_06594c0) — always writable.
    false
}

// 0x0659600 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")]
pub fn stub_0659600(first: &PartSurfaceData, second: &PartSurfaceData) -> bool {
    // IDA 0x659600 (`SurfaceEnumPropDescriptor<2, InputType>::
    // equalValues`): reads the inner value for both instances via
    // the +44 `GetSet` (0x659610-0x659626) and compares. Host:
    // compare the live face slots.
    first.faces[2].surface_input == second.faces[2].surface_input
}

// 0x0659628 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")]
pub fn stub_0659628(part: &PartSurfaceData) -> SurfaceVariant {
    // IDA 0x659628 (`SurfaceEnumPropDescriptor<2, InputType>::
    // getVariant`): reads the inner value (0x659638), tags it with
    // the `InputType` singleton (0x65963e) and placement-moves it
    // into the variant (0x65964c). Host: the tagged value.
    SurfaceVariant::SurfaceInput(part.faces[2].surface_input)
}

// 0x0659650 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")]
pub fn stub_0659650() -> ! {
    todo!("0x0659650 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x06597a8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")]
pub fn stub_06597a8() -> ! {
    todo!("0x06597a8 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x06597cc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14hasStringValueEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::hasStringValue(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14hasStringValueEv")]
pub fn stub_06597cc() -> ! {
    todo!("0x06597cc RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::hasStringValue(void)const")
}

// 0x06597d0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_06597d0() -> ! {
    todo!("0x06597d0 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0659820 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
pub fn stub_0659820() -> ! {
    todo!("0x0659820 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x0659884 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
pub fn stub_0659884() -> ! {
    todo!("0x0659884 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x06598a4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_06598a4() -> ! {
    todo!("0x06598a4 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x0659afc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_0659afc() -> ! {
    todo!("0x0659afc RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0659b44 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
pub fn stub_0659b44() -> ! {
    todo!("0x0659b44 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x0659ba0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_0659ba0() -> ! {
    todo!("0x0659ba0 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0659ba8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
pub fn stub_0659ba8() -> ! {
    todo!("0x0659ba8 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x0659c1c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_0659c1c() -> ! {
    todo!("0x0659c1c RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x0659c6c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
pub fn stub_0659c6c() -> ! {
    todo!("0x0659c6c RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x0659cc8 — __ZNK3RBX10Reflection8EnumDescINS_16LegacyController9InputTypeEE14convertToIndexES3_
// demangled: RBX::Reflection::EnumDesc<RBX::LegacyController::InputType>::convertToIndex(RBX::LegacyController::InputType)const
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::LegacyController::InputType>::convertToIndex(RBX::LegacyController::InputType)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16LegacyController9InputTypeEE14convertToIndexES3_")]
pub fn stub_0659cc8() -> ! {
    todo!("0x0659cc8 RBX::Reflection::EnumDesc<RBX::LegacyController::InputType>::convertToIndex(RBX::LegacyController::InputType)const")
}

// 0x0659d38 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv")]
pub fn stub_0659d38() -> ! {
    todo!("0x0659d38 RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const")
}

// 0x0659d3c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv")]
pub fn stub_0659d3c() -> ! {
    todo!("0x0659d3c RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const")
}

// 0x0659d40 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_0659d40() -> ! {
    todo!("0x0659d40 RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0659d60 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_")]
pub fn stub_0659d60() -> ! {
    todo!("0x0659d60 RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const")
}

// 0x0659d84 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
pub fn stub_0659d84() -> ! {
    todo!("0x0659d84 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0x0659e30 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEED0Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEED0Ev")]
pub fn stub_0659e30() {
    // IDA 0x0659e30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0659e5c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10isReadOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10isReadOnlyEv")]
pub fn stub_0659e5c() -> ! {
    todo!("0x0659e5c RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::isReadOnly(void)const")
}

// 0x0659e6c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11isWriteOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11isWriteOnlyEv")]
pub fn stub_0659e6c() -> ! {
    todo!("0x0659e6c RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::isWriteOnly(void)const")
}

// 0x0659e7c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_")]
pub fn stub_0659e7c() -> ! {
    todo!("0x0659e7c RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x0659ea4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE")]
pub fn stub_0659ea4() -> ! {
    todo!("0x0659ea4 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x0659ecc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE")]
pub fn stub_0659ecc() -> ! {
    todo!("0x0659ecc RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x065a024 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_")]
pub fn stub_065a024() -> ! {
    todo!("0x065a024 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x065a048 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14hasStringValueEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::hasStringValue(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14hasStringValueEv")]
pub fn stub_065a048() -> ! {
    todo!("0x065a048 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::hasStringValue(void)const")
}

// 0x065a04c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065a04c() -> ! {
    todo!("0x065a04c RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x065a09c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
pub fn stub_065a09c() -> ! {
    todo!("0x065a09c RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x065a100 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
pub fn stub_065a100() -> ! {
    todo!("0x065a100 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x065a120 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_065a120() -> ! {
    todo!("0x065a120 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x065a378 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065a378() -> ! {
    todo!("0x065a378 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x065a3c0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
pub fn stub_065a3c0() -> ! {
    todo!("0x065a3c0 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x065a41c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065a41c() -> ! {
    todo!("0x065a41c RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x065a424 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
pub fn stub_065a424() -> ! {
    todo!("0x065a424 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x065a498 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065a498() -> ! {
    todo!("0x065a498 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x065a4e8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
pub fn stub_065a4e8() -> ! {
    todo!("0x065a4e8 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x065a544 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToIndexES2_
// demangled: RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToIndex(RBX::SurfaceType)const
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToIndex(RBX::SurfaceType)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToIndexES2_")]
pub fn stub_065a544() -> ! {
    todo!("0x065a544 RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToIndex(RBX::SurfaceType)const")
}

// 0x065a5b4 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE10isReadOnlyEv")]
pub fn stub_065a5b4() -> ! {
    todo!("0x065a5b4 RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isReadOnly(void)const")
}

// 0x065a5b8 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE11isWriteOnlyEv")]
pub fn stub_065a5b8() -> ! {
    todo!("0x065a5b8 RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::isWriteOnly(void)const")
}

// 0x065a5bc — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065a5bc() -> ! {
    todo!("0x065a5bc RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x065a5dc — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_11SurfaceTypeEMNS_12PartInstanceEKFS2_S1_EMS3_FvS1_S2_EE8setValueEPNS_10Reflection13DescribedBaseERKS2_")]
pub fn stub_065a5dc() -> ! {
    todo!("0x065a5dc RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::SurfaceType,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>::setValue(RBX::Reflection::DescribedBase *,RBX::SurfaceType const&)const")
}

// 0x065a600 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE5EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)5,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)5,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE5EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE")]
pub fn stub_065a600() -> ! {
    todo!("0x065a600 RBX::SurfacePropDescriptor<(RBX::NormalId)5,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")
}

// 0x065a714 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE5EfED0Ev
// demangled: RBX::SurfacePropDescriptor<(RBX::NormalId)5,float>::~SurfacePropDescriptor()
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)5,float>::~SurfacePropDescriptor()")]
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE5EfED0Ev")]
pub fn stub_065a714() {
    // IDA 0x065a714: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065a740 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)5,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)5,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE10isReadOnlyEv")]
pub fn stub_065a740() -> ! {
    todo!("0x065a740 RBX::SurfaceGetSet<(RBX::NormalId)5,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isReadOnly(void)const")
}

// 0x065a744 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)5,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)5,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE11isWriteOnlyEv")]
pub fn stub_065a744() -> ! {
    todo!("0x065a744 RBX::SurfaceGetSet<(RBX::NormalId)5,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::isWriteOnly(void)const")
}

// 0x065a748 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)5,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)5,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8getValueEPKNS_10Reflection13DescribedBaseE")]
pub fn stub_065a748() -> ! {
    todo!("0x065a748 RBX::SurfaceGetSet<(RBX::NormalId)5,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x065a768 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf
// demangled: RBX::SurfaceGetSet<(RBX::NormalId)5,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)5,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE5EfMNS_12PartInstanceEKFfS1_EMS2_FvS1_fEE8setValueEPNS_10Reflection13DescribedBaseERKf")]
pub fn stub_065a768() -> ! {
    todo!("0x065a768 RBX::SurfaceGetSet<(RBX::NormalId)5,float,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x065a78c — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
pub fn stub_065a78c() -> ! {
    todo!("0x065a78c RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0x065a838 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED0Ev
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED0Ev")]
pub fn stub_065a838() {
    // IDA 0x065a838: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x065a864 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10isReadOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::isReadOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10isReadOnlyEv")]
pub fn stub_065a864() -> ! {
    todo!("0x065a864 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::isReadOnly(void)const")
}

// 0x065a874 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11isWriteOnlyEv
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::isWriteOnly(void)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")]
pub fn stub_065a874() -> ! {
    todo!("0x065a874 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::isWriteOnly(void)const")
}

// 0x065a884 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")]
pub fn stub_065a884() -> ! {
    todo!("0x065a884 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x065a8ac — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE
// demangled: RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")]
pub fn stub_065a8ac() -> ! {
    todo!("0x065a8ac RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}
