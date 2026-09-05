// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace (10215) complete — fallback global gap filler lowest uncovered EA asc not yet in datamodel
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x35179c..0x357858 | datamodel distinct 34139->34259 global uncovered 52207->52087, lowest gap EA-sorted asc next 120 after watchdog_v (0x35179c..0x357858)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias where needed
// Shard: datamodel_shard_277 EA-sorted ascending next uncovered gap after datamodel_shard_276/watchdog_v (distinct check via export.json sorted EA, no overlap)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use rbx_core::WeakPtr;
use crate::generated_05::{FunctorOp, Variant};
use crate::generated_13::HttpRequestResult;
use crate::instance::LuaWebService;
use rbx_core::shared_ptr::{ControlBlockP, shared_ptr_from_raw};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;


/// Success continuation behind
/// `boost::function<void(shared_ptr<vector<Variant>> const)>`.
pub type LuaWebSuccessFn = Arc<dyn Fn(&SharedPtr<Vec<Variant>>) + Send + Sync>;
/// Error continuation behind `boost::function<void(string)>`.
pub type LuaWebErrorFn = Arc<dyn Fn(&str) + Send + Sync>;

/// Free-function word of the bind (`*a1 = a2` at IDA `0x35283e`): the
/// completion target invoked with the retained weak service, the late-bound
/// result, and the retained url/callbacks.
pub type LuaWebDispatchFn = fn(
    service: &WeakPtr<LuaWebService>,
    result: HttpRequestResult,
    url: &str,
    on_success: &LuaWebSuccessFn,
    on_error: &LuaWebErrorFn,
);

/// Rust model of the `storage4` list (IDA `0x352614`): bound weak service,
/// url string, and success callback. Cloning a value is the C++ retain/copy
/// (weak inc at disasm `0x3526a8`, string copy at `0x3526c0`, callback copy
/// at `0x352704`).
#[derive(Clone)]
pub struct LuaWebCallbackList4 {
    pub service: WeakPtr<LuaWebService>,
    pub url: String,
    pub on_success: LuaWebSuccessFn,
}

/// Rust model of the `storage5` list (IDA `0x352384`): the `storage4` base
/// plus the error callback installed at +28 (disasm `0x352488`/`0x352492`).
#[derive(Clone)]
pub struct LuaWebCallbackStorage5 {
    pub service: WeakPtr<LuaWebService>,
    pub url: String,
    pub on_success: LuaWebSuccessFn,
    pub on_error: LuaWebErrorFn,
}

/// Rust model of the full `bind_t` (IDA `0x35281c`): the function word plus
/// the five list values.
#[derive(Clone)]
pub struct LuaWebCallbackBind {
    pub func: LuaWebDispatchFn,
    pub service: WeakPtr<LuaWebService>,
    pub url: String,
    pub on_success: LuaWebSuccessFn,
    pub on_error: LuaWebErrorFn,
}

/// Rust model of `boost::function3<void, RequestResult, istream*,
/// shared_ptr<string const>>` holding the bind (IDA `0x3517c0`): nullability
/// of the retained bind is the vtable word. Twin of `DataModelCallback`.
#[derive(Default)]
pub struct LuaWebCallbackFunction {
    pub inner: Option<LuaWebCallbackBind>,
}

impl LuaWebCallbackFunction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }

    pub fn call(&self, result: HttpRequestResult) {
        if let Some(bind) = &self.inner {
            stub_0x35179c(bind, result);
        }
    }
}

/// Mangled type name `strcmp`ed by the check-type path (decompile `0x351ce0`,
/// disasm `0x351cda`), mirroring `BIND_PREDICATE_TYPE_NAME` in
/// `generated_05`.
pub const LUA_WEB_CALLBACK_BIND_TYPE_NAME: &str =
    "N5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS3_10Reflection7VariantESaISC_EEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSO_ISsEENSO_ISI_EENSO_ISK_EEEEEE";

/// Rust model of `RBX::LuaWebService::CachedRawLuaWebServiceInfo` (IDA
/// `0x345c24`): the two retained `SharedPtr<String>` words (body plus content
/// type) built by `registerContent` (IDA `0x353378`).
#[derive(Clone)]
pub struct CachedRawLuaWebServiceInfo {
    pub body: SharedPtr<String>,
    pub content_type: SharedPtr<String>,
}

impl CachedRawLuaWebServiceInfo {
    pub fn new(body: SharedPtr<String>, content_type: SharedPtr<String>) -> Self {
        Self { body, content_type }
    }
}

/// Rust model of `RBX::AsyncHttpCache<CachedRawLuaWebServiceInfo, true>`
/// storage: the `unordered_map<string, lru-node>` (IDA `0x352ad8` family)
/// over `(stamp, info)` LRU entries (`pair<string, pair<ulong, Info>>`). The
/// C++ shell (mutex at `+0x120` per disasm `0x353322`, LRU list, size
/// counters) is folded: callers hold the Mutex, recency order is unmodeled.
pub struct RawWebCache {
    pub entries: HashMap<String, (u64, CachedRawLuaWebServiceInfo)>,
}

impl RawWebCache {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }
}

impl Default for RawWebCache {
    fn default() -> Self {
        Self::new()
    }
}
/// Rust model of `RBX::LuaWebService::CachedLuaWebServiceInfo` (IDA
/// `0x345cf4`): the two retained `SharedPtr<String>` words (body plus content
/// type) built by `registerContent` (IDA `0x354eba`). Twin of
/// `CachedRawLuaWebServiceInfo`.
#[derive(Clone)]
pub struct CachedLuaWebServiceInfo {
    pub body: SharedPtr<String>,
    pub content_type: SharedPtr<String>,
}

impl CachedLuaWebServiceInfo {
    pub fn new(body: SharedPtr<String>, content_type: SharedPtr<String>) -> Self {
        Self { body, content_type }
    }
}

/// Rust model of `RBX::AsyncHttpCache<CachedLuaWebServiceInfo, true>`
/// storage: the `unordered_map<string, lru-node>` (IDA `0x354f02` family)
/// over `(stamp, info)` LRU entries (`pair<string, pair<ulong, Info>>`). The
/// C++ shell (mutex at `+0x120` per disasm `0x354e64`, LRU list, size
/// counters) is folded: callers hold the Mutex, recency order is unmodeled.
/// Twin of `RawWebCache`.
pub struct WebCache {
    pub entries: HashMap<String, (u64, CachedLuaWebServiceInfo)>,
}

impl WebCache {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }
}

impl Default for WebCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Smallest prime >= `need`: the boost 38-entry prime-list lower bound
/// (decompile walks at `0x3540ee`, `0x355b76`, `0x354744`, `0x3562e8`;
/// disasm loops at `0x354108`, `0x355b90`, `0x354744`, `0x3562e8` with the
/// end clamp at `0x354122`/`0x355baa`/`0x35476e`/`0x356312`). Trial division
/// is exact for the small sizes used here; the max_load divisor (float at
/// table `+12`) is folded to 1.0 — `HashMap` manages density itself.
fn table_prime_at_least(need: usize) -> usize {
    fn is_prime(n: usize) -> bool {
        if n < 2 {
            return false;
        }
        if n % 2 == 0 {
            return n == 2;
        }
        let mut d = 3;
        while d <= n / d {
            if n % d == 0 {
                return false;
            }
            d += 2;
        }
        true
    }
    let mut n = need.max(2);
    while !is_prime(n) {
        n += 1;
    }
    n
}

// Keeps the `AsyncHttpQueue`/`Instance` contract imports referenced: the
// result word belongs to the queue, and cached payloads surface as Variants
// over instances.
#[allow(dead_code)]

// 0x35179c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEvSA_PSiNSC_IKSsEEE6invokeERNS1_15function_bufferESA_S10_S12_
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
// was: boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)
pub fn stub_0x35179c(bind: &LuaWebCallbackBind, result: HttpRequestResult) {
    // IDA 0x35179c (decompile: packs list3 {&result, &stream, shared<string>} then tail-calls list5::operator() at 0x3517bc; disasm: 0x3517a8 LDR buffer words, 0x3517b2 arg+4, 0x3517b6 BLX operator()).
    // Unwraps the bind and calls func; the istream/shared<string> late args are queue plumbing folded into the retained url/callbacks.
    stub_0x3519f0(bind, result);
}

// 0x3517c0 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt6vectorINS3_10Reflection7VariantESaISK_EEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x3517c0(slot: &mut LuaWebCallbackFunction, bind: LuaWebCallbackBind) -> bool {
    // IDA 0x3517c0 (decompile: bind_t copy at 0x3517e4, tag-overload assign_to at 0x35181c, ~bind_t at 0x351828, return 1 at 0x351848; disasm: 0x3517e4 BLX bind_t copy, 0x35181c BLX assign_to<tag>, 0x351834 MOVS R0, #1).
    // Copies the bind into the slot; the 0x30-word bind always fits the buffer, hence always true.
    slot.inner = Some(bind);
    true
}

// 0x351884 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt6vectorINS3_10Reflection7VariantESaISK_EEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x351884(slot: &mut LuaWebCallbackFunction, bind: LuaWebCallbackBind) -> bool {
    // IDA 0x351884 (decompile: bind_t copy at 0x3518a6, assign_functor at 0x3518dc, ~bind_t at 0x3518e8, return 1 at 0x351908; disasm: 0x3518a6 BLX bind_t, 0x3518dc BLX assign_functor, 0x3518f4 MOVS R0, #1).
    // function_obj_tag overload of 0x3517c0: heap-installs through assign_functor, always fits.
    stub_0x351944(slot, bind);
    true
}

// 0x351944 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt6vectorINS3_10Reflection7VariantESaISK_EEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x351944(slot: &mut LuaWebCallbackFunction, bind: LuaWebCallbackBind) {
    // IDA 0x351944 (decompile: operator new(0x30) at 0x35197a, bind_t copy at 0x3519a0, *a3 = v3 at 0x3519a8; disasm: 0x351960 new 0x30, 0x35199e BLX bind_t, 0x3519a8 STR into buffer).
    // Heap-installs the bind; the Box is the 0x30-byte heap node, moving out installs the memberwise copy.
    let heap: Box<LuaWebCallbackBind> = Box::new(bind);
    slot.inner = Some(*heap);
}

// 0x3519f0 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEENS2_INSB_IFvSsEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSL_SO_ENS0_5list3IRST_RPSiRNSC_IKSsEEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
// was: void boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const> &>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const> &> &,int)
pub fn stub_0x3519f0(bind: &LuaWebCallbackBind, result: HttpRequestResult) {
    // IDA 0x3519f0 (decompile: weak retain under spinlock at 0x351a84, result word copy at 0x351a9a, string copy at 0x351aa8, callback assign_to_own at 0x351aba/0x351acc, indirect call at 0x351ae2, clears at 0x351aea/0x351af4, weak_release at 0x351b18; disasm: 0x351a7c lock, 0x351ae2 BLX R6, 0x351b18 weak_release).
    // Retains/copies args, calls func, releases: the clones are the retains, end-of-scope drops are the clears/releases.
    let service = bind.service.clone();
    let url = bind.url.clone();
    let on_success = Arc::clone(&bind.on_success);
    let on_error = Arc::clone(&bind.on_error);
    (bind.func)(&service, result, &url, &on_success, &on_error);
}

// 0x351c10 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEE7managerERKNS1_15function_bufferERS11_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x351c10(src: &LuaWebCallbackBind, dst: &mut LuaWebCallbackFunction, op: FunctorOp) -> bool {
    // IDA 0x351c10 (decompile: op switch at 0x351c74 — 0: new 0x30 + copy at 0x351c8e/0x351c96; 1: move words + null src at 0x351ca4/0x351ca8; 2: ~bind_t + delete at 0x351cbc/0x351cc2; 3: strcmp type name at 0x351ce0; default: typeinfo at 0x351c6e; disasm: 0x351c74 TBB, 0x351c88 new, 0x351cb8 delete path, 0x351cda strcmp).
    // Move through a shared borrow clones like clone (nulling the source is unobservable); destroy clears the slot.
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

// 0x351d3c — __ZN3RBX13LuaWebService18TryDispatchRequestIN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEbPNS_14AsyncHttpCacheINS0_23CachedLuaWebServiceInfoELb1EEERKSsNS2_8functionIFvT_EEENSH_IFvSsEEE
#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: bool RBX::LuaWebService::TryDispatchRequest<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)
pub fn stub_0x351d3c(
    cache: &RawWebCache,
    url: &str,
    on_success: &LuaWebSuccessFn,
    on_error: &LuaWebErrorFn,
) -> bool {
    // IDA 0x351d3c (decompile: findCacheItem at 0x351db4, hit == 1 at 0x351dbc, typeinfo check at 0x351de2, any_cast at 0x351e54 + success call at 0x351e60 returning 1, wrong-type error "Wrong return data type" at 0x351df6/0x351e02 returning 1, miss returns 0; disasm: 0x351db4 BLX findCacheItem, 0x351dba CMP, 0x351e60 BLX success op, 0x351e02 BLX error op).
    // On hit invokes the success callback with the cached value and returns true; on miss returns false so the caller dispatches async.
    // The stored raw body surfaces as the Text variant (the C++ any_cast of the typed blob is unmodeled); the error continuation covers the unreachable wrong-type arm.
    let _ = on_error;
    match cache.entries.get(url) {
        None => false,
        Some((_, info)) => {
            let values: SharedPtr<Vec<Variant>> =
                SharedPtr::new(vec![Variant::Text(info.body.as_ref().clone())]);
            on_success(&values);
            true
        }
    }
}

// 0x352174 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SM_SP_
#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)
pub fn stub_0x352174(
    service: &WeakPtr<LuaWebService>,
    url: &str,
    on_success: &LuaWebSuccessFn,
    on_error: &LuaWebErrorFn,
    func: LuaWebDispatchFn,
) -> LuaWebCallbackBind {
    // IDA 0x352174 (decompile: weak retain at 0x3521fa/0x352208, string copy at 0x352220, callback assign_to_own at 0x352230/0x352240, storage5 build at 0x352254, clears + weak_release on the temps; disasm: 0x352200 lock, 0x352220 string copy, 0x352254 BLX storage5).
    // Memberwise copy of all five list values; the clones are the retains/copies. func is bind_t word 0, attached here.
    LuaWebCallbackBind {
        func,
        service: service.clone(),
        url: url.to_string(),
        on_success: Arc::clone(on_success),
        on_error: Arc::clone(on_error),
    }
}

// 0x352384 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SM_SP_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: boost::_bi::storage5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)
pub fn stub_0x352384(
    service: &WeakPtr<LuaWebService>,
    url: &str,
    on_success: &LuaWebSuccessFn,
    on_error: &LuaWebErrorFn,
) -> LuaWebCallbackStorage5 {
    // IDA 0x352384 (decompile: weak retain at 0x35240a/0x352418, string copy at 0x352430, success copy at 0x352440, storage4 delegation at 0x352450, error slot zero + assign_to_own at 0x352488/0x352492; disasm: 0x352410 lock, 0x352450 BLX storage4, 0x352492 assign_to_own).
    // Builds the storage4 base first, then installs the error callback, mirroring the delegation.
    let base = stub_0x352614(service, url, on_success);
    LuaWebCallbackStorage5 {
        service: base.service,
        url: base.url,
        on_success: base.on_success,
        on_error: Arc::clone(on_error),
    }
}

// 0x352614 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEEEC2ES7_S9_SA_SM_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>)")]
// was: boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>)
pub fn stub_0x352614(
    service: &WeakPtr<LuaWebService>,
    url: &str,
    on_success: &LuaWebSuccessFn,
) -> LuaWebCallbackList4 {
    // IDA 0x352614 (decompile: weak retain at 0x35269a/0x3526a8, string copy at 0x3526c0, storage3 delegation at 0x3526ce, success slot zero + assign_to_own at 0x3526fa/0x352704; disasm mirrors; // type: int __fastcall(int, int, int, boost::detail::sp_counted_base *)).
    // Memberwise copy of weak + url + success; the clones are the retains/copies.
    LuaWebCallbackList4 {
        service: service.clone(),
        url: url.to_string(),
        on_success: Arc::clone(on_success),
    }
}

// 0x35281c — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS3_10Reflection7VariantESaISC_EEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSO_ISsEENSO_ISI_EENSO_ISK_EEEEEC2ESM_RKSV_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)
pub fn stub_0x35281c(
    func: LuaWebDispatchFn,
    service: &WeakPtr<LuaWebService>,
    url: &str,
    on_success: &LuaWebSuccessFn,
    on_error: &LuaWebErrorFn,
) -> LuaWebCallbackBind {
    // IDA 0x35281c (decompile: *a1 = func at 0x35283e, storage4 memberwise copy at 0x352846, error slot zero + assign_to_own at 0x35285e/0x352886; disasm: 0x35283e STR func, 0x352846 BLX storage4, 0x352886 BLX assign_to_own).
    // Stores the function word plus a memberwise copy of the list.
    LuaWebCallbackBind {
        func,
        service: service.clone(),
        url: url.to_string(),
        on_success: Arc::clone(on_success),
        on_error: Arc::clone(on_error),
    }
}

// 0x35296c — __ZN5boost8weak_ptrIN3RBX13LuaWebServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::WeakPtr<RBX::LuaWebService>::weak_ptr<RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const&,boost::detail::sp_enable_if_convertible<RBX::LuaWebService,RBX::LuaWebService>::type)")]
// was: boost::weak_ptr<RBX::LuaWebService>::weak_ptr<RBX::LuaWebService>(boost::shared_ptr<RBX::LuaWebService> const&,boost::detail::sp_enable_if_convertible<RBX::LuaWebService,RBX::LuaWebService>::type)
pub fn stub_0x35296c(src: &SharedPtr<LuaWebService>) -> WeakPtr<LuaWebService> {
    // IDA 0x35296c (decompile: px/pi word copies at 0x352972/0x352978, weak-count inc under spinlock at 0x3529a2/0x3529b0; disasm: 0x352972 STR px, 0x352978 STR pi, 0x3529a8 lock, 0x3529b0 inc).
    // weak_ptr-from-shared_ptr ctor: same two-word copy plus weak retain is Arc::downgrade.
    Arc::downgrade(src)
}

// 0x3529bc — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EE13findCacheItemERKSsPS2_
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::findCacheItem(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo*)")]
pub use rbx_core::generated_core_shard_hy::stub_3529bc as stub_0x3529bc;

// 0x352ad8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
pub fn stub_0x352ad8(cache: &RawWebCache, url: &str) -> bool {
    // IDA 0x352ad8 (decompile: boost string-hash loop at 0x352ae8 then tail-call to find_node_impl; disasm: 0x352ae8 hash loop with 0x9E3779B9, 0x352b0e B.W find_node_impl shim).
    // Hash + lookup; contains is the same hit/miss report.
    cache.entries.contains_key(url)
}

// 0x352b14 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
pub fn stub_0x352b14<'a>(cache: &'a RawWebCache, url: &str) -> Option<&'a (u64, CachedRawLuaWebServiceInfo)> {
    // IDA 0x352b14 (decompile: bucket = hash % size at 0x352b2e, chain walk with stored-hash compare at 0x352b5a plus string::compare at 0x352b64, hit returns the node else 0; disasm: 0x352b2a umod, 0x352b56 LDR hash, 0x352b60 compare).
    // Hit returns the node payload, miss returns null.
    cache.entries.get(url)
}

// 0x352b80 — __ZN5boost10shared_ptrIN3RBX13LuaWebServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService>(rbx_core::WeakPtr<RBX::LuaWebService> const&,boost::detail::sp_nothrow_tag)")]
// was: boost::shared_ptr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService>(boost::weak_ptr<RBX::LuaWebService> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0x352b80(src: &WeakPtr<LuaWebService>) -> Option<SharedPtr<LuaWebService>> {
    // IDA 0x352b80 (decompile: px zeroed at 0x352b8e, use_count checked under lock at 0x352bc4/0x352bcc, inc + adopt px at 0x352bd0/0x352be4, expired takes the null-pi path at 0x352bea/0x352bf0; disasm: 0x352b8e STR 0, 0x352bcc BEQ expired, 0x352bf0 null).
    // Expired weak -> None, matching the nothrow tag semantics.
    src.upgrade()
}

// 0x352bfc — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEEC2IS5_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")]
// was: boost::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)
pub fn stub_0x352bfc(ptr: *mut RawWebCache) -> SharedPtr<RawWebCache> {
    // IDA 0x352bfc (decompile: px store at 0x352c2c, shared_count new at 0x352c5a, _internal_accept_owner at 0x352c82; disasm: 0x352c2c STR px, 0x352c5a BLX shared_count, 0x352c82 BLX accept_owner; // type: int __fastcall(int, void *, int, int, int, int)).
    // SAFETY: `ptr` must be a live heap allocation owned by the caller; adoption takes ownership (Box -> Arc is the same single-owner adoption).
    // The enable_shared_from_this owner link (accept_owner) is unmodeled: RawWebCache folds only the cache storage.
    shared_ptr_from_raw(unsafe { Box::from_raw(ptr) })
}

// 0x352ce4 — __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)const")]
// was: void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(boost::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)const
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x352ce4 as stub_0x352ce4;

// 0x352e1c — __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")]
pub fn stub_0x352e1c(ptr: *mut RawWebCache) -> ControlBlockP<RawWebCache> {
    // IDA 0x352e1c (decompile: operator new(0x10) at 0x352e70, use/weak counts at 1 at 0x352e7e/0x352e82, vtable at 0x352e88, px at 0x352e8e; disasm mirrors; // type: int __fastcall(int, int, int, int, void *, int)).
    // SAFETY: `ptr` must be a live heap allocation owned by the caller.
    ControlBlockP::new(unsafe { Box::from_raw(ptr) })
}

// 0x352f14 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x352f14 as stub_0x352f14;

// 0x352f18 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x352f18 as stub_0x352f18;

// 0x352f1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::dispose(void)")]
pub fn stub_0x352f1c(block: &mut ControlBlockP<RawWebCache>) {
    // IDA 0x352f1c (decompile: load px at +12, null early-out, virtual delete through D0 at 0x352f28; disasm: 0x352f1c LDR, 0x352f20 IT EQ / 0x352f22 BXEQ, 0x352f28 BX delete).
    // take + drop is exactly dtor-then-free, skipped when null.
    block.dispose();
}

// 0x352f30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::get_deleter(std::type_info const&)")]
pub fn stub_0x352f30(block: &ControlBlockP<RawWebCache>) -> Option<rbx_core::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x352f30 (decompile: return 0; disasm: 0x352f30 MOVS R0, #0 then BX LR). A _p block never carries a deleter.
    block.get_deleter()
}

// 0x352f34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::get_untyped_deleter(void)")]
pub fn stub_0x352f34(block: &ControlBlockP<RawWebCache>) -> Option<rbx_core::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x352f34 (decompile: return 0; disasm: 0x352f34 MOVS R0, #0 then BX LR). Unconditionally null like 0x352f30.
    block.get_untyped_deleter()
}

// 0x353088 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EED1Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::~AsyncHttpCache()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x353088 as stub_0x353088;

// 0x353190 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EED0Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::~AsyncHttpCache()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x353190 as stub_0x353190;

// 0x3532a8 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// was: RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)
pub fn stub_0x3532a8(
    cache: &Mutex<RawWebCache>,
    url: &str,
    body: SharedPtr<String>,
    content_type: SharedPtr<String>,
    max_entries: usize,
) {
    // IDA 0x3532a8 (decompile: FastLog URL at 0x35331a, mutex lock at 0x353336, shared_count retains at 0x35334c/0x353366, CachedRaw ctor at 0x353378, LRU insert with stamp 0 at 0x35338c, size walk + over-capacity check at 0x3533a2/0x3533b0, find_node on the LRU front key at 0x3533c0, unhook/destroy/delete + erase_nodes at 0x3533e0/0x3533ee/0x3533f4/0x35340e, releases at 0x353430/0x35343c, unlock at 0x35344e; disasm mirrors).
    // Insert/overwrite with stamp 0; while over capacity evict the least-recently-used entry. Recency order is folded (HashMap keeps no LRU list), so eviction removes an arbitrary entry; the temp retains/releases are automatic drops.
    let info = CachedRawLuaWebServiceInfo::new(body, content_type);
    let mut guard = cache.lock();
    guard.entries.insert(url.to_string(), (0, info));
    while guard.entries.len() > max_entries {
        let victim: Option<String> = guard.entries.keys().next().cloned();
        match victim {
            Some(key) => {
                guard.entries.remove(&key);
            }
            None => break,
        }
    }
}

// 0x353554 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6insertERKSsRKS2_m
// type: unsigned int __fastcall(int)
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_353554 as stub_0x353554;

// 0x353588 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6insertERKSsRKS2_m
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_353588 as stub_0x353588;

// 0x353b10 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE23removeLeastRecentlyUsedEv
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::removeLeastRecentlyUsed(void)")]
pub use rbx_core::generated_core_shard_hy::stub_353b10 as stub_0x353b10;

// 0x353b68 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6removeERKSs
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::remove(std::string const&)")]
pub use rbx_core::generated_core_shard_hy::stub_353b68 as stub_0x353b68;

// 0x353bbc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *)")]
pub fn stub_0x353bbc(cache: &mut RawWebCache, url: &str) -> bool {
    // IDA 0x353bbc (decompile: bucket locate via hash % size at 0x353bd8, node scan to the range end at 0x353bde/0x353bec, delete_node + fix_bucket per node at 0x353bf2/0x353c00 until the end at 0x353c10; disasm mirrors).
    // Erases the node range for the key — a single-key range here; reports whether anything was erased.
    cache.entries.remove(url).is_some()
}

// 0x353c18 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x353c18(cache: &mut RawWebCache, url: &str) -> bool {
    // IDA 0x353c18 (decompile: unlink at 0x353c2c, string dtor at 0x353c30, operator delete at 0x353c36, --size at 0x353c3c; disasm: 0x353c2c STR unlink, 0x353c30 dtor, 0x353c36 delete, 0x353c3c/0x353c3e decrement).
    // Frees the single node; remove drops the key + value (dtor + free) and the size in one step.
    cache.entries.remove(url).is_some()
}

// 0x353c44 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x353c44(cache: &mut RawWebCache, url: &str) -> bool {
    // IDA 0x353c44 (decompile: recompute hash % size at 0x353c58, relink the bucket head at 0x353c68 or clear it at 0x353c7e; disasm: 0x353c58 umod, 0x353c68 STR relink, 0x353c7e clear).
    // Rust rehashes automatically; remove + reinsert forces the same bucket re-placement. Reports whether the key was present.
    match cache.entries.remove(url) {
        None => false,
        Some(value) => {
            cache.entries.insert(url.to_string(), value);
            true
        }
    }
}

// 0x353c84 — __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEE7destroyEPS6_
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>::destroy(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>*)")]
pub use rbx_core::generated_core_shard_hy::stub_353c84 as stub_0x353c84;

// 0x353d3c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&)")]
pub fn stub_0x353d3c(
    cache: &mut RawWebCache,
    url: &str,
    stamp: u64,
    info: CachedRawLuaWebServiceInfo,
) -> bool {
    // IDA 0x353d3c (decompile: string hash at 0x353d9a, find_node_impl at 0x353dc0, hit returns existing + false at 0x353dc8/0x353dca, miss constructs the node at 0x353dee, reserve_for_insert at 0x353dfc, bucket link at 0x353e1c plus ++size at 0x353e7c, returns new + true; disasm mirrors).
    // Find-or-place with the same newness report, mirroring instance.rs 0x3d8ab4.
    use std::collections::hash_map::Entry;
    match cache.entries.entry(url.to_string()) {
        Entry::Occupied(_) => false,
        Entry::Vacant(slot) => {
            slot.insert((stamp, info));
            true
        }
    }
}

// 0x353eec — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&)")]
pub fn stub_0x353eec(
    url: &str,
    stamp: u64,
    info: CachedRawLuaWebServiceInfo,
) -> (String, (u64, CachedRawLuaWebServiceInfo)) {
    // IDA 0x353eec (decompile: construct() at 0x353ef4, string copy at 0x353f02, hash store at 0x353f08, value flag at 0x353f0c; disasm: 0x353ef4 BLX construct, 0x353f02 string copy, 0x353f08 STR hash).
    // Builds the owned node payload: the copied key plus (stamp, value).
    (url.to_string(), (stamp, info))
}

// 0x353f10 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x353f10(cache: &mut RawWebCache, additional: usize) {
    // IDA 0x353f10 (decompile: buckets exist and size fits -> return at 0x353f1a/0x353f1e, else min_buckets -> rehash at 0x353f34/0x353f5c or create_buckets at 0x353f40/0x353f52; disasm: 0x353f16 CBZ no-buckets, 0x353f1c CMP fit, 0x353f5c rehash shim).
    // Same growth, mirroring instance.rs 0x3d8c44.
    if cache.entries.capacity() < cache.entries.len().saturating_add(additional) {
        cache.entries.reserve(additional);
    }
}

// 0x353f60 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEEEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::~node_constructor()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x353f60 as stub_0x353f60;

// 0x353f80 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
pub fn stub_0x353f80(cache: &mut RawWebCache, buckets: usize) {
    // IDA 0x353f80 (decompile: array_constructor at 0x353fe4, old-table adoption at 0x353ffc/0x354002, size + max_load install at 0x35400e/0x354052 with ceil(size * load) at 0x354038; disasm mirrors).
    // Lays out the bucket array; over a live table reserve keeps contents, mirroring instance.rs 0x3d8c98.
    cache.entries.reserve(buckets.saturating_sub(cache.entries.len()));
}

// 0x3540a8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x3540a8(size: usize) -> usize {
    // IDA 0x3540a8 (decompile: floor(size / max_load) + 1 at 0x3540cc/0x3540e6, prime-list binary search at 0x3540ee/0x354108, end clamp at 0x354122/0x354128; disasm: 0x3540c4 BLX floor, 0x3540e6 ADD #1, 0x3540f6 prime-list load, 0x354108 search loop, 0x354124 clamp).
    // Minimum bucket count fitting `size` entries; the float max_load word is folded, mirroring the non-raw twin 0x355b30.
    table_prime_at_least(size.saturating_add(1))
}

// 0x354138 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
pub fn stub_0x354138(cache: &mut RawWebCache, buckets: usize) {
    // IDA 0x354138 (decompile: create_buckets at 0x35413e, place_in_bucket per overflow node at 0x35415a until the chain end at 0x35415c; disasm: 0x35413e BLX create_buckets, 0x354156 BLX place_in_bucket, 0x35415c chain walk).
    // Rebuilds the bucket array and re-links every node; reserve keeps contents while the automatic rehash is the relink, mirroring the non-raw twin 0x355bc0.
    cache.entries.reserve(buckets.saturating_sub(cache.entries.len()));
}

// 0x354164 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x354164(cache: &mut RawWebCache, url: &str) -> bool {
    // IDA 0x354164 (decompile: stored-hash % size at 0x35417c, occupied-bucket splice at 0x35418e/0x3541a4, empty-bucket install at 0x3541a8/0x3541b2; disasm: 0x35417c umod, 0x354180 bucket test, 0x3541a8 STR install, 0x3541b2 node return).
    // Links the node into its bucket; remove + reinsert forces the same re-placement under HashMap hashing. Reports whether the key was present, mirroring the non-raw twin 0x355bec.
    match cache.entries.remove(url) {
        None => false,
        Some(value) => {
            cache.entries.insert(url.to_string(), value);
            true
        }
    }
}

// 0x3541bc — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct(void)")]
pub fn stub_0x3541bc(url: &str) -> String {
    // IDA 0x3541bc (decompile: live-node reuse path destroys the string and clears the value flag at 0x3541d0/0x3541d6, fresh path news the 0x10 node at 0x3541e0 with zeroing at 0x3541ee and flag install at 0x3541f4; disasm: 0x3541d0 BLX string dtor, 0x3541e0 new 0x10, 0x3541f4 flag store).
    // Allocates the node shell: the copied key; hash/value words are filled by construct_with_value (0x353eec). Mirrors the non-raw twin 0x355c44.
    url.to_string()
}

// 0x3541f8 — __ZNSt4pairISsS_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEC2ERKSsRKS3_
#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo> const&)")]
pub use rbx_core::generated_core_shard_hy::stub_3541f8 as stub_0x3541f8;

// 0x3542c4 — __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEESaIS5_EE14_M_create_nodeERKS5_
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>> const&)")]
pub use rbx_core::generated_core_shard_hy::stub_3542c4 as stub_0x3542c4;

// 0x3543d4 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEED2Ev
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::~LRUCache()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x3543d4 as stub_0x3543d4;

// 0x3544e8 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6resizeEm
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::resize(unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_3544e8 as stub_0x3544e8;

// 0x354520 — __ZNSt10_List_baseISt4pairISsS0_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEESaIS5_EE8_M_clearEv
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_clear(void)")]
pub use rbx_core::generated_core_shard_hy::stub_354520 as stub_0x354520;

// 0x354548 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
pub fn stub_0x354548(cache: &mut RawWebCache) {
    // IDA 0x354548 (decompile: bucket-array null early-out at 0x354552, size check at 0x354556, delete_node per node at 0x354564 until the end at 0x354568, operator delete of the array at 0x354570, zero words at +16 at 0x354578; disasm: 0x354556 LDR size, 0x354564 BLX delete_node, 0x354570 delete, 0x354578 zero).
    // Frees every node and the bucket array itself (unlike clear, 0x354580, which keeps the array): drop entries and release the backing store. Mirrors the non-raw twin 0x3560f8.
    cache.entries.clear();
    cache.entries.shrink_to_fit();
}

// 0x354580 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
pub fn stub_0x354580(cache: &mut RawWebCache) {
    // IDA 0x354580 (decompile: size-zero early-out at 0x354588, delete_node per node at 0x354596 until the end at 0x35459a, memset of the retained bucket array at 0x3545ae; disasm: 0x354588 CBZ, 0x354596 BLX delete_node, 0x3545ae memset).
    // Drops every node but keeps the bucket array for reuse; clear retains capacity. Mirrors the non-raw twin 0x356130.
    cache.entries.clear();
}

// 0x3545b4 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEEC2Ev
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::LRUCache(void)")]
pub use rbx_core::generated_core_shard_hy::stub_3545b4 as stub_0x3545b4;

// 0x354694 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6resizeEm
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::resize(unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_354694 as stub_0x354694;

// 0x354718 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> const&)")]
pub fn stub_0x354718(buckets: usize) -> RawWebCache {
    // IDA 0x354718 (decompile: prime-list walk against the requested count at 0x354744/0x354754, end clamp at 0x35476e/0x354774, size/max_load installs at 0x354778/0x35477c; disasm: 0x35472e prime-list load, 0x354774 prime pick, 0x354778/0x35477c installs).
    // Fresh table over the prime bucket count; with_capacity is the bucket-array layout, mirroring the non-raw twin 0x3562bc.
    RawWebCache { entries: HashMap::with_capacity(table_prime_at_least(buckets)) }
}

// 0x354784 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService23CachedLuaWebServiceInfoELb1EEEEC2IS5_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")]
// was: boost::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)
pub fn stub_0x354784(ptr: *mut WebCache) -> SharedPtr<WebCache> {
    // IDA 0x354784 (decompile: px store at 0x3547b4, shared_count new at 0x3547e2, _internal_accept_owner at 0x35480a; disasm: 0x3547b4 STR px, 0x3547e2 BLX shared_count, 0x35480a BLX accept_owner).
    // SAFETY: `ptr` must be a live heap allocation owned by the caller; adoption takes ownership (Box -> Arc is the same single-owner adoption).
    // The enable_shared_from_this owner link (accept_owner) is unmodeled: WebCache folds only the cache storage. Mirrors the raw twin 0x352bfc.
    shared_ptr_from_raw(unsafe { Box::from_raw(ptr) })
}

// 0x35486c — __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_13LuaWebService23CachedLuaWebServiceInfoELb1EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)const")]
// was: void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(boost::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)const
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x35486c as stub_0x35486c;

// 0x354950 — __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_13LuaWebService23CachedLuaWebServiceInfoELb1EEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")]
pub fn stub_0x354950(ptr: *mut WebCache) -> ControlBlockP<WebCache> {
    // IDA 0x354950 (decompile: operator new(0x10) at 0x3549a4, use/weak counts at 1 at 0x3549b2/0x3549b6, vtable at 0x3549bc, px at 0x3549c2; disasm: 0x3549a4 new, 0x3549b2/0x3549b6 counts, 0x3549b8 vtable load, 0x3549c2 px store).
    // SAFETY: `ptr` must be a live heap allocation owned by the caller. Mirrors the raw twin 0x352e1c.
    ControlBlockP::new(unsafe { Box::from_raw(ptr) })
}

// 0x354a48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x354a48 as stub_0x354a48;

// 0x354a4c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x354a4c as stub_0x354a4c;

// 0x354a50 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::dispose(void)")]
pub fn stub_0x354a50(block: &mut ControlBlockP<WebCache>) {
    // IDA 0x354a50 (decompile: load px at 0x354a50, null early-out at 0x354a54, virtual delete through D0 at 0x354a5c; disasm: 0x354a50 LDR, 0x354a54 CMP/IT EQ, 0x354a58/0x354a5c BX delete).
    // take + drop is exactly dtor-then-free, skipped when null. Mirrors the raw twin 0x352f1c.
    block.dispose();
}

// 0x354a60 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::get_deleter(std::type_info const&)")]
pub fn stub_0x354a60(block: &ControlBlockP<WebCache>) -> Option<rbx_core::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x354a60 (decompile: return 0 at 0x354a62; disasm: 0x354a60 MOVS R0, #0 then BX LR). A _p block never carries a deleter. Mirrors the raw twin 0x352f30.
    block.get_deleter()
}

// 0x354a64 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::get_untyped_deleter(void)")]
pub fn stub_0x354a64(block: &ControlBlockP<WebCache>) -> Option<rbx_core::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x354a64 (decompile: return 0 at 0x354a66; disasm: 0x354a64 MOVS R0, #0, 0x354a66 BX LR). Unconditionally null like 0x354a60. Mirrors the raw twin 0x352f34.
    block.get_untyped_deleter()
}

// 0x354bb8 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EED1Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::~AsyncHttpCache()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x354bb8 as stub_0x354bb8;

// 0x354cc0 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EED0Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::~AsyncHttpCache()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x354cc0 as stub_0x354cc0;

// 0x354dd8 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// was: RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)
pub fn stub_0x354dd8(
    cache: &Mutex<WebCache>,
    url: &str,
    body: SharedPtr<String>,
    content_type: SharedPtr<String>,
    max_entries: usize,
) {
    // IDA 0x354dd8 (decompile: FastLog URL at 0x354e5c, mutex lock at 0x354e78, shared_count retains at 0x354e8e/0x354ea8, CachedLua ctor at 0x354eba, LRU insert with stamp 0 at 0x354ece, size walk + over-capacity check at 0x354ee4/0x354ef2, find_node on the LRU front key at 0x354f02, unhook/destroy/delete + erase_nodes at 0x354f22/0x354f28/0x354f30/0x354f3a, releases at 0x354f56/0x354f62, unlock at 0x354f74; disasm mirrors).
    // Insert/overwrite with stamp 0; while over capacity evict the least-recently-used entry. Recency order is folded (HashMap keeps no LRU list), so eviction removes an arbitrary entry; the temp retains/releases are automatic drops. Mirrors the raw twin 0x3532a8.
    let info = CachedLuaWebServiceInfo::new(body, content_type);
    let mut guard = cache.lock();
    guard.entries.insert(url.to_string(), (0, info));
    while guard.entries.len() > max_entries {
        let victim: Option<String> = guard.entries.keys().next().cloned();
        match victim {
            Some(key) => {
                guard.entries.remove(&key);
            }
            None => break,
        }
    }
}

// 0x355034 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6insertERKSsRKS2_m
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_355034 as stub_0x355034;

// 0x3550a8 — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6insertERKSsRKS2_m
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_3550a8 as stub_0x3550a8;

// 0x35561c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *)")]
pub fn stub_0x35561c(cache: &mut WebCache, url: &str) -> bool {
    // IDA 0x35561c (decompile: bucket locate via hash % size at 0x355638, node scan to the range end at 0x35563e/0x35564c, delete_node + fix_bucket per node at 0x355652/0x355660 until the end at 0x355670; disasm: 0x355634 umod, 0x355652 BLX delete_node, 0x35565c BLX fix_bucket).
    // Erases the node range for the key — a single-key range here; reports whether anything was erased. Mirrors the raw twin 0x353bbc.
    cache.entries.remove(url).is_some()
}

// 0x355678 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x355678(cache: &mut WebCache, url: &str) -> bool {
    // IDA 0x355678 (decompile: unlink at 0x35568c, string dtor at 0x355690, operator delete at 0x355696, --size at 0x35569c; disasm: 0x35568c STR unlink, 0x355690 dtor, 0x355696 delete, 0x35569c/0x35569e decrement).
    // Frees the single node; remove drops the key + value (dtor + free) and the size in one step. Mirrors the raw twin 0x353c18.
    cache.entries.remove(url).is_some()
}

// 0x3556a4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x3556a4(cache: &mut WebCache, url: &str) -> bool {
    // IDA 0x3556a4 (decompile: recompute hash % size at 0x3556b8, same-bucket return at 0x3556be, relink the bucket head at 0x3556c8 or clear it at 0x3556de; disasm: 0x3556b8 umod, 0x3556c8 STR relink, 0x3556de clear).
    // Rust rehashes automatically; remove + reinsert forces the same bucket re-placement. Reports whether the key was present. Mirrors the raw twin 0x353c44.
    match cache.entries.remove(url) {
        None => false,
        Some(value) => {
            cache.entries.insert(url.to_string(), value);
            true
        }
    }
}

// 0x3556e4 — __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE8_M_eraseESt14_List_iteratorIS5_E
// type: int __fastcall(int, std::_List_node_base *this, int, int, int, int)
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>)")]
pub use rbx_core::generated_core_shard_hy::stub_3556e4 as stub_0x3556e4;

// 0x3557bc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
// type: void __fastcall(int, int, char **, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&)")]
pub fn stub_0x3557bc(
    cache: &mut WebCache,
    url: &str,
    stamp: u64,
    info: CachedLuaWebServiceInfo,
) -> bool {
    // IDA 0x3557bc (decompile: string hash at 0x355822/0x355830, find_node_impl at 0x355846, hit returns existing + false at 0x35584e/0x355852, miss constructs the node at 0x355876, reserve_for_insert at 0x355884, bucket link at 0x3558b0/0x3558e6 plus ++size at 0x355904, returns new + true at 0x355908/0x35590c; disasm: 0x355822 hash loop with 0x9E3779B9, 0x355846 BLX find_node_impl, 0x355876 construct_with_value, 0x355884 reserve, 0x355904 increment).
    // Find-or-place with the same newness report. Mirrors the raw twin 0x353d3c.
    use std::collections::hash_map::Entry;
    match cache.entries.entry(url.to_string()) {
        Entry::Occupied(_) => false,
        Entry::Vacant(slot) => {
            slot.insert((stamp, info));
            true
        }
    }
}

// 0x355974 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&)")]
pub fn stub_0x355974(
    url: &str,
    stamp: u64,
    info: CachedLuaWebServiceInfo,
) -> (String, (u64, CachedLuaWebServiceInfo)) {
    // IDA 0x355974 (decompile: construct() at 0x35597c, string copy at 0x35598a, hash store at 0x355990, value flag at 0x355994; disasm: 0x35597c BLX construct, 0x35598a string copy, 0x355990 STR hash).
    // Builds the owned node payload: the copied key plus (stamp, value). Mirrors the raw twin 0x353eec.
    (url.to_string(), (stamp, info))
}

// 0x355998 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x355998(cache: &mut WebCache, additional: usize) {
    // IDA 0x355998 (decompile: buckets exist and size fits -> return at 0x35599e/0x3559a8, else min_buckets -> rehash at 0x3559bc/0x3559e4 or create_buckets at 0x3559c8/0x3559da; disasm: 0x35599e CBZ no-buckets, 0x3559a4 CMP fit, 0x3559da create_buckets shim, 0x3559e4 rehash shim).
    // Same growth. Mirrors the raw twin 0x353f10.
    if cache.entries.capacity() < cache.entries.len().saturating_add(additional) {
        cache.entries.reserve(additional);
    }
}

// 0x3559e8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEEEEED2Ev
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::~node_constructor()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x3559e8 as stub_0x3559e8;

// 0x355a08 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
pub fn stub_0x355a08(cache: &mut WebCache, buckets: usize) {
    // IDA 0x355a08 (decompile: array_constructor at 0x355a6c, old-table adoption at 0x355a84/0x355a8a, size + max_load install at 0x355a96/0x355ada with ceil(size * load) at 0x355ac0; disasm: 0x355a6c construct, 0x355a8a delete old, 0x355ab8 ceil).
    // Lays out the bucket array; over a live table reserve keeps contents. Mirrors the raw twin 0x353f80.
    cache.entries.reserve(buckets.saturating_sub(cache.entries.len()));
}

// 0x355b30 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x355b30(size: usize) -> usize {
    // IDA 0x355b30 (decompile: floor(size / max_load) + 1 at 0x355b54/0x355b6e, prime-list binary search at 0x355b76/0x355b90, end clamp at 0x355baa/0x355bb0; disasm: 0x355b4c BLX floor, 0x355b6e ADD #1, 0x355b7e prime-list load, 0x355b90 search loop, 0x355bac clamp).
    // Minimum bucket count fitting `size` entries; the float max_load word is folded. Mirrors the raw twin 0x3540a8.
    table_prime_at_least(size.saturating_add(1))
}

// 0x355bc0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
pub fn stub_0x355bc0(cache: &mut WebCache, buckets: usize) {
    // IDA 0x355bc0 (decompile: create_buckets at 0x355bc6, place_in_bucket per overflow node at 0x355be2 until the chain end at 0x355be4; disasm: 0x355bc6 BLX create_buckets, 0x355bde BLX place_in_bucket, 0x355be4 chain walk).
    // Rebuilds the bucket array and re-links every node; reserve keeps contents while the automatic rehash is the relink. Mirrors the raw twin 0x354138.
    cache.entries.reserve(buckets.saturating_sub(cache.entries.len()));
}

// 0x355bec — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x355bec(cache: &mut WebCache, url: &str) -> bool {
    // IDA 0x355bec (decompile: stored-hash % size at 0x355c04, occupied-bucket splice at 0x355c16/0x355c2c, empty-bucket install at 0x355c30/0x355c3a; disasm: 0x355c04 umod, 0x355c08 bucket test, 0x355c30 STR install, 0x355c3a node return).
    // Links the node into its bucket; remove + reinsert forces the same re-placement under HashMap hashing. Reports whether the key was present. Mirrors the raw twin 0x354164.
    match cache.entries.remove(url) {
        None => false,
        Some(value) => {
            cache.entries.insert(url.to_string(), value);
            true
        }
    }
}

// 0x355c44 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct(void)")]
pub fn stub_0x355c44(url: &str) -> String {
    // IDA 0x355c44 (decompile: live-node reuse path destroys the string and clears the value flag at 0x355c58/0x355c5e, fresh path news the 0x10 node at 0x355c68 with zeroing at 0x355c76 and flag install at 0x355c7c; disasm: 0x355c58 BLX string dtor, 0x355c68 new 0x10, 0x355c7c flag store).
    // Allocates the node shell: the copied key; hash/value words are filled by construct_with_value (0x355974). Mirrors the raw twin 0x3541bc.
    url.to_string()
}

// 0x355c80 — __ZNSt4pairISsS_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEC2ERKSsRKS3_
#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo> const&)")]
pub use rbx_core::generated_core_shard_hy::stub_355c80 as stub_0x355c80;

// 0x355d60 — __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE14_M_create_nodeERKS5_
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>> const&)")]
pub use rbx_core::generated_core_shard_hy::stub_355d60 as stub_0x355d60;

// 0x355e88 — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEED2Ev
// type: int __fastcall(std::string *, int, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::~LRUCache()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x355e88 as stub_0x355e88;

// 0x355f9c — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6resizeEm
// type: unsigned int __fastcall(unsigned int result, unsigned int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::resize(unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_355f9c as stub_0x355f9c;

// 0x356010 — __ZNSt10_List_baseISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE8_M_clearEv
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, std::string *, int, int, int, int)
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_clear(void)")]
pub use rbx_core::generated_core_shard_hy::stub_356010 as stub_0x356010;

// 0x3560f8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
pub fn stub_0x3560f8(cache: &mut WebCache) {
    // IDA 0x3560f8 (decompile: bucket-array null early-out at 0x356102, size check at 0x356106, delete_node per node at 0x356114 until the end at 0x356118, operator delete of the array at 0x356120, zero words at +16 at 0x356128; disasm: 0x356106 LDR size, 0x356114 BLX delete_node, 0x356120 delete, 0x356128 zero).
    // Frees every node and the bucket array itself (unlike clear, 0x356130, which keeps the array): drop entries and release the backing store. Mirrors the raw twin 0x354548.
    cache.entries.clear();
    cache.entries.shrink_to_fit();
}

// 0x356130 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
pub fn stub_0x356130(cache: &mut WebCache) {
    // IDA 0x356130 (decompile: size-zero early-out at 0x356138, delete_node per node at 0x356146 until the end at 0x35614a, memset of the retained bucket array at 0x35615e; disasm: 0x356138 CBZ, 0x356146 BLX delete_node, 0x35615e memset).
    // Drops every node but keeps the bucket array for reuse; clear retains capacity. Mirrors the raw twin 0x354580.
    cache.entries.clear();
}

// 0x356164 — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEEC2Ev
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::LRUCache(void)")]
pub use rbx_core::generated_core_shard_hy::stub_356164 as stub_0x356164;

// 0x356244 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6resizeEm
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::resize(unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_356244 as stub_0x356244;

// 0x3562bc — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>> const&)")]
pub fn stub_0x3562bc(buckets: usize) -> WebCache {
    // IDA 0x3562bc (decompile: prime-list walk against the requested count at 0x3562e8/0x3562f8, end clamp at 0x356312/0x356318, size/max_load installs at 0x35631c/0x356320; disasm: 0x3562d2 prime-list load, 0x356318 prime pick, 0x35631c/0x356320 installs).
    // Fresh table over the prime bucket count; with_capacity is the bucket-array layout. Mirrors the raw twin 0x354718.
    WebCache { entries: HashMap::with_capacity(table_prime_at_least(buckets)) }
}

// 0x3565e4 — __GLOBAL__I_a_126
#[doc(alias = "global constructor keyed to_a_126")]
pub use rbx_core::generated_core_shard_hy::stub_3565e4 as stub_0x3565e4;

// 0x35677c — __ZN3RBX4Math12sumDeltaAxisERKN3G3D7Matrix3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::sumDeltaAxis(G3D::Matrix3 const&,G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_35677c as stub_0x35677c;

// 0x3567e0 — __ZN3RBX4Math19mulMatrixDiagVectorERKN3G3D7Matrix3ERKNS1_7Vector3ERS2_
#[doc(alias = "RBX::Math::mulMatrixDiagVector(G3D::Matrix3 const&,G3D::Vector3 const&,G3D::Matrix3&)")]
pub use rbx_core::generated_core_shard_hy::stub_3567e0 as stub_0x3567e0;

// 0x356878 — __ZN3RBX4Math24mulMatrixMatrixTransposeERKN3G3D7Matrix3ES4_RS2_
#[doc(alias = "RBX::Math::mulMatrixMatrixTranspose(G3D::Matrix3 const&,G3D::Matrix3 const&,G3D::Matrix3&)")]
pub use rbx_core::generated_core_shard_hy::stub_356878 as stub_0x356878;

// 0x3568e0 — __ZN3RBX4Math18deltaRotationCloseEff
// type: _DWORD __fastcall(RBX::Math *__hidden this, float, float)
#[doc(alias = "RBX::Math::deltaRotationClose(float,float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x3568e0 as stub_0x3568e0;

// 0x3569d8 — __ZN3RBX4Math20averageRotationCloseEff
// type: _DWORD __fastcall(RBX::Math *__hidden this, float, float)
#[doc(alias = "RBX::Math::averageRotationClose(float,float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x3569d8 as stub_0x3569d8;

// 0x356ae0 — __ZN3RBX4Math13getFocusSpaceERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Math::getFocusSpace(G3D::CoordinateFrame const&)")]
pub use rbx_core::generated_core_shard_hy::stub_356ae0 as stub_0x356ae0;

// 0x356b18 — __ZN3RBX4Math19getHeadingElevationERKN3G3D15CoordinateFrameERfS5_
// type: double __fastcall(RBX::Math *this, const G3D::CoordinateFrame *, float *, float *)
#[doc(alias = "RBX::Math::getHeadingElevation(G3D::CoordinateFrame const&,float &,float &)")]
pub use rbx_core::generated_core_shard_hy::stub_356b18 as stub_0x356b18;

// 0x356b84 — __ZN3RBX4Math19setHeadingElevationERN3G3D15CoordinateFrameEff
// type: _DWORD __fastcall(RBX::Math *__hidden this, G3D::CoordinateFrame *, float, float)
#[doc(alias = "RBX::Math::setHeadingElevation(G3D::CoordinateFrame &,float,float)")]
pub use rbx_core::generated_core_shard_hy::stub_356b84 as stub_0x356b84;

// 0x356c3c — __ZN3RBX4Math8lessThanERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "RBX::Math::lessThan(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_356c3c as stub_0x356c3c;

// 0x356c80 — __ZN3RBX4Math10isDenormalEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, float)
#[doc(alias = "RBX::Math::isDenormal(float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x356c80 as stub_0x356c80;

// 0x356c94 — __ZN3RBX4Math8isNanInfEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, float)
#[doc(alias = "RBX::Math::isNanInf(float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x356c94 as stub_0x356c94;

// 0x356cc8 — __ZN3RBX4Math15isNanInfVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Math::isNanInfVector3(G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_356cc8 as stub_0x356cc8;

// 0x356d38 — __ZN3RBX4Math21isNanInfDenormVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Math::isNanInfDenormVector3(G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_356d38 as stub_0x356d38;

// 0x356d70 — __ZN3RBX4Math11hasNanOrInfERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Math::hasNanOrInf(G3D::CoordinateFrame const&)")]
pub use rbx_core::generated_core_shard_hy::stub_356d70 as stub_0x356d70;

// 0x356df4 — __ZN3RBX4Math9fixDenormERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, Vector3 *)
#[doc(alias = "RBX::Math::fixDenorm(G3D::Vector3 &)")]
pub use rbx_core::generated_core_shard_hy::stub_356df4 as stub_0x356df4;

// 0x356e34 — __ZN3RBX14segSizeRadiansEv
// type: _DWORD __fastcall(RBX *__hidden this)
#[doc(alias = "RBX::segSizeRadians(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x356e34 as stub_0x356e34;

// 0x356e6c — __ZN3RBX18rotationToByteBaseEf
// type: _DWORD __fastcall(RBX *__hidden this, float)
#[doc(alias = "RBX::rotationToByteBase(float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x356e6c as stub_0x356e6c;

// 0x356ff0 — __ZN3RBX4Math14rotationToByteEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, float)
#[doc(alias = "RBX::Math::rotationToByte(float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x356ff0 as stub_0x356ff0;

// 0x3570e8 — __ZN3RBX4Math16rotationFromByteEh
// type: _DWORD __fastcall(RBX::Math *__hidden this, unsigned __int8)
#[doc(alias = "RBX::Math::rotationFromByte(unsigned char)")]
pub use rbx_core::generated_core_shard_aq::stub_0x3570e8 as stub_0x3570e8;

// 0x3571c0 — __ZN3RBX4Math15getIBodyAtPointERKN3G3D7Vector3ERKNS1_7Matrix3Ef
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Matrix3 *, float)
#[doc(alias = "RBX::Math::getIBodyAtPoint(G3D::Vector3 const&,G3D::Matrix3 const&,float)")]
pub use rbx_core::generated_core_shard_hy::stub_3571c0 as stub_0x3571c0;

// 0x357250 — __ZN3RBX4Math19momentToObjectSpaceERKN3G3D7Matrix3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::momentToObjectSpace(G3D::Matrix3 const&,G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_357250 as stub_0x357250;

// 0x3572c4 — __ZN3RBX4Math10toDiagonalERKN3G3D7Matrix3E
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "RBX::Math::toDiagonal(G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_3572c4 as stub_0x3572c4;

// 0x3572e4 — __ZN3RBX4Math26fromVectorToVectorRotationERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::fromVectorToVectorRotation(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_3572e4 as stub_0x3572e4;

// 0x357450 — __ZN3RBX4Math24fromRotationAxisAndAngleERKN3G3D7Vector3ERKf
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const float *)
#[doc(alias = "RBX::Math::fromRotationAxisAndAngle(G3D::Vector3 const&,float const&)")]
pub use rbx_core::generated_core_shard_hy::stub_357450 as stub_0x357450;

// 0x3575bc — __ZN3RBX4Math25orthonormalizeIfNecessaryERN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, G3D::Matrix3 *)
#[doc(alias = "RBX::Math::orthonormalizeIfNecessary(G3D::Matrix3 &)")]
pub use rbx_core::generated_core_shard_hy::stub_3575bc as stub_0x3575bc;

// 0x3575dc — __ZN3RBX4Math20fromDirectionCosinesERKN3G3D7Vector3ES4_S4_S4_S4_S4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::fromDirectionCosines(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_3575dc as stub_0x3575dc;

// 0x357744 — __ZN3RBX4Math13isAxisAlignedERKN3G3D7Matrix3E
// type: int __fastcall(RBX::Math *this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::isAxisAligned(G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_357744 as stub_0x357744;

// 0x35781c — __ZN3RBX4Math11getOrientIdERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::getOrientId(G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_35781c as stub_0x35781c;

// 0x357858 — __ZN3RBX4Math11idToMatrix3EiRN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, int, G3D::Matrix3 *)
#[doc(alias = "RBX::Math::idToMatrix3(int,G3D::Matrix3 &)")]
pub use rbx_core::generated_core_shard_hz::stub_357858 as stub_0x357858;

#[cfg(test)]
mod shard_277_batch_a_tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    static DISPATCHED: AtomicBool = AtomicBool::new(false);
    static SUCCESS_LEN: AtomicUsize = AtomicUsize::new(usize::MAX);
    static ERRORED: AtomicBool = AtomicBool::new(false);

    fn record_dispatch(
        _service: &WeakPtr<LuaWebService>,
        _result: HttpRequestResult,
        _url: &str,
        _on_success: &LuaWebSuccessFn,
        _on_error: &LuaWebErrorFn,
    ) {
        DISPATCHED.store(true, Ordering::SeqCst);
    }

    fn test_success() -> LuaWebSuccessFn {
        Arc::new(|values: &SharedPtr<Vec<Variant>>| {
            SUCCESS_LEN.store(values.len(), Ordering::SeqCst);
        })
    }

    fn test_error() -> LuaWebErrorFn {
        Arc::new(|_: &str| {
            ERRORED.store(true, Ordering::SeqCst);
        })
    }

    fn live_service() -> SharedPtr<LuaWebService> {
        SharedPtr::new(LuaWebService::default())
    }

    fn test_bind() -> LuaWebCallbackBind {
        let svc = live_service();
        LuaWebCallbackBind {
            func: record_dispatch,
            service: Arc::downgrade(&svc),
            url: "http://example.test/x".to_string(),
            on_success: test_success(),
            on_error: test_error(),
        }
    }

    fn test_info(body: &str) -> CachedRawLuaWebServiceInfo {
        CachedRawLuaWebServiceInfo::new(
            SharedPtr::new(body.to_string()),
            SharedPtr::new("text/plain".to_string()),
        )
    }

    #[test]
    fn invoker_0x35179c_calls_bound_func() {
        DISPATCHED.store(false, Ordering::SeqCst);
        let bind = test_bind();
        stub_0x35179c(&bind, 1);
        assert!(DISPATCHED.load(Ordering::SeqCst));
    }

    #[test]
    fn assign_to_0x3517c0_installs_and_reports_fit() {
        let mut slot = LuaWebCallbackFunction::new();
        assert!(slot.is_empty());
        assert!(stub_0x3517c0(&mut slot, test_bind()));
        assert!(!slot.is_empty());
        assert_eq!(slot.inner.as_ref().unwrap().url, "http://example.test/x");
    }

    #[test]
    fn assign_to_tag_0x351884_installs_and_reports_fit() {
        let mut slot = LuaWebCallbackFunction::new();
        assert!(stub_0x351884(&mut slot, test_bind()));
        assert!(!slot.is_empty());
    }

    #[test]
    fn assign_functor_0x351944_heap_installs() {
        // `test_bind()` cannot keep its service alive past return, so build
        // the bind against a service owned by this scope.
        let svc = live_service();
        let bind = LuaWebCallbackBind {
            func: record_dispatch,
            service: Arc::downgrade(&svc),
            url: "http://example.test/x".to_string(),
            on_success: test_success(),
            on_error: test_error(),
        };
        let mut slot = LuaWebCallbackFunction::new();
        stub_0x351944(&mut slot, bind);
        let inner = slot.inner.as_ref().unwrap();
        assert_eq!(inner.url, "http://example.test/x");
        assert!(inner.service.upgrade().is_some());
    }
    #[test]
    fn list_operator_0x3519f0_calls_func() {
        DISPATCHED.store(false, Ordering::SeqCst);
        let bind = test_bind();
        stub_0x3519f0(&bind, 0);
        assert!(DISPATCHED.load(Ordering::SeqCst));
    }

    #[test]
    fn manager_0x351c10_dispatches_ops() {
        let src = test_bind();
        let mut dst = LuaWebCallbackFunction::new();
        assert!(stub_0x351c10(&src, &mut dst, FunctorOp::Clone));
        assert_eq!(dst.inner.as_ref().unwrap().url, src.url);
        assert!(stub_0x351c10(&src, &mut dst, FunctorOp::Move));
        assert!(!dst.is_empty());
        assert!(stub_0x351c10(&src, &mut dst, FunctorOp::CheckType));
        assert!(stub_0x351c10(&src, &mut dst, FunctorOp::GetType));
        assert!(!stub_0x351c10(&src, &mut dst, FunctorOp::Destroy));
        assert!(dst.is_empty());
    }

    #[test]
    fn try_dispatch_0x351d3c_hit_calls_success_miss_returns_false() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_string(), (0, test_info("hello")));
        SUCCESS_LEN.store(usize::MAX, Ordering::SeqCst);
        ERRORED.store(false, Ordering::SeqCst);
        let ok = stub_0x351d3c(&cache, "u", &test_success(), &test_error());
        assert!(ok);
        assert_eq!(SUCCESS_LEN.load(Ordering::SeqCst), 1);
        assert!(!ERRORED.load(Ordering::SeqCst));
        let miss = stub_0x351d3c(&cache, "nope", &test_success(), &test_error());
        assert!(!miss);
    }

    #[test]
    fn list5_ctor_0x352174_copies_fields() {
        let svc = live_service();
        let weak = Arc::downgrade(&svc);
        let bind = stub_0x352174(&weak, "u", &test_success(), &test_error(), record_dispatch);
        assert_eq!(bind.url, "u");
        assert!(bind.service.upgrade().is_some());
        DISPATCHED.store(false, Ordering::SeqCst);
        stub_0x3519f0(&bind, 1);
        assert!(DISPATCHED.load(Ordering::SeqCst));
    }

    #[test]
    fn storage5_ctor_0x352384_installs_error_cb() {
        let svc = live_service();
        let weak = Arc::downgrade(&svc);
        let s = stub_0x352384(&weak, "u", &test_success(), &test_error());
        assert_eq!(s.url, "u");
        assert!(s.service.upgrade().is_some());
        ERRORED.store(false, Ordering::SeqCst);
        (s.on_error)("boom");
        assert!(ERRORED.load(Ordering::SeqCst));
    }

    #[test]
    fn storage4_ctor_0x352614_copies_three_fields() {
        let svc = live_service();
        let weak = Arc::downgrade(&svc);
        let l = stub_0x352614(&weak, "u", &test_success());
        assert_eq!(l.url, "u");
        assert!(l.service.upgrade().is_some());
        SUCCESS_LEN.store(usize::MAX, Ordering::SeqCst);
        (l.on_success)(&SharedPtr::new(vec![]));
        assert_eq!(SUCCESS_LEN.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn bind_t_ctor_0x35281c_stores_func_word() {
        let svc = live_service();
        let weak = Arc::downgrade(&svc);
        let bind = stub_0x35281c(record_dispatch, &weak, "u", &test_success(), &test_error());
        assert_eq!(bind.func as usize, record_dispatch as usize);
        assert_eq!(bind.url, "u");
    }

    #[test]
    fn weak_copy_0x35296c_downgrades_shared() {
        let svc = live_service();
        let copy = stub_0x35296c(&svc);
        assert!(copy.upgrade().is_some());
        drop(svc);
        assert!(copy.upgrade().is_none());
    }

    #[test]
    fn find_node_0x352ad8_reports_membership() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_string(), (0, test_info("b")));
        assert!(stub_0x352ad8(&cache, "u"));
        assert!(!stub_0x352ad8(&cache, "missing"));
    }

    #[test]
    fn find_node_impl_0x352b14_returns_payload_or_none() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_string(), (7, test_info("b")));
        let hit = stub_0x352b14(&cache, "u").unwrap();
        assert_eq!(hit.0, 7);
        assert_eq!(hit.1.body.as_ref().as_str(), "b");
        assert!(stub_0x352b14(&cache, "missing").is_none());
    }

    #[test]
    fn shared_from_weak_0x352b80_upgrades_or_none() {
        let svc = live_service();
        let weak = Arc::downgrade(&svc);
        let shared = stub_0x352b80(&weak).unwrap();
        assert!(Arc::ptr_eq(&shared, &svc));
        drop(svc);
        drop(shared);
        assert!(stub_0x352b80(&weak).is_none());
    }

    #[test]
    fn shared_ptr_ctor_0x352bfc_adopts_raw() {
        let ptr = Box::into_raw(Box::new(RawWebCache::new()));
        let shared = stub_0x352bfc(ptr);
        assert_eq!(Arc::strong_count(&shared), 1);
        assert!(shared.entries.is_empty());
    }

    #[test]
    fn shared_count_ctor_0x352e1c_fresh_counts() {
        let ptr = Box::into_raw(Box::new(RawWebCache::new()));
        let block = stub_0x352e1c(ptr);
        assert_eq!(block.use_count(), 1);
        assert_eq!(block.weak_count(), 1);
        assert!(block.get().is_some());
        let mut block = block;
        stub_0x352f1c(&mut block);
        assert!(block.get().is_none());
    }

    #[test]
    fn dispose_0x352f1c_drops_payload() {
        let ptr = Box::into_raw(Box::new(RawWebCache::new()));
        let mut block = ControlBlockP::new(unsafe { Box::from_raw(ptr) });
        stub_0x352f1c(&mut block);
        assert!(block.get().is_none());
    }

    #[test]
    fn get_deleter_0x352f30_returns_none() {
        let ptr = Box::into_raw(Box::new(RawWebCache::new()));
        let block = ControlBlockP::new(unsafe { Box::from_raw(ptr) });
        assert!(stub_0x352f30(&block).is_none());
        let mut block = block;
        stub_0x352f1c(&mut block);
    }

    #[test]
    fn get_untyped_deleter_0x352f34_returns_none() {
        let ptr = Box::into_raw(Box::new(RawWebCache::new()));
        let block = ControlBlockP::new(unsafe { Box::from_raw(ptr) });
        assert!(stub_0x352f34(&block).is_none());
        let mut block = block;
        stub_0x352f1c(&mut block);
    }

    #[test]
    fn register_content_0x3532a8_inserts_with_stamp_and_evicts() {
        let cache = Mutex::new(RawWebCache::new());
        stub_0x3532a8(
            &cache,
            "a",
            SharedPtr::new("body-a".to_string()),
            SharedPtr::new("text/plain".to_string()),
            8,
        );
        stub_0x3532a8(
            &cache,
            "b",
            SharedPtr::new("body-b".to_string()),
            SharedPtr::new("text/plain".to_string()),
            8,
        );
        {
            let guard = cache.lock();
            assert_eq!(guard.entries.len(), 2);
            assert_eq!(guard.entries["a"].0, 0);
            assert_eq!(guard.entries["b"].1.body.as_ref().as_str(), "body-b");
        }
        stub_0x3532a8(
            &cache,
            "c",
            SharedPtr::new("body-c".to_string()),
            SharedPtr::new("text/plain".to_string()),
            0,
        );
        assert!(cache.lock().entries.is_empty());
    }

    #[test]
    fn erase_nodes_0x353bbc_removes_key() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_string(), (0, test_info("b")));
        assert!(stub_0x353bbc(&mut cache, "u"));
        assert!(!stub_0x353bbc(&mut cache, "u"));
    }

    #[test]
    fn delete_node_0x353c18_frees_single_node() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_string(), (0, test_info("b")));
        cache.entries.insert("v".to_string(), (0, test_info("c")));
        assert!(stub_0x353c18(&mut cache, "u"));
        assert!(cache.entries.contains_key("v"));
        assert!(!stub_0x353c18(&mut cache, "u"));
    }

    #[test]
    fn fix_bucket_0x353c44_relinks_present_key() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_string(), (5, test_info("b")));
        assert!(stub_0x353c44(&mut cache, "u"));
        assert_eq!(cache.entries["u"].0, 5);
        assert!(!stub_0x353c44(&mut cache, "missing"));
    }

    #[test]
    fn emplace_impl_0x353d3c_inserts_new_only() {
        let mut cache = RawWebCache::new();
        assert!(stub_0x353d3c(&mut cache, "u", 3, test_info("b")));
        assert!(!stub_0x353d3c(&mut cache, "u", 9, test_info("other")));
        assert_eq!(cache.entries["u"].0, 3);
        assert_eq!(cache.entries["u"].1.body.as_ref().as_str(), "b");
    }

    #[test]
    fn construct_with_value_0x353eec_builds_node() {
        let (key, (stamp, info)) = stub_0x353eec("u", 11, test_info("b"));
        assert_eq!(key, "u");
        assert_eq!(stamp, 11);
        assert_eq!(info.content_type.as_ref().as_str(), "text/plain");
    }

    #[test]
    fn reserve_for_insert_0x353f10_grows_when_needed() {
        let mut cache = RawWebCache::new();
        stub_0x353f10(&mut cache, 64);
        assert!(cache.entries.capacity() >= 64);
        let before = cache.entries.capacity();
        stub_0x353f10(&mut cache, 1);
        assert_eq!(cache.entries.capacity(), before);
    }

    #[test]
    fn create_buckets_0x353f80_lays_out_buckets() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_string(), (0, test_info("b")));
        stub_0x353f80(&mut cache, 64);
        assert!(cache.entries.capacity() >= 64);
        assert!(cache.entries.contains_key("u"));
    }
}

#[cfg(test)]
mod shard_277_batch_b_tests {
    use super::*;

    fn test_info(body: &str) -> CachedLuaWebServiceInfo {
        CachedLuaWebServiceInfo::new(
            SharedPtr::new(body.to_string()),
            SharedPtr::new("text/plain".to_string()),
        )
    }

    fn test_raw_info(body: &str) -> CachedRawLuaWebServiceInfo {
        CachedRawLuaWebServiceInfo::new(
            SharedPtr::new(body.to_string()),
            SharedPtr::new("text/plain".to_string()),
        )
    }

    #[test]
    fn shared_types_hold_both_words() {
        let info = test_info("b");
        assert_eq!(info.body.as_ref().as_str(), "b");
        assert_eq!(info.content_type.as_ref().as_str(), "text/plain");
        assert!(WebCache::new().entries.is_empty());
        assert!(WebCache::default().entries.is_empty());
    }

    #[test]
    fn min_buckets_0x3540a8_rounds_to_prime() {
        assert_eq!(stub_0x3540a8(0), 2);
        assert_eq!(stub_0x3540a8(10), 11);
        assert_eq!(stub_0x3540a8(12), 13);
        assert!(stub_0x3540a8(100) >= 101);
        assert_eq!(stub_0x3540a8(100), stub_0x3540a8(100));
    }

    #[test]
    fn rehash_0x354138_rebuilds_keeping_contents() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_string(), (1, test_raw_info("b")));
        cache.entries.insert("v".to_string(), (2, test_raw_info("c")));
        stub_0x354138(&mut cache, 64);
        assert!(cache.entries.capacity() >= 64);
        assert_eq!(cache.entries["u"].0, 1);
        assert_eq!(cache.entries["v"].1.body.as_ref().as_str(), "c");
    }

    #[test]
    fn place_in_bucket_0x354164_relinks_present_key() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_string(), (5, test_raw_info("b")));
        assert!(stub_0x354164(&mut cache, "u"));
        assert_eq!(cache.entries["u"].0, 5);
        assert_eq!(cache.entries["u"].1.body.as_ref().as_str(), "b");
        assert!(!stub_0x354164(&mut cache, "missing"));
    }

    #[test]
    fn construct_0x3541bc_allocates_key_shell() {
        assert_eq!(stub_0x3541bc("u"), "u");
    }

    #[test]
    fn delete_buckets_0x354548_frees_nodes_and_array() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_string(), (0, test_raw_info("b")));
        cache.entries.insert("v".to_string(), (0, test_raw_info("c")));
        stub_0x354548(&mut cache);
        assert!(cache.entries.is_empty());
        assert_eq!(cache.entries.capacity(), 0);
    }

    #[test]
    fn clear_0x354580_drops_nodes_keeps_array() {
        let mut cache = RawWebCache::new();
        cache.entries.insert("u".to_string(), (0, test_raw_info("b")));
        stub_0x353f80(&mut cache, 64);
        let before = cache.entries.capacity();
        assert!(before >= 64);
        stub_0x354580(&mut cache);
        assert!(cache.entries.is_empty());
        assert_eq!(cache.entries.capacity(), before);
    }

    #[test]
    fn table_ctor_0x354718_sizes_fresh_table() {
        let cache = stub_0x354718(8);
        assert!(cache.entries.is_empty());
        assert!(cache.entries.capacity() >= 8);
    }

    #[test]
    fn shared_ptr_ctor_0x354784_adopts_raw() {
        let ptr = Box::into_raw(Box::new(WebCache::new()));
        let shared = stub_0x354784(ptr);
        assert_eq!(Arc::strong_count(&shared), 1);
        assert!(shared.entries.is_empty());
    }

    #[test]
    fn shared_count_ctor_0x354950_fresh_counts() {
        let ptr = Box::into_raw(Box::new(WebCache::new()));
        let block = stub_0x354950(ptr);
        assert_eq!(block.use_count(), 1);
        assert_eq!(block.weak_count(), 1);
        assert!(block.get().is_some());
        let mut block = block;
        stub_0x354a50(&mut block);
        assert!(block.get().is_none());
    }

    #[test]
    fn dispose_0x354a50_drops_payload() {
        let ptr = Box::into_raw(Box::new(WebCache::new()));
        let mut block = ControlBlockP::new(unsafe { Box::from_raw(ptr) });
        stub_0x354a50(&mut block);
        assert!(block.get().is_none());
    }

    #[test]
    fn get_deleter_0x354a60_returns_none() {
        let ptr = Box::into_raw(Box::new(WebCache::new()));
        let block = ControlBlockP::new(unsafe { Box::from_raw(ptr) });
        assert!(stub_0x354a60(&block).is_none());
        let mut block = block;
        stub_0x354a50(&mut block);
    }

    #[test]
    fn get_untyped_deleter_0x354a64_returns_none() {
        let ptr = Box::into_raw(Box::new(WebCache::new()));
        let block = ControlBlockP::new(unsafe { Box::from_raw(ptr) });
        assert!(stub_0x354a64(&block).is_none());
        let mut block = block;
        stub_0x354a50(&mut block);
    }

    #[test]
    fn register_content_0x354dd8_inserts_with_stamp_and_evicts() {
        let cache = Mutex::new(WebCache::new());
        stub_0x354dd8(
            &cache,
            "a",
            SharedPtr::new("body-a".to_string()),
            SharedPtr::new("text/plain".to_string()),
            8,
        );
        stub_0x354dd8(
            &cache,
            "b",
            SharedPtr::new("body-b".to_string()),
            SharedPtr::new("text/plain".to_string()),
            8,
        );
        {
            let guard = cache.lock();
            assert_eq!(guard.entries.len(), 2);
            assert_eq!(guard.entries["a"].0, 0);
            assert_eq!(guard.entries["b"].1.body.as_ref().as_str(), "body-b");
        }
        stub_0x354dd8(
            &cache,
            "c",
            SharedPtr::new("body-c".to_string()),
            SharedPtr::new("text/plain".to_string()),
            0,
        );
        assert!(cache.lock().entries.is_empty());
    }

    #[test]
    fn erase_nodes_0x35561c_removes_key() {
        let mut cache = WebCache::new();
        cache.entries.insert("u".to_string(), (0, test_info("b")));
        assert!(stub_0x35561c(&mut cache, "u"));
        assert!(!stub_0x35561c(&mut cache, "u"));
    }

    #[test]
    fn delete_node_0x355678_frees_single_node() {
        let mut cache = WebCache::new();
        cache.entries.insert("u".to_string(), (0, test_info("b")));
        cache.entries.insert("v".to_string(), (0, test_info("c")));
        assert!(stub_0x355678(&mut cache, "u"));
        assert!(cache.entries.contains_key("v"));
        assert!(!stub_0x355678(&mut cache, "u"));
    }

    #[test]
    fn fix_bucket_0x3556a4_relinks_present_key() {
        let mut cache = WebCache::new();
        cache.entries.insert("u".to_string(), (5, test_info("b")));
        assert!(stub_0x3556a4(&mut cache, "u"));
        assert_eq!(cache.entries["u"].0, 5);
        assert!(!stub_0x3556a4(&mut cache, "missing"));
    }

    #[test]
    fn emplace_impl_0x3557bc_inserts_new_only() {
        let mut cache = WebCache::new();
        assert!(stub_0x3557bc(&mut cache, "u", 3, test_info("b")));
        assert!(!stub_0x3557bc(&mut cache, "u", 9, test_info("other")));
        assert_eq!(cache.entries["u"].0, 3);
        assert_eq!(cache.entries["u"].1.body.as_ref().as_str(), "b");
    }

    #[test]
    fn construct_with_value_0x355974_builds_node() {
        let (key, (stamp, info)) = stub_0x355974("u", 11, test_info("b"));
        assert_eq!(key, "u");
        assert_eq!(stamp, 11);
        assert_eq!(info.content_type.as_ref().as_str(), "text/plain");
    }

    #[test]
    fn reserve_for_insert_0x355998_grows_when_needed() {
        let mut cache = WebCache::new();
        stub_0x355998(&mut cache, 64);
        assert!(cache.entries.capacity() >= 64);
        let before = cache.entries.capacity();
        stub_0x355998(&mut cache, 1);
        assert_eq!(cache.entries.capacity(), before);
    }

    #[test]
    fn create_buckets_0x355a08_lays_out_buckets() {
        let mut cache = WebCache::new();
        cache.entries.insert("u".to_string(), (0, test_info("b")));
        stub_0x355a08(&mut cache, 64);
        assert!(cache.entries.capacity() >= 64);
        assert!(cache.entries.contains_key("u"));
    }

    #[test]
    fn min_buckets_0x355b30_rounds_to_prime() {
        assert_eq!(stub_0x355b30(0), 2);
        assert_eq!(stub_0x355b30(20), 23);
        assert!(stub_0x355b30(100) >= 101);
        assert_eq!(stub_0x355b30(20), stub_0x355b30(20));
    }

    #[test]
    fn rehash_0x355bc0_rebuilds_keeping_contents() {
        let mut cache = WebCache::new();
        cache.entries.insert("u".to_string(), (1, test_info("b")));
        cache.entries.insert("v".to_string(), (2, test_info("c")));
        stub_0x355bc0(&mut cache, 64);
        assert!(cache.entries.capacity() >= 64);
        assert_eq!(cache.entries["u"].0, 1);
        assert_eq!(cache.entries["v"].1.body.as_ref().as_str(), "c");
    }

    #[test]
    fn place_in_bucket_0x355bec_relinks_present_key() {
        let mut cache = WebCache::new();
        cache.entries.insert("u".to_string(), (5, test_info("b")));
        assert!(stub_0x355bec(&mut cache, "u"));
        assert_eq!(cache.entries["u"].0, 5);
        assert!(!stub_0x355bec(&mut cache, "missing"));
    }

    #[test]
    fn construct_0x355c44_allocates_key_shell() {
        assert_eq!(stub_0x355c44("u"), "u");
    }

    #[test]
    fn delete_buckets_0x3560f8_frees_nodes_and_array() {
        let mut cache = WebCache::new();
        cache.entries.insert("u".to_string(), (0, test_info("b")));
        cache.entries.insert("v".to_string(), (0, test_info("c")));
        stub_0x3560f8(&mut cache);
        assert!(cache.entries.is_empty());
        assert_eq!(cache.entries.capacity(), 0);
    }

    #[test]
    fn clear_0x356130_drops_nodes_keeps_array() {
        let mut cache = WebCache::new();
        cache.entries.insert("u".to_string(), (0, test_info("b")));
        stub_0x355a08(&mut cache, 64);
        let before = cache.entries.capacity();
        assert!(before >= 64);
        stub_0x356130(&mut cache);
        assert!(cache.entries.is_empty());
        assert_eq!(cache.entries.capacity(), before);
    }

    #[test]
    fn table_ctor_0x3562bc_sizes_fresh_table() {
        let cache = stub_0x3562bc(8);
        assert!(cache.entries.is_empty());
        assert!(cache.entries.capacity() >= 8);
    }
}
