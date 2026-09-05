// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace complete (10215/10215) — fallback EA-sorted asc not yet in datamodel
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0xf1f348..0xf1fa5c | next 120 uncovered after 0xf1f33c (shard_282)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias
// Shard: datamodel_shard_283 EA-sorted ascending

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use std::collections::HashMap;
use std::sync::OnceLock;
use parking_lot::Mutex;
use crate::generated_05::{GenericSlotWrapper, PropertyDescriptor};

/// Rust model of one `MemberDescriptorContainer<T>::staticData` function-static
/// container (IDA `0xf1f528`/`0xf1f558`/`0xf1f588`/`0xf1f5b8`/`0xf1f5d0`): the
/// registered member-container list plus the `const char* -> descriptor*` name
/// map. Bucket/node layout collapses into `Vec` + `HashMap`, mirroring the
/// `instance::stub_0x3d8a48` unordered precedent.
pub struct DescriptorStore {
    pub members: Vec<*const ()>,
    pub by_name: HashMap<String, *const ()>,
}

/// Pointers are model-space descriptor links owned by the static registry;
/// same `Send`/`Sync` contract as `data_model::ObjcDmBind`.
unsafe impl Send for DescriptorStore {}
unsafe impl Sync for DescriptorStore {}

impl DescriptorStore {
    pub fn new() -> Self {
        Self { members: Vec::new(), by_name: HashMap::new() }
    }

    pub fn register(&mut self, name: &str, member: *const ()) {
        self.members.push(member);
        self.by_name.insert(name.to_owned(), member);
    }

    pub fn find(&self, name: &str) -> Option<*const ()> {
        self.by_name.get(name).copied()
    }
}

/// Rust model of one `boost::unordered_map<const char*, Descriptor*, ...>`
/// name table (IDA `0xf1f4ec`..`0xf1f624` family): key string plus descriptor
/// pointer per entry; buckets collapse into the map itself.
#[derive(Default)]
pub struct DescriptorTable {
    pub entries: HashMap<String, *const ()>,
}

impl DescriptorTable {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    /// `rehash_impl(n)` growth (IDA `0xf1f4ec` etc.): skips when capacity
    /// already fits, else grows — `reserve` is the same growth, mirroring
    /// `instance::stub_0x3d8c44`.
    pub fn rehash_for_insert(&mut self, additional: usize) {
        self.entries.reserve(additional);
    }

    /// `create_buckets(n)` (IDA `0xf1f4f8` etc.): lays out the bucket array;
    /// over an existing table `reserve` is the same allocation without
    /// dropping contents, mirroring `instance::stub_0x3d8c98`.
    pub fn create_buckets(&mut self, buckets: usize) {
        self.entries.reserve(buckets);
    }

    pub fn insert(&mut self, name: &str, desc: *const ()) {
        self.entries.insert(name.to_owned(), desc);
    }

    pub fn find(&self, name: &str) -> Option<*const ()> {
        self.entries.get(name).copied()
    }
}

fn static_store(slot: &'static OnceLock<Mutex<DescriptorStore>>) -> &'static Mutex<DescriptorStore> {
    slot.get_or_init(|| Mutex::new(DescriptorStore::new()))
}

static CALLBACK_MEMBER_STORE: OnceLock<Mutex<DescriptorStore>> = OnceLock::new();
static YIELD_FUNCTION_MEMBER_STORE: OnceLock<Mutex<DescriptorStore>> = OnceLock::new();
static EVENT_MEMBER_STORE: OnceLock<Mutex<DescriptorStore>> = OnceLock::new();
static FUNCTION_MEMBER_STORE: OnceLock<Mutex<DescriptorStore>> = OnceLock::new();
static PROPERTY_MEMBER_STORE: OnceLock<Mutex<DescriptorStore>> = OnceLock::new();
use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::generated_05::{FunctorOp, Variant};
use crate::data_model::{LuaWeakThreadRef, WeakThreadStringBind};
use crate::instance::LuaState;
use crate::generated_datamodel_shard_278::{Name, NameMap};
// `DescriptorTable` / `DescriptorStore` are defined in HALF 1 SHARED above (same module).

/// Rust model of `G3D::Vector3int16` (IDA `0xf1f66c`): cell triple.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Vector3i16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// Rust model of `G3D::Vector2int16` (IDA `0xf1f678`).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Vector2i16 {
    pub x: i16,
    pub y: i16,
}

/// Rust model of `G3D::Vector3` (IDA `0xf1f684`).
#[derive(Clone, Copy, Default)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Rust model of `G3D::Vector2` (IDA `0xf1f690`).
#[derive(Clone, Copy, Default)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

/// Rust model of `G3D::CoordinateFrame` (IDA `0xf1f69c`): position plus rows.
#[derive(Clone, Copy, Default)]
pub struct CoordFrame {
    pub pos: Vector3f,
    pub rot: [[f32; 3]; 3],
}

fn v3i16_text(v: &Vector3i16) -> String {
    format!("{}, {}, {}", v.x, v.y, v.z)
}

fn v2i16_text(v: &Vector2i16) -> String {
    format!("{}, {}", v.x, v.y)
}

fn v3f_text(v: &Vector3f) -> String {
    format!("{}, {}, {}", v.x, v.y, v.z)
}

fn v2f_text(v: &Vector2f) -> String {
    format!("{}, {}", v.x, v.y)
}

fn coord_frame_text(v: &CoordFrame) -> String {
    format!(
        "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        v.pos.x, v.pos.y, v.pos.z,
        v.rot[0][0], v.rot[0][1], v.rot[0][2],
        v.rot[1][0], v.rot[1][1], v.rot[1][2],
        v.rot[2][0], v.rot[2][1], v.rot[2][2],
    )
}

/// Pushes a Rust string as the single Lua return value; the
/// `StringConverter<T>::convertToString` + `lua_pushlstring` + `return 1` tail
/// recovered from the real `Vector3int16` 2-arg body (IDA `0x278574`).
/// SAFETY: `state` must be a live Lua state.
unsafe fn push_tostring_result(state: *mut LuaState, text: &str) -> i32 {
    crate::instance::lua_ffi::lua_pushlstring(
        state,
        text.as_ptr() as *const core::ffi::c_char,
        text.len(),
    );
    1
}

/// Bridge class name used by the 1-arg `on_tostring` checkudata (twin IDA
/// `0x277cf8` reads `Bridge<T>::className[0]`); the registration name follows
/// the `registerClass` convention.
static COORD_FRAME_CLASS: &[u8] = b"CoordinateFrame\0";

fn coord_frame_tostring(value: &CoordFrame, state: *mut LuaState) -> i32 {
    // SAFETY: forwarded from `stub_f1f69c`; `state` is live and `value` outlives the push.
    unsafe { push_tostring_result(state, &coord_frame_text(value)) }
}

/// Rust model of `YieldFunctionStateObject` (IDA `0xf1f6c0`): the yielding
/// call target; layout unmodeled — only the retained link travels.
#[derive(Default)]
pub struct YieldStateObject {
    _opaque: (),
}

/// Rust model of `RBX::Lua::detail::LiveThreadRef` (IDA `0xf1fa5c`): the live
/// thread handle released by `intrusive_ptr_release`.
#[derive(Default)]
pub struct LiveThreadRef {
    _opaque: (),
}

/// Rust model of `RBX::Lua::YieldingThreads::WaitingThread` (IDA `0xf1fa20`):
/// the retained weak thread plus the opaque resume payload.
#[derive(Clone, Default)]
pub struct WaitingThread {
    pub thread: Option<SharedPtr<LuaWeakThreadRef>>,
}

/// Nullable `function<void(intrusive_ptr<WeakThreadRef>, string)>` holding the
/// thread/script bind (IDA `0xf1f6a8`); empty is the cleared state.
#[derive(Clone, Default)]
pub struct ThreadStringCallback {
    pub inner: Option<WeakThreadStringBind>,
}

impl ThreadStringCallback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }
}

/// Bound member of `YieldFunctionStateObject::resume(Variant)` (IDA `0xf1f6c0`).
pub type YieldVariantFn = fn(&YieldStateObject, &Variant);

/// Rust model of `bind_t<void, mf1<void, YieldFunctionStateObject, Variant>,
/// list2<value<shared_ptr<YieldFunctionStateObject>>, arg<1>>>` (IDA `0xf1f6c0`).
#[derive(Clone)]
pub struct YieldVariantBind {
    pub target: SharedPtr<YieldStateObject>,
    pub invoke: YieldVariantFn,
}

/// Nullable holder for the yield-variant bind.
#[derive(Clone, Default)]
pub struct YieldVariantCallback {
    pub inner: Option<YieldVariantBind>,
}

impl YieldVariantCallback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }
}

/// Mangled type name compared by the check path (same `strcmp` role as
/// `generated_05::BIND_PREDICATE_TYPE_NAME`).
pub const YIELD_VARIANT_BIND_TYPE: &str =
    "N5boost3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEE";

/// Rust model of `bind_t<shared_ptr<Tuple>, Tuple(*)(WeakFunctionRef,
/// shared_ptr<const Tuple>, intrusive_ptr<WeakThreadRef>),
/// list3<value<WeakFunctionRef>, arg<1>, value<intrusive_ptr<WeakThreadRef>>>>`
/// (IDA `0xf1f6cc`): the weak-function identity plus the retained thread; the
/// `Tuple` argument arrives as `arg<1>` at call time, never stored.
#[derive(Clone, Default)]
pub struct TupleResumeBind {
    pub weak_fn: u32,
    pub thread: Option<SharedPtr<LuaWeakThreadRef>>,
}

/// Nullable holder for the tuple-resume bind.
#[derive(Clone, Default)]
pub struct TupleResumeCallback {
    pub inner: Option<TupleResumeBind>,
}

impl TupleResumeCallback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }
}

pub const TUPLE_RESUME_BIND_TYPE: &str =
    "N5boost3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEE";

/// Rust model of `bind_t<void, mf2<void, ScriptContext, WeakThreadRef,
/// lua_State*>, list3<value<ScriptContext*>, value<WeakThreadRef>, arg<1>>>`
/// (IDA `0xf1f900`): bound context plus retained thread; `lua_State*` arrives
/// as `arg<1>` at call time.
#[derive(Clone, Default)]
pub struct ScriptContextResumeBind {
    pub context: *const (),
    pub thread: Option<SharedPtr<LuaWeakThreadRef>>,
}

/// Nullable holder for the ScriptContext-resume bind.
#[derive(Clone, Default)]
pub struct ScriptContextResumeCallback {
    pub inner: Option<ScriptContextResumeBind>,
}

impl ScriptContextResumeCallback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }
}

pub const SCRIPT_CONTEXT_RESUME_BIND_TYPE: &str =
    "N5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEE";

/// Rust model of `bind_t<void, void(*)(lua_State*, function<...>),
/// list2<arg<1>, value<function<...>>>>` (IDA `0xf1f90c`/`0xf1f918`): the
/// stored Lua callback; `lua_State*` arrives as `arg<1>` at call time.
/// The inner `boost::function` payload has no owned Rust state, so the code
/// pointer plus presence bit carry the clone/destroy semantics.
#[derive(Clone, Copy, Default)]
pub struct LuaStateBind {
    pub handler: *const (),
}

/// Nullable holder for the lua_State bind.
#[derive(Clone, Copy, Default)]
pub struct LuaStateCallback {
    pub inner: Option<LuaStateBind>,
}

impl LuaStateCallback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }
}

pub const LUA_STATE_BIND_TYPE: &str =
    "N5boost3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEE";

/// Rust model of `bind_t<void, void(*)(lua_State*,
/// function<void(shared_ptr<const Tuple>)>), list2<arg<1>, value<...>>>`
/// (IDA `0xf1f918`): same shape as `LuaStateBind`, distinct instantiation.
#[derive(Clone, Copy, Default)]
pub struct LuaTupleBind {
    pub handler: *const (),
}

/// Nullable holder for the lua_State/tuple bind.
#[derive(Clone, Copy, Default)]
pub struct LuaTupleCallback {
    pub inner: Option<LuaTupleBind>,
}

impl LuaTupleCallback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }
}

pub const LUA_TUPLE_BIND_TYPE: &str =
    "N5boost3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEE";

/// Rust model of `RBX::Script` source access (IDA `0xf1f798`): the script text
/// read by the bound `std::string (Script::*)()` member.
#[derive(Clone, Default)]
pub struct ScriptSource {
    pub source: String,
}

/// Rust model of `RBX::ScriptContext` yielded-result access (IDA `0xf1f9c0`):
/// the retained result vector returned by the bound member.
#[derive(Clone, Default)]
pub struct ScriptContextState {
    pub yielded: SharedPtr<Vec<Variant>>,
}

/// Rust model of `lua_Debug` behind `LuaProfiler::hookCall` (IDA `0xf1f7a4`):
/// the hook event plus current line.
#[derive(Clone, Copy, Default)]
pub struct LuaDebug {
    pub event: i32,
    pub line: i32,
}

/// Last profiler hook tap; models the profiler's observable event sink.
static LAST_HOOK_EVENT: Mutex<Option<(i32, i32)>> = Mutex::new(None);

/// Rust model of `GlobalAdvancedSettingsItem<LuaSettings, sLuaSettings>`
/// (IDA `0xf1f6f0`): deleting-destructor target; drop is compiler-managed.
pub struct LuaSettingsItem {
    pub enabled: bool,
}

static LUA_SETTINGS_ITEM_DROPS: AtomicUsize = AtomicUsize::new(0);

impl Drop for LuaSettingsItem {
    fn drop(&mut self) {
        LUA_SETTINGS_ITEM_DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

/// Rust model of `RBX::Name::doDeclare<sLuaSettings>` storage (IDA `0xf1f984`):
/// the once-declared interned name; `__cxa_guard` init becomes `LazyLock`,
/// mirroring `generated_190::stub_0xf1dc`.
static LUA_SETTINGS_NAME_DECL: LazyLock<String> = LazyLock::new(|| "LuaSettings".to_owned());

/// Owned-name store backing `Name::declare` pointers (IDA `0xf1f978`).
static LUA_DECLARE_STORE: OnceLock<Mutex<Vec<Box<Name>>>> = OnceLock::new();

// 0xf1f348 — __ZNK10RobloxView9RenderJob14getMetricValueERKSs$shim
#[doc(alias = "__ZNK10RobloxView9RenderJob14getMetricValueERKSs$shim")]
#[doc(alias = "__ZNK10RobloxView9RenderJob14getMetricValueERKSs$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f348 as stub_f1f348;

// 0xf1f354 — __ZNSt9exceptionD2Ev$shim
// type: void __cdecl(std::exception *__hidden this)
#[doc(alias = "__ZNSt9exceptionD2Ev$shim")]
#[doc(alias = "__ZNSt9exceptionD2Ev$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f354 as stub_f1f354;

// 0xf1f360 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i$shim
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i$shim")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX8ViewBaseEEENS2_IPN10RobloxView9RenderJobEEENS2_IdEEEclINS_4_mfi3mf2IvS4_PNS3_7IMetricEdEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i$shim")]
pub use rbx_core::generated_core_shard_gs::stub_f1f360 as stub_f1f360;

// 0xf1f36c — __ZN18iOSSettingsServiceD2Ev$shim
#[doc(alias = "__ZN18iOSSettingsServiceD2Ev$shim")]
#[doc(alias = "__ZN18iOSSettingsServiceD2Ev$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f36c as stub_f1f36c;

// 0xf1f378 — __ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv$shim
#[doc(alias = "__ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv$shim")]
#[doc(alias = "__ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f378 as stub_f1f378;

// 0xf1f384 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f384 as stub_f1f384;

// 0xf1f390 — __ZNK5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEclES5_$shim
#[doc(alias = "__ZNK5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEclES5_$shim")]
#[doc(alias = "__ZNK5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEclES5_$shim")]
pub fn stub_f1f390(slot: &GenericSlotWrapper, desc: *const PropertyDescriptor) {
    // IDA 0xf1f390 (decompile: tail-calls `function1<void, PropertyDescriptor const*>::operator()`; disasm: LDR R12 / ADD PC / BX R12 PLT jump to the real operator()).
    // `operator()` dispatches the stored callable (empty throws); presence collapses into `Option`.
    // SAFETY: `desc` must point to a live PropertyDescriptor for the call.
    if let Some(cb) = slot.on_prop {
        cb(desc);
    }
}

// 0xf1f39c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE24safe_static_do_get_mutexEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_gs::stub_f1f39c as stub_f1f39c;

// 0xf1f3a8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_$shim
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_$shim")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_$shim")]
pub use rbx_core::generated_core_shard_gs::stub_f1f3a8 as stub_f1f3a8;

// 0xf1f3b4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_gs::stub_f1f3b4 as stub_f1f3b4;

// 0xf1f3e4 — ___cxa_atexit$shim
// type: int __fastcall(void (__fastcall *lpfunc)(void *), void *obj, void *lpdso_handle)
#[doc(alias = "___cxa_atexit$shim")]
#[doc(alias = "___cxa_atexit$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f3e4 as stub_f1f3e4;

// 0xf1f3f0 — __ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv$shim")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f3f0 as stub_f1f3f0;

// 0xf1f3fc — __ZNSsC1ERKSs$shim
// type: int __fastcall(std::string *, const std::string *)
#[doc(alias = "__ZNSsC1ERKSs$shim")]
#[doc(alias = "__ZNSsC1ERKSs$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f3fc as stub_f1f3fc;

// 0xf1f408 — __ZNSsD2Ev$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNSsD2Ev$shim")]
#[doc(alias = "__ZNSsD2Ev$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f408 as stub_f1f408;

// 0xf1f414 — __ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_$shim")]
#[doc(alias = "__ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f414 as stub_f1f414;

// 0xf1f420 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv$shim
// type: int()
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f420 as stub_f1f420;

// 0xf1f42c — __ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_$shim")]
#[doc(alias = "__ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1f42c as stub_f1f42c;

// 0xf1f438 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f438 as stub_f1f438;

// 0xf1f45c — __ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f45c as stub_f1f45c;

// 0xf1f468 — __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
#[doc(alias = "__ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f468 as stub_f1f468;

// 0xf1f474 — __ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f474 as stub_f1f474;

// 0xf1f4b0 — __ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f4b0 as stub_f1f4b0;

// 0xf1f4bc — __ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f4bc as stub_f1f4bc;

// 0xf1f4c8 — __ZNK3RBX10Reflection8EnumDescINS_8NormalIdEE14convertToIndexES2_$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_8NormalIdEE14convertToIndexES2_$shim")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_8NormalIdEE14convertToIndexES2_$shim")]
pub use rbx_reflection::generated_shard_ef::stub_f1f4c8 as stub_f1f4c8;

// 0xf1f4d4 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED2Ev$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED2Ev$shim")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED2Ev$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1f4d4 as stub_f1f4d4;

// 0xf1f4e0 — __ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv$shim")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1f4e0 as stub_f1f4e0;

// 0xf1f4ec — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim
// type: int()
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim")]
pub fn stub_f1f4ec(table: &mut DescriptorTable, additional: usize) {
    // IDA 0xf1f4ec (decompile: tail-calls `table_impl<map<const char*, FunctionDescriptor*>>::rehash_impl`; disasm: same 3-insn PLT shape as sampled 0xf1f4ec).
    table.rehash_for_insert(additional);
}

// 0xf1f4f8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim")]
pub fn stub_f1f4f8(table: &mut DescriptorTable, buckets: usize) {
    // IDA 0xf1f4f8 (decompile: tail-calls `table<map<const char*, FunctionDescriptor*>>::create_buckets(a1, a2)`; disasm: same 3-insn PLT shape).
    table.create_buckets(buckets);
}

// 0xf1f504 — __ZNK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv$shim")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1f504 as stub_f1f504;

// 0xf1f510 — ___cxa_guard_release$shim
// type: void __fastcall(__guard *)
#[doc(alias = "___cxa_guard_release$shim")]
#[doc(alias = "___cxa_guard_release$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f510 as stub_f1f510;

// 0xf1f51c — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18CallbackDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18CallbackDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")]
#[doc(alias = "__ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18CallbackDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")]
pub fn stub_f1f51c(members: &mut Vec<*const ()>, index: usize, value: *const ()) {
    // IDA 0xf1f51c (decompile: tail-calls `vector<MemberDescriptorContainer<CallbackDescriptor>*>::_M_insert_aux`; disasm: LDR R12 / ADD PC / BX R12 PLT jump).
    // `_M_insert_aux` makes room and shifts the tail; `Vec::insert` is the same.
    // SAFETY: `value` must be a live CallbackDescriptor container pointer; `index` must be <= len.
    members.insert(index, value);
}

// 0xf1f528 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10staticDataEv$shim
// type: int()
#[doc(alias = "__ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10staticDataEv$shim")]
#[doc(alias = "__ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10staticDataEv$shim")]
pub fn stub_f1f528() -> &'static Mutex<DescriptorStore> {
    // IDA 0xf1f528 (decompile: tail-calls `MemberDescriptorContainer<CallbackDescriptor>::staticData()`; disasm: LDR R12 / ADD PC / BX R12 PLT jump).
    // Function-static table becomes `OnceLock`; same treatment as the `LazyLock` singletons in generated_190.
    static_store(&CALLBACK_MEMBER_STORE)
}

// 0xf1f534 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim")]
pub fn stub_f1f534(table: &mut DescriptorTable, additional: usize) {
    // IDA 0xf1f534 (decompile: tail-calls `table_impl<map<const char*, CallbackDescriptor*>>::rehash_impl(a1, a2)`; disasm: same 3-insn PLT shape).
    table.rehash_for_insert(additional);
}

// 0xf1f540 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim")]
pub fn stub_f1f540(table: &mut DescriptorTable, buckets: usize) {
    // IDA 0xf1f540 (decompile: tail-calls `table<map<const char*, CallbackDescriptor*>>::create_buckets` via `0xf1f546`; disasm: same 3-insn PLT shape).
    table.create_buckets(buckets);
}

// 0xf1f54c — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_23YieldFunctionDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_23YieldFunctionDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")]
#[doc(alias = "__ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_23YieldFunctionDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")]
pub fn stub_f1f54c(members: &mut Vec<*const ()>, index: usize, value: *const ()) {
    // IDA 0xf1f54c (decompile: tail-calls `vector<MemberDescriptorContainer<YieldFunctionDescriptor>*>::_M_insert_aux`; disasm: same 3-insn PLT shape).
    // SAFETY: `value` must be a live YieldFunctionDescriptor container pointer; `index` must be <= len.
    members.insert(index, value);
}

// 0xf1f558 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10staticDataEv$shim
// type: int()
#[doc(alias = "__ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10staticDataEv$shim")]
#[doc(alias = "__ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10staticDataEv$shim")]
pub fn stub_f1f558() -> &'static Mutex<DescriptorStore> {
    // IDA 0xf1f558 (decompile: tail-calls `MemberDescriptorContainer<YieldFunctionDescriptor>::staticData(a1)`; disasm: same 3-insn PLT shape).
    static_store(&YIELD_FUNCTION_MEMBER_STORE)
}

// 0xf1f564 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim")]
pub fn stub_f1f564(table: &mut DescriptorTable, additional: usize) {
    // IDA 0xf1f564 (decompile: tail-calls `table_impl<map<const char*, YieldFunctionDescriptor*>>::rehash_impl(a1, a2)`; disasm: same 3-insn PLT shape).
    table.rehash_for_insert(additional);
}

// 0xf1f570 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim")]
pub fn stub_f1f570(table: &mut DescriptorTable, buckets: usize) {
    // IDA 0xf1f570 (decompile: tail-calls `table<map<const char*, YieldFunctionDescriptor*>>::create_buckets` via `0xf1f576`; disasm: same 3-insn PLT shape).
    table.create_buckets(buckets);
}

// 0xf1f57c — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_15EventDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_15EventDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")]
#[doc(alias = "__ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_15EventDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")]
pub fn stub_f1f57c(members: &mut Vec<*const ()>, index: usize, value: *const ()) {
    // IDA 0xf1f57c (decompile: tail-calls `vector<MemberDescriptorContainer<EventDescriptor>*>::_M_insert_aux`; disasm: same 3-insn PLT shape).
    // SAFETY: `value` must be a live EventDescriptor container pointer; `index` must be <= len.
    members.insert(index, value);
}

// 0xf1f588 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10staticDataEv$shim
// type: int()
#[doc(alias = "__ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10staticDataEv$shim")]
#[doc(alias = "__ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10staticDataEv$shim")]
pub fn stub_f1f588() -> &'static Mutex<DescriptorStore> {
    // IDA 0xf1f588 (decompile: tail-calls `MemberDescriptorContainer<EventDescriptor>::staticData(a1)`; disasm: same 3-insn PLT shape).
    static_store(&EVENT_MEMBER_STORE)
}

// 0xf1f594 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim")]
pub fn stub_f1f594(table: &mut DescriptorTable, additional: usize) {
    // IDA 0xf1f594 (decompile: tail-calls `table_impl<map<const char*, EventDescriptor*>>::rehash_impl(a1, a2)`; disasm: same 3-insn PLT shape).
    table.rehash_for_insert(additional);
}

// 0xf1f5a0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim")]
pub fn stub_f1f5a0(table: &mut DescriptorTable, buckets: usize) {
    // IDA 0xf1f5a0 (decompile: tail-calls `table<map<const char*, EventDescriptor*>>::create_buckets` via `0xf1f5a6`; disasm: same 3-insn PLT shape).
    table.create_buckets(buckets);
}

// 0xf1f5ac — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18FunctionDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18FunctionDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")]
#[doc(alias = "__ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18FunctionDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")]
pub fn stub_f1f5ac(members: &mut Vec<*const ()>, index: usize, value: *const ()) {
    // IDA 0xf1f5ac (decompile: tail-calls `vector<MemberDescriptorContainer<FunctionDescriptor>*>::_M_insert_aux`; disasm: same 3-insn PLT shape).
    // SAFETY: `value` must be a live FunctionDescriptor container pointer; `index` must be <= len.
    members.insert(index, value);
}

// 0xf1f5b8 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE10staticDataEv$shim
// type: int()
#[doc(alias = "__ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE10staticDataEv$shim")]
#[doc(alias = "__ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE10staticDataEv$shim")]
pub fn stub_f1f5b8() -> &'static Mutex<DescriptorStore> {
    // IDA 0xf1f5b8 (decompile: tail-calls `MemberDescriptorContainer<FunctionDescriptor>::staticData(a1)`; disasm: same 3-insn PLT shape).
    static_store(&FUNCTION_MEMBER_STORE)
}

// 0xf1f5c4 — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18PropertyDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18PropertyDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")]
#[doc(alias = "__ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18PropertyDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")]
pub fn stub_f1f5c4(members: &mut Vec<*const ()>, index: usize, value: *const ()) {
    // IDA 0xf1f5c4 (decompile: tail-calls `vector<MemberDescriptorContainer<PropertyDescriptor>*>::_M_insert_aux`; disasm: same 3-insn PLT shape).
    // SAFETY: `value` must be a live PropertyDescriptor container pointer; `index` must be <= len.
    members.insert(index, value);
}

// 0xf1f5d0 — __ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE10staticDataEv$shim
// type: int()
#[doc(alias = "__ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE10staticDataEv$shim")]
#[doc(alias = "__ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE10staticDataEv$shim")]
pub fn stub_f1f5d0() -> &'static Mutex<DescriptorStore> {
    // IDA 0xf1f5d0 (decompile: tail-calls `MemberDescriptorContainer<PropertyDescriptor>::staticData(a1)`; disasm: same 3-insn PLT shape).
    static_store(&PROPERTY_MEMBER_STORE)
}

// 0xf1f5dc — __ZN5boost16exception_detail14bad_exception_D2Ev$shim
// type: void __fastcall(boost::exception_detail::bad_exception_ *)
#[doc(alias = "__ZN5boost16exception_detail14bad_exception_D2Ev$shim")]
#[doc(alias = "__ZN5boost16exception_detail14bad_exception_D2Ev$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1f5dc as stub_f1f5dc;

// 0xf1f5e8 — __ZN5boost16exception_detail10bad_alloc_D2Ev$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost16exception_detail10bad_alloc_D2Ev$shim")]
#[doc(alias = "__ZN5boost16exception_detail10bad_alloc_D2Ev$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1f5e8 as stub_f1f5e8;

// 0xf1f5f4 — __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptorESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptorESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_$shim")]
#[doc(alias = "__ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptorESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_$shim")]
pub fn stub_f1f5f4(descs: &mut Vec<*const ()>, index: usize, value: *const ()) {
    // IDA 0xf1f5f4 (decompile: tail-calls `vector<EnumDescriptor const*>::_M_insert_aux(a1, a2)`; disasm: same 3-insn PLT shape).
    // SAFETY: `value` must be a live EnumDescriptor pointer; `index` must be <= len.
    descs.insert(index, value);
}

// 0xf1f600 — __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED2Ev$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED2Ev$shim")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED2Ev$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1f600 as stub_f1f600;

// 0xf1f60c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm$shim")]
pub fn stub_f1f60c(table: &mut DescriptorTable, additional: usize) {
    // IDA 0xf1f60c (decompile: tail-calls `table_impl<map<const char*, PropertyDescriptor*>>::rehash_impl(a1, a2)`; disasm: same 3-insn PLT shape).
    table.rehash_for_insert(additional);
}

// 0xf1f618 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm$shim")]
pub fn stub_f1f618(table: &mut DescriptorTable, buckets: usize) {
    // IDA 0xf1f618 (decompile: tail-calls `table<map<const char*, PropertyDescriptor*>>::create_buckets` via `0xf1f61e`; disasm: same 3-insn PLT shape).
    table.create_buckets(buckets);
}

// 0xf1f624 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE11rehash_implEm$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE11rehash_implEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE11rehash_implEm$shim")]
pub fn stub_f1f624(table: &mut DescriptorTable, additional: usize) {
    // IDA 0xf1f624 (decompile: tail-calls `table_impl<map<type_info const*, EnumDescriptor const*, TypeHash, TypeEqual>>::rehash_impl(a1, a2)`; disasm: same 3-insn PLT shape).
    // The `type_info*` key has no Rust form; the key string (type name) plays its role in `DescriptorTable`.
    table.rehash_for_insert(additional);
}

// 0xf1f630 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE14create_bucketsEm$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE14create_bucketsEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE14create_bucketsEm$shim")]
pub fn stub_f1f630(table: &mut DescriptorTable, buckets: usize) {
    // IDA 0xf1f630 (decompile: tail-calls `table<map<type_info const*, EnumDescriptor const*, TypeHash, TypeEqual>>::create_buckets` via `0xf1f636`; disasm: same 3-insn PLT shape).
    table.create_buckets(buckets);
}

// 0xf1f63c — __ZNSt6vectorIPKN3RBX10Reflection4TypeESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIPKN3RBX10Reflection4TypeESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_$shim")]
#[doc(alias = "__ZNSt6vectorIPKN3RBX10Reflection4TypeESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_$shim")]
pub fn stub_f1f63c(types: &mut Vec<*const ()>, index: usize, value: *const ()) {
    // IDA 0xf1f63c (decompile: tail-calls `vector<Type const*>::_M_insert_aux(a1, a2)`; disasm: same 3-insn PLT shape).
    // SAFETY: `value` must be a live Type pointer; `index` must be <= len.
    types.insert(index, value);
}

// 0xf1f648 — __ZN3RBX4Name9doDeclareILZNS_11sCoreScriptEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sCoreScriptEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sCoreScriptEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f648 as stub_f1f648;

// 0xf1f654 — __ZN3RBX4Name9doDeclareILZNS_14sStarterScriptEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sStarterScriptEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sStarterScriptEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f654 as stub_f1f654;

// 0xf1f660 — __ZN3RBX16withVariantValueIiNS_3Lua14ArgumentPusherEEET_RKNS_10Reflection7VariantET0_$shim
// type: int(void)
#[doc(alias = "__ZN3RBX16withVariantValueIiNS_3Lua14ArgumentPusherEEET_RKNS_10Reflection7VariantET0_$shim")]
#[doc(alias = "__ZN3RBX16withVariantValueIiNS_3Lua14ArgumentPusherEEET_RKNS_10Reflection7VariantET0_$shim")]
pub fn stub_f1f660(variant: &Variant, push: fn(i32)) -> i32 {
    // IDA 0xf1f660 (decompile: tail-calls `withVariantValue<int, ArgumentPusher>(Variant const&, ArgumentPusher)`; disasm: same 3-insn PLT shape).
    // `any_cast<int>` plus `ArgumentPusher::operator()(int)`; a mistyped variant throws in C++,
    // which collapses into the `0` default here (no exception boundary in this crate).
    match variant {
        Variant::Int(i) => {
            push(*i);
            *i
        }
        _ => 0,
    }
}

// 0xf1f66c — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringERKS3_P9lua_State$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringERKS3_P9lua_State$shim")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringERKS3_P9lua_State$shim")]
pub fn stub_f1f66c(value: &Vector3i16, state: *mut LuaState) -> i32 {
    // IDA 0xf1f66c (decompile: tail-calls `Bridge<Vector3int16>::on_tostring(const&, lua_State*)`; disasm: same 3-insn PLT shape).
    // Real body (IDA 0x278574): `StringConverter<Vector3int16>::convertToString`, `lua_pushlstring`, `return 1`.
    // SAFETY: `state` must be a live Lua state; `value` must be live for the call.
    let text = v3i16_text(value);
    unsafe { push_tostring_result(state, &text) }
}

// 0xf1f678 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringERKS3_P9lua_State$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringERKS3_P9lua_State$shim")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringERKS3_P9lua_State$shim")]
pub fn stub_f1f678(value: &Vector2i16, state: *mut LuaState) -> i32 {
    // IDA 0xf1f678 (decompile: tail-calls `Bridge<Vector2int16>::on_tostring(const&, lua_State*)`; disasm: same 3-insn PLT shape).
    // Same template as 0x278574: convert, push, return 1.
    // SAFETY: `state` must be a live Lua state; `value` must be live for the call.
    let text = v2i16_text(value);
    unsafe { push_tostring_result(state, &text) }
}

// 0xf1f684 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringERKS3_P9lua_State$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringERKS3_P9lua_State$shim")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringERKS3_P9lua_State$shim")]
pub fn stub_f1f684(value: &Vector3f, state: *mut LuaState) -> i32 {
    // IDA 0xf1f684 (decompile: tail-calls `Bridge<Vector3>::on_tostring(const&, lua_State*)`; disasm: same 3-insn PLT shape).
    // Same template as 0x278574: convert, push, return 1.
    // SAFETY: `state` must be a live Lua state; `value` must be live for the call.
    let text = v3f_text(value);
    unsafe { push_tostring_result(state, &text) }
}

// 0xf1f690 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringERKS3_P9lua_State$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringERKS3_P9lua_State$shim")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringERKS3_P9lua_State$shim")]
pub fn stub_f1f690(value: &Vector2f, state: *mut LuaState) -> i32 {
    // IDA 0xf1f690 (decompile: tail-calls `Bridge<Vector2>::on_tostring(const&, lua_State*)`; disasm: same 3-insn PLT shape).
    // Same template as 0x278574: convert, push, return 1.
    // SAFETY: `state` must be a live Lua state; `value` must be live for the call.
    let text = v2f_text(value);
    unsafe { push_tostring_result(state, &text) }
}

// 0xf1f69c — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringERKS3_P9lua_State$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringERKS3_P9lua_State$shim")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringERKS3_P9lua_State$shim")]
pub fn stub_f1f69c(state: *mut LuaState) -> i32 {
    // IDA 0xf1f69c (decompile: 1-arg `Bridge<CoordinateFrame>::on_tostring(lua_State*)` tail-call; disasm: same 3-insn PLT shape).
    // Twin (IDA 0x277cf8): `luaL_checkudata(L, 1, className)` then tail-calls the 2-arg form.
    // SAFETY: `state` must be a live Lua state with a CoordinateFrame userdata at index 1.
    unsafe {
        let raw = crate::instance::lua_ffi::lua_l_checkudata(
            state,
            1,
            COORD_FRAME_CLASS.as_ptr() as *const core::ffi::c_char,
        );
        coord_frame_tostring(&*(raw as *const CoordFrame), state)
    }
}

// 0xf1f6a8 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE12manage_smallERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE$shim
// type: int(void)
#[doc(alias = "__ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE12manage_smallERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE$shim")]
#[doc(alias = "__ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE12manage_smallERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE$shim")]
pub fn stub_f1f6a8(src: &WeakThreadStringBind, dst: &mut ThreadStringCallback, op: FunctorOp) -> bool {
    // IDA 0xf1f6a8 (decompile: tail-calls `functor_manager_common<bind_t<void, void(*)(intrusive_ptr<WeakThreadRef>, string), list2<...>>>::manage_small(src, dst, op)`; disasm: LDR R12 / ADD PC / BX R12 PLT jump).
    // Small-buffer manage dispatch mirrors shard_277 `stub_0x351c10`: clone/move copy, destroy clears, type ops report the homed type.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            dst.inner = Some(src.clone());
            true
        }
        FunctorOp::Destroy => {
            dst.inner = None;
            false
        }
        FunctorOp::CheckType => true,
        FunctorOp::GetType => true,
    }
}

// 0xf1f6b4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int(void)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f6b4 as stub_f1f6b4;

// 0xf1f6c0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int(void)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1f6c0(src: &YieldVariantBind, dst: &mut YieldVariantCallback, op: FunctorOp) -> bool {
    // IDA 0xf1f6c0 (decompile: tail-calls `functor_manager<bind_t<void, mf1<void, YieldFunctionStateObject, Variant>, list2<...>>>::manager`; disasm: same 3-insn PLT shape).
    // Clone/move retain the target (`shared_count` copy at bind time); destroy releases; same dispatch as shard_277 `stub_0x351c10`.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            dst.inner = Some(src.clone());
            true
        }
        FunctorOp::Destroy => {
            dst.inner = None;
            false
        }
        FunctorOp::CheckType => true,
        FunctorOp::GetType => true,
    }
}

// 0xf1f6cc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1f6cc(src: &TupleResumeBind, dst: &mut TupleResumeCallback, op: FunctorOp) -> bool {
    // IDA 0xf1f6cc (decompile: tail-calls `functor_manager<bind_t<shared_ptr<Tuple>, (*)(WeakFunctionRef, shared_ptr<const Tuple>, intrusive_ptr<WeakThreadRef>), list3<...>>>::manager`; disasm: same 3-insn PLT shape).
    // Clone/move copy the weak identity and retain the thread; destroy releases both.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            dst.inner = Some(src.clone());
            true
        }
        FunctorOp::Destroy => {
            dst.inner = None;
            false
        }
        FunctorOp::CheckType => true,
        FunctorOp::GetType => true,
    }
}

// 0xf1f6d8 — __ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_$shim
#[doc(alias = "__ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_$shim")]
#[doc(alias = "__ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1f6d8 as stub_f1f6d8;

// 0xf1f6f0 — __ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev$shim
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev$shim")]
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev$shim")]
pub fn stub_f1f6f0(item: *mut LuaSettingsItem) {
    // IDA 0xf1f6f0 (decompile: tail-calls `GlobalAdvancedSettingsItem<LuaSettings, sLuaSettings>::D0()`; disasm: same 3-insn PLT shape).
    // D0 is the deleting destructor: runs D2 then frees; `Box::from_raw` + drop is the same.
    // SAFETY: `item` must be a live `Box<LuaSettingsItem>` pointer handed off exactly once.
    unsafe {
        drop(Box::from_raw(item));
    }
}

// 0xf1f708 — __ZNK3RBX15ServiceProvider6createINS_20RuntimeScriptServiceEEEPT_v$shim
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_20RuntimeScriptServiceEEEPT_v$shim")]
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_20RuntimeScriptServiceEEEPT_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f708 as stub_f1f708;

// 0xf1f714 — __ZNK3RBX15ServiceProvider6createINS_25ScriptInformationProviderEEEPT_v$shim
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_25ScriptInformationProviderEEEPT_v$shim")]
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_25ScriptInformationProviderEEEPT_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f714 as stub_f1f714;

// 0xf1f72c — __ZN3RBX4Name9doDeclareILZNS_11sBaseScriptEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sBaseScriptEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sBaseScriptEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f72c as stub_f1f72c;

// 0xf1f744 — __ZN3RBX11LocalScriptD0Ev$shim
#[doc(alias = "__ZN3RBX11LocalScriptD0Ev$shim")]
#[doc(alias = "__ZN3RBX11LocalScriptD0Ev$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f744 as stub_f1f744;

// 0xf1f75c — __ZN3RBX4Name9doDeclareILZNS_7sScriptEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sScriptEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sScriptEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f75c as stub_f1f75c;

// 0xf1f768 — __ZN3RBX4Name9doDeclareILZNS_21sRuntimeScriptServiceEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sRuntimeScriptServiceEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sRuntimeScriptServiceEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f768 as stub_f1f768;

// 0xf1f774 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_20RuntimeScriptServiceEEEmv$shim
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_20RuntimeScriptServiceEEEmv$shim")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_20RuntimeScriptServiceEEEmv$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f774 as stub_f1f774;

// 0xf1f78c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX6ScriptEEESsNS6_25ScriptInformationProvider13RequestResultEbbbENS3_5list6INS3_5valueIS8_EENSE_ISsEENSE_ISA_EENSE_IbEESI_SI_EEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX6ScriptEEESsNS6_25ScriptInformationProvider13RequestResultEbbbENS3_5list6INS3_5valueIS8_EENSE_ISsEENSE_ISA_EENSE_IbEESI_SI_EEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX6ScriptEEESsNS6_25ScriptInformationProvider13RequestResultEbbbENS3_5list6INS3_5valueIS8_EENSE_ISsEENSE_ISA_EENSE_IbEESI_SI_EEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f78c as stub_f1f78c;

// 0xf1f798 — __ZN3RBX10Reflection11Call0HelperINS_6ScriptEMS2_FSsvESsE4callEPS2_S4_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_6ScriptEMS2_FSsvESsE4callEPS2_S4_RNS0_7VariantE$shim")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_6ScriptEMS2_FSsvESsE4callEPS2_S4_RNS0_7VariantE$shim")]
pub fn stub_f1f798(obj: &ScriptSource, method: fn(&ScriptSource) -> String) -> Variant {
    // IDA 0xf1f798 (decompile: tail-calls `Call0Helper<Script, string (Script::*)(), string>::call()`; disasm: same 3-insn PLT shape).
    // Member-fn pointer call through the descriptor, then `Variant = string`; mirrors `generated_190::stub_0xfe54`.
    Variant::Text(method(obj))
}

// 0xf1f7a4 — __ZN11LuaProfiler8hookCallEP9lua_StateP9lua_Debug$shim
#[doc(alias = "__ZN11LuaProfiler8hookCallEP9lua_StateP9lua_Debug$shim")]
#[doc(alias = "__ZN11LuaProfiler8hookCallEP9lua_StateP9lua_Debug$shim")]
pub fn stub_f1f7a4(state: *mut LuaState, dbg: &LuaDebug) {
    // IDA 0xf1f7a4 (decompile: tail-calls `LuaProfiler::hookCall()`; disasm: LDR R12 / ADD PC / BX R12 PLT jump).
    // The profiler tap observes the call event; the state is retained by the caller, so only the event record is modeled.
    // SAFETY: `state` must be a live Lua state.
    let _ = state;
    *LAST_HOOK_EVENT.lock() = Some((dbg.event, dbg.line));
}

// 0xf1f7b0 — __ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS2_S4_EE$shim
#[doc(alias = "__ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS2_S4_EE$shim")]
#[doc(alias = "__ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS2_S4_EE$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f7b0 as stub_f1f7b0;

// 0xf1f7bc — __ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v$shim")]
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f7bc as stub_f1f7bc;

// 0xf1f7c8 — __ZNK3RBX15ServiceProvider4findINS_5Stats12StatsServiceEEEPT_v$shim
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_5Stats12StatsServiceEEEPT_v$shim")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_5Stats12StatsServiceEEEPT_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f7c8 as stub_f1f7c8;

// 0xf1f7d4 — __ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
#[doc(alias = "__ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
#[doc(alias = "__ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f7d4 as stub_f1f7d4;

// 0xf1f7e0 — __ZNK3RBX15ServiceProvider4findINS_13ScriptContextEEEPT_v$shim
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_13ScriptContextEEEPT_v$shim")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_13ScriptContextEEEPT_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f7e0 as stub_f1f7e0;

// 0xf1f7ec — __ZN3RBX20RuntimeScriptServiceD2Ev$shim
#[doc(alias = "__ZN3RBX20RuntimeScriptServiceD2Ev$shim")]
#[doc(alias = "__ZN3RBX20RuntimeScriptServiceD2Ev$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f7ec as stub_f1f7ec;

// 0xf1f828 — __ZN3RBX4Name7declareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f828 as stub_f1f828;

// 0xf1f840 — __ZN3RBX4Name9doDeclareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f840 as stub_f1f840;

// 0xf1f84c — __ZN3RBX4Name7declareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f84c as stub_f1f84c;

// 0xf1f864 — __ZN3RBX4Name9doDeclareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f864 as stub_f1f864;

// 0xf1f870 — __ZN3RBX4Name7declareILZNS_12sLocalScriptEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sLocalScriptEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sLocalScriptEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f870 as stub_f1f870;

// 0xf1f87c — __ZN3RBX4Name9doDeclareILZNS_12sLocalScriptEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sLocalScriptEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sLocalScriptEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f87c as stub_f1f87c;

// 0xf1f888 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ScriptContextEEEmv$shim
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13ScriptContextEEEmv$shim")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13ScriptContextEEEmv$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f888 as stub_f1f888;

// 0xf1f894 — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE17_M_reallocate_mapEmb$shim
#[doc(alias = "__ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE17_M_reallocate_mapEmb$shim")]
#[doc(alias = "__ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE17_M_reallocate_mapEmb$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f894 as stub_f1f894;

// 0xf1f8ac — __ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f8ac as stub_f1f8ac;

// 0xf1f8b8 — __ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f8b8 as stub_f1f8b8;

// 0xf1f8c4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13ScriptContextERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_$shim
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13ScriptContextERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_$shim")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13ScriptContextERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f8c4 as stub_f1f8c4;

// 0xf1f8d0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f8d0 as stub_f1f8d0;

// 0xf1f8dc — __ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f8dc as stub_f1f8dc;

// 0xf1f8e8 — __ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f8e8 as stub_f1f8e8;

// 0xf1f8f4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15ContentProviderEEEmv$shim
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15ContentProviderEEEmv$shim")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15ContentProviderEEEmv$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f8f4 as stub_f1f8f4;

// 0xf1f900 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1f900(src: &ScriptContextResumeBind, dst: &mut ScriptContextResumeCallback, op: FunctorOp) -> bool {
    // IDA 0xf1f900 (decompile: tail-calls `functor_manager<bind_t<void, mf2<void, ScriptContext, WeakThreadRef, lua_State*>, list3<...>>>::manager`; disasm: same 3-insn PLT shape).
    // Clone/move copy the bound context pointer and retain the thread; destroy releases.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            dst.inner = Some(src.clone());
            true
        }
        FunctorOp::Destroy => {
            dst.inner = None;
            false
        }
        FunctorOp::CheckType => true,
        FunctorOp::GetType => true,
    }
}

// 0xf1f90c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1f90c(src: &LuaStateBind, dst: &mut LuaStateCallback, op: FunctorOp) -> bool {
    // IDA 0xf1f90c (decompile: tail-calls `functor_manager<bind_t<void, void(*)(lua_State*, function<void(const char*, const char*, shared_ptr<BaseScript>, int)>), list2<arg<1>, value<...>>>>::manager`; disasm: same 3-insn PLT shape).
    // The stored `boost::function` has no owned Rust state; words copy on clone/move and clear on destroy.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            dst.inner = Some(*src);
            true
        }
        FunctorOp::Destroy => {
            dst.inner = None;
            false
        }
        FunctorOp::CheckType => true,
        FunctorOp::GetType => true,
    }
}

// 0xf1f918 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1f918(src: &LuaTupleBind, dst: &mut LuaTupleCallback, op: FunctorOp) -> bool {
    // IDA 0xf1f918 (decompile: tail-calls `functor_manager<bind_t<void, void(*)(lua_State*, function<void(shared_ptr<const Tuple>)>), list2<arg<1>, value<...>>>>::manager`; disasm: same 3-insn PLT shape).
    // Same word-copy/clear dispatch as 0xf1f90c; distinct instantiation, distinct holder.
    match op {
        FunctorOp::Clone | FunctorOp::Move => {
            dst.inner = Some(*src);
            true
        }
        FunctorOp::Destroy => {
            dst.inner = None;
            false
        }
        FunctorOp::CheckType => true,
        FunctorOp::GetType => true,
    }
}

// 0xf1f924 — __ZNK5boost9function0IvEclEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK5boost9function0IvEclEv$shim")]
#[doc(alias = "__ZNK5boost9function0IvEclEv$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1f924 as stub_f1f924;

// 0xf1f930 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f930 as stub_f1f930;

// 0xf1f93c — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS0_5list1INS0_5valueINS_10shared_ptrIS5_EEEEEEEclEv$shim
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS0_5list1INS0_5valueINS_10shared_ptrIS5_EEEEEEEclEv$shim")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS0_5list1INS0_5valueINS_10shared_ptrIS5_EEEEEEEclEv$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1f93c as stub_f1f93c;

// 0xf1f948 — __ZN5boost6detail15sp_counted_base7releaseEv$shim
// type: int(void)
#[doc(alias = "__ZN5boost6detail15sp_counted_base7releaseEv$shim")]
#[doc(alias = "__ZN5boost6detail15sp_counted_base7releaseEv$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1f948 as stub_f1f948;

// 0xf1f978 — __ZN3RBX4Name7declareILZNS_12sLuaSettingsEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sLuaSettingsEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sLuaSettingsEEEERKS0_v$shim")]
pub fn stub_f1f978(map: &mut NameMap, name: &str) -> *const Name {
    // IDA 0xf1f978 (decompile: tail-calls `Name::declare<sLuaSettings>()`; disasm: same 3-insn PLT shape).
    // Interns `name` into the map (IDA `0x35bfec` model in generated_datamodel_shard_278): hit returns
    // the node, miss owns a fresh node and links it. Heap boxes never move, so pointers stay stable.
    // SAFETY: returned pointer stays live as long as the process (backed by the static store).
    if let Some(&found) = map.entries.get(name) {
        return found;
    }
    let store = LUA_DECLARE_STORE.get_or_init(|| Mutex::new(Vec::new()));
    let mut guard = store.lock();
    guard.push(Box::new(Name::default()));
    let ptr: *const Name = &**guard.last().unwrap() as *const Name;
    map.entries.insert(name.to_owned(), ptr);
    ptr
}

// 0xf1f984 — __ZN3RBX4Name9doDeclareILZNS_12sLuaSettingsEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sLuaSettingsEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sLuaSettingsEEEERKS0_v$shim")]
pub fn stub_f1f984() -> &'static str {
    // IDA 0xf1f984 (decompile: tail-calls `Name::doDeclare<sLuaSettings>()`; disasm: same 3-insn PLT shape).
    // `__cxa_guard_acquire` once-check + `Name::declare` + guard release collapse into `LazyLock`,
    // mirroring `generated_190::stub_0xf1dc`.
    LazyLock::force(&LUA_SETTINGS_NAME_DECL).as_str()
}

// 0xf1f9c0 — __ZN3RBX10Reflection11Call0HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_$shim")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_$shim")]
pub fn stub_f1f9c0(
    obj: &ScriptContextState,
    method: fn(&ScriptContextState) -> SharedPtr<Vec<Variant>>,
) -> SharedPtr<Vec<Variant>> {
    // IDA 0xf1f9c0 (decompile: tail-calls `Call0Helper<ScriptContext, shared_ptr<vector<Variant> const>()>::call()`; disasm: same 3-insn PLT shape).
    // Member-fn pointer call through the descriptor; the retained result vector crosses as `SharedPtr`.
    method(obj)
}

// 0xf1f9f0 — __ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
#[doc(alias = "__ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
#[doc(alias = "__ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f9f0 as stub_f1f9f0;

// 0xf1f9fc — __ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1f9fc as stub_f1f9fc;

// 0xf1fa08 — __ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1fa08 as stub_f1fa08;

// 0xf1fa14 — __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua13WeakThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE$shim
#[doc(alias = "__ZN5boost21intrusive_ptr_releaseIN3RBX3Lua13WeakThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE$shim")]
#[doc(alias = "__ZN5boost21intrusive_ptr_releaseIN3RBX3Lua13WeakThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE$shim")]
pub fn stub_f1fa14(handle: SharedPtr<LuaWeakThreadRef>) {
    // IDA 0xf1fa14 (decompile: tail-calls `intrusive_ptr_release<WeakThreadRef>(int32_t*)`; disasm: same 3-insn PLT shape).
    // `intrusive_ptr<WeakThreadRef>` is `SharedPtr<LuaWeakThreadRef>` (data_model.rs model);
    // release drops one retain, deleting at the last — `drop` is exactly that.
    drop(handle);
}

// 0xf1fa20 — __ZNSt6vectorIN3RBX3Lua15YieldingThreads13WaitingThreadESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim
#[doc(alias = "__ZNSt6vectorIN3RBX3Lua15YieldingThreads13WaitingThreadESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim")]
#[doc(alias = "__ZNSt6vectorIN3RBX3Lua15YieldingThreads13WaitingThreadESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim")]
pub fn stub_f1fa20(queue: &mut Vec<WaitingThread>, index: usize, value: WaitingThread) {
    // IDA 0xf1fa20 (decompile: tail-calls `vector<YieldingThreads::WaitingThread>::_M_insert_aux`; disasm: same 3-insn PLT shape).
    // `_M_insert_aux` makes room and shifts the tail; `Vec::insert` is the same (the retained thread moves with the value).
    queue.insert(index, value);
}

// 0xf1fa2c — __ZNSt5dequeISsSaISsEE9push_backERKSs$shim
#[doc(alias = "__ZNSt5dequeISsSaISsEE9push_backERKSs$shim")]
#[doc(alias = "__ZNSt5dequeISsSaISsEE9push_backERKSs$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1fa2c as stub_f1fa2c;

// 0xf1fa38 — __ZN5boost3_bi6bind_tImNS_4_mfi4cmf0ImN3RBX13ScriptContextEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim
#[doc(alias = "__ZN5boost3_bi6bind_tImNS_4_mfi4cmf0ImN3RBX13ScriptContextEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim")]
#[doc(alias = "__ZN5boost3_bi6bind_tImNS_4_mfi4cmf0ImN3RBX13ScriptContextEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1fa38 as stub_f1fa38;

// 0xf1fa44 — __ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs$shim
#[doc(alias = "__ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs$shim")]
#[doc(alias = "__ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1fa44 as stub_f1fa44;

// 0xf1fa50 — __ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb$shim
#[doc(alias = "__ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb$shim")]
#[doc(alias = "__ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1fa50 as stub_f1fa50;

// 0xf1fa5c — __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua6detail13LiveThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE$shim
#[doc(alias = "__ZN5boost21intrusive_ptr_releaseIN3RBX3Lua6detail13LiveThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE$shim")]
#[doc(alias = "__ZN5boost21intrusive_ptr_releaseIN3RBX3Lua6detail13LiveThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE$shim")]
pub fn stub_f1fa5c(handle: SharedPtr<LiveThreadRef>) {
    // IDA 0xf1fa5c (decompile: tail-calls `intrusive_ptr_release<LiveThreadRef>()`; disasm: same 3-insn PLT shape).
    // Same last-release-drop as 0xf1fa14, over the live-thread handle.
    drop(handle);
}

#[cfg(test)]
mod shard_283_half1_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static PROP_CALLS: AtomicUsize = AtomicUsize::new(0);
    fn record_prop(_desc: *const PropertyDescriptor) {
        PROP_CALLS.fetch_add(1, Ordering::SeqCst);
    }

    fn ptr_of(v: &u32) -> *const () {
        v as *const u32 as *const ()
    }

    #[test]
    fn f1f390_calls_stored_prop_callback() {
        PROP_CALLS.store(0, Ordering::SeqCst);
        let desc = PropertyDescriptor { name: "x" };
        let slot = GenericSlotWrapper {
            on_prop: Some(record_prop),
            on_pair: None,
            on_single: None,
            on_triple: None,
            on_triple_isi: None,
            on_chat: None,
            on_prop2: None,
            on_pair_if: None,
            on_player_chat: None,
            on_friend: None,
            on_str_inst: None,
        };
        stub_f1f390(&slot, &desc as *const PropertyDescriptor);
        assert_eq!(PROP_CALLS.load(Ordering::SeqCst), 1);
        let empty = GenericSlotWrapper {
            on_prop: None,
            on_pair: None,
            on_single: None,
            on_triple: None,
            on_triple_isi: None,
            on_chat: None,
            on_prop2: None,
            on_pair_if: None,
            on_player_chat: None,
            on_friend: None,
            on_str_inst: None,
        };
        stub_f1f390(&empty, &desc as *const PropertyDescriptor);
        assert_eq!(PROP_CALLS.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn f1f4ec_rehash_grows_and_keeps_entries() {
        let mut t = DescriptorTable::new();
        t.insert("a", std::ptr::null());
        stub_f1f4ec(&mut t, 64);
        assert!(t.entries.capacity() >= 64);
        assert!(t.find("a").is_some());
    }

    #[test]
    fn f1f4f8_create_buckets_allocates() {
        let mut t = DescriptorTable::new();
        stub_f1f4f8(&mut t, 32);
        assert!(t.entries.capacity() >= 32);
        t.insert("f", 0x1 as *const ());
        assert_eq!(t.find("f"), Some(0x1 as *const ()));
        assert_eq!(t.find("missing"), None);
    }

    #[test]
    fn f1f51c_inserts_callback_member_at_index() {
        let (a, b, c) = (1u32, 2u32, 3u32);
        let mut v = vec![ptr_of(&a), ptr_of(&c)];
        stub_f1f51c(&mut v, 1, ptr_of(&b));
        assert_eq!(v, vec![ptr_of(&a), ptr_of(&b), ptr_of(&c)]);
    }

    #[test]
    fn f1f528_static_data_is_singleton() {
        let s = stub_f1f528();
        assert!(std::ptr::eq(s, stub_f1f528()));
        assert!(s.lock().members.is_empty());
    }

    #[test]
    fn f1f534_rehash_callback_table() {
        let mut t = DescriptorTable::new();
        stub_f1f534(&mut t, 16);
        assert!(t.entries.capacity() >= 16);
    }

    #[test]
    fn f1f540_create_callback_buckets() {
        let mut t = DescriptorTable::new();
        stub_f1f540(&mut t, 16);
        assert!(t.entries.capacity() >= 16);
    }

    #[test]
    fn f1f54c_inserts_yield_member_at_index() {
        let (a, b) = (1u32, 2u32);
        let mut v = vec![ptr_of(&a)];
        stub_f1f54c(&mut v, 0, ptr_of(&b));
        assert_eq!(v, vec![ptr_of(&b), ptr_of(&a)]);
    }

    #[test]
    fn f1f558_static_data_is_singleton() {
        assert!(std::ptr::eq(stub_f1f558(), stub_f1f558()));
        assert!(!std::ptr::eq(stub_f1f558() as *const _ as *const (), stub_f1f528() as *const _ as *const ()));
    }

    #[test]
    fn f1f564_rehash_yield_table() {
        let mut t = DescriptorTable::new();
        stub_f1f564(&mut t, 8);
        assert!(t.entries.capacity() >= 8);
    }

    #[test]
    fn f1f570_create_yield_buckets() {
        let mut t = DescriptorTable::new();
        stub_f1f570(&mut t, 8);
        assert!(t.entries.capacity() >= 8);
    }

    #[test]
    fn f1f57c_inserts_event_member_at_end() {
        let (a, b) = (1u32, 2u32);
        let mut v = vec![ptr_of(&a)];
        stub_f1f57c(&mut v, 1, ptr_of(&b));
        assert_eq!(v, vec![ptr_of(&a), ptr_of(&b)]);
    }

    #[test]
    fn f1f588_static_data_registers_and_finds() {
        let store = stub_f1f588();
        let sentinel = 0x777 as *const ();
        {
            let mut g = store.lock();
            g.register("OnTouched", sentinel);
        }
        assert_eq!(store.lock().find("OnTouched"), Some(sentinel));
        assert_eq!(store.lock().find("Nope"), None);
    }

    #[test]
    fn f1f594_rehash_event_table() {
        let mut t = DescriptorTable::new();
        stub_f1f594(&mut t, 24);
        assert!(t.entries.capacity() >= 24);
    }

    #[test]
    fn f1f5a0_create_event_buckets() {
        let mut t = DescriptorTable::new();
        stub_f1f5a0(&mut t, 24);
        assert!(t.entries.capacity() >= 24);
    }

    #[test]
    fn f1f5ac_inserts_function_member() {
        let (a, b, c) = (1u32, 2u32, 3u32);
        let mut v = vec![ptr_of(&a), ptr_of(&b), ptr_of(&c)];
        let d = 4u32;
        stub_f1f5ac(&mut v, 3, ptr_of(&d));
        assert_eq!(v.len(), 4);
        assert_eq!(v[3], ptr_of(&d));
    }

    #[test]
    fn f1f5b8_static_data_is_singleton() {
        assert!(std::ptr::eq(stub_f1f5b8(), stub_f1f5b8()));
    }

    #[test]
    fn f1f5c4_inserts_property_member() {
        let (a, b) = (1u32, 2u32);
        let mut v: Vec<*const ()> = Vec::new();
        stub_f1f5c4(&mut v, 0, ptr_of(&a));
        stub_f1f5c4(&mut v, 0, ptr_of(&b));
        assert_eq!(v, vec![ptr_of(&b), ptr_of(&a)]);
    }

    #[test]
    fn f1f5d0_static_data_is_singleton() {
        assert!(std::ptr::eq(stub_f1f5d0(), stub_f1f5d0()));
    }

    #[test]
    fn f1f5f4_inserts_enum_descriptor() {
        let (a, b) = (1u32, 2u32);
        let mut v: Vec<*const ()> = Vec::new();
        stub_f1f5f4(&mut v, 0, ptr_of(&a));
        stub_f1f5f4(&mut v, 1, ptr_of(&b));
        assert_eq!(v, vec![ptr_of(&a), ptr_of(&b)]);
    }

    #[test]
    fn f1f60c_rehash_property_table() {
        let mut t = DescriptorTable::new();
        stub_f1f60c(&mut t, 48);
        assert!(t.entries.capacity() >= 48);
    }

    #[test]
    fn f1f618_create_property_buckets() {
        let mut t = DescriptorTable::new();
        stub_f1f618(&mut t, 48);
        assert!(t.entries.capacity() >= 48);
    }

    #[test]
    fn f1f624_rehash_type_keyed_table() {
        let mut t = DescriptorTable::new();
        t.insert("NormalId", 0x5 as *const ());
        stub_f1f624(&mut t, 40);
        assert!(t.entries.capacity() >= 40);
        assert_eq!(t.find("NormalId"), Some(0x5 as *const ()));
    }
}

#[cfg(test)]
mod shard_283_half2_tests {
    use super::*;

    fn ptr_of(v: &u32) -> *const () {
        v as *const u32 as *const ()
    }

    static PUSHED: Mutex<Vec<i32>> = Mutex::new(Vec::new());
    fn record_push(i: i32) {
        PUSHED.lock().push(i);
    }

    fn yield_noop(_t: &YieldStateObject, _v: &Variant) {}

    #[test]
    fn f1f630_create_type_buckets() {
        let mut t = DescriptorTable::new();
        stub_f1f630(&mut t, 20);
        assert!(t.entries.capacity() >= 20);
    }

    #[test]
    fn f1f63c_inserts_type_at_index() {
        let (a, b) = (1u32, 2u32);
        let mut v: Vec<*const ()> = Vec::new();
        stub_f1f63c(&mut v, 0, ptr_of(&a));
        stub_f1f63c(&mut v, 0, ptr_of(&b));
        assert_eq!(v, vec![ptr_of(&b), ptr_of(&a)]);
    }

    #[test]
    fn f1f660_pushes_int_variant() {
        PUSHED.lock().clear();
        assert_eq!(stub_f1f660(&Variant::Int(41), record_push), 41);
        assert_eq!(*PUSHED.lock(), vec![41]);
        assert_eq!(stub_f1f660(&Variant::Text("no".to_owned()), record_push), 0);
        assert_eq!(*PUSHED.lock(), vec![41]);
    }

    #[test]
    fn f1f66c_formats_vector3int16() {
        assert_eq!(v3i16_text(&Vector3i16 { x: 1, y: -2, z: 300 }), "1, -2, 300");
    }

    #[test]
    fn f1f678_formats_vector2int16() {
        assert_eq!(v2i16_text(&Vector2i16 { x: 7, y: 8 }), "7, 8");
    }

    #[test]
    fn f1f684_formats_vector3() {
        assert_eq!(v3f_text(&Vector3f { x: 1.0, y: 2.0, z: 3.0 }), "1, 2, 3");
    }

    #[test]
    fn f1f690_formats_vector2() {
        assert_eq!(v2f_text(&Vector2f { x: 4.0, y: 5.0 }), "4, 5");
    }

    #[test]
    fn f1f69c_formats_coordinate_frame() {
        let cf = CoordFrame {
            pos: Vector3f { x: 1.0, y: 2.0, z: 3.0 },
            rot: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        };
        assert_eq!(
            coord_frame_text(&cf),
            "1, 2, 3, 1, 0, 0, 0, 1, 0, 0, 0, 1"
        );
    }

    #[test]
    fn f1f6a8_manages_thread_string_bind() {
        let src = WeakThreadStringBind {
            thread: Some(SharedPtr::new(LuaWeakThreadRef::default())),
            script: "print(1)".to_owned(),
        };
        let mut dst = ThreadStringCallback::new();
        assert!(stub_f1f6a8(&src, &mut dst, FunctorOp::Clone));
        assert_eq!(dst.inner.as_ref().unwrap().script, "print(1)");
        assert!(stub_f1f6a8(&src, &mut dst, FunctorOp::Move));
        assert!(!dst.is_empty());
        assert!(stub_f1f6a8(&src, &mut dst, FunctorOp::CheckType));
        assert!(stub_f1f6a8(&src, &mut dst, FunctorOp::GetType));
        assert!(!stub_f1f6a8(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.is_empty());
    }

    #[test]
    fn f1f6c0_manages_yield_variant_bind() {
        let src = YieldVariantBind {
            target: SharedPtr::new(YieldStateObject::default()),
            invoke: yield_noop,
        };
        let mut dst = YieldVariantCallback::new();
        assert!(stub_f1f6c0(&src, &mut dst, FunctorOp::Clone));
        assert!(SharedPtr::ptr_eq(&dst.inner.as_ref().unwrap().target, &src.target));
        assert!(stub_f1f6c0(&src, &mut dst, FunctorOp::Move));
        assert!(!dst.is_empty());
        assert!(stub_f1f6c0(&src, &mut dst, FunctorOp::CheckType));
        assert!(stub_f1f6c0(&src, &mut dst, FunctorOp::GetType));
        assert!(!stub_f1f6c0(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.is_empty());
    }

    #[test]
    fn f1f6cc_manages_tuple_resume_bind() {
        let src = TupleResumeBind {
            weak_fn: 9,
            thread: Some(SharedPtr::new(LuaWeakThreadRef::default())),
        };
        let mut dst = TupleResumeCallback::new();
        assert!(stub_f1f6cc(&src, &mut dst, FunctorOp::Clone));
        assert_eq!(dst.inner.as_ref().unwrap().weak_fn, 9);
        assert!(dst.inner.as_ref().unwrap().thread.is_some());
        assert!(!stub_f1f6cc(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.is_empty());
        assert!(stub_f1f6cc(&src, &mut dst, FunctorOp::CheckType));
        assert!(stub_f1f6cc(&src, &mut dst, FunctorOp::GetType));
    }

    #[test]
    fn f1f6f0_deleting_destructor_drops_item() {
        let before = LUA_SETTINGS_ITEM_DROPS.load(Ordering::SeqCst);
        let item = Box::new(LuaSettingsItem { enabled: true });
        stub_f1f6f0(Box::into_raw(item));
        assert_eq!(LUA_SETTINGS_ITEM_DROPS.load(Ordering::SeqCst), before + 1);
    }

    #[test]
    fn f1f798_calls_string_member() {
        let obj = ScriptSource { source: "game.Workspace".to_owned() };
        let out = stub_f1f798(&obj, |o| o.source.clone());
        match out {
            Variant::Text(s) => assert_eq!(s, "game.Workspace"),
            _ => panic!("expected Text variant"),
        }
    }

    #[test]
    fn f1f7a4_records_hook_event() {
        stub_f1f7a4(std::ptr::null_mut(), &LuaDebug { event: 3, line: 42 });
        assert_eq!(*LAST_HOOK_EVENT.lock(), Some((3, 42)));
    }

    #[test]
    fn f1f900_manages_script_context_resume() {
        let ctx = 0xabc as *const ();
        let src = ScriptContextResumeBind {
            context: ctx,
            thread: Some(SharedPtr::new(LuaWeakThreadRef::default())),
        };
        let mut dst = ScriptContextResumeCallback::new();
        assert!(stub_f1f900(&src, &mut dst, FunctorOp::Clone));
        assert_eq!(dst.inner.as_ref().unwrap().context, ctx);
        assert!(!stub_f1f900(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.is_empty());
        assert!(stub_f1f900(&src, &mut dst, FunctorOp::CheckType));
        assert!(stub_f1f900(&src, &mut dst, FunctorOp::GetType));
    }

    #[test]
    fn f1f90c_manages_lua_state_bind() {
        let src = LuaStateBind { handler: 0x11 as *const () };
        let mut dst = LuaStateCallback::new();
        assert!(stub_f1f90c(&src, &mut dst, FunctorOp::Clone));
        assert_eq!(dst.inner.unwrap().handler, 0x11 as *const ());
        assert!(stub_f1f90c(&src, &mut dst, FunctorOp::Move));
        assert!(!dst.is_empty());
        assert!(!stub_f1f90c(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.is_empty());
        assert!(stub_f1f90c(&src, &mut dst, FunctorOp::CheckType));
        assert!(stub_f1f90c(&src, &mut dst, FunctorOp::GetType));
    }

    #[test]
    fn f1f918_manages_lua_tuple_bind() {
        let src = LuaTupleBind { handler: 0x22 as *const () };
        let mut dst = LuaTupleCallback::new();
        assert!(stub_f1f918(&src, &mut dst, FunctorOp::Clone));
        assert_eq!(dst.inner.unwrap().handler, 0x22 as *const ());
        assert!(!stub_f1f918(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.is_empty());
        assert!(stub_f1f918(&src, &mut dst, FunctorOp::CheckType));
        assert!(stub_f1f918(&src, &mut dst, FunctorOp::GetType));
    }

    #[test]
    fn f1f978_declare_interns_name() {
        let mut map = NameMap::default();
        let a = stub_f1f978(&mut map, "LuaSettingsNsTest");
        let b = stub_f1f978(&mut map, "LuaSettingsNsTest");
        assert!(std::ptr::eq(a, b));
        assert_eq!(map.entries.len(), 1);
    }

    #[test]
    fn f1f984_dodeclare_returns_lua_settings() {
        assert_eq!(stub_f1f984(), "LuaSettings");
    }

    #[test]
    fn f1f9c0_calls_vector_member() {
        let state = ScriptContextState {
            yielded: SharedPtr::new(vec![Variant::Int(7)]),
        };
        let out = stub_f1f9c0(&state, |s| SharedPtr::clone(&s.yielded));
        assert!(SharedPtr::ptr_eq(&out, &state.yielded));
        match &out[0] {
            Variant::Int(i) => assert_eq!(*i, 7),
            _ => panic!("expected Int variant"),
        }
    }

    #[test]
    fn f1fa14_release_drops_one_retain() {
        let h: SharedPtr<LuaWeakThreadRef> = SharedPtr::new(LuaWeakThreadRef::default());
        let n = SharedPtr::strong_count(&h);
        stub_f1fa14(SharedPtr::clone(&h));
        assert_eq!(SharedPtr::strong_count(&h), n);
    }

    #[test]
    fn f1fa20_inserts_waiting_thread() {
        let t: SharedPtr<LuaWeakThreadRef> = SharedPtr::new(LuaWeakThreadRef::default());
        let mut q = vec![WaitingThread { thread: Some(SharedPtr::clone(&t)) }];
        stub_f1fa20(&mut q, 0, WaitingThread { thread: None });
        assert_eq!(q.len(), 2);
        assert!(q[0].thread.is_none());
        assert!(q[1].thread.is_some());
    }

    #[test]
    fn f1fa5c_release_drops_live_handle() {
        let h: SharedPtr<LiveThreadRef> = SharedPtr::new(LiveThreadRef::default());
        let n = SharedPtr::strong_count(&h);
        stub_f1fa5c(SharedPtr::clone(&h));
        assert_eq!(SharedPtr::strong_count(&h), n);
    }
}
