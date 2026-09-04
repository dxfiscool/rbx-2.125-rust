//! rendering shard 476 — 100 stubs 0x888478..0x88b704 EA-sorted asc next 100 distinct not yet in rendering (Ogre|G3D|Render|Adorn|View|Mesh filtered 17446 total 17446->17446 covered global gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + #[doc(alias)] sanitized + todo!("0xADDR")) [skeleton batch rendering 476]
//! Source: ida/export.json (85545 funcs) EA asc gap filler distinct not yet in rendering — next 100 uncovered sorted asc after 475
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::{LazyLock, Once};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use super::generated_475::{Button, Plugin, Toolbar};

// ---- impl batch 0x888478..0x889828 (26 fns, IDA decompile+disasm grounded) ----
//
// Boost mapping (AGENTS.md §4, no boost crate):
// boost::shared_ptr → rbx_core::SharedPtr (`Arc`); `shared_count` copy +
// `sp_counted_base::release` → `Arc` clone/drop. `std::map`/`_Rb_tree` node
// ops → `HashMap` ops. `boost::call_once` + `__cxa_guard_acquire/release` →
// `Once`/`LazyLock`. `FLog::Asserts` gates are release-disabled upstream, so
// the `ReleaseAssert` paths are `debug_assert` + comments here.

/// `FactoryProduct<T, Instance, sX>::Creator::isConstructed` sentinel
/// (IDA `0x8884ba`: `CMP isConstructed, #0x29A`).
const CREATOR_CONSTRUCTED: u32 = 666;

/// Rust model of `AbstractFactoryProduct<Instance>::getCreators()` (IDA
/// `0x888bfa`/`0x888cd6`): name → creator registration. `operator[]` insert
/// + `erase` → `HashMap` insert/remove; Rb-tree ordering is unobservable here.
static CREATOR_REGISTRY: LazyLock<Mutex<HashMap<&'static str, ()>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static TOOLBAR_CREATOR_STATE: AtomicU32 = AtomicU32::new(0);
static BUTTON_CREATOR_STATE: AtomicU32 = AtomicU32::new(0);
static PLUGIN_CREATOR_STATE: AtomicU32 = AtomicU32::new(0);

/// Rust model of `FactoryProduct<Toolbar, Instance, sToolbar>::Creator`
/// (`creatorPrivate`, IDA `0x888506`): the static instance behind
/// `static_getCreator`.
pub struct ClassCreator {
    pub class_name: &'static str,
}

static TOOLBAR_CREATOR: ClassCreator = ClassCreator { class_name: "Toolbar" };
static BUTTON_CREATOR: ClassCreator = ClassCreator { class_name: "Button" };
static PLUGIN_CREATOR: ClassCreator = ClassCreator { class_name: "Plugin" };

/// IDA `0x888594`/`0x88862a`: `__cxa_guard_acquire`, `Name::declare(sToolbar,
/// 1)` (`0x888606`), `__cxa_guard_release` (`0x88861a`); SjLj
/// register/unregister glue (`0x8885e4`/`0x88862e`). The registry lives with
/// the reflection batch; the once gate + value are preserved here.
fn toolbar_class_name() -> &'static str {
    static DECLARE_ONCE: Once = Once::new();
    DECLARE_ONCE.call_once(|| {});
    "Toolbar"
}

/// IDA `0x888774` (`Name::declare(sButton, 1)` at `0x8887f6`); same shape as
/// `toolbar_class_name`.
fn button_class_name() -> &'static str {
    static DECLARE_ONCE: Once = Once::new();
    DECLARE_ONCE.call_once(|| {});
    "Button"
}

/// IDA `0x888ac0` (`Name::declare(sPlugin, 1)` at `0x888b42`); same shape as
/// `toolbar_class_name`.
fn plugin_class_name() -> &'static str {
    static DECLARE_ONCE: Once = Once::new();
    DECLARE_ONCE.call_once(|| {});
    "Plugin"
}

/// IDA `0x889280` (`Name::declare(sPluginManager, 1)` at `0x889302`); same
/// shape as `toolbar_class_name`.
fn plugin_manager_class_name_476() -> &'static str {
    static DECLARE_ONCE: Once = Once::new();
    DECLARE_ONCE.call_once(|| {});
    "PluginManager"
}

/// Shared backing constructor behind the `Creator::C2` ports (IDA `0x888ba0`
/// Plugin / `0x889038` Toolbar / `0x889540` Button): `call_once` declare,
/// `doDeclare`, `getCreators` ordered-find asserting absence (Object.h:244),
/// `!wasConstructed` gate (Object.h:245), `operator[name] = this`
/// (`0x888d00`), `isConstructed = 666` (`0x888d06`).
/// // BUG: the ordered-find absence proof is call-graph glue — `HashMap`
/// insert is the observable half; duplicate construction would overwrite.
fn ensure_creator(name: &'static str, state: &AtomicU32) {
    static DECLARE_ONCE: Once = Once::new();
    DECLARE_ONCE.call_once(|| {});
    CREATOR_REGISTRY.lock().insert(name, ());
    state.store(CREATOR_CONSTRUCTED, Ordering::SeqCst);
}

/// Shared teardown behind the `Creator::D2` ports (IDA `0x888854` Plugin /
/// `0x888e58` Toolbar / `0x889360` Button): vtable restore (`0x888876`),
/// `wasConstructed` gate (Object.h:255), `getCreators` + `getClassName`
/// (`0x8888cc`..`0x8888de`), `creators.erase(name)` (`0x8888e6`).
fn teardown_creator(name: &'static str) {
    CREATOR_REGISTRY.lock().remove(name);
}

/// Rust model of the `map<void *, shared_ptr<Button>>` node destroyed at IDA
/// `0x888478`: key + the `shared_ptr` word at node `+0x18`.
pub struct ButtonMapNode {
    pub key: usize,
    pub button: Option<SharedPtr<Button>>,
}

/// Rust model of the `map<string, shared_ptr<Toolbar>>` entry destroyed at
/// IDA `0x889784`: the string plus the `shared_ptr` word at pair `+8`.
pub struct ToolbarMapEntry {
    pub name: String,
    pub toolbar: Option<SharedPtr<Toolbar>>,
}

// 0x888478 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS8_E

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<void * const,boost::shared_ptr<RBX::Button>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS8_E")]
// IDA 0x888478: `LDR R0, [R4,#0x18]` (`0x88847e`) loads the node's shared
// `pi_`; `sp_counted_base::release` when non-null (`0x888480`..`0x888484`);
// `operator delete(node)` (`0x88848e`: `B.W __ZdlPv`).
// was: conditional release + delete → drop the node; the `Arc` count drops
// with the `Option<SharedPtr<Button>>` field.
pub fn stub_888478(node: ButtonMapNode) {
    drop(node);
}

// 0x888494 — __ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E17static_getCreatorEv

#[doc(alias = "__ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E17static_getCreatorEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E17static_getCreatorEv")]
// IDA 0x888494: `ReleaseAssert` gate on `Creator::wasConstructed()`
// (Object.h:282, `isConstructed != 666` at `0x8884a8`..`0x8884f6`) with the
// `_debugHook` bypass (`0x8884c8`..`0x8884e8`); returns `&creatorPrivate`
// (`0x8884fa`..`0x888506`).
// was: static storage address → shared reference; construction runs in C2.
pub fn stub_888494() -> &'static ClassCreator {
    debug_assert!(
        TOOLBAR_CREATOR_STATE.load(Ordering::SeqCst) == CREATOR_CONSTRUCTED,
        "Creator::wasConstructed() file: include/Util/Object.h line: 282"
    );
    ensure_creator("Toolbar", &TOOLBAR_CREATOR_STATE);
    &TOOLBAR_CREATOR
}

// 0x888508 — __ZNK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7Creator12getClassNameEv

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7Creator12getClassNameEv")]
// IDA 0x888508: decompile failed; 42-insn disasm: `wasConstructed` gate
// (Object.h:236 at `0x888518`..`0x888568`), `boost::call_once(flag,
// callDoDeclare<sToolbar>)` (`0x888584`), tail-call
// `Name::doDeclare<sToolbar>` (`0x88858c`) returning the registered name.
// was: Name registry → the `sToolbar` literal; the once gate is preserved.
pub fn stub_888508() -> &'static str {
    toolbar_class_name()
}

// 0x888590 — __ZN3RBX4Name13callDoDeclareILZNS_8sToolbarEEEEvv

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sToolbarEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sToolbarEEEEvv")]
// IDA 0x888590: 1-insn thunk (`B.W` at `0x888590`) branching to `doDeclare`
// `0x888594`.
pub fn stub_888590() -> &'static str {
    stub_888594()
}

// 0x888594 — __ZN3RBX4Name9doDeclareILZNS_8sToolbarEEEERKS0_v

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sToolbarEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sToolbarEEEERKS0_v")]
// IDA 0x888594: `__cxa_guard_acquire` (`0x8885f0`), `Name::declare(sToolbar,
// 1)` (`0x888606`), `__cxa_guard_release` (`0x88861a`); SjLj
// register/unregister glue (`0x8885e4`/`0x88862e`); returns the registered
// cell (`0x88862a`); cleanup path aborts the guard and resumes (`0x88864a`
// ..`0x888668`).
// was: cxa guard + registry cell → `Once` + literal; unwind glue dropped.
pub fn stub_888594() -> &'static str {
    toolbar_class_name()
}

// 0x888674 — __ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E17static_getCreatorEv

#[doc(alias = "__ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E17static_getCreatorEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E17static_getCreatorEv")]
// IDA 0x888674: same 37-insn shape as `0x888494` for `sButton`
// (`FLog::Asserts` at `0x888684`, `666` compare at `0x88868c`, `_debugHook`
// bypass at `0x8886aa`, `&creatorPrivate` return at the tail).
// was: static storage address → shared reference; construction runs in C2.
pub fn stub_888674() -> &'static ClassCreator {
    debug_assert!(
        BUTTON_CREATOR_STATE.load(Ordering::SeqCst) == CREATOR_CONSTRUCTED,
        "Creator::wasConstructed() file: include/Util/Object.h line: 282"
    );
    ensure_creator("Button", &BUTTON_CREATOR_STATE);
    &BUTTON_CREATOR
}

// 0x8886e8 — __ZNK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7Creator12getClassNameEv

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7Creator12getClassNameEv")]
// IDA 0x8886e8: decompile failed; same 42-insn shape as `0x888508` for
// `sButton` (`wasConstructed` gate, `call_once` declare, tail-call
// `doDeclare<sButton>`).
// was: Name registry → the `sButton` literal; the once gate is preserved.
pub fn stub_8886e8() -> &'static str {
    button_class_name()
}

// 0x888770 — __ZN3RBX4Name13callDoDeclareILZNS_7sButtonEEEEvv

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sButtonEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sButtonEEEEvv")]
// IDA 0x888770: 1-insn thunk (`B.W` at `0x888770`) branching to `doDeclare`
// `0x888774`.
pub fn stub_888770() -> &'static str {
    stub_888774()
}

// 0x888774 — __ZN3RBX4Name9doDeclareILZNS_7sButtonEEEERKS0_v

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sButtonEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sButtonEEEERKS0_v")]
// IDA 0x888774: `__cxa_guard_acquire` (`0x8887d0`), `Name::declare(sButton,
// 1)` (`0x8887f6`), `__cxa_guard_release` (`0x8887fa`); same SjLj shape as
// `0x888594`.
// was: cxa guard + registry cell → `Once` + literal; unwind glue dropped.
pub fn stub_888774() -> &'static str {
    button_class_name()
}

// 0x888854 — __ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7CreatorD2Ev

#[doc(alias = "__ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7CreatorD2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7CreatorD2Ev")]
// IDA 0x888854: vtable restore to `off_12ACD24` (`0x888876`); `wasConstructed`
// gate (Object.h:255 at `0x888878`..`0x8888c8`) with the `_debugHook` bypass;
// `getCreators` + `Creator::getClassName` (`0x8888cc`..`0x8888de`);
// `creators.erase(name)` (`0x8888e6`); returns `this` (`0x8888ee`).
// was: vtable/erase → unregister from the creator map; `Arc` drops model the
// shared releases.
pub fn stub_888854() {
    teardown_creator("Plugin");
}

// 0x8888f0 — __ZNK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7Creator12getClassNameEv

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7Creator12getClassNameEv")]
// IDA 0x8888f0: decompile failed; same 42-insn shape as `0x888508` for
// `sPlugin` (`wasConstructed` gate, `call_once` declare, tail-call
// `doDeclare<sPlugin>`).
// was: Name registry → the `sPlugin` literal; the once gate is preserved.
pub fn stub_8888f0() -> &'static str {
    plugin_class_name()
}

// 0x888978 — __ZNK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7Creator6createEv

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7Creator6createEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7Creator6createEv")]
// IDA 0x888978: `wasConstructed` gate (Object.h:231 at `0x8889c0`..`0x888a16`)
// with the `_debugHook` bypass; `Creatable<Instance>::create<Plugin>`
// (`0x888a30`); null check + `+32` Instance→Plugin adjustment (`0x888a42`
// ..`0x888a44`); `shared_count` copy into the sret slot (`0x888a52`); temp
// release (`0x888a58`..`0x888a60`).
// was: factory create + sret shared copy → fresh `Plugin` in an `Arc`; the
// `+32` multi-inheritance adjustment is a no-op in the flat model.
// // BUG: the factory's exact vtable/instance header words are unrecovered —
// the observable half is a fresh default plugin with no DataModel link.
pub fn stub_888978() -> SharedPtr<Plugin> {
    debug_assert!(
        PLUGIN_CREATOR_STATE.load(Ordering::SeqCst) == CREATOR_CONSTRUCTED,
        "wasConstructed() file: include/Util/Object.h line: 231"
    );
    ensure_creator("Plugin", &PLUGIN_CREATOR_STATE);
    SharedPtr::new(Plugin {
        data_model: Mutex::new(None),
        active: AtomicBool::new(false),
        exclusive: AtomicBool::new(false),
        mouse: Mutex::new(None),
    })
}

// 0x888abc — __ZN3RBX4Name13callDoDeclareILZNS_7sPluginEEEEvv

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sPluginEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sPluginEEEEvv")]
// IDA 0x888abc: 1-insn thunk (`B.W` at `0x888abc`) branching to `doDeclare`
// `0x888ac0`.
pub fn stub_888abc() -> &'static str {
    stub_888ac0()
}

// 0x888ac0 — __ZN3RBX4Name9doDeclareILZNS_7sPluginEEEERKS0_v

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sPluginEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sPluginEEEERKS0_v")]
// IDA 0x888ac0: `__cxa_guard_acquire` (`0x888b1c`), `Name::declare(sPlugin,
// 1)` (`0x888b42`), `__cxa_guard_release` (`0x888b46`); same SjLj shape as
// `0x888594`.
// was: cxa guard + registry cell → `Once` + literal; unwind glue dropped.
pub fn stub_888ac0() -> &'static str {
    plugin_class_name()
}

// 0x888ba0 — __ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7CreatorC2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E7CreatorC2Ev")]
// IDA 0x888ba0: vtable store `off_12ACD24` (`0x888bd6`); `call_once` declare
// (`0x888bd8`); `doDeclare<sPlugin>` (`0x888bee`); `getCreators` ordered-find
// asserting absence (Object.h:244 at `0x888bfa`..`0x888c7e`);
// `!wasConstructed` gate (Object.h:245 at `0x888c88`..`0x888cc4`);
// `operator[name] = this` (`0x888cd6`..`0x888d00`); `isConstructed = 666`
// (`0x888d06`); post-insert find asserts (`0x888d12`..`0x888d6e`) and the
// `wasConstructed` tail gate (Object.h:251 at `0x888d70`..`0x888da8`).
// was: static-init registration → `ensure_creator`; see its `// BUG` note.
pub fn stub_888ba0() {
    ensure_creator("Plugin", &PLUGIN_CREATOR_STATE);
}

// 0x888de4 — __ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E17static_getCreatorEv

#[doc(alias = "__ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E17static_getCreatorEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_6PluginENS_8InstanceELZNS_7sPluginEES2_E17static_getCreatorEv")]
// IDA 0x888de4: same 37-insn shape as `0x888494` for `sPlugin`
// (`FLog::Asserts` at `0x888df4`, `666` compare at `0x888dfc`, `_debugHook`
// bypass at `0x888e1a`, `&creatorPrivate` return at the tail).
// was: static storage address → shared reference; construction runs in C2.
pub fn stub_888de4() -> &'static ClassCreator {
    debug_assert!(
        PLUGIN_CREATOR_STATE.load(Ordering::SeqCst) == CREATOR_CONSTRUCTED,
        "Creator::wasConstructed() file: include/Util/Object.h line: 282"
    );
    ensure_creator("Plugin", &PLUGIN_CREATOR_STATE);
    &PLUGIN_CREATOR
}

// 0x888e58 — __ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7CreatorD2Ev

#[doc(alias = "__ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7CreatorD2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7CreatorD2Ev")]
// IDA 0x888e58: same D2 shape as `0x888854` for `sToolbar` (vtable restore at
// `0x888e7a`, `wasConstructed` gate, `getClassName` at `0x888ee2`,
// `creators.erase` at `0x888eea`).
// was: vtable/erase → unregister from the creator map.
pub fn stub_888e58() {
    teardown_creator("Toolbar");
}

// 0x888ef4 — __ZNK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7Creator6createEv

#[doc(alias = "__ZNK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7Creator6createEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7Creator6createEv")]
// IDA 0x888ef4: same 110-insn `create` shape as `0x888978` for `sToolbar`
// (`wasConstructed` gate, `Creatable<Instance>::create<Toolbar>`, `+32`
// adjustment, `shared_count` copy + temp release).
// was: factory create + sret shared copy → fresh `Toolbar` in an `Arc`.
pub fn stub_888ef4() -> SharedPtr<Toolbar> {
    debug_assert!(
        TOOLBAR_CREATOR_STATE.load(Ordering::SeqCst) == CREATOR_CONSTRUCTED,
        "wasConstructed() file: include/Util/Object.h line: 231"
    );
    ensure_creator("Toolbar", &TOOLBAR_CREATOR_STATE);
    SharedPtr::new(Toolbar::default())
}

// 0x889038 — __ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7CreatorC2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7ToolbarENS_8InstanceELZNS_8sToolbarEES2_E7CreatorC2Ev")]
// IDA 0x889038: same 184-insn C2 shape as `0x888ba0` for `sToolbar`
// (`call_once` declare, `doDeclare`, ordered-find absence proof,
// `!wasConstructed` gate, `operator[name] = this`, `isConstructed = 666`).
// was: static-init registration → `ensure_creator`; see its `// BUG` note.
pub fn stub_889038() {
    ensure_creator("Toolbar", &TOOLBAR_CREATOR_STATE);
}

// 0x88927c — __ZN3RBX4Name13callDoDeclareILZNS_14sPluginManagerEEEEvv

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sPluginManagerEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sPluginManagerEEEEvv")]
// IDA 0x88927c: 1-insn thunk (`B.W` at `0x88927c`) branching to `doDeclare`
// `0x889280`.
pub fn stub_88927c() -> &'static str {
    stub_889280()
}

// 0x889280 — __ZN3RBX4Name9doDeclareILZNS_14sPluginManagerEEEERKS0_v

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sPluginManagerEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sPluginManagerEEEERKS0_v")]
// IDA 0x889280: `__cxa_guard_acquire` (`0x8892dc`),
// `Name::declare(sPluginManager, 1)` (`0x889302`), `__cxa_guard_release`
// (`0x889306`); returns the registered cell (`0x889334`); same SjLj shape as
// `0x888594`.
// was: cxa guard + registry cell → `Once` + literal; unwind glue dropped.
pub fn stub_889280() -> &'static str {
    plugin_manager_class_name_476()
}

// 0x889360 — __ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7CreatorD2Ev

#[doc(alias = "__ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7CreatorD2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7CreatorD2Ev")]
// IDA 0x889360: same D2 shape as `0x888854` for `sButton` (vtable restore,
// `wasConstructed` gate, `getClassName` at `0x8893ea`, `creators.erase` at
// `0x8893f2`).
// was: vtable/erase → unregister from the creator map.
pub fn stub_889360() {
    teardown_creator("Button");
}

// 0x8893fc — __ZNK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7Creator6createEv

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7Creator6createEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7Creator6createEv")]
// IDA 0x8893fc: same 110-insn `create` shape as `0x888978` for `sButton`
// (`wasConstructed` gate, `Creatable<Instance>::create<Button>`, `+32`
// adjustment, `shared_count` copy + temp release).
// was: factory create + sret shared copy → fresh `Button` in an `Arc`.
pub fn stub_8893fc() -> SharedPtr<Button> {
    debug_assert!(
        BUTTON_CREATOR_STATE.load(Ordering::SeqCst) == CREATOR_CONSTRUCTED,
        "wasConstructed() file: include/Util/Object.h line: 231"
    );
    ensure_creator("Button", &BUTTON_CREATOR_STATE);
    SharedPtr::new(Button::default())
}

// 0x889540 — __ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7CreatorC2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_6ButtonENS_8InstanceELZNS_7sButtonEES2_E7CreatorC2Ev")]
// IDA 0x889540: same 184-insn C2 shape as `0x888ba0` for `sButton`
// (`call_once` declare, `doDeclare`, ordered-find absence proof,
// `!wasConstructed` gate, `operator[name] = this`, `isConstructed = 666`).
// was: static-init registration → `ensure_creator`; see its `// BUG` note.
pub fn stub_889540() {
    ensure_creator("Button", &BUTTON_CREATOR_STATE);
}

// 0x889784 — __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEEE7destroyEPS8_

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>::destroy(std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEEE7destroyEPS8_")]
// IDA 0x889784: `LDR R4, [R2,#8]` (`0x8897b4`) loads the pair's shared `pi_`;
// `sp_counted_base::release` when non-null (`0x8897d0`..`0x8897d8`);
// `std::string::~string` (`0x8897de`); SjLj register/unregister glue
// (`0x8897cc`/`0x8897e2`) with the resume path re-running `~string`
// (`0x88980e`..`0x88981c`).
// was: conditional release + string dtor → drop the entry; the `Arc` count
// drops with the `Option<SharedPtr<Toolbar>>` field.
pub fn stub_889784(entry: ToolbarMapEntry) {
    drop(entry);
}

// 0x889828 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// IDA 0x889828: null check (`0x889832`); post-order walk — recurse into the
// left child (`0x88983a`), `destroy(node)` = shared release + `~string`
// (`0x889844`, i.e. `0x889784`), `operator delete` (`0x88984a`), step to the
// right sibling (`0x88984e`) until exhausted (`0x889852`).
// was: Rb-tree node recursion + per-node shared release → `HashMap::clear`
// (each `Arc<Toolbar>` drops with its entry, same release count).
pub fn stub_889828(toolbars: &Mutex<HashMap<String, SharedPtr<Toolbar>>>) {
    toolbars.lock().clear();
}

// 0x889858 — __ZNSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEEC2ERS0_RKS5_

#[doc(alias = "std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>::pair(std::string const&,boost::shared_ptr<RBX::Toolbar> const&)")]
#[doc(alias = "__ZNSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEEC2ERS0_RKS5_")]
// IDA 0x889858: 66 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889858() {
}

// 0x889914 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
// IDA 0x889914: 94 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889914() {
}

// 0x889a00 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")]
// IDA 0x889a00: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889a00() {
}

// 0x889a50 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_")]
// IDA 0x889a50: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889a50() {
}

// 0x889ad4 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_")]
// IDA 0x889ad4: 96 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889ad4() {
}

// 0x889bdc — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::lower_bound(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_")]
// IDA 0x889bdc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889bdc() {
}

// 0x889c0c — __ZN5boost10shared_ptrIN3RBX7ToolbarEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_

#[doc(alias = "boost::shared_ptr<RBX::Toolbar>::shared_ptr<RBX::Toolbar,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Toolbar *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7ToolbarEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// IDA 0x889c0c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889c0c() {
}

// 0x889cd4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7ToolbarES6_EEvPKNS_10shared_ptrIT_EEPT0_

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Toolbar,RBX::Toolbar>(boost::shared_ptr<RBX::Toolbar> const*,RBX::Toolbar *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7ToolbarES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0x889cd4: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889cd4() {
}

// 0x889dbc — __ZN5boost6detail12shared_countC2IPN3RBX7ToolbarENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Toolbar *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Toolbar *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX7ToolbarENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// IDA 0x889dbc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889dbc() {
}

// 0x889ec4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7ToolbarENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Toolbar *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7ToolbarENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0x889ec4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_889ec4() {
}

// 0x889ec8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7ToolbarENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Toolbar *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7ToolbarENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0x889ec8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_889ec8() {
}

// 0x889ecc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7ToolbarENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Toolbar *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7ToolbarENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0x889ecc: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889ecc() {
}

// 0x889eec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7ToolbarENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Toolbar *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7ToolbarENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0x889eec: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889eec() {
}

// 0x889f04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7ToolbarENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Toolbar *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7ToolbarENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0x889f04: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889f04() {
}

// 0x889f08 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::find(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_")]
// IDA 0x889f08: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889f08() {
}

// 0x889f58 — __ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EEC2ERKS6_

#[doc(alias = "std::list<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>>::list(std::list<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>> const&)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EEC2ERKS6_")]
// IDA 0x889f58: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_889f58() {
}

// 0x88a020 — __ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE22_M_initialize_dispatchISt20_List_const_iteratorIS4_EEEvT_SA_St12__false_type
// type: int __fastcall(int)
#[doc(alias = "void std::list<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>>::_M_initialize_dispatch<std::_List_const_iterator<boost::shared_ptr<RBX::Plugin>>>(std::_List_const_iterator<boost::shared_ptr<RBX::Plugin>>,std::_List_const_iterator<boost::shared_ptr<RBX::Plugin>>,std::__false_type)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE22_M_initialize_dispatchISt20_List_const_iteratorIS4_EEEvT_SA_St12__false_type")]
// IDA 0x88a020: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a020() {
}

// 0x88a044 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE8_M_clearEv

#[doc(alias = "std::_List_base<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>>::_M_clear(void)")]
#[doc(alias = "__ZNSt10_List_baseIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE8_M_clearEv")]
// IDA 0x88a044: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a044() {
}

// 0x88a06c — __ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE14_M_create_nodeERKS4_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::list<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>>::_M_create_node(boost::shared_ptr<RBX::Plugin> const&)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE14_M_create_nodeERKS4_")]
// IDA 0x88a06c: 81 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a06c() {
}

// 0x88a150 — __ZNSt8_Rb_treeIPN3RBX9DataModelESt4pairIKS2_NS0_13PluginManager14StateDataEntryEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E

#[doc(alias = "std::_Rb_tree<RBX::DataModel *,std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>,std::_Select1st<std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>>,std::less<RBX::DataModel *>,std::allocator<std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX9DataModelESt4pairIKS2_NS0_13PluginManager14StateDataEntryEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E")]
// IDA 0x88a150: 76 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a150() {
}

// 0x88a228 — __ZNSt8_Rb_treeIPN3RBX9DataModelESt4pairIKS2_NS0_13PluginManager14StateDataEntryEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E

#[doc(alias = "std::_Rb_tree<RBX::DataModel *,std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>,std::_Select1st<std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>>,std::less<RBX::DataModel *>,std::allocator<std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX9DataModelESt4pairIKS2_NS0_13PluginManager14StateDataEntryEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// IDA 0x88a228: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a228() {
}

// 0x88a250 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EEC2ERKSD_

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_Rb_tree(std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EEC2ERKSD_")]
// IDA 0x88a250: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a250() {
}

// 0x88a294 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE7_M_copyEPKSt13_Rb_tree_nodeIS7_EPSF_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>> const*,std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>*)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE7_M_copyEPKSt13_Rb_tree_nodeIS7_EPSF_")]
// IDA 0x88a294: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a294() {
}

// 0x88a3e8 — __ZNSt8_Rb_treeIPN3RBX9DataModelESt4pairIKS2_NS0_13PluginManager14StateDataEntryEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE16_M_insert_uniqueERKS7_

#[doc(alias = "std::_Rb_tree<RBX::DataModel *,std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>,std::_Select1st<std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>>,std::less<RBX::DataModel *>,std::allocator<std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>>>::_M_insert_unique(std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX9DataModelESt4pairIKS2_NS0_13PluginManager14StateDataEntryEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE16_M_insert_uniqueERKS7_")]
// IDA 0x88a3e8: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a3e8() {
}

// 0x88a450 — __ZNSt8_Rb_treeIPN3RBX9DataModelESt4pairIKS2_NS0_13PluginManager14StateDataEntryEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_

#[doc(alias = "std::_Rb_tree<RBX::DataModel *,std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>,std::_Select1st<std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>>,std::less<RBX::DataModel *>,std::allocator<std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX9DataModelESt4pairIKS2_NS0_13PluginManager14StateDataEntryEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")]
// IDA 0x88a450: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a450() {
}

// 0x88a49c — __ZNSt8_Rb_treeIPN3RBX9DataModelESt4pairIKS2_NS0_13PluginManager14StateDataEntryEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE14_M_create_nodeERKS7_

#[doc(alias = "std::_Rb_tree<RBX::DataModel *,std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>,std::_Select1st<std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>>,std::less<RBX::DataModel *>,std::allocator<std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry>>>::_M_create_node(std::pair<RBX::DataModel * const,RBX::PluginManager::StateDataEntry> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX9DataModelESt4pairIKS2_NS0_13PluginManager14StateDataEntryEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE14_M_create_nodeERKS7_")]
// IDA 0x88a49c: 107 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a49c() {
}

// 0x88a5c0 — __ZN5boost10shared_ptrIN3RBX6PluginEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_

#[doc(alias = "boost::shared_ptr<RBX::Plugin>::shared_ptr<RBX::Plugin,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Plugin *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6PluginEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// IDA 0x88a5c0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a5c0() {
}

// 0x88a688 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6PluginES6_EEvPKNS_10shared_ptrIT_EEPT0_

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Plugin,RBX::Plugin>(boost::shared_ptr<RBX::Plugin> const*,RBX::Plugin *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6PluginES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0x88a688: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a688() {
}

// 0x88a770 — __ZN5boost6detail12shared_countC2IPN3RBX6PluginENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Plugin *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Plugin *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX6PluginENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// IDA 0x88a770: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a770() {
}

// 0x88a878 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6PluginENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Plugin *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6PluginENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0x88a878: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_88a878() {
}

// 0x88a87c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6PluginENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Plugin *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6PluginENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0x88a87c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_88a87c() {
}

// 0x88a880 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6PluginENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Plugin *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6PluginENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0x88a880: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a880() {
}

// 0x88a8a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6PluginENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Plugin *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6PluginENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0x88a8a0: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a8a0() {
}

// 0x88a8b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6PluginENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Plugin *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6PluginENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0x88a8b8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88a8b8() {
}

// 0x88a8bc — __ZN3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x88a8bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_88a8bc() {
}

// 0x88a8c0 — __ZN3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x88a8c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88a8c0() {
}

// 0x88a960 — __ZThn32_N3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x88a960: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88a960() {
}

// 0x88a968 — __ZThn32_N3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x88a968: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88a968() {
}

// 0x88aa0c — __ZThn36_N3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x88aa0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88aa0c() {
}

// 0x88aa14 — __ZThn36_N3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13PluginManagerELZNS_14sPluginManagerEENS_17NonFactoryProductINS_8InstanceELZNS_14sPluginManagerEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x88aa14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88aa14() {
}

// 0x88aab8 — __ZN5boost10shared_ptrIN3RBX11PluginMouseEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_

#[doc(alias = "boost::shared_ptr<RBX::PluginMouse>::shared_ptr<RBX::PluginMouse,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginMouse *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11PluginMouseEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// IDA 0x88aab8: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88aab8() {
}

// 0x88ab80 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11PluginMouseES6_EEvPKNS_10shared_ptrIT_EEPT0_

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PluginMouse,RBX::PluginMouse>(boost::shared_ptr<RBX::PluginMouse> const*,RBX::PluginMouse *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11PluginMouseES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0x88ab80: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88ab80() {
}

// 0x88ac68 — __ZN5boost6detail12shared_countC2IPN3RBX11PluginMouseENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PluginMouse *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginMouse *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX11PluginMouseENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// IDA 0x88ac68: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88ac68() {
}

// 0x88ad70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PluginMouseENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginMouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PluginMouseENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0x88ad70: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_88ad70() {
}

// 0x88ad74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PluginMouseENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginMouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PluginMouseENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0x88ad74: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_88ad74() {
}

// 0x88ad78 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PluginMouseENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginMouse *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PluginMouseENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0x88ad78: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88ad78() {
}

// 0x88ad98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PluginMouseENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginMouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PluginMouseENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0x88ad98: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88ad98() {
}

// 0x88adb0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PluginMouseENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginMouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11PluginMouseENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0x88adb0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88adb0() {
}

// 0x88adb4 — __ZN3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x88adb4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_88adb4() {
}

// 0x88adb8 — __ZN3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x88adb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88adb8() {
}

// 0x88ae58 — __ZThn32_N3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x88ae58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88ae58() {
}

// 0x88ae60 — __ZThn32_N3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x88ae60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88ae60() {
}

// 0x88af04 — __ZThn36_N3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x88af04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88af04() {
}

// 0x88af0c — __ZThn36_N3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6PluginELZNS_7sPluginEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sPluginEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x88af0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88af0c() {
}

// 0x88afb0 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::pair<void * const,boost::shared_ptr<RBX::Button>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// IDA 0x88afb0: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88afb0() {
}

// 0x88b064 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<void * const,boost::shared_ptr<RBX::Button>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// IDA 0x88b064: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88b064() {
}

// 0x88b0b0 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_insert_unique(std::pair<void * const,boost::shared_ptr<RBX::Button>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// IDA 0x88b0b0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88b0b0() {
}

// 0x88b118 — __ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE14_M_create_nodeERKS8_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_create_node(std::pair<void * const,boost::shared_ptr<RBX::Button>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE14_M_create_nodeERKS8_")]
// IDA 0x88b118: 86 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88b118() {
}

// 0x88b208 — __ZN5boost10shared_ptrIN3RBX6ButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_

#[doc(alias = "boost::shared_ptr<RBX::Button>::shared_ptr<RBX::Button,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Button *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6ButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// IDA 0x88b208: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88b208() {
}

// 0x88b2d0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6ButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Button,RBX::Button>(boost::shared_ptr<RBX::Button> const*,RBX::Button *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6ButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0x88b2d0: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88b2d0() {
}

// 0x88b3b8 — __ZN5boost6detail12shared_countC2IPN3RBX6ButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Button *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Button *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX6ButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// IDA 0x88b3b8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88b3b8() {
}

// 0x88b4c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Button *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0x88b4c0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_88b4c0() {
}

// 0x88b4c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Button *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0x88b4c4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_88b4c4() {
}

// 0x88b4c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Button *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0x88b4c8: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88b4c8() {
}

// 0x88b4e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Button *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0x88b4e8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88b4e8() {
}

// 0x88b500 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Button *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0x88b500: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88b500() {
}

// 0x88b504 — __ZN3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x88b504: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_88b504() {
}

// 0x88b508 — __ZN3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x88b508: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88b508() {
}

// 0x88b5a8 — __ZThn32_N3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x88b5a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88b5a8() {
}

// 0x88b5b0 — __ZThn32_N3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x88b5b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88b5b0() {
}

// 0x88b654 — __ZThn36_N3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x88b654: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88b654() {
}

// 0x88b65c — __ZThn36_N3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7ToolbarELZNS_8sToolbarEENS_14FactoryProductIS2_NS_8InstanceELZNS_8sToolbarEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x88b65c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88b65c() {
}

// 0x88b700 — __ZN3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x88b700: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_88b700() {
}

// 0x88b704 — __ZN3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x88b704: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88b704() {
}
