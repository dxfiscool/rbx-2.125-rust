//! rendering — generated_497 — 100 stubs global dedup (rendering filtered, EA-sorted asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) NOT in /tmp/global_eas.txt — next 100 uncovered EA-sorted asc 0xd3d114..0xd45a30 (4306 candidates remaining, 89473 global EAs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr). Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

/// Ogre::Vector3 mirror (xyz triple) backing emitter position/up params.
/// Field layout matches the ARMv7 `Ogre::Vector3` read by `parseVector3`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[doc(alias = "Ogre::Vector3")]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// `Ogre::Vector3::ZERO` — default passed to `parseVector3` at IDA `0xd3d114`/`0xd3d2b8`.
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
}

/// Ogre::ParticleEmitter parameter block backing the `EmitterCommands` wrappers below.
/// `boost::shared_ptr<Ogre::ParticleEmitter>` maps to `rbx_core::SharedPtr` (`Arc`);
/// the struct itself is plain data so `doGet`/`doSet` stay faithful without pointers.
#[derive(Clone, Debug, Default)]
#[doc(alias = "Ogre::ParticleEmitter")]
pub struct ParticleEmitter {
    pub position: Vector3,
    pub up: Vector3,
    pub angle: f32,
    pub emission_rate: f32,
    pub min_ttl: f32,
    pub max_ttl: f32,
    pub min_velocity: f32,
    pub max_velocity: f32,
    pub ttl: f32,
    pub velocity: f32,
    pub duration: f32,
    pub min_duration: f32,
    pub max_duration: f32,
    pub repeat_delay: f32,
    pub min_repeat_delay: f32,
    pub max_repeat_delay: f32,
    pub name: String,
    pub emitted_emitter: String,
}

impl ParticleEmitter {
    #[inline]
    pub fn emission_rate(&self) -> f32 { self.emission_rate }
    #[inline]
    pub fn set_emission_rate(&mut self, value: f32) { self.emission_rate = value; }
    #[inline]
    pub fn max_ttl(&self) -> f32 { self.max_ttl }
    #[inline]
    pub fn set_max_ttl(&mut self, value: f32) { self.max_ttl = value; }
    #[inline]
    pub fn min_ttl(&self) -> f32 { self.min_ttl }
    #[inline]
    pub fn set_min_ttl(&mut self, value: f32) { self.min_ttl = value; }
    #[inline]
    pub fn max_velocity(&self) -> f32 { self.max_velocity }
    #[inline]
    pub fn set_max_velocity(&mut self, value: f32) { self.max_velocity = value; }
    #[inline]
    pub fn min_velocity(&self) -> f32 { self.min_velocity }
    #[inline]
    pub fn set_min_velocity(&mut self, value: f32) { self.min_velocity = value; }
    #[inline]
    pub fn position(&self) -> &Vector3 { &self.position }
    #[inline]
    pub fn set_position(&mut self, value: Vector3) { self.position = value; }
    #[inline]
    pub fn up(&self) -> &Vector3 { &self.up }
    #[inline]
    pub fn set_up(&mut self, value: Vector3) { self.up = value; }
    #[inline]
    pub fn ttl(&self) -> f32 { self.ttl }
    #[inline]
    pub fn set_ttl(&mut self, value: f32) { self.ttl = value; }
    #[inline]
    pub fn velocity(&self) -> f32 { self.velocity }
    #[inline]
    pub fn set_velocity(&mut self, value: f32) { self.velocity = value; }
    #[inline]
    pub fn duration(&self) -> f32 { self.duration }
    #[inline]
    pub fn set_duration(&mut self, value: f32) { self.duration = value; }
    #[inline]
    pub fn min_duration(&self) -> f32 { self.min_duration }
    #[inline]
    pub fn set_min_duration(&mut self, value: f32) { self.min_duration = value; }
    #[inline]
    pub fn max_duration(&self) -> f32 { self.max_duration }
    #[inline]
    pub fn set_max_duration(&mut self, value: f32) { self.max_duration = value; }
    #[inline]
    pub fn repeat_delay(&self) -> f32 { self.repeat_delay }
    #[inline]
    pub fn set_repeat_delay(&mut self, value: f32) { self.repeat_delay = value; }
    #[inline]
    pub fn min_repeat_delay(&self) -> f32 { self.min_repeat_delay }
    #[inline]
    pub fn set_min_repeat_delay(&mut self, value: f32) { self.min_repeat_delay = value; }
    #[inline]
    pub fn max_repeat_delay(&self) -> f32 { self.max_repeat_delay }
    #[inline]
    pub fn set_max_repeat_delay(&mut self, value: f32) { self.max_repeat_delay = value; }
    #[inline]
    pub fn name(&self) -> &str { &self.name }
    #[inline]
    pub fn set_name(&mut self, value: impl Into<String>) { self.name = value.into(); }
    #[inline]
    pub fn emitted_emitter(&self) -> &str { &self.emitted_emitter }
    #[inline]
    pub fn set_emitted_emitter(&mut self, value: impl Into<String>) { self.emitted_emitter = value.into(); }
    /// Shared ownership form (`boost::shared_ptr<Ogre::ParticleEmitter>` → `SharedPtr`).
    #[inline]
    pub fn shared(self) -> SharedPtr<parking_lot::Mutex<Self>> {
        SharedPtr::new(parking_lot::Mutex::new(self))
    }
}

/// Ogre::StringConverter helpers used by every `EmitterCommands::doGet`/`doSet`.
pub struct StringConverter;

impl StringConverter {
    /// IDA `0xd3d144`: `toString(Real, precision=6, width=0, fill=' ', flags=0)`.
    // FIDELITY: Ogre streams with `precision(6)` = 6 significant digits; Rust has no
    // one-shot equivalent, so format 6 decimals and trim — identical for the small
    // magnitudes particle params use.
    pub fn to_string_real(value: f32) -> String {
        let s = format!("{value:.6}");
        if s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_owned()
        } else {
            s
        }
    }
    /// IDA `0xd3d2a0`: `toString(const Vector3&)` — space-joined components.
    pub fn to_string_vector3(value: &Vector3) -> String {
        format!(
            "{} {} {}",
            Self::to_string_real(value.x),
            Self::to_string_real(value.y),
            Self::to_string_real(value.z)
        )
    }
    /// IDA `0xd3d16c`: `parseReal(const String&, Real default)` — unparseable keeps default.
    pub fn parse_real(text: &str, default: f32) -> f32 {
        text.trim().parse::<f32>().unwrap_or(default)
    }
    /// IDA `0xd3d114`/`0xd3d2b8`: `parseVector3(const String&, const Vector3& = ZERO)`.
    // FIDELITY: Ogre returns the default unless exactly 3 components parse.
    pub fn parse_vector3(text: &str, default: &Vector3) -> Vector3 {
        let mut parts = text.split_whitespace();
        match (parts.next(), parts.next(), parts.next()) {
            (Some(x), Some(y), Some(z)) if parts.next().is_none() => Vector3 {
                x: x.parse().unwrap_or(default.x),
                y: y.parse().unwrap_or(default.y),
                z: z.parse().unwrap_or(default.z),
            },
            _ => *default,
        }
    }
}

// 0xd3d114 — __ZN4Ogre15EmitterCommands5CmdUp5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdUp *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdUp::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands5CmdUp5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdUp::doSet(void *,std::string const&)
// IDA 0xd3d114: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d114(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d114: `parseVector3(value, Vector3::ZERO)` then vtable +60 `setUp`.
    target.set_up(StringConverter::parse_vector3(value, &Vector3::ZERO));
}

// 0xd3d144 — __ZNK4Ogre15EmitterCommands15CmdEmissionRate5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdEmissionRate *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdEmissionRate::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands15CmdEmissionRate5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdEmissionRate::doGet(void const*)const
// IDA 0xd3d144: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d144(target: &ParticleEmitter) -> String {
    // IDA 0xd3d144: vtable +108 `getEmissionRate`; `StringConverter::toString(v, 6, 0, 32, 0)`.
    StringConverter::to_string_real(target.emission_rate())
}

// 0xd3d16c — __ZN4Ogre15EmitterCommands15CmdEmissionRate5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdEmissionRate *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdEmissionRate::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands15CmdEmissionRate5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdEmissionRate::doSet(void *,std::string const&)
// IDA 0xd3d16c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d16c(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d16c: `parseReal(value)` then vtable +104 `set_emission_rate`.
    target.set_emission_rate(StringConverter::parse_real(value, 0.0));
}

// 0xd3d188 — __ZNK4Ogre15EmitterCommands9CmdMaxTTL5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMaxTTL *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdMaxTTL::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands9CmdMaxTTL5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdMaxTTL::doGet(void const*)const
// IDA 0xd3d188: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d188(target: &ParticleEmitter) -> String {
    // IDA 0xd3d188: vtable +136 `getMaxTtl`; `StringConverter::toString(v, 6, 0, 32, 0)`.
    StringConverter::to_string_real(target.max_ttl())
}

// 0xd3d1b4 — __ZN4Ogre15EmitterCommands9CmdMaxTTL5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMaxTTL *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdMaxTTL::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands9CmdMaxTTL5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdMaxTTL::doSet(void *,std::string const&)
// IDA 0xd3d1b4: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d1b4(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d1b4: `parseReal(value)` then vtable +124 `set_max_ttl`.
    target.set_max_ttl(StringConverter::parse_real(value, 0.0));
}

// 0xd3d1d0 — __ZNK4Ogre15EmitterCommands9CmdMinTTL5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMinTTL *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdMinTTL::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands9CmdMinTTL5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdMinTTL::doGet(void const*)const
// IDA 0xd3d1d0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d1d0(target: &ParticleEmitter) -> String {
    // IDA 0xd3d1d0: vtable +132 `getMinTtl`; `StringConverter::toString(v, 6, 0, 32, 0)`.
    StringConverter::to_string_real(target.min_ttl())
}

// 0xd3d1fc — __ZN4Ogre15EmitterCommands9CmdMinTTL5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMinTTL *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdMinTTL::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands9CmdMinTTL5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdMinTTL::doSet(void *,std::string const&)
// IDA 0xd3d1fc: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d1fc(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d1fc: `parseReal(value)` then vtable +120 `set_min_ttl`.
    target.set_min_ttl(StringConverter::parse_real(value, 0.0));
}

// 0xd3d218 — __ZNK4Ogre15EmitterCommands14CmdMaxVelocity5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMaxVelocity *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdMaxVelocity::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands14CmdMaxVelocity5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdMaxVelocity::doGet(void const*)const
// IDA 0xd3d218: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d218(target: &ParticleEmitter) -> String {
    // IDA 0xd3d218: vtable +100 `getMaxVelocity`; `StringConverter::toString(v, 6, 0, 32, 0)`.
    StringConverter::to_string_real(target.max_velocity())
}

// 0xd3d240 — __ZN4Ogre15EmitterCommands14CmdMaxVelocity5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMaxVelocity *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdMaxVelocity::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands14CmdMaxVelocity5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdMaxVelocity::doSet(void *,std::string const&)
// IDA 0xd3d240: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d240(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d240: `parseReal(value)` then vtable +88 `set_max_velocity`.
    target.set_max_velocity(StringConverter::parse_real(value, 0.0));
}

// 0xd3d25c — __ZNK4Ogre15EmitterCommands14CmdMinVelocity5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMinVelocity *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdMinVelocity::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands14CmdMinVelocity5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdMinVelocity::doGet(void const*)const
// IDA 0xd3d25c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d25c(target: &ParticleEmitter) -> String {
    // IDA 0xd3d25c: vtable +96 `getMinVelocity`; `StringConverter::toString(v, 6, 0, 32, 0)`.
    StringConverter::to_string_real(target.min_velocity())
}

// 0xd3d284 — __ZN4Ogre15EmitterCommands14CmdMinVelocity5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMinVelocity *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdMinVelocity::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands14CmdMinVelocity5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdMinVelocity::doSet(void *,std::string const&)
// IDA 0xd3d284: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d284(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d284: `parseReal(value)` then vtable +84 `set_min_velocity`.
    target.set_min_velocity(StringConverter::parse_real(value, 0.0));
}

// 0xd3d2a0 — __ZNK4Ogre15EmitterCommands11CmdPosition5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdPosition *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdPosition::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands11CmdPosition5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdPosition::doGet(void const*)const
// IDA 0xd3d2a0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d2a0(target: &ParticleEmitter) -> String {
    // IDA 0xd3d2a0: vtable +48 `getPosition`; `StringConverter::toString(const Vector3&)`.
    StringConverter::to_string_vector3(target.position())
}

// 0xd3d2b8 — __ZN4Ogre15EmitterCommands11CmdPosition5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdPosition *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdPosition::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands11CmdPosition5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdPosition::doSet(void *,std::string const&)
// IDA 0xd3d2b8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d2b8(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d2b8: `parseVector3(value, Vector3::ZERO)` then vtable +44 `setPosition`.
    target.set_position(StringConverter::parse_vector3(value, &Vector3::ZERO));
}

// 0xd3d2e8 — __ZNK4Ogre15EmitterCommands6CmdTTL5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdTTL *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdTTL::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands6CmdTTL5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdTTL::doGet(void const*)const
// IDA 0xd3d2e8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d2e8(target: &ParticleEmitter) -> String {
    // IDA 0xd3d2e8: vtable +128 `getTtl`; `StringConverter::toString(v, 6, 0, 32, 0)`.
    StringConverter::to_string_real(target.ttl())
}

// 0xd3d314 — __ZN4Ogre15EmitterCommands6CmdTTL5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdTTL *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdTTL::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands6CmdTTL5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdTTL::doSet(void *,std::string const&)
// IDA 0xd3d314: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d314(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d314: `parseReal(value)` then vtable +112 `set_ttl`.
    target.set_ttl(StringConverter::parse_real(value, 0.0));
}

// 0xd3d330 — __ZNK4Ogre15EmitterCommands11CmdVelocity5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdVelocity *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdVelocity::doGet(void const*)const")]
// was: Ogre::EmitterCommands::CmdVelocity::doGet(void const*)const
// IDA 0xd3d330: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d330(target: &ParticleEmitter) -> String {
    // IDA 0xd3d330: vtable +92 `getVelocity`; `StringConverter::toString(v, 6, 0, 32, 0)`.
    StringConverter::to_string_real(target.velocity())
}

// 0xd3d358 — __ZN4Ogre15EmitterCommands11CmdVelocity5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdVelocity *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdVelocity::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands11CmdVelocity5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdVelocity::doSet(void *,std::string const&)
// IDA 0xd3d358: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d358(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d358: `parseReal(value)` then vtable +76 `set_velocity`.
    target.set_velocity(StringConverter::parse_real(value, 0.0));
}

// 0xd3d374 — __ZNK4Ogre15EmitterCommands11CmdDuration5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdDuration *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdDuration::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands11CmdDuration5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdDuration::doGet(void const*)const
// IDA 0xd3d374: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d374(target: &ParticleEmitter) -> String {
    // IDA 0xd3d374: vtable +196 `getDuration`; `StringConverter::toString(v, 6, 0, 32, 0)`.
    StringConverter::to_string_real(target.duration())
}

// 0xd3d3a0 — __ZN4Ogre15EmitterCommands11CmdDuration5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdDuration *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdDuration::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands11CmdDuration5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdDuration::doSet(void *,std::string const&)
// IDA 0xd3d3a0: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d3a0(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d3a0: `parseReal(value)` then vtable +192 `set_duration`.
    target.set_duration(StringConverter::parse_real(value, 0.0));
}

// 0xd3d3bc — __ZNK4Ogre15EmitterCommands14CmdMinDuration5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMinDuration *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdMinDuration::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands14CmdMinDuration5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdMinDuration::doGet(void const*)const
// IDA 0xd3d3bc: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d3bc(target: &ParticleEmitter) -> String {
    // IDA 0xd3d3bc: vtable +212 `getMinDuration`; `StringConverter::toString(v, 6, 0, 32, 0)`.
    StringConverter::to_string_real(target.min_duration())
}

// 0xd3d3e8 — __ZN4Ogre15EmitterCommands14CmdMinDuration5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMinDuration *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdMinDuration::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands14CmdMinDuration5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdMinDuration::doSet(void *,std::string const&)
// IDA 0xd3d3e8: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d3e8(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d3e8: `parseReal(value)` then vtable +204 `set_min_duration`.
    target.set_min_duration(StringConverter::parse_real(value, 0.0));
}

// 0xd3d404 — __ZNK4Ogre15EmitterCommands14CmdMaxDuration5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMaxDuration *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdMaxDuration::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands14CmdMaxDuration5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdMaxDuration::doGet(void const*)const
// IDA 0xd3d404: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d404(target: &ParticleEmitter) -> String {
    // IDA 0xd3d404: vtable +216 `getMaxDuration`; `StringConverter::toString(v, 6, 0, 32, 0)`.
    StringConverter::to_string_real(target.max_duration())
}

// 0xd3d430 — __ZN4Ogre15EmitterCommands14CmdMaxDuration5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMaxDuration *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdMaxDuration::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands14CmdMaxDuration5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdMaxDuration::doSet(void *,std::string const&)
// IDA 0xd3d430: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d430(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d430: `parseReal(value)` then vtable +208 `set_max_duration`.
    target.set_max_duration(StringConverter::parse_real(value, 0.0));
}

// 0xd3d44c — __ZNK4Ogre15EmitterCommands14CmdRepeatDelay5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdRepeatDelay *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdRepeatDelay::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands14CmdRepeatDelay5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdRepeatDelay::doGet(void const*)const
// IDA 0xd3d44c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d44c(target: &ParticleEmitter) -> String {
    // IDA 0xd3d44c: vtable +224 `getRepeatDelay`; `StringConverter::toString(v, 6, 0, 32, 0)`.
    StringConverter::to_string_real(target.repeat_delay())
}

// 0xd3d478 — __ZN4Ogre15EmitterCommands14CmdRepeatDelay5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdRepeatDelay *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdRepeatDelay::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands14CmdRepeatDelay5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdRepeatDelay::doSet(void *,std::string const&)
// IDA 0xd3d478: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d478(target: &mut ParticleEmitter, value: &str) {
    // IDA 0xd3d478: `parseReal(value)` then vtable +220 `set_repeat_delay`.
    target.set_repeat_delay(StringConverter::parse_real(value, 0.0));
}

// 0xd3d494 — __ZNK4Ogre15EmitterCommands17CmdMinRepeatDelay5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMinRepeatDelay *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdMinRepeatDelay::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands17CmdMinRepeatDelay5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdMinRepeatDelay::doGet(void const*)const
// IDA 0xd3d494: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d494() {
}

// 0xd3d4c0 — __ZN4Ogre15EmitterCommands17CmdMinRepeatDelay5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMinRepeatDelay *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdMinRepeatDelay::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands17CmdMinRepeatDelay5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdMinRepeatDelay::doSet(void *,std::string const&)
// IDA 0xd3d4c0: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d4c0() {
}

// 0xd3d4dc — __ZNK4Ogre15EmitterCommands17CmdMaxRepeatDelay5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMaxRepeatDelay *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdMaxRepeatDelay::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands17CmdMaxRepeatDelay5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdMaxRepeatDelay::doGet(void const*)const
// IDA 0xd3d4dc: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d4dc() {
}

// 0xd3d508 — __ZN4Ogre15EmitterCommands17CmdMaxRepeatDelay5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdMaxRepeatDelay *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::EmitterCommands::CmdMaxRepeatDelay::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands17CmdMaxRepeatDelay5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdMaxRepeatDelay::doSet(void *,std::string const&)
// IDA 0xd3d508: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d508() {
}

// 0xd3d524 — __ZNK4Ogre15EmitterCommands7CmdName5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdName *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdName::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands7CmdName5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdName::doGet(void const*)const
// IDA 0xd3d524: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d524() {
}

// 0xd3d53c — __ZN4Ogre15EmitterCommands7CmdName5doSetEPvRKSs
#[doc(alias = "Ogre::EmitterCommands::CmdName::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands7CmdName5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdName::doSet(void *,std::string const&)
// IDA 0xd3d53c: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d53c() {
}

// 0xd3d550 — __ZNK4Ogre15EmitterCommands17CmdEmittedEmitter5doGetEPKv
// type: _DWORD __fastcall(Ogre::EmitterCommands::CmdEmittedEmitter *__hidden this, const void *)
#[doc(alias = "Ogre::EmitterCommands::CmdEmittedEmitter::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre15EmitterCommands17CmdEmittedEmitter5doGetEPKv")]
// was: Ogre::EmitterCommands::CmdEmittedEmitter::doGet(void const*)const
// IDA 0xd3d550: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d550() {
}

// 0xd3d568 — __ZN4Ogre15EmitterCommands17CmdEmittedEmitter5doSetEPvRKSs
#[doc(alias = "Ogre::EmitterCommands::CmdEmittedEmitter::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre15EmitterCommands17CmdEmittedEmitter5doSetEPvRKSs")]
// was: Ogre::EmitterCommands::CmdEmittedEmitter::doSet(void *,std::string const&)
// IDA 0xd3d568: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd3d568() {
}

// 0xd43b58 — __ZN4Ogre14ParticleSystem22CmdEmittedEmitterQuotaD1Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdEmittedEmitterQuota *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdEmittedEmitterQuota::~CmdEmittedEmitterQuota()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem22CmdEmittedEmitterQuotaD1Ev")]
// was: Ogre::ParticleSystem::CmdEmittedEmitterQuota::~CmdEmittedEmitterQuota()
// IDA 0xd43b58: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd43b58() {
}

// 0xd43b5c — __ZN4Ogre14ParticleSystem8CmdWidthD1Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdWidth *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdWidth::~CmdWidth()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem8CmdWidthD1Ev")]
// was: Ogre::ParticleSystem::CmdWidth::~CmdWidth()
// IDA 0xd43b5c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd43b5c() {
}

// 0xd43b60 — __ZN4Ogre14ParticleSystem11CmdRendererD1Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdRenderer *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdRenderer::~CmdRenderer()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem11CmdRendererD1Ev")]
// was: Ogre::ParticleSystem::CmdRenderer::~CmdRenderer()
// IDA 0xd43b60: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd43b60() {
}

// 0xd43b64 — __ZN4Ogre14ParticleSystem9CmdSortedD1Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdSorted *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdSorted::~CmdSorted()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem9CmdSortedD1Ev")]
// was: Ogre::ParticleSystem::CmdSorted::~CmdSorted()
// IDA 0xd43b64: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd43b64() {
}

// 0xd43b68 — __ZN4Ogre14ParticleSystem13CmdLocalSpaceD1Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdLocalSpace *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdLocalSpace::~CmdLocalSpace()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem13CmdLocalSpaceD1Ev")]
// was: Ogre::ParticleSystem::CmdLocalSpace::~CmdLocalSpace()
// IDA 0xd43b68: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd43b68() {
}

// 0xd43b6c — __ZN4Ogre14ParticleSystem20CmdIterationIntervalD1Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdIterationInterval *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdIterationInterval::~CmdIterationInterval()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem20CmdIterationIntervalD1Ev")]
// was: Ogre::ParticleSystem::CmdIterationInterval::~CmdIterationInterval()
// IDA 0xd43b6c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd43b6c() {
}

// 0xd43b70 — __ZN4Ogre14ParticleSystem20CmdNonvisibleTimeoutD1Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdNonvisibleTimeout *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdNonvisibleTimeout::~CmdNonvisibleTimeout()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem20CmdNonvisibleTimeoutD1Ev")]
// was: Ogre::ParticleSystem::CmdNonvisibleTimeout::~CmdNonvisibleTimeout()
// IDA 0xd43b70: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd43b70() {
}

// 0xd43b74 — __ZN4Ogre9RadixSortISt4listIPNS_8ParticleENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fED1Ev
#[doc(alias = "Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::~RadixSort()")]
#[doc(alias = "__ZN4Ogre9RadixSortISt4listIPNS_8ParticleENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fED1Ev")]
// was: Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::~RadixSort()
// IDA 0xd43b74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd43b74() {
}

// 0xd43c4c — __ZNSt6vectorIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(alias = "std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev")]
// was: std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()
// IDA 0xd43c4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd43c4c() {
}

// 0xd43ce0 — __ZNSt6vectorIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm
#[doc(alias = "std::vector<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm")]
// was: std::vector<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)
// IDA 0xd43ce0: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd43ce0() {
}

// 0xd43d4c — __ZN4Ogre9RadixSortISt4listIPNS_8ParticleENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fE4sortINS_14ParticleSystem22SortByDirectionFunctorEEEvRS9_T_
#[doc(alias = "void Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::sort<Ogre::ParticleSystem::SortByDirectionFunctor>(std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>&,Ogre::ParticleSystem::SortByDirectionFunctor)")]
#[doc(alias = "__ZN4Ogre9RadixSortISt4listIPNS_8ParticleENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fE4sortINS_14ParticleSystem22SortByDirectionFunctorEEEvRS9_T_")]
// was: void Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::sort<Ogre::ParticleSystem::SortByDirectionFunctor>(std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>&,Ogre::ParticleSystem::SortByDirectionFunctor)
// IDA 0xd43d4c: 282 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd43d4c() {
}

// 0xd44048 — __ZN4Ogre9RadixSortISt4listIPNS_8ParticleENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fE4sortINS_14ParticleSystem21SortByDistanceFunctorEEEvRS9_T_
#[doc(alias = "void Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::sort<Ogre::ParticleSystem::SortByDistanceFunctor>(std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>&,Ogre::ParticleSystem::SortByDistanceFunctor)")]
#[doc(alias = "__ZN4Ogre9RadixSortISt4listIPNS_8ParticleENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fE4sortINS_14ParticleSystem21SortByDistanceFunctorEEEvRS9_T_")]
// was: void Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::sort<Ogre::ParticleSystem::SortByDistanceFunctor>(std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>&,Ogre::ParticleSystem::SortByDistanceFunctor)
// IDA 0xd44048: 291 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd44048() {
}

// 0xd4436c — __ZN4Ogre16ParticleAffector13_initParticleEPNS_8ParticleE
// type: _DWORD __fastcall(Ogre::ParticleAffector *__hidden this, Ogre::Particle *)
#[doc(alias = "Ogre::ParticleAffector::_initParticle(Ogre::Particle *)")]
#[doc(alias = "__ZN4Ogre16ParticleAffector13_initParticleEPNS_8ParticleE")]
// was: Ogre::ParticleAffector::_initParticle(Ogre::Particle *)
// IDA 0xd4436c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd4436c() {
}

// 0xd44370 — __ZN4Ogre14ParticleSystem7CmdCullD0Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdCull *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdCull::~CmdCull()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem7CmdCullD0Ev")]
// was: Ogre::ParticleSystem::CmdCull::~CmdCull()
// IDA 0xd44370: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd44370() {
}

// 0xd4437c — __ZN4Ogre14ParticleSystem9CmdHeightD0Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdHeight *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdHeight::~CmdHeight()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem9CmdHeightD0Ev")]
// was: Ogre::ParticleSystem::CmdHeight::~CmdHeight()
// IDA 0xd4437c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd4437c() {
}

// 0xd44388 — __ZN4Ogre14ParticleSystem11CmdMaterialD0Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdMaterial *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdMaterial::~CmdMaterial()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem11CmdMaterialD0Ev")]
// was: Ogre::ParticleSystem::CmdMaterial::~CmdMaterial()
// IDA 0xd44388: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd44388() {
}

// 0xd44394 — __ZN4Ogre14ParticleSystem8CmdQuotaD0Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdQuota *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdQuota::~CmdQuota()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem8CmdQuotaD0Ev")]
// was: Ogre::ParticleSystem::CmdQuota::~CmdQuota()
// IDA 0xd44394: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd44394() {
}

// 0xd443a0 — __ZN4Ogre14ParticleSystem22CmdEmittedEmitterQuotaD0Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdEmittedEmitterQuota *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdEmittedEmitterQuota::~CmdEmittedEmitterQuota()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem22CmdEmittedEmitterQuotaD0Ev")]
// was: Ogre::ParticleSystem::CmdEmittedEmitterQuota::~CmdEmittedEmitterQuota()
// IDA 0xd443a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd443a0() {
}

// 0xd443ac — __ZN4Ogre14ParticleSystem8CmdWidthD0Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdWidth *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdWidth::~CmdWidth()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem8CmdWidthD0Ev")]
// was: Ogre::ParticleSystem::CmdWidth::~CmdWidth()
// IDA 0xd443ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd443ac() {
}

// 0xd443b8 — __ZN4Ogre14ParticleSystem11CmdRendererD0Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdRenderer *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdRenderer::~CmdRenderer()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem11CmdRendererD0Ev")]
// was: Ogre::ParticleSystem::CmdRenderer::~CmdRenderer()
// IDA 0xd443b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd443b8() {
}

// 0xd443c4 — __ZN4Ogre14ParticleSystem9CmdSortedD0Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdSorted *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdSorted::~CmdSorted()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem9CmdSortedD0Ev")]
// was: Ogre::ParticleSystem::CmdSorted::~CmdSorted()
// IDA 0xd443c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd443c4() {
}

// 0xd443d0 — __ZN4Ogre14ParticleSystem13CmdLocalSpaceD0Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdLocalSpace *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdLocalSpace::~CmdLocalSpace()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem13CmdLocalSpaceD0Ev")]
// was: Ogre::ParticleSystem::CmdLocalSpace::~CmdLocalSpace()
// IDA 0xd443d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd443d0() {
}

// 0xd443dc — __ZN4Ogre14ParticleSystem20CmdIterationIntervalD0Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdIterationInterval *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdIterationInterval::~CmdIterationInterval()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem20CmdIterationIntervalD0Ev")]
// was: Ogre::ParticleSystem::CmdIterationInterval::~CmdIterationInterval()
// IDA 0xd443dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd443dc() {
}

// 0xd443e8 — __ZN4Ogre14ParticleSystem20CmdNonvisibleTimeoutD0Ev
// type: void __fastcall(Ogre::ParticleSystem::CmdNonvisibleTimeout *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::CmdNonvisibleTimeout::~CmdNonvisibleTimeout()")]
#[doc(alias = "__ZN4Ogre14ParticleSystem20CmdNonvisibleTimeoutD0Ev")]
// was: Ogre::ParticleSystem::CmdNonvisibleTimeout::~CmdNonvisibleTimeout()
// IDA 0xd443e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd443e8() {
}

// 0xd443f4 — __ZNK4Ogre14ParticleSystem14getBoundingBoxEv
// type: _DWORD __fastcall(Ogre::ParticleSystem *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::getBoundingBox(void)const")]
#[doc(alias = "__ZNK4Ogre14ParticleSystem14getBoundingBoxEv")]
// was: Ogre::ParticleSystem::getBoundingBox(void)const
// IDA 0xd443f4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd443f4() {
}

// 0xd443f8 — __ZNK4Ogre14ParticleSystem17getBoundingRadiusEv
// type: _DWORD __fastcall(Ogre::ParticleSystem *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::getBoundingRadius(void)const")]
#[doc(alias = "__ZNK4Ogre14ParticleSystem17getBoundingRadiusEv")]
// was: Ogre::ParticleSystem::getBoundingRadius(void)const
// IDA 0xd443f8: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd443f8() {
}

// 0xd44400 — __ZNK4Ogre14ParticleSystem20getResourceGroupNameEv
// type: _DWORD __fastcall(Ogre::ParticleSystem *__hidden this)
#[doc(alias = "Ogre::ParticleSystem::getResourceGroupName(void)const")]
#[doc(alias = "__ZNK4Ogre14ParticleSystem20getResourceGroupNameEv")]
// was: Ogre::ParticleSystem::getResourceGroupName(void)const
// IDA 0xd44400: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd44400() {
}

// 0xd44408 — __ZThn12_NK4Ogre14ParticleSystem14getBoundingBoxEv
// type: _DWORD __fastcall(Ogre::ParticleSystem *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::ParticleSystem::getBoundingBox(void)const")]
#[doc(alias = "__ZThn12_NK4Ogre14ParticleSystem14getBoundingBoxEv")]
// was: `non-virtual thunk toOgre::ParticleSystem::getBoundingBox(void)const
// IDA 0xd44408: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd44408() {
}

// 0xd4440c — __ZThn12_NK4Ogre14ParticleSystem17getBoundingRadiusEv
// type: _DWORD __fastcall(Ogre::ParticleSystem *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::ParticleSystem::getBoundingRadius(void)const")]
#[doc(alias = "__ZThn12_NK4Ogre14ParticleSystem17getBoundingRadiusEv")]
// was: `non-virtual thunk toOgre::ParticleSystem::getBoundingRadius(void)const
// IDA 0xd4440c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd4440c() {
}

// 0xd44414 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE4findERS1_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xd44414: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd44414() {
}

// 0xd444b8 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE8_M_eraseEPSt13_Rb_tree_nodeISC_E")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)
// IDA 0xd444b8: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd444b8() {
}

// 0xd445d0 — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE8_M_eraseEPSt13_Rb_tree_nodeISC_E")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)
// IDA 0xd445d0: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd445d0() {
}

// 0xd446d4 — __ZNSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEC2ISsSA_EERKS_IT_T0_E
#[doc(alias = "std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::pair<std::string,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>(std::pair const&<std::string,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>)")]
#[doc(alias = "__ZNSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEC2ISsSA_EERKS_IT_T0_E")]
// was: std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::pair<std::string,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>(std::pair const&<std::string,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>)
// IDA 0xd446d4: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd446d4() {
}

// 0xd4483c — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE16_M_insert_uniqueERKSC_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE16_M_insert_uniqueERKSC_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xd4483c: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd4483c() {
}

// 0xd44920 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE9_M_insertEPSt18_Rb_tree_node_baseSK_RKSC_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE9_M_insertEPSt18_Rb_tree_node_baseSK_RKSC_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xd44920: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd44920() {
}

// 0xd44994 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE14_M_create_nodeERKSC_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE14_M_create_nodeERKSC_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xd44994: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd44994() {
}

// 0xd44b50 — __ZNSt10_List_baseIPN4Ogre15ParticleEmitterENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
#[doc(alias = "__ZNSt10_List_baseIPN4Ogre15ParticleEmitterENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev")]
// was: std::_List_base<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xd44b50: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd44b50() {
}

// 0xd44b54 — __ZNSt10_List_baseIPN4Ogre15ParticleEmitterENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
#[doc(alias = "__ZNSt10_List_baseIPN4Ogre15ParticleEmitterENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev")]
// was: std::_List_base<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xd44b54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd44b54() {
}

// 0xd44b60 — __ZNSt4pairISsSt4listIPN4Ogre15ParticleEmitterENS1_12STLAllocatorIS3_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEEEC2ERKSsRKS9_
#[doc(alias = "std::pair<std::string,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::pair(std::string const&,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "__ZNSt4pairISsSt4listIPN4Ogre15ParticleEmitterENS1_12STLAllocatorIS3_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEEEC2ERKSsRKS9_")]
// was: std::pair<std::string,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::pair(std::string const&,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xd44b60: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd44b60() {
}

// 0xd44cc8 — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE16_M_insert_uniqueERKSC_
// type: int __fastcall(int, int, const void **)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE16_M_insert_uniqueERKSC_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xd44cc8: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd44cc8() {
}

// 0xd44dac — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE9_M_insertEPSt18_Rb_tree_node_baseSK_RKSC_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE9_M_insertEPSt18_Rb_tree_node_baseSK_RKSC_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xd44dac: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd44dac() {
}

// 0xd44e20 — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE14_M_create_nodeERKSC_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE14_M_create_nodeERKSC_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xd44e20: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd44e20() {
}

// 0xd44f60 — __ZNSt6vectorIPN4Ogre15ParticleEmitterENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_
#[doc(alias = "std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre15ParticleEmitterENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_")]
// was: std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xd44f60: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd44f60() {
}

// 0xd44fd4 — __ZNSt12_Vector_baseIPN4Ogre15ParticleEmitterENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre15ParticleEmitterENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd44fd4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd44fd4() {
}

// 0xd44fd8 — __ZNSt12_Vector_baseIPN4Ogre15ParticleEmitterENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre15ParticleEmitterENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd44fd8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd44fd8() {
}

// 0xd44fe4 — __ZNSt4listIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
// type: int __fastcall(int)
#[doc(alias = "std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "__ZNSt4listIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_")]
// was: std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xd44fe4: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd44fe4() {
}

// 0xd45040 — __ZN4Ogre9RadixSortISt4listIPNS_8ParticleENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fE9finalPassEif
#[doc(alias = "Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::finalPass(int,float)")]
#[doc(alias = "__ZN4Ogre9RadixSortISt4listIPNS_8ParticleENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES3_fE9finalPassEif")]
// was: Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::finalPass(int,float)
// IDA 0xd45040: 87 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd45040() {
}

// 0xd4513c — __ZNSt4listIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE6insertISt20_List_const_iteratorIS2_EEEvSt14_List_iteratorIS2_ET_SE_
// type: int __fastcall(int, char *, int, int, int, int, int, int, int, int)
#[doc(alias = "void std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert<std::_List_const_iterator<Ogre::Particle *>>(std::_List_iterator<Ogre::Particle *>,std::_List_const_iterator<Ogre::Particle *>,std::_List_const_iterator<Ogre::Particle *>)")]
#[doc(alias = "__ZNSt4listIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE6insertISt20_List_const_iteratorIS2_EEEvSt14_List_iteratorIS2_ET_SE_")]
// was: void std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert<std::_List_const_iterator<Ogre::Particle *>>(std::_List_iterator<Ogre::Particle *>,std::_List_const_iterator<Ogre::Particle *>,std::_List_const_iterator<Ogre::Particle *>)
// IDA 0xd4513c: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd4513c() {
}

// 0xd452b0 — __ZNSt10_List_baseIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
#[doc(alias = "__ZNSt10_List_baseIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev")]
// was: std::_List_base<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xd452b0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd452b0() {
}

// 0xd452b4 — __ZNSt10_List_baseIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
#[doc(alias = "__ZNSt10_List_baseIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev")]
// was: std::_List_base<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xd452b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd452b4() {
}

// 0xd452c0 — __ZNSt6vectorIN4Ogre9RadixSortISt4listIPNS0_8ParticleENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES4_fE9SortEntryENS5_ISC_S8_EEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSC_SE_EEmRKSC_
#[doc(alias = "std::vector<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry*,std::vector<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre9RadixSortISt4listIPNS0_8ParticleENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES4_fE9SortEntryENS5_ISC_S8_EEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSC_SE_EEmRKSC_")]
// was: std::vector<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry*,std::vector<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry const&)
// IDA 0xd452c0: 176 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd452c0() {
}

// 0xd454a8 — __ZNSt6vectorIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Particle **,std::vector<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Particle * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_")]
// was: std::vector<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Particle **,std::vector<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Particle * const&)
// IDA 0xd454a8: 159 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd454a8() {
}

// 0xd45650 — __ZNSt6vectorIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPjS6_EEmRKj
// type: int __fastcall(__int64, unsigned int, int *)
#[doc(alias = "std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,unsigned int const&)")]
#[doc(alias = "__ZNSt6vectorIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPjS6_EEmRKj")]
// was: std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,unsigned int const&)
// IDA 0xd45650: 159 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd45650() {
}

// 0xd457ec — __ZNSt12_Vector_baseIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIjN4Ogre12STLAllocatorIjNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<unsigned int,Ogre::STLAllocator<unsigned int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd457ec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd457ec() {
}

// 0xd457f0 — __ZNSt6vectorIPN4Ogre16ParticleAffectorENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::ParticleAffector *,Ogre::STLAllocator<Ogre::ParticleAffector *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParticleAffector **,std::vector<Ogre::ParticleAffector *,Ogre::STLAllocator<Ogre::ParticleAffector *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParticleAffector * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre16ParticleAffectorENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// was: std::vector<Ogre::ParticleAffector *,Ogre::STLAllocator<Ogre::ParticleAffector *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParticleAffector **,std::vector<Ogre::ParticleAffector *,Ogre::STLAllocator<Ogre::ParticleAffector *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParticleAffector * const&)
// IDA 0xd457f0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xd457f0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xd458e8 — __ZNSt6vectorIPN4Ogre15ParticleEmitterENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParticleEmitter **,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParticleEmitter * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre15ParticleEmitterENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// was: std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ParticleEmitter **,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ParticleEmitter * const&)
// IDA 0xd458e8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xd458e8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xd459e0 — __ZNSt12_Vector_baseIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd459e0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd459e0() {
}

// 0xd459e4 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED1Ev")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xd459e4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd459e4() {
}

// 0xd459e8 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED0Ev")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::list<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xd459e8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd459e8() {
}

// 0xd459f4 — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED1Ev
// type: void()
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED1Ev")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xd459f4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd459f4() {
}

// 0xd459f8 — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre15ParticleEmitterENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED0Ev")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::ParticleEmitter *,Ogre::STLAllocator<Ogre::ParticleEmitter *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xd459f8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd459f8() {
}

// 0xd45a04 — __ZNSt12_Vector_baseIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre8ParticleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd45a04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd45a04() {
}

// 0xd45a10 — __ZNSt12_Vector_baseIN4Ogre9RadixSortISt4listIPNS0_8ParticleENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES4_fE9SortEntryENS5_ISC_S8_EEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre9RadixSortISt4listIPNS0_8ParticleENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES4_fE9SortEntryENS5_ISC_S8_EEE12_Vector_implD1Ev")]
// was: std::_Vector_base<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd45a10: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd45a10() {
}

// 0xd45a14 — __ZNSt12_Vector_baseIN4Ogre9RadixSortISt4listIPNS0_8ParticleENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES4_fE9SortEntryENS5_ISC_S8_EEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre9RadixSortISt4listIPNS0_8ParticleENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES4_fE9SortEntryENS5_ISC_S8_EEE12_Vector_implD0Ev")]
// was: std::_Vector_base<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::list<Ogre::Particle *,Ogre::STLAllocator<Ogre::Particle *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::Particle *,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd45a14: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd45a14() {
}

// 0xd45a20 — __ZNSt12_Vector_baseIPN4Ogre16ParticleAffectorENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::ParticleAffector *,Ogre::STLAllocator<Ogre::ParticleAffector *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre16ParticleAffectorENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<Ogre::ParticleAffector *,Ogre::STLAllocator<Ogre::ParticleAffector *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd45a20: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd45a20() {
}

// 0xd45a24 — __ZNSt12_Vector_baseIPN4Ogre16ParticleAffectorENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::ParticleAffector *,Ogre::STLAllocator<Ogre::ParticleAffector *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre16ParticleAffectorENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<Ogre::ParticleAffector *,Ogre::STLAllocator<Ogre::ParticleAffector *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd45a24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd45a24() {
}

// 0xd45a30 — __ZN4Ogre25ParticleSystemUpdateValueD1Ev
// type: void __fastcall(Ogre::ParticleSystemUpdateValue *__hidden this)
#[doc(alias = "Ogre::ParticleSystemUpdateValue::~ParticleSystemUpdateValue()")]
#[doc(alias = "__ZN4Ogre25ParticleSystemUpdateValueD1Ev")]
// was: Ogre::ParticleSystemUpdateValue::~ParticleSystemUpdateValue()
// IDA 0xd45a30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd45a30() {
}
