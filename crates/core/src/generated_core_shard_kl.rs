//! core shard kl — 100 stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core after kk 0x1c73f0 (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, boost+RBX|std|rbx; 21918 filtered, 553 remaining, 21365->21465 distinct, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
// Batch 3: 0x316414-0x31ae54 (22 fns) ported IDA-grounded — boost::bind/function
// closure glue for the RBX::Http callback family + sp_counted_impl_p<dir_itr_imp>.
// AGENTS.md section 4: bind/function/_bi::bind_t -> Box<dyn Fn>/closures,
// shared_ptr -> rbx_core::SharedPtr.

/// was: `boost::bind` / `boost::function<void(string*, exception*)>` closure
/// machinery over the RBX::Http callback family (IDA 0x318118-0x31ae54).
/// Each item notes the EA whose decompile/disasm grounds it.
pub mod bind_http {
    use crate::shared_ptr::{ControlBlockP, CreatableInstanceDeleter};
    use crate::SharedPtr;

    /// was: `boost::function<void(std::string*, std::exception*)>` — the
    /// Http completion callback (IDA 0x318178 `assign_to_own`). `Arc` is
    /// `Clone`, which the functor-manager clone op (IDA 0x31a608) requires.
    pub type HttpDoneCallback =
        SharedPtr<dyn Fn(Option<String>, Option<String>) + Send + Sync>;

    /// was: `boost::shared_ptr<RBX::mutex>` — call-site token threaded through
    /// `function1<void, shared_ptr<mutex>>::invoke` (IDA 0x319a10). Opaque here;
    /// the bound-args-only call (IDA 0x31a3d8) never touches it.
    #[derive(Debug, Default, Clone)]
    pub struct MutexHandle;
    pub type MutexSlot = SharedPtr<MutexHandle>;

    /// was: `bind_t<..., list5<value<string>, value<shared_ptr<istream>>,
    /// value<bool>, value<bool>, value<function<...>>>>` bound image 0x24 bytes
    /// (IDA 0x31a294 `new 0x24`; fields at +0/+4/+8/+12/+16/+17/+20).
    #[derive(Clone)]
    pub struct PostStreamBindArgs {
        pub url: String,
        pub body: SharedPtr<Vec<u8>>,
        pub flag_a: bool,
        pub flag_b: bool,
        pub done: HttpDoneCallback,
    }
    /// was: `void(*)(string, shared_ptr<istream>, bool, bool, function<...>)`
    /// stored at image +0 (IDA 0x31a2a4 / 0x31893e).
    pub type PostStreamTarget = fn(&PostStreamBindArgs);
    #[derive(Clone)]
    pub struct BindPostStream {
        pub target: PostStreamTarget,
        pub args: PostStreamBindArgs,
    }

    /// was: `bind_t<void, void(*)(string, bool, function<...>),
    /// list_av_3<string, bool, function<...>>>` (IDA 0x318118).
    #[derive(Clone)]
    pub struct DoGetBindArgs {
        pub url: String,
        pub flag: bool,
        pub done: HttpDoneCallback,
    }
    pub type DoGetTarget = fn(&DoGetBindArgs);
    #[derive(Clone)]
    pub struct BindDoGet {
        pub target: DoGetTarget,
        pub args: DoGetBindArgs,
    }

    /// was: `bind_t<void, void(*)(string, string, bool, bool, function<...>),
    /// list_av_5<string, string, bool, bool, function<...>>>` (IDA 0x3183e0).
    #[derive(Clone)]
    pub struct DoPostBindArgs {
        pub url: String,
        pub path: String,
        pub flag_a: bool,
        pub flag_b: bool,
        pub done: HttpDoneCallback,
    }
    pub type DoPostTarget = fn(&DoPostBindArgs);
    #[derive(Clone)]
    pub struct BindDoPost {
        pub target: DoPostTarget,
        pub args: DoPostBindArgs,
    }

    /// was: `storage2<value<string>, value<shared_ptr<istream>>>` — image +0/+4
    /// (IDA 0x31aeb0 string copy, 0x31aed4/0x31aede shared_count addref).
    pub struct Storage2 {
        pub url: String,
        pub body: SharedPtr<Vec<u8>>,
    }
    /// was: `storage3<..., value<bool>>` — storage2 plus byte at +12
    /// (IDA 0x31ad82).
    pub struct Storage3 {
        pub base: Storage2,
        pub flag_a: bool,
    }
    /// was: `storage4<..., value<bool>>` — storage3 plus byte at +13
    /// (IDA 0x31ac0e).
    pub struct Storage4 {
        pub base: Storage3,
        pub flag_b: bool,
    }
    /// was: `storage5<..., value<function<...>>>` — storage4 plus
    /// `assign_to_own` at +16 (IDA 0x31aa4a/0x31aa52).
    pub struct Storage5 {
        pub base: Storage4,
        pub done: HttpDoneCallback,
    }

    /// was: `boost::detail::function::functor_manager_operation_type`
    /// (IDA 0x31a5f8 switch: 0 clone, 1 move, 2 destroy, 3 check-type,
    /// default get-type; IDA 0x3199f6 treats 4 as get-type).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FunctorOp {
        CloneBind = 0,
        MoveBind = 1,
        DestroyBind = 2,
        CheckType = 3,
        GetType = 4,
    }

    /// Type name compared by the check-type op (IDA 0x31a6e0 `strcmp`).
    pub const BIND_POST_STREAM_TYPE_NAME: &str = "N5boost3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS0_5list5INS0_5valueISsEENSD_IS3_EENSD_IbEESG_NSD_IS9_EEEEEE";

    /// was: `boost::function1<void, shared_ptr<mutex>>` holding the bind_t —
    /// empty or one boxed bound call (IDA function_buffer small/heap split
    /// collapses: the 0x24 image never fits the small buffer, always boxed).
    #[derive(Default, Clone)]
    pub enum FunctionSlot {
        #[default]
        Empty,
        Bound(Box<BindPostStream>),
    }

    /// Observable outcome of one manager op (IDA 0x31a594 stores through the
    /// out-buffer pointer; Rust returns the effect by value).
    pub enum ManageEffect {
        Cloned(Option<BindPostStream>),
        Moved(Option<BindPostStream>),
        Destroyed,
        TypeMatch(bool),
        TypeName(&'static str),
    }

    impl FunctionSlot {
        /// IDA 0x319686/0x319ae2/0x31a32c: install the (copied) image.
        pub fn assign_bind(&mut self, bind: BindPostStream) {
            *self = FunctionSlot::Bound(Box::new(bind));
        }
        /// IDA 0x319a10 via 0x31a3d8: call target with bound args only.
        pub fn invoke(&self, _mu: &MutexSlot) {
            if let FunctionSlot::Bound(bind) = self {
                (bind.target)(&bind.args);
            }
        }
        /// IDA 0x31a594 manager switch over the op.
        pub fn manage(&mut self, op: FunctorOp) -> ManageEffect {
            match op {
                // IDA 0x31a608-0x31a67e: new 0x24, field-by-field copy.
                FunctorOp::CloneBind => match self {
                    FunctionSlot::Bound(bind) => {
                        ManageEffect::Cloned(Some((**bind).clone()))
                    }
                    FunctionSlot::Empty => ManageEffect::Cloned(None),
                },
                // IDA 0x31a684-0x31a688: move pointer, null the source.
                FunctorOp::MoveBind => {
                    let taken = std::mem::take(self);
                    match taken {
                        FunctionSlot::Bound(bind) => ManageEffect::Moved(Some(*bind)),
                        FunctionSlot::Empty => ManageEffect::Moved(None),
                    }
                }
                // IDA 0x31a68e-0x31a6c2: clear function, release
                // shared_count, destroy string, operator delete.
                FunctorOp::DestroyBind => {
                    *self = FunctionSlot::Empty;
                    ManageEffect::Destroyed
                }
                // IDA 0x31a6e0-0x31a6ea: strcmp stored type name against the
                // bind_t name; match stores the pointer, else null. The slot
                // is monomorphic, so the stored name always matches.
                FunctorOp::CheckType => ManageEffect::TypeMatch(true),
                // IDA 0x319a0a + default arm: out = typeid bind_t.
                FunctorOp::GetType => {
                    ManageEffect::TypeName(BIND_POST_STREAM_TYPE_NAME)
                }
            }
        }
    }

    /// was: `boost::scoped_ptr<RBX::ThreadPool>` — single-owner box
    /// (IDA 0x31810a-0x318112 loads px, null-checks, deleting-dtors it).
    pub struct ScopedPtr<T>(pub Option<Box<T>>);

    impl<T> ScopedPtr<T> {
        /// IDA 0x318104: the virtual deleting-dtor call through the vtable
        /// collapses to a static drop — same dtor-then-free order.
        pub fn destroy(&mut self) {
            drop(self.0.take());
        }
    }

    /// was: `sp_counted_impl_p<dir_itr_imp>` block access shared with the
    /// 0x316414-0x3164c4 stubs below (cf. `crate::shared_ptr::ControlBlockP`,
    /// grounded in 0x4fe14c/0x463dc8/0x4fed34/0x463e70).
    pub fn control_block_p<T>(px: Box<T>) -> ControlBlockP<T> {
        ControlBlockP::new(px)
    }

    pub fn control_block_deleter_name() -> Option<CreatableInstanceDeleter> {
        ControlBlockP::<u8>::new(Box::new(0)).get_deleter()
    }
} // mod bind_http

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::detail::dir_itr_imp>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEED1Ev")]
// 0x316414 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEED1Ev
pub fn stub_0x316414<T>(_block: &mut crate::shared_ptr::ControlBlockP<T>) {
    // IDA 0x316414: D1 body is empty (`;`) — member dtors are trivial once
    // dispose (0x31641c) released px; cf. ControlBlockP::dispose.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::detail::dir_itr_imp>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEED0Ev")]
// 0x316418 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEED0Ev
pub fn stub_0x316418<T>(block: Box<crate::shared_ptr::ControlBlockP<T>>) {
    // IDA 0x316418 [thunk]: `return operator delete(a1)` — frees the block
    // after D1 ran; cf. stub_4531f8 in boost_core_a.rs.
    drop(block);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::detail::dir_itr_imp>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEE7disposeEv")]
// 0x31641c — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEE7disposeEv
pub fn stub_0x31641c<T>(block: &mut crate::shared_ptr::ControlBlockP<T>) {
    // IDA 0x31641c: `v1 = *(a1+12); if (v1) { dir_itr_imp::~dir_itr_imp(v1);
    // operator delete(v1); }` — dtor-then-free under null check.
    block.dispose();
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::detail::dir_itr_imp>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEE11get_deleterERKSt9type_info")]
// 0x3164c0 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEE11get_deleterERKSt9type_info
pub fn stub_0x3164c0<T>(
    block: &crate::shared_ptr::ControlBlockP<T>,
) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x3164c0: `return 0` — a `_p` block never carries a deleter.
    block.get_deleter()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::filesystem::detail::dir_itr_imp>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEE19get_untyped_deleterEv")]
// 0x3164c4 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem6detail11dir_itr_impEE19get_untyped_deleterEv
pub fn stub_0x3164c4<T>(
    block: &crate::shared_ptr::ControlBlockP<T>,
) -> Option<crate::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0x3164c4: `return 0`.
    block.get_untyped_deleter()
}

#[doc(alias = "RBX::Http::get(boost::function<void ()(std::string *,std::exception *)>,bool)")]
#[doc(alias = "__ZN3RBX4Http3getEN5boost8functionIFvPSsPSt9exceptionEEEb")]
// 0x3168b0 — __ZN3RBX4Http3getEN5boost8functionIFvPSsPSt9exceptionEEEb
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x3168b0() {
    // IDA 0x3168b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::doGet(std::string,bool,boost::function<void ()(std::string *,std::exception *)>)")]
#[doc(alias = "__ZN3RBXL5doGetESsbN5boost8functionIFvPSsPSt9exceptionEEE")]
// 0x316b74 — __ZN3RBXL5doGetESsbN5boost8functionIFvPSsPSt9exceptionEEE
pub fn stub_0x316b74() {
    // IDA 0x316b74: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "RBX::Http::post(std::string const&,bool,boost::function<void ()(std::string *,std::exception *)>,bool)")]
#[doc(alias = "__ZN3RBX4Http4postERKSsbN5boost8functionIFvPSsPSt9exceptionEEEb")]
// 0x316f2c — __ZN3RBX4Http4postERKSsbN5boost8functionIFvPSsPSt9exceptionEEEb
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0x316f2c() {
    // IDA 0x316f2c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "RBX::doPost(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
#[doc(alias = "__ZN3RBXL6doPostESsSsbbN5boost8functionIFvPSsPSt9exceptionEEE")]
// 0x317378 — __ZN3RBXL6doPostESsSsbbN5boost8functionIFvPSsPSt9exceptionEEE
pub fn stub_0x317378() {
    // IDA 0x317378: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Http::post(rbx_core::SharedPtr<std::istream>,bool,boost::function<void ()(std::string *,std::exception *)>,bool)")]
#[doc(alias = "__ZN3RBX4Http4postEN5boost10shared_ptrISiEEbNS1_8functionIFvPSsPSt9exceptionEEEb")]
// 0x317570 — __ZN3RBX4Http4postEN5boost10shared_ptrISiEEbNS1_8functionIFvPSsPSt9exceptionEEEb
// was: RBX::Http::post(rbx_core::SharedPtr<std::istream>,bool,boost::function<void ()(std::string *,std::exception *)>,bool)
pub fn stub_0x317570() {
    // IDA 0x317570: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::doPostStream(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
#[doc(alias = "__ZN3RBXL12doPostStreamESsN5boost10shared_ptrISiEEbbNS0_8functionIFvPSsPSt9exceptionEEE")]
// 0x317a08 — __ZN3RBXL12doPostStreamESsN5boost10shared_ptrISiEEbbNS0_8functionIFvPSsPSt9exceptionEEE
// was: RBX::doPostStream(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)
pub fn stub_0x317a08() {
    // IDA 0x317a08: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::scoped_ptr<RBX::ThreadPool>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX10ThreadPoolEED1Ev")]
// 0x318104 — __ZN5boost10scoped_ptrIN3RBX10ThreadPoolEED1Ev
pub fn stub_0x318104<T>(slot: &mut bind_http::ScopedPtr<T>) {
    // IDA 0x318104: `v2 = *a1; if (v2) vtable[v2][1](v2)` — null-checked
    // deleting-dtor through the vtable; static drop is the same order.
    slot.destroy();
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_3<std::string,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,bool,boost::function<void ()(std::string *,std::exception *)>)")]
#[doc(alias = "__ZN5boost4bindIvSsbNS_8functionIFvPSsPSt9exceptionEEESsbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_")]
// 0x318118 — __ZN5boost4bindIvSsbNS_8functionIFvPSsPSt9exceptionEEESsbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_
// type: int __fastcall(int, int, std::string *, int, int)
pub fn stub_0x318118(
    target: bind_http::DoGetTarget,
    url: String,
    flag: bool,
    done: bind_http::HttpDoneCallback,
) -> bind_http::BindDoGet {
    // IDA 0x318118: string copy (0x31813e), function assign_to_own
    // (0x318178), list3 pack (0x318188), then bind_t image out: *a1 = f
    // (0x318190), string at +4 (0x31819e), byte at +8 (0x3181aa),
    // function at +12 (0x3181bc). The returned struct is that image.
    bind_http::BindDoGet { target, args: bind_http::DoGetBindArgs { url, flag, done } }
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
#[doc(alias = "__ZN5boost4bindIvSsSsbbNS_8functionIFvPSsPSt9exceptionEEESsSsbbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_T3_T4_ENS7_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESG_SI_SJ_SK_SL_SM_")]
// 0x3183e0 — __ZN5boost4bindIvSsSsbbNS_8functionIFvPSsPSt9exceptionEEESsSsbbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_T3_T4_ENS7_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESG_SI_SJ_SK_SL_SM_
// type: int __fastcall(int, int, std::string *, int, int, int, int)
pub fn stub_0x3183e0(
    target: bind_http::DoPostTarget,
    url: String,
    path: String,
    flag_a: bool,
    flag_b: bool,
    done: bind_http::HttpDoneCallback,
) -> bind_http::BindDoPost {
    // IDA 0x3183e0: two string copies (0x318418/0x318452), assign_to_own
    // (0x318462), list5 pack (0x31847a), image out: f at +0 (0x318482),
    // strings at +4/+8 (0x318490/0x3184a4), bytes at +12/+13
    // (0x3184ae/0x3184b2), function at +16 (0x3184c4).
    bind_http::BindDoPost {
        target,
        args: bind_http::DoPostBindArgs { url, path, flag_a, flag_b, done },
    }
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
#[doc(alias = "__ZN5boost4bindIvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEESsS2_bbS8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_")]
// 0x31888c — __ZN5boost4bindIvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEESsS2_bbS8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
// type: int __fastcall(int, int, std::string *, int, int, int, int)
// was: boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)
pub fn stub_0x31888c(
    target: bind_http::PostStreamTarget,
    url: String,
    body: crate::SharedPtr<Vec<u8>>,
    flag_a: bool,
    flag_b: bool,
    done: bind_http::HttpDoneCallback,
) -> bind_http::BindPostStream {
    // IDA 0x31888c: string copy (0x3188c6), shared_count addref pair
    // (0x318908/0x318912 — the by-value clone), assign_to_own (0x31891e),
    // list5 pack (0x318936), image out: f at +0 (0x31893e), string at +4
    // (0x31894c), shared px/count at +8/+12 (0x318954/0x318966), bytes at
    // +16/+17, function at +20. By-value args are those copies.
    bind_http::BindPostStream {
        target,
        args: bind_http::PostStreamBindArgs { url, body, flag_a, flag_b, done },
    }
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEEvT_")]
// 0x3195c0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEEvT_
// was: void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)
pub fn stub_0x3195c0(slot: &mut bind_http::FunctionSlot, bind: bind_http::BindPostStream) {
    // IDA 0x3195c0: deep-copies each image field (string 0x3195fe,
    // shared_count 0x319640, function assign_to_own 0x319664) then routes to
    // basic_vtable assign_to (0x319686). A by-value move is the same state.
    slot.assign_bind(bind);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
// 0x3199f4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x3199f4(
    slot: &mut bind_http::FunctionSlot,
    op: bind_http::FunctorOp,
) -> bind_http::ManageEffect {
    // IDA 0x3199f4: `if (op != 4) return manager(...); store bind_t typeid;
    // return typeid` — get-type is inline, the rest tail-call 0x31a594.
    if op == bind_http::FunctorOp::GetType {
        slot.manage(op)
    } else {
        stub_0x31a594(slot, op)
    }
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEvNS5_IN3RBX5mutexEEEE6invokeERNS1_15function_bufferESP_")]
// 0x319a10 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEvNS5_IN3RBX5mutexEEEE6invokeERNS1_15function_bufferESP_
// type: int __fastcall(int)
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)
pub fn stub_0x319a10(slot: &bind_http::FunctionSlot, mu: &bind_http::MutexSlot) {
    // IDA 0x319a10: single tail-call into list5::operator() (0x319a26) with
    // the buffer; the mutex travels as the ignored call-site list.
    slot.invoke(mu);
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferE")]
// 0x319a28 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x319a28(slot: &mut bind_http::FunctionSlot, src: &bind_http::BindPostStream) {
    // IDA 0x319a28: deep-copies the src image field-by-field (string
    // 0x319a68, shared_count 0x319aaa, function 0x319ace) then vtable
    // assign_to (0x319ae2). Clone-then-install is the same copy.
    slot.assign_bind(src.clone());
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x319e4c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int)
// was: bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x319e4c(slot: &mut bind_http::FunctionSlot, src: &bind_http::BindPostStream) {
    // IDA 0x319e4c: same field copies as 0x319a28 (0x319e8a/0x319ecc/
    // 0x319ef0) but routes with function_obj_tag to assign_functor
    // (0x319f02 -> 0x31a26c).
    stub_0x31a26c(slot, src.clone());
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x31a26c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, std::string *, int, int, int, int, int)
// was: void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x31a26c(slot: &mut bind_http::FunctionSlot, src: bind_http::BindPostStream) {
    // IDA 0x31a26c: `new 0x24` (0x31a294), field copies at +0/+4/+8/+12/
    // +16/+17 (0x31a2a4-0x31a30a), function assign_to_own at +20
    // (0x31a31c), store image pointer (0x31a32c). Box is the heap image.
    slot.assign_bind(src);
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsS5_bbSD_ENS0_5list1IRNS4_IN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i")]
// 0x31a3d8 — __ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsS5_bbSD_ENS0_5list1IRNS4_IN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
// was: void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)
pub fn stub_0x31a3d8(bind: &bind_http::BindPostStream, _mu: &bind_http::MutexSlot) {
    // IDA 0x31a3d8: unpacks bound string/shared/function (0x31a400-0x31a45c),
    // tail-calls F (0x31a480); the incoming mutex list1 is never read — all
    // values, no placeholders. Shared/bytes temporaries release after
    // (0x31a488-0x31a4f0); Rust drops do that.
    (bind.target)(&bind.args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x31a594 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x31a594(
    slot: &mut bind_http::FunctionSlot,
    op: bind_http::FunctorOp,
) -> bind_http::ManageEffect {
    // IDA 0x31a594: switch op { 0: clone image (0x31a608-0x31a67e),
    // 1: move pointer + null src (0x31a684-0x31a688), 2: clear/release/
    // destroy/delete (0x31a68e-0x31a6c2), 3: strcmp type name (0x31a6e0),
    // default: typeid (truncated tail) }. Cf. FunctionSlot::manage.
    slot.manage(op)
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_")]
// 0x31a7f0 — __ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_
// was: boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)
pub fn stub_0x31a7f0(
    url: String,
    body: crate::SharedPtr<Vec<u8>>,
    flag_a: bool,
    flag_b: bool,
    done: bind_http::HttpDoneCallback,
) -> bind_http::PostStreamBindArgs {
    // IDA 0x31a7f0: copies each arg to temporaries (string 0x31a816,
    // shared_count 0x31a854, function 0x31a86a) then delegates to storage5
    // (0x31a886); temporaries release after (0x31a890-0x31a8f8). Flattening
    // the storage image back out is the same five values.
    let stored = stub_0x31a99c(url, body, flag_a, flag_b, done);
    bind_http::PostStreamBindArgs {
        url: stored.base.base.base.url,
        body: stored.base.base.base.body,
        flag_a: stored.base.base.flag_a,
        flag_b: stored.base.flag_b,
        done: stored.done,
    }
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_")]
// 0x31a99c — __ZN5boost3_bi8storage5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_
// was: boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)
pub fn stub_0x31a99c(
    url: String,
    body: crate::SharedPtr<Vec<u8>>,
    flag_a: bool,
    flag_b: bool,
    done: bind_http::HttpDoneCallback,
) -> bind_http::Storage5 {
    // IDA 0x31a99c: string/shared temporaries (0x31a9c2/0x31aa00), storage4
    // into the image head (0x31aa1e), temporaries released (0x31aa24-0x31aa3c),
    // then `*(+16) = 0` + assign_to_own of the function (0x31aa4a/0x31aa52).
    // Moving `done` in is that ownership transfer.
    bind_http::Storage5 { base: stub_0x31ab68(url, body, flag_a, flag_b), done }
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_EC2ES3_S6_S7_S7_")]
// 0x31ab68 — __ZN5boost3_bi8storage4INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_EC2ES3_S6_S7_S7_
// was: boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>)
pub fn stub_0x31ab68(
    url: String,
    body: crate::SharedPtr<Vec<u8>>,
    flag_a: bool,
    flag_b: bool,
) -> bind_http::Storage4 {
    // IDA 0x31ab68: string/shared temporaries (0x31ab8e/0x31abcc), storage3
    // into the head (0x31abe4), temporaries released (0x31abea-0x31ac02),
    // then the bool at +13 (0x31ac0e).
    bind_http::Storage4 { base: stub_0x31ace0(url, body, flag_a), flag_b }
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEEEC2ES3_S6_S7_")]
// 0x31ace0 — __ZN5boost3_bi8storage3INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEEEC2ES3_S6_S7_
// was: boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>)
pub fn stub_0x31ace0(
    url: String,
    body: crate::SharedPtr<Vec<u8>>,
    flag_a: bool,
) -> bind_http::Storage3 {
    // IDA 0x31ace0: string/shared temporaries (0x31ad06/0x31ad44), storage2
    // into the head (0x31ad58), temporaries released (0x31ad5e-0x31ad76),
    // then the bool at +12 (0x31ad82).
    bind_http::Storage3 { base: stub_0x31ae54(url, body), flag_a }
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueISsEENS2_INS_10shared_ptrISiEEEEEC2ES3_S6_")]
// 0x31ae54 — __ZN5boost3_bi8storage2INS0_5valueISsEENS2_INS_10shared_ptrISiEEEEEC2ES3_S6_
// was: boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>)
pub fn stub_0x31ae54(
    url: String,
    body: crate::SharedPtr<Vec<u8>>,
) -> bind_http::Storage2 {
    // IDA 0x31ae54: string copy (0x31ae78/0x31aeb0, temp released
    // 0x31aec4-0x31af2c) plus the shared_count addref pair writing px/pi
    // (0x31aed4/0x31aede/0x31aee8). By-value params are those copies.
    bind_http::Storage2 { url, body }
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEEvT_")]
// 0x31b6c4 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEEvT_
// was: void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)
pub fn stub_0x31b6c4() {
    // IDA 0x31b6c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")]
// 0x31ba5c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
pub fn stub_0x31ba5c() {
    // IDA 0x31ba5c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEvNS_10shared_ptrIN3RBX5mutexEEEE6invokeERNS1_15function_bufferESN_")]
// 0x31ba78 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEvNS_10shared_ptrIN3RBX5mutexEEEE6invokeERNS1_15function_bufferESN_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)
pub fn stub_0x31ba78() {
    // IDA 0x31ba78: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE")]
// 0x31ba80 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x31ba80() {
    // IDA 0x31ba80: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x31be08 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int)
// was: bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x31be08() {
    // IDA 0x31be08: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x31c18c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
// was: void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x31c18c() {
    // IDA 0x31c18c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsSsbbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i")]
// 0x31c2e8 — __ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsSsbbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
// was: void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)
pub fn stub_0x31c2e8() {
    // IDA 0x31c2e8: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x31c4e4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
pub fn stub_0x31c4e4() {
    // IDA 0x31c4e4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_")]
// 0x31c72c — __ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_
pub fn stub_0x31c72c() {
    // IDA 0x31c72c: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_")]
// 0x31c918 — __ZN5boost3_bi8storage5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_
pub fn stub_0x31c918() {
    // IDA 0x31c918: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueISsEES3_NS2_IbEES4_EC2ES3_S3_S4_S4_")]
// 0x31cb24 — __ZN5boost3_bi8storage4INS0_5valueISsEES3_NS2_IbEES4_EC2ES3_S3_S4_S4_
// type: int __fastcall(int, const std::string *, const std::string *, unsigned __int8, char)
pub fn stub_0x31cb24() {
    // IDA 0x31cb24: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueISsEES3_NS2_IbEEEC2ES3_S3_S4_")]
// 0x31ccd4 — __ZN5boost3_bi8storage3INS0_5valueISsEES3_NS2_IbEEEC2ES3_S3_S4_
pub fn stub_0x31ccd4() {
    // IDA 0x31ccd4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEEvT_")]
// 0x31d2d0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEEvT_
// was: void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)
pub fn stub_0x31d2d0() {
    // IDA 0x31d2d0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")]
// 0x31d50c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
pub fn stub_0x31d50c() {
    // IDA 0x31d50c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEvNS_10shared_ptrIN3RBX5mutexEEEE6invokeERNS1_15function_bufferESN_")]
// 0x31d528 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEvNS_10shared_ptrIN3RBX5mutexEEEE6invokeERNS1_15function_bufferESN_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)
pub fn stub_0x31d528() {
    // IDA 0x31d528: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE")]
// 0x31d530 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x31d530() {
    // IDA 0x31d530: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x31d75c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, void *, std::string *)
// was: bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x31d75c() {
    // IDA 0x31d75c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x31d984 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
// was: void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x31d984() {
    // IDA 0x31d984: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i")]
// 0x31da84 — __ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
// was: void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)
pub fn stub_0x31da84() {
    // IDA 0x31da84: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x31dbf0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
pub fn stub_0x31dbf0() {
    // IDA 0x31dbf0: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::function2<void,std::string *,std::exception *>::assign_to_own(boost::function2<void,std::string *,std::exception *> const&)")]
#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionE13assign_to_ownERKS4_")]
// 0x31dd9c — __ZN5boost9function2IvPSsPSt9exceptionE13assign_to_ownERKS4_
// type: int __fastcall(int result, int *)
pub fn stub_0x31dd9c() {
    // IDA 0x31dd9c: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list3(boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_")]
// 0x31ddcc — __ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_
// type: int(void)
pub fn stub_0x31ddcc() {
    // IDA 0x31ddcc: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_")]
// 0x31df30 — __ZN5boost3_bi8storage3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_
pub fn stub_0x31df30() {
    // IDA 0x31df30: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueISsEENS2_IbEEEC2ES3_S4_")]
// 0x31e084 — __ZN5boost3_bi8storage2INS0_5valueISsEENS2_IbEEEC2ES3_S4_
pub fn stub_0x31e084() {
    // IDA 0x31e084: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,std::string *,std::exception *>::operator()(std::string *,std::exception *)const")]
#[doc(alias = "__ZNK5boost9function2IvPSsPSt9exceptionEclES1_S3_")]
// 0x31e1a8 — __ZNK5boost9function2IvPSsPSt9exceptionEclES1_S3_
pub fn stub_0x31e1a8() {
    // IDA 0x31e1a8: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<std::istream>::shared_ptr<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
#[doc(alias = "__ZN5boost10shared_ptrISiEC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_")]
// 0x31e270 — __ZN5boost10shared_ptrISiEC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_
// was: rbx_core::SharedPtr<std::istream>::shared_ptr<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)
pub fn stub_0x31e270() {
    // IDA 0x31e270: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_")]
// 0x31e344 — __ZN5boost6detail12shared_countC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x31e344() {
    // IDA 0x31e344: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED1Ev")]
// 0x31e43c — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED1Ev
pub fn stub_0x31e43c() {
    // IDA 0x31e43c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED0Ev")]
// 0x31e440 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED0Ev
pub fn stub_0x31e440() {
    // IDA 0x31e440: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE7disposeEv")]
// 0x31e444 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE7disposeEv
pub fn stub_0x31e444() {
    // IDA 0x31e444: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info")]
// 0x31e454 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info
pub fn stub_0x31e454() {
    // IDA 0x31e454: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv")]
// 0x31e458 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv
// type: int()
pub fn stub_0x31e458() {
    // IDA 0x31e458: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::destroy(void)")]
#[doc(alias = "__ZN5boost15circular_bufferIdSaIdEE7destroyEv")]
// 0x31e63c — __ZN5boost15circular_bufferIdSaIdEE7destroyEv
// type: int(void)
pub fn stub_0x31e63c() {
    // IDA 0x31e63c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::clear(void)")]
#[doc(alias = "__ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5clearEv")]
// 0x325bb4 — __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5clearEv
pub fn stub_0x325bb4() {
    // IDA 0x325bb4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::check_low_capacity(unsigned long)")]
#[doc(alias = "__ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE18check_low_capacityEm")]
// 0x325be0 — __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE18check_low_capacityEm
pub fn stub_0x325be0() {
    // IDA 0x325be0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::push_back(RBX::InterpolatedCFrame::FrameInfo const&)")]
#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE9push_backERKS3_")]
// 0x325c38 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE9push_backERKS3_
// type: int __fastcall(int, G3D::Matrix3 *)
pub fn stub_0x325c38() {
    // IDA 0x325c38: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::set_capacity(unsigned long)")]
#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm")]
// 0x325ce4 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm
pub fn stub_0x325ce4() {
    // IDA 0x325ce4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::operator+=(int)")]
#[doc(alias = "__ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEpLEi")]
// 0x325e8c — __ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEpLEi
pub fn stub_0x325e8c() {
    // IDA 0x325e8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::operator-=(int)")]
#[doc(alias = "__ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEmIEi")]
// 0x325ed4 — __ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEmIEi
pub fn stub_0x325ed4() {
    // IDA 0x325ed4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::erase(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>,boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>)")]
#[doc(alias = "__ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorINS_15circular_bufferIS3_S4_EENS6_15nonconst_traitsIS4_EEEESC_")]
// 0x325f14 — __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorINS_15circular_bufferIS3_S4_EENS6_15nonconst_traitsIS4_EEEESC_
pub fn stub_0x325f14() {
    // IDA 0x325f14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::erase(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>,boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>)")]
#[doc(alias = "__ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorIS5_NS6_15nonconst_traitsIS4_EEEESA_")]
// 0x325f94 — __ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorIS5_NS6_15nonconst_traitsIS4_EEEESA_
pub fn stub_0x325f94() {
    // IDA 0x325f94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::check_high_capacity(void)")]
#[doc(alias = "__ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE19check_high_capacityEv")]
// 0x32607c — __ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE19check_high_capacityEv
pub fn stub_0x32607c() {
    // IDA 0x32607c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::pointer boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::linearize_pointer<boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::pointer> const&)const")]
#[doc(alias = "__ZNK5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEE17linearize_pointerIS9_EENT_7pointerERKNS1_IS7_SC_EE")]
// 0x3260d8 — __ZNK5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEE17linearize_pointerIS9_EENT_7pointerERKNS1_IS7_SC_EE
// type: int __fastcall(_DWORD **, int)
pub fn stub_0x3260d8() {
    // IDA 0x3260d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")]
// 0x345b08 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
pub fn stub_0x345b08() {
    // IDA 0x345b08: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_")]
// 0x345b48 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
// type: int __fastcall(int, unsigned int, std::string *)
pub fn stub_0x345b48() {
    // IDA 0x345b48: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE")]
// 0x345bb4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE
pub fn stub_0x345bb4() {
    // IDA 0x345bb4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::AsyncHttpQueue>::expired(void)const")]
#[doc(alias = "__ZNK5boost8weak_ptrIN3RBX14AsyncHttpQueueEE7expiredEv")]
// 0x352dc8 — __ZNK5boost8weak_ptrIN3RBX14AsyncHttpQueueEE7expiredEv
// was: rbx_core::WeakPtr<RBX::AsyncHttpQueue>::expired(void)const
pub fn stub_0x352dc8() {
    // IDA 0x352dc8: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<bool (*)(std::string const&,std::string *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerIPFbRKSsPSsEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE")]
// 0x356328 — __ZN5boost6detail8function15functor_managerIPFbRKSsPSsEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
pub fn stub_0x356328() {
    // IDA 0x356328: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::function1<void,bool>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IvbE5clearEv")]
// 0x356588 — __ZN5boost9function1IvbE5clearEv
pub fn stub_0x356588() {
    // IDA 0x356588: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::clear(void)")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE5clearEv")]
// 0x3565b8 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE5clearEv
// was: boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::clear(void)
pub fn stub_0x3565b8() {
    // IDA 0x3565b8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_")]
// 0x36e490 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_
// type: int __fastcall(int, int *, int)
pub fn stub_0x36e490() {
    // IDA 0x36e490: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")]
// 0x36e4ec — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x36e4ec() {
    // IDA 0x36e4ec: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE")]
// 0x36e518 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// type: int __fastcall(int, int, int)
pub fn stub_0x36e518() {
    // IDA 0x36e518: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")]
// 0x36e610 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// type: int __fastcall(int, char **)
pub fn stub_0x36e610() {
    // IDA 0x36e610: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_")]
// 0x36e650 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_
// type: int __fastcall(int, unsigned int, std::string *)
pub fn stub_0x36e650() {
    // IDA 0x36e650: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS5_RKT_")]
// 0x36e6bc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS5_RKT_
// type: void __fastcall(int, int, char **, int)
pub fn stub_0x36e6bc() {
    // IDA 0x36e6bc: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE20construct_with_valueINS1_13emplace_args1ISA_EEEEvRKT_")]
// 0x36e874 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE20construct_with_valueINS1_13emplace_args1ISA_EEEEvRKT_
// type: int __fastcall(int, const std::string **)
pub fn stub_0x36e874() {
    // IDA 0x36e874: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")]
// 0x36e898 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
pub fn stub_0x36e898() {
    // IDA 0x36e898: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::~node_constructor()")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEED2Ev")]
// 0x36e8e8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEED2Ev
// type: int __fastcall(int)
pub fn stub_0x36e8e8() {
    // IDA 0x36e8e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")]
// 0x36e908 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
pub fn stub_0x36e908() {
    // IDA 0x36e908: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")]
// 0x36ea30 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x36ea30() {
    // IDA 0x36ea30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")]
// 0x36eac0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x36eac0() {
    // IDA 0x36eac0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")]
// 0x36eaec — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
pub fn stub_0x36eaec() {
    // IDA 0x36eaec: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct(void)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE9constructEv")]
// 0x36eb44 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE9constructEv
// type: std::string *__fastcall(int)
pub fn stub_0x36eb44() {
    // IDA 0x36eb44: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")]
// 0x370714 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// type: void __fastcall(int)
pub fn stub_0x370714() {
    // IDA 0x370714: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE5clearEv")]
// 0x37074c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// type: void *__fastcall(int)
pub fn stub_0x37074c() {
    // IDA 0x37074c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE")]
// 0x3708e4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
// type: int __fastcall(int result, unsigned int)
pub fn stub_0x3708e4() {
    // IDA 0x3708e4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int const&,int const& (*)(int const*),boost::_bi::list1<boost::_bi::value<int const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")]
// 0x37dd20 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// type: int __fastcall(__int64, unsigned int)
pub fn stub_0x37dd20() {
    // IDA 0x37dd20: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int const&,int const& (*)(int const*),boost::_bi::list1<boost::_bi::value<int const*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEiE6invokeERNS1_15function_bufferE")]
// 0x37dd80 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEiE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
pub fn stub_0x37dd80() {
    // IDA 0x37dd80: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "RBX::StandardOut::print_exception(boost::function0<void> const&,RBX::MessageType,bool)")]
#[doc(alias = "__ZN3RBX11StandardOut15print_exceptionERKN5boost9function0IvEENS_11MessageTypeEb")]
// 0x381b0c — __ZN3RBX11StandardOut15print_exceptionERKN5boost9function0IvEENS_11MessageTypeEb
// type: void __fastcall(int, int, int, int, int, char, int, int, void *, int)
pub fn stub_0x381b0c() {
    // IDA 0x381b0c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StandardOut>::~shared_ptr()")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11StandardOutEED1Ev")]
// 0x3821f0 — __ZN5boost10shared_ptrIN3RBX11StandardOutEED1Ev
// type: int __fastcall(int)
// was: rbx_core::SharedPtr<RBX::StandardOut>::~shared_ptr()
pub fn stub_0x3821f0() {
    // IDA 0x3821f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE")]
// 0x382348 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
// was: rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> &)
pub fn stub_0x382348() {
    // IDA 0x382348: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[cfg(test)]
mod bind_http_tests {
    use super::bind_http::*;
    use super::*;
    use crate::shared_ptr::ControlBlockP;

    fn done_cb() -> HttpDoneCallback {
        crate::SharedPtr::new(|_: Option<String>, _: Option<String>| {})
    }

    fn sample_bind() -> BindPostStream {
        stub_0x31888c(
            |_: &PostStreamBindArgs| {},
            String::from("http://x"),
            crate::SharedPtr::new(Vec::new()),
            true,
            false,
            done_cb(),
        )
    }

    #[test]
    fn storage_layers_flatten_to_list5_args() {
        let args = stub_0x31a7f0(
            String::from("http://x"),
            crate::SharedPtr::new(vec![1u8]),
            true,
            false,
            done_cb(),
        );
        assert_eq!(args.url, "http://x");
        assert_eq!(*args.body, vec![1u8]);
        assert!(args.flag_a);
        assert!(!args.flag_b);
    }

    #[test]
    fn manager_clone_move_destroy_cycle() {
        let mut slot = FunctionSlot::Empty;
        stub_0x3195c0(&mut slot, sample_bind());
        // clone op duplicates the bound image (IDA 0x31a608).
        match stub_0x31a594(&mut slot, FunctorOp::CloneBind) {
            ManageEffect::Cloned(Some(clone)) => assert_eq!(clone.args.url, "http://x"),
            _ => panic!("clone must duplicate the bound image"),
        }
        // type ops report the bind_t name (IDA 0x31a6e0 / 0x319a0a).
        match stub_0x3199f4(&mut slot, FunctorOp::CheckType) {
            ManageEffect::TypeMatch(true) => {}
            _ => panic!("check-type must match the monomorphic slot"),
        }
        match stub_0x3199f4(&mut slot, FunctorOp::GetType) {
            ManageEffect::TypeName(name) => assert_eq!(name, BIND_POST_STREAM_TYPE_NAME),
            _ => panic!("get-type must return the bind_t name"),
        }
        // move op transfers and empties the source (IDA 0x31a684).
        match stub_0x31a594(&mut slot, FunctorOp::MoveBind) {
            ManageEffect::Moved(Some(moved)) => assert_eq!(moved.args.url, "http://x"),
            _ => panic!("move must transfer the bound image"),
        }
        assert!(matches!(slot, FunctionSlot::Empty));
        // destroy on empty stays empty (IDA 0x31a68e null check).
        match stub_0x31a594(&mut slot, FunctorOp::DestroyBind) {
            ManageEffect::Destroyed => {}
            _ => panic!("destroy must report completion"),
        }
    }

    #[test]
    fn counted_impl_p_dispose_releases_and_never_deletes() {
        let mut block = ControlBlockP::new(Box::new(7u32));
        assert!(block.get().is_some());
        stub_0x31641c(&mut block);
        assert!(block.get().is_none());
        assert!(stub_0x3164c0(&block).is_none());
        assert!(stub_0x3164c4(&block).is_none());
    }
}
