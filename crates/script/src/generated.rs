// Auto-generated skeletons for rbx-script — Lua/Script/Yield/lua batch
// Filter: Script|Lua|Yield|lua (case-sensitive, lua lower)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +121 stubs | range 0xf556a4..0xf69f64 | filtered 5401, existing 5280, remaining 0 (5280 -> 5401 total)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::ops::Range;

// ── Hand-written ScriptContext/Lua support (IDA batch 0xf2c2b4..0xf2c924) ────
// `boost::unordered_map` → [`HashMap`], `std::map`/`_Rb_tree` → [`BTreeMap`],
// `std::deque` → [`VecDeque`], `boost::function`/`bind`/`_mfi` → closures,
// `boost::shared_ptr` → [`SharedPtr`].

/// `RBX::ScriptContext` handle behind `ServiceProvider::find/create`
/// (IDA 0xf2c344/0xf2c364).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScriptContextHandle {
    pub name: String,
}

impl ScriptContextHandle {
    pub const CLASS_NAME: &'static str = "ScriptContext";
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned() }
    }
}

/// Service table slot holding the `ScriptContext` (IDA 0xf2c344/0xf2c364).
#[derive(Debug, Default)]
pub struct ScriptContextRegistry {
    pub context: Option<SharedPtr<ScriptContextHandle>>,
}

/// `RBX::ScriptContext::WaitingThread` (IDA 0xf2c6b4): suspended Lua thread
/// plus its wakeup tick.
#[derive(Debug, Clone, PartialEq)]
pub struct WaitingThread {
    pub thread_id: u64,
    pub wake_tick: f64,
}

impl WaitingThread {
    pub fn new(thread_id: u64, wake_tick: f64) -> Self {
        Self { thread_id, wake_tick }
    }
}

/// `std::deque<WaitingThread>` (IDA 0xf2c7e4).
#[derive(Debug, Default, Clone)]
pub struct WaitingThreadQueue {
    pub threads: VecDeque<WaitingThread>,
}

impl WaitingThreadQueue {
    pub fn with_capacity(cap: usize) -> Self {
        Self { threads: VecDeque::with_capacity(cap) }
    }
}

/// `RBX::ScriptContext::ScriptStart` (IDA 0xf2c864): queued script plus its
/// start options payload.
#[derive(Debug, Clone, PartialEq)]
pub struct ScriptStart {
    pub name: String,
    pub source: String,
}

impl ScriptStart {
    pub fn new(name: &str, source: &str) -> Self {
        Self { name: name.to_owned(), source: source.to_owned() }
    }
}

/// `LuaProfiler::StringCache::Function` map key (IDA 0xf2c2e4).
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProfilerFunction {
    pub name: String,
}

impl ProfilerFunction {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned() }
    }
}

/// `RBX::ScriptContext::ScriptStatInformation` (IDA 0xf2c734).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScriptStatInfo {
    pub calls: u64,
    pub total_time: f64,
}

/// `RBX::Scripting::DebuggerBreakpoint` table entry (IDA 0xf2c2b4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebuggerBreakpoint {
    pub id: i32,
    pub enabled: bool,
}

/// `boost::unordered` int → breakpoint table (IDA 0xf2c2b4).
#[derive(Debug, Default)]
pub struct BreakpointTable {
    pub map: HashMap<i32, DebuggerBreakpoint>,
}

/// `RBX::Lua::WeakThreadRef` (IDA 0xf2c454).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct WeakThreadRef {
    pub id: u64,
}

/// Opaque `lua_State` view (IDA 0xf2c454): only the stack top is modeled.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LuaStateRef {
    pub stack_top: i32,
}

/// `RBX::ScriptContext::ScriptStartOptions` (IDA 0xf2c464).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ScriptStartOptions {
    pub timeout_secs: f64,
}

/// `RBX::Script` entry for the allocator destroy path (IDA 0xf2c2d4).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScriptEntry {
    pub name: String,
}

// 0xf2bdf4 — j___ZN5boost6detail12shared_countC2IPN3RBX10CoreScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2bdf4]")]
pub fn stub_0xf2bdf4() {
    // IDA 0xf2bdf4: shared_count ctor thunk (j__); refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0xf2be04 — j___ZN5boost6detail12shared_countC2IPN3RBX11LuaSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2be04]")]
pub fn stub_0xf2be04() {
    // IDA 0xf2be04: shared_count ctor thunk (j__); refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0xf2be14 — j___ZN5boost6detail12shared_countC2IPN3RBX12LuaStatsItemENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2be14]")]
pub fn stub_0xf2be14() {
    // IDA 0xf2be14: shared_count ctor thunk (j__); refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0xf2be34 — j___ZN5boost6detail12shared_countC2IPN3RBX13ScriptContextENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2be34]")]
pub fn stub_0xf2be34() {
    // IDA 0xf2be34: shared_count ctor thunk (j__); refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0xf2be44 — j___ZN5boost6detail12shared_countC2IPN3RBX13StarterScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2be44]")]
pub fn stub_0xf2be44() {
    // IDA 0xf2be44: shared_count ctor thunk (j__); refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0xf2be64 — j___ZN5boost6detail12shared_countC2IPN3RBX9Scripting13DebuggerWatchENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2be64]")]
pub fn stub_0xf2be64() {
    // IDA 0xf2be64: shared_count ctor thunk (j__); refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0xf2be74 — j___ZN5boost6detail12shared_countC2IPN3RBX9Scripting14ScriptDebuggerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2be74]")]
pub fn stub_0xf2be74() {
    // IDA 0xf2be74: shared_count ctor thunk (j__); refcount owned by SharedPtr (Arc) — carrier no-op.
}

// 0xf2bea4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2bea4]")]
pub fn stub_0xf2bea4() {
    // IDA 0xf2bea4: functor_manager thunk (j__); closure buffer ops fold into Box<dyn Fn> — carrier no-op.
}

// 0xf2beb4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2beb4]")]
pub fn stub_0xf2beb4() {
    // IDA 0xf2beb4: functor_manager thunk (j__); closure buffer ops fold into Box<dyn Fn> — carrier no-op.
}

// 0xf2bef4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2bef4]")]
pub fn stub_0xf2bef4() {
    // IDA 0xf2bef4: functor_manager thunk (j__); closure buffer ops fold into Box<dyn Fn> — carrier no-op.
}

// 0xf2bf64 — j___ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, RBX::Lua::WeakThreadRef *, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2bf64() {
    // IDA 0xf2bf64: function-bind ctor/assign thunk (j__); binds are plain closures — carrier no-op.
}

// 0xf2bf84 — j___ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2bf84() {
    // IDA 0xf2bf84: function-bind ctor/assign thunk (j__); binds are plain closures — carrier no-op.
}

// 0xf2bf94 — j___ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeESP_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, int, RBX::Lua::WeakThreadRef *, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeESP_")]
pub fn stub_0xf2bf94() {
    // IDA 0xf2bf94: function-bind ctor/assign thunk (j__); binds are plain closures — carrier no-op.
}

// 0xf2bfb4 — j___ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeESQ_
#[doc(alias = "j___ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeESQ_")]
pub fn stub_0xf2bfb4() {
    // IDA 0xf2bfb4: function-bind ctor/assign thunk (j__); binds are plain closures — carrier no-op.
}

// 0xf2bfc4 — j___ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSERKS8_
#[doc(alias = "boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>::operator=(boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)> const&) [0xf2bfc4]")]
pub fn stub_0xf2bfc4() {
    // IDA 0xf2bfc4: function-bind ctor/assign thunk (j__); binds are plain closures — carrier no-op.
}

// 0xf2bfe4 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>) [0xf2bfe4]")]
pub fn stub_0xf2bfe4() {
    // IDA 0xf2bfe4: function-bind ctor/assign thunk (j__); binds are plain closures — carrier no-op.
}

// 0xf2bff4 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2bff4() {
    // IDA 0xf2bff4: function-bind ctor/assign thunk (j__); binds are plain closures — carrier no-op.
}

// 0xf2c0e4 — j___ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS9_3Lua13WeakThreadRefES2_EENS5_5list3INS5_5valueIPSA_EENSF_ISC_EENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, RBX::Lua::WeakThreadRef *, int, int, int, int)
#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>) [0xf2c0e4]")]
pub fn stub_0xf2c0e4() {
    // IDA 0xf2c0e4: function-bind ctor/assign thunk (j__); binds are plain closures — carrier no-op.
}

// 0xf2c104 — j___ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>) [0xf2c104]")]
pub fn stub_0xf2c104() {
    // IDA 0xf2c104: function-bind ctor/assign thunk (j__); binds are plain closures — carrier no-op.
}

// 0xf2c114 — j___ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS9_3Lua13WeakThreadRefES2_EENS5_5list3INS5_5valueIPSA_EENSF_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, RBX::Lua::WeakThreadRef *, int, int, int, int)
#[doc(alias = "j___ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS9_3Lua13WeakThreadRefES2_EENS5_5list3INS5_5valueIPSA_EENSF_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2c114() {
    // IDA 0xf2c114: function-bind ctor/assign thunk (j__); binds are plain closures — carrier no-op.
}

// 0xf2c134 — j___ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
#[doc(alias = "j___ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2c134() {
    // IDA 0xf2c134: function-bind ctor/assign thunk (j__); binds are plain closures — carrier no-op.
}

// 0xf2c214 — j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE11move_assignERS7_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::move_assign(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>&) [0xf2c214]")]
pub fn stub_0xf2c214() {
    // IDA 0xf2c214: function slot lifecycle thunk; Drop/assignment covers it — carrier no-op.
}

// 0xf2c224 — j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE13assign_to_ownERKS7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to_own(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int> const&) [0xf2c224]")]
pub fn stub_0xf2c224() {
    // IDA 0xf2c224: function slot lifecycle thunk; Drop/assignment covers it — carrier no-op.
}

// 0xf2c234 — j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE4swapERS7_
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::swap(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>&) [0xf2c234]")]
pub fn stub_0xf2c234() {
    // IDA 0xf2c234: function slot lifecycle thunk; Drop/assignment covers it — carrier no-op.
}

// 0xf2c244 — j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE5clearEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::clear(void) [0xf2c244]")]
pub fn stub_0xf2c244() {
    // IDA 0xf2c244: function slot lifecycle thunk; Drop/assignment covers it — carrier no-op.
}

// 0xf2c2b4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::table(unsigned long,boost::hash<int> const&,std::equal_to<int> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>> const&) [0xf2c2b4]")]
pub fn stub_0xf2c2b4(buckets: usize) -> BreakpointTable {
    // IDA 0xf2c2b4: table(n, hash, equal, alloc) for int → DebuggerBreakpoint*;
    // `HashMap::with_capacity` sizes the bucket array for the same `n`.
    BreakpointTable { map: HashMap::with_capacity(buckets) }
}

// 0xf2c2d4 — j___ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEEE7destroyEPS8_
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>*) [0xf2c2d4]")]
pub fn stub_0xf2c2d4(entry: (String, SharedPtr<ScriptEntry>)) {
    // IDA 0xf2c2d4: new_allocator<pair<string const, shared_ptr<Script>>>::destroy —
    // runs the pair dtor (string + shared_ptr release); owned drop matches.
    drop(entry);
}

// 0xf2c2e4 — j___ZNK11LuaProfiler11StringCache8FunctionltERKS1_
#[doc(alias = "LuaProfiler::StringCache::Function::operator<(LuaProfiler::StringCache::Function const&)const [0xf2c2e4]")]
pub fn stub_0xf2c2e4(a: &ProfilerFunction, b: &ProfilerFunction) -> bool {
    // IDA 0xf2c2e4: StringCache::Function::operator< — lexicographic compare
    // of the cached function name; drives the profiler map ordering.
    a.name < b.name
}

// 0xf2c2f4 — j___ZNK3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0xf2c2f4() -> &'static str {
    // IDA 0xf2c2f4: FactoryProduct<LocalScript, Script, sLocalScript>::Creator::getClassName —
    // `Name::declare<sLocalScript>()`.
    "LocalScript"
}

// 0xf2c304 — j___ZNK3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0xf2c304() -> &'static str {
    // IDA 0xf2c304: FactoryProduct<LuaSettings, Item, sLuaSettings>::Creator::getClassName —
    // `Name::declare<sLuaSettings>()`.
    "LuaSettings"
}

// 0xf2c314 — j___ZNK3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7Creator12getClassNameEv
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7Creator12getClassNameEv")]
pub fn stub_0xf2c314() -> &'static str {
    // IDA 0xf2c314: FactoryProduct<DebuggerWatch, sDebuggerWatch>::Creator::getClassName —
    // `Name::declare<sDebuggerWatch>()`.
    "DebuggerWatch"
}

// 0xf2c324 — j___ZNK3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7Creator12getClassNameEv
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7Creator12getClassNameEv")]
pub fn stub_0xf2c324() -> &'static str {
    // IDA 0xf2c324: FactoryProduct<ScriptDebugger, sScriptDebugger>::Creator::getClassName —
    // `Name::declare<sScriptDebugger>()`.
    "ScriptDebugger"
}

// 0xf2c344 — j___ZNK3RBX15ServiceProvider4findINS_13ScriptContextEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ScriptContext * RBX::ServiceProvider::find<RBX::ScriptContext>(void)const [0xf2c344]")]
pub fn stub_0xf2c344(registry: &ScriptContextRegistry) -> Option<SharedPtr<ScriptContextHandle>> {
    // IDA 0xf2c344: ServiceProvider::find<ScriptContext> — service-table
    // lookup; null when the provider holds no ScriptContext yet.
    registry.context.clone()
}

// 0xf2c364 — j___ZNK3RBX15ServiceProvider6createINS_13ScriptContextEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ScriptContext * RBX::ServiceProvider::create<RBX::ScriptContext>(void)const [0xf2c364]")]
pub fn stub_0xf2c364(registry: &mut ScriptContextRegistry, name: &str) -> SharedPtr<ScriptContextHandle> {
    // IDA 0xf2c364: ServiceProvider::create<ScriptContext> — find-or-create;
    // an existing context is returned, otherwise one is constructed,
    // registered, and returned.
    if let Some(ctx) = registry.context.clone() {
        return ctx;
    }
    let ctx = SharedPtr::new(ScriptContextHandle::new(name));
    registry.context = Some(ctx.clone());
    ctx
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreScript,RBX::CoreScript>(rbx_core::SharedPtr<RBX::CoreScript> const*,RBX::CoreScript *)const [0xf2c384]")]
pub fn stub_0xf2c384() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CoreScript")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalScript,RBX::LocalScript>(rbx_core::SharedPtr<RBX::LocalScript> const*,RBX::LocalScript *)const [0xf2c3a4]")]
pub fn stub_0xf2c3a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LocalScript")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaSettings,RBX::LuaSettings>(rbx_core::SharedPtr<RBX::LuaSettings> const*,RBX::LuaSettings *)const [0xf2c3b4]")]
pub fn stub_0xf2c3b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaSettings")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaStatsItem,RBX::LuaStatsItem>(rbx_core::SharedPtr<RBX::LuaStatsItem> const*,RBX::LuaStatsItem *)const")]
pub fn stub_0xf2c3c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaStatsItem")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterScript,RBX::StarterScript>(rbx_core::SharedPtr<RBX::StarterScript> const*,RBX::StarterScript *)const [0xf2c3e4]")]
pub fn stub_0xf2c3e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StarterScript")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::DebuggerWatch,RBX::Scripting::DebuggerWatch>(rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch> const*,RBX::Scripting::DebuggerWatch *)const [0xf2c404]")]
pub fn stub_0xf2c404() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerWatch")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::ScriptDebugger,RBX::Scripting::ScriptDebugger>(rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> const*,RBX::Scripting::ScriptDebugger *)const [0xf2c414]")]
pub fn stub_0xf2c414() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::ScriptDebugger")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::WaitingScriptsJob>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const*,RBX::WaitingScriptsJob *)const [0xf2c424]")]
pub fn stub_0xf2c424() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TaskScheduler::Job")
}

// 0xf2c444 — j___ZNK5boost4_mfi3mf1IvN3RBX13ScriptContextENS3_11ScriptStartEEclEPS3_S4_
#[doc(alias = "boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>::operator()(RBX::ScriptContext*,RBX::ScriptContext::ScriptStart)const [0xf2c444]")]
pub fn stub_0xf2c444(ctx: &ScriptContextHandle, start: &ScriptStart, run: &mut dyn FnMut(&str, &str)) {
    // IDA 0xf2c444: _mfi::mf1<void, ScriptContext, ScriptStart>::operator() —
    // applies the bound member to (ctx, start); the closure is the member body.
    run(&ctx.name, &start.name);
}

// 0xf2c454 — j___ZNK5boost4_mfi3mf2IvN3RBX13ScriptContextENS2_3Lua13WeakThreadRefEP9lua_StateEclEPS3_S5_S7_
#[doc(alias = "boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>::operator()(RBX::ScriptContext*,RBX::Lua::WeakThreadRef,lua_State *)const [0xf2c454]")]
pub fn stub_0xf2c454(ctx: &ScriptContextHandle, thread: WeakThreadRef, state: &LuaStateRef, run: &mut dyn FnMut(&str, u64, i32)) {
    // IDA 0xf2c454: _mfi::mf2<void, ScriptContext, WeakThreadRef, lua_State*>::operator() —
    // forwards the bound (thread, state) pair; the closure is the member body.
    run(&ctx.name, thread.id, state.stack_top);
}

// 0xf2c464 — j___ZNK5boost4_mfi3mf2IvN3RBX13ScriptContextEPNS2_10BaseScriptENS3_18ScriptStartOptionsEEclEPS3_S5_S6_
#[doc(alias = "boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>::operator()(RBX::ScriptContext*,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions)const [0xf2c464]")]
pub fn stub_0xf2c464(ctx: &ScriptContextHandle, script: &str, options: ScriptStartOptions, run: &mut dyn FnMut(&str, &str, f64)) {
    // IDA 0xf2c464: _mfi::mf2<void, ScriptContext, BaseScript*, ScriptStartOptions>::operator() —
    // forwards (script, options); the closure is the member body.
    run(&ctx.name, script, options.timeout_secs);
}

// 0xf2c474 — j___ZNK5boost4_mfi3mf3IvN3RBX13ScriptContextENS_10shared_ptrINS2_8InstanceEEESsS6_EclEPS3_S6_SsS6_
#[doc(alias = "boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ScriptContext*,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)const [0xf2c474]")]
pub fn stub_0xf2c474(ctx: &ScriptContextHandle, instance: &str, text: &str, target: &str, run: &mut dyn FnMut(&str, &str, &str, &str)) {
    // IDA 0xf2c474: _mfi::mf3<void, ScriptContext, shared_ptr<Instance>, string, shared_ptr<Instance>>::operator() —
    // forwards the three bound args; the closure is the member body.
    run(&ctx.name, instance, text, target);
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf2c484]")]
pub fn stub_0xf2c484(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &)const [0xf2c494]")]
pub fn stub_0xf2c494(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2c4a4]")]
pub fn stub_0xf2c4a4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf2c4b4]")]
pub fn stub_0xf2c4b4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf2c4d4]")]
pub fn stub_0xf2c4d4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,boost::detail::function::function_buffer &)const [0xf2c4e4]")]
pub fn stub_0xf2c4e4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2c4f4]")]
pub fn stub_0xf2c4f4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &)const [0xf2c524]")]
pub fn stub_0xf2c524(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2c534]")]
pub fn stub_0xf2c534(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0xf2c654 — j___ZNK5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEclES2_S2_S6_i
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::operator()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)const [0xf2c654]")]
pub fn stub_0xf2c654(run: &dyn Fn(&str, &str, &str, i32), first: &str, source: &str, script: &str, code: i32) {
    // IDA 0xf2c654: function4<void, char const*, char const*, shared_ptr<BaseScript>, int>::operator() —
    // dispatches to the stored target with the four call args; the closure is the target.
    run(first, source, script, code);
}

// 0xf2c694 — j___ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_allocate_map(unsigned long) [0xf2c694]")]
pub fn stub_0xf2c694(map_size: usize) -> WaitingThreadQueue {
    // IDA 0xf2c694: _Deque_base::_M_allocate_map(n) — allocates the index map;
    // the reserved deque is the owned buffer.
    WaitingThreadQueue::with_capacity(map_size)
}

// 0xf2c6a4 — j___ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EE15_M_create_nodesEPPS2_S6_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_create_nodes(RBX::ScriptContext::WaitingThread**,RBX::ScriptContext::WaitingThread**) [0xf2c6a4]")]
pub fn stub_0xf2c6a4(queue: &mut WaitingThreadQueue, additional: usize) {
    // IDA 0xf2c6a4: _M_create_nodes — allocates node storage for the new range;
    // `reserve` grows the backing ring the same way.
    queue.threads.reserve(additional);
}

// 0xf2c6b4 — j___ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_initialize_map(unsigned long) [0xf2c6b4]")]
pub fn stub_0xf2c6b4(map_size: usize) -> WaitingThreadQueue {
    // IDA 0xf2c6b4: _M_initialize_map(n) — allocate_map plus centering the
    // start/finish iterators; an empty reserved deque has the same shape.
    WaitingThreadQueue::with_capacity(map_size)
}

// 0xf2c6c4 — j___ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EED2Ev
#[doc(alias = "std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::~_Deque_base() [0xf2c6c4]")]
pub fn stub_0xf2c6c4(queue: WaitingThreadQueue) {
    // IDA 0xf2c6c4: _Deque_base dtor — destroys elements then frees the map;
    // owned drop matches.
    drop(queue);
}

// 0xf2c6f4 — j___ZNSt12_Vector_baseIN3RBX13ScriptContext11ScriptStartESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::_M_allocate(unsigned long) [0xf2c6f4]")]
pub fn stub_0xf2c6f4(count: usize) -> Vec<ScriptStart> {
    // IDA 0xf2c6f4: _Vector_base<ScriptStart>::_M_allocate(n) → operator new;
    // the reserved `Vec` is the owned buffer.
    Vec::with_capacity(count)
}

// 0xf2c714 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13ScriptContext11ScriptStartES6_EET0_T_S8_S7_
#[doc(alias = "RBX::ScriptContext::ScriptStart * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *>(RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *) [0xf2c714]")]
pub fn stub_0xf2c714(items: &mut Vec<ScriptStart>, src: Range<usize>, dst: usize) {
    // IDA 0xf2c714: __copy_backward for ScriptStart — shifts the tail up to
    // make room. `ScriptStart` is not `Copy`, so the range is snapshotted
    // then spliced back; same overlapping move without a `Copy` bound.
    let end = src.end.min(items.len());
    let start = src.start.min(end);
    let buf: Vec<ScriptStart> = items[start..end].to_vec();
    let dst = dst.min(items.len());
    for (i, value) in buf.into_iter().enumerate() {
        if dst + i < items.len() {
            items[dst + i] = value;
        } else {
            items.push(value);
        }
    }
}

// 0xf2c724 — j___ZNSt3mapIN11LuaProfiler11StringCache8FunctionESsSt4lessIS2_ESaISt4pairIKS2_SsEEEixERS6_
#[doc(alias = "std::map<LuaProfiler::StringCache::Function,std::string,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::operator[](LuaProfiler::StringCache::Function const&) [0xf2c724]")]
pub fn stub_0xf2c724<'a>(map: &'a mut BTreeMap<ProfilerFunction, String>, key: &ProfilerFunction) -> &'a mut String {
    // IDA 0xf2c724: map<Function, string>::operator[] — lower_bound plus
    // default-string insert on a miss; `entry().or_default()` matches.
    map.entry(key.clone()).or_default()
}

// 0xf2c734 — j___ZNSt3mapISsN3RBX13ScriptContext21ScriptStatInformationESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
#[doc(alias = "std::map<std::string,RBX::ScriptContext::ScriptStatInformation,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::operator[](std::string const&) [0xf2c734]")]
pub fn stub_0xf2c734<'a>(map: &'a mut HashMap<String, ScriptStatInfo>, key: &str) -> &'a mut ScriptStatInfo {
    // IDA 0xf2c734: map<string, ScriptStatInformation>::operator[] — same
    // lookup-or-default pattern over the stat table.
    map.entry(key.to_owned()).or_default()
}

// 0xf2c754 — j___ZNSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEEC2ERS0_RKS3_
#[doc(alias = "std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>::pair(std::string const&,RBX::ScriptContext::ScriptStatInformation const&) [0xf2c754]")]
pub fn stub_0xf2c754(name: &str, info: ScriptStatInfo) -> (String, ScriptStatInfo) {
    // IDA 0xf2c754: pair<string const, ScriptStatInformation>::pair — copies
    // both members into the new pair.
    (name.to_owned(), info)
}

// 0xf2c774 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_pop_front_auxEv
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_pop_front_aux(void) [0xf2c774]")]
pub fn stub_0xf2c774(queue: &mut WaitingThreadQueue) -> Option<WaitingThread> {
    // IDA 0xf2c774: _M_pop_front_aux — destroys the front node and advances;
    // returns the popped thread for the caller to release.
    queue.threads.pop_front()
}

// 0xf2c784 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_push_back_auxERKS2_
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_push_back_aux(RBX::ScriptContext::WaitingThread const&) [0xf2c784]")]
pub fn stub_0xf2c784(queue: &mut WaitingThreadQueue, thread: WaitingThread) {
    // IDA 0xf2c784: _M_push_back_aux — allocates a new back node when the last
    // one is full, then copy-constructs; `push_back` covers both.
    queue.threads.push_back(thread);
}

// 0xf2c794 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_reallocate_map(unsigned long,bool) [0xf2c794]")]
pub fn stub_0xf2c794(queue: &mut WaitingThreadQueue, additional: usize, _front: bool) {
    // IDA 0xf2c794: _M_reallocate_map(n, add_at_front) — grows the map on the
    // front or back; `reserve` grows the ring either way.
    queue.threads.reserve(additional);
}

// 0xf2c7a4 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE19_M_destroy_data_auxESt15_Deque_iteratorIS2_RS2_PS2_ES8_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_destroy_data_aux(std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*>,std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*>) [0xf2c7a4]")]
pub fn stub_0xf2c7a4(queue: &mut WaitingThreadQueue) {
    // IDA 0xf2c7a4: _M_destroy_data_aux(first, last) — destroys every node in
    // the range; `clear` drops all elements.
    queue.threads.clear();
}

// 0xf2c7b4 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE22_M_reserve_map_at_backEm
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_reserve_map_at_back(unsigned long) [0xf2c7b4]")]
pub fn stub_0xf2c7b4(queue: &mut WaitingThreadQueue, nodes: usize) {
    // IDA 0xf2c7b4: _M_reserve_map_at_back(n) — ensures `n` free map slots at
    // the back; `reserve` keeps the same spare capacity.
    queue.threads.reserve(nodes);
}

// 0xf2c7c4 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9pop_frontEv
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::pop_front(void) [0xf2c7c4]")]
pub fn stub_0xf2c7c4(queue: &mut WaitingThreadQueue) -> Option<WaitingThread> {
    // IDA 0xf2c7c4: pop_front — destroys the front element (via
    // _M_pop_front_aux when crossing a node boundary) and advances.
    queue.threads.pop_front()
}

// 0xf2c7d4 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9push_backERKS2_
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::push_back(RBX::ScriptContext::WaitingThread const&) [0xf2c7d4]")]
pub fn stub_0xf2c7d4(queue: &mut WaitingThreadQueue, thread: WaitingThread) {
    // IDA 0xf2c7d4: push_back — in-place construct or _M_push_back_aux
    // (stub_0xf2c784) on a full back node; `push_back` covers both.
    queue.threads.push_back(thread);
}

// 0xf2c7e4 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EEC2ERKS4_
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::deque(std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>> const&) [0xf2c7e4]")]
pub fn stub_0xf2c7e4(queue: &WaitingThreadQueue) -> WaitingThreadQueue {
    // IDA 0xf2c7e4: deque copy ctor — _M_initialize_map for the size, then
    // element-wise copy; `clone` matches.
    queue.clone()
}

// 0xf2c7f4 — j___ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EED2Ev
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::~deque() [0xf2c7f4]")]
pub fn stub_0xf2c7f4(queue: WaitingThreadQueue) {
    // IDA 0xf2c7f4: ~deque — destroys elements then the _Deque_base map;
    // owned drop matches.
    drop(queue);
}

// 0xf2c814 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX13ScriptContext11ScriptStartES6_EET0_T_S8_S7_
#[doc(alias = "RBX::ScriptContext::ScriptStart * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *>(RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *) [0xf2c814]")]
pub fn stub_0xf2c814(items: &mut Vec<ScriptStart>, src: Range<usize>, dst: usize) {
    // IDA 0xf2c814: __copy for ScriptStart — forward copy of the range;
    // same snapshot-splice as stub_0xf2c714 for the non-`Copy` element type.
    let end = src.end.min(items.len());
    let start = src.start.min(end);
    let buf: Vec<ScriptStart> = items[start..end].to_vec();
    let dst = dst.min(items.len());
    for (i, value) in buf.into_iter().enumerate() {
        if dst + i < items.len() {
            items[dst + i] = value;
        } else {
            items.push(value);
        }
    }
}

// 0xf2c844 — j___ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart*,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,RBX::ScriptContext::ScriptStart const&) [0xf2c844]")]
pub fn stub_0xf2c844(items: &mut Vec<ScriptStart>, pos: usize, value: ScriptStart) -> usize {
    // IDA 0xf2c844: vector<ScriptStart>::_M_insert_aux — grow, shift the
    // tail, construct at pos. `Vec::insert` matches.
    let pos = pos.min(items.len());
    items.insert(pos, value);
    pos
}

// 0xf2c854 — j___ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS2_S4_EE
#[doc(alias = "std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::erase(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart*,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>) [0xf2c854]")]
pub fn stub_0xf2c854(items: &mut Vec<ScriptStart>, pos: usize) -> ScriptStart {
    // IDA 0xf2c854: vector<ScriptStart>::erase — destroys at pos and shifts
    // the tail down; `remove` returns the erased element (panics on a bad
    // iterator, matching the C++ precondition).
    items.remove(pos)
}

// 0xf2c864 — j___ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE9push_backERKS2_
// type: int __fastcall(int, RBX::ScriptContext::ScriptStart *)
#[doc(alias = "std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::push_back(RBX::ScriptContext::ScriptStart const&) [0xf2c864]")]
pub fn stub_0xf2c864(items: &mut Vec<ScriptStart>, value: ScriptStart) {
    // IDA 0xf2c864: vector<ScriptStart>::push_back — in-place construct or
    // _M_insert_aux (stub_0xf2c844) when full; `push` covers both.
    items.push(value);
}

// 0xf2c874 — j___ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EED2Ev
#[doc(alias = "std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::~vector() [0xf2c874]")]
pub fn stub_0xf2c874(items: Vec<ScriptStart>) {
    // IDA 0xf2c874: ~vector<ScriptStart> — destroys elements, frees the
    // buffer; owned drop matches.
    drop(items);
}

// 0xf2c8e4 — j___ZNSt8_Rb_treeIN11LuaProfiler11StringCache8FunctionESt4pairIKS2_SsESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_create_node(std::pair<LuaProfiler::StringCache::Function const,std::string> const&) [0xf2c8e4]")]
pub fn stub_0xf2c8e4(key: ProfilerFunction, value: String) -> (ProfilerFunction, String) {
    // IDA 0xf2c8e4: _Rb_tree<Function, pair<Function const, string>>::_M_create_node —
    // allocates the node and copy-constructs the pair.
    (key, value)
}

// 0xf2c8f4 — j___ZNSt8_Rb_treeIN11LuaProfiler11StringCache8FunctionESt4pairIKS2_SsESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_insert_unique(std::pair<LuaProfiler::StringCache::Function const,std::string> const&) [0xf2c8f4]")]
pub fn stub_0xf2c8f4(tree: &mut BTreeMap<ProfilerFunction, String>, key: ProfilerFunction, value: String) -> bool {
    // IDA 0xf2c8f4: _M_insert_unique(pair) — inserts only when the key is
    // absent; returns whether insertion happened.
    if tree.contains_key(&key) {
        return false;
    }
    tree.insert(key, value);
    true
}

#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::pair<LuaProfiler::StringCache::Function const,std::string> const&) [0xf2c904]")]
pub fn stub_0xf2c904(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0xf2c914 — j___ZNSt8_Rb_treeIN11LuaProfiler11StringCache8FunctionESt4pairIKS2_SsESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_erase(std::_Rb_tree_node<std::pair<LuaProfiler::StringCache::Function const,std::string>> *) [0xf2c914]")]
pub fn stub_0xf2c914(tree: &mut BTreeMap<ProfilerFunction, String>, key: &ProfilerFunction) -> bool {
    // IDA 0xf2c914: _M_erase(node) — unlinks and frees the node; returns
    // whether a node was erased.
    tree.remove(key).is_some()
}

// 0xf2c924 — j___ZNSt8_Rb_treeIN11LuaProfiler11StringCache8FunctionESt4pairIKS2_SsESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<LuaProfiler::StringCache::Function const,std::string> const&) [0xf2c924]")]
pub fn stub_0xf2c924(tree: &mut BTreeMap<ProfilerFunction, String>, key: ProfilerFunction, value: String) -> Option<String> {
    // IDA 0xf2c924: _M_insert(hint-node, pair) — links the new node into the
    // tree; returns the displaced value, if any.
    tree.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::equal_range(RBX::BaseScript * const&) [0xf2c934]")]
pub fn stub_0xf2c934(handle: &crate::slot::InstanceHandle) {
// std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::l~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert_unique(RBX::BaseScript * const&) [0xf2c944]")]
pub fn stub_0xf2c944(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert_unique(std::_Rb_tree_iterator<RBX::BaseScript *>,RBX::BaseScript * const&) [0xf2c954]")]
pub fn stub_0xf2c954(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "void std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert_unique<std::_Rb_tree_const_iterator<RBX::BaseScript *>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>) [0xf2c964]")]
pub fn stub_0xf2c964(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::swap(std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>&) [0xf2c974]")]
pub fn stub_0xf2c974(handle: &crate::slot::InstanceHandle) {
// std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::l~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::erase(RBX::BaseScript * const&) [0xf2c984]")]
pub fn stub_0xf2c984(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::erase(std::_Rb_tree_iterator<RBX::BaseScript *>,std::_Rb_tree_iterator<RBX::BaseScript *>) [0xf2c994]")]
pub fn stub_0xf2c994(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_erase(std::_Rb_tree_node<RBX::BaseScript *> *) [0xf2c9a4]")]
pub fn stub_0xf2c9a4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::BaseScript * const&) [0xf2c9b4]")]
pub fn stub_0xf2c9b4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::lower_bound(std::string const&) [0xf2c9c4]")]
pub fn stub_0xf2c9c4(handle: &crate::slot::InstanceHandle) {
// std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInform~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::upper_bound(std::string const&) [0xf2c9d4]")]
pub fn stub_0xf2c9d4(handle: &crate::slot::InstanceHandle) {
// std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInform~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_create_node(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&) [0xf2c9e4]")]
pub fn stub_0xf2c9e4(handle: &crate::slot::InstanceHandle) {
// std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInform~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>> *) [0xf2c9f4]")]
pub fn stub_0xf2c9f4(handle: &crate::slot::InstanceHandle) {
// std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInform~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&) [0xf2ca04]")]
pub fn stub_0xf2ca04(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&) [0xf2ca14]")]
pub fn stub_0xf2ca14(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::find(std::string const&) [0xf2ca24]")]
pub fn stub_0xf2ca24(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::erase(std::string const&) [0xf2ca34]")]
pub fn stub_0xf2ca34(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>) [0xf2ca44]")]
pub fn stub_0xf2ca44(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>> *) [0xf2ca54]")]
pub fn stub_0xf2ca54(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&) [0xf2ca64]")]
pub fn stub_0xf2ca64(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>> *) [0xf2ca94]")]
pub fn stub_0xf2ca94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread const&,RBX::ScriptContext::WaitingThread const*>,std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*>>(std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread const&,RBX::ScriptContext::WaitingThread const*>,std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread const&,RBX::ScriptContext::WaitingThread const*>,std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*>,std::__false_type) [0xf2caf4]")]
pub fn stub_0xf2caf4(handle: &crate::slot::InstanceHandle) {
// std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>) [0xf2cb14]")]
pub fn stub_0xf2cb14() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>) [0xf2cb24]")]
pub fn stub_0xf2cb24() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>> std::for_each<std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>>) [0xf2cb34]")]
pub fn stub_0xf2cb34() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "RBX::Lua::YieldingThreads::WaitingThread::WaitingThread(lua_State *,RBX::Time::Interval) [0xf2cb54]")]
pub fn stub_0xf2cb54() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>::operator=(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef> const&)")]
pub fn stub_0xf2cb64(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "std::_Vector_base<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::_M_allocate(unsigned long) [0xf2cb74]")]
pub fn stub_0xf2cb74() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "std::_Vector_base<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::_Vector_base(unsigned long,std::allocator<RBX::Lua::YieldingThreads::WaitingThread> const&) [0xf2cb84]")]
pub fn stub_0xf2cb84() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "std::priority_queue<RBX::Lua::YieldingThreads::WaitingThread,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>>::pop(void) [0xf2cb94]")]
pub fn stub_0xf2cb94(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::priority_queue<RBX::Lua::YieldingThreads::WaitingThread,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>>::push(RBX::Lua::YieldingThreads::WaitingThread const&) [0xf2cba4]")]
pub fn stub_0xf2cba4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::priority_queue<RBX::Lua::YieldingThreads::WaitingThread,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>>::priority_queue(std::less<RBX::Lua::YieldingThreads::WaitingThread> const&,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>> const&) [0xf2cbb4]")]
pub fn stub_0xf2cbb4() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "RBX::Lua::YieldingThreads::WaitingThread * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Lua::YieldingThreads::WaitingThread *,RBX::Lua::YieldingThreads::WaitingThread *>(RBX::Lua::YieldingThreads::WaitingThread *,RBX::Lua::YieldingThreads::WaitingThread *,RBX::Lua::YieldingThreads::WaitingThread *) [0xf2cbc4]")]
pub fn stub_0xf2cbc4() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread*,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,RBX::Lua::YieldingThreads::WaitingThread const&) [0xf2cbd4]")]
pub fn stub_0xf2cbd4() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::push_back(RBX::Lua::YieldingThreads::WaitingThread const&) [0xf2cbe4]")]
pub fn stub_0xf2cbe4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::vector(std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>> const&) [0xf2cbf4]")]
pub fn stub_0xf2cbf4() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::~vector() [0xf2cc04]")]
pub fn stub_0xf2cc04(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,int,RBX::Lua::YieldingThreads::WaitingThread,std::less<RBX::Lua::YieldingThreads::WaitingThread>>(__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,int,int,RBX::Lua::YieldingThreads::WaitingThread,std::less<RBX::Lua::YieldingThreads::WaitingThread>) [0xf2cc14]")]
pub fn stub_0xf2cc14() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,int,RBX::Lua::YieldingThreads::WaitingThread,std::less<RBX::Lua::YieldingThreads::WaitingThread>>(__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,int,int,RBX::Lua::YieldingThreads::WaitingThread,std::less<RBX::Lua::YieldingThreads::WaitingThread>) [0xf2cc24]")]
pub fn stub_0xf2cc24() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>>(__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>) [0xf2cc34]")]
pub fn stub_0xf2cc34(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "boost::_bi::bind_t<unsigned long,boost::_mfi::cmf0<unsigned long,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<RBX::ScriptContext*>>>::operator()(void) [0xf2ccb4]")]
pub fn stub_0xf2ccb4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "std::pair<std::string const,RBX::ScriptStats::StatCollection>::pair(std::string const&,RBX::ScriptStats::StatCollection const&) [0xf2cd24]")]
pub fn stub_0xf2cd24() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_create_node(std::pair<std::string const,RBX::ScriptStats::StatCollection> const&) [0xf2cda4]")]
pub fn stub_0xf2cda4() -> crate::slot::PortedFn {
// IDA 0xf2cda4: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::s~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2cda4, "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select~")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptStats::StatCollection>> *) [0xf2cdb4]")]
pub fn stub_0xf2cdb4() -> crate::slot::PortedFn {
// IDA 0xf2cdb4: std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::s~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2cdb4, "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select~")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_insert_unique(std::pair<std::string const,RBX::ScriptStats::StatCollection> const&) [0xf2cdc4]")]
pub fn stub_0xf2cdc4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::find(std::string const&) [0xf2cdd4]")]
pub fn stub_0xf2cdd4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptStats::StatCollection>> *) [0xf2cde4]")]
pub fn stub_0xf2cde4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ScriptStats::StatCollection> const&) [0xf2cdf4]")]
pub fn stub_0xf2cdf4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(char const*,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> *) [0xf2ce34]")]
pub fn stub_0xf2ce34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost:~")
}

#[doc(alias = "RBX::Reflection::Type::Type<RBX::Lua::WeakFunctionRef>(char const*,RBX::Lua::WeakFunctionRef *) [0xf2ce44]")]
pub fn stub_0xf2ce44() -> crate::slot::InstanceHandle {
// RBX::Reflection::Type::Type ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Type::Type")
}

#[doc(alias = "rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>* RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::pushNewObject<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(lua_State *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>) [0xf2ce54]")]
pub fn stub_0xf2ce54() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::S~")
}

#[doc(alias = "rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>* RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::pushNewObject<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(lua_State *,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>) [0xf2ce64]")]
pub fn stub_0xf2ce64() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost:~")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::singleton(void) [0xf2ce74]")]
pub fn stub_0xf2ce74(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::singleton(void) [0xf2ce74] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::WeakFunctionRef * rbx::any_cast<RBX::Lua::WeakFunctionRef,RBX::Region3>(rbx::placement_any<RBX::Region3> *) [0xf2ce84]")]
pub fn stub_0xf2ce84(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Lua::detail::LiveThreadRef>::operator=(RBX::Lua::detail::LiveThreadRef*) [0xf2ce94]")]
pub fn stub_0xf2ce94(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>::list3(boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>)")]
pub fn stub_0xf2cea4() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>::operator()<void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list1<RBX::Lua::IAsyncResult *&>>(boost::_bi::type<void>,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *) &,boost::_bi::list1<RBX::Lua::IAsyncResult *&> &,int)")]
pub fn stub_0xf2ceb4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2ceb4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::value<RBX::Lua::ThreadRef>::value(RBX::Lua::ThreadRef const&) [0xf2cec4]")]
pub fn stub_0xf2cec4() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>>::bind_t(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>> const&)")]
pub fn stub_0xf2ced4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::storage1<boost::_bi::value<RBX::Lua::ThreadRef>>::storage1(boost::_bi::value<RBX::Lua::ThreadRef>) [0xf2cee4]")]
pub fn stub_0xf2cee4() -> crate::slot::BindPiece {
// boost::bind fragment (storage1) composing a host BoundCall.
crate::slot::BindPiece::new("storage1")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>>::storage2(boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>)")]
pub fn stub_0xf2cef4() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>::storage3(boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>)")]
pub fn stub_0xf2cf04() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list_av_3<RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,boost::arg<1>>::type> boost::bind<void,RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *,RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,boost::arg<1>>(void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,boost::arg<1>)")]
pub fn stub_0xf2cf14() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf2cf24(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "j___ZN5boost8functionIFvPN3RBX3Lua12IAsyncResultEEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS8_5list3INS8_5valueISA_EENSH_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2cf34() -> crate::slot::PortedFn {
// IDA 0xf2cf34: j___ZN5boost8functionIFvPN3RBX3Lua12IAsyncResultEEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContext~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2cf34, "j___ZN5boost8functionIFvPN3RBX3Lua12IAsyncResultEEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptr~")
}

#[doc(alias = "boost::function1<void,RBX::Lua::IAsyncResult *>::assign_to_own(boost::function1<void,RBX::Lua::IAsyncResult *> const&) [0xf2cf44]")]
pub fn stub_0xf2cf44(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2bee4]")]
pub fn stub_0xf2bee4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "j___ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2bf74() -> crate::slot::PortedFn {
// IDA 0xf2bf74: j___ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2bf74, "j___ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflect~")
}

#[doc(alias = "j___ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeESQ_")]
pub fn stub_0xf2bfa4() -> crate::slot::PortedFn {
// IDA 0xf2bfa4: j___ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2bfa4, "j___ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflect~")
}

#[doc(alias = "boost::function1<unsigned long,lua_State *>::assign_to_own(boost::function1<unsigned long,lua_State *> const&) [0xf2c044]")]
pub fn stub_0xf2c044(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function1<unsigned long,lua_State *>::clear(void) [0xf2c054]")]
pub fn stub_0xf2c054(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "boost::function1<void,lua_State *>::move_assign(boost::function1<void,lua_State *>&) [0xf2c0a4]")]
pub fn stub_0xf2c0a4() -> crate::slot::PortedFn {
// IDA 0xf2c0a4: boost::function1<void,lua_State *>::move_assign(boost::function1<void,lua_State *>&) [0xf2c0a4].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2c0a4, "boost::function1<void,lua_State *>::move_assign(boost::function1<void,lua_State *>&) [0xf2c0a4]")
}

#[doc(alias = "boost::function1<void,lua_State *>::assign_to_own(boost::function1<void,lua_State *> const&) [0xf2c0b4]")]
pub fn stub_0xf2c0b4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function1<void,lua_State *>::swap(boost::function1<void,lua_State *>&) [0xf2c0c4]")]
pub fn stub_0xf2c0c4() -> crate::slot::PortedFn {
// IDA 0xf2c0c4: boost::function1<void,lua_State *>::swap(boost::function1<void,lua_State *>&) [0xf2c0c4].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2c0c4, "boost::function1<void,lua_State *>::swap(boost::function1<void,lua_State *>&) [0xf2c0c4]")
}

#[doc(alias = "boost::function1<void,lua_State *>::clear(void) [0xf2c0d4]")]
pub fn stub_0xf2c0d4(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>) [0xf2c0f4]")]
pub fn stub_0xf2c0f4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2c124() -> crate::slot::PortedFn {
// IDA 0xf2c124: j___ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2c124, "j___ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10~")
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::clear(void) [0xf2c144]")]
pub fn stub_0xf2c144(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::assign_to_own(boost::function2<void,lua_State *,unsigned long> const&) [0xf2c154]")]
pub fn stub_0xf2c154(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::clear(void) [0xf2c164]")]
pub fn stub_0xf2c164(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "void boost::function2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>) [0xf2c174]")]
pub fn stub_0xf2c174(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2c184() -> crate::slot::PortedFn {
// IDA 0xf2c184: j___ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEE~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2c184, "j___ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2E~")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf2c4c4]")]
pub fn stub_0xf2c4c4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &)const [0xf2c504]")]
pub fn stub_0xf2c504(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2c514]")]
pub fn stub_0xf2c514(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const [0xf2c544]")]
pub fn stub_0xf2c544(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2c554]")]
pub fn stub_0xf2c554(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function1<unsigned long,lua_State *>::operator()(lua_State *)const [0xf2c5e4]")]
pub fn stub_0xf2c5e4(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::function1<void,lua_State *>::operator()(lua_State *)const [0xf2c604]")]
pub fn stub_0xf2c604(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::operator()(lua_State *,unsigned long)const [0xf2c624]")]
pub fn stub_0xf2c624(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::function1<void,RBX::Lua::IAsyncResult *>::clear(void) [0xf2cf54]")]
pub fn stub_0xf2cf54(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "void boost::function1<void,RBX::Lua::IAsyncResult *>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>>)")]
pub fn stub_0xf2cf64(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS7_5list3INS7_5valueIS9_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2cf74() -> crate::slot::PortedFn {
// IDA 0xf2cf74: j___ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextE~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2cf74, "j___ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrI~")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::Lua::IAsyncResult *>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0xf2cf84(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Lua::IAsyncResult *>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0xf2cf94(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Lua::IAsyncResult *>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::WeakPtr<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xf2cfa4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>>::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)const [0xf2cfc4]")]
pub fn stub_0xf2cfc4(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvLuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0xf2cfd4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragTool")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragTool>::shared_ptr<RBX::AdvLuaDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter) [0xf2cfe4]")]
pub fn stub_0xf2cfe4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter) [0xf2cff4]")]
pub fn stub_0xf2cff4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvLuaDragTool,RBX::AdvLuaDragTool>(rbx_core::SharedPtr<RBX::AdvLuaDragTool> const*,RBX::AdvLuaDragTool *)const [0xf2d004]")]
pub fn stub_0xf2d004() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragTool")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E17static_getCreatorEv")]
pub fn stub_0xf2d034() -> crate::slot::PortedFn {
// IDA 0xf2d034: j___ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E17static_getCreatorEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d034, "j___ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E17static_getCr~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorC2Ev")]
pub fn stub_0xf2d044() -> crate::slot::PortedFn {
// IDA 0xf2d044: j___ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorC2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d044, "j___ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorC2Ev")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD2Ev")]
pub fn stub_0xf2d054() -> crate::slot::PortedFn {
// IDA 0xf2d054: j___ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d054, "j___ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD2Ev")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v")]
pub fn stub_0xf2d064() -> crate::slot::PortedFn {
// IDA 0xf2d064: j___ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d064, "j___ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragger> RBX::Creatable<RBX::Instance>::create<RBX::AdvLuaDragger>(void) [0xf2d074]")]
pub fn stub_0xf2d074() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragger")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragger>::shared_ptr<RBX::AdvLuaDragger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2d084]")]
pub fn stub_0xf2d084() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragger")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2d094]")]
pub fn stub_0xf2d094() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "j___ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator12getClassNameEv")]
pub fn stub_0xf2d0a4() -> crate::slot::PortedFn {
// IDA 0xf2d0a4: j___ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator12getClassNameEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d0a4, "j___ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator12get~")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AdvLuaDragger,RBX::AdvLuaDragger>(rbx_core::SharedPtr<RBX::AdvLuaDragger> const*,RBX::AdvLuaDragger *)const [0xf2d0b4]")]
pub fn stub_0xf2d0b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragger")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragTool> RBX::shared_from<RBX::AdvLuaDragTool>(RBX::AdvLuaDragTool*) [0xf2d0e4]")]
pub fn stub_0xf2d0e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragTool")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v")]
pub fn stub_0xf2d0f4() -> crate::slot::PortedFn {
// IDA 0xf2d0f4: j___ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d0f4, "j___ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragger>::operator=(rbx_core::SharedPtr<RBX::AdvLuaDragger> const&) [0xf2d104]")]
pub fn stub_0xf2d104(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0xf2d294() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragTool")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool>::shared_ptr<RBX::LuaDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter) [0xf2d2a4]")]
pub fn stub_0xf2d2a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter) [0xf2d2b4]")]
pub fn stub_0xf2d2b4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LuaDragTool,RBX::LuaDragTool>(rbx_core::SharedPtr<RBX::LuaDragTool> const*,RBX::LuaDragTool *)const [0xf2d2c4]")]
pub fn stub_0xf2d2c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragTool")
}

#[doc(alias = "RBX::Reflection::Call3Helper<RBX::LuaDragger,void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,void>::call(RBX::LuaDragger*,void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,G3D::Vector3 const&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&) [0xf2d314]")]
pub fn stub_0xf2d314() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf2d324]")]
pub fn stub_0xf2d324() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::LuaDragger", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(G3D::Vector3::Axis),char const*,char const*,G3D::Vector3::Axis,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2d334]")]
pub fn stub_0xf2d334() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::LuaDragger", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf2d344]")]
pub fn stub_0xf2d344() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::BoundFuncDesc(void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2d354]")]
pub fn stub_0xf2d354() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc() [0xf2d364]")]
pub fn stub_0xf2d364(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf2d374]")]
pub fn stub_0xf2d374() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::LuaDragger", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(RBX::RbxRay),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2d384]")]
pub fn stub_0xf2d384() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::LuaDragger", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(void),0>::BoundFuncDesc(void (RBX::LuaDragger::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2d394]")]
pub fn stub_0xf2d394() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::LuaDragger", "void", 0)
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E17static_getCreatorEv")]
pub fn stub_0xf2d3e4() -> crate::slot::PortedFn {
// IDA 0xf2d3e4: j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E17static_getCreatorEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d3e4, "j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E17static_getCreatorE~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorC2Ev")]
pub fn stub_0xf2d3f4() -> crate::slot::PortedFn {
// IDA 0xf2d3f4: j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorC2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d3f4, "j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorC2Ev")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD2Ev")]
pub fn stub_0xf2d404() -> crate::slot::PortedFn {
// IDA 0xf2d404: j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d404, "j___ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD2Ev")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v")]
pub fn stub_0xf2d414() -> crate::slot::PortedFn {
// IDA 0xf2d414: j___ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d414, "j___ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger> RBX::Creatable<RBX::Instance>::create<RBX::LuaDragger>(void) [0xf2d424]")]
pub fn stub_0xf2d424() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragger")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger>::shared_ptr<RBX::LuaDragger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2d434]")]
pub fn stub_0xf2d434() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragger")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2d464]")]
pub fn stub_0xf2d464() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "j___ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator12getClassNameEv")]
pub fn stub_0xf2d474() -> crate::slot::PortedFn {
// IDA 0xf2d474: j___ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator12getClassNameEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d474, "j___ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator12getClassN~")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaDragger,RBX::LuaDragger>(rbx_core::SharedPtr<RBX::LuaDragger> const*,RBX::LuaDragger *)const [0xf2d484]")]
pub fn stub_0xf2d484() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragger")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool> RBX::shared_from<RBX::LuaDragTool>(RBX::LuaDragTool*) [0xf2d524]")]
pub fn stub_0xf2d524() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragTool")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sLuaDragToolEEEERKS0_v")]
pub fn stub_0xf2d534() -> crate::slot::PortedFn {
// IDA 0xf2d534: j___ZN3RBX4Name9doDeclareILZNS_12sLuaDragToolEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d534, "j___ZN3RBX4Name9doDeclareILZNS_12sLuaDragToolEEEERKS0_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger>::operator=(rbx_core::SharedPtr<RBX::LuaDragger> const&) [0xf2d544]")]
pub fn stub_0xf2d544(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "bool RBX::LuaWebService::checkCache<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>) [0xf2e914]")]
pub fn stub_0xf2e914() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> c~")
}

#[doc(alias = "bool RBX::LuaWebService::checkCache<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>) [0xf2e924]")]
pub fn stub_0xf2e924(handle: &crate::slot::InstanceHandle) {
// bool RBX::LuaWebService::checkCache<std::string>(std::string const&,boost::function<void (~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "bool RBX::LuaWebService::checkCache<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) [0xf2e934]")]
pub fn stub_0xf2e934(handle: &crate::slot::InstanceHandle) {
// bool RBX::LuaWebService::checkCache<bool>(std::string const&,boost::function<void ()(bool)~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "bool RBX::LuaWebService::checkCache<int>(std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>) [0xf2e944]")]
pub fn stub_0xf2e944(handle: &crate::slot::InstanceHandle) {
// bool RBX::LuaWebService::checkCache<int>(std::string const&,boost::function<void ()(int)>,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>) [0xf2e954]")]
pub fn stub_0xf2e954() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::alloca~")
}

#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>) [0xf2e964]")]
pub fn stub_0xf2e964() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> c~")
}

#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<std::string>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>) [0xf2e974]")]
pub fn stub_0xf2e974(handle: &crate::slot::InstanceHandle) {
// bool RBX::LuaWebService::TryDispatchRequest<std::string>(RBX::AsyncHttpCache<RBX::LuaWebSe~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<bool>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) [0xf2e984]")]
pub fn stub_0xf2e984(handle: &crate::slot::InstanceHandle) {
// bool RBX::LuaWebService::TryDispatchRequest<bool>(RBX::AsyncHttpCache<RBX::LuaWebService::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<int>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>) [0xf2e994]")]
pub fn stub_0xf2e994(handle: &crate::slot::InstanceHandle) {
// bool RBX::LuaWebService::TryDispatchRequest<int>(RBX::AsyncHttpCache<RBX::LuaWebService::C~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "bool RBX::LuaWebService::TryRawDispatchRequest<std::string>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>) [0xf2e9a4]")]
pub fn stub_0xf2e9a4() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::findCacheItem(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo*) [0xf2e9b4]")]
pub fn stub_0xf2e9b4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int) [0xf2e9c4]")]
pub fn stub_0xf2e9c4() -> crate::slot::InstanceHandle {
// RBX::AsyncHttpCache ctor.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache")
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::findCacheItem(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo*) [0xf2e9d4]")]
pub fn stub_0xf2e9d4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int) [0xf2e9e4]")]
pub fn stub_0xf2e9e4() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long) [0xf2e9f4]")]
pub fn stub_0xf2e9f4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::LRUCache(void) [0xf2ea04]")]
pub fn stub_0xf2ea04() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::~LRUCache() [0xf2ea14]")]
pub fn stub_0xf2ea14(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::removeLeastRecentlyUsed(void) [0xf2ea24]")]
pub fn stub_0xf2ea24(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long) [0xf2ea34]")]
pub fn stub_0xf2ea34(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::remove(std::string const&) [0xf2ea44]")]
pub fn stub_0xf2ea44(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::LRUCache(void) [0xf2ea54]")]
pub fn stub_0xf2ea54() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::~LRUCache() [0xf2ea64]")]
pub fn stub_0xf2ea64(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "rbx_core::WeakPtr<RBX::LuaWebService> RBX::weak_from<RBX::LuaWebService>(RBX::LuaWebService*)")]
pub fn stub_0xf2ea74(handle: &crate::slot::InstanceHandle) {
// rbx_core::WeakPtr<RBX::LuaWebService> RBX::weak_from<RBX::LuaWebService>(RBX::LuaWebServic~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService>(rbx_core::WeakPtr<RBX::LuaWebService> const&,boost::detail::sp_nothrow_tag) [0xf2eaa4]")]
pub fn stub_0xf2eaa4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaWebService")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::reset<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *) [0xf2eab4]")]
pub fn stub_0xf2eab4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *) [0xf2eac4]")]
pub fn stub_0xf2eac4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::reset<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *) [0xf2ead4]")]
pub fn stub_0xf2ead4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *) [0xf2eae4]")]
pub fn stub_0xf2eae4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0xf2eb14() -> crate::slot::BindPiece {
// boost::bind fragment (list5) composing a host BoundCall.
crate::slot::BindPiece::new("list5")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
pub fn stub_0xf2eb24(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0xf2eb34() -> crate::slot::BindPiece {
// boost::bind fragment (list5) composing a host BoundCall.
crate::slot::BindPiece::new("list5")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
pub fn stub_0xf2eb44(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0xf2eb54() -> crate::slot::BindPiece {
// boost::bind fragment (list5) composing a host BoundCall.
crate::slot::BindPiece::new("list5")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
pub fn stub_0xf2eb64(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>) [0xf2eb74]")]
pub fn stub_0xf2eb74() -> crate::slot::BindPiece {
// boost::bind fragment (list5) composing a host BoundCall.
crate::slot::BindPiece::new("list5")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int) [0xf2eb84]")]
pub fn stub_0xf2eb84(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>) [0xf2eb94]")]
pub fn stub_0xf2eb94() -> crate::slot::BindPiece {
// boost::bind fragment (list5) composing a host BoundCall.
crate::slot::BindPiece::new("list5")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int) [0xf2eba4]")]
pub fn stub_0xf2eba4(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>::value(rbx_core::WeakPtr<RBX::LuaWebService> const&)")]
pub fn stub_0xf2ebb4() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")]
pub fn stub_0xf2ebd4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
pub fn stub_0xf2ebe4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")]
pub fn stub_0xf2ebf4(call: crate::slot::BoundCall) {
// bind_t dtor — releases the receiver/functor copies.
drop(call);
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")]
pub fn stub_0xf2ec04() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
pub fn stub_0xf2ec14() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")]
pub fn stub_0xf2ec24(call: crate::slot::BoundCall) {
// bind_t dtor — releases the receiver/functor copies.
drop(call);
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")]
pub fn stub_0xf2ec34() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
pub fn stub_0xf2ec44() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")]
pub fn stub_0xf2ec54(call: crate::slot::BoundCall) {
// bind_t dtor — releases the receiver/functor copies.
drop(call);
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&) [0xf2ec64]")]
pub fn stub_0xf2ec64() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&) [0xf2ec74]")]
pub fn stub_0xf2ec74() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")]
pub fn stub_0xf2ec84(call: crate::slot::BoundCall) {
// bind_t dtor — releases the receiver/functor copies.
drop(call);
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")]
pub fn stub_0xf2ec94() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&) [0xf2eca4]")]
pub fn stub_0xf2eca4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")]
pub fn stub_0xf2ecb4(call: crate::slot::BoundCall) {
// bind_t dtor — releases the receiver/functor copies.
drop(call);
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>)")]
pub fn stub_0xf2ecc4() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>)")]
pub fn stub_0xf2ecd4() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>> const&)")]
pub fn stub_0xf2ece4() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>)")]
pub fn stub_0xf2ecf4() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>> const&)")]
pub fn stub_0xf2ed04() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>)")]
pub fn stub_0xf2ed14() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
pub fn stub_0xf2ed24() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0xf2ed34() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>> const&) [0xf2ed44]")]
pub fn stub_0xf2ed44() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>) [0xf2ed54]")]
pub fn stub_0xf2ed54() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>> const&)")]
pub fn stub_0xf2ed64() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>) [0xf2ed74]")]
pub fn stub_0xf2ed74() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0xf2ed84() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0xf2ed94() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0xf2eda4() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>) [0xf2edb4]")]
pub fn stub_0xf2edb4() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>) [0xf2edc4]")]
pub fn stub_0xf2edc4() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_0xf2ede4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_0xf2edf4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
pub fn stub_0xf2ee04() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0xf2ee14() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
pub fn stub_0xf2ee24() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *) [0xf2ee34]")]
pub fn stub_0xf2ee34() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *) [0xf2ee44]")]
pub fn stub_0xf2ee44() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf2ee64(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf2ee74(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf2ee84(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2ee94]")]
pub fn stub_0xf2ee94(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf2eea4]")]
pub fn stub_0xf2eea4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvNS5_IKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIS6_SI_EEEEEEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSY_ISsEENSY_ISS_EENSY_ISU_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS17_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2eeb4() -> crate::slot::PortedFn {
// IDA 0xf2eeb4: j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_pt~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2eeb4, "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bi~")
}

#[doc(alias = "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvNS5_IKSt6vectorINS1_10Reflection7VariantESaISI_EEEEEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSU_ISsEENSU_ISO_EENSU_ISQ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS13_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2eec4() -> crate::slot::PortedFn {
// IDA 0xf2eec4: j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_pt~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2eec4, "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bi~")
}

#[doc(alias = "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvSsEEESH_ENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSL_ISsEENSL_ISH_EESQ_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2eed4() -> crate::slot::PortedFn {
// IDA 0xf2eed4: j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_pt~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2eed4, "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bi~")
}

#[doc(alias = "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvbEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2eee4() -> crate::slot::PortedFn {
// IDA 0xf2eee4: j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_pt~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2eee4, "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bi~")
}

#[doc(alias = "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFviEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2eef4() -> crate::slot::PortedFn {
// IDA 0xf2eef4: j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_pt~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2eef4, "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bi~")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::LuaWebService>::weak_ptr<RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const&,boost::detail::sp_enable_if_convertible<RBX::LuaWebService,RBX::LuaWebService>::type) [0xf2ef14]")]
pub fn stub_0xf2ef14() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaWebService")
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
pub fn stub_0xf2ef74(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
pub fn stub_0xf2ef84(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
pub fn stub_0xf2ef94(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>) [0xf2efa4]")]
pub fn stub_0xf2efa4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>) [0xf2efb4]")]
pub fn stub_0xf2efb4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIS6_SI_EEEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSY_ISsEENSY_ISS_EENSY_ISU_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS17_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2efc4() -> crate::slot::PortedFn {
// IDA 0xf2efc4: j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptr~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2efc4, "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bin~")
}

#[doc(alias = "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt6vectorINS1_10Reflection7VariantESaISI_EEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSU_ISsEENSU_ISO_EENSU_ISQ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS13_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2efd4() -> crate::slot::PortedFn {
// IDA 0xf2efd4: j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptr~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2efd4, "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bin~")
}

#[doc(alias = "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvSsEEESH_ENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSL_ISsEENSL_ISH_EESQ_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2efe4() -> crate::slot::PortedFn {
// IDA 0xf2efe4: j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptr~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2efe4, "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bin~")
}

#[doc(alias = "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvbEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2eff4() -> crate::slot::PortedFn {
// IDA 0xf2eff4: j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptr~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2eff4, "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bin~")
}

#[doc(alias = "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFviEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2f004() -> crate::slot::PortedFn {
// IDA 0xf2f004: j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptr~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2f004, "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bin~")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *) [0xf2f014]")]
pub fn stub_0xf2f014(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long) [0xf2f024]")]
pub fn stub_0xf2f024(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&) [0xf2f034]")]
pub fn stub_0xf2f034(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *) [0xf2f044]")]
pub fn stub_0xf2f044(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *) [0xf2f054]")]
pub fn stub_0xf2f054(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long) [0xf2f064]")]
pub fn stub_0xf2f064(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&) [0xf2f074]")]
pub fn stub_0xf2f074(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *) [0xf2f084]")]
pub fn stub_0xf2f084(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&) [0xf2f094]")]
pub fn stub_0xf2f094(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct(void) [0xf2f0a4]")]
pub fn stub_0xf2f0a4(handle: &crate::slot::InstanceHandle) {
// boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_no~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::~node_constructor() [0xf2f0b4]")]
pub fn stub_0xf2f0b4(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&) [0xf2f0c4]")]
pub fn stub_0xf2f0c4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct(void) [0xf2f0d4]")]
pub fn stub_0xf2f0d4() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::~node_constructor() [0xf2f0e4]")]
pub fn stub_0xf2f0e4(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *) [0xf2f0f4]")]
pub fn stub_0xf2f0f4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *) [0xf2f104]")]
pub fn stub_0xf2f104(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long) [0xf2f114]")]
pub fn stub_0xf2f114(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void) [0xf2f124]")]
pub fn stub_0xf2f124(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long) [0xf2f134]")]
pub fn stub_0xf2f134(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void) [0xf2f144]")]
pub fn stub_0xf2f144(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>> const&) [0xf2f154]")]
pub fn stub_0xf2f154() -> crate::slot::InstanceHandle {
// boost::unordered::detail::table ctor.
crate::slot::InstanceHandle::new("boost::unordered::detail::table")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *) [0xf2f164]")]
pub fn stub_0xf2f164(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *) [0xf2f174]")]
pub fn stub_0xf2f174(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long) [0xf2f184]")]
pub fn stub_0xf2f184(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void) [0xf2f194]")]
pub fn stub_0xf2f194(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long) [0xf2f1a4]")]
pub fn stub_0xf2f1a4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void) [0xf2f1b4]")]
pub fn stub_0xf2f1b4(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> const&) [0xf2f1c4]")]
pub fn stub_0xf2f1c4() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>::destroy(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>*) [0xf2f1d4]")]
pub fn stub_0xf2f1d4() -> crate::slot::PortedFn {
// IDA 0xf2f1d4: __gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf2f1d4, "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRaw~")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)const [0xf2f1e4]")]
pub fn stub_0xf2f1e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)const [0xf2f1f4]")]
pub fn stub_0xf2f1f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>")
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0xf2f234(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0xf2f244(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0xf2f254(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf2f264]")]
pub fn stub_0xf2f264(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf2f274]")]
pub fn stub_0xf2f274(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0xf2f284(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xf2f294(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0xf2f2a4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xf2f2b4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0xf2f2c4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xf2f2d4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const [0xf2f2e4]")]
pub fn stub_0xf2f2e4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2f2f4]")]
pub fn stub_0xf2f2f4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const [0xf2f304]")]
pub fn stub_0xf2f304(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf2f314]")]
pub fn stub_0xf2f314(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const [0xf2f354]")]
pub fn stub_0xf2f354(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const [0xf2f364]")]
pub fn stub_0xf2f364(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const [0xf2f374]")]
pub fn stub_0xf2f374(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const [0xf2f384]")]
pub fn stub_0xf2f384(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const [0xf2f394]")]
pub fn stub_0xf2f394(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const [0xf2f3a4]")]
pub fn stub_0xf2f3a4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_clear(void) [0xf2f3b4]")]
pub fn stub_0xf2f3b4(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_clear(void) [0xf2f3c4]")]
pub fn stub_0xf2f3c4(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>> const&) [0xf2f3d4]")]
pub fn stub_0xf2f3d4(handle: &crate::slot::InstanceHandle) {
// std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebSe~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>) [0xf2f3e4]")]
pub fn stub_0xf2f3e4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>> const&) [0xf2f3f4]")]
pub fn stub_0xf2f3f4() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo> const&) [0xf2f404]")]
pub fn stub_0xf2f404() -> crate::slot::InstanceHandle {
// std::pair ctor.
crate::slot::InstanceHandle::new("std::pair")
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo> const&) [0xf2f414]")]
pub fn stub_0xf2f414() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ScriptInformationProvider,void (RBX::ScriptInformationProvider::*)(std::string),std::string,void>::call(RBX::ScriptInformationProvider*,void (RBX::ScriptInformationProvider::*)(std::string),RBX::Reflection::Variant &,std::string const&) [0xf2fd94]")]
pub fn stub_0xf2fd94(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call1Helper<RBX::ScriptInformationProvider,void (RBX::ScriptInformationPr~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf2fda4]")]
pub fn stub_0xf2fda4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptInformationProvider", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::BoundFuncDesc(void (RBX::ScriptInformationProvider::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf2fdb4]")]
pub fn stub_0xf2fdb4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptInformationProvider", "void", 1)
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::findCacheItem(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo*) [0xf2fdc4]")]
pub fn stub_0xf2fdc4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int) [0xf2fdd4]")]
pub fn stub_0xf2fdd4() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::ScriptInformationProvider::~ScriptInformationProvider() [0xf2fdf4]")]
pub fn stub_0xf2fdf4(handle: crate::slot::InstanceHandle) {
// RBX::ScriptInformationProvider dtor.
drop(handle);
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::removeLeastRecentlyUsed(void) [0xf2fe04]")]
pub fn stub_0xf2fe04(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::insert(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo const&,unsigned long) [0xf2fe14]")]
pub fn stub_0xf2fe14(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::remove(std::string const&) [0xf2fe24]")]
pub fn stub_0xf2fe24(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::LRUCache(void) [0xf2fe34]")]
pub fn stub_0xf2fe34() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::~LRUCache() [0xf2fe44]")]
pub fn stub_0xf2fe44(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptInformationProvider> RBX::weak_from<RBX::ScriptInformationProvider>(RBX::ScriptInformationProvider*) [0xf2fea4]")]
pub fn stub_0xf2fea4(handle: &crate::slot::InstanceHandle) {
// rbx_core::WeakPtr<RBX::ScriptInformationProvider> RBX::weak_from<RBX::ScriptInformationPro~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::reset<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *) [0xf2feb4]")]
pub fn stub_0xf2feb4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::shared_ptr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *) [0xf2fec4]")]
pub fn stub_0xf2fec4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>")
}

#[doc(alias = "boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::list4(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>) [0xf2fed4]")]
pub fn stub_0xf2fed4() -> crate::slot::BindPiece {
// boost::bind fragment (list4) composing a host BoundCall.
crate::slot::BindPiece::new("list4")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int) [0xf2fee4]")]
pub fn stub_0xf2fee4(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>) [0xf2fef4]")]
pub fn stub_0xf2fef4() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>) [0xf2ff04]")]
pub fn stub_0xf2ff04() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>> const&) [0xf2ff14]")]
pub fn stub_0xf2ff14() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>) [0xf2ff24]")]
pub fn stub_0xf2ff24() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "bool boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>::operator()<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<bool>,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *) &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,long) [0xf50a24]")]
pub fn stub_0xf50a24(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf50a24: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
pub fn stub_0xf50a34() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
pub fn stub_0xf50a44(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
pub fn stub_0xf50a54() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
pub fn stub_0xf50a64(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
pub fn stub_0xf50a74() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
pub fn stub_0xf50a84(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>::operator()(void) [0xf50a94]")]
pub fn stub_0xf50a94() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>::operator()<lua_State *>(lua_State * &) [0xf50ab4]")]
pub fn stub_0xf50ab4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>) [0xf50b14]")]
pub fn stub_0xf50b14() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>)")]
pub fn stub_0xf50b24() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>) [0xf50b34]")]
pub fn stub_0xf50b34() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
pub fn stub_0xf50b44() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
pub fn stub_0xf50b54() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
pub fn stub_0xf50b64() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list_av_2<std::string,boost::arg<1>>::type> boost::bind<RBX::Reflection::Variant,std::string,lua_State *,std::string,boost::arg<1>>(RBX::Reflection::Variant (*)(std::string,lua_State *),std::string,boost::arg<1>) [0xf50b74]")]
pub fn stub_0xf50b74() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list_av_3<std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>::type> boost::bind<bool,std::string,RBX::Reflection::Variant const&,lua_State *,std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>(bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>) [0xf50b84]")]
pub fn stub_0xf50b84() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list_av_4<std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,int,boost::arg<1>>::type> boost::bind<bool,std::string,RBX::Reflection::Variant const&,int,lua_State *,std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,int,boost::arg<1>>(bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,int,boost::arg<1>) [0xf50b94]")]
pub fn stub_0xf50b94() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
pub fn stub_0xf50bb4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
pub fn stub_0xf50bc4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>) [0xf50bd4]")]
pub fn stub_0xf50bd4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter) [0xf50be4]")]
pub fn stub_0xf50be4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter) [0xf50bf4]")]
pub fn stub_0xf50bf4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf50c34]")]
pub fn stub_0xf50c34(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf50c54(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf50c64(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf50c74(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "j___ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf50c84() -> crate::slot::PortedFn {
// IDA 0xf50c84: j___ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSB_5list2INSB_~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50c84, "j___ZN5boost8functionIFN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIS3_PFS3_S~")
}

#[doc(alias = "j___ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS8_5list3INS8_5valueISsEENS_17reference_wrapperISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf50c94() -> crate::slot::PortedFn {
// IDA 0xf50c94: j___ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS8_5list3INS8_5~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50c94, "j___ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantE~")
}

#[doc(alias = "j___ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS8_5list4INS8_5valueISsEENS_17reference_wrapperISD_EENSI_IiEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf50ca4() -> crate::slot::PortedFn {
// IDA 0xf50ca4: j___ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS8_5list4INS8_~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50ca4, "j___ZN5boost8functionIFbP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantE~")
}

#[doc(alias = "j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf50cb4() -> crate::slot::PortedFn {
// IDA 0xf50cb4: j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_N~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50cb4, "j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14Sc~")
}

#[doc(alias = "j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf50cc4() -> crate::slot::PortedFn {
// IDA 0xf50cc4: j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_N~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50cc4, "j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14Sc~")
}

#[doc(alias = "j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf50cd4() -> crate::slot::PortedFn {
// IDA 0xf50cd4: j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_N~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50cd4, "j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14Sc~")
}

#[doc(alias = "j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNSC_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES12_")]
pub fn stub_0xf50ce4() -> crate::slot::PortedFn {
// IDA 0xf50ce4: j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_N~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50ce4, "j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14Sc~")
}

#[doc(alias = "j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFNS_10shared_ptrIKSt3mapISsNSC_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES1C_")]
pub fn stub_0xf50cf4() -> crate::slot::PortedFn {
// IDA 0xf50cf4: j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_N~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50cf4, "j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14Sc~")
}

#[doc(alias = "j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES10_")]
pub fn stub_0xf50d04() -> crate::slot::PortedFn {
// IDA 0xf50d04: j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_N~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50d04, "j___ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14Sc~")
}

#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to_own(boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *> const&) [0xf50d44]")]
pub fn stub_0xf50d44(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::clear(void) [0xf50d54]")]
pub fn stub_0xf50d54(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "void boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>) [0xf50d64]")]
pub fn stub_0xf50d64(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5valueISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf50d74() -> crate::slot::PortedFn {
// IDA 0xf50d74: j___ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_SsS5_ENSA_5list2INSA_5~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50d74, "j___ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIS3_PFS3_Ss~")
}

#[doc(alias = "boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::assign_to_own(boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *> const&)")]
pub fn stub_0xf50d84(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::clear(void) [0xf50d94]")]
pub fn stub_0xf50d94(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::assign_to_own(boost::function2<bool,lua_State *,lua_Debug *> const&) [0xf50da4]")]
pub fn stub_0xf50da4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::clear(void) [0xf50db4]")]
pub fn stub_0xf50db4(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "void boost::function2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>) [0xf50dc4]")]
pub fn stub_0xf50dc4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::function2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>) [0xf50dd4]")]
pub fn stub_0xf50dd4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS7_5list3INS7_5valueISsEENS_17reference_wrapperISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf50de4() -> crate::slot::PortedFn {
// IDA 0xf50de4: j___ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES2_ENS7_5list3INS7_5v~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50de4, "j___ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantES~")
}

#[doc(alias = "j___ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS7_5list4INS7_5valueISsEENS_17reference_wrapperISC_EENSH_IiEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf50df4() -> crate::slot::PortedFn {
// IDA 0xf50df4: j___ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiS2_ENS7_5list4INS7_5~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50df4, "j___ZN5boost9function2IbP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEi~")
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::move_assign(boost::function2<void,lua_State *,lua_Debug *>&) [0xf50e04]")]
pub fn stub_0xf50e04() -> crate::slot::PortedFn {
// IDA 0xf50e04: boost::function2<void,lua_State *,lua_Debug *>::move_assign(boost::function2<void,lua_State *,lua_Debug *>&) [0xf50e04].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf50e04, "boost::function2<void,lua_State *,lua_Debug *>::move_assign(boost::function2<void,lua_State *,lua_De~")
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::swap(boost::function2<void,lua_State *,lua_Debug *>&) [0xf50e14]")]
pub fn stub_0xf50e14() -> crate::slot::PortedFn {
// IDA 0xf50e14: boost::function2<void,lua_State *,lua_Debug *>::swap(boost::function2<void,lua_State *,lua_Debug *>&) [0xf50e14].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf50e14, "boost::function2<void,lua_State *,lua_Debug *>::swap(boost::function2<void,lua_State *,lua_Debug *>&~")
}

#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>)")]
pub fn stub_0xf50e24(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>)")]
pub fn stub_0xf50e34(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>)")]
pub fn stub_0xf50e44(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNSB_10Reflection7VariantES2_S4_EEERSG_RNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS11_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf50e54() -> crate::slot::PortedFn {
// IDA 0xf50e54: j___ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50e54, "j___ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14Scr~")
}

#[doc(alias = "j___ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFNS_10shared_ptrIKSt3mapISsNSB_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEES2_S4_EEERSR_RNSF_ISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS1B_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf50e64() -> crate::slot::PortedFn {
// IDA 0xf50e64: j___ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50e64, "j___ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14Scr~")
}

#[doc(alias = "j___ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf50e74() -> crate::slot::PortedFn {
// IDA 0xf50e74: j___ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50e74, "j___ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14Scr~")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::rehash_impl(unsigned long) [0xf50e84]")]
pub fn stub_0xf50e84(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>> &,boost::unordered::detail::ptr_bucket *) [0xf50e94]")]
pub fn stub_0xf50e94(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::erase_key(RBX::Script const* const&) [0xf50ea4]")]
pub fn stub_0xf50ea4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::operator[](RBX::Script const* const&) [0xf50eb4]")]
pub fn stub_0xf50eb4(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::rehash_impl(unsigned long) [0xf50ec4]")]
pub fn stub_0xf50ec4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>> &,boost::unordered::detail::ptr_bucket *) [0xf50ed4]")]
pub fn stub_0xf50ed4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::erase_key(int const&) [0xf50ee4]")]
pub fn stub_0xf50ee4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::operator[](int const&) [0xf50ef4]")]
pub fn stub_0xf50ef4(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>>>::construct(void) [0xf50f04]")]
pub fn stub_0xf50f04(handle: &crate::slot::InstanceHandle) {
// boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_no~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>>>::construct(void) [0xf50f14]")]
pub fn stub_0xf50f14(handle: &crate::slot::InstanceHandle) {
// boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_no~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *) [0xf50f24]")]
pub fn stub_0xf50f24(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *) [0xf50f34]")]
pub fn stub_0xf50f34(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::create_buckets(unsigned long) [0xf50f44]")]
pub fn stub_0xf50f44(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::delete_buckets(void) [0xf50f54]")]
pub fn stub_0xf50f54(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::reserve_for_insert(unsigned long) [0xf50f64]")]
pub fn stub_0xf50f64(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::table(unsigned long,boost::hash<RBX::Script const*> const&,std::equal_to<RBX::Script const*> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>> const&) [0xf50f74]")]
pub fn stub_0xf50f74() -> crate::slot::InstanceHandle {
// boost::unordered::detail::table ctor — fresh debugger identity.
crate::slot::InstanceHandle::new("boost::unordered::detail::table")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *) [0xf50f84]")]
pub fn stub_0xf50f84(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *) [0xf50f94]")]
pub fn stub_0xf50f94(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::create_buckets(unsigned long) [0xf50fa4]")]
pub fn stub_0xf50fa4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::delete_buckets(void) [0xf50fb4]")]
pub fn stub_0xf50fb4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::reserve_for_insert(unsigned long) [0xf50fc4]")]
pub fn stub_0xf50fc4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const [0xf50fd4]")]
pub fn stub_0xf50fd4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "j___ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7Creator12getClassNameEv")]
pub fn stub_0xf50ff4() -> crate::slot::PortedFn {
// IDA 0xf50ff4: j___ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7Creator12ge~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf50ff4, "j___ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpoi~")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::DebuggerManager,RBX::Scripting::DebuggerManager>(rbx_core::SharedPtr<RBX::Scripting::DebuggerManager> const*,RBX::Scripting::DebuggerManager *)const [0xf51014]")]
pub fn stub_0xf51014() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerManager")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::DebuggerBreakpoint,RBX::Scripting::DebuggerBreakpoint>(rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> const*,RBX::Scripting::DebuggerBreakpoint *)const [0xf51024]")]
pub fn stub_0xf51024() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerBreakpoint")
}

#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &)const")]
pub fn stub_0xf51034(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &)const")]
pub fn stub_0xf51044(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &)const")]
pub fn stub_0xf51054(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &)const [0xf51084]")]
pub fn stub_0xf51084(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf51094]")]
pub fn stub_0xf51094(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<true>)const [0xf510a4]")]
pub fn stub_0xf510a4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf510b4]")]
pub fn stub_0xf510b4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,boost::detail::function::function_buffer &)const [0xf510c4]")]
pub fn stub_0xf510c4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list3<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf510d4]")]
pub fn stub_0xf510d4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,boost::detail::function::function_buffer &)const [0xf510e4]")]
pub fn stub_0xf510e4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<bool,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>>(boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list4<boost::_bi::value<std::string>,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::_bi::value<int>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf510f4]")]
pub fn stub_0xf510f4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0xf51104(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0xf51114(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0xf51124(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0xf51134(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>,RBX::Reflection::Variant&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<RBX::Reflection::Variant>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xf51144(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0xf51154(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xf51164(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0xf51174(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xf51184(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const [0xf511a4]")]
pub fn stub_0xf511a4(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::function2<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
pub fn stub_0xf511b4(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const [0xf511c4]")]
pub fn stub_0xf511c4(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const [0xf511d4]")]
pub fn stub_0xf511d4(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::find_node_impl<RBX::Script const*,std::equal_to<RBX::Script const*>>(unsigned long,RBX::Script const* const&,std::equal_to<RBX::Script const*> const&)const [0xf511e4]")]
pub fn stub_0xf511e4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::find_node_impl<int,std::equal_to<int>>(unsigned long,int const&,std::equal_to<int> const&)const [0xf511f4]")]
pub fn stub_0xf511f4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::min_buckets_for_size(unsigned long)const [0xf51204]")]
pub fn stub_0xf51204(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::min_buckets_for_size(unsigned long)const [0xf51214]")]
pub fn stub_0xf51214(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::_Vector_base<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::_M_allocate(unsigned long) [0xf51224]")]
pub fn stub_0xf51224(handle: &crate::slot::InstanceHandle) {
// std::_Vector_base<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripti~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Vector_base<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::_M_allocate(unsigned long) [0xf51244]")]
pub fn stub_0xf51244(handle: &crate::slot::InstanceHandle) {
// std::_Vector_base<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerW~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::FunctionInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *>(RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *) [0xf51254]")]
pub fn stub_0xf51254(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::FunctionInfo * std::__copy_backward<false,std::random_acce~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Scripting::ScriptDebugger::FunctionInfo*,std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>>,RBX::Scripting::ScriptDebugger::FunctionInfo const&) [0xf51284]")]
pub fn stub_0xf51284(handle: &crate::slot::InstanceHandle) {
// std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::Sc~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::push_back(RBX::Scripting::ScriptDebugger::FunctionInfo const&) [0xf51294]")]
pub fn stub_0xf51294(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::~vector() [0xf512a4]")]
pub fn stub_0xf512a4(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&) [0xf512c4]")]
pub fn stub_0xf512c4(handle: &crate::slot::InstanceHandle) {
// std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::push_back(RBX::Scripting::DebuggerWatch * const&) [0xf512d4]")]
pub fn stub_0xf512d4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::FunctionInfo * std::__uninitialized_copy_a<RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo>(RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>) [0xf51314]")]
pub fn stub_0xf51314(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::FunctionInfo * std::__uninitialized_copy_a<RBX::Scripting:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch *>(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&,std::random_access_iterator_tag) [0xf51324]")]
pub fn stub_0xf51324() -> crate::slot::PortedFn {
// IDA 0xf51324: __gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf51324, "__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWa~")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>> std::remove<__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch *>(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&) [0xf51334]")]
pub fn stub_0xf51334(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::TestService,void ()(void),void,0>::BoundYieldFuncDesc(void (RBX::TestService::*)(boost::function<void ()(void)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf54a44]")]
pub fn stub_0xf54a44() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::TestService", "void", 0)
}

#[doc(alias = "RBX::ScriptContext::ScriptStartOptions::LuaSyntaxError::LuaSyntaxError(int,std::exception &) [0xf54b74]")]
pub fn stub_0xf54b74(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "RBX::Lua::ArgumentParser::getClosing(char) [0xf54c54]")]
pub fn stub_0xf54c54(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Lua::ArgumentParser getter.
cell.get()
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseString<__gnu_cxx::__normal_iterator<char const*,std::string>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>) [0xf54c64]")]
pub fn stub_0xf54c64() -> crate::slot::PortedFn {
// IDA 0xf54c64: __gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseString<__gnu_cxx::__normal_iterator~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf54c64, "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseString<__gnu_cx~")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseBracket<__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>),boost::_bi::list3<boost::_bi::value<std::vector<std::string,std::allocator<std::string>> *>,boost::arg<1>,boost::arg<2>>>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>),boost::_bi::list3<boost::_bi::value<std::vector<std::string,std::allocator<std::string>> *>,boost::arg<1>,boost::arg<2>>>) [0xf54c74]")]
pub fn stub_0xf54c74() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 4 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(4)
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseBracket<__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(void),boost::_bi::list0>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(void),boost::_bi::list0>) [0xf54c84]")]
pub fn stub_0xf54c84() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parse_arg<__gnu_cxx::__normal_iterator<char const*,std::string>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,char) [0xf54c94]")]
pub fn stub_0xf54c94() -> crate::slot::PortedFn {
// IDA 0xf54c94: __gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parse_arg<__gnu_cxx::__normal_iterator<c~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf54c94, "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parse_arg<__gnu_cxx:~")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script> boost::dynamic_pointer_cast<RBX::Script,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&) [0xf54f04]")]
pub fn stub_0xf54f04() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list4<char const*&,char const*&,rbx_core::SharedPtr<RBX::BaseScript>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int> &,boost::_bi::list4<char const*&,char const*&,rbx_core::SharedPtr<RBX::BaseScript>&,int &> &,int) [0xf54fa4]")]
pub fn stub_0xf54fa4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf54fa4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list_av_6<rbx_core::SharedPtr<RBX::TestService>,int,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int,rbx_core::SharedPtr<RBX::TestService>,int,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::TestService::*)(int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int),rbx_core::SharedPtr<RBX::TestService>,int,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>) [0xf55064]")]
pub fn stub_0xf55064() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf550d4]")]
pub fn stub_0xf550d4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "j___ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf55114() -> crate::slot::PortedFn {
// IDA 0xf55114: j___ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceE~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55114, "j___ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5~")
}

#[doc(alias = "j___ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS8_E4typeEST_")]
pub fn stub_0xf55124() -> crate::slot::PortedFn {
// IDA 0xf55124: j___ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceE~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55124, "j___ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5~")
}

#[doc(alias = "void boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>) [0xf551d4]")]
pub fn stub_0xf551d4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf551e4() -> crate::slot::PortedFn {
// IDA 0xf551e4: j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEi~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf551e4, "j___ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf5I~")
}

#[doc(alias = "void boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::call<rbx_core::SharedPtr<RBX::TestService>,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>(rbx_core::SharedPtr<RBX::TestService> &,void const*,int &,char const* &,char const* &,rbx_core::SharedPtr<RBX::BaseScript> &,int &)const [0xf552a4]")]
pub fn stub_0xf552a4() -> crate::slot::BindPiece {
// boost::bind fragment (mf5) composing a host BoundCall.
crate::slot::BindPiece::new("mf5")
}

#[doc(alias = "void boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf55344]")]
pub fn stub_0xf55344(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const [0xf55354]")]
pub fn stub_0xf55354(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf55364]")]
pub fn stub_0xf55364(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::LibraryService::LibraryStateObject::LibraryStateObject(lua_State *,std::string const&,bool) [0xf554a4]")]
pub fn stub_0xf554a4() -> crate::slot::InstanceHandle {
// RBX::LibraryService::LibraryStateObject ctor.
crate::slot::InstanceHandle::new("RBX::LibraryService::LibraryStateObject")
}

#[doc(alias = "RBX::Lua::Library* RBX::Lua::Bridge<RBX::Lua::Library,true>::pushNewObject<RBX::Lua::Library>(lua_State *,RBX::Lua::Library) [0xf554b4]")]
pub fn stub_0xf554b4(thread: &mut crate::lua::LuaThreadState, value: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// Bridge pushNewObject — pushes the host handle identity.
thread.push(crate::lua::LuaStackValue::Number(value.id as f64));
*value
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_12sLuaSettingsEEEERKS0_v")]
pub fn stub_0xf554c4() -> crate::slot::PortedFn {
// IDA 0xf554c4: j___ZN3RBX4Name7declareILZNS_12sLuaSettingsEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf554c4, "j___ZN3RBX4Name7declareILZNS_12sLuaSettingsEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sLuaSettingsEEEERKS0_v")]
pub fn stub_0xf554d4() -> crate::slot::PortedFn {
// IDA 0xf554d4: j___ZN3RBX4Name9doDeclareILZNS_12sLuaSettingsEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf554d4, "j___ZN3RBX4Name9doDeclareILZNS_12sLuaSettingsEEEERKS0_v")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptContext> RBX::weak_from<RBX::ScriptContext>(RBX::ScriptContext*) [0xf554e4]")]
pub fn stub_0xf554e4(handle: &crate::slot::InstanceHandle) {
// rbx_core::WeakPtr<RBX::ScriptContext> RBX::weak_from<RBX::ScriptContext>(RBX::ScriptContex~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script>::operator=(rbx_core::SharedPtr<RBX::Script> const&) [0xf55534]")]
pub fn stub_0xf55534(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "void rbx_core::SharedPtr_release<RBX::Lua::WeakThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef,int,0> const*) [0xf555a4]")]
pub fn stub_0xf555a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef")
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::list6(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>) [0xf55644]")]
pub fn stub_0xf55644() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&> &,int) [0xf55654]")]
pub fn stub_0xf55654(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf55654: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>) [0xf55684]")]
pub fn stub_0xf55684() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&) [0xf55694]")]
pub fn stub_0xf55694() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>) [0xf556a4]")]
pub fn stub_0xf556a4() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>) [0xf556b4]")]
pub fn stub_0xf556b4() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>) [0xf556c4]")]
pub fn stub_0xf556c4() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>) [0xf556d4]")]
pub fn stub_0xf556d4() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>) [0xf55724]")]
pub fn stub_0xf55724() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf55764]")]
pub fn stub_0xf55764(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf55784() -> crate::slot::PortedFn {
// IDA 0xf55784: j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_pt~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55784, "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bi~")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptContext>::weak_ptr<RBX::ScriptContext>(rbx_core::SharedPtr<RBX::ScriptContext> const&,boost::detail::sp_enable_if_convertible<RBX::ScriptContext,RBX::ScriptContext>::type) [0xf557c4]")]
pub fn stub_0xf557c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>) [0xf55834]")]
pub fn stub_0xf55834(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf55844() -> crate::slot::PortedFn {
// IDA 0xf55844: j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptr~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf55844, "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bin~")
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf55924]")]
pub fn stub_0xf55924(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const [0xf55934]")]
pub fn stub_0xf55934(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf55944]")]
pub fn stub_0xf55944(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::Script>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::operator[](std::string const&) [0xf559a4]")]
pub fn stub_0xf559a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>::pair(std::string const&,rbx_core::SharedPtr<RBX::Script> const&) [0xf55a44]")]
pub fn stub_0xf55a44() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::lower_bound(std::string const&) [0xf55ac4]")]
pub fn stub_0xf55ac4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&) [0xf55ad4]")]
pub fn stub_0xf55ad4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&) [0xf55ae4]")]
pub fn stub_0xf55ae4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&) [0xf55af4]")]
pub fn stub_0xf55af4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::find(std::string const&) [0xf55b04]")]
pub fn stub_0xf55b04() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&) [0xf55b14]")]
pub fn stub_0xf55b14() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "lua_exception::~lua_exception() [0xf55c24]")]
pub fn stub_0xf55c24(msg: String) {
// exception dtor.
drop(msg);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::PersonalServerService,std::string ()(int),std::string,1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf58814]")]
pub fn stub_0xf58814() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::PersonalServerService", "std::string", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::PersonalServerService,std::string ()(int),std::string,1>::BoundYieldFuncDesc(void (RBX::PersonalServerService::*)(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf58824]")]
pub fn stub_0xf58824() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::PersonalServerService", "std::string", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf58f34]")]
pub fn stub_0xf58f34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BindableFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::BindableFunction::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf58f44]")]
pub fn stub_0xf58f44() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf59104]")]
pub fn stub_0xf59104() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::GamePassService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::GamePassService::*)(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf59114]")]
pub fn stub_0xf59114() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf5a4a4]")]
pub fn stub_0xf5a4a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5a4b4]")]
pub fn stub_0xf5a4b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf5a4c4]")]
pub fn stub_0xf5a4c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::MarketplaceService,bool ()(rbx_core::SharedPtr<RBX::Instance>,int),bool,2>::BoundYieldFuncDesc(void (RBX::MarketplaceService::*)(rbx_core::SharedPtr<RBX::Instance>,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5a4d4]")]
pub fn stub_0xf5a4d4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LuaWebService>(void) [0xf5a604]")]
pub fn stub_0xf5a604() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "RBX::LuaWebService * RBX::ServiceProvider::create<RBX::LuaWebService>(RBX::Instance const*) [0xf5a614]")]
pub fn stub_0xf5a614() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::LuaWebService")
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sLuaWebServiceEEEERKS0_v")]
pub fn stub_0xf5a644() -> crate::slot::PortedFn {
// IDA 0xf5a644: j___ZN3RBX4Name7declareILZNS_14sLuaWebServiceEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf5a644, "j___ZN3RBX4Name7declareILZNS_14sLuaWebServiceEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sLuaWebServiceEEEERKS0_v")]
pub fn stub_0xf5a654() -> crate::slot::PortedFn {
// IDA 0xf5a654: j___ZN3RBX4Name9doDeclareILZNS_14sLuaWebServiceEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf5a654, "j___ZN3RBX4Name9doDeclareILZNS_14sLuaWebServiceEEEERKS0_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter) [0xf5aa54]")]
pub fn stub_0xf5aa54() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaWebService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter) [0xf5ac54]")]
pub fn stub_0xf5ac54() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::LuaWebService * RBX::ServiceProvider::find<RBX::LuaWebService>(void)const [0xf5af04]")]
pub fn stub_0xf5af04() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::LuaWebService"))
}

#[doc(alias = "RBX::LuaWebService * RBX::ServiceProvider::create<RBX::LuaWebService>(void)const [0xf5af14]")]
pub fn stub_0xf5af14() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::LuaWebService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaWebService,RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const*,RBX::LuaWebService *)const [0xf5af24]")]
pub fn stub_0xf5af24() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaWebService")
}

#[doc(alias = "RBX::ScriptService::Info::~Info() [0xf5b3f4]")]
pub fn stub_0xf5b3f4(handle: crate::slot::InstanceHandle) {
// RBX::ScriptService::Info dtor.
drop(handle);
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>> const&) [0xf5b404]")]
pub fn stub_0xf5b404() -> crate::slot::SlotConnection {
// IDA 0xf5b404: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info>::shared_ptr<RBX::ScriptService::Info>(RBX::ScriptService::Info *) [0xf5b414]")]
pub fn stub_0xf5b414() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info>::operator=(rbx_core::SharedPtr<RBX::ScriptService::Info> const&) [0xf5b424]")]
pub fn stub_0xf5b424(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int) [0xf5b434]")]
pub fn stub_0xf5b434(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf5b434: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptService::Info>(RBX::ScriptService::Info *) [0xf5b444]")]
pub fn stub_0xf5b444() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ScriptService*,rbx_core::SharedPtr<RBX::Instance>)const [0xf5b454]")]
pub fn stub_0xf5b454() -> crate::slot::BindPiece {
// boost::bind fragment (mf1) composing a host BoundCall.
crate::slot::BindPiece::new("mf1")
}

#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::_M_allocate(unsigned long) [0xf5b464]")]
pub fn stub_0xf5b464() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *>(rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *) [0xf5b474]")]
pub fn stub_0xf5b474() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *>(rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *) [0xf5b484]")]
pub fn stub_0xf5b484() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info>*,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,rbx_core::SharedPtr<RBX::ScriptService::Info> const&) [0xf5b494]")]
pub fn stub_0xf5b494() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::ScriptService::Info>*) [0xf5b4a4]")]
pub fn stub_0xf5b4a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::erase(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info>*,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info>*,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>) [0xf5b4b4]")]
pub fn stub_0xf5b4b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::push_back(rbx_core::SharedPtr<RBX::ScriptService::Info> const&) [0xf5b4c4]")]
pub fn stub_0xf5b4c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>> std::remove_copy_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>) [0xf5b4d4]")]
pub fn stub_0xf5b4d4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>> std::__find_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>,std::random_access_iterator_tag) [0xf5b4e4]")]
pub fn stub_0xf5b4e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>> std::remove_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>) [0xf5b4f4]")]
pub fn stub_0xf5b4f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf5bb34]")]
pub fn stub_0xf5bb34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::BoundYieldFuncDesc(void (RBX::OverlayDataModel::*)(int,int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5bb44]")]
pub fn stub_0xf5bb44() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,int ()(std::string,std::string,int),int,3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf5bb54]")]
pub fn stub_0xf5bb54() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "int", 3)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,int ()(std::string,std::string,int),int,3>::BoundYieldFuncDesc(void (RBX::OverlayDataModel::*)(std::string,std::string,int,boost::function<void ()(int)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5bb64]")]
pub fn stub_0xf5bb64() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "int", 3)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int),void,1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf5bb74]")]
pub fn stub_0xf5bb74() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int),void,1>::BoundYieldFuncDesc(void (RBX::OverlayDataModel::*)(int,boost::function<void ()(void)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5bb84]")]
pub fn stub_0xf5bb84() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int,std::string),void,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf5bb94]")]
pub fn stub_0xf5bb94() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(int,std::string),void,2>::BoundYieldFuncDesc(void (RBX::OverlayDataModel::*)(int,std::string,boost::function<void ()(void)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,std::string,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5bba4]")]
pub fn stub_0xf5bba4() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::OverlayDataModel,void ()(void),void,0>::BoundYieldFuncDesc(void (RBX::OverlayDataModel::*)(boost::function<void ()(void)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5bbb4]")]
pub fn stub_0xf5bbb4() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::OverlayDataModel", "void", 0)
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sScriptContextEEEERKS0_v")]
pub fn stub_0xf5bcc4() -> crate::slot::PortedFn {
// IDA 0xf5bcc4: j___ZN3RBX4Name7declareILZNS_14sScriptContextEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf5bcc4, "j___ZN3RBX4Name7declareILZNS_14sScriptContextEEEERKS0_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptContext>::shared_ptr<RBX::ScriptContext,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter) [0xf5bd54]")]
pub fn stub_0xf5bd54() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptContext,RBX::ScriptContext>(rbx_core::SharedPtr<RBX::ScriptContext> const*,RBX::ScriptContext *)const [0xf5c334]")]
pub fn stub_0xf5c334() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf5c5c4]")]
pub fn stub_0xf5c5c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5c5d4]")]
pub fn stub_0xf5c5d4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf5c5e4]")]
pub fn stub_0xf5c5e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int,int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5c5f4]")]
pub fn stub_0xf5c5f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf5c604]")]
pub fn stub_0xf5c604() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> c~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,RBX::AssetService::AccessType,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::AssetService::AccessType,char const*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5c614]")]
pub fn stub_0xf5c614() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> c~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,RBX::AssetService::AccessType,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),bool,3>::~BoundYieldFuncDesc() [0xf5c624]")]
pub fn stub_0xf5c624(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf5c634]")]
pub fn stub_0xf5c634() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::AssetService", "bool", 2)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,bool ()(int,int),bool,2>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5c644]")]
pub fn stub_0xf5c644() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::AssetService", "bool", 2)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,int ()(int),int,1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf5c654]")]
pub fn stub_0xf5c654() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::AssetService", "int", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::AssetService,int ()(int),int,1>::BoundYieldFuncDesc(void (RBX::AssetService::*)(int,boost::function<void ()(int)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5c664]")]
pub fn stub_0xf5c664() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::AssetService", "int", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf5cb04]")]
pub fn stub_0xf5cb04() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,2>::BoundYieldFuncDesc(void (RBX::RemoteFunction::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5cb14]")]
pub fn stub_0xf5cb14() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf5cb24]")]
pub fn stub_0xf5cb24() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::RemoteFunction::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf5cb34]")]
pub fn stub_0xf5cb34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_6ScriptELZNS_7sScriptEENS_14FactoryProductIS2_NS_10BaseScriptELZNS_7sScriptEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf600e4() -> crate::slot::PortedFn {
// IDA 0xf600e4: j___ZN3RBX10Reflection9DescribedINS_6ScriptELZNS_7sScriptEENS_14FactoryProductIS2_NS_10BaseScriptELZNS_7sScriptEENS_8Ins~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf600e4, "j___ZN3RBX10Reflection9DescribedINS_6ScriptELZNS_7sScriptEENS_14FactoryProductIS2_NS_10BaseScriptELZ~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0xf60a14() -> crate::slot::PortedFn {
// IDA 0xf60a14: j___ZN3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7CreatorC2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf60a14, "j___ZN3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7CreatorC2~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0xf60a24() -> crate::slot::PortedFn {
// IDA 0xf60a24: j___ZN3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7CreatorD2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf60a24, "j___ZN3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7CreatorD2~")
}

#[doc(alias = "RBX::ScriptInformationProvider * RBX::ServiceProvider::find<RBX::ScriptInformationProvider>(void)const [0xf61594]")]
pub fn stub_0xf61594() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::ScriptInformationProvider"))
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptInformationProvider,RBX::ScriptInformationProvider>(rbx_core::SharedPtr<RBX::ScriptInformationProvider> const*,RBX::ScriptInformationProvider *)const [0xf61654]")]
pub fn stub_0xf61654() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf62074]")]
pub fn stub_0xf62074() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash~")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(int),std::string,1>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf62084]")]
pub fn stub_0xf62084() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "std::string", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(int),bool,1>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf62094]")]
pub fn stub_0xf62094() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "bool", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(void),bool,0>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf620a4]")]
pub fn stub_0xf620a4() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "bool", 0)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,int ()(int),int,1>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(int,boost::function<void ()(int)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf620b4]")]
pub fn stub_0xf620b4() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::Network::Player", "int", 1)
}

#[doc(alias = "Ogre::ShaderScriptListener::getAutogeneratedShaderPath(Ogre::CreateHighLevelGpuProgramScriptCompilerEvent const&) [0xf651b4]")]
pub fn stub_0xf651b4() -> crate::slot::PortedFn {
// IDA 0xf651b4: Ogre::ShaderScriptListener::getAutogeneratedShaderPath(Ogre::CreateHighLevelGpuProgramScriptCompilerEvent const&) [0xf65~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf651b4, "Ogre::ShaderScriptListener::getAutogeneratedShaderPath(Ogre::CreateHighLevelGpuProgramScriptCompiler~")
}

#[doc(alias = "Ogre::ShaderScriptListener::getAutogeneratedShaderPath(std::string const&,std::string const&,std::string const&) [0xf651c4]")]
pub fn stub_0xf651c4() -> crate::slot::PortedFn {
// IDA 0xf651c4: Ogre::ShaderScriptListener::getAutogeneratedShaderPath(std::string const&,std::string const&,std::string const&) [0xf651~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf651c4, "Ogre::ShaderScriptListener::getAutogeneratedShaderPath(std::string const&,std::string const&,std::st~")
}

#[doc(alias = "Ogre::MaterialScriptContext::~MaterialScriptContext() [0xf67db4]")]
pub fn stub_0xf67db4(handle: crate::slot::InstanceHandle) {
// Ogre::MaterialScriptContext dtor.
drop(handle);
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,std::_Select1st<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)> const&) [0xf67dc4]")]
pub fn stub_0xf67dc4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,std::_Select1st<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>> *) [0xf67dd4]")]
pub fn stub_0xf67dd4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,std::_Select1st<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)> const&) [0xf67de4]")]
pub fn stub_0xf67de4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<float,std::pair<float const,Ogre::ScriptLoader *>,std::_Select1st<std::pair<float const,Ogre::ScriptLoader *>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,Ogre::ScriptLoader *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,Ogre::ScriptLoader *>> *) [0xf68f24]")]
pub fn stub_0xf68f24(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "Ogre::ScriptCompiler::~ScriptCompiler() [0xf69be4]")]
pub fn stub_0xf69be4() -> crate::slot::PortedFn {
// IDA 0xf69be4: Ogre::ScriptCompiler::~ScriptCompiler() [0xf69be4].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf69be4, "Ogre::ScriptCompiler::~ScriptCompiler() [0xf69be4]")
}

#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptCompiler::Error>::destroy(void) [0xf69c54]")]
pub fn stub_0xf69c54() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::ScriptCompiler::Error")
}

#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void) [0xf69ca4]")]
pub fn stub_0xf69ca4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPt~")
}

#[doc(alias = "std::vector<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ScriptTranslatorManager **,std::vector<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ScriptTranslatorManager * const&) [0xf69d54]")]
pub fn stub_0xf69d54(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptToken>::destroy(void) [0xf69e84]")]
pub fn stub_0xf69e84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::ScriptToken")
}

#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptToken>::operator=(Ogre::SharedPtr<Ogre::ScriptToken> const&) [0xf69e94]")]
pub fn stub_0xf69e94(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SharedPtr<Ogre::ScriptToken>*,std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SharedPtr<Ogre::ScriptToken> const&) [0xf69ea4]")]
pub fn stub_0xf69ea4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::ScriptToken")
}

#[doc(alias = "Ogre::CreateMaterialScriptCompilerEvent::CreateMaterialScriptCompilerEvent(std::string const&,std::string const&,std::string const&) [0xf69eb4]")]
pub fn stub_0xf69eb4() -> crate::slot::PortedFn {
// IDA 0xf69eb4: Ogre::CreateMaterialScriptCompilerEvent::CreateMaterialScriptCompilerEvent(std::string const&,std::string const&,std::st~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf69eb4, "Ogre::CreateMaterialScriptCompilerEvent::CreateMaterialScriptCompilerEvent(std::string const&,std::s~")
}

#[doc(alias = "Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent() [0xf69ec4]")]
pub fn stub_0xf69ec4() -> crate::slot::PortedFn {
// IDA 0xf69ec4: Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent() [0xf69ec4].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf69ec4, "Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent() [0xf69ec4]")
}

#[doc(alias = "Ogre::CreateCompositorScriptCompilerEvent::CreateCompositorScriptCompilerEvent(std::string const&,std::string const&,std::string const&) [0xf69ed4]")]
pub fn stub_0xf69ed4() -> crate::slot::PortedFn {
// IDA 0xf69ed4: Ogre::CreateCompositorScriptCompilerEvent::CreateCompositorScriptCompilerEvent(std::string const&,std::string const&,std~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf69ed4, "Ogre::CreateCompositorScriptCompilerEvent::CreateCompositorScriptCompilerEvent(std::string const&,st~")
}

#[doc(alias = "Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent() [0xf69ee4]")]
pub fn stub_0xf69ee4() -> crate::slot::PortedFn {
// IDA 0xf69ee4: Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent() [0xf69ee4].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf69ee4, "Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent() [0xf69ee4]")
}

#[doc(alias = "Ogre::CreateGpuProgramScriptCompilerEvent::CreateGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType) [0xf69ef4]")]
pub fn stub_0xf69ef4() -> crate::slot::PortedFn {
// IDA 0xf69ef4: Ogre::CreateGpuProgramScriptCompilerEvent::CreateGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf69ef4, "Ogre::CreateGpuProgramScriptCompilerEvent::CreateGpuProgramScriptCompilerEvent(std::string const&,st~")
}

#[doc(alias = "Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent() [0xf69f04]")]
pub fn stub_0xf69f04() -> crate::slot::PortedFn {
// IDA 0xf69f04: Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent() [0xf69f04].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf69f04, "Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent() [0xf69f04]")
}

#[doc(alias = "Ogre::CreateParticleSystemScriptCompilerEvent::CreateParticleSystemScriptCompilerEvent(std::string const&,std::string const&,std::string const&) [0xf69f14]")]
pub fn stub_0xf69f14() -> crate::slot::PortedFn {
// IDA 0xf69f14: Ogre::CreateParticleSystemScriptCompilerEvent::CreateParticleSystemScriptCompilerEvent(std::string const&,std::string co~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf69f14, "Ogre::CreateParticleSystemScriptCompilerEvent::CreateParticleSystemScriptCompilerEvent(std::string c~")
}

#[doc(alias = "Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent() [0xf69f24]")]
pub fn stub_0xf69f24() -> crate::slot::PortedFn {
// IDA 0xf69f24: Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent() [0xf69f24].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf69f24, "Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent() [0xf69f24]")
}

#[doc(alias = "Ogre::CreateGpuSharedParametersScriptCompilerEvent::CreateGpuSharedParametersScriptCompilerEvent(std::string const&,std::string const&,std::string const&) [0xf69f34]")]
pub fn stub_0xf69f34() -> crate::slot::PortedFn {
// IDA 0xf69f34: Ogre::CreateGpuSharedParametersScriptCompilerEvent::CreateGpuSharedParametersScriptCompilerEvent(std::string const&,std:~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf69f34, "Ogre::CreateGpuSharedParametersScriptCompilerEvent::CreateGpuSharedParametersScriptCompilerEvent(std~")
}

#[doc(alias = "Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent() [0xf69f44]")]
pub fn stub_0xf69f44() -> crate::slot::PortedFn {
// IDA 0xf69f44: Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent() [0xf69f44].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf69f44, "Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent() ~")
}

#[doc(alias = "Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::CreateHighLevelGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType,std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*) [0xf69f54]")]
pub fn stub_0xf69f54() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent() [0xf69f64]")]
pub fn stub_0xf69f64() -> crate::slot::PortedFn {
// IDA 0xf69f64: Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent() [0xf69f64].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf69f64, "Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent() ~")
}

#[cfg(test)]
mod script_context_tests {
    use super::*;

    #[test]
    fn class_names_match_declare_tags() {
        assert_eq!(stub_0xf2c2f4(), "LocalScript");
        assert_eq!(stub_0xf2c304(), "LuaSettings");
        assert_eq!(stub_0xf2c314(), "DebuggerWatch");
        assert_eq!(stub_0xf2c324(), "ScriptDebugger");
    }

    #[test]
    fn service_registry_finds_and_creates_once() {
        let mut reg = ScriptContextRegistry::default();
        assert!(stub_0xf2c344(&reg).is_none());
        let a = stub_0xf2c364(&mut reg, "game");
        let b = stub_0xf2c364(&mut reg, "other");
        assert_eq!(a.name, "game");
        assert!(SharedPtr::ptr_eq(&a, &b));
        assert!(stub_0xf2c344(&reg).is_some());
    }

    #[test]
    fn breakpoint_table_and_script_entry_drop() {
        let mut table = stub_0xf2c2b4(8);
        table.map.insert(7, DebuggerBreakpoint { id: 7, enabled: true });
        assert!(table.map[&7].enabled);
        stub_0xf2c2d4(("x".to_owned(), SharedPtr::new(ScriptEntry { name: "x".to_owned() })));
    }

    #[test]
    fn profiler_ordering_and_tree_ops() {
        let f = ProfilerFunction::new("b");
        let g = ProfilerFunction::new("a");
        assert!(stub_0xf2c2e4(&g, &f));
        assert!(!stub_0xf2c2e4(&f, &g));
        let mut tree = BTreeMap::new();
        assert!(stub_0xf2c8f4(&mut tree, f.clone(), "body".to_owned()));
        assert!(!stub_0xf2c8f4(&mut tree, f.clone(), "again".to_owned()));
        assert_eq!(tree[&f], "body");
        let (k, v) = stub_0xf2c8e4(g.clone(), "s".to_owned());
        assert_eq!((k, v.as_str()), (g.clone(), "s"));
        assert_eq!(stub_0xf2c924(&mut tree, g.clone(), "s".to_owned()), None);
        assert!(stub_0xf2c914(&mut tree, &f));
        assert!(!stub_0xf2c914(&mut tree, &f));
        let mut profane = BTreeMap::new();
        stub_0xf2c724(&mut profane, &g).push_str("src");
        assert_eq!(profane[&g], "src");
    }

    #[test]
    fn stat_map_and_pair_ctor() {
        let mut stats = HashMap::new();
        stub_0xf2c734(&mut stats, "render").calls += 1;
        stub_0xf2c734(&mut stats, "render").total_time += 0.5;
        assert_eq!(stats["render"].calls, 1);
        let (name, info) = stub_0xf2c754("physics", ScriptStatInfo { calls: 2, total_time: 1.0 });
        assert_eq!((name.as_str(), info.calls), ("physics", 2));
    }

    #[test]
    fn waiting_queue_push_pop_copy_clear() {
        let mut queue = stub_0xf2c6b4(4);
        stub_0xf2c6a4(&mut queue, 4);
        stub_0xf2c7d4(&mut queue, WaitingThread::new(1, 10.0));
        stub_0xf2c784(&mut queue, WaitingThread::new(2, 20.0));
        let copy = stub_0xf2c7e4(&queue);
        assert_eq!(copy.threads.len(), 2);
        assert_eq!(stub_0xf2c774(&mut queue).unwrap().thread_id, 1);
        assert_eq!(stub_0xf2c7c4(&mut queue).unwrap().thread_id, 2);
        assert!(stub_0xf2c7c4(&mut queue).is_none());
        stub_0xf2c7b4(&mut queue, 8);
        stub_0xf2c794(&mut queue, 8, false);
        stub_0xf2c7a4(&mut queue);
        stub_0xf2c694(2);
        stub_0xf2c6c4(queue);
        stub_0xf2c7f4(copy);
    }

    #[test]
    fn script_start_vec_insert_erase_copy() {
        let mut items = stub_0xf2c6f4(4);
        stub_0xf2c864(&mut items, ScriptStart::new("a", "src-a"));
        stub_0xf2c844(&mut items, 0, ScriptStart::new("b", "src-b"));
        assert_eq!(items[0].name, "b");
        stub_0xf2c714(&mut items, 0..1, 1);
        assert_eq!(items[1].name, "b");
        stub_0xf2c814(&mut items, 0..1, 1);
        let erased = stub_0xf2c854(&mut items, 0);
        assert_eq!(erased.name, "b");
        stub_0xf2c874(items);
    }

    #[test]
    fn member_dispatch_forwards_bound_args() {
        let ctx = ScriptContextHandle::new("ctx");
        let start = ScriptStart::new("job", "print(1)");
        let mut seen = String::new();
        stub_0xf2c444(&ctx, &start, &mut |c, s| seen = format!("{c}/{s}"));
        assert_eq!(seen, "ctx/job");
        stub_0xf2c454(&ctx, WeakThreadRef { id: 9 }, &LuaStateRef { stack_top: 3 }, &mut |c, t, s| seen = format!("{c}/{t}/{s}"));
        assert_eq!(seen, "ctx/9/3");
        stub_0xf2c464(&ctx, "base", ScriptStartOptions { timeout_secs: 1.5 }, &mut |c, s, t| seen = format!("{c}/{s}/{t}"));
        assert_eq!(seen, "ctx/base/1.5");
        stub_0xf2c474(&ctx, "i", "txt", "t", &mut |c, i, x, t| seen = format!("{c}/{i}/{x}/{t}"));
        assert_eq!(seen, "ctx/i/txt/t");
        // IDA 0xf2c654 is `const`: the stored target cannot mutate captures,
        // so the probe writes through a shared cell like a captured pointer.
        let probe = std::cell::RefCell::new(String::new());
        stub_0xf2c654(&|a, b, c, d| *probe.borrow_mut() = format!("{a}/{b}/{c}/{d}"), "a", "b", "c", 4);
        assert_eq!(*probe.borrow(), "a/b/c/4");
}
}
