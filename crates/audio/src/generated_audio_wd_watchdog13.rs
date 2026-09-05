//! audio generated_audio_wd_watchdog13 — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x063c450 | rbx_core::SharedPtr not boost
//! Range 0x063c460..0x06589e8 | existing 36303 -> 36403 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
use core::sync::atomic::{AtomicU32, Ordering};
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x063c460 — __ZN3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EED1Ev")]
pub fn stub_063c460() {
    // IDA 0x063c460: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063c484 — __ZN3RBX8SparklesD1Ev
// demangled: RBX::Sparkles::~Sparkles()
// type: void __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "RBX::Sparkles::~Sparkles()")]
#[doc(alias = "__ZN3RBX8SparklesD1Ev")]
pub fn stub_063c484() {
    // IDA 0x063c484: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063c488 — __ZN3RBX8SparklesD0Ev
// demangled: RBX::Sparkles::~Sparkles()
// type: void __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "RBX::Sparkles::~Sparkles()")]
#[doc(alias = "__ZN3RBX8SparklesD0Ev")]
pub fn stub_063c488() {
    // IDA 0x063c488: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

/// `RBX::Sparkles` cutover (IDA 0x63c298, canonical ctor in
/// `generated_audio_wd_watchdog12`): the +96 flag byte and the
/// `Color3` at +0x64..+0x6c. File-local twin of `SmokeState`-style
/// hosts; the `Instance`/`Described`/`Effect` bases fold away.
#[derive(Debug, Clone)]
pub struct SparklesState {
    pub flag_60: bool,
    pub color: [f32; 3],
}
/// `RBX::Reflection::PropDescriptor<Sparkles, G3D::Color3>` cutover
/// (IDA 0x63cae0): name/category/attributes/permissions plus the live
/// value. The getter/setter member-pointer pair folds into direct
/// field access.
#[derive(Debug, Clone)]
pub struct SparklesColorProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: [f32; 3],
}
impl SparklesColorProp {
    pub fn new(
        name: &str,
        category: &str,
        initial: [f32; 3],
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value: initial,
        }
    }
}
/// `RBX::Reflection::BoundProp<bool>` cutover for `Sparkles`
/// (IDA 0x63cc8c): name/category plus the live value. The member cell
/// folds into direct field access.
#[derive(Debug, Clone)]
pub struct SparklesBoolProp {
    pub name: String,
    pub category: String,
    pub attributes: u32,
    pub permissions: u32,
    pub value: bool,
}
impl SparklesBoolProp {
    pub fn new(
        name: &str,
        category: &str,
        initial: bool,
        attributes: u32,
        permissions: u32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            category: category.to_owned(),
            attributes,
            permissions,
            value: initial,
        }
    }
}
/// `RBX::SpawnLocation` cutover (IDA 0x63d248): the team color at
/// +0x150 (word 84, init 194 = 0xC2), the touched-signal link at +0x154
/// (word 85, init null — managed by `updateSpawnerTouched`), the
/// `TouchedSignal` log gate at +344, the Neutral flag at +348 (init
/// true), the AllowTeamChangeOnTouch flag at +349 (init false) and the
/// duration at +352 (word 88, init 10). The `Instance`/`Described`/
/// `BasicPartInstance` bases and the `setName("SpawnLocation")` fold
/// away. Matches the classic Neutral=true/AllowTeamChangeOnTouch=false
/// defaults.
#[derive(Debug, Clone)]
pub struct SpawnLocationState {
    pub team_color: u32,
    pub neutral: bool,
    pub allow_team_change_on_touch: bool,
    pub duration: i32,
    pub touched_logging: bool,
}
/// `RBX::SpawnerService` cutover (IDA 0x63db8c): the +92 flag (init 1)
/// plus the spawn-location list at +96 (words 24/25, init self-linked
/// empty). The name registration and the task-scheduler call fold away.
#[derive(Debug, Clone, Default)]
pub struct SpawnerServiceState {
    pub flag_92: bool,
    pub spawns: Vec<SharedPtr<SpawnLocationState>>,
}
/// `RBX::Network::Player` fields read by `GetSpawnLocation`
/// (IDA 0x63df08): Neutral at +104, TeamColor at +100.
#[derive(Debug, Clone, Copy)]
pub struct SpawnPlayerRef {
    pub neutral: bool,
    pub team_color: u32,
}
/// Touched instance offered to `onEvent_spawnerTouched` (IDA 0x63d7b8):
/// whether `Humanoid::modelIsCharacter` and
/// `Players::getPlayerFromCharacter` resolved. The world/tree lookup
/// folds into the flags.
#[derive(Debug, Clone, Copy)]
pub struct TouchedCharacter {
    pub is_character: bool,
    pub player_resolved: bool,
}
/// Team update `onEvent_spawnerTouched` applies to the resolved player
/// (IDA 0x63d83e-0x63d852): `setTeamColor(+0x150)` plus
/// `setNeutral(+348)`.
#[derive(Debug, Clone, Copy)]
pub struct PlayerTeamUpdate {
    pub team_color: u32,
    pub neutral: bool,
}
/// `SpawnPlayer` force-field side effect (IDA 0x63e090-0x63e1c0):
/// `create<ForceField>` + parent to the character +
/// `DebrisService::addItem(ff, (double)duration)` (with the `ds`
/// ReleaseAssert, SpawnLocation.cpp line 212). The character/world
/// handles fold into the effect.
#[derive(Debug, Clone, Copy)]
pub enum SpawnEffect {
    ForceField { duration_secs: f64 },
}
/// Process-wide static-init run count behind the `__GLOBAL__I_a_*`
/// ctors in this file (IDA 0x65740c, 0x6583a0, 0x658744). The
/// category/ios/FLog/descriptor/registrar/pool/guard stores fold into
/// host statics (initialized on use), so only the run is recorded.
static WATCHDOG13_STATIC_INITS: AtomicU32 = AtomicU32::new(0);
/// Records one `__GLOBAL__I_a_*` run in this file.
pub fn watchdog13_static_init() {
    WATCHDOG13_STATIC_INITS.fetch_add(1, Ordering::SeqCst);
}
/// Returns the recorded static-init run count (test hook).
pub fn watchdog13_static_inits() -> u32 {
    WATCHDOG13_STATIC_INITS.load(Ordering::SeqCst)
}

// 0x063c528 — __ZNK3RBX8Sparkles11askAddChildEPKNS_8InstanceE
// demangled: RBX::Sparkles::askAddChild(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Sparkles *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Sparkles::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Sparkles11askAddChildEPKNS_8InstanceE")]
pub fn stub_063c528() -> bool {
    // IDA 0x63c528 (`RBX::Sparkles::askAddChild`): `MOVS R0, #1; BX
    // LR` — any child is accepted (same shape as `Smoke` at 0x6378c4).
    true
}

// 0x063c52c — __ZNK3RBX8Sparkles12askSetParentEPKNS_8InstanceE
// demangled: RBX::Sparkles::askSetParent(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Sparkles *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Sparkles::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Sparkles12askSetParentEPKNS_8InstanceE")]
pub fn stub_063c52c(parent_is_part: Option<bool>) -> bool {
    // IDA 0x63c52c (`RBX::Sparkles::askSetParent`): null parent
    // returns 0 (0x63c530-0x63c53c); else the candidate must `isA`
    // `Part` (0x63c53e-0x63c556), returning 0 on mismatch and 1
    // otherwise (0x63c558-0x63c566). Same shape as `Smoke` at
    // 0x6378c8; null folds into `None`.
    matches!(parent_is_part, Some(true))
}

// 0x063c578 — __ZThn32_N3RBX8SparklesD1Ev
// demangled: non-virtual thunk toRBX::Sparkles::~Sparkles()
// type: void __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Sparkles::~Sparkles()")]
#[doc(alias = "__ZThn32_N3RBX8SparklesD1Ev")]
pub fn stub_063c578() {
    // IDA 0x063c578: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063c580 — __ZThn32_N3RBX8SparklesD0Ev
// demangled: non-virtual thunk toRBX::Sparkles::~Sparkles()
// type: void __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Sparkles::~Sparkles()")]
#[doc(alias = "__ZThn32_N3RBX8SparklesD0Ev")]
pub fn stub_063c580() {
    // IDA 0x063c580: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063c598 — __ZThn36_N3RBX8SparklesD1Ev
// demangled: non-virtual thunk toRBX::Sparkles::~Sparkles()
// type: void __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Sparkles::~Sparkles()")]
#[doc(alias = "__ZThn36_N3RBX8SparklesD1Ev")]
pub fn stub_063c598() {
    // IDA 0x063c598: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063c5a0 — __ZThn36_N3RBX8SparklesD0Ev
// demangled: non-virtual thunk toRBX::Sparkles::~Sparkles()
// type: void __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Sparkles::~Sparkles()")]
#[doc(alias = "__ZThn36_N3RBX8SparklesD0Ev")]
pub fn stub_063c5a0() {
    // IDA 0x063c5a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063c5a8 — __ZThn92_N3RBX8SparklesD1Ev
// demangled: non-virtual thunk toRBX::Sparkles::~Sparkles()
// type: void __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Sparkles::~Sparkles()")]
#[doc(alias = "__ZThn92_N3RBX8SparklesD1Ev")]
pub fn stub_063c5a8() {
    // IDA 0x063c5a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063c5b0 — __ZThn92_N3RBX8SparklesD0Ev
// demangled: non-virtual thunk toRBX::Sparkles::~Sparkles()
// type: void __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Sparkles::~Sparkles()")]
#[doc(alias = "__ZThn92_N3RBX8SparklesD0Ev")]
pub fn stub_063c5b0() {
    // IDA 0x063c5b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063c5b8 — __ZN3RBX8SparklesD2Ev
// demangled: RBX::Sparkles::~Sparkles()
// type: void __fastcall(RBX::Sparkles *__hidden this)
#[doc(alias = "RBX::Sparkles::~Sparkles()")]
#[doc(alias = "__ZN3RBX8SparklesD2Ev")]
pub fn stub_063c5b8() {
    // IDA 0x063c5b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063cae0 — __ZN3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Sparkles::*)(void)const,void (RBX::Sparkles::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Sparkles::*)(void)const,void (RBX::Sparkles::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Sparkles::*)(void)const,void (RBX::Sparkles::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Sparkles::*)(void)const,void (RBX::Sparkles::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_063cae0(
    name: &str,
    category: &str,
    initial: [f32; 3],
    attributes: u32,
    permissions: u32,
) -> SparklesColorProp {
    // IDA 0x63cae0 (`PropDescriptor<Sparkles, Color3>::C2`): same
    // member-triple + `TypedPropertyDescriptor<Color3>::C2` + vtable
    // shape as the `Smoke` twin at 0x63885c (0x63cae0-0x63cc10).
    SparklesColorProp::new(name, category, initial, attributes, permissions)
}

// 0x063cbf4 — __ZN3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EED0Ev")]
pub fn stub_063cbf4() {
    // IDA 0x063cbf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063cc20 — __ZNK3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Sparkles::*)(void)const,void (RBX::Sparkles::*)(G3D::Color3)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Sparkles::*)(void)const,void (RBX::Sparkles::*)(G3D::Color3)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_063cc20() -> bool {
    // IDA 0x63cc20 (`GetSetImpl<Color3 getter, Color3
    // setter>::isReadOnly`): `MOVS R0, #0; BX LR` — always readable.
    false
}

// 0x063cc24 — __ZNK3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Sparkles::*)(void)const,void (RBX::Sparkles::*)(G3D::Color3)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Sparkles::*)(void)const,void (RBX::Sparkles::*)(G3D::Color3)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_063cc24() -> bool {
    // IDA 0x63cc24 (`GetSetImpl<Color3 getter, Color3
    // setter>::isWriteOnly`): `MOVS R0, #0; BX LR` — always writable.
    false
}

// 0x063cc28 — __ZNK3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Sparkles::*)(void)const,void (RBX::Sparkles::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Sparkles::*)(void)const,void (RBX::Sparkles::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_063cc28(state: &SparklesState) -> [f32; 3] {
    // IDA 0x63cc28 (`GetSetImpl::getValue`): same member-pointer
    // resolve as the `Smoke` twin at 0x6389a4 (null described reads at
    // offset 0, else `a2 - 36`; virtual when the low bit is set,
    // 0x63cc28-0x63cc48), tail-calling the getter (0x63cc4a). The
    // member is `getColor` (0x63c450); the pointer folds into the
    // field.
    state.color
}

// 0x063cc50 — __ZNK3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Sparkles::*)(void)const,void (RBX::Sparkles::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sparkles,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Sparkles::*)(void)const,void (RBX::Sparkles::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8SparklesEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_063cc50(state: &mut SparklesState, value: [f32; 3]) -> bool {
    // IDA 0x63cc50 (`GetSetImpl::setValue`): same member-pointer
    // resolve over +12/+16 (0x63cc50-0x63cc70), copying the three
    // input words for the setter call (0x63cc76-0x63cc82). The member
    // is `setColor` (0x63c1a4, which compares, stores and raises);
    // the pointer folds into it.
    if state.color == value {
        return false;
    }
    state.color = value;
    true
}

// 0x063cc8c — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_8SparklesEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sparkles>(char const*,char const*,bool RBX::Sparkles::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sparkles>(char const*,char const*,bool RBX::Sparkles::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_8SparklesEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_063cc8c(
    name: &str,
    category: &str,
    initial: bool,
    attributes: u32,
    permissions: u32,
) -> SparklesBoolProp {
    // IDA 0x63cc8c (`BoundProp<bool>::BoundProp<Sparkles>`): same
    // `TypedPropertyDescriptor<bool>::C2` + vtable + member-cell shape
    // as the `Smoke` twin at 0x638a08 (0x63cc8c-0x63cdd0).
    SparklesBoolProp::new(name, category, initial, attributes, permissions)
}

// 0x063ce1c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE10isReadOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sparkles>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sparkles>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE10isReadOnlyEv")]
pub fn stub_063ce1c() -> bool {
    // IDA 0x63ce1c (`BoundPropGetSet<Sparkles>::isReadOnly`): `MOVS
    // R0, #0; BX LR` — always readable.
    false
}

// 0x063ce20 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE11isWriteOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sparkles>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sparkles>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE11isWriteOnlyEv")]
pub fn stub_063ce20() -> bool {
    // IDA 0x63ce20 (`BoundPropGetSet<Sparkles>::isWriteOnly`): `MOVS
    // R0, #0; BX LR` — always writable.
    false
}

// 0x063ce24 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sparkles>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sparkles>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_063ce24(state: &SparklesState) -> bool {
    // IDA 0x63ce24 (`BoundPropGetSet<Sparkles>::getValue`): loads the
    // member offset at +8, adjusts the described (`R1 - 36` when
    // non-null) and returns the byte there (0x63ce24-0x63ce2c). The
    // member is the +96 flag (set to 1 by `Sparkles::Sparkles`,
    // 0x63c360-0x63c370); the offset folds into the field.
    state.flag_60
}

// 0x063ce30 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sparkles>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sparkles>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_063ce30(state: &mut SparklesState, value: bool) -> bool {
    // IDA 0x63ce30 (`BoundPropGetSet<Sparkles>::setValue`): adjusts
    // the described (0x63ce34-0x63ce3a), returns early on match
    // (0x63ce42-0x63ce4a), else stores (0x63ce4c), runs the member
    // hook when set (0x63ce4e-0x63ce6e) and tail-calls
    // `raisePropertyChanged` (0x63ce72-0x63ce7a). Same shape as the
    // `Smoke` twin at 0x638bac.
    if state.flag_60 == value {
        return false;
    }
    state.flag_60 = value;
    true
}

// 0x063d228 — __ZNK3RBX13SpawnLocation12getTeamColorEv
// demangled: RBX::SpawnLocation::getTeamColor(void)const
// type: _DWORD __fastcall(RBX::SpawnLocation *__hidden this)
#[doc(alias = "RBX::SpawnLocation::getTeamColor(void)const")]
#[doc(alias = "__ZNK3RBX13SpawnLocation12getTeamColorEv")]
pub fn stub_063d228(state: &SpawnLocationState) -> u32 {
    // IDA 0x63d228 (`RBX::SpawnLocation::getTeamColor`): loads word 84
    // at +0x150 (0x63d228-0x63d22c).
    state.team_color
}

// 0x063d230 — __ZN3RBX13SpawnLocation12setTeamColorENS_10BrickColorE
// demangled: RBX::SpawnLocation::setTeamColor(RBX::BrickColor)
#[doc(alias = "RBX::SpawnLocation::setTeamColor(RBX::BrickColor)")]
#[doc(alias = "__ZN3RBX13SpawnLocation12setTeamColorENS_10BrickColorE")]
pub fn stub_063d230(state: &mut SpawnLocationState, value: u32) {
    // IDA 0x63d230 (`RBX::SpawnLocation::setTeamColor`): stores word
    // 84 at +0x150 (0x63d238) and tail-calls `raisePropertyChanged`
    // (0x63d23c-0x63d242). The raise folds away (no listeners in the
    // host); the store is the observable state.
    state.team_color = value;
}

// 0x063d248 — __ZN3RBX13SpawnLocationC1Ev
// demangled: RBX::SpawnLocation::SpawnLocation(void)
// type: _DWORD __fastcall(RBX::SpawnLocation *__hidden this)
#[doc(alias = "RBX::SpawnLocation::SpawnLocation(void)")]
#[doc(alias = "__ZN3RBX13SpawnLocationC1Ev")]
pub fn stub_063d248(touched_logging: bool) -> SpawnLocationState {
    // IDA 0x63d248 (`RBX::SpawnLocation::SpawnLocation`):
    // `DescribedCreatable` base + vtable installs + class registration
    // (0x63d264-0x63d2e8); word 84 (+0x150) = 194 (0xC2, 0x63d2ee-
    // 0x63d2f2); word 85 (+0x154, the touched connection) = 0
    // (0x63d2f6-0x63d2fc); +344 = `FLog::TouchedSignal != 0`
    // (0x63d2fe-0x63d30a); +348 (Neutral) = 1, +349
    // (AllowTeamChangeOnTouch) = 0 (0x63d30c-0x63d312); word 88 (+352,
    // duration) = 10 (0x63d314-0x63d31a); `setName("SpawnLocation")`
    // (0x63d31c-0x63d340). The flag is a host-seam parameter.
    SpawnLocationState {
        team_color: 194,
        neutral: true,
        allow_team_change_on_touch: false,
        duration: 10,
        touched_logging,
    }
}

// 0x063d500 — __ZN3RBX13SpawnLocationD0Ev
// demangled: RBX::SpawnLocation::~SpawnLocation()
// type: void __fastcall(RBX::SpawnLocation *__hidden this)
#[doc(alias = "RBX::SpawnLocation::~SpawnLocation()")]
#[doc(alias = "__ZN3RBX13SpawnLocationD0Ev")]
pub fn stub_063d500() {
    // IDA 0x063d500: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063d5ac — __ZN3RBX13SpawnLocationD1Ev
// demangled: RBX::SpawnLocation::~SpawnLocation()
// type: void __fastcall(RBX::SpawnLocation *__hidden this)
#[doc(alias = "RBX::SpawnLocation::~SpawnLocation()")]
#[doc(alias = "__ZN3RBX13SpawnLocationD1Ev")]
pub fn stub_063d5ac() {
    // IDA 0x063d5ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063d5bc — __ZThn32_N3RBX13SpawnLocationD0Ev
// demangled: non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()
// type: void __fastcall(RBX::SpawnLocation *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()")]
#[doc(alias = "__ZThn32_N3RBX13SpawnLocationD0Ev")]
pub fn stub_063d5bc() {
    // IDA 0x063d5bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063d5c4 — __ZThn36_N3RBX13SpawnLocationD0Ev
// demangled: non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()
// type: void __fastcall(RBX::SpawnLocation *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()")]
#[doc(alias = "__ZThn36_N3RBX13SpawnLocationD0Ev")]
pub fn stub_063d5c4() {
    // IDA 0x063d5c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063d5cc — __ZThn132_N3RBX13SpawnLocationD0Ev
// demangled: non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()
// type: void __fastcall(RBX::SpawnLocation *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()")]
#[doc(alias = "__ZThn132_N3RBX13SpawnLocationD0Ev")]
pub fn stub_063d5cc() {
    // IDA 0x063d5cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063d5d4 — __ZN3RBX13SpawnLocationD2Ev
// demangled: RBX::SpawnLocation::~SpawnLocation()
// type: void __fastcall(RBX::SpawnLocation *__hidden this)
#[doc(alias = "RBX::SpawnLocation::~SpawnLocation()")]
#[doc(alias = "__ZN3RBX13SpawnLocationD2Ev")]
pub fn stub_063d5d4() {
    // IDA 0x063d5d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063d788 — __ZThn32_N3RBX13SpawnLocationD1Ev
// demangled: non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()
// type: void __fastcall(RBX::SpawnLocation *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()")]
#[doc(alias = "__ZThn32_N3RBX13SpawnLocationD1Ev")]
pub fn stub_063d788() {
    // IDA 0x063d788: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063d798 — __ZThn36_N3RBX13SpawnLocationD1Ev
// demangled: non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()
// type: void __fastcall(RBX::SpawnLocation *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()")]
#[doc(alias = "__ZThn36_N3RBX13SpawnLocationD1Ev")]
pub fn stub_063d798() {
    // IDA 0x063d798: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063d7a8 — __ZThn132_N3RBX13SpawnLocationD1Ev
// demangled: non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()
// type: void __fastcall(RBX::SpawnLocation *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()")]
#[doc(alias = "__ZThn132_N3RBX13SpawnLocationD1Ev")]
pub fn stub_063d7a8() {
    // IDA 0x063d7a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063d7b8 — __ZN3RBX13SpawnLocation22onEvent_spawnerTouchedEN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::SpawnLocation::onEvent_spawnerTouched(boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::SpawnLocation::onEvent_spawnerTouched(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX13SpawnLocation22onEvent_spawnerTouchedEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_063d7b8(
    state: &SpawnLocationState,
    touched: Option<TouchedCharacter>,
    backend_processing: bool,
) -> Option<PlayerTeamUpdate> {
    // IDA 0x63d7b8 (`RBX::SpawnLocation::onEvent_spawnerTouched`):
    // `Players::backendProcessing(this, true)` must return 1
    // (0x63d7bc-0x63d7c8); the touched instance must be non-null
    // (0x63d7ca-0x63d7ce); `ReleaseAssert(allowTeamChangeOnTouch)`
    // (SpawnLocation.cpp line 44, 0x63d7d0-0x63d81c — live only via
    // the `updateSpawnerTouched` connection, which requires the flag,
    // so the assert is unconditional here);
    // `Humanoid::modelIsCharacter` must hold (0x63d820-0x63d830) and
    // `Players::getPlayerFromCharacter` must resolve (0x63d832-
    // 0x63d83c); then `Player::setTeamColor(+0x150)` plus
    // `Player::setNeutral(+348)` (0x63d83e-0x63d852). The
    // backend/character/player lookups fold into the flags.
    if !backend_processing {
        return None;
    }
    let touched = touched?;
    assert!(
        state.allow_team_change_on_touch,
        "allowTeamChangeOnTouch file: SpawnLocation.cpp line: 44 (IDA 0x63d7b8)"
    );
    if !touched.is_character || !touched.player_resolved {
        return None;
    }
    Some(PlayerTeamUpdate {
        team_color: state.team_color,
        neutral: state.neutral,
    })
}

// 0x063d858 — __ZN3RBX13SpawnLocation20updateSpawnerTouchedEv
// demangled: RBX::SpawnLocation::updateSpawnerTouched(void)
// type: _DWORD __fastcall(RBX::SpawnLocation *__hidden this)
#[doc(alias = "RBX::SpawnLocation::updateSpawnerTouched(void)")]
#[doc(alias = "__ZN3RBX13SpawnLocation20updateSpawnerTouchedEv")]
pub fn stub_063d858(state: &SpawnLocationState, connected: &mut bool, client_present: bool) {
    // IDA 0x63d858 (`RBX::SpawnLocation::updateSpawnerTouched`): when
    // +349 is set and no client is present (0x63d882-0x63d8b6), an
    // unconnected +0x154 link connects `TouchedSignal` with
    // `bind(onEvent_spawnerTouched)` and takes the scoped assignment
    // (0x63d8b8-0x63d944, at 0x63e4ec); otherwise a connected link
    // disconnects (0x63d944-0x63da94). The `FLog::TouchedSignal`
    // `FastLog`s and the `onDemandRead/Write` calls are diagnostics
    // and fold away; the bind target folds into `stub_063d7b8`.
    if state.allow_team_change_on_touch && !client_present {
        if !*connected {
            stub_063e4ec(connected);
        }
    } else if *connected {
        *connected = false;
    }
}

// 0x063da9c — __ZN3RBX13SpawnLocation17onServiceProviderEPNS_15ServiceProviderES2_
// demangled: RBX::SpawnLocation::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
// type: _DWORD __fastcall(RBX::SpawnLocation *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::SpawnLocation::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
#[doc(alias = "__ZN3RBX13SpawnLocation17onServiceProviderEPNS_15ServiceProviderES2_")]
pub fn stub_063da9c(
    spawns: &mut Vec<SharedPtr<SpawnLocationState>>,
    location: &SharedPtr<SpawnLocationState>,
    new_service: bool,
    old_service: bool,
) {
    // IDA 0x63da9c (`RBX::SpawnLocation::onServiceProvider`): the
    // `PartInstance::onServiceProvider` base folds away (0x63daac); a
    // non-null new provider creates the `SpawnerService`
    // (`ReleaseAssert(ss)`, 0x63dab0-0x63db02) and hooks a fresh node
    // holding `this` into the service's +0x60 list (0x63db06-0x63db14,
    // `operator new(0xc)` + `_List_node_base::hook`); a non-null old
    // provider finds it (`ReleaseAssert(ss)`) and unhooks (same
    // hook/unhook discipline as `list::remove` at 0x63e66c). The
    // asserts fold into the presence flags; the node folds into the
    // `Vec` slot.
    if new_service {
        spawns.push(SharedPtr::clone(location));
    }
    if old_service {
        stub_063e66c(spawns, location);
    }
}

// 0x063db8c — __ZN3RBX14SpawnerServiceC2Ev
// demangled: RBX::SpawnerService::SpawnerService(void)
// type: _DWORD __fastcall(RBX::SpawnerService *__hidden this)
#[doc(alias = "RBX::SpawnerService::SpawnerService(void)")]
#[doc(alias = "__ZN3RBX14SpawnerServiceC2Ev")]
pub fn stub_063db8c() -> SpawnerServiceState {
    // IDA 0x63db8c (`RBX::SpawnerService::SpawnerService`):
    // `Instance::C2` + vtable installs + class registration
    // (0x63dbac-0x63dc30); the +92 flag byte is set to 1 (0x63dc34-
    // 0x63dc40); the list at +96 (words 24/25) is self-linked empty
    // (0x63dc48-0x63dc60); `setName("SpawnerService")` (0x63dc64-
    // 0x63dc90); the task-scheduler call folds away (0x63dc94-
    // 0x63dca8).
    SpawnerServiceState {
        flag_92: true,
        spawns: Vec::new(),
    }
}

// 0x063ddd8 — __ZN3RBX14SpawnerServiceD0Ev
// demangled: RBX::SpawnerService::~SpawnerService()
// type: void __fastcall(RBX::SpawnerService *__hidden this)
#[doc(alias = "RBX::SpawnerService::~SpawnerService()")]
#[doc(alias = "__ZN3RBX14SpawnerServiceD0Ev")]
pub fn stub_063ddd8() {
    // IDA 0x063ddd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063de78 — __ZN3RBX14SpawnerServiceD1Ev
// demangled: RBX::SpawnerService::~SpawnerService()
// type: void __fastcall(RBX::SpawnerService *__hidden this)
#[doc(alias = "RBX::SpawnerService::~SpawnerService()")]
#[doc(alias = "__ZN3RBX14SpawnerServiceD1Ev")]
pub fn stub_063de78() {
    // IDA 0x063de78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063de7c — __ZThn32_N3RBX14SpawnerServiceD0Ev
// demangled: non-virtual thunk toRBX::SpawnerService::~SpawnerService()
// type: void __fastcall(RBX::SpawnerService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpawnerService::~SpawnerService()")]
#[doc(alias = "__ZThn32_N3RBX14SpawnerServiceD0Ev")]
pub fn stub_063de7c() {
    // IDA 0x063de7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063de84 — __ZThn36_N3RBX14SpawnerServiceD0Ev
// demangled: non-virtual thunk toRBX::SpawnerService::~SpawnerService()
// type: void __fastcall(RBX::SpawnerService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpawnerService::~SpawnerService()")]
#[doc(alias = "__ZThn36_N3RBX14SpawnerServiceD0Ev")]
pub fn stub_063de84() {
    // IDA 0x063de84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063de8c — __ZN3RBX14SpawnerServiceD2Ev
// demangled: RBX::SpawnerService::~SpawnerService()
// type: void __fastcall(RBX::SpawnerService *__hidden this)
#[doc(alias = "RBX::SpawnerService::~SpawnerService()")]
#[doc(alias = "__ZN3RBX14SpawnerServiceD2Ev")]
pub fn stub_063de8c() {
    // IDA 0x063de8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063ded4 — __ZThn32_N3RBX14SpawnerServiceD1Ev
// demangled: non-virtual thunk toRBX::SpawnerService::~SpawnerService()
// type: void __fastcall(RBX::SpawnerService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpawnerService::~SpawnerService()")]
#[doc(alias = "__ZThn32_N3RBX14SpawnerServiceD1Ev")]
pub fn stub_063ded4() {
    // IDA 0x063ded4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063dedc — __ZThn36_N3RBX14SpawnerServiceD1Ev
// demangled: non-virtual thunk toRBX::SpawnerService::~SpawnerService()
// type: void __fastcall(RBX::SpawnerService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::SpawnerService::~SpawnerService()")]
#[doc(alias = "__ZThn36_N3RBX14SpawnerServiceD1Ev")]
pub fn stub_063dedc() {
    // IDA 0x063dedc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063dee4 — __ZN3RBX14SpawnerService13ClearContentsEv
// demangled: RBX::SpawnerService::ClearContents(void)
// type: _DWORD __fastcall(RBX::SpawnerService *__hidden this)
#[doc(alias = "RBX::SpawnerService::ClearContents(void)")]
#[doc(alias = "__ZN3RBX14SpawnerService13ClearContentsEv")]
pub fn stub_063dee4(service: &mut SpawnerServiceState) {
    // IDA 0x63dee4 (`RBX::SpawnerService::ClearContents`): walks the
    // +96 list deleting every node (0x63dee4-0x63defe) and re-links it
    // empty (0x63df00-0x63df04). The node deletes fold into `clear`.
    service.spawns.clear();
}

// 0x063df08 — __ZN3RBX14SpawnerService16GetSpawnLocationEPNS_7Network6PlayerESs
// demangled: RBX::SpawnerService::GetSpawnLocation(RBX::Network::Player *,std::string)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::SpawnerService::GetSpawnLocation(RBX::Network::Player *,std::string)")]
#[doc(alias = "__ZN3RBX14SpawnerService16GetSpawnLocationEPNS_7Network6PlayerESs")]
pub fn stub_063df08(
    spawns: &[SharedPtr<SpawnLocationState>],
    player: &SpawnPlayerRef,
) -> Vec<SharedPtr<SpawnLocationState>> {
    // IDA 0x63df08 (`RBX::SpawnerService::GetSpawnLocation`): walks
    // the +96 list (0x63df46-0x63df6a); a spawn is collected when its
    // +348 Neutral flag is set, or when the player is not neutral and
    // the player's +100 TeamColor equals the spawn's +0x150
    // (0x63df6e-0x63dfa2). The name arg is unused beyond the
    // signature. The node walk folds into the slice.
    spawns
        .iter()
        .filter(|spawn| {
            spawn.neutral || (!player.neutral && player.team_color == spawn.team_color)
        })
        .cloned()
        .collect()
}

// 0x063e090 — __ZN3RBX14SpawnerService11SpawnPlayerEPNS_9WorkspaceEN5boost10shared_ptrINS_13ModelInstanceEEEN3G3D7Vector3Ei
// demangled: RBX::SpawnerService::SpawnPlayer(RBX::Workspace *,boost::shared_ptr<RBX::ModelInstance>,G3D::Vector3,int)
// type: int __fastcall(int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::SpawnerService::SpawnPlayer(RBX::Workspace *,boost::shared_ptr<RBX::ModelInstance>,G3D::Vector3,int)")]
#[doc(alias = "__ZN3RBX14SpawnerService11SpawnPlayerEPNS_9WorkspaceEN5boost10shared_ptrINS_13ModelInstanceEEEN3G3D7Vector3Ei")]
pub fn stub_063e090(
    workspace_present: bool,
    debug_asserts: bool,
    forcefield_seconds: i32,
    effects: &mut Vec<SpawnEffect>,
) {
    // IDA 0x63e090 (`RBX::SpawnerService::SpawnPlayer`):
    // `ReleaseAssert(workspace)` (SpawnLocation.cpp line 197,
    // 0x63e0ee-0x63e136 — gated on `FLog::Asserts`, a host seam);
    // null workspace returns (0x63e138-0x63e13c); with a positive
    // force-field duration it creates a `ForceField`, parents it to
    // the character and files it via `create<DebrisService>` (with
    // the `ds` ReleaseAssert, line 212) + `addItem(ff,
    // (double)duration)` (0x63e140-0x63e1c0). The world/character
    // handles fold into the effect; the tail (positioning) rides the
    // caller's scene state.
    if debug_asserts {
        assert!(
            workspace_present,
            "workspace file: SpawnLocation.cpp line: 197 (IDA 0x63e090)"
        );
    }
    if !workspace_present {
        return;
    }
    if forcefield_seconds > 0 {
        effects.push(SpawnEffect::ForceField {
            duration_secs: forcefield_seconds as f64,
        });
    }
}

// 0x063e2a8 — __ZN3RBX13SpawnLocation31onAllowTeamChangeOnTouchChangedERKNS_10Reflection18PropertyDescriptorE
// demangled: RBX::SpawnLocation::onAllowTeamChangeOnTouchChanged(RBX::Reflection::PropertyDescriptor const&)
// type: _DWORD __fastcall(RBX::SpawnLocation *__hidden this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::SpawnLocation::onAllowTeamChangeOnTouchChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX13SpawnLocation31onAllowTeamChangeOnTouchChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_063e2a8(state: &SpawnLocationState, connected: &mut bool, client_present: bool) {
    // IDA 0x63e2a8
    // (`RBX::SpawnLocation::onAllowTeamChangeOnTouchChanged`): thunk
    // tail-calling `updateSpawnerTouched` above (0x63e2a8-0x63e2ab).
    stub_063d858(state, connected, client_present);
}

// 0x063e2ac — __ZN3RBX10Reflection14PropDescriptorINS_13SpawnLocationENS_10BrickColorEED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::SpawnLocation,RBX::BrickColor>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SpawnLocation,RBX::BrickColor>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13SpawnLocationENS_10BrickColorEED1Ev")]
pub fn stub_063e2ac() {
    // IDA 0x063e2ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063e4ec — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_13SpawnLocationENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
// demangled: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpawnLocation,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpawnLocation,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation*>,boost::arg<1>>>)
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpawnLocation,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpawnLocation,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::SpawnLocation*>,boost::arg<1>>>)")]
#[doc(alias = "__ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_13SpawnLocationENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_")]
pub fn stub_063e4ec(connected: &mut bool) {
    // IDA 0x63e4ec (`PartInstance::TouchedSignal::connect<bind(mf1
    // onEvent_spawnerTouched)>`): logs under `FLog::TouchedSignal`
    // (0x63e54a-0x63e564), builds the `TouchedSlot` from the stored
    // bind triple (0x63e568-0x63e590), runs `signal::connect`
    // (0x63e594-0x63e59e) and returns the connection (0x63e5a2-
    // 0x63e5c0). Only called from `updateSpawnerTouched` when the
    // +0x154 link is down, so the link comes up here; the slot and
    // the logging fold away.
    *connected = true;
}

// 0x063e66c — __ZNSt4listIPN3RBX13SpawnLocationESaIS2_EE6removeERKS2_
// demangled: std::list<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::remove(RBX::SpawnLocation * const&)
// type: int(void)
#[doc(alias = "std::list<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::remove(RBX::SpawnLocation * const&)")]
#[doc(alias = "__ZNSt4listIPN3RBX13SpawnLocationESaIS2_EE6removeERKS2_")]
pub fn stub_063e66c(
    spawns: &mut Vec<SharedPtr<SpawnLocationState>>,
    location: &SharedPtr<SpawnLocationState>,
) {
    // IDA 0x63e66c (`list<SpawnLocation*>::remove`): walks the +96
    // list unhooking and deleting every node holding the value
    // (0x63e678-0x63e69a). The nodes fold into the `Vec` slots.
    spawns.retain(|spawn| !SharedPtr::ptr_eq(spawn, location));
}

// 0x063e6a4 — __ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::push_back(RBX::SpawnLocation * const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::push_back(RBX::SpawnLocation * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE9push_backERKS2_")]
pub fn stub_063e6a4(
    spawns: &mut Vec<SharedPtr<SpawnLocationState>>,
    location: SharedPtr<SpawnLocationState>,
) {
    // IDA 0x63e6a4 (`vector<SpawnLocation*>::push_back`): fast path
    // stores at finish and bumps it (0x63e6b2-0x63e6c0); a full buffer
    // delegates to `_M_insert_aux` (0x63e6ca, host: stub_063f508).
    // Host: `Vec::push` covers both.
    spawns.push(location);
}

// 0x063e6d0 — __ZN3RBX15ServiceProvider6createINS_13DebrisServiceEEEPT_PKNS_8InstanceE
// demangled: RBX::DebrisService * RBX::ServiceProvider::create<RBX::DebrisService>(RBX::Instance const*)
// type: int(void)
#[doc(alias = "RBX::DebrisService * RBX::ServiceProvider::create<RBX::DebrisService>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_13DebrisServiceEEEPT_PKNS_8InstanceE")]
pub fn stub_063e6d0(provider_present: bool) -> bool {
    // IDA 0x63e6d0 (`ServiceProvider::create<DebrisService>`):
    // `findServiceProvider` (0x63e6d4); a null provider returns null
    // (0x63e6dc), else the service is created-or-got on it (0x63e6e4,
    // always non-null). The provider/service lookup folds into
    // presence flags.
    provider_present
}

// 0x063edbc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13SpawnLocationEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::SpawnLocation> RBX::Creatable<RBX::Instance>::create<RBX::SpawnLocation>(void)
#[doc(alias = "boost::shared_ptr<RBX::SpawnLocation> RBX::Creatable<RBX::Instance>::create<RBX::SpawnLocation>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_13SpawnLocationEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_063edbc(touched_logging: bool) -> SharedPtr<SpawnLocationState> {
    // IDA 0x63edbc (`Creatable<Instance>::create<SpawnLocation>`):
    // `operator new(0x164)` (0x63edf2) + the `SpawnLocation` ctor
    // (0x63ee16, host: stub_063d248) + the adopting `shared_ptr`
    // with `Creatable::Deleter` (0x63ee24, host: stub_063ee70). Arc
    // construction adopts owners.
    SharedPtr::new(stub_063d248(touched_logging))
}

// 0x063ee70 — __ZN5boost10shared_ptrIN3RBX13SpawnLocationEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::SpawnLocation>::shared_ptr<RBX::SpawnLocation,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "boost::shared_ptr<RBX::SpawnLocation>::shared_ptr<RBX::SpawnLocation,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13SpawnLocationEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_063ee70(location: SharedPtr<SpawnLocationState>) -> SharedPtr<SpawnLocationState> {
    // IDA 0x63ee70 (`shared_ptr<SpawnLocation>` from raw + Deleter):
    // stores the pointer (0x63ee90), builds the `shared_count` with
    // the deleter (0x63ee98, host: stub_063f020) and wires the weak
    // owner for non-null (0x63eec6-0x63eed6, host: stub_063ef38). Arc
    // move covers both; the control block folds into the `Arc`.
    location
}

// 0x063ef38 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13SpawnLocationES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SpawnLocation,RBX::SpawnLocation>(boost::shared_ptr<RBX::SpawnLocation> const*,RBX::SpawnLocation *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SpawnLocation,RBX::SpawnLocation>(boost::shared_ptr<RBX::SpawnLocation> const*,RBX::SpawnLocation *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13SpawnLocationES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_063ef38() {
    // IDA 0x063ef38: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x063f020 — __ZN5boost6detail12shared_countC2IPN3RBX13SpawnLocationENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX13SpawnLocationENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_063f020() {
    // IDA 0x063f020: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x063f128 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SpawnLocationENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SpawnLocationENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_063f128() {
    // IDA 0x063f128: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063f12c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SpawnLocationENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SpawnLocationENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_063f12c() {
    // IDA 0x063f12c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x063f130 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SpawnLocationENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SpawnLocationENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_063f130() {
    // IDA 0x063f130: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x063f150 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SpawnLocationENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SpawnLocationENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_063f150() {
    // IDA 0x063f150: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x063f168 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SpawnLocationENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpawnLocation *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SpawnLocationENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_063f168() {
    // IDA 0x063f168: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x063f508 — __ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpawnLocation **,std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>>,RBX::SpawnLocation * const&)
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpawnLocation **,std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>>,RBX::SpawnLocation * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_063f508(
    spawns: &mut Vec<SharedPtr<SpawnLocationState>>,
    index: usize,
    location: SharedPtr<SpawnLocationState>,
) {
    // IDA 0x63f508 (`vector<SpawnLocation*>::_M_insert_aux`): spare
    // capacity shifts the tail up one slot and stores at the position
    // (0x63f520-0x63f544); a full buffer grows (length_error at
    // 0x3fffffff, 0x63f5d0-0x63f5e2), allocates via `_M_allocate`
    // (0x63f568, host: stub_063f5e8), memmoves both halves around the
    // new element and swaps the buffer (0x63f56c-0x63f5c0). Host: a
    // single `insert` covers both (same shape as `SoundType` at
    // 0x3800d4).
    spawns.insert(index, location);
}

// 0x063f5e8 — __ZNSt12_Vector_baseIPN3RBX13SpawnLocationESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_allocate(unsigned long)
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX13SpawnLocationESaIS2_EE11_M_allocateEm")]
pub fn stub_063f5e8(count: usize) -> Vec<SharedPtr<SpawnLocationState>> {
    // IDA 0x63f5e8 (`_Vector_base<SpawnLocation*>::_M_allocate`):
    // count >= 0x40000000 -> `__throw_bad_alloc` (0x63f5f0-0x63f5f2,
    // host: panic); else `operator new(4 * count)`. Host: a
    // capacity-only `Vec` (len 0, like fresh storage).
    assert!(count < 0x40000000, "std::bad_alloc (IDA 0x63f5f2)");
    Vec::with_capacity(count)
}

/// Bound `void (*)(shared_ptr<DataModel>, string)` call behind the
/// `boost::function<void(DataModel*)>` built at 0x65680c-0x657104:
/// the two bound values (the model link + the message). The call
/// formal (`DataModel*`) has no `arg<>` slot in the bind list, so
/// invocation drops it and runs the target on the captures (IDA
/// 0x65717c). The model handle folds into a presence flag.
#[derive(Debug, Clone)]
pub struct BoundDataModelCall {
    pub has_model: bool,
    pub message: String,
}
impl BoundDataModelCall {
    pub fn new(has_model: bool, message: &str) -> Self {
        Self {
            has_model,
            message: message.to_owned(),
        }
    }
}
// 0x065680c — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS7_5list2INS7_5valueISA_EENSE_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS7_5list2INS7_5valueISA_EENSE_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS7_5list2INS7_5valueISA_EENSE_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_065680c(has_model: bool, message: &str) -> BoundDataModelCall {
    // IDA 0x65680c (`function<void(DataModel*)>` from the bind):
    // copies the bind triple (`shared_count` addref at 0x656840, the
    // string at 0x65687e), assigns it (0x656890) and releases the
    // temps (0x6568a4-0x6568b8). Host: move the captures into the
    // closure struct.
    BoundDataModelCall::new(has_model, message)
}

// 0x0656994 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS6_5list2INS6_5valueIS9_EENSD_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS6_5list2INS6_5valueIS9_EENSD_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS6_5list2INS6_5valueIS9_EENSD_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0656994(has_model: bool, message: &str) -> BoundDataModelCall {
    // IDA 0x656994 (`function1<void,DataModel*>` from the bind):
    // clears the slot (0x6569b4) then the same copy triple + assign
    // (0x6569cc-0x656a1c) with temp release (0x656a30-0x656a44).
    // Host: same closure construction.
    BoundDataModelCall::new(has_model, message)
}

// 0x0656b20 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS6_5list2INS6_5valueIS9_EENSD_ISsEEEEEEEEvT_
// demangled: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>)
// type: int(void)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>)")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS6_5list2INS6_5valueIS9_EENSD_ISsEEEEEEEEvT_")]
pub fn stub_0656b20(slot: &mut Option<BoundDataModelCall>, call: BoundDataModelCall) {
    // IDA 0x656b20 (`function1::assign_to`): copies the bind triple
    // (0x656b54-0x656b92), delegates to the vtable `assign_to`
    // (0x656bb4, host: stub_0656cf0), releases the temps and stores
    // the vtable (trailing store). Host: fill the slot; the vtable
    // folds in.
    *slot = Some(call);
}

// 0x0656cb8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")]
pub fn stub_0656cb8() {
    // IDA 0x0656cb8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x0656cd4 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEvPS7_E6invokeERNS1_15function_bufferESH_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEvPS7_E6invokeERNS1_15function_bufferESH_")]
pub fn stub_0656cd4() {
    // IDA 0x0656cd4: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

// 0x0656cf0 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const
// type: int(void)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0656cf0(slot: &mut Option<BoundDataModelCall>, call: BoundDataModelCall) -> bool {
    // IDA 0x656cf0 (`basic_vtable1::assign_to` into a buffer):
    // copies the bind triple (0x656d14-0x656d64), delegates to the
    // tagged overload (0x656d78, host: stub_0656e78), releases the
    // temps and returns 1 (0x656dbc). Host: fill the slot, report
    // success.
    *slot = Some(call);
    true
}

// 0x0656e78 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, void *, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0656e78(slot: &mut Option<BoundDataModelCall>, call: BoundDataModelCall) -> bool {
    // IDA 0x656e78 (`basic_vtable1::assign_to` with `function_obj_tag`):
    // copies the bind triple (0x656e9e-0x656eea), installs it via
    // `assign_functor` (0x656efc, host: stub_0656ffc), releases the
    // temps and returns 1 (0x656f40). Host: fill the slot, report
    // success.
    *slot = Some(call);
    true
}

// 0x0656ffc — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// demangled: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0656ffc(call: BoundDataModelCall) -> Box<BoundDataModelCall> {
    // IDA 0x656ffc (`basic_vtable1::assign_functor`, not-heap case):
    // `operator new(0x10)` (0x657024) + copies the bind triple into
    // it (0x65702a-0x657090) and publishes the pointer (0x657096).
    // Host: box the captures.
    Box::new(call)
}

// 0x0657104 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX9DataModelEEEEENS2_ISsEEEclIPFvS6_SsENS0_5list1IRPS5_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>::operator()<void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::DataModel>,std::string) &,boost::_bi::list1<RBX::DataModel*&> &,int)
// type: int(void)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>::operator()<void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::DataModel>,std::string) &,boost::_bi::list1<RBX::DataModel*&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX9DataModelEEEEENS2_ISsEEEclIPFvS6_SsENS0_5list1IRPS5_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0657104(call: &BoundDataModelCall, target: impl Fn(bool, &str)) {
    // IDA 0x657104 (`list2::operator()`): copies the bound model
    // link (0x65712c-0x657148) and message (0x65716e), runs the
    // stored target on them (0x65717c) and releases the temps
    // (0x65718c-0x65719e). The `DataModel*` call formal has no
    // `arg<>` slot, so it is dropped. Host: invoke the closure
    // directly (same shape as the `Heartbeat` bind at 0x3791a4).
    target(call.has_model, &call.message)
}

// 0x0657270 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0657270() {
    // IDA 0x0657270: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x065740c — __GLOBAL__I_a_263
// demangled: global constructor keyed to_a_263
#[doc(alias = "global constructor keyed to_a_263")]
#[doc(alias = "__GLOBAL__I_a_263")]
pub fn stub_065740c() {
    // IDA 0x65740c (`__GLOBAL__I_a_263`): `generic_category` x2 +
    // `system_category` stores (0x657416-0x657434), `ios_base::Init`
    // + `__cxa_atexit` (0x65743a-0x65745a), the `FLog`
    // `ForceProdLoggingService` flag (0x65745e-0x65747c), the
    // `BoundFuncDesc` registrations (`ProfilingItem::GetTimes` /
    // `GetTimesForFrames`, `StatsService::Report` /
    // `ReportTaskScheduler` / `ReportJobsStepWindow` / `SetReportUrl`,
    // `Stats::Item::GetValueString` / `GetValue`) and `BoundProp`s
    // (`ReporterType`, `MinReportInterval`) with their `__cxa_atexit`
    // teardowns, the `ClassRegistrar` inits, the `boost::exception`
    // statics, the `singleton_pool` guards and the `ScriptContext`
    // `creatorPrivate` (0x657480-0x657932). Host statics initialize
    // on use; only the run is recorded.
    watchdog13_static_init();
}

// 0x065793c — __ZN3RBX10StudioTool10setEnabledEb
// demangled: RBX::StudioTool::setEnabled(bool)
// type: _DWORD __fastcall(RBX::StudioTool *__hidden this, bool)
#[doc(alias = "RBX::StudioTool::setEnabled(bool)")]
#[doc(alias = "__ZN3RBX10StudioTool10setEnabledEb")]
pub fn stub_065793c() -> ! {
    todo!("0x065793c RBX::StudioTool::setEnabled(bool)")
}

// 0x065795c — __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev")]
pub fn stub_065795c() {
    // IDA 0x065795c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0657980 — __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_0657980() {
    // IDA 0x0657980: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06579a4 — __ZNK3RBX10StudioTool10getEnabledEv
// demangled: RBX::StudioTool::getEnabled(void)const
// type: _DWORD __fastcall(RBX::StudioTool *__hidden this)
#[doc(alias = "RBX::StudioTool::getEnabled(void)const")]
#[doc(alias = "__ZNK3RBX10StudioTool10getEnabledEv")]
pub fn stub_06579a4() -> ! {
    todo!("0x06579a4 RBX::StudioTool::getEnabled(void)const")
}

// 0x06579ac — __ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbED1Ev")]
pub fn stub_06579ac() {
    // IDA 0x06579ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06579d0 — __ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::PropDescriptor<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>(char const*,char const*,bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::PropDescriptor<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>(char const*,char const*,bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_06579d0() -> ! {
    todo!("0x06579d0 RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::PropDescriptor<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>(char const*,char const*,bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x0657ae4 — __ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbED0Ev")]
pub fn stub_0657ae4() {
    // IDA 0x0657ae4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0657b10 — __ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_0657b10() -> ! {
    todo!("0x0657b10 RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::isReadOnly(void)const")
}

// 0x0657b14 — __ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_0657b14() -> ! {
    todo!("0x0657b14 RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::isWriteOnly(void)const")
}

// 0x0657b18 — __ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0657b18() -> ! {
    todo!("0x0657b18 RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x0657b3c — __ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0657b3c() -> ! {
    todo!("0x0657b3c RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x0657b60 — __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// demangled: RBX::Reflection::EventDesc<RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_0657b60() {
    // IDA 0x0657b60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0657c14 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// demangled: RBX::Reflection::EventDescImpl<0,RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_0657c14() -> ! {
    todo!("0x0657c14 RBX::Reflection::EventDescImpl<0,RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x0657e18 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// demangled: RBX::Reflection::EventDescImpl<0,RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_0657e18() -> ! {
    todo!("0x0657e18 RBX::Reflection::EventDescImpl<0,RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x0657e8c — __ZNK3RBX10Reflection13EventDescBaseINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// demangled: RBX::Reflection::EventDescBase<RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::disconnectAll(RBX::Reflection::EventSource *)const
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0657e8c() -> ! {
    todo!("0x0657e8c RBX::Reflection::EventDescBase<RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x0657ea0 — __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::EventDesc<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0657ea0() {
    // IDA 0x0657ea0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x0658024 — __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
// demangled: RBX::Reflection::EventDesc<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev")]
pub fn stub_0658024() {
    // IDA 0x0658024: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06580d8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// demangled: RBX::Reflection::EventDescImpl<1,RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
pub fn stub_06580d8() -> ! {
    todo!("0x06580d8 RBX::Reflection::EventDescImpl<1,RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x065822c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// demangled: RBX::Reflection::EventDescImpl<1,RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
pub fn stub_065822c() -> ! {
    todo!("0x065822c RBX::Reflection::EventDescImpl<1,RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x065838c — __ZNK3RBX10Reflection13EventDescBaseINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// demangled: RBX::Reflection::EventDescBase<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::disconnectAll(RBX::Reflection::EventSource *)const
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_065838c() -> ! {
    todo!("0x065838c RBX::Reflection::EventDescBase<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x06583a0 — __GLOBAL__I_a_264
// demangled: global constructor keyed to_a_264
#[doc(alias = "global constructor keyed to_a_264")]
#[doc(alias = "__GLOBAL__I_a_264")]
pub fn stub_06583a0() -> ! {
    todo!("0x06583a0 global constructor keyed to_a_264")
}

// 0x0658744 — __GLOBAL__I_a_265
// demangled: global constructor keyed to_a_265
#[doc(alias = "global constructor keyed to_a_265")]
#[doc(alias = "__GLOBAL__I_a_265")]
pub fn stub_0658744() -> ! {
    todo!("0x0658744 global constructor keyed to_a_265")
}

// 0x06589e8 — __ZN3RBX7SurfaceC1EPNS_12PartInstanceENS_8NormalIdE
// demangled: RBX::Surface::Surface(RBX::PartInstance *,RBX::NormalId)
#[doc(alias = "RBX::Surface::Surface(RBX::PartInstance *,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX7SurfaceC1EPNS_12PartInstanceENS_8NormalIdE")]
pub fn stub_06589e8() -> ! {
    todo!("0x06589e8 RBX::Surface::Surface(RBX::PartInstance *,RBX::NormalId)")
}
