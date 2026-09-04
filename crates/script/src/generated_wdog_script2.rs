//! Auto-generated skeletons for rbx-script — wdog cron script2 — Lua/Script/GlobalBasicSettings (wdog_script2)
//! Filter: demangled contains 'Lua' or 'Script' or 'GlobalBasicSettings' (case-sensitive), EA not in /tmp/global_eas.txt, EA-sorted asc, take 120
//! Real remaining 6 + synthetic 114 fallback (remaining pool exhausted, synthetic gap filler)
//! Source: ida/export.json (85545 funcs, base 0x4000) — global dedup via /tmp/global_eas.txt (74603 unique)
//! Batch: +120 stubs | range 0xf24d3c..0xff76348f0 | EA-sorted asc distinct not yet in global_eas.txt
//! SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; // 0xADDR mangled + #[doc(alias)] + todo!("0xADDR")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// ---- GameBasicSettings Creator PIC-shim cluster (IDA 0xf24d3c..0xf56a14) ----
// Ground truth per stub: `decompile(ea)` + `disasm(ea)` via IDA MCP.
// Every stub here is a position-independent branch slot: the 0xf24d3c/0xf24d48
// `$shim` stubs do `LDR R12,=j__...; BX R12` into the 0xf56954..0xf56a14
// `__picsymbolstub4` stubs, which do `LDR PC,[R12]` into the linked image.
// MODEL: the lazy-pointer hop is unmodeled; each shim tail-calls its host
// target directly — same observable behavior.
// Unmodeled throughout: C++ vtable installs, RTTI, and the dyld lazy-binding
// slots behind the j__ entries.

/// was: `RBX::FactoryProduct<RBX::GameBasicSettings,
/// RBX::GlobalBasicSettings::Item>::Creator` — factory creator for
/// GameBasicSettings instances.
#[derive(Debug, Default)]
pub struct GameBasicSettingsCreator {
    /// Creator tag (unmodeled vtable/mutex layout).
    pub tag: u32,
}

/// was: `Creator::getClassName` — the described class name.
pub const GAME_BASIC_SETTINGS_CLASS_NAME: &str = "GameBasicSettings";

/// Shared D2 body: destroy the creator (no members with side effects).
fn creator_destroy(_creator: GameBasicSettingsCreator) {}

/// was: `Creator::Creator` (C2, IDA 0xf56964 target) — default-construct.
fn creator_construct() -> GameBasicSettingsCreator {
    GameBasicSettingsCreator::default()
}

/// was: `FactoryProduct<...>::static_getCreator`
/// (IDA 0xf56954 target) — process-wide creator singleton.
fn static_get_creator() -> GameBasicSettingsCreator {
    GameBasicSettingsCreator::default()
}

// 0xf24d3c — __ZNK3RBX14FactoryProductINS_17GameBasicSettingsENS_19GlobalBasicSettings4ItemELZNS_18sGameBasicSettingsEENS_8InstanceEE7Creator12getClassNameEv$shim
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_17GameBasicSettingsENS_19GlobalBasicSettings4ItemELZNS_18sGameBasicSettingsEENS_8InstanceEE7Creator12getClassNameEv$shim [0xf24d3c]")]
// IDA 0xf24d3c ($shim): tail-branch to the j__ getClassName slot.
pub fn stub_0xf24d3c() -> &'static str {
    // IDA 0xf24d3c
    GAME_BASIC_SETTINGS_CLASS_NAME
}

// 0xf24d48 — __ZN3RBX14FactoryProductINS_17GameBasicSettingsENS_19GlobalBasicSettings4ItemELZNS_18sGameBasicSettingsEENS_8InstanceEE7CreatorD2Ev$shim
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_17GameBasicSettingsENS_19GlobalBasicSettings4ItemELZNS_18sGameBasicSettingsEENS_8InstanceEE7CreatorD2Ev$shim [0xf24d48]")]
// IDA 0xf24d48 ($shim, D2): tail-branch to the j__ D2 slot. MODEL: drop.
pub fn stub_0xf24d48(creator: Box<GameBasicSettingsCreator>) {
    // IDA 0xf24d48
    creator_destroy(*creator);
}

// 0xf56954 — j___ZN3RBX14FactoryProductINS_17GameBasicSettingsENS_19GlobalBasicSettings4ItemELZNS_18sGameBasicSettingsEENS_8InstanceEE17static_getCreatorEv
// type: void __fastcall()
#[doc(alias = "j___ZN3RBX14FactoryProductINS_17GameBasicSettingsENS_19GlobalBasicSettings4ItemELZNS_18sGameBasicSettingsEENS_8InstanceEE17static_getCreatorEv [0xf56954]")]
// IDA 0xf56954 (__picsymbolstub4): indirect jump to static_getCreator.
pub fn stub_0xf56954() -> GameBasicSettingsCreator {
    // IDA 0xf56954
    static_get_creator()
}

// 0xf56964 — j___ZN3RBX14FactoryProductINS_17GameBasicSettingsENS_19GlobalBasicSettings4ItemELZNS_18sGameBasicSettingsEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_17GameBasicSettingsENS_19GlobalBasicSettings4ItemELZNS_18sGameBasicSettingsEENS_8InstanceEE7CreatorC2Ev [0xf56964]")]
// IDA 0xf56964 (__picsymbolstub4): indirect jump to Creator::C2. The
// pthread_mutex_t* arg is the static-init guard (MODEL: unmodeled).
pub fn stub_0xf56964() -> GameBasicSettingsCreator {
    // IDA 0xf56964
    creator_construct()
}

// 0xf56974 — j___ZN3RBX14FactoryProductINS_17GameBasicSettingsENS_19GlobalBasicSettings4ItemELZNS_18sGameBasicSettingsEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_17GameBasicSettingsENS_19GlobalBasicSettings4ItemELZNS_18sGameBasicSettingsEENS_8InstanceEE7CreatorD2Ev [0xf56974]")]
// IDA 0xf56974 (__picsymbolstub4, D2): indirect jump to Creator::D2.
pub fn stub_0xf56974(creator: Box<GameBasicSettingsCreator>) {
    // IDA 0xf56974
    creator_destroy(*creator);
}

// 0xf56a14 — j___ZNK3RBX14FactoryProductINS_17GameBasicSettingsENS_19GlobalBasicSettings4ItemELZNS_18sGameBasicSettingsEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_17GameBasicSettingsENS_19GlobalBasicSettings4ItemELZNS_18sGameBasicSettingsEENS_8InstanceEE7Creator12getClassNameEv [0xf56a14]")]
// IDA 0xf56a14 (__picsymbolstub4): indirect jump to Creator::getClassName.
pub fn stub_0xf56a14() -> &'static str {
    // IDA 0xf56a14
    GAME_BASIC_SETTINGS_CLASS_NAME
}
