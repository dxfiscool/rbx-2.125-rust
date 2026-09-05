// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace complete — fallback EA-sorted asc not yet in datamodel
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0xf1fa68..0xf1ff6c | next 100 uncovered after 0xf1fa5c (shard_283)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias
// Shard: datamodel_shard_284 EA-sorted ascending

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// Rust model of `G3D::Vector3::Axis` (IDA `0xf1fbd0`): the axis tag converted
/// out of a `Variant` by `genericConvert<Axis>()`. No G3D Axis model exists in
/// `crates/core` or `crates/datamodel` (grep `enum Axis` is clean), so the
/// standard G3D discriminants (`X_AXIS = 0, Y_AXIS = 1, Z_AXIS = 2`) land here.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Vector3Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Vector3Axis {
    pub fn from_tag(tag: i32) -> Self {
        match tag {
            1 => Vector3Axis::Y,
            2 => Vector3Axis::Z,
            _ => Vector3Axis::X,
        }
    }
}

/// Rust model of `RBX::Name::doDeclare<sAdvLuaDragger>` storage (IDA `0xf1fa8c`):
/// the once-declared interned name; the `__cxa_guard` init becomes `LazyLock`,
/// mirroring `generated_datamodel_shard_283::stub_f1f984`.
static ADV_LUA_DRAGGER_NAME_DECL: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| "AdvLuaDragger".to_owned());
/// Rust model of `RBX::Name::doDeclare<sAdvLuaDragTool>` storage (IDA `0xf1fa98`).
static ADV_LUA_DRAG_TOOL_NAME_DECL: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| "AdvLuaDragTool".to_owned());
/// Rust model of `RBX::Name::doDeclare<sLuaDragger>` storage (IDA `0xf1fb34`).
static LUA_DRAGGER_NAME_DECL: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| "LuaDragger".to_owned());
/// Rust model of `RBX::Name::doDeclare<sLuaDragTool>` storage (IDA `0xf1fb40`).
static LUA_DRAG_TOOL_NAME_DECL: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| "LuaDragTool".to_owned());
/// Rust model of `RBX::Name::doDeclare<sLuaWebService>` storage (IDA `0xf1fce4`).
static LUA_WEB_SERVICE_NAME_DECL: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| "LuaWebService".to_owned());

/// Success continuation behind `boost::function<void(string)>` when the raw
/// string body (not the parsed `vector<Variant>`) crosses (IDA `0xf1fcf0`).
/// Same shape as the 277 `LuaWebErrorFn`; a distinct alias marks the success
/// role in the raw-string bind.
pub type LuaWebStringFn = std::sync::Arc<dyn Fn(&str) + Send + Sync>;
/// Success continuation behind `boost::function<void(int)>` (IDA `0xf1fd08`).
pub type LuaWebIntFn = std::sync::Arc<dyn Fn(i32) + Send + Sync>;
/// Success continuation behind `boost::function<void(bool)>` (IDA `0xf1fd14`).
pub type LuaWebBoolFn = std::sync::Arc<dyn Fn(bool) + Send + Sync>;
/// Success continuation behind
/// `boost::function<void(shared_ptr<map<string, Variant>> const)>`
/// (IDA `0xf1fd20`).
pub type LuaWebMapFn =
    std::sync::Arc<dyn Fn(&SharedPtr<std::collections::HashMap<String, crate::generated_05::Variant>>) + Send + Sync>;

/// Free-function word of the raw-string bind (IDA `0xf1fcf0`): completion
/// target invoked with the retained weak service, the late-bound result, the
/// retained url, and the two raw-string continuations.
pub type RawStringDispatchFn = fn(
    service: &rbx_core::WeakPtr<crate::instance::LuaWebService>,
    result: crate::generated_13::HttpRequestResult,
    url: &str,
    on_success: &LuaWebStringFn,
    on_error: &LuaWebStringFn,
);

/// Rust model of the full raw-string `bind_t` (IDA `0xf1fcf0`): the function
/// word plus the five list values. Twin of the 277 `LuaWebCallbackBind` with
/// both continuations in the raw-string flavor.
#[derive(Clone)]
pub struct RawStringWebBind {
    pub func: RawStringDispatchFn,
    pub service: rbx_core::WeakPtr<crate::instance::LuaWebService>,
    pub url: String,
    pub on_success: LuaWebStringFn,
    pub on_error: LuaWebStringFn,
}

/// Nullable holder for the raw-string bind (IDA `0xf1fcf0`).
#[derive(Default)]
pub struct RawStringWebCallback {
    pub inner: Option<RawStringWebBind>,
}

/// Mangled type name `strcmp`ed by the check-type path (same role as the 277
/// `LUA_WEB_CALLBACK_BIND_TYPE_NAME`).
pub const RAW_STRING_WEB_BIND_TYPE_NAME: &str =
    "N5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESD_ENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSH_ISsEENSH_ISD_EESM_EEEEEE";

/// Free-function word of the int-success bind (IDA `0xf1fd08`).
pub type IntWebDispatchFn = fn(
    service: &rbx_core::WeakPtr<crate::instance::LuaWebService>,
    result: crate::generated_13::HttpRequestResult,
    url: &str,
    on_success: &LuaWebIntFn,
    on_error: &LuaWebStringFn,
);

/// Rust model of the full int-success `bind_t` (IDA `0xf1fd08`). Twin of
/// `RawStringWebBind` with the `void(int)` success flavor.
#[derive(Clone)]
pub struct IntWebBind {
    pub func: IntWebDispatchFn,
    pub service: rbx_core::WeakPtr<crate::instance::LuaWebService>,
    pub url: String,
    pub on_success: LuaWebIntFn,
    pub on_error: LuaWebStringFn,
}

/// Nullable holder for the int-success bind (IDA `0xf1fd08`).
#[derive(Default)]
pub struct IntWebCallback {
    pub inner: Option<IntWebBind>,
}

pub const INT_WEB_BIND_TYPE_NAME: &str =
    "N5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENSB_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEE";

/// Free-function word of the bool-success bind (IDA `0xf1fd14`).
pub type BoolWebDispatchFn = fn(
    service: &rbx_core::WeakPtr<crate::instance::LuaWebService>,
    result: crate::generated_13::HttpRequestResult,
    url: &str,
    on_success: &LuaWebBoolFn,
    on_error: &LuaWebStringFn,
);

/// Rust model of the full bool-success `bind_t` (IDA `0xf1fd14`). Twin of
/// `RawStringWebBind` with the `void(bool)` success flavor.
#[derive(Clone)]
pub struct BoolWebBind {
    pub func: BoolWebDispatchFn,
    pub service: rbx_core::WeakPtr<crate::instance::LuaWebService>,
    pub url: String,
    pub on_success: LuaWebBoolFn,
    pub on_error: LuaWebStringFn,
}

/// Nullable holder for the bool-success bind (IDA `0xf1fd14`).
#[derive(Default)]
pub struct BoolWebCallback {
    pub inner: Option<BoolWebBind>,
}

pub const BOOL_WEB_BIND_TYPE_NAME: &str =
    "N5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENSB_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEE";

/// Free-function word of the map-success bind (IDA `0xf1fd20`).
pub type MapWebDispatchFn = fn(
    service: &rbx_core::WeakPtr<crate::instance::LuaWebService>,
    result: crate::generated_13::HttpRequestResult,
    url: &str,
    on_success: &LuaWebMapFn,
    on_error: &LuaWebStringFn,
);

/// Rust model of the full map-success `bind_t` (IDA `0xf1fd20`). Twin of
/// `RawStringWebBind` with the `void(shared_ptr<map<string, Variant>>)`
/// success flavor.
#[derive(Clone)]
pub struct MapWebBind {
    pub func: MapWebDispatchFn,
    pub service: rbx_core::WeakPtr<crate::instance::LuaWebService>,
    pub url: String,
    pub on_success: LuaWebMapFn,
    pub on_error: LuaWebStringFn,
}

/// Nullable holder for the map-success bind (IDA `0xf1fd20`).
#[derive(Default)]
pub struct MapWebCallback {
    pub inner: Option<MapWebBind>,
}

pub const MAP_WEB_BIND_TYPE_NAME: &str =
    "N5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEENSB_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEE";

/// Rust model of `bind_t<unspecified, function<void(shared_ptr<map<string,
/// Variant>> const)>, list1<value<shared_ptr<map<string, Variant>>>>>`
/// (IDA `0xf1fd2c`): the retained variant map; the single list value is the
/// whole payload (`arg<1>` never stored). The map crosses as `SharedPtr`, so
/// the bind stays `Clone` even though `Variant` itself is not.
#[derive(Clone)]
pub struct VariantMapBind {
    pub map: SharedPtr<std::collections::HashMap<String, crate::generated_05::Variant>>,
}

/// Nullable holder for the variant-map bind (IDA `0xf1fd2c`).
#[derive(Clone, Default)]
pub struct VariantMapCallback {
    pub inner: Option<VariantMapBind>,
}

pub const VARIANT_MAP_BIND_TYPE_NAME: &str =
    "N5boost3_bi6bind_tINS_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS0_5list1INS0_5valueISK_EEEEEE";

/// Rust model of `bind_t<void, void(*)(ThreadRef, weak_ptr<ScriptContext>,
/// IAsyncResult*), list3<value<ThreadRef>, value<weak<ScriptContext>>, arg<1>>>`
/// (IDA `0xf1fa68`): the retained thread plus the weak script context; the
/// `IAsyncResult*` arrives as `arg<1>` at call time, never stored. Mirrors the
/// 283 `ScriptContextResumeBind` (raw context word, retained thread).
#[derive(Clone, Default)]
pub struct ThreadRefBind {
    pub thread: Option<SharedPtr<crate::data_model::LuaWeakThreadRef>>,
    /// Raw context word; the pointee is owned by the script context, never by
    /// the bind.
    /// SAFETY: must be null or point at a live `ScriptContext` for the whole
    /// time the bind is reachable.
    pub context: *const (),
}

/// Nullable holder for the thread-ref bind (IDA `0xf1fa68`).
#[derive(Clone, Default)]
pub struct ThreadRefCallback {
    pub inner: Option<ThreadRefBind>,
}

pub const THREAD_REF_BIND_TYPE_NAME: &str =
    "N5boost3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS0_5list3INS0_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEE";

/// Rust model of `bind_t<void, mf1<void, GenericSlotWrapper, double const&>,
/// list2<value<shared_ptr<GenericSlotWrapper>>, arg<1>>>` (IDA `0xf1feac`):
/// the retained wrapper plus the native member stand-in; the `double` arrives
/// as `arg<1>` at call time. Mirrors the 283 `YieldVariantBind`.
#[derive(Clone)]
pub struct SlotDoubleBind {
    pub target: SharedPtr<crate::generated_05::GenericSlotWrapper>,
    pub on_value: Option<fn(&crate::generated_05::GenericSlotWrapper, f64)>,
}

/// Nullable holder for the double-slot bind (IDA `0xf1feac`).
#[derive(Clone, Default)]
pub struct SlotDoubleCallback {
    pub inner: Option<SlotDoubleBind>,
}

pub const SLOT_DOUBLE_BIND_TYPE_NAME: &str =
    "N5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEE";

/// Rust model of `bind_t<void, mf2<void, GenericSlotWrapper, double const&,
/// double const&>, list3<value<shared_ptr<GenericSlotWrapper>>, arg<1>,
/// arg<2>>>` (IDA `0xf1fedc`): same shape as `SlotDoubleBind` with the two-arg
/// member flavor; both doubles arrive at call time.
#[derive(Clone)]
pub struct SlotDouble2Bind {
    pub target: SharedPtr<crate::generated_05::GenericSlotWrapper>,
    pub on_values: Option<fn(&crate::generated_05::GenericSlotWrapper, f64, f64)>,
}

/// Nullable holder for the two-double slot bind (IDA `0xf1fedc`).
#[derive(Clone, Default)]
pub struct SlotDouble2Callback {
    pub inner: Option<SlotDouble2Bind>,
}

pub const SLOT_DOUBLE2_BIND_TYPE_NAME: &str =
    "N5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKdSB_EENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSI_ILi2EEEEEE";

// 0xf1fa68 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1fa68(src: &ThreadRefBind, dst: &mut ThreadRefCallback, op: crate::generated_05::FunctorOp) -> bool {
    // IDA 0xf1fa68 (decompile: tail-calls `functor_manager<bind_t<void, void(*)(ThreadRef, weak_ptr<ScriptContext>, IAsyncResult*), list3<...>>>::manager`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Clone/move retain the thread and copy the context word; destroy releases. Mirrors 277 `stub_0x351c10`.
    use crate::generated_05::FunctorOp;
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

// 0xf1fa8c — __ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v$shim")]
pub fn stub_f1fa8c() -> &'static str {
    // IDA 0xf1fa8c (decompile: tail-calls `Name::doDeclare<sAdvLuaDragger>()`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // `__cxa_guard_acquire` once-check + `Name::declare` + guard release collapse into `LazyLock`, mirroring 283 `stub_f1f984`.
    std::sync::LazyLock::force(&ADV_LUA_DRAGGER_NAME_DECL).as_str()
}

// 0xf1fa98 — __ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v$shim")]
pub fn stub_f1fa98() -> &'static str {
    // IDA 0xf1fa98 (decompile: tail-calls `Name::doDeclare<sAdvLuaDragTool>()`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Same once-init collapse as `stub_f1fa8c`; mirrors 283 `stub_f1f984`.
    std::sync::LazyLock::force(&ADV_LUA_DRAG_TOOL_NAME_DECL).as_str()
}

// 0xf1faa4 — __ZNSs6assignERKSs$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNSs6assignERKSs$shim")]
#[doc(alias = "__ZNSs6assignERKSs$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1faa4 as stub_f1faa4;

// 0xf1fab0 — __ZN3RBX15AdvMoveToolBaseD2Ev$shim
// type: void __fastcall(RBX::AdvMoveToolBase *__hidden this)
#[doc(alias = "__ZN3RBX15AdvMoveToolBaseD2Ev$shim")]
#[doc(alias = "__ZN3RBX15AdvMoveToolBaseD2Ev$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1fab0 as stub_f1fab0;

// 0xf1fabc — __ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1fabc as stub_f1fabc;

// 0xf1fac8 — __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
#[doc(alias = "__ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1fac8 as stub_f1fac8;

// 0xf1fad4 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE6resizeEib$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE6resizeEib$shim")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE6resizeEib$shim")]
pub use rbx_core::generated_watchdog_core_w7::stub_0xf1fad4 as stub_f1fad4;

// 0xf1fae0 — __ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1fae0 as stub_f1fae0;

// 0xf1faec — __ZNSs6assignEPKcm$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZNSs6assignEPKcm$shim")]
#[doc(alias = "__ZNSs6assignEPKcm$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1faec as stub_f1faec;

// 0xf1faf8 — __ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1faf8 as stub_f1faf8;

// 0xf1fb04 — __ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1fb04 as stub_f1fb04;

// 0xf1fb34 — __ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v$shim")]
pub fn stub_f1fb34() -> &'static str {
    // IDA 0xf1fb34 (decompile: tail-calls `Name::doDeclare<sLuaDragger>()`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Same once-init collapse as `stub_f1fa8c`; mirrors 283 `stub_f1f984`.
    std::sync::LazyLock::force(&LUA_DRAGGER_NAME_DECL).as_str()
}

// 0xf1fb40 — __ZN3RBX4Name9doDeclareILZNS_12sLuaDragToolEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sLuaDragToolEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sLuaDragToolEEEERKS0_v$shim")]
pub fn stub_f1fb40() -> &'static str {
    // IDA 0xf1fb40 (decompile: tail-calls `Name::doDeclare<sLuaDragTool>()`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Same once-init collapse as `stub_f1fa8c`; mirrors 283 `stub_f1f984`.
    std::sync::LazyLock::force(&LUA_DRAG_TOOL_NAME_DECL).as_str()
}

// 0xf1fb4c — __ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1fb4c as stub_f1fb4c;

// 0xf1fb58 — __ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1fb58 as stub_f1fb58;

// 0xf1fb64 — __ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mo::stub_0xf1fb64 as stub_f1fb64;

// 0xf1fb70 — __ZN3G3D5ArrayImLi10ELm32EE7reallocEi$shim
#[doc(alias = "__ZN3G3D5ArrayImLi10ELm32EE7reallocEi$shim")]
#[doc(alias = "__ZN3G3D5ArrayImLi10ELm32EE7reallocEi$shim")]
pub use rbx_core::generated_watchdog_core_w12b::stub_0xf1fb70 as stub_f1fb70;

// 0xf1fb88 — __ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v$shim
// type: int(void)
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fb88 as stub_f1fb88;

// 0xf1fba0 — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
#[doc(alias = "__ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
#[doc(alias = "__ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fba0 as stub_f1fba0;

// 0xf1fbac — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
#[doc(alias = "__ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
#[doc(alias = "__ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fbac as stub_f1fbac;

// 0xf1fbb8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fbb8 as stub_f1fbb8;

// 0xf1fbc4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fbc4 as stub_f1fbc4;

// 0xf1fbd0 — __ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector34AxisEEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector34AxisEEERT_v$shim")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector34AxisEEERT_v$shim")]
pub fn stub_f1fbd0(variant: &crate::generated_05::Variant) -> Vector3Axis {
    // IDA 0xf1fbd0 (decompile: tail-calls `Variant::genericConvert<Vector3::Axis>(void)`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // The shim body carries no discriminant table, so the G3D `X_AXIS = 0, Y_AXIS = 1, Z_AXIS = 2`
    // mapping is [INFERENCE] from the G3D convention; an `Int` tag converts, any other arm throws
    // (`bad_any_cast`), which is the panic here.
    match variant {
        crate::generated_05::Variant::Int(tag) => Vector3Axis::from_tag(*tag),
        _ => panic!("0xf1fbd0: bad_any_cast converting Variant to Vector3::Axis"),
    }
}

// 0xf1fbdc — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
pub use rbx_core::generated_watchdog_core_w12b::stub_0xf1fbdc as stub_f1fbdc;

// 0xf1fbe8 — __ZN3RBX10BrickColor8BrickMapD2Ev$shim
#[doc(alias = "__ZN3RBX10BrickColor8BrickMapD2Ev$shim")]
#[doc(alias = "__ZN3RBX10BrickColor8BrickMapD2Ev$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fbe8 as stub_f1fbe8;

// 0xf1fbf4 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fbf4 as stub_f1fbf4;

// 0xf1fc00 — __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_$shim
#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_$shim")]
#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fc00 as stub_f1fc00;

// 0xf1fc0c — __ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v$shim
// type: int(void)
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fc0c as stub_f1fc0c;

// 0xf1fc18 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fc18 as stub_f1fc18;

// 0xf1fc24 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fc24 as stub_f1fc24;

// 0xf1fc30 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsbENS3_5list3INS3_5valueIS8_EENSC_ISsEENSC_IbEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsbENS3_5list3INS3_5valueIS8_EENSC_ISsEENSC_IbEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsbENS3_5list3INS3_5valueIS8_EENSC_ISsEENSC_IbEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fc30 as stub_f1fc30;

// 0xf1fc3c — __ZNSs6appendERKSs$shim
#[doc(alias = "__ZNSs6appendERKSs$shim")]
#[doc(alias = "__ZNSs6appendERKSs$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fc3c as stub_f1fc3c;

// 0xf1fc48 — __ZN3RBX4Http10MutexGuardD2Ev$shim
#[doc(alias = "__ZN3RBX4Http10MutexGuardD2Ev$shim")]
#[doc(alias = "__ZN3RBX4Http10MutexGuardD2Ev$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fc48 as stub_f1fc48;

// 0xf1fc54 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fc54 as stub_f1fc54;

// 0xf1fc60 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fc60 as stub_f1fc60;

// 0xf1fc6c — __ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsSsbbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i$shim
// type: int __fastcall(std::string *)
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsSsbbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i$shim")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsSsbbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fc6c as stub_f1fc6c;

// 0xf1fc78 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fc78 as stub_f1fc78;

// 0xf1fc84 — __ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i$shim
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i$shim")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fc84 as stub_f1fc84;

// 0xf1fc90 — __ZN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEaSERKS4_$shim
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEaSERKS4_$shim")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEaSERKS4_$shim")]
pub fn stub_f1fc90(
    dst: &mut SharedPtr<crate::instance::DescribedBase>,
    src: &SharedPtr<crate::instance::DescribedBase>,
) -> SharedPtr<crate::instance::DescribedBase> {
    // IDA 0xf1fc90 (decompile: tail-calls `shared_ptr<DescribedBase>::operator=(a1, a2)`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // `shared_ptr<DescribedBase>` copy-assign: addref source, store, release old, return `*this`.
    // Mirrors `crate::instance::stub_0x26c350`.
    *dst = SharedPtr::clone(src);
    SharedPtr::clone(dst)
}

// 0xf1fca8 — __ZN3G3D5ArrayIPN3RBX11IndexedTreeELi10ELm32EE7reallocEi$shim
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX11IndexedTreeELi10ELm32EE7reallocEi$shim")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX11IndexedTreeELi10ELm32EE7reallocEi$shim")]
pub use rbx_core::generated_watchdog_core_w7::stub_0xf1fca8 as stub_f1fca8;

// 0xf1fcb4 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm$shim
#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm$shim")]
#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fcb4 as stub_f1fcb4;

// 0xf1fcc0 — __ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
#[doc(alias = "__ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
#[doc(alias = "__ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fcc0 as stub_f1fcc0;

// 0xf1fccc — __ZNSs6resizeEmc$shim
#[doc(alias = "__ZNSs6resizeEmc$shim")]
#[doc(alias = "__ZNSs6resizeEmc$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fccc as stub_f1fccc;

// 0xf1fcd8 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_$shim
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_$shim")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fcd8 as stub_f1fcd8;

// 0xf1fce4 — __ZN3RBX4Name9doDeclareILZNS_14sLuaWebServiceEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sLuaWebServiceEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sLuaWebServiceEEEERKS0_v$shim")]
pub fn stub_f1fce4() -> &'static str {
    // IDA 0xf1fce4 (decompile: tail-calls `Name::doDeclare<sLuaWebService>()`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Same once-init collapse as `stub_f1fa8c`; mirrors 283 `stub_f1f984`.
    std::sync::LazyLock::force(&LUA_WEB_SERVICE_NAME_DECL).as_str()
}

// 0xf1fcf0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESD_ENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSH_ISsEENSH_ISD_EESM_EEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESD_ENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSH_ISsEENSH_ISD_EESM_EEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESD_ENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSH_ISsEENSH_ISD_EESM_EEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1fcf0(src: &RawStringWebBind, dst: &mut RawStringWebCallback, op: crate::generated_05::FunctorOp) -> bool {
    // IDA 0xf1fcf0 (decompile: tail-calls `functor_manager<bind_t<void, void(*)(weak<LuaWebService>, RequestResult, string, fn(string), fn(string)), list5<...>>>::manager`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Raw-string flavor of 277 `stub_0x351c10`: both continuations are `void(string)` here, not the
    // `void(shared_ptr<vector<Variant>>)` success there. Move through a shared borrow clones like
    // clone; destroy clears the slot.
    use crate::generated_05::FunctorOp;
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

// 0xf1fcfc — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_$shim
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_$shim")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_$shim")]
pub fn stub_f1fcfc<'a>(
    cache: &'a crate::generated_datamodel_shard_277::WebCache,
    url: &str,
) -> Option<&'a (u64, crate::generated_datamodel_shard_277::CachedLuaWebServiceInfo)> {
    // IDA 0xf1fcfc (decompile: tail-calls `table_impl<...CachedLuaWebServiceInfo...>::find_node_impl(hash, key)`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Non-raw flavor of the 277 `stub_0x352b14` raw twin: hit returns the node payload, miss returns null.
    cache.entries.get(url)
}

// 0xf1fd08 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1fd08(src: &IntWebBind, dst: &mut IntWebCallback, op: crate::generated_05::FunctorOp) -> bool {
    // IDA 0xf1fd08 (decompile: tail-calls `functor_manager<bind_t<void, void(*)(weak<LuaWebService>, RequestResult, string, fn(int), fn(string)), list5<...>>>::manager`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // `void(int)`-success flavor of `stub_f1fcf0`; same clone/move/destroy/check/get dispatch as 277 `stub_0x351c10`.
    use crate::generated_05::FunctorOp;
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

// 0xf1fd14 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1fd14(src: &BoolWebBind, dst: &mut BoolWebCallback, op: crate::generated_05::FunctorOp) -> bool {
    // IDA 0xf1fd14 (decompile: tail-calls `functor_manager<bind_t<void, void(*)(weak<LuaWebService>, RequestResult, string, fn(bool), fn(string)), list5<...>>>::manager`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // `void(bool)`-success flavor of `stub_f1fcf0`; same dispatch as 277 `stub_0x351c10`.
    use crate::generated_05::FunctorOp;
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

// 0xf1fd20 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEE7managerERKNS1_15function_bufferERS16_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEE7managerERKNS1_15function_bufferERS16_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEE7managerERKNS1_15function_bufferERS16_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1fd20(src: &MapWebBind, dst: &mut MapWebCallback, op: crate::generated_05::FunctorOp) -> bool {
    // IDA 0xf1fd20 (decompile: tail-calls `functor_manager<bind_t<void, void(*)(weak<LuaWebService>, RequestResult, string, fn(shared_ptr<map<string, Variant>>), fn(string)), list5<...>>>::manager`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Map-success flavor of `stub_f1fcf0`; same dispatch as 277 `stub_0x351c10`.
    use crate::generated_05::FunctorOp;
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

// 0xf1fd2c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1fd2c(src: &VariantMapBind, dst: &mut VariantMapCallback, op: crate::generated_05::FunctorOp) -> bool {
    // IDA 0xf1fd2c (decompile: tail-calls `functor_manager<bind_t<unspecified, fn(shared_ptr<map<string, Variant>>), list1<value<...>>>>::manager` with 10 forwarded words; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Single-value bind: no free-function word, only the retained map payload. Same clone/move/destroy
    // dispatch as 277 `stub_0x351c10`; the retained `SharedPtr` clone is the `shared_count` addref.
    use crate::generated_05::FunctorOp;
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

// 0xf1fd38 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEE7managerERKNS1_15function_bufferERS11_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEE7managerERKNS1_15function_bufferERS11_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEE7managerERKNS1_15function_bufferERS11_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1fd38(
    src: &crate::generated_datamodel_shard_277::LuaWebCallbackBind,
    dst: &mut crate::generated_datamodel_shard_277::LuaWebCallbackFunction,
    op: crate::generated_05::FunctorOp,
) -> bool {
    // IDA 0xf1fd38 (decompile: tail-calls `functor_manager<bind_t<void, void(*)(weak<LuaWebService>, RequestResult, string, fn(shared_ptr<vector<Variant>>), fn(string)), list5<...>>>::manager`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Vector-success flavor identical to the 277 `stub_0x351c10` instantiation, so the 277
    // `LuaWebCallbackBind` / `LuaWebCallbackFunction` types are reused directly.
    use crate::generated_05::FunctorOp;
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

// 0xf1fd44 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_$shim
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_$shim")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_$shim")]
pub fn stub_f1fd44<'a>(
    cache: &'a crate::generated_datamodel_shard_277::RawWebCache,
    url: &str,
) -> Option<&'a (u64, crate::generated_datamodel_shard_277::CachedRawLuaWebServiceInfo)> {
    // IDA 0xf1fd44 (decompile: tail-calls `table_impl<...CachedRawLuaWebServiceInfo...>::find_node_impl(hash, key)`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Raw flavor of `stub_f1fcfc`; mirrors 277 `stub_0x352b14`: hit returns the node payload, miss null.
    cache.entries.get(url)
}

// 0xf1fd50 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE23removeLeastRecentlyUsedEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE23removeLeastRecentlyUsedEv$shim")]
#[doc(alias = "__ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE23removeLeastRecentlyUsedEv$shim")]
pub fn stub_f1fd50(cache: &mut crate::generated_datamodel_shard_277::RawWebCache) {
    // IDA 0xf1fd50 (decompile: tail-calls `LRUCache<string, CachedRawLuaWebServiceInfo>::removeLeastRecentlyUsed()`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Evicts one entry; recency order is folded (`HashMap` keeps no LRU list), so the first entry goes —
    // mirrors the 277 `stub_0x3532a8` `registerContent` evict loop.
    if let Some(victim) = cache.entries.keys().next().cloned() {
        cache.entries.remove(&victim);
    }
}

// 0xf1fd5c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_$shim
// type: int()
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_$shim")]
pub fn stub_f1fd5c(cache: &mut crate::generated_datamodel_shard_277::RawWebCache, url: &str) -> bool {
    // IDA 0xf1fd5c (decompile: tail-calls `table_impl<...CachedRaw...>::erase_nodes()`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Erases the node range for the key — a single-key range here; mirrors 277 `stub_0x353bbc`.
    cache.entries.remove(url).is_some()
}

// 0xf1fd68 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
pub fn stub_f1fd68(cache: &mut crate::generated_datamodel_shard_277::RawWebCache, buckets: usize) {
    // IDA 0xf1fd68 (decompile: tail-calls `table_impl<...CachedRaw...>::rehash_impl(buckets)`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Rebuilds the bucket array and re-links every node; reserve keeps contents; mirrors 277 `stub_0x354138`.
    cache.entries.reserve(buckets.saturating_sub(cache.entries.len()));
}

// 0xf1fd74 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
pub fn stub_f1fd74(cache: &mut crate::generated_datamodel_shard_277::RawWebCache, buckets: usize) {
    // IDA 0xf1fd74 (decompile: tail-calls `table<...CachedRaw...>::create_buckets(buckets)`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Lays out the bucket array; over a live table reserve keeps contents; mirrors 277 `stub_0x353f80`.
    cache.entries.reserve(buckets.saturating_sub(cache.entries.len()));
}

// 0xf1fd80 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_$shim
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_$shim")]
pub fn stub_f1fd80(cache: &mut crate::generated_datamodel_shard_277::WebCache, url: &str) -> bool {
    // IDA 0xf1fd80 (decompile: tail-calls `table_impl<...CachedLuaWebServiceInfo...>::erase_nodes(a1, a2, a3)`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Non-raw twin of `stub_f1fd5c`; mirrors 277 `stub_0x35561c`.
    cache.entries.remove(url).is_some()
}

// 0xf1fd8c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
pub fn stub_f1fd8c(cache: &mut crate::generated_datamodel_shard_277::WebCache, buckets: usize) {
    // IDA 0xf1fd8c (decompile: tail-calls `table_impl<...CachedLuaWebServiceInfo...>::rehash_impl(buckets)`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Non-raw twin of `stub_f1fd68`; mirrors 277 `stub_0x355bc0`.
    cache.entries.reserve(buckets.saturating_sub(cache.entries.len()));
}

// 0xf1fd98 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
pub fn stub_f1fd98(cache: &mut crate::generated_datamodel_shard_277::WebCache, buckets: usize) {
    // IDA 0xf1fd98 (decompile: tail-calls `table<...CachedLuaWebServiceInfo...>::create_buckets(buckets)`; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Non-raw twin of `stub_f1fd74`; mirrors 277 `stub_0x355a08`.
    cache.entries.reserve(buckets.saturating_sub(cache.entries.len()));
}

// 0xf1fda4 — _acosf$shim
// type: float __cdecl(float)
#[doc(alias = "_acosf$shim")]
#[doc(alias = "_acosf$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fda4 as stub_f1fda4;

// 0xf1fdb0 — __ZNSt6vectorIN3G3D7Vector2ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector2ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector2ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
pub use rbx_core::generated_watchdog_core_w12b::stub_0xf1fdb0 as stub_f1fdb0;

// 0xf1fdbc — __ZNSt6vectorIN3G3D7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
pub use rbx_core::generated_watchdog_core_w12b::stub_0xf1fdbc as stub_f1fdbc;

// 0xf1fdc8 — _CC_MD5_Update$shim
// type: int __cdecl(CC_MD5_CTX *c, const void *data, CC_LONG len)
#[doc(alias = "_CC_MD5_Update$shim")]
#[doc(alias = "_CC_MD5_Update$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fdc8 as stub_f1fdc8;

// 0xf1fdd4 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fdd4 as stub_f1fdd4;

// 0xf1fde0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fde0 as stub_f1fde0;

// 0xf1fdec — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_$shim
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_$shim")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fdec as stub_f1fdec;

// 0xf1fdf8 — _pthread_getspecific$shim
// type: void *__cdecl(pthread_key_t)
#[doc(alias = "_pthread_getspecific$shim")]
#[doc(alias = "_pthread_getspecific$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fdf8 as stub_f1fdf8;

// 0xf1fe04 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_$shim")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fe04 as stub_f1fe04;

// 0xf1fe10 — __ZN3RBX4Name7declareILZNS_7sCameraEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name7declareILZNS_7sCameraEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name7declareILZNS_7sCameraEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fe10 as stub_f1fe10;

// 0xf1fe1c — __ZN3RBX4Name9doDeclareILZNS_7sCameraEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sCameraEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sCameraEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fe1c as stub_f1fe1c;

// 0xf1fe28 — __ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED2Ev$shim
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED2Ev$shim")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_17bad_function_callEED2Ev$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fe28 as stub_f1fe28;

// 0xf1fe34 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE24safe_static_do_get_mutexEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fe34 as stub_f1fe34;

// 0xf1fe40 — __ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fe40 as stub_f1fe40;

// 0xf1fe4c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv$shim
// type: int()
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fe4c as stub_f1fe4c;

// 0xf1fe58 — __ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fe58 as stub_f1fe58;

// 0xf1fe64 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fe64 as stub_f1fe64;

// 0xf1fe70 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE5cloneEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE5cloneEv$shim")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE5cloneEv$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fe70 as stub_f1fe70;

// 0xf1fe7c — __ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd$shim")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fe7c as stub_f1fe7c;

// 0xf1fe88 — __ZN3rbx7signals6signalIFvdEE13disconnectAllEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3rbx7signals6signalIFvdEE13disconnectAllEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvdEE13disconnectAllEv$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fe88 as stub_f1fe88;

// 0xf1fe94 — __ZNSt8bad_castD2Ev$shim
// type: void __cdecl(std::bad_cast *__hidden this)
#[doc(alias = "__ZNSt8bad_castD2Ev$shim")]
#[doc(alias = "__ZNSt8bad_castD2Ev$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fe94 as stub_f1fe94;

// 0xf1fea0 — __ZN5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED2Ev$shim
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED2Ev$shim")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED2Ev$shim")]
pub use rbx_core::generated_watchdog_core_w5::stub_0xf1fea0 as stub_f1fea0;

// 0xf1feac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1feac(src: &SlotDoubleBind, dst: &mut SlotDoubleCallback, op: crate::generated_05::FunctorOp) -> bool {
    // IDA 0xf1feac (decompile: tail-calls `functor_manager<bind_t<void, mf1<void, GenericSlotWrapper, double const&>, list2<...>>>::manager` with 10 forwarded words; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Clone/move retain the wrapper and copy the member word; destroy releases. Mirrors the 283
    // `YieldVariantBind` family (`stub_f1f900`).
    use crate::generated_05::FunctorOp;
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

// 0xf1feb8 — __ZNK5boost9function1IvdEclEd$shim
#[doc(alias = "__ZNK5boost9function1IvdEclEd$shim")]
#[doc(alias = "__ZNK5boost9function1IvdEclEd$shim")]
pub use rbx_core::generated_watchdog_core_w5b::stub_0xf1feb8 as stub_f1feb8;

// 0xf1fec4 — __ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fec4 as stub_f1fec4;

// 0xf1fed0 — __ZN3rbx7signals6signalIFvddEE13disconnectAllEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvddEE13disconnectAllEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvddEE13disconnectAllEv$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fed0 as stub_f1fed0;

// 0xf1fedc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKdSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKdSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKdSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1fedc(src: &SlotDouble2Bind, dst: &mut SlotDouble2Callback, op: crate::generated_05::FunctorOp) -> bool {
    // IDA 0xf1fedc (decompile: tail-calls `functor_manager<bind_t<void, mf2<void, GenericSlotWrapper, double const&, double const&>, list3<...>>>::manager` with 10 forwarded words; disasm: LDR.W R12 / ADD R12,PC / BX R12 3-insn PLT shim).
    // Two-double member flavor of `stub_f1feac`; same dispatch.
    use crate::generated_05::FunctorOp;
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

// 0xf1fee8 — __ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv$shim")]
#[doc(alias = "__ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fee8 as stub_f1fee8;

// 0xf1fef4 — __ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv$shim")]
#[doc(alias = "__ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv$shim")]
pub use rbx_core::generated_core_shard_mp::stub_0xf1fef4 as stub_f1fef4;

// 0xf1ff00 — __ZN3RBX25ScriptInformationProviderD2Ev$shim
// type: void __fastcall(RBX::ScriptInformationProvider *__hidden this)
#[doc(alias = "__ZN3RBX25ScriptInformationProviderD2Ev$shim")]
#[doc(alias = "__ZN3RBX25ScriptInformationProviderD2Ev$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1ff00 as stub_f1ff00;

// 0xf1ff0c — __ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v$shim")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1ff0c as stub_f1ff0c;

// 0xf1ff18 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int(void)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1ff18 as stub_f1ff18;

// 0xf1ff24 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int(void)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1ff24 as stub_f1ff24;

// 0xf1ff30 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_$shim")]
pub use rbx_core::generated_watchdog_core_w5b::stub_0xf1ff30 as stub_f1ff30;

// 0xf1ff3c — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_$shim
// type: int()
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_$shim")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_$shim")]
pub use rbx_core::generated_watchdog_core_w5b::stub_0xf1ff3c as stub_f1ff3c;

// 0xf1ff48 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
pub use rbx_core::generated_watchdog_core_w5b::stub_0xf1ff48 as stub_f1ff48;

// 0xf1ff54 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
pub use rbx_core::generated_watchdog_core_w5b::stub_0xf1ff54 as stub_f1ff54;

// 0xf1ff60 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_$shim")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1ff60 as stub_f1ff60;

// 0xf1ff6c — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE23removeLeastRecentlyUsedEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE23removeLeastRecentlyUsedEv$shim")]
#[doc(alias = "__ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE23removeLeastRecentlyUsedEv$shim")]
pub use rbx_core::generated_core_shard_od::stub_0xf1ff6c as stub_f1ff6c;

#[cfg(test)]
mod shard_284_native_tests {
    use super::*;
    use crate::generated_05::{FunctorOp, GenericSlotWrapper, Variant};
    use crate::generated_datamodel_shard_277::{
        CachedLuaWebServiceInfo, CachedRawLuaWebServiceInfo, LuaWebCallbackBind,
        LuaWebCallbackFunction, RawWebCache, WebCache,
    };

    fn raw_info(body: &str) -> CachedRawLuaWebServiceInfo {
        CachedRawLuaWebServiceInfo::new(
            SharedPtr::new(body.to_owned()),
            SharedPtr::new("text/plain".to_owned()),
        )
    }

    fn cooked_info(body: &str) -> CachedLuaWebServiceInfo {
        CachedLuaWebServiceInfo::new(
            SharedPtr::new(body.to_owned()),
            SharedPtr::new("text/plain".to_owned()),
        )
    }

    fn empty_weak_service() -> rbx_core::WeakPtr<crate::instance::LuaWebService> {
        Default::default()
    }

    fn noop_str() -> LuaWebStringFn {
        std::sync::Arc::new(|_: &str| {})
    }

    fn empty_slot() -> SharedPtr<GenericSlotWrapper> {
        SharedPtr::new(GenericSlotWrapper {
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
        })
    }

    #[test]
    fn f1fa68_clone_move_destroy_cycle() {
        let src = ThreadRefBind { thread: None, context: std::ptr::null() };
        let mut dst = ThreadRefCallback::default();
        assert!(stub_f1fa68(&src, &mut dst, FunctorOp::Clone));
        assert!(dst.inner.is_some());
        assert!(stub_f1fa68(&src, &mut dst, FunctorOp::Move));
        assert!(dst.inner.is_some());
        assert!(stub_f1fa68(&src, &mut dst, FunctorOp::CheckType));
        assert!(stub_f1fa68(&src, &mut dst, FunctorOp::GetType));
        assert!(!stub_f1fa68(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.inner.is_none());
    }

    #[test]
    fn f1fa8c_declares_adv_lua_dragger_stably() {
        assert_eq!(stub_f1fa8c(), "AdvLuaDragger");
        assert!(std::ptr::eq(stub_f1fa8c(), stub_f1fa8c()));
    }

    #[test]
    fn f1fa98_declares_adv_lua_drag_tool() {
        assert_eq!(stub_f1fa98(), "AdvLuaDragTool");
    }

    #[test]
    fn f1fb34_declares_lua_dragger() {
        assert_eq!(stub_f1fb34(), "LuaDragger");
    }

    #[test]
    fn f1fb40_declares_lua_drag_tool() {
        assert_eq!(stub_f1fb40(), "LuaDragTool");
    }

    #[test]
    fn f1fbd0_converts_int_tags_to_axes() {
        assert_eq!(stub_f1fbd0(&Variant::Int(0)), Vector3Axis::X);
        assert_eq!(stub_f1fbd0(&Variant::Int(1)), Vector3Axis::Y);
        assert_eq!(stub_f1fbd0(&Variant::Int(2)), Vector3Axis::Z);
    }

    #[test]
    #[should_panic]
    fn f1fbd0_rejects_non_int_variant() {
        let _ = stub_f1fbd0(&Variant::Text("X".to_owned()));
    }

    #[test]
    fn f1fc90_assign_copies_source_link() {
        let src = SharedPtr::new(crate::instance::DescribedBase::default());
        let mut dst = SharedPtr::new(crate::instance::DescribedBase::default());
        let back = stub_f1fc90(&mut dst, &src);
        assert!(std::sync::Arc::ptr_eq(&dst, &src));
        assert!(std::sync::Arc::ptr_eq(&back, &src));
    }

    #[test]
    fn f1fce4_declares_lua_web_service() {
        assert_eq!(stub_f1fce4(), "LuaWebService");
    }

    #[test]
    fn f1fcf0_clone_move_destroy_cycle() {
        fn dispatch(
            _s: &rbx_core::WeakPtr<crate::instance::LuaWebService>,
            _r: crate::generated_13::HttpRequestResult,
            _u: &str,
            _ok: &LuaWebStringFn,
            _err: &LuaWebStringFn,
        ) {
        }
        let src = RawStringWebBind {
            func: dispatch,
            service: empty_weak_service(),
            url: "http://x/".to_owned(),
            on_success: noop_str(),
            on_error: noop_str(),
        };
        let mut dst = RawStringWebCallback::default();
        assert!(stub_f1fcf0(&src, &mut dst, FunctorOp::Clone));
        assert_eq!(dst.inner.as_ref().unwrap().url, "http://x/");
        assert!(!stub_f1fcf0(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.inner.is_none());
    }

    #[test]
    fn f1fcfc_hits_and_misses_cooked_cache() {
        let mut cache = WebCache::new();
        cache.entries.insert("u".to_owned(), (7, cooked_info("b")));
        assert_eq!(stub_f1fcfc(&cache, "u").unwrap().0, 7);
        assert!(stub_f1fcfc(&cache, "missing").is_none());
    }

    #[test]
    fn f1fd08_clone_move_destroy_cycle() {
        fn dispatch(
            _s: &rbx_core::WeakPtr<crate::instance::LuaWebService>,
            _r: crate::generated_13::HttpRequestResult,
            _u: &str,
            _ok: &LuaWebIntFn,
            _err: &LuaWebStringFn,
        ) {
        }
        let src = IntWebBind {
            func: dispatch,
            service: empty_weak_service(),
            url: "http://i/".to_owned(),
            on_success: std::sync::Arc::new(|_: i32| {}),
            on_error: noop_str(),
        };
        let mut dst = IntWebCallback::default();
        assert!(stub_f1fd08(&src, &mut dst, FunctorOp::Clone));
        assert_eq!(dst.inner.as_ref().unwrap().url, "http://i/");
        assert!(stub_f1fd08(&src, &mut dst, FunctorOp::Move));
        assert!(!stub_f1fd08(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.inner.is_none());
    }

    #[test]
    fn f1fd14_clone_and_destroy_cycle() {
        fn dispatch(
            _s: &rbx_core::WeakPtr<crate::instance::LuaWebService>,
            _r: crate::generated_13::HttpRequestResult,
            _u: &str,
            _ok: &LuaWebBoolFn,
            _err: &LuaWebStringFn,
        ) {
        }
        let src = BoolWebBind {
            func: dispatch,
            service: empty_weak_service(),
            url: "http://b/".to_owned(),
            on_success: std::sync::Arc::new(|_: bool| {}),
            on_error: noop_str(),
        };
        let mut dst = BoolWebCallback::default();
        assert!(stub_f1fd14(&src, &mut dst, FunctorOp::Clone));
        assert!(dst.inner.is_some());
        assert!(!stub_f1fd14(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.inner.is_none());
    }

    #[test]
    fn f1fd20_clone_and_destroy_cycle() {
        fn dispatch(
            _s: &rbx_core::WeakPtr<crate::instance::LuaWebService>,
            _r: crate::generated_13::HttpRequestResult,
            _u: &str,
            _ok: &LuaWebMapFn,
            _err: &LuaWebStringFn,
        ) {
        }
        let src = MapWebBind {
            func: dispatch,
            service: empty_weak_service(),
            url: "http://m/".to_owned(),
            on_success: std::sync::Arc::new(
                |_: &SharedPtr<std::collections::HashMap<String, Variant>>| {},
            ),
            on_error: noop_str(),
        };
        let mut dst = MapWebCallback::default();
        assert!(stub_f1fd20(&src, &mut dst, FunctorOp::Clone));
        assert_eq!(dst.inner.as_ref().unwrap().url, "http://m/");
        assert!(!stub_f1fd20(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.inner.is_none());
    }

    #[test]
    fn f1fd2c_retains_map_payload() {
        let mut entries = std::collections::HashMap::new();
        entries.insert("k".to_owned(), Variant::Int(3));
        let src = VariantMapBind { map: SharedPtr::new(entries) };
        let mut dst = VariantMapCallback::default();
        assert!(stub_f1fd2c(&src, &mut dst, FunctorOp::Clone));
        let held = dst.inner.as_ref().unwrap();
        assert!(std::sync::Arc::ptr_eq(&held.map, &src.map));
        assert!(!stub_f1fd2c(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.inner.is_none());
    }

    #[test]
    fn f1fd38_reuses_vector_bind_dispatch() {
        fn dispatch(
            _s: &rbx_core::WeakPtr<crate::instance::LuaWebService>,
            _r: crate::generated_13::HttpRequestResult,
            _u: &str,
            _ok: &crate::generated_datamodel_shard_277::LuaWebSuccessFn,
            _err: &crate::generated_datamodel_shard_277::LuaWebErrorFn,
        ) {
        }
        let src = LuaWebCallbackBind {
            func: dispatch,
            service: empty_weak_service(),
            url: "http://v/".to_owned(),
            on_success: std::sync::Arc::new(|_: &SharedPtr<Vec<Variant>>| {}),
            on_error: std::sync::Arc::new(|_: &str| {}),
        };
        let mut dst = LuaWebCallbackFunction::default();
        assert!(stub_f1fd38(&src, &mut dst, FunctorOp::Clone));
        assert_eq!(dst.inner.as_ref().unwrap().url, "http://v/");
        assert!(!stub_f1fd38(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.inner.is_none());
    }

    #[test]
    fn f1fd44_hits_and_misses_raw_cache() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_owned(), (9, raw_info("b")));
        assert_eq!(stub_f1fd44(&cache, "u").unwrap().0, 9);
        assert!(stub_f1fd44(&cache, "missing").is_none());
    }

    #[test]
    fn f1fd50_evicts_one_raw_entry() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("a".to_owned(), (0, raw_info("a")));
        cache.entries.insert("b".to_owned(), (0, raw_info("b")));
        stub_f1fd50(&mut cache);
        assert_eq!(cache.entries.len(), 1);
        stub_f1fd50(&mut cache);
        assert!(cache.entries.is_empty());
    }

    #[test]
    fn f1fd50_on_empty_cache_is_noop() {
        let mut cache = RawWebCache::new();
        stub_f1fd50(&mut cache);
        assert!(cache.entries.is_empty());
    }

    #[test]
    fn f1fd5c_erases_raw_key() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_owned(), (0, raw_info("b")));
        assert!(stub_f1fd5c(&mut cache, "u"));
        assert!(!stub_f1fd5c(&mut cache, "u"));
    }

    #[test]
    fn f1fd68_rehash_keeps_raw_contents() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_owned(), (0, raw_info("b")));
        stub_f1fd68(&mut cache, 64);
        assert!(cache.entries.contains_key("u"));
    }

    #[test]
    fn f1fd74_create_buckets_keeps_raw_contents() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_owned(), (0, raw_info("b")));
        stub_f1fd74(&mut cache, 64);
        assert!(cache.entries.contains_key("u"));
    }

    #[test]
    fn f1fd80_erases_cooked_key() {
        let mut cache = WebCache::new();
        cache.entries.insert("u".to_owned(), (0, cooked_info("b")));
        assert!(stub_f1fd80(&mut cache, "u"));
        assert!(!stub_f1fd80(&mut cache, "u"));
    }

    #[test]
    fn f1fd8c_rehash_keeps_cooked_contents() {
        let mut cache = WebCache::new();
        cache.entries.insert("u".to_owned(), (0, cooked_info("b")));
        stub_f1fd8c(&mut cache, 64);
        assert!(cache.entries.contains_key("u"));
    }

    #[test]
    fn f1fd98_create_buckets_keeps_cooked_contents() {
        let mut cache = WebCache::new();
        cache.entries.insert("u".to_owned(), (0, cooked_info("b")));
        stub_f1fd98(&mut cache, 64);
        assert!(cache.entries.contains_key("u"));
    }

    #[test]
    fn f1feac_clone_and_destroy_cycle() {
        let src = SlotDoubleBind { target: empty_slot(), on_value: None };
        let mut dst = SlotDoubleCallback::default();
        assert!(stub_f1feac(&src, &mut dst, FunctorOp::Clone));
        assert!(std::sync::Arc::ptr_eq(
            &dst.inner.as_ref().unwrap().target,
            &src.target
        ));
        assert!(!stub_f1feac(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.inner.is_none());
    }

    #[test]
    fn f1fedc_clone_and_destroy_cycle() {
        let src = SlotDouble2Bind { target: empty_slot(), on_values: None };
        let mut dst = SlotDouble2Callback::default();
        assert!(stub_f1fedc(&src, &mut dst, FunctorOp::Clone));
        assert!(std::sync::Arc::ptr_eq(
            &dst.inner.as_ref().unwrap().target,
            &src.target
        ));
        assert!(!stub_f1fedc(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.inner.is_none());
    }
}
