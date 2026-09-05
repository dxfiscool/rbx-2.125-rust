//! Auto-generated skeletons for rbx-network — RakNet|Network|Replicat|Socket|Upnp|HTTP EA-sorted asc (filtered 6232 ci / 5273 cs, 699 remaining after batch)
//! Filter: RakNet|Network|Replicat|Socket|Upnp|HTTP -> 5273 funcs (cs), 6232 (ci), 799 remaining before batch; batch EA-sorted asc next 100 filtered
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x2fee5c..0x3480fc | existing 18369 -> 18469 total (filtered EA-sorted asc, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
/// HTTP completion: `boost::function<void(string*, exception*)>` (IDA 0x3168b0: response body + error).
pub type HttpDoneCallback = SharedPtr<dyn Fn(Option<String>, Option<String>)>;

/// RBX::Http client init state (IDA 0x3165b0: default API selector, -1 = Uninitialized).
#[derive(Debug)]
pub struct HttpInit {
    pub default_api: i32,
    pub pool_threads: u32,
}

impl HttpInit {
    /// Fresh client before init (defaultApi = Uninitialized).
    pub fn uninitialized() -> Self {
        HttpInit { default_api: -1, pool_threads: 0 }
    }
}

/// RBX::Http response locks (IDA 0x31e45c/0x31e558: roblox/cdn statics, nullable during teardown).
#[derive(Clone, Debug, Default)]
pub struct HttpLocks {
    pub roblox: Option<SharedPtr<QueueMutex>>,
    pub cdn: Option<SharedPtr<QueueMutex>>,
}

/// RBX::Http::MutexGuard (IDA 0x318100: unlocks on drop; no state of its own).
#[derive(Debug, Default)]
pub struct HttpMutexGuard;

/// LuaWebService callback bind tuple (IDA 0x347518: result + url + variant/string callbacks; the weak service retain is caller-held).
#[derive(Clone)]
pub struct LuaCallbackBind {
    pub result: HttpRequestResult,
    pub url: String,
    pub on_variants: SharedPtr<dyn Fn()>,
    pub on_string: SharedPtr<dyn Fn(String)>,
}

/// AsyncHttpQueue::AsyncRetryTask (IDA 0x301b4c: 12-byte deque element).
#[derive(Clone, Copy, Debug, Default)]
pub struct AsyncRetryTask {
    pub words: [u32; 3],
}

/// AsyncHttpQueue retry deque: `std::deque<AsyncRetryTask>` (IDA 0x301b4c et al.; 504-byte chunks folded into VecDeque growth).
pub type RetryDeque = std::collections::VecDeque<AsyncRetryTask>;

/// HttpQueueStatsItem instance (IDA 0x302418: Stats::Item with queue/process averages, slow count, name).
#[derive(Debug, Default)]
pub struct HttpQueueStatsItem {
    pub name: String,
    pub avg_queue_ms: f64,
    pub avg_process_ms: f64,
    pub slow_requests: i32,
}

/// Queue mutex: `boost::shared_ptr<mutex>` guarding AsyncHttpQueue requests (IDA 0x300d3c et al.).
pub type QueueMutex = std::sync::Mutex<()>;

/// Queue-worker callback: `boost::function<void(shared_ptr<mutex>)>` (IDA 0x300d3c et al.; shared ownership models value semantics).
pub type QueueWorkerCallback = SharedPtr<dyn Fn(SharedPtr<QueueMutex>)>;

/// Queue-worker bind triple (IDA 0x301770 list3 ctor: weak queue + request index; the arg<1> mutex placeholder carries no value).
#[derive(Clone)]
pub struct WorkerBind {
    pub queue: std::sync::Weak<crate::generated_164::AsyncHttpQueue>,
    pub index: usize,
}

/// AsyncHttpQueue request result code (IDA RBX::AsyncHttpQueue::RequestResult; passed through opaquely).
pub type HttpRequestResult = u32;

/// AsyncHttpQueue completion callback: `boost::function<void(RequestResult, istream*, shared_ptr<string const>)>` (IDA 0x2fee80 et al.; shared ownership models boost::function value semantics).
pub type HttpCallback = SharedPtr<dyn Fn(HttpRequestResult, &mut dyn std::io::Read, SharedPtr<String>)>;

/// AsyncHttpQueue::CallbackWrapper (IDA 0x2fee5c: 20 bytes — callback + tag at +16).
#[derive(Clone)]
pub struct CallbackWrapper {
    pub callback: HttpCallback,
    pub tag: i32,
}

/// AsyncHttpQueue::Request (IDA 0x2ff188: url + callbacks + priority/options).
#[derive(Clone)]
pub struct HttpRequest {
    pub url: String,
    pub callbacks: Vec<CallbackWrapper>,
    pub priority: i32,
    pub options: u32,
}

/// Bound callback invocation tuple (IDA 0x2ff588 list4 ctor: callback + instance + result + data).
#[derive(Clone)]
pub struct BoundCallback {
    pub callback: HttpCallback,
    pub instance: usize,
    pub result: HttpRequestResult,
    pub data: SharedPtr<String>,
}

/// DataModel callback: `boost::function<void(DataModel*)>` (IDA 0x2fff80 et al.; shared ownership models value semantics).
pub type DataModelCallback = SharedPtr<dyn Fn(usize)>;

/// Owned payload destroyed by `sp_counted_impl_p<RBX::Http>::dispose` (IDA 0x2ffd3c: strings + header map).
#[derive(Default)]
pub struct HttpDispose {
    pub headers: std::collections::HashMap<String, String>,
    pub fields: Vec<String>,
}


// 0x2fee5c — __ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)")]
pub fn stub_2fee5c(n: usize) -> Vec<CallbackWrapper> { // IDA 0x2fee5c: checked 20-byte array alloc (bad_alloc at 0xCCCCCCD+); maps to reserved Vec capacity.
    if n >= 0xCCCCCCD {
        panic!("_M_allocate: bad_alloc");
    }
    Vec::with_capacity(n)
}

// 0x2fee80 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEaSERKS9_
// demangled: boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>::operator=(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)> const&)
#[doc(alias = "boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>::operator=(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)> const&)")]
pub fn stub_2fee80(dst: &mut HttpCallback, src: &HttpCallback) { // IDA 0x2fee80: copy-assign via temp + swap (assign_to_own/swap/clear); observable: dst becomes src.
    *dst = src.clone();
}

// 0x2fef44 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE4swapERS8_
// demangled: boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>&)
#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
pub fn stub_2fef44(a: &mut HttpCallback, b: &mut HttpCallback) { // IDA 0x2fef44: three-way move-assign swap with self-check; observable: swap.
    std::mem::swap(a, b);
}

// 0x2ff020 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE11move_assignERS8_
// demangled: boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>&)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
pub fn stub_2ff020(dst: &mut Option<HttpCallback>, src: &mut Option<HttpCallback>) { // IDA 0x2ff020: move-assign (small-object copy or heap-clone dispatch); source cleared; observable: take.
    *dst = src.take();
}

// 0x2ff128 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
// demangled: RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
pub fn stub_2ff128(dst: &mut [CallbackWrapper], src: &[CallbackWrapper]) { // IDA 0x2ff128: backward elementwise copy (overlap-safe direction preserved; per-element assign_to_own folded into clone).
    for (d, s) in dst.iter_mut().rev().zip(src.iter().rev()) {
        d.clone_from(s);
    }
}

// 0x2ff188 — __ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE14_M_create_nodeERKS2_
// demangled: std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&)
// type: int __fastcall(int, int, int, int, std::string *, int, int, int, int, int)
#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&)")]
pub fn stub_2ff188(url: String, callbacks: Vec<CallbackWrapper>, priority: i32, options: u32) -> HttpRequest { // IDA 0x2ff188: list node alloc (0x2C) + url/callbacks/priority/options/shared-count init.
    HttpRequest { url, callbacks, priority, options }
}

// 0x2ff2d4 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2ERKS4_
// demangled: std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
pub fn stub_2ff2d4(src: &[CallbackWrapper]) -> Vec<CallbackWrapper> { // IDA 0x2ff2d4: range vector ctor (per-element assign_to_own folded into clone).
    src.to_vec()
}

// 0x2ff43c — __ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2EmRKS3_
// demangled: std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)")]
pub fn stub_2ff43c(n: usize) -> Vec<CallbackWrapper> { // IDA 0x2ff43c: vector-base ctor with n-element storage (null triple when 0).
    Vec::with_capacity(n)
}

// 0x2ff470 — __ZN5boost3_bi5list4INS_3argILi1EEENS0_5valueIPN3RBX8InstanceEEENS4_INS5_14AsyncHttpQueue13RequestResultEEENS4_INS_10shared_ptrISsEEEEEclIPFvNS9_15CallbackWrapperES7_SA_SD_ENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>::operator()<void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&>>(boost::_bi::type<void>,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>) &,boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&> &,int)
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>::operator()<void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&>>(boost::_bi::type<void>,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>) &,boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&> &,int)")]
pub fn stub_2ff470(target: &mut dyn FnMut(&CallbackWrapper, usize, HttpRequestResult, SharedPtr<String>), wrapper: &CallbackWrapper, instance: usize, result: HttpRequestResult, data: SharedPtr<String>) { // IDA 0x2ff470: bind-list invocation — calls the bound target with stored (instance, result, data) plus the wrapper argument.
    target(wrapper, instance, result, data);
}

// 0x2ff588 — __ZN5boost3_bi5list4INS_3argILi1EEENS0_5valueIPN3RBX8InstanceEEENS4_INS5_14AsyncHttpQueue13RequestResultEEENS4_INS_10shared_ptrISsEEEEEC2ES3_S8_SB_SE_
// demangled: boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>::list4(boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>)
#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>::list4(boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>)")]
pub fn stub_2ff588(callback: HttpCallback, instance: usize, result: HttpRequestResult, data: SharedPtr<String>) -> BoundCallback { // IDA 0x2ff588: bind-tuple ctor (shared-count copies folded into Arc clones).
    BoundCallback { callback, instance, result, data }
}

// 0x2ff674 — __ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
// demangled: std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>)
#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>)")]
pub fn stub_2ff674(requests: &mut Vec<HttpRequest>, index: usize) -> HttpRequest { // IDA 0x2ff674: list node erase (unhook + destroy); Vec removal preserves order likewise.
    requests.remove(index)
}

// 0x2ff758 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS2_S4_EEEEPS2_mT_SC_
// demangled: RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>)
// type: char *__fastcall(int, unsigned int, int, int)
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>)")]
pub fn stub_2ff758(src: &[CallbackWrapper]) -> Vec<CallbackWrapper> { // IDA 0x2ff758: allocate + copy range (per-element assign_to_own folded into clone).
    src.to_vec()
}

// 0x2ff8c0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
// demangled: RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)
// type: int(void)
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
pub fn stub_2ff8c0(dst: &mut [CallbackWrapper], src: &[CallbackWrapper]) { // IDA 0x2ff8c0: forward elementwise copy.
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        d.clone_from(s);
    }
}

// 0x2ff91c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX14AsyncHttpQueue15CallbackWrapperEPS5_EET0_T_SA_S9_
// demangled: RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*)
// type: int(void)
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*)")]
pub fn stub_2ff91c(dst: &mut [CallbackWrapper], src: &[CallbackWrapper]) { // IDA 0x2ff91c: forward elementwise copy (const source).
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        d.clone_from(s);
    }
}

// 0x2ff978 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EED2Ev
// demangled: std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::~vector()
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::~vector()")]
pub fn stub_2ff978(v: Vec<CallbackWrapper>) { // IDA 0x2ff978: vector dtor (per-element clear + dealloc); drop.
    drop(v);
}

// 0x2ffa44 — __ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE14_M_create_nodeERKS2_
// demangled: std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_create_node(RBX::AsyncHttpQueue::FailedUrl const&)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_create_node(RBX::AsyncHttpQueue::FailedUrl const&)")]
pub fn stub_2ffa44(url: String, time: f64) -> crate::generated_164::FailedUrl { // IDA 0x2ffa44: FailedUrl node alloc (url + timestamp); reuses the sibling-owned type.
    crate::generated_164::FailedUrl { url, time }
}

// 0x2ffb24 — __ZN5boost10shared_ptrIN3RBX4HttpEEC2IS2_EEPT_
// demangled: boost::shared_ptr<RBX::Http>::shared_ptr<RBX::Http>(RBX::Http *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Http>::shared_ptr<RBX::Http>(RBX::Http *)")]
pub fn stub_2ffb24<T>(value: T) -> SharedPtr<T> { // IDA 0x2ffb24: shared_ptr<Http> aliasing ctor (temp count released); Arc ownership.
    SharedPtr::new(value)
}

// 0x2ffbfc — __ZN5boost6detail12shared_countC2IN3RBX4HttpEEEPT_
// demangled: boost::detail::shared_count::shared_count<RBX::Http>(RBX::Http *)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Http>(RBX::Http *)")]
pub fn stub_2ffbfc<T>(value: T) -> SharedPtr<T> { // IDA 0x2ffbfc: shared_count ctor (control block, use/weak 1); Arc alloc.
    SharedPtr::new(value)
}

// 0x2ffd34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEED1Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::Http>::~sp_counted_impl_p()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::~sp_counted_impl_p()")]
pub fn stub_2ffd34<T>(v: SharedPtr<T>) { // IDA 0x2ffd34: counted-impl dtor (empty body; base releases).
    drop(v);
}

// 0x2ffd38 — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEED0Ev
// demangled: boost::detail::sp_counted_impl_p<RBX::Http>::~sp_counted_impl_p()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::~sp_counted_impl_p()")]
pub fn stub_2ffd38<T>(v: SharedPtr<T>) { // IDA 0x2ffd38: deleting-destructor thunk (operator delete); drop.
    drop(v);
}

// 0x2ffd3c — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE7disposeEv
// demangled: boost::detail::sp_counted_impl_p<RBX::Http>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::dispose(void)")]
pub fn stub_2ffd3c(data: HttpDispose) { // IDA 0x2ffd3c: dispose the Http payload (strings + header map) then free; drop.
    drop(data);
}

// 0x2ffe10 — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_p<RBX::Http>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::get_deleter(std::type_info const&)")]
pub fn stub_2ffe10() -> Option<SharedPtr<()>> { // IDA 0x2ffe10: get_deleter on plain ownership → null.
    None
}

// 0x2ffe14 — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_p<RBX::Http>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::get_untyped_deleter(void)")]
pub fn stub_2ffe14() -> Option<SharedPtr<()>> { // IDA 0x2ffe14: get_untyped_deleter on plain ownership → null.
    None
}

// 0x2ffe1c — __ZN5boost10shared_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// demangled: boost::shared_ptr<RBX::AsyncHttpQueue>::shared_ptr<RBX::AsyncHttpQueue>(boost::weak_ptr<RBX::AsyncHttpQueue> const&,boost::detail::sp_nothrow_tag)
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpQueue>::shared_ptr<RBX::AsyncHttpQueue>(rbx_core::WeakPtr<RBX::AsyncHttpQueue> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_2ffe1c(dst: &mut Option<SharedPtr<crate::generated_164::AsyncHttpQueue>>, src: &Option<SharedPtr<crate::generated_164::AsyncHttpQueue>>) { // IDA 0x2ffe1c: copy with spinlocked retain (maps to atomic Arc clone); empty source clears.
    *dst = src.clone();
}

// 0x2fff80 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS7_5list3INS7_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS7_5list3INS7_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2fff80(f: &DataModelCallback) -> DataModelCallback { // IDA 0x2fff80: function<DataModel*> ctor from the bind triple (shared state copied); clone.
    f.clone()
}

// 0x3000d8 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS6_5list3INS6_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS6_5list3INS6_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_3000d8(f: &DataModelCallback) { // IDA 0x3000d8: function1 ctor from the bind triple (copies shared state; the int return is a decompiler artifact).
    let _ = f.clone();
}

// 0x300230 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS6_5list3INS6_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEEvT_
// demangled: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>)")]
pub fn stub_300230(dst: &mut DataModelCallback, src: &DataModelCallback) { // IDA 0x300230: assign_to the bind triple (shared state copied).
    *dst = src.clone();
}

// 0x30039c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_30039c(op: u32) -> Option<&'static str> { // IDA 0x30039c: functor manager — non-4 → base manager; 4 → bind_t typeinfo (mangled name preserved verbatim from IDA).
    if op == 4 {
        Some("N5boost3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES5_S9_ENS0_5list3INS0_5valueISB_EENSF_IS5_EENSF_IS9_EEEEEE")
    } else {
        None
    }
}

// 0x3003b8 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESP_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn stub_3003b8(f: &DataModelCallback, arg: usize) { // IDA 0x3003b8: invoker thunk — calls the bound target with the stored triple plus the call argument (verified against disasm).
    f(arg);
}

// 0x3003d4 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_SG_ENS8_5list3INS8_5valueISI_EENSM_ISC_EENSM_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_3003d4(dst: &mut DataModelCallback, src: &DataModelCallback) -> bool { // IDA 0x3003d4: vtable assign_to the bind triple; returns 1.
    *dst = src.clone();
    true
}

// 0x300530 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_SG_ENS8_5list3INS8_5valueISI_EENSM_ISC_EENSM_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, void *, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_300530(dst: &mut DataModelCallback, src: &DataModelCallback) -> bool { // IDA 0x300530: vtable assign_to with heap functor (alloc folded into Arc); returns 1.
    *dst = src.clone();
    true
}

// 0x300688 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_SG_ENS8_5list3INS8_5valueISI_EENSM_ISC_EENSM_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// demangled: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// type: void __fastcall(int, const shared_count *, sp_counted_base ***, int, void *, int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_300688(f: &DataModelCallback) -> DataModelCallback { // IDA 0x300688: assign_functor (0x20 heap alloc + copy); Arc clone.
    f.clone()
}

// 0x300798 — __ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEclIPFvSC_S6_SA_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>::operator()<void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>) &,boost::_bi::list1<RBX::DataModel *&> &,int)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::operator()<void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
pub fn stub_300798(target: &mut dyn FnMut(HttpCallback, HttpRequestResult, SharedPtr<String>), cb: HttpCallback, result: HttpRequestResult, data: SharedPtr<String>, _arg: usize) { // IDA 0x300798: bind-list invocation (the DataModel* call argument is discarded; stored triple forwarded).
    target(cb, result, data);
}

// 0x3008a4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_3008a4(dst: &mut Option<DataModelCallback>, src: &mut Option<DataModelCallback>, op: u32) -> Option<&'static str> { // IDA 0x3008a4: functor manager — 0 clone-alloc, 1 move-take, 2 destroy+free, 3 type-name check, default typeinfo report.
    match op {
        0 => {
            *dst = src.clone();
            None
        }
        1 => {
            *dst = src.take();
            None
        }
        2 => {
            *dst = None;
            None
        }
        _ => Some("N5boost3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES5_S9_ENS0_5list3INS0_5valueISB_EENSF_IS5_EENSF_IS9_EEEEEE"),
    }
}

// 0x300a60 — __ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_
// demangled: boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>::list3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::list3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")]
pub fn stub_300a60(cb: HttpCallback, result: HttpRequestResult, data: SharedPtr<String>) -> (HttpCallback, HttpRequestResult, SharedPtr<String>) { // IDA 0x300a60: list3 bind-tuple ctor (nested storage3 call folded in).
    (cb, result, data)
}

// 0x300b6c — __ZN5boost3_bi8storage3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_
// demangled: boost::_bi::storage3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>::storage3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::storage3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")]
pub fn stub_300b6c(cb: HttpCallback, result: HttpRequestResult, data: SharedPtr<String>) -> (HttpCallback, HttpRequestResult, SharedPtr<String>) { // IDA 0x300b6c: storage3 ctor (nested storage2 + shared-count copy folded into Arc clones).
    (cb, result, data)
}

// 0x300c6c — __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EEEC2ESD_SE_
// demangled: boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>)")]
pub fn stub_300c6c(cb: HttpCallback, result: HttpRequestResult) -> (HttpCallback, HttpRequestResult) { // IDA 0x300c6c: storage2 ctor (pair copy).
    (cb, result)
}

// 0x300d3c — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_300d3c(f: &QueueWorkerCallback) { // IDA 0x300d3c: function<mutex> ctor from the weak-queue bind triple (weak retain folded into Weak clone; the int return is a decompiler artifact).
    let _ = f.clone();
}

// 0x300e68 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_300e68(dst: &mut QueueWorkerCallback, src: &QueueWorkerCallback) { // IDA 0x300e68: function1 assign_to the bind triple (shared state copied).
    *dst = src.clone();
}

// 0x300f98 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_
// demangled: void boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>)
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>)")]
pub fn stub_300f98(dst: &mut QueueWorkerCallback, src: &QueueWorkerCallback) { // IDA 0x300f98: assign_to the bind triple (vtable dispatch folded in).
    *dst = src.clone();
}

// 0x3010d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_3010d8(op: u32) -> Option<&'static str> { // IDA 0x3010d8: functor manager — non-4 → base manager; 4 → bind_t typeinfo (mangled name preserved verbatim from IDA).
    if op == 4 {
        Some("N5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS4_7RequestEENS_10shared_ptrINS3_5mutexEEEENS0_5list3INS0_5valueIS5_EENSF_IS8_EENS_3argILi1EEEEEEE")
    } else {
        None
    }
}

// 0x3010f4 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEvSE_E6invokeERNS1_15function_bufferESE_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::mutex>)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
pub fn stub_3010f4(f: &QueueWorkerCallback, m: SharedPtr<QueueMutex>) { // IDA 0x3010f4: invoker thunk — calls the bound worker with the stored triple plus the mutex argument (verified against disasm).
    f(m);
}

// 0x30110c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_30110c(dst: &mut QueueWorkerCallback, src: &QueueWorkerCallback) -> bool { // IDA 0x30110c: vtable assign_to the bind triple; returns 1.
    *dst = src.clone();
    true
}

// 0x301238 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_301238(dst: &mut QueueWorkerCallback, src: &QueueWorkerCallback) -> bool { // IDA 0x301238: vtable assign_to with heap functor (alloc folded into Arc); returns 1.
    *dst = src.clone();
    true
}

// 0x301360 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// demangled: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_301360(f: &QueueWorkerCallback) -> QueueWorkerCallback { // IDA 0x301360: assign_functor (0x10 heap alloc + weak-retain copy); Arc clone.
    f.clone()
}

// 0x301478 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEclIPFvS6_SA_NS_10shared_ptrINS4_5mutexEEEENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list1<boost::shared_ptr<RBX::mutex>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>) &,boost::_bi::list1<boost::shared_ptr<RBX::mutex>&> &,int)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::operator()<void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex>&> &,int)")]
pub fn stub_301478(target: &mut dyn FnMut(std::sync::Weak<crate::generated_164::AsyncHttpQueue>, usize, SharedPtr<QueueMutex>), queue: std::sync::Weak<crate::generated_164::AsyncHttpQueue>, index: usize, mutex: SharedPtr<QueueMutex>) { // IDA 0x301478: bind-list invocation (weak retain/release folded into Weak clone; stored triple forwarded with the mutex argument).
    target(queue, index, mutex);
}

// 0x3015d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_3015d8(dst: &mut Option<WorkerBind>, src: &mut Option<WorkerBind>, op: u32) -> Option<&'static str> { // IDA 0x3015d8: functor manager — 0 clone-alloc (weak retain folded into Weak clone), 1 move-take, 2 destroy, 3 type-name check, default typeinfo report.
    match op {
        0 => {
            *dst = src.clone();
            None
        }
        1 => {
            *dst = src.take();
            None
        }
        2 => {
            *dst = None;
            None
        }
        _ => Some("N5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS4_7RequestEENS_10shared_ptrINS3_5mutexEEEENS0_5list3INS0_5valueIS5_EENSF_IS8_EENS_3argILi1EEEEEEE"),
    }
}

// 0x301770 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_
// demangled: boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::list3(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
pub fn stub_301770(queue: std::sync::Weak<crate::generated_164::AsyncHttpQueue>, index: usize) -> WorkerBind { // IDA 0x301770: list3 bind-tuple ctor (weak retain folded into Weak clone; arg<1> placeholder carries no value).
    WorkerBind { queue, index }
}

// 0x30188c — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_
// demangled: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
pub fn stub_30188c(queue: std::sync::Weak<crate::generated_164::AsyncHttpQueue>, index: usize) -> WorkerBind { // IDA 0x30188c: storage3 ctor (nested storage2 + weak retain folded in).
    WorkerBind { queue, index }
}

// 0x3019a8 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEEEC2ES7_SB_
// demangled: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>)")]
pub fn stub_3019a8(queue: std::sync::Weak<crate::generated_164::AsyncHttpQueue>, index: usize) -> (std::sync::Weak<crate::generated_164::AsyncHttpQueue>, usize) { // IDA 0x3019a8: storage2 pair ctor (weak retain folded into Weak clone).
    (queue, index)
}

// 0x301afc — __ZN5boost8weak_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// demangled: boost::weak_ptr<RBX::AsyncHttpQueue>::weak_ptr<RBX::AsyncHttpQueue>(boost::shared_ptr<RBX::AsyncHttpQueue> const&,boost::detail::sp_enable_if_convertible<RBX::AsyncHttpQueue,RBX::AsyncHttpQueue>::type)
#[doc(alias = "rbx_core::WeakPtr<RBX::AsyncHttpQueue>::weak_ptr<RBX::AsyncHttpQueue>(rbx_core::SharedPtr<RBX::AsyncHttpQueue> const&,boost::detail::sp_enable_if_convertible<RBX::AsyncHttpQueue,RBX::AsyncHttpQueue>::type)")]
pub fn stub_301afc(src: &std::sync::Weak<crate::generated_164::AsyncHttpQueue>) -> std::sync::Weak<crate::generated_164::AsyncHttpQueue> { // IDA 0x301afc: weak_ptr copy with spinlocked weak-retain (maps to atomic Weak clone).
    src.clone()
}

// 0x301b4c — __ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE9pop_frontEv
// demangled: std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void)
// type: int(void)
#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void)")]
pub fn stub_301b4c(q: &mut RetryDeque) { // IDA 0x301b4c: deque pop_front (chunk-boundary dance folded into VecDeque).
    q.pop_front();
}

// 0x301b80 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_allocate_mapEm
// demangled: std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long)
// type: int(void)
#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long)")]
pub fn stub_301b80(n: usize) -> Vec<Vec<AsyncRetryTask>> { // IDA 0x301b80: deque map alloc (bad_alloc at 0x40000000+); chunk-pointer array.
    if n >= 0x40000000 {
        panic!("_M_allocate_map: bad_alloc");
    }
    Vec::with_capacity(n)
}

// 0x301b98 — __ZNSt10_List_baseIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_clearEv
// demangled: std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void)
// type: int __fastcall(int, int, int, int, int, std::string *, int, int, int, int)
#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void)")]
pub fn stub_301b98(q: &mut Vec<HttpRequest>) { // IDA 0x301b98: list clear (per-node release + destroy folded into drop).
    q.clear();
}

// 0x301f74 — __ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EEC2ERKS4_
// demangled: std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>> const&)
// type: int __fastcall(int)
#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>> const&)")]
pub fn stub_301f74(src: &RetryDeque) -> RetryDeque { // IDA 0x301f74: deque range ctor (map sizing + elementwise copy folded into clone).
    src.clone()
}

// 0x302028 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EED2Ev
// demangled: std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::~_Deque_base()
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::~_Deque_base()")]
pub fn stub_302028(q: RetryDeque) { // IDA 0x302028: deque-base dtor (chunk deletes folded into drop).
    drop(q);
}

// 0x302054 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE17_M_initialize_mapEm
// demangled: std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long)")]
pub fn stub_302054(q: &mut RetryDeque, n: usize) { // IDA 0x302054: deque map init (count/centering folded into VecDeque growth).
    q.reserve(n);
}

// 0x3021d4 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_create_nodesEPPS2_S6_
// demangled: std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**)
// type: void __fastcall(int, _DWORD *, unsigned int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**)")]
pub fn stub_3021d4(n: usize) -> Vec<Vec<AsyncRetryTask>> { // IDA 0x3021d4: deque chunk-node allocs (0x1F8-byte nodes); capacity array.
    (0..n).map(|_| Vec::with_capacity(42)).collect()
}

// 0x302324 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18HttpQueueStatsItemEPNS_14AsyncHttpQueueEPS1_EEN5boost10shared_ptrIT_EET0_T1_
// demangled: boost::shared_ptr<RBX::HttpQueueStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::HttpQueueStatsItem,RBX::AsyncHttpQueue *,RBX::Instance*>(RBX::AsyncHttpQueue *,RBX::Instance*)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::HttpQueueStatsItem,RBX::AsyncHttpQueue *,RBX::Instance*>(RBX::AsyncHttpQueue *,RBX::Instance*)")]
pub fn stub_302324(item: HttpQueueStatsItem) -> SharedPtr<HttpQueueStatsItem> { // IDA 0x302324: Creatable::create — shared_ptr with Creatable deleter; Arc ownership.
    SharedPtr::new(item)
}

// 0x3023dc — __ZN3RBX18HttpQueueStatsItem4initEv
// demangled: RBX::HttpQueueStatsItem::init(void)
// type: _DWORD __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "RBX::HttpQueueStatsItem::init(void)")]
pub fn stub_3023dc(slots: &mut [i32; 3], create_child: &mut dyn FnMut(&str) -> i32) -> i32 { // IDA 0x3023dc: create "Average time in queue", "Average process time", "Num slow requests"; returns the last.
    slots[0] = create_child("Average time in queue");
    slots[1] = create_child("Average process time");
    slots[2] = create_child("Num slow requests");
    slots[2]
}

// 0x302418 — __ZN3RBX18HttpQueueStatsItemC2EPNS_14AsyncHttpQueueEPNS_8InstanceE
// demangled: RBX::HttpQueueStatsItem::HttpQueueStatsItem(RBX::AsyncHttpQueue *,RBX::Instance *)
// type: _DWORD __fastcall(RBX::HttpQueueStatsItem *__hidden this, RBX::AsyncHttpQueue *, RBX::Instance *)
#[doc(alias = "RBX::HttpQueueStatsItem::HttpQueueStatsItem(RBX::AsyncHttpQueue *,RBX::Instance *)")]
pub fn stub_302418(queue_id: u32) -> HttpQueueStatsItem { // IDA 0x302418: Instance base + Stats::Item descriptor, name "HttpQueue_<id>" (refcount dance folded into String).
    HttpQueueStatsItem { name: format!("HttpQueue_{}", queue_id), ..Default::default() }
}

// 0x30266c — __ZN3RBX18HttpQueueStatsItemD1Ev
// demangled: RBX::HttpQueueStatsItem::~HttpQueueStatsItem()
// type: void __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
pub fn stub_30266c(item: HttpQueueStatsItem) { // IDA 0x30266c: destructor D1 (vtable resets + string/Instance teardown folded into drop).
    drop(item);
}

// 0x3026a8 — __ZN3RBX18HttpQueueStatsItemD0Ev
// demangled: RBX::HttpQueueStatsItem::~HttpQueueStatsItem()
// type: void __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
pub fn stub_3026a8(item: HttpQueueStatsItem) { // IDA 0x3026a8: destructor D0 (teardown + operator delete folded into drop).
    drop(item);
}

// 0x30277c — __ZN3RBX18HttpQueueStatsItem6updateEv
// demangled: RBX::HttpQueueStatsItem::update(void)
// type: _DWORD __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "RBX::HttpQueueStatsItem::update(void)")]
pub fn stub_30277c(avg_queue_ms: f64, avg_process_ms: f64, slow_requests: i32, format_msec: &mut dyn FnMut(f64) -> String, format_int: &mut dyn FnMut(i32) -> String) -> (String, String, String) { // IDA 0x30277c: format the two averages ("%.4f msec") and the slow count.
    (format_msec(avg_queue_ms), format_msec(avg_process_ms), format_int(slow_requests))
}

// 0x3027d0 — __ZThn32_N3RBX18HttpQueueStatsItemD1Ev
// demangled: non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()
// type: void __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
pub fn stub_3027d0(item: HttpQueueStatsItem) { // IDA 0x3027d0: non-virtual thunk (this-32 adjustment folded in); destroy.
    drop(item);
}

// 0x302810 — __ZThn32_N3RBX18HttpQueueStatsItemD0Ev
// demangled: non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()
// type: void __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
pub fn stub_302810(item: HttpQueueStatsItem) { // IDA 0x302810: non-virtual thunk (this-32 + delete folded in); destroy.
    drop(item);
}

// 0x3028e8 — __ZThn36_N3RBX18HttpQueueStatsItemD1Ev
// demangled: non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()
// type: void __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
pub fn stub_3028e8(item: HttpQueueStatsItem) { // IDA 0x3028e8: non-virtual thunk (this-36 adjustment folded in); destroy.
    drop(item);
}

// 0x302928 — __ZThn36_N3RBX18HttpQueueStatsItemD0Ev
// demangled: non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()
// type: void __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
pub fn stub_302928(item: HttpQueueStatsItem) { // IDA 0x302928: non-virtual thunk (this-36 + delete folded in); destroy.
    drop(item);
}

// 0x3029fc — __ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::HttpQueueStatsItem>::shared_ptr<RBX::HttpQueueStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem>::shared_ptr<RBX::HttpQueueStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_3029fc<T>(value: T) -> SharedPtr<T> { // IDA 0x3029fc: shared_ptr<StatsItem> ctor with Creatable deleter (+ accept_owner latch by caller); Arc ownership.
    SharedPtr::new(value)
}

// 0x302ac4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18HttpQueueStatsItemES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpQueueStatsItem,RBX::HttpQueueStatsItem>(boost::shared_ptr<RBX::HttpQueueStatsItem> const*,RBX::HttpQueueStatsItem *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpQueueStatsItem,RBX::HttpQueueStatsItem>(rbx_core::SharedPtr<RBX::HttpQueueStatsItem> const*,RBX::HttpQueueStatsItem *)const")]
pub fn stub_302ac4<T>(slot: &mut std::sync::Weak<T>, value: &SharedPtr<T>) { // IDA 0x302ac4: if the described-base weak expired, latch the new weak owner.
    if slot.upgrade().is_none() {
        *slot = SharedPtr::downgrade(value);
    }
}

// 0x302bac — __ZN5boost6detail12shared_countC2IPN3RBX18HttpQueueStatsItemENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_302bac<T>(value: T) -> SharedPtr<T> { // IDA 0x302bac: shared_count ctor for StatsItem+Deleter (control block, use/weak 1); Arc alloc.
    SharedPtr::new(value)
}

// 0x302cb4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_302cb4<T>(v: SharedPtr<T>) { // IDA 0x302cb4: counted-impl dtor (empty body; base releases).
    drop(v);
}

// 0x302cb8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_302cb8<T>(v: SharedPtr<T>) { // IDA 0x302cb8: deleting-destructor thunk (operator delete); drop.
    drop(v);
}

// 0x302cbc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_302cbc<T>(value: SharedPtr<T>, predelete: &mut dyn FnMut()) { // IDA 0x302cbc: Instance::predelete, then destroy via the Deleter; drop covers both.
    predelete();
    drop(value);
}

// 0x302cdc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_302cdc(type_name: &str) -> bool { // IDA 0x302cdc: deleter query — non-null iff the requested type is the Creatable deleter (name preserved verbatim).
    type_name == "N3RBX9CreatableINS_8InstanceEE7DeleterE"
}

// 0x302cf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_302cf4<T>(_v: &SharedPtr<T>) -> bool { // IDA 0x302cf4: get_untyped_deleter (unconditional deleter address → non-null).
    true
}

// 0x302cf8 — __ZNSt10_List_baseIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE8_M_clearEv
// demangled: std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void)
// type: int(void)
#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void)")]
pub fn stub_302cf8(q: &mut Vec<crate::generated_164::FailedUrl>) { // IDA 0x302cf8: FailedUrl list clear; reuses the sibling-owned type.
    q.clear();
}

// 0x316590 — __ZN3RBX4Http21getRobloxResponceLockEv
// demangled: RBX::Http::getRobloxResponceLock(void)
// type: _DWORD __fastcall(RBX::Http *__hidden this)
#[doc(alias = "RBX::Http::getRobloxResponceLock(void)")]
pub fn stub_316590(locks: &HttpLocks) -> Option<SharedPtr<QueueMutex>> { // IDA 0x316590: returns the static roblox response lock (null during teardown).
    locks.roblox.clone()
}

// 0x3165a0 — __ZN3RBX4Http18getCdnResponceLockEv
// demangled: RBX::Http::getCdnResponceLock(void)
// type: _DWORD __fastcall(RBX::Http *__hidden this)
#[doc(alias = "RBX::Http::getCdnResponceLock(void)")]
pub fn stub_3165a0(locks: &HttpLocks) -> Option<SharedPtr<QueueMutex>> { // IDA 0x3165a0: returns the static cdn response lock (null during teardown).
    locks.cdn.clone()
}

// 0x3165b0 — __ZN3RBX4Http4initENS0_3APIE
// demangled: RBX::Http::init(RBX::Http::API)
#[doc(alias = "RBX::Http::init(RBX::Http::API)")]
pub fn stub_3165b0(st: &mut HttpInit, api: i32) { // IDA 0x3165b0: ReleaseAssert when already initialized (Http.cpp:67); one-time 8-thread pool; latch default API.
    assert!(st.default_api == -1, "Http::defaultApi==Http::Uninitialized (Http.cpp:67)");
    if st.pool_threads == 0 {
        st.pool_threads = 8;
    }
    st.default_api = api;
}

// 0x316738 — __ZN3RBX4Http14ThrowIfFailureEbPKcS2_
// demangled: RBX::Http::ThrowIfFailure(bool,char const*,char const*)
// type: void __fastcall(RBX::Http *this, const char *, const char *, const char *)
#[doc(alias = "RBX::Http::ThrowIfFailure(bool,char const*,char const*)")]
pub fn stub_316738(ok: bool, what: &str, why: &str) { // IDA 0x316738: null self → throw std::runtime_error ("%s: %s"); maps to panic with the same message.
    if !ok {
        panic!("{}: {}", what, why);
    }
}

// 0x316814 — __ZN3RBX4Http15httpGetPostImplEbRSibRKSt3mapISsSsSt4lessISsESaISt4pairIKSsSsEEEbRSs
// demangled: RBX::Http::httpGetPostImpl(bool,std::istream &,bool,std::map<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>> const&,bool,std::string &)
#[doc(alias = "RBX::Http::httpGetPostImpl(bool,std::istream &,bool,std::map<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>> const&,bool,std::string &)")]
pub fn stub_316814(url: &str, is_post: bool, body: Option<&[u8]>, verify_trust: bool, init_trust: &mut dyn FnMut(), is_roblox_site: &mut dyn FnMut(&str) -> bool, fetch: &mut dyn FnMut(&str, bool, Option<&[u8]>) -> HttpRequestResult) -> HttpRequestResult { // IDA 0x316814: one-time trust init; off-site with verification → ThrowIfFailure; otherwise the Cocoa fetch.
    if verify_trust {
        init_trust();
        if !is_roblox_site(url) {
            stub_316738(false, url, "Trust check failed");
        }
    }
    fetch(url, is_post, body)
}

// 0x31688c — __ZN3RBX4Http4postERSibRSsb
// demangled: RBX::Http::post(std::istream &,bool,std::string &,bool)
// type: _DWORD __fastcall(RBX::Http *__hidden this, std::istream *, bool, std::string *, bool)
#[doc(alias = "RBX::Http::post(std::istream &,bool,std::string &,bool)")]
pub fn stub_31688c(url: &str, body: &[u8], post: &mut dyn FnMut(&str, &[u8]) -> HttpRequestResult) -> HttpRequestResult { // IDA 0x31688c: sync POST via httpGetPostImpl (post=true).
    post(url, body)
}

// 0x3168b0 — __ZN3RBX4Http3getEN5boost8functionIFvPSsPSt9exceptionEEEb
// demangled: RBX::Http::get(boost::function<void ()(std::string *,std::exception *)>,bool)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Http::get(boost::function<void ()(std::string *,std::exception *)>,bool)")]
pub fn stub_3168b0(url: &str, schedule_get: &mut dyn FnMut(&str)) { // IDA 0x3168b0: bind doGet and schedule on the Http pool (bind/thread-pool folded into the closure).
    schedule_get(url);
}

// 0x316f2c — __ZN3RBX4Http4postERKSsbN5boost8functionIFvPSsPSt9exceptionEEEb
// demangled: RBX::Http::post(std::string const&,bool,boost::function<void ()(std::string *,std::exception *)>,bool)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Http::post(std::string const&,bool,boost::function<void ()(std::string *,std::exception *)>,bool)")]
pub fn stub_316f2c(url: &str, data: &str, flag_a: bool, done: HttpDoneCallback, flag_b: bool, schedule_post: &mut dyn FnMut(&str, &str, bool, HttpDoneCallback, bool)) { // IDA 0x316f2c: bind doPost(url, data, flags, callback) and schedule on the Http pool (bind/thread-pool folded into the closure).
    schedule_post(url, data, flag_a, done, flag_b);
}

// 0x317570 — __ZN3RBX4Http4postEN5boost10shared_ptrISiEEbNS1_8functionIFvPSsPSt9exceptionEEEb
// demangled: RBX::Http::post(boost::shared_ptr<std::istream>,bool,boost::function<void ()(std::string *,std::exception *)>,bool)
#[doc(alias = "RBX::Http::post(rbx_core::SharedPtr<std::istream>,bool,boost::function<void ()(std::string *,std::exception *)>,bool)")]
pub fn stub_317570(url: &str, body: SharedPtr<String>, flag_a: bool, flag_b: bool, done: HttpDoneCallback, schedule_stream: &mut dyn FnMut(&str, SharedPtr<String>, bool, bool, HttpDoneCallback)) { // IDA 0x317570: bind doPostStream(url, body stream, flags, callback) and schedule on the Http pool.
    schedule_stream(url, body, flag_a, flag_b, done);
}

// 0x317de0 — __ZN3RBX4Http3getERSsb
// demangled: RBX::Http::get(std::string &,bool)
// type: _DWORD __fastcall(RBX::Http *__hidden this, std::string *, bool)
#[doc(alias = "RBX::Http::get(std::string &,bool)")]
pub fn stub_317de0(url: &str, count_cdn: bool, fetch: &mut dyn FnMut(&str) -> HttpRequestResult, count: &mut dyn FnMut()) -> HttpRequestResult { // IDA 0x317de0: timestamped sync GET via httpGetPostImpl; CDN success counted by caller flag.
    let r = fetch(url);
    if count_cdn {
        count();
    }
    r
}

// 0x3180dc — __ZN3RBX4Http12isRobloxSiteEPKc
// demangled: RBX::Http::isRobloxSite(char const*)
// type: _DWORD __fastcall(RBX::Http *__hidden this, const char *)
#[doc(alias = "RBX::Http::isRobloxSite(char const*)")]
pub fn stub_3180dc(host: &str, check: &mut dyn FnMut(&str) -> bool) -> bool { // IDA 0x3180dc: rbx_isRobloxSite predicate.
    check(host)
}

// 0x318100 — __ZN3RBX4Http10MutexGuardD1Ev
// demangled: RBX::Http::MutexGuard::~MutexGuard()
// type: void __fastcall(RBX::Http::MutexGuard *__hidden this)
#[doc(alias = "RBX::Http::MutexGuard::~MutexGuard()")]
pub fn stub_318100(_g: HttpMutexGuard) { // IDA 0x318100: MutexGuard dtor thunk (unlock folded into drop).
    drop(_g);
}

// 0x31e45c — __ZN3RBX4Http10MutexGuardD2Ev
// demangled: RBX::Http::MutexGuard::~MutexGuard()
// type: void __fastcall(RBX::Http::MutexGuard *__hidden this)
#[doc(alias = "RBX::Http::MutexGuard::~MutexGuard()")]
pub fn stub_31e45c(locks: &mut HttpLocks) { // IDA 0x31e45c: MutexGuard dtor — destroy both response locks and null them (drop).
    locks.roblox = None;
    locks.cdn = None;
}

// 0x31e558 — __ZN3RBX4Http10MutexGuardC2Ev
// demangled: RBX::Http::MutexGuard::MutexGuard(void)
// type: _DWORD __fastcall(RBX::Http::MutexGuard *__hidden this)
#[doc(alias = "RBX::Http::MutexGuard::MutexGuard(void)")]
pub fn stub_31e558(locks: &mut HttpLocks) { // IDA 0x31e558: MutexGuard ctor — create both response locks.
    locks.roblox = Some(SharedPtr::new(QueueMutex::new(())));
    locks.cdn = Some(SharedPtr::new(QueueMutex::new(())));
}

// 0x346168 — __ZN3RBX13LuaWebService11RawCallbackEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS1_8functionIFvSsEEES8_
// demangled: RBX::LuaWebService::RawCallback(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
#[doc(alias = "RBX::LuaWebService::RawCallback(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
pub fn stub_346168(ok: bool, dispatch: &mut dyn FnMut() -> bool, on_error: &mut dyn FnMut(&str)) { // IDA 0x346168: valid service + clean dispatch → return; else "RawCallback error" to the error callback.
    if ok && dispatch() {
        return;
    }
    on_error("RawCallback error");
}

// 0x346620 — __ZN3RBX13LuaWebService19asyncRequestNoCacheERKSsfN5boost8functionIFvNS3_10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIS1_S8_EEEEEEEENS_14AsyncHttpQueue9ResultJobE
// demangled: RBX::LuaWebService::asyncRequestNoCache(std::string const&,float,boost::function<void ()(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,RBX::AsyncHttpQueue::ResultJob)
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int)
#[doc(alias = "RBX::LuaWebService::asyncRequestNoCache(std::string const&,float,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,RBX::AsyncHttpQueue::ResultJob)")]
pub fn stub_346620(url: &str) -> crate::player::WebRequest {
    // IDA 0x346620: callback rebound through `Callback<...>` into `AsyncHttpQueue::asyncRequest` with caching disabled.
    crate::player::async_request_no_cache(url)
}

// 0x347178 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE5resetIS5_EEvPT_
// demangled: void boost::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::reset<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)
#[doc(alias = "void rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::reset<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")]
pub fn stub_347178<T>(dst: &mut SharedPtr<T>, value: T) { // IDA 0x347178: shared_ptr<AsyncHttpCache> reset (release old, take new); Arc swap.
    *dst = SharedPtr::new(value);
}

// 0x3471a4 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE5resetIS5_EEvPT_
// demangled: void boost::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::reset<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)
#[doc(alias = "void rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::reset<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")]
pub fn stub_3471a4<T>(dst: &mut SharedPtr<T>, value: T) { // IDA 0x3471a4: shared_ptr<CachedRawLuaWebServiceInfo cache> reset; Arc swap.
    *dst = SharedPtr::new(value);
}

// 0x3471d0 — __ZN3RBX13LuaWebService21TryRawDispatchRequestISsEEbPNS_14AsyncHttpCacheINS0_26CachedRawLuaWebServiceInfoELb1EEERKSsN5boost8functionIFvT_EEENS9_IFvSsEEE
// demangled: bool RBX::LuaWebService::TryRawDispatchRequest<std::string>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
// type: int __fastcall(int, int, int, int)
#[doc(alias = "bool RBX::LuaWebService::TryRawDispatchRequest<std::string>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
pub fn stub_3471d0(cached: Option<&str>, on_hit: &mut dyn FnMut(&str), on_empty: &mut dyn FnMut(&str)) -> bool { // IDA 0x3471d0: cache hit with body → hit callback, TRUE; empty body → empty callback ("Raw Request: string is empty"), TRUE; miss → FALSE.
    match cached {
        Some(body) if !body.is_empty() => {
            on_hit(body);
            true
        }
        Some(_) => {
            on_empty("Raw Request: string is empty");
            true
        }
        None => false,
    }
}

// 0x347518 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS2_10Reflection7VariantESaISB_EEEEEEENS7_IFvSsEEES4_NS_3argILi1EEESsSH_SJ_EENS_3_bi6bind_tIT_PFSO_T0_T1_T2_T3_T4_ENSM_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESV_SX_SY_SZ_S10_S11_
// demangled: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<boost::weak_ptr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>,boost::weak_ptr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>>(void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::weak_ptr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_347518(result: HttpRequestResult, url: String, on_variants: SharedPtr<dyn Fn()>, on_string: SharedPtr<dyn Fn(String)>) -> LuaCallbackBind { // IDA 0x347518: bind tuple ctor (weak service + result + url + callbacks; weak retain caller-held).
    LuaCallbackBind { result, url, on_variants, on_string }
}

// 0x347a14 — __ZN3RBX13LuaWebService8CallbackIN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEvNS2_8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS2_8functionIFvT_EEENSF_IFvSsEEE
// demangled: void RBX::LuaWebService::Callback<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)
#[doc(alias = "void RBX::LuaWebService::Callback<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_347a14(ok: bool, dispatch: &mut dyn FnMut() -> bool, on_error: &mut dyn FnMut(&str)) { // IDA 0x347a14: valid service + clean dispatch → return; else "LuaWebService error" to the error callback.
    if ok && dispatch() {
        return;
    }
    on_error("LuaWebService error");
}

// 0x347e10 — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS3_10Reflection7VariantESaISC_EEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSO_ISsEENSO_ISI_EENSO_ISK_EEEEED1Ev
// demangled: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::~bind_t()")]
pub fn stub_347e10(bind: LuaCallbackBind) { // IDA 0x347e10: bind_t dtor (callback clears + string/weak releases folded into drop).
    drop(bind);
}

// 0x3480fc — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS2_10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS7_IFvSsEEES4_NS_3argILi1EEESsSM_SO_EENS_3_bi6bind_tIT_PFST_T0_T1_T2_T3_T4_ENSR_9list_av_5IT5_T6_T7_T8_T9_E4typeEEES10_S12_S13_S14_S15_S16_
// demangled: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<boost::weak_ptr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>,boost::weak_ptr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>>(void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::weak_ptr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::LuaWebService>,boost::arg<1>,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_3480fc(result: HttpRequestResult, url: String, on_map: SharedPtr<dyn Fn()>, on_string: SharedPtr<dyn Fn(String)>) -> LuaCallbackBind { // IDA 0x3480fc: bind tuple ctor (map-variant callback flavor; same layout as 0x347518).
    LuaCallbackBind { result, url, on_variants: on_map, on_string }
}
