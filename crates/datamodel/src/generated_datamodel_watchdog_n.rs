// Auto-generated datamodel watchdog N — 100 gap filler EA-sorted asc 0x2ca52c..0x2d2554 (RBX::Instance|DataModel|Workspace filter exhausted 10215/10215, gap filler next 100 not in datamodel)

// Source: ida/export.json (85545 funcs) filtered demangled contains RBX::Instance|RBX::DataModel|Workspace — fallback global gap filler (all 10215 datamodel EAs already stubbed, 85545 total, datamodel 33059->33159 distinct, gap 52486->52386 remaining)
// Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + todo!("0xADDR") using rbx_core::SharedPtr

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::{SharedPtr, WeakPtr};
use crate::instance::{AdvLuaDragger, LuaState, ScriptContext, lua_ffi};
use rbx_reflection::generated::Tuple;
use std::ffi::c_void;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

/// Rust model of `RBX::Lua::detail::LiveThreadRef` (IDA `0x2cbc1c`): the
/// intrusive-refcounted Lua thread handle behind `RBX::Lua::ThreadRef`.
/// The atomic add/release collapses into `Arc`; layout unmodeled.
#[derive(Default)]
pub struct LiveThreadRef {
    _opaque: (),
}
/// Rust model of `RBX::Lua::ThreadRef`: `intrusive_ptr<LiveThreadRef>` → `Arc`.
pub type ThreadRef = SharedPtr<LiveThreadRef>;
/// Rust model of `RBX::Lua::IAsyncResult` (IDA `0x2cb958`): async result
/// producing the resume tuple via its conversion operator.
pub struct AsyncResult {
    pub tuple: SharedPtr<Tuple>,
}
/// Rust model of `RBX::AdvLuaDragTool` (IDA `0x2ce804`): the studio drag
/// tool with a `Creatable<MouseCommand>` base; members land with the tool batch.
#[derive(Default)]
pub struct AdvLuaDragTool {
    _opaque: (),
}
/// `boost::detail::sp_counted_impl_pd` deleter tag for
/// `Creatable<MouseCommand>` (IDA `0x2ceac0`); type identity compared at 0x2cead2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseCommandDeleter;
/// `type_info` name compared at IDA `0x2cead2` (cf. core
/// `CREATABLE_INSTANCE_DELETER_TYPE_NAME`).
pub const MOUSE_COMMAND_DELETER_TYPE_NAME: &str =
    "N3RBX9CreatableINS_12MouseCommandEE7DeleterE";
/// `bind_t<void, onAsyncResult(ThreadRef, weak_ptr<ScriptContext>, IAsyncResult*)>`
/// payload (IDA `0x2cd154`): the retained thread plus the context weak link. The
/// `list3`/`storage3`/`storage2` wrappers collapse into this pair (`arg<1>` is a
/// type-level placeholder with no data).
#[derive(Clone)]
pub struct OnAsyncResult {
    pub thread: ThreadRef,
    pub context: WeakPtr<ScriptContext>,
}
/// `boost::function<void ()(IAsyncResult *)>` holding the above (IDA `0x2ccf68`):
/// empty is the cleared state (cf. `FilterCallbackDesc` in generated_10).
#[derive(Clone, Default)]
pub struct AsyncCallback {
    inner: Option<OnAsyncResult>,
}
/// Pending `ScriptContext::scheduleResume` handoff produced by `onAsyncResult`
/// (IDA `0x2cb958`); drained when `0x2a279c` lands.
pub struct PendingResume {
    pub context: SharedPtr<ScriptContext>,
    pub thread: ThreadRef,
    pub args: SharedPtr<Tuple>,
}
/// Resume handoffs queued by `onAsyncResult`, in completion order.
static PENDING_RESUMES: Mutex<Vec<PendingResume>> = Mutex::new(Vec::new());
/// `RBX::Diagnostics::Countable<RBX::Lua::ThreadRef>::count` mirror (IDA `0x2cba44`,
/// `0x2cdc3a`); incremented at `ThreadRef` creation (unmodeled here), decremented
/// on callback completion/destroy.
static THREAD_REF_COUNT: AtomicUsize = AtomicUsize::new(0);
/// `typeinfo` name written by `functor_manager::manage` and compared by `manager`
/// (IDA `0x2cd4d4`, `0x2cdc62`).
pub const ASYNC_BIND_TYPE_NAME: &str = "N5boost3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS2_13ScriptContextEEEPNS3_12IAsyncResultEENS0_5list3INS0_5valueIS4_EENSD_IS7_EENS_3argILi1EEEEEEE";
/// `functor_manager_operation_type` dispatch behind `manager` (IDA `0x2cdb6c`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncFunctorOp {
    Clone = 0,
    Move = 1,
    Destroy = 2,
    Check = 3,
    GetType = 4,
}
/// Sync Lua-callable bridge function behind `lua_pushfunction` (IDA `0x2ca57c`):
/// `boost::function<SharedPtr<Tuple const>()(SharedPtr<Tuple const>)>`.
/// AGENTS.md §4: `boost::function` → closure.
pub type SyncBridgeFn = SharedPtr<dyn Fn(&SharedPtr<Tuple>) -> SharedPtr<Tuple> + Send + Sync>;
/// Async variant behind `lua_pushfunction` (IDA `0x2ca820`):
/// `boost::function<void ()(SharedPtr<Tuple const>, AsyncCallback)>`.
pub type AsyncBridgeFn = SharedPtr<dyn Fn(&SharedPtr<Tuple>, &AsyncCallback) + Send + Sync>;

impl AsyncCallback {
    /// Invokes the stored `onAsyncResult` binding, if any (IDA `0x2cd4f0`/`0x2cd984` shape).
    pub fn invoke(&self, result: &AsyncResult) {
        if let Some(bound) = &self.inner {
            stub_2cb958(bound, result);
        }
    }
}

// 0x2ca52c — __ZN3RBX3Lua16lua_pushfunctionEP9lua_StateRKNS0_15WeakFunctionRefE
#[doc(alias = "RBX::Lua::lua_pushfunction(lua_State *,RBX::Lua::WeakFunctionRef const&)")]
#[doc(alias = "__ZN3RBX3Lua16lua_pushfunctionEP9lua_StateRKNS0_15WeakFunctionRefE")]
pub use rbx_core::generated_core_shard_b::stub_0x2ca52c as stub_2ca52c;

// 0x2ca57c — __ZN3RBX3Lua16lua_pushfunctionEP9lua_StateN5boost10shared_ptrINS3_8functionIFNS4_IKNS_10Reflection5TupleEEES9_EEEEE
#[doc(alias = "RBX::Lua::lua_pushfunction(lua_State *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
#[doc(alias = "__ZN3RBX3Lua16lua_pushfunctionEP9lua_StateN5boost10shared_ptrINS3_8functionIFNS4_IKNS_10Reflection5TupleEEES9_EEEEE")]
// was: RBX::Lua::lua_pushfunction(lua_State *,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)
pub fn stub_2ca57c(state: *mut LuaState, func: &SyncBridgeFn) {
    // IDA 0x2ca57c (decompiled): `lua_pushfunction` (sync) — `shared_ptr` copy
    // (0x2ca5a4..0x2ca5ba), `Bridge<...>::pushNewObject` (0x2ca5e2), temp release
    // (0x2ca5e8..0x2ca5f0), `lua_pushcclosure(callGenericFunctionBridge, 1)` (0x2ca608).
    // The clone is Lua-owned from here (release lands with the userdata `__gc` batch).
    // SAFETY: `state` must be a live `lua_State`.
    unsafe {
        let ud = lua_ffi::lua_newuserdata(state, std::mem::size_of::<SyncBridgeFn>())
            as *mut SyncBridgeFn;
        ud.write(SharedPtr::clone(func));
        lua_ffi::lua_pushcclosure(
            state,
            rbx_core::generated_core_shard_af::stub_0x2ca664 as fn() as *const c_void,
            1,
        );
    }
}

// 0x2ca664 — __ZL25callGenericFunctionBridgeP9lua_State
#[doc(alias = "callGenericFunctionBridge(lua_State *)")]
#[doc(alias = "__ZL25callGenericFunctionBridgeP9lua_State")]
pub use rbx_core::generated_core_shard_af::stub_0x2ca664 as stub_2ca664;

// 0x2ca820 — __ZN3RBX3Lua16lua_pushfunctionEP9lua_StateN5boost10shared_ptrINS3_8functionIFvNS4_IKNS_10Reflection5TupleEEENS5_IFvPNS0_12IAsyncResultEEEEEEEEE
#[doc(alias = "RBX::Lua::lua_pushfunction(lua_State *,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")]
#[doc(alias = "__ZN3RBX3Lua16lua_pushfunctionEP9lua_StateN5boost10shared_ptrINS3_8functionIFvNS4_IKNS_10Reflection5TupleEEENS5_IFvPNS0_12IAsyncResultEEEEEEEEE")]
// was: RBX::Lua::lua_pushfunction(lua_State *,boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)
pub fn stub_2ca820(state: *mut LuaState, func: &AsyncBridgeFn) {
    // IDA 0x2ca820 (decompiled): `lua_pushfunction` (async) — same copy +
    // `pushNewObject` (0x2ca886) + release shape as 0x2ca57c, closing over
    // `callGenericAsyncFunctionBridge` (0x2ca8ac). Same Lua-owned clone discipline.
    // SAFETY: `state` must be a live `lua_State`.
    unsafe {
        let ud = lua_ffi::lua_newuserdata(state, std::mem::size_of::<AsyncBridgeFn>())
            as *mut AsyncBridgeFn;
        ud.write(SharedPtr::clone(func));
        lua_ffi::lua_pushcclosure(
            state,
            rbx_core::generated_core_shard_af::stub_0x2ca908 as fn() as *const c_void,
            1,
        );
    }
}

// 0x2ca908 — __ZL30callGenericAsyncFunctionBridgeP9lua_State
#[doc(alias = "callGenericAsyncFunctionBridge(lua_State *)")]
#[doc(alias = "__ZL30callGenericAsyncFunctionBridgeP9lua_State")]
pub use rbx_core::generated_core_shard_af::stub_0x2ca908 as stub_2ca908;

// 0x2cad6c — __ZN3RBX3Lua15WeakFunctionRefD0Ev
#[doc(alias = "RBX::Lua::WeakFunctionRef::~WeakFunctionRef()")]
#[doc(alias = "__ZN3RBX3Lua15WeakFunctionRefD0Ev")]
pub use rbx_reflection::generated_refl_wd_10p::stub_2cad6c as stub_2cad6c;

// 0x2cae0c — __ZN3RBX3Lua15WeakFunctionRefD1Ev
#[doc(alias = "RBX::Lua::WeakFunctionRef::~WeakFunctionRef()")]
#[doc(alias = "__ZN3RBX3Lua15WeakFunctionRefD1Ev")]
pub use rbx_reflection::generated_refl_wd_10p::stub_2cae0c as stub_2cae0c;

// 0x2cae10 — __ZN3RBX3Lua15WeakFunctionRefD2Ev
#[doc(alias = "RBX::Lua::WeakFunctionRef::~WeakFunctionRef()")]
#[doc(alias = "__ZN3RBX3Lua15WeakFunctionRefD2Ev")]
pub use rbx_reflection::generated_refl_wd_10p::stub_2cae10 as stub_2cae10;

// 0x2caf24 — __ZN3RBX3Lua15WeakFunctionRef9removeRefEv
#[doc(alias = "RBX::Lua::WeakFunctionRef::removeRef(void)")]
#[doc(alias = "__ZN3RBX3Lua15WeakFunctionRef9removeRefEv")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2caf24 as stub_2caf24;

// 0x2caf98 — __ZN3RBX3Lua15WeakFunctionRefC1ERKS1_
#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(RBX::Lua::WeakFunctionRef const&)")]
#[doc(alias = "__ZN3RBX3Lua15WeakFunctionRefC1ERKS1_")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2caf98 as stub_2caf98;

// 0x2caf9c — __ZN3RBX3Lua15WeakFunctionRefC2ERKS1_
#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(RBX::Lua::WeakFunctionRef const&)")]
#[doc(alias = "__ZN3RBX3Lua15WeakFunctionRefC2ERKS1_")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2caf9c as stub_2caf9c;

// 0x2cb0fc — __ZN3RBX3Lua6detail13LiveThreadRefC2EP9lua_State
#[doc(alias = "RBX::Lua::detail::LiveThreadRef::LiveThreadRef(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6detail13LiveThreadRefC2EP9lua_State")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2cb0fc as stub_2cb0fc;

// 0x2cb2ec — __ZN3RBX3Lua6detail13LiveThreadRefD1Ev
#[doc(alias = "RBX::Lua::detail::LiveThreadRef::~LiveThreadRef()")]
#[doc(alias = "__ZN3RBX3Lua6detail13LiveThreadRefD1Ev")]
pub use rbx_reflection::generated_refl_wd_10p::stub_2cb2ec as stub_2cb2ec;

// 0x2cb2f0 — __ZN3RBX3Lua6detail13LiveThreadRefD2Ev
#[doc(alias = "RBX::Lua::detail::LiveThreadRef::~LiveThreadRef()")]
#[doc(alias = "__ZN3RBX3Lua6detail13LiveThreadRefD2Ev")]
pub use rbx_reflection::generated_refl_wd_10p::stub_2cb2f0 as stub_2cb2f0;

// 0x2cb3fc — __ZN3RBX3Lua15WeakFunctionRefaSERKS1_
#[doc(alias = "RBX::Lua::WeakFunctionRef::operator=(RBX::Lua::WeakFunctionRef const&)")]
#[doc(alias = "__ZN3RBX3Lua15WeakFunctionRefaSERKS1_")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2cb3fc as stub_2cb3fc;

// 0x2cb4d0 — __ZN3RBX10Reflection4Type12getSingletonINS_3Lua15WeakFunctionRefEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Lua::WeakFunctionRef>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_3Lua15WeakFunctionRefEEERKS1_v")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2cb4d0 as stub_2cb4d0;

// 0x2cb5b4 — __ZN3RBX10Reflection7Variant7convertINS_3Lua15WeakFunctionRefEEERT_v
#[doc(alias = "RBX::Lua::WeakFunctionRef & RBX::Reflection::Variant::convert<RBX::Lua::WeakFunctionRef>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant7convertINS_3Lua15WeakFunctionRefEEERT_v")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2cb5b4 as stub_2cb5b4;

// 0x2cb874 — __ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrINS3_8functionIFvNS4_IKNS0_5TupleEEENS5_IFvPNS_3Lua12IAsyncResultEEEEEEEEEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrINS3_8functionIFvNS4_IKNS0_5TupleEEENS5_IFvPNS_3Lua12IAsyncResultEEEEEEEEEEERKS1_v")]
// was: RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(void)
pub use rbx_reflection::generated_shard_fv::stub_0x2cb874 as stub_2cb874;

// 0x2cb958 — __ZL13onAsyncResultN3RBX3Lua9ThreadRefEN5boost8weak_ptrINS_13ScriptContextEEEPNS0_12IAsyncResultE
#[doc(alias = "onAsyncResult(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *)")]
#[doc(alias = "__ZL13onAsyncResultN3RBX3Lua9ThreadRefEN5boost8weak_ptrINS_13ScriptContextEEEPNS0_12IAsyncResultE")]
pub fn stub_2cb958(bound: &OnAsyncResult, result: &AsyncResult) {
    // IDA 0x2cb958 (decompiled): `onAsyncResult` — the `IAsyncResult` conversion
    // operator builds `shared_ptr<Tuple const>` (0x2cb9b8..0x2cb9c2), the weak
    // `ScriptContext` upgrades to shared (0x2cb9da); expired → release + return.
    // Live: addref the `ThreadRef` (0x2cb9ea..0x2cb9f2),
    // `ScriptContext::scheduleResume(ctx, thread, tuple)` (0x2cba18), release temps
    // (0x2cba1e..0x2cba30), `Diagnostics::Countable<ThreadRef>::count` decrement
    // (0x2cba44). The resume handoff queues into `PENDING_RESUMES` until 0x2a279c lands.
    let args = SharedPtr::clone(&result.tuple);
    let Some(context) = bound.context.upgrade() else { return; };
    let thread = SharedPtr::clone(&bound.thread);
    PENDING_RESUMES.lock().push(PendingResume {
        context,
        thread: SharedPtr::clone(&thread),
        args,
    });
    THREAD_REF_COUNT.fetch_sub(1, Ordering::Relaxed);
}

// 0x2cbc1c — __ZN5boost13intrusive_ptrIN3RBX3Lua6detail13LiveThreadRefEEaSEPS4_
#[doc(alias = "boost::intrusive_ptr<RBX::Lua::detail::LiveThreadRef>::operator=(RBX::Lua::detail::LiveThreadRef*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3RBX3Lua6detail13LiveThreadRefEEaSEPS4_")]
pub fn stub_2cbc1c(slot: &mut ThreadRef, value: &ThreadRef) {
    // IDA 0x2cbc1c (decompiled): `intrusive_ptr<LiveThreadRef>::operator=` —
    // addref the new value first (`OSAtomicAdd32`, 0x2cbc26..0x2cbc2c), store it
    // (0x2cbc30..0x2cbc32), release the old (0x2cbc36..0x2cbc38). Arc move-assign
    // clones before dropping, which is the same order. Same shape as core boost carriers.
    *slot = SharedPtr::clone(value);
}

// 0x2cbc40 — __ZN16RobloxExtraSpace13createNewNodeEv
#[doc(alias = "RobloxExtraSpace::createNewNode(void)")]
#[doc(alias = "__ZN16RobloxExtraSpace13createNewNodeEv")]
pub use rbx_core::generated_core_shard_af::stub_0x2cbc40 as stub_2cbc40;

// 0x2cbd58 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE13pushNewObjectISB_EEPSB_P9lua_StateT_
#[doc(alias = "rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>* RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::pushNewObject<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(lua_State *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE13pushNewObjectISB_EEPSB_P9lua_StateT_")]
// was: boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>* RBX::Lua::Bridge<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,true>::pushNewObject<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>(lua_State *,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)
pub use rbx_reflection::generated_shard_fv::stub_0x2cbd58 as stub_2cbd58;

// 0x2cbda8 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE13pushNewObjectISF_EEPSF_P9lua_StateT_
#[doc(alias = "rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>* RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::pushNewObject<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(lua_State *,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE13pushNewObjectISF_EEPSF_P9lua_StateT_")]
// was: boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>* RBX::Lua::Bridge<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::pushNewObject<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(lua_State *,boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)
pub use rbx_reflection::generated_shard_fv::stub_0x2cbda8 as stub_2cbda8;

// 0x2cbdf8 — __ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED1Ev
#[doc(alias = "RBX::Reflection::TType<RBX::Lua::WeakFunctionRef>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED1Ev")]
pub use rbx_reflection::generated_refl_wd_10t::stub_0x2cbdf8 as stub_2cbdf8;

// 0x2cbdfc — __ZN3rbx8any_castIN3RBX3Lua15WeakFunctionRefENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Lua::WeakFunctionRef * rbx::any_cast<RBX::Lua::WeakFunctionRef,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX3Lua15WeakFunctionRefENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2cbdfc as stub_2cbdfc;

// 0x2cbe54 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS0_5TupleEEENS4_IFvPNS_3Lua12IAsyncResultEEEEEEEEEED1Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS0_5TupleEEENS4_IFvPNS_3Lua12IAsyncResultEEEEEEEEEED1Ev")]
// was: RBX::Reflection::TType<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>::~TType()
pub use rbx_reflection::generated_refl_wd_10t::stub_0x2cbe54 as stub_2cbe54;

// 0x2cbe58 — __ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrINS3_8functionIFvNS4_IKNS0_5TupleEEENS5_IFvPNS_3Lua12IAsyncResultEEEEEEEEEEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(char const*,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> *)")]
#[doc(alias = "__ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrINS3_8functionIFvNS4_IKNS0_5TupleEEENS5_IFvPNS_3Lua12IAsyncResultEEEEEEEEEEEPKcPT_")]
// was: RBX::Reflection::Type::Type<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(char const*,boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> *)
pub use rbx_reflection::generated_shard_fv::stub_0x2cbe58 as stub_2cbe58;

// 0x2cbf04 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS0_5TupleEEENS4_IFvPNS_3Lua12IAsyncResultEEEEEEEEEED0Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS0_5TupleEEENS4_IFvPNS_3Lua12IAsyncResultEEEEEEEEEED0Ev")]
// was: RBX::Reflection::TType<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>::~TType()
pub use rbx_reflection::generated_refl_wd_10t::stub_0x2cbf04 as stub_2cbf04;

// 0x2cbfb8 — __ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE9singletonEv")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2cbfb8 as stub_2cbfb8;

// 0x2cc020 — __ZN3RBX10Reflection4TypeC2INS_3Lua15WeakFunctionRefEEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Lua::WeakFunctionRef>(char const*,RBX::Lua::WeakFunctionRef *)")]
#[doc(alias = "__ZN3RBX10Reflection4TypeC2INS_3Lua15WeakFunctionRefEEEPKcPT_")]
pub use rbx_reflection::generated_shard_fv::stub_0x2cc020 as stub_2cc020;

// 0x2cc0c8 — __ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED0Ev
#[doc(alias = "RBX::Reflection::TType<RBX::Lua::WeakFunctionRef>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED0Ev")]
pub use rbx_reflection::generated_refl_wd_10t::stub_0x2cc0c8 as stub_2cc0c8;

// 0x2cc0cc — __ZNK5boost9function2IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEENS_8functionIFvPNS2_3Lua12IAsyncResultEEEEEclES6_SC_
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>>::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)const")]
#[doc(alias = "__ZNK5boost9function2IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEENS_8functionIFvPNS2_3Lua12IAsyncResultEEEEEclES6_SC_")]
// was: boost::function2<void,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>>::operator()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)const
pub use rbx_reflection::generated_shard_fv::stub_0x2cc0cc as stub_2cc0cc;

// 0x2cc210 — __ZN5boost4bindIvN3RBX3Lua9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEEPNS2_12IAsyncResultES3_S6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list_av_3<RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,boost::arg<1>>::type> boost::bind<void,RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *,RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,boost::arg<1>>(void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX3Lua9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEEPNS2_12IAsyncResultES3_S6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_")]
pub fn stub_2cc210(thread: &ThreadRef, context: &WeakPtr<ScriptContext>) -> OnAsyncResult {
    // IDA 0x2cc210 (decompiled): `bind<void, ThreadRef, weak_ptr<ScriptContext>,
    // IAsyncResult*, ...>` — copies the thread (spinlock addref) and the weak link
    // into the `bind_t` payload. `Arc`/`Weak` clones are the same copies.
    OnAsyncResult { thread: SharedPtr::clone(thread), context: WeakPtr::clone(context) }
}

// 0x2cc608 — __ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEE5clearEv
#[doc(alias = "boost::function1<void,RBX::Lua::IAsyncResult *>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEE5clearEv")]
pub fn stub_2cc608(callback: &mut AsyncCallback) {
    // IDA 0x2cc608 (decompiled): `function1<void, IAsyncResult*>::clear` — runs the
    // vtable destroy on the held functor when present (0x2cc61a..0x2cc62c), then zeroes
    // the slot (0x2cc630). Dropping the `Option` runs the same teardown.
    callback.inner = None;
}

// 0x2cc634 — __ZN5boost3_bi5valueIN3RBX3Lua9ThreadRefEEC2ERKS4_
#[doc(alias = "boost::_bi::value<RBX::Lua::ThreadRef>::value(RBX::Lua::ThreadRef const&)")]
#[doc(alias = "__ZN5boost3_bi5valueIN3RBX3Lua9ThreadRefEEC2ERKS4_")]
pub fn stub_2cc634(value: &ThreadRef) -> ThreadRef {
    // IDA 0x2cc634 (decompiled): `_bi::value<ThreadRef>::value` — copies the
    // intrusive link (`OSAtomicAdd32`, 0x2cc680..0x2cc68a). Same as 0x2cccc4.
    SharedPtr::clone(value)
}

// 0x2cc6f0 — __ZN5boost3_bi5list3INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEENS_3argILi1EEEEC2ES6_SA_SC_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>::list3(boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEENS_3argILi1EEEEC2ES6_SA_SC_")]
pub fn stub_2cc6f0(thread: &ThreadRef, context: &WeakPtr<ScriptContext>) -> OnAsyncResult {
    // IDA 0x2cc6f0 (decompiled): `list3<value<ThreadRef>, value<weak_ptr<ScriptContext>>,
    // arg<1>>::list3` — stores the two bound values (`arg<1>` is type-level, no data;
    // 0x2cc720..). Collapses into the same pair as the `bind_t` (cf. 0x2cc210).
    OnAsyncResult { thread: SharedPtr::clone(thread), context: WeakPtr::clone(context) }
}

// 0x2cc8d0 — __ZN5boost3_bi8storage3INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEENS_3argILi1EEEEC2ES6_SA_SC_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>::storage3(boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEENS_3argILi1EEEEC2ES6_SA_SC_")]
pub fn stub_2cc8d0(thread: &ThreadRef, context: &WeakPtr<ScriptContext>) -> OnAsyncResult {
    // IDA 0x2cc8d0 (decompiled): `storage3<...>::storage3` — same two-value store as
    // the `list3` it wraps (0x2cc900..). Same collapse as 0x2cc6f0.
    OnAsyncResult { thread: SharedPtr::clone(thread), context: WeakPtr::clone(context) }
}

// 0x2ccab0 — __ZN5boost3_bi8storage2INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEEEC2ES6_SA_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>>::storage2(boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEEEC2ES6_SA_")]
pub fn stub_2ccab0(thread: &ThreadRef, context: &WeakPtr<ScriptContext>) -> OnAsyncResult {
    // IDA 0x2ccab0 (decompiled): `storage2<value<ThreadRef>, value<weak_ptr<ScriptContext>>>`
    // — addrefs the thread (0x2ccafe..0x2ccb08), stores both, releases the temp (0x2ccb1a..).
    // The stored content is exactly the `OnAsyncResult` pair.
    OnAsyncResult { thread: SharedPtr::clone(thread), context: WeakPtr::clone(context) }
}

// 0x2cccc4 — __ZN5boost3_bi8storage1INS0_5valueIN3RBX3Lua9ThreadRefEEEEC2ES6_
#[doc(alias = "boost::_bi::storage1<boost::_bi::value<RBX::Lua::ThreadRef>>::storage1(boost::_bi::value<RBX::Lua::ThreadRef>)")]
#[doc(alias = "__ZN5boost3_bi8storage1INS0_5valueIN3RBX3Lua9ThreadRefEEEEC2ES6_")]
pub fn stub_2cccc4(value: &ThreadRef) -> ThreadRef {
    // IDA 0x2cccc4 (decompiled): `storage1<value<ThreadRef>>::storage1` — same
    // intrusive copy as 0x2cc634 (0x2cccea..0x2ccd1a).
    SharedPtr::clone(value)
}

// 0x2ccd80 — __ZN5boost8functionIFvPN3RBX3Lua12IAsyncResultEEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS8_5list3INS8_5valueISA_EENSH_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvPN3RBX3Lua12IAsyncResultEEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS8_5list3INS8_5valueISA_EENSH_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvPN3RBX3Lua12IAsyncResultEEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS8_5list3INS8_5valueISA_EENSH_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2ccd80(bound: &OnAsyncResult) -> AsyncCallback {
    // IDA 0x2ccd80 (decompiled): `function<void ()(IAsyncResult*)>::function(bind_t)` —
    // wraps the bound payload into the function object. Same wrap as 0x2ccf68.
    AsyncCallback { inner: Some(bound.clone()) }
}

// 0x2ccf68 — __ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS7_5list3INS7_5valueIS9_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS7_5list3INS7_5valueIS9_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS7_5list3INS7_5valueIS9_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2ccf68(bound: &OnAsyncResult) -> AsyncCallback {
    // IDA 0x2ccf68 (decompiled): `function1<void, IAsyncResult*>::function1(bind_t)` —
    // same payload wrap as 0x2ccd80.
    AsyncCallback { inner: Some(bound.clone()) }
}

// 0x2cd154 — __ZN5boost3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS2_13ScriptContextEEEPNS3_12IAsyncResultEENS0_5list3INS0_5valueIS4_EENSD_IS7_EENS_3argILi1EEEEEEC2ERKSJ_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>>::bind_t(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS2_13ScriptContextEEEPNS3_12IAsyncResultEENS0_5list3INS0_5valueIS4_EENSD_IS7_EENS_3argILi1EEEEEEC2ERKSJ_")]
pub fn stub_2cd154(other: &OnAsyncResult) -> OnAsyncResult {
    // IDA 0x2cd154 (decompiled): `bind_t::bind_t` copy — copies the function pointer
    // words (0x2cd180..0x2cd21c), addrefs the thread (`OSAtomicAdd32`, 0x2cd1aa..0x2cd1b4)
    // and bumps the weak count under the spinlock (0x2cd1c6..0x2cd20c). Clone is the same.
    other.clone()
}

// 0x2cd2dc — __ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEE9assign_toINS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS7_5list3INS7_5valueIS9_EENSG_ISC_EENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::Lua::IAsyncResult *>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEE9assign_toINS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS7_5list3INS7_5valueIS9_EENSG_ISC_EENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_2cd2dc(slot: &mut AsyncCallback, bound: &OnAsyncResult) {
    // IDA 0x2cd2dc (decompiled): `function1::assign_to(bind_t)` — replaces the held
    // functor with a copy of the payload. Same replace as the vtable twins 0x2cd50c/0x2cd6f4.
    slot.inner = Some(bound.clone());
}

// 0x2cd4d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
pub fn stub_2cd4d4() -> &'static str {
    // IDA 0x2cd4d4 (decompiled): `functor_manager<...>::manage` — for the get-type op
    // (`a3 == 4`, 0x2cd4d6) writes the `bind_t` `typeinfo` into the slot (0x2cd4ea..0x2cd4ec);
    // other ops delegate to `manager` (0x2cd4d8, i.e. 0x2cdb6c). The typeinfo identity
    // is the mangled name (cf. the `strcmp` at 0x2cdc62).
    ASYNC_BIND_TYPE_NAME
}

// 0x2cd4f0 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>>,void,RBX::Lua::IAsyncResult *>::invoke(boost::detail::function::function_buffer &,RBX::Lua::IAsyncResult *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_")]
pub fn stub_2cd4f0(callback: &AsyncCallback, result: &AsyncResult) {
    // IDA 0x2cd4f0 (disasm): `void_function_obj_invoker1<...>::invoke` — builds the
    // one-arg `list1` and tail-calls `list3::operator()` (0x2cd502, i.e. 0x2cd984).
    // Invoking the callback is the same call.
    callback.invoke(result);
}

// 0x2cd50c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX3Lua12IAsyncResultEE9assign_toINS_3_bi6bind_tIvPFvNS4_9ThreadRefENS_8weak_ptrINS3_13ScriptContextEEES6_ENS9_5list3INS9_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Lua::IAsyncResult *>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX3Lua12IAsyncResultEE9assign_toINS_3_bi6bind_tIvPFvNS4_9ThreadRefENS_8weak_ptrINS3_13ScriptContextEEES6_ENS9_5list3INS9_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_2cd50c(slot: &mut AsyncCallback, bound: &OnAsyncResult) {
    // IDA 0x2cd50c (decompiled): `basic_vtable1::assign_to(bind_t)` (same-type) — copies
    // the payload into the function buffer. Same replace as 0x2cd2dc.
    slot.inner = Some(bound.clone());
}

// 0x2cd6f4 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX3Lua12IAsyncResultEE9assign_toINS_3_bi6bind_tIvPFvNS4_9ThreadRefENS_8weak_ptrINS3_13ScriptContextEEES6_ENS9_5list3INS9_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Lua::IAsyncResult *>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX3Lua12IAsyncResultEE9assign_toINS_3_bi6bind_tIvPFvNS4_9ThreadRefENS_8weak_ptrINS3_13ScriptContextEEES6_ENS9_5list3INS9_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_2cd6f4(slot: &mut AsyncCallback, bound: &OnAsyncResult) {
    // IDA 0x2cd6f4 (decompiled): `basic_vtable1::assign_to(bind_t)` (`function_obj_tag`
    // overload) — same buffer copy as 0x2cd50c; the tag only selects the overload.
    slot.inner = Some(bound.clone());
}

// 0x2cd8d8 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX3Lua12IAsyncResultEE14assign_functorINS_3_bi6bind_tIvPFvNS4_9ThreadRefENS_8weak_ptrINS3_13ScriptContextEEES6_ENS9_5list3INS9_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::Lua::IAsyncResult *>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX3Lua12IAsyncResultEE14assign_functorINS_3_bi6bind_tIvPFvNS4_9ThreadRefENS_8weak_ptrINS3_13ScriptContextEEES6_ENS9_5list3INS9_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_2cd8d8(other: &OnAsyncResult) -> Box<OnAsyncResult> {
    // IDA 0x2cd8d8 (decompiled): `basic_vtable1::assign_functor(bind_t)` — heap-allocates
    // (`operator new(0x10)`, 0x2cd90e) and copy-constructs the payload into it (0x2cd934).
    // `Box::new` on the clone is the same heap copy.
    Box::new(other.clone())
}

// 0x2cd984 — __ZN5boost3_bi5list3INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEENS_3argILi1EEEEclIPFvS5_S9_PNS4_12IAsyncResultEENS0_5list1IRSG_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>::operator()<void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list1<RBX::Lua::IAsyncResult *&>>(boost::_bi::type<void>,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *) &,boost::_bi::list1<RBX::Lua::IAsyncResult *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEENS_3argILi1EEEEclIPFvS5_S9_PNS4_12IAsyncResultEENS0_5list1IRSG_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_2cd984(bound: &OnAsyncResult, result: &AsyncResult) {
    // IDA 0x2cd984 (decompiled): `list3<...>::operator()` — invokes the stored
    // `onAsyncResult` with the bound thread + weak context and the `IAsyncResult`
    // argument. That call is `stub_0x2cb958`.
    stub_2cb958(bound, result);
}

// 0x2cdb6c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,boost::weak_ptr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<boost::weak_ptr<RBX::ScriptContext>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_2cdb6c(slot: &mut AsyncCallback, other: &AsyncCallback, op: AsyncFunctorOp) {
    // IDA 0x2cdb6c (decompiled): `functor_manager<...>::manager` — dispatches the
    // `functor_manager_operation_type`: 0 clones (`new` + `bind_t` copy, 0x2cdbde..),
    // 2 destroys (weak release + intrusive release + `Countable<ThreadRef>` decrement +
    // `delete`, 0x2cdc08..0x2cdc40), 3 checks the `typeinfo` name (`strcmp`, 0x2cdc62..0x2cdc6c;
    // single monomorph, always matches), default writes the `typeinfo` (0x2cdbca..0x2cdbcc).
    // Move (1) is clone-shaped under `Arc` (clone-then-drop-source is unobservable here).
    match op {
        AsyncFunctorOp::Clone | AsyncFunctorOp::Move => *slot = other.clone(),
        AsyncFunctorOp::Destroy => {
            *slot = AsyncCallback::default();
            THREAD_REF_COUNT.fetch_sub(1, Ordering::Relaxed);
        }
        AsyncFunctorOp::Check | AsyncFunctorOp::GetType => {}
    }
}

// 0x2cdd44 — __ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEE13assign_to_ownERKS5_
#[doc(alias = "boost::function1<void,RBX::Lua::IAsyncResult *>::assign_to_own(boost::function1<void,RBX::Lua::IAsyncResult *> const&)")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEE13assign_to_ownERKS5_")]
pub fn stub_2cdd44(slot: &mut AsyncCallback, other: &AsyncCallback) {
    // IDA 0x2cdd44 (decompiled): `function1::assign_to_own` — empty source copies the
    // words (0x2cdd44..0x2cdd4a); small (in-buffer) functors copy the buffer
    // (0x2cdd52..0x2cdd5c), heap ones clone through the vtable (0x2cdd72). Either way
    // the destination ends up an equal copy: a clone.
    *slot = other.clone();
}

// 0x2cde88 — __GLOBAL__I_a_75
#[doc(alias = "global constructor keyed to_a_75")]
#[doc(alias = "__GLOBAL__I_a_75")]
pub use rbx_core::generated_core_shard_af::stub_0x2cde88 as stub_2cde88;

// 0x2ce130 — __ZN3RBX8Security7Context8isInRoleENS0_10IdentitiesENS0_11PermissionsE
#[doc(alias = "RBX::Security::Context::isInRole(RBX::Security::Identities,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX8Security7Context8isInRoleENS0_10IdentitiesENS0_11PermissionsE")]
pub use rbx_core::generated_core_shard_af::stub_0x2ce130 as stub_2ce130;

// 0x2ce1fc — __GLOBAL__I_a_76
#[doc(alias = "global constructor keyed to_a_76")]
#[doc(alias = "__GLOBAL__I_a_76")]
pub use rbx_core::generated_core_shard_af::stub_0x2ce1fc as stub_2ce1fc;

// 0x2ce804 — __ZN5boost10shared_ptrIN3RBX14AdvLuaDragToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragTool>::shared_ptr<RBX::AdvLuaDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14AdvLuaDragToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::AdvLuaDragTool>::shared_ptr<RBX::AdvLuaDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_2ce804(tool: Box<AdvLuaDragTool>) -> SharedPtr<AdvLuaDragTool> {
    // IDA 0x2ce804 (decompiled): `shared_ptr<AdvLuaDragTool>(ptr, Creatable<MouseCommand>::Deleter)` —
    // stores the pointer (0x2ce824), builds the `sp_counted_impl_pd` control block (0x2ce82c,
    // i.e. 0x2ce9b0), then `enable_shared_from_this<MouseCommand>::_internal_accept_owner`
    // (0x2ce85a..0x2ce86a). `Box`→`Arc` adoption is the same single-owner handoff
    // (cf. `shared_ptr_from_raw`); the weak accept collapses like reflection 0x31a10.
    rbx_core::shared_ptr::shared_ptr_from_raw(tool)
}

// 0x2ce8cc — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_14AdvLuaDragToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvLuaDragTool,RBX::AdvLuaDragTool>(rbx_core::SharedPtr<RBX::AdvLuaDragTool> const*,RBX::AdvLuaDragTool *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_14AdvLuaDragToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvLuaDragTool,RBX::AdvLuaDragTool>(boost::shared_ptr<RBX::AdvLuaDragTool> const*,RBX::AdvLuaDragTool *)const
pub use rbx_reflection::generated_refl_wd_10p::stub_2ce8cc as stub_2ce8cc;

// 0x2ce9b0 — __ZN5boost6detail12shared_countC2IPN3RBX14AdvLuaDragToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX14AdvLuaDragToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")]
pub fn stub_2ce9b0() {
    // IDA 0x2ce9b0 (decompiled): `shared_count<AdvLuaDragTool*, Creatable<MouseCommand>::Deleter>` —
    // fresh `sp_counted_impl_pd` block (`new(0x14)`, counts 1/1, vtable, deleter; 0x2cea04..0x2cea24).
    // The block only exists inside the `Arc` produced by 0x2ce804, so a standalone call
    // has no observable body left; same carrier doctrine as core monomorph artifacts.
}

// 0x2ceaa8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")]
pub use rbx_reflection::generated_refl_wd_10p::stub_2ceaa8 as stub_2ceaa8;

// 0x2ceaac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2ceaac as stub_2ceaac;

// 0x2ceab0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")]
pub fn stub_2ceab0(slot: &mut Option<Box<AdvLuaDragTool>>) {
    // IDA 0x2ceab0 (decompiled): `sp_counted_impl_pd<...>::dispose` — when the payload
    // is present, runs the deleter through the vtable (0x2ceabc). `Option::take` + drop
    // is exactly dtor-then-free, skipped when null (cf. core `ControlBlockP::dispose`).
    slot.take();
}

// 0x2ceac0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_2ceac0(type_name: &str) -> Option<MouseCommandDeleter> {
    // IDA 0x2ceac0 (decompiled): `sp_counted_impl_pd<...>::get_deleter` — returns the
    // deleter at block+16 (0x2ceac4) when the queried `type_info` name matches
    // (0x2cead2), else null (0x2cead4). Same shape as core `get_deleter(type_name)`.
    (type_name == MOUSE_COMMAND_DELETER_TYPE_NAME).then_some(MouseCommandDeleter)
}

// 0x2cead8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_2cead8() -> MouseCommandDeleter {
    // IDA 0x2cead8 (decompiled): `sp_counted_impl_pd<...>::get_untyped_deleter` —
    // unconditionally returns the deleter at block+16 (0x2ceada).
    MouseCommandDeleter
}

// 0x2ceadc — __GLOBAL__I_a_77
#[doc(alias = "global constructor keyed to_a_77")]
#[doc(alias = "__GLOBAL__I_a_77")]
pub use rbx_core::generated_core_shard_af::stub_0x2ceadc as stub_2ceadc;

// 0x2ced4c — __ZN3RBX13AdvLuaDraggerC2Ev
#[doc(alias = "RBX::AdvLuaDragger::AdvLuaDragger(void)")]
#[doc(alias = "__ZN3RBX13AdvLuaDraggerC2Ev")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2ced4c as stub_2ced4c;

// 0x2cef40 — __ZN3RBX13AdvLuaDraggerD0Ev
#[doc(alias = "RBX::AdvLuaDragger::~AdvLuaDragger()")]
#[doc(alias = "__ZN3RBX13AdvLuaDraggerD0Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2cef40 as stub_2cef40;

// 0x2cefe0 — __ZN3RBX13AdvLuaDraggerD1Ev
#[doc(alias = "RBX::AdvLuaDragger::~AdvLuaDragger()")]
#[doc(alias = "__ZN3RBX13AdvLuaDraggerD1Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2cefe0 as stub_2cefe0;

// 0x2cefe4 — __ZThn32_N3RBX13AdvLuaDraggerD0Ev
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
#[doc(alias = "__ZThn32_N3RBX13AdvLuaDraggerD0Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2cefe4 as stub_2cefe4;

// 0x2cefec — __ZThn36_N3RBX13AdvLuaDraggerD0Ev
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
#[doc(alias = "__ZThn36_N3RBX13AdvLuaDraggerD0Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2cefec as stub_2cefec;

// 0x2ceff4 — __ZN3RBX13AdvLuaDraggerD2Ev
#[doc(alias = "RBX::AdvLuaDragger::~AdvLuaDragger()")]
#[doc(alias = "__ZN3RBX13AdvLuaDraggerD2Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2ceff4 as stub_2ceff4;

// 0x2cf168 — __ZThn32_N3RBX13AdvLuaDraggerD1Ev
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
#[doc(alias = "__ZThn32_N3RBX13AdvLuaDraggerD1Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2cf168 as stub_2cf168;

// 0x2cf170 — __ZThn36_N3RBX13AdvLuaDraggerD1Ev
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
#[doc(alias = "__ZThn36_N3RBX13AdvLuaDraggerD1Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2cf170 as stub_2cf170;

// 0x2cf3b8 — __ZN3RBX13AdvLuaDragger9mouseMoveENS_6RbxRayE
#[doc(alias = "RBX::AdvLuaDragger::mouseMove(RBX::RbxRay)")]
#[doc(alias = "__ZN3RBX13AdvLuaDragger9mouseMoveENS_6RbxRayE")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2cf3b8 as stub_2cf3b8;

// 0x2cf6d0 — __ZN3RBX13AdvLuaDragger16tryStartDraggingERKNS_6RbxRayE
#[doc(alias = "RBX::AdvLuaDragger::tryStartDragging(RBX::RbxRay const&)")]
#[doc(alias = "__ZN3RBX13AdvLuaDragger16tryStartDraggingERKNS_6RbxRayE")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2cf6d0 as stub_2cf6d0;

// 0x2cf930 — __ZN3RBX13AdvLuaDragger6doDragERKNS_6RbxRayE
#[doc(alias = "RBX::AdvLuaDragger::doDrag(RBX::RbxRay const&)")]
#[doc(alias = "__ZN3RBX13AdvLuaDragger6doDragERKNS_6RbxRayE")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2cf930 as stub_2cf930;

// 0x2cfd7c — __ZN3RBX13AdvLuaDragger7mouseUpEv
#[doc(alias = "RBX::AdvLuaDragger::mouseUp(void)")]
#[doc(alias = "__ZN3RBX13AdvLuaDragger7mouseUpEv")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2cfd7c as stub_2cfd7c;

// 0x2d0154 — __ZN3RBX13AdvLuaDragger13startDraggingEv
#[doc(alias = "RBX::AdvLuaDragger::startDragging(void)")]
#[doc(alias = "__ZN3RBX13AdvLuaDragger13startDraggingEv")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2d0154 as stub_2d0154;

// 0x2d03b0 — __ZN3RBX13AdvLuaDragger16rotateOnSnapFaceEN3G3D7Vector34AxisERKNS1_7Matrix3E
#[doc(alias = "RBX::AdvLuaDragger::rotateOnSnapFace(G3D::Vector3::Axis,G3D::Matrix3 const&)")]
#[doc(alias = "__ZN3RBX13AdvLuaDragger16rotateOnSnapFaceEN3G3D7Vector34AxisERKNS1_7Matrix3E")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2d03b0 as stub_2d03b0;

// 0x2d072c — __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEE5resetEPS1_
#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::reset(RBX::AdvRunDragger*)")]
#[doc(alias = "__ZNSt8auto_ptrIN3RBX13AdvRunDraggerEE5resetEPS1_")]
pub use rbx_core::generated_core_shard_af::stub_0x2d072c as stub_2d072c;

// 0x2d0be8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13AdvLuaDraggerES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AdvLuaDragger,RBX::AdvLuaDragger>(rbx_core::SharedPtr<RBX::AdvLuaDragger> const*,RBX::AdvLuaDragger *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13AdvLuaDraggerES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AdvLuaDragger,RBX::AdvLuaDragger>(boost::shared_ptr<RBX::AdvLuaDragger> const*,RBX::AdvLuaDragger *)const
pub use rbx_reflection::generated_refl_wd_10t::stub_0x2d0be8 as stub_2d0be8;

// 0x2d0e1c — __ZN3RBX4Name13callDoDeclareILZNS_14sAdvLuaDraggerEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sAdvLuaDraggerEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sAdvLuaDraggerEEEEvv")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2d0e1c as stub_2d0e1c;

// 0x2d0e20 — __ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v")]
pub use rbx_core::generated_core_watchdog_k::stub_0x2d0e20 as stub_2d0e20;

// 0x2d11b8 — __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEED2Ev
#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::~auto_ptr()")]
#[doc(alias = "__ZNSt8auto_ptrIN3RBX13AdvRunDraggerEED2Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2d11b8 as stub_2d11b8;

// 0x2d145c — __GLOBAL__I_a_78
#[doc(alias = "global constructor keyed to_a_78")]
#[doc(alias = "__GLOBAL__I_a_78")]
pub use rbx_core::generated_core_shard_js::stub_2d145c as stub_2d145c;

// 0x2d1a5c — __ZN3RBX14AdvLuaDragToolD0Ev
#[doc(alias = "RBX::AdvLuaDragTool::~AdvLuaDragTool()")]
#[doc(alias = "__ZN3RBX14AdvLuaDragToolD0Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2d1a5c as stub_2d1a5c;

// 0x2d1afc — __ZN3RBX14AdvLuaDragToolD1Ev
#[doc(alias = "RBX::AdvLuaDragTool::~AdvLuaDragTool()")]
#[doc(alias = "__ZN3RBX14AdvLuaDragToolD1Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2d1afc as stub_2d1afc;

// 0x2d1b00 — __ZThn36_N3RBX14AdvLuaDragToolD0Ev
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragTool::~AdvLuaDragTool()")]
#[doc(alias = "__ZThn36_N3RBX14AdvLuaDragToolD0Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2d1b00 as stub_2d1b00;

// 0x2d1b08 — __ZN3RBX14AdvLuaDragToolD2Ev
#[doc(alias = "RBX::AdvLuaDragTool::~AdvLuaDragTool()")]
#[doc(alias = "__ZN3RBX14AdvLuaDragToolD2Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2d1b08 as stub_2d1b08;

// 0x2d1c48 — __ZThn36_N3RBX14AdvLuaDragToolD1Ev
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragTool::~AdvLuaDragTool()")]
#[doc(alias = "__ZThn36_N3RBX14AdvLuaDragToolD1Ev")]
pub use rbx_reflection::generated_refl_wd_10q::stub_2d1c48 as stub_2d1c48;

// 0x2d1c50 — __ZN3RBX14AdvLuaDragTool11onMouseDownERKNS_7UIEventE
#[doc(alias = "RBX::AdvLuaDragTool::onMouseDown(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX14AdvLuaDragTool11onMouseDownERKNS_7UIEventE")]
pub use rbx_core::generated_core_watchdog_l::stub_0x2d1c50 as stub_2d1c50;

// 0x2d1e34 — __ZN3RBX14AdvLuaDragTool11onMouseMoveERKNS_7UIEventE
#[doc(alias = "RBX::AdvLuaDragTool::onMouseMove(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX14AdvLuaDragTool11onMouseMoveERKNS_7UIEventE")]
pub use rbx_core::generated_core_watchdog_l::stub_0x2d1e34 as stub_2d1e34;

// 0x2d1edc — __ZN3RBX14AdvLuaDragTool11onMouseIdleERKNS_7UIEventE
#[doc(alias = "RBX::AdvLuaDragTool::onMouseIdle(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX14AdvLuaDragTool11onMouseIdleERKNS_7UIEventE")]
pub use rbx_core::generated_core_watchdog_l::stub_0x2d1edc as stub_2d1edc;

// 0x2d1f5c — __ZN3RBX14AdvLuaDragTool9onMouseUpERKNS_7UIEventE
#[doc(alias = "RBX::AdvLuaDragTool::onMouseUp(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX14AdvLuaDragTool9onMouseUpERKNS_7UIEventE")]
pub use rbx_core::generated_core_watchdog_l::stub_0x2d1f5c as stub_2d1f5c;

// 0x2d21d8 — __ZN3RBX14AdvLuaDragTool9onKeyDownERKNS_7UIEventE
#[doc(alias = "RBX::AdvLuaDragTool::onKeyDown(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX14AdvLuaDragTool9onKeyDownERKNS_7UIEventE")]
pub use rbx_core::generated_core_watchdog_l::stub_0x2d21d8 as stub_2d21d8;

// 0x2d2374 — __ZN5boost10shared_ptrIN3RBX13AdvLuaDraggerEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragger>::operator=(rbx_core::SharedPtr<RBX::AdvLuaDragger> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13AdvLuaDraggerEEaSERKS3_")]
// was: boost::shared_ptr<RBX::AdvLuaDragger>::operator=(boost::shared_ptr<RBX::AdvLuaDragger> const&)
pub fn stub_2d2374(slot: &mut SharedPtr<AdvLuaDragger>, value: &SharedPtr<AdvLuaDragger>) {
    // IDA 0x2d2374 (decompiled): `shared_ptr<AdvLuaDragger>::operator=` — adopts the new
    // count first (`shared_count` copy, 0x2d2388), stores it (0x2d2392..0x2d239a), releases
    // the old (0x2d239e..0x2d23a0). Arc move-assign clones before dropping: same order.
    *slot = SharedPtr::clone(value);
}

// 0x2d23ac — __ZN3RBX11shared_fromINS_14AdvLuaDragToolEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragTool> RBX::shared_from<RBX::AdvLuaDragTool>(RBX::AdvLuaDragTool*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_14AdvLuaDragToolEEEN5boost10shared_ptrIT_EEPS4_")]
// was: boost::shared_ptr<RBX::AdvLuaDragTool> RBX::shared_from<RBX::AdvLuaDragTool>(RBX::AdvLuaDragTool*)
pub fn stub_2d23ac(tool: &SharedPtr<AdvLuaDragTool>) -> SharedPtr<AdvLuaDragTool> {
    // IDA 0x2d23ac (decompiled): `shared_from<AdvLuaDragTool>` — null input yields null
    // (0x2d23f8..0x2d247e); live input clones under the lock (expired weak throws
    // `bad_weak_ptr`). A `&SharedPtr` is always live, so the clone is exact.
    // BUG: original at 0x2d23ac throws on the expired/null paths; the cutover takes an
    // already-formed `Arc`, so those paths are unrepresentable (cf. reflection 0x17aac).
    SharedPtr::clone(tool)
}

// 0x2d2514 — __ZNK3RBX5NamedINS_16AdvArrowToolBaseELZNS_15sAdvLuaDragToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_16AdvArrowToolBaseELZNS_15sAdvLuaDragToolEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_16AdvArrowToolBaseELZNS_15sAdvLuaDragToolEEE7getNameEv")]
pub use rbx_core::generated_core_watchdog_l::stub_0x2d2514 as stub_2d2514;

// 0x2d253c — __ZNK3RBX14AdvLuaDragTool13getCursorNameEv
#[doc(alias = "RBX::AdvLuaDragTool::getCursorName(void)const")]
#[doc(alias = "__ZNK3RBX14AdvLuaDragTool13getCursorNameEv")]
pub use rbx_core::generated_core_watchdog_l::stub_0x2d253c as stub_2d253c;

// 0x2d2548 — __ZN3RBX14AdvLuaDragTool9setCursorESs
#[doc(alias = "RBX::AdvLuaDragTool::setCursor(std::string)")]
#[doc(alias = "__ZN3RBX14AdvLuaDragTool9setCursorESs")]
pub use rbx_core::generated_core_watchdog_l::stub_0x2d2548 as stub_2d2548;

// 0x2d2550 — __ZN3RBX4Name13callDoDeclareILZNS_15sAdvLuaDragToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sAdvLuaDragToolEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sAdvLuaDragToolEEEEvv")]
pub use rbx_core::generated_core_watchdog_l::stub_0x2d2550 as stub_2d2550;

// 0x2d2554 — __ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v")]
pub use rbx_core::generated_core_watchdog_l::stub_0x2d2554 as stub_2d2554;

#[cfg(test)]
mod async_bind_tests {
    use super::*;

    fn live_setup() -> (ThreadRef, WeakPtr<ScriptContext>, SharedPtr<ScriptContext>) {
        let ctx = SharedPtr::new(ScriptContext::default());
        let weak = SharedPtr::downgrade(&ctx);
        let thread: ThreadRef = SharedPtr::new(LiveThreadRef::default());
        (thread, weak, ctx)
    }

    fn result_with() -> AsyncResult {
        AsyncResult { tuple: SharedPtr::new(Tuple) }
    }

    #[test]
    fn invoke_enqueues_resume_for_live_context() {
        PENDING_RESUMES.lock().clear();
        let (thread, weak, ctx) = live_setup();
        let bound = stub_2cc210(&thread, &weak);
        let cb = stub_2ccd80(&bound);
        THREAD_REF_COUNT.store(7, Ordering::Relaxed);
        cb.invoke(&result_with());
        let queue = PENDING_RESUMES.lock();
        assert_eq!(queue.len(), 1);
        assert!(SharedPtr::ptr_eq(&queue[0].context, &ctx));
        assert!(SharedPtr::ptr_eq(&queue[0].thread, &thread));
        assert_eq!(THREAD_REF_COUNT.load(Ordering::Relaxed), 6);
    }

    #[test]
    fn expired_context_drops_result_silently() {
        PENDING_RESUMES.lock().clear();
        let (thread, weak, _ctx) = live_setup();
        drop(_ctx);
        assert!(weak.upgrade().is_none());
        let bound = stub_2cc210(&thread, &weak);
        stub_2cd984(&bound, &result_with());
        assert!(PENDING_RESUMES.lock().is_empty());
    }

    #[test]
    fn clear_and_manager_destroy_disarm() {
        PENDING_RESUMES.lock().clear();
        let (thread, weak, _ctx) = live_setup();
        let bound = stub_2cc210(&thread, &weak);
        let mut cb = stub_2ccf68(&bound);
        stub_2cc608(&mut cb);
        cb.invoke(&result_with());
        assert!(PENDING_RESUMES.lock().is_empty());
        let mut slot = AsyncCallback::default();
        stub_2cdb6c(&mut slot, &cb, AsyncFunctorOp::Clone);
        stub_2cdb6c(&mut slot, &cb, AsyncFunctorOp::Destroy);
        THREAD_REF_COUNT.store(3, Ordering::Relaxed);
        stub_2cdb6c(&mut slot, &cb, AsyncFunctorOp::Destroy);
        assert_eq!(THREAD_REF_COUNT.load(Ordering::Relaxed), 2);
        assert_eq!(stub_2cd4d4(), ASYNC_BIND_TYPE_NAME);
    }

    #[test]
    fn ctors_clone_thread_and_assign_functor() {
        let (thread, weak, _ctx) = live_setup();
        let before = SharedPtr::strong_count(&thread);
        let a = stub_2cc634(&thread);
        let b = stub_2cccc4(&thread);
        let c = stub_2cc6f0(&thread, &weak);
        let d = stub_2cc8d0(&thread, &weak);
        let e = stub_2ccab0(&thread, &weak);
        let f = stub_2cd154(&c);
        assert_eq!(SharedPtr::strong_count(&thread), before + 6);
        drop((a, b, d, e, f));
        let mut slot: ThreadRef = SharedPtr::clone(&thread);
        stub_2cbc1c(&mut slot, &c.thread);
        assert!(SharedPtr::ptr_eq(&slot, &c.thread));
        let boxed = stub_2cd8d8(&c);
        assert!(SharedPtr::ptr_eq(&boxed.thread, &thread));
        let mut dst = AsyncCallback::default();
        stub_2cd2dc(&mut dst, &c);
        stub_2cd50c(&mut dst, &c);
        stub_2cd6f4(&mut dst, &c);
        let src = dst.clone();
        stub_2cdd44(&mut dst, &src);
        assert!(dst.inner.is_some());
    }

    #[test]
    fn drag_tool_shared_ownership() {
        let tool = stub_2ce804(Box::new(AdvLuaDragTool::default()));
        assert_eq!(SharedPtr::strong_count(&tool), 1);
        let again = stub_2d23ac(&tool);
        assert_eq!(SharedPtr::strong_count(&tool), 2);
        drop(again);
        assert!(stub_2ceac0(MOUSE_COMMAND_DELETER_TYPE_NAME).is_some());
        assert!(stub_2ceac0("bogus").is_none());
        let _ = stub_2cead8();
        let mut slot = Some(Box::new(AdvLuaDragTool::default()));
        stub_2ceab0(&mut slot);
        assert!(slot.is_none());
        stub_2ce9b0();
        let one = SharedPtr::new(AdvLuaDragger::default());
        let mut dst = SharedPtr::clone(&one);
        stub_2d2374(&mut dst, &one);
        assert!(SharedPtr::ptr_eq(&dst, &one));
    }
}
