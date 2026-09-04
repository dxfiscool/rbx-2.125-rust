//! core shard kk — 25 IDA-grounded ports 0x777ad4-0x778c34.
//! Two boost closure families used by the ScriptDebugger hook site:
//! `rbx::make_shared<string>` + `sp_counted_impl_pd<string*, sp_ms_deleter<string>>`
//! (IDA 0x777ad4-0x778030) and the `function<void(lua_State*, lua_Debug*)>`
//! bind_t/mf5/list6/storage6/storage5 machinery (IDA 0x778034-0x778c34).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)].
//! AGENTS.md section 4: bind/function/_bi::bind_t -> Box<dyn Fn>/closures,
//! shared_ptr -> rbx_core::SharedPtr. Carriers in generated_core_shard_f.rs
//! are untouched; these ports live under new idiomatic names.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;

/// was: `rbx::make_shared<std::string>` control block
/// (`sp_counted_impl_pd<string*, sp_ms_deleter<string>>`, IDA 0x777e14:
/// fresh 0x18 block, use/weak counts 1, pointee slot, empty-flag, deleter).
pub mod make_string_shared {
    use crate::SharedPtr;
    use std::sync::Mutex;

    /// was: `rbx::detail::sp_ms_deleter<std::string>` — empty tag stored at
    /// block+16 (IDA 0x778018 compares its type_info name, 0x778030 returns
    /// the slot unconditionally).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct MsStringDeleter;

    /// type_info name compared at IDA 0x77802a.
    pub const MS_STRING_DELETER_TYPE_NAME: &str = "N3rbx6detail13sp_ms_deleterISsEE";

    /// was: `sp_counted_impl_pd<string*, sp_ms_deleter<string>>` — 0x18-byte
    /// block: vtable(+0), use_count(+4), weak_count(+8), pointee(+12),
    /// empty-flag(+16), inline string(+20). The pointee lives in `value`
    /// either way; `has_value` tracks the make_shared inline-construct flag.
    pub struct MsStringBlock {
        pub use_count: usize,
        pub weak_count: usize,
        pub value: Option<String>,
        pub has_value: bool,
        pub deleter: MsStringDeleter,
    }

    /// was: `boost::shared_ptr<std::string>` over the block above —
    /// one Arc owner adopting the control block (IDA 0x777cd8 px+pi pair).
    #[derive(Clone)]
    pub struct MsStringShared {
        pub inner: SharedPtr<Mutex<MsStringBlock>>,
    }
}

/// was: `boost::function<void(lua_State*, lua_Debug*)>` holding the
/// `bind_t<mf5<ScriptDebugger::hook...>, list6<...>>` debug-hook closure
/// (IDA 0x778034-0x778c34). The 0x24 bound image never fits the small
/// function_buffer, so it is always boxed — like `bind_http::FunctionSlot`.
pub mod debugger_hook {
    use crate::SharedPtr;

    /// was: `lua_State *` — call arg carried by `arg<1>` (IDA 0x778a88).
    pub type LuaStatePtr = usize;
    /// was: `lua_Debug *` — call arg carried by `arg<2>` (IDA 0x778a90).
    pub type LuaDebugPtr = usize;

    /// was: `RBX::Scripting::ScriptDebugger *` — bound receiver
    /// (`value<ScriptDebugger*>`, IDA 0x77821e).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct DebuggerTarget(pub usize);

    /// was: `boost::function<bool(lua_State*, lua_Debug*)>` — the hook
    /// predicate (`value<function<...>>`, IDA 0x778210 assign_to_own).
    /// `Arc` is `Clone`, which the temp-copy dance requires.
    pub type HookPredicate = SharedPtr<dyn Fn(LuaStatePtr, LuaDebugPtr) -> bool + Send + Sync>;

    /// was: the resolved `mf5<void, ScriptDebugger, lua_State*, lua_Debug*,
    /// function<bool(...)>, bool&, shared_ptr<string>&>` member call
    /// (IDA 0x778bd2 indirect call through the resolved pointer).
    pub type DebuggerHookFn =
        fn(DebuggerTarget, LuaStatePtr, LuaDebugPtr, HookPredicate, bool, SharedPtr<String>);

    /// was: the `mf5` member-pointer encoding — raw pointer/adjust word
    /// (IDA 0x778b78) plus vtable offset (IDA 0x778b98 `a2 + (v18 >> 1)`).
    #[derive(Debug, Default, Clone, Copy)]
    pub struct MemberFnAdjust {
        pub encoding: usize,
        pub vtable_offset: usize,
    }

    impl MemberFnAdjust {
        /// IDA 0x778ba4: `(v18 & 1) != 0` selects the virtual path.
        pub fn is_virtual_thunk(&self) -> bool {
            self.encoding & 1 != 0
        }
        /// IDA 0x778b98: dispatch slot for the virtual path.
        pub fn dispatch_slot(&self, obj_word: usize) -> usize {
            obj_word.wrapping_add(self.encoding >> 1)
        }
    }

    /// was: `list6<value<ScriptDebugger*>, arg<1>, arg<2>,
    /// value<function<bool...>>, reference_wrapper<bool>,
    /// reference_wrapper<shared_ptr<string>>>` (IDA 0x778034).
    /// BUG (divergence, preserved deliberately): the original keeps live
    /// `reference_wrapper` aliases into the caller's bool/source; the port
    /// snapshots both values (same as `bind_http` owned captures).
    #[derive(Clone)]
    pub struct HookBindArgs {
        pub debugger: DebuggerTarget,
        pub predicate: HookPredicate,
        pub enabled: bool,
        pub source: SharedPtr<String>,
    }

    /// was: `storage5<...>` without the trailing source slot
    /// (IDA 0x7781e0 stores debugger/predicate/enabled only).
    #[derive(Clone)]
    pub struct HookBindArgs5 {
        pub debugger: DebuggerTarget,
        pub predicate: HookPredicate,
        pub enabled: bool,
    }

    /// was: `bind_t<void, mf5<...>, list6<...>>` — member hook plus the
    /// bound-argument image (IDA 0x778c34 clones 0x24 bytes of it).
    #[derive(Clone)]
    pub struct HookBindImage {
        pub hook: DebuggerHookFn,
        pub adjust: MemberFnAdjust,
        pub args: HookBindArgs,
    }

    /// Type name compared by the check-type op (IDA 0x778d36 `strcmp`).
    pub const HOOK_BIND_TYPE_NAME: &str = "N5boost3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbS8_SA_EEERbRNS_10shared_ptrISsEEEENS0_5list6INS0_5valueIPS6_EENS_3argILi1EEENSN_ILi2EEENSK_ISD_EENS_17reference_wrapperIbEENSR_ISG_EEEEEE";

    /// was: `boost::function2<void, lua_State*, lua_Debug*>` holding the
    /// bind_t — empty or one boxed bound image.
    #[derive(Default, Clone)]
    pub enum HookSlot {
        #[default]
        Empty,
        Bound(Box<HookBindImage>),
    }

    /// was: `boost::detail::function::functor_manager_operation_type`
    /// (IDA 0x778c98 switch: 0 clone, 1 move, 2 destroy, 3 check-type,
    /// default get-type; IDA 0x778772 treats 4 as get-type).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum HookFunctorOp {
        CloneBind = 0,
        MoveBind = 1,
        DestroyBind = 2,
        CheckType = 3,
        GetType = 4,
    }

    /// Observable outcome of one manager op (IDA 0x778c34 stores through the
    /// out-buffer pointer; Rust returns the effect by value).
    pub enum HookManageEffect {
        Cloned(Option<HookBindImage>),
        Moved(Option<HookBindImage>),
        Destroyed,
        TypeMatch(bool),
        TypeName(&'static str),
    }
}

#[doc(alias = "boost::shared_ptr<std::string> rbx::make_shared<std::string,char const*>(char const* const&)")]
#[doc(alias = "__ZN3rbx11make_sharedISsPKcEEN5boost10shared_ptrIT_EERKT0_")]
// 0x777ad4 — __ZN3rbx11make_sharedISsPKcEEN5boost10shared_ptrIT_EERKT0_
pub fn stub_0x777ad4(cstr: &str) -> make_string_shared::MsStringShared {
    use make_string_shared::{MsStringBlock, MsStringShared};
    // IDA 0x777b02-0x777b2e: temp empty shared plus throw-guard flag; no
    // unwinding in Rust, so the guard (0x777b36-0x777b50) collapses to
    // straight-line construction.
    let mut block = MsStringBlock {
        use_count: 1,
        weak_count: 1,
        value: None,
        has_value: false,
        deleter: make_string_shared::MsStringDeleter,
    };
    // IDA 0x777b5a: fetch the deleter slot; 0x777b70: placement-new the
    // string from *a2 into the slot; 0x777b78: set the empty-flag.
    let _slot = block.deleter;
    block.value = Some(String::from(cstr));
    block.has_value = true;
    // IDA 0x777b7e-0x777b98: out pointer/count adopted, temp count released.
    MsStringShared {
        inner: SharedPtr::new(std::sync::Mutex::new(block)),
    }
}

#[doc(alias = "rbx::detail::sp_ms_deleter<std::string> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::string>,std::string>(boost::shared_ptr<std::string> const&)")]
#[doc(alias = "__ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISsEESsEEPT_RKNS_10shared_ptrIT0_EE")]
// 0x777c7c — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISsEESsEEPT_RKNS_10shared_ptrIT0_EE
pub fn stub_0x777c7c(
    shared: &make_string_shared::MsStringShared,
    want: &str,
) -> Option<make_string_shared::MsStringDeleter> {
    // IDA 0x777c80-0x777c86: null control block returns 0 (Arc is never null
    // here — the check collapses to the direct lookup below).
    // IDA 0x777c9c: vtable get_deleter(sp_ms_deleter typeinfo).
    let block = shared.inner.lock().unwrap();
    let direct = if want == make_string_shared::MS_STRING_DELETER_TYPE_NAME {
        Some(block.deleter)
    } else {
        None
    };
    // IDA 0x777ca0-0x777cd4: on miss, probe esft2_deleter_wrapper then
    // re-issue the query through it. The monomorphic block never installs
    // the wrapper, so a direct miss is final.
    direct
}

#[doc(alias = "boost::shared_ptr<std::string>::shared_ptr<std::string,rbx::detail::sp_ms_deleter<std::string>>(std::string *,rbx::detail::sp_ms_deleter<std::string>)")]
#[doc(alias = "__ZN5boost10shared_ptrISsEC2ISsN3rbx6detail13sp_ms_deleterISsEEEEPT_T0_")]
// 0x777cd8 — __ZN5boost10shared_ptrISsEC2ISsN3rbx6detail13sp_ms_deleterISsEEEEPT_T0_
pub fn stub_0x777cd8(s: String) -> make_string_shared::MsStringShared {
    use make_string_shared::{MsStringBlock, MsStringDeleter, MsStringShared};
    // IDA 0x777d06: px stored; 0x777d38: shared_count ctor builds the block.
    // IDA 0x777d0a-0x777d5a: throw-guard flag collapses (no exceptions).
    // The adopted string is owned, not inline-constructed, so has_value
    // stays false exactly like the original flag.
    let block = MsStringBlock {
        use_count: 1,
        weak_count: 1,
        value: Some(s),
        has_value: false,
        deleter: MsStringDeleter,
    };
    MsStringShared {
        inner: SharedPtr::new(std::sync::Mutex::new(block)),
    }
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::string *,rbx::detail::sp_ms_deleter<std::string>>(std::string *,rbx::detail::sp_ms_deleter<std::string>)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPSsN3rbx6detail13sp_ms_deleterISsEEEET_T0_")]
// 0x777e14 — __ZN5boost6detail12shared_countC2IPSsN3rbx6detail13sp_ms_deleterISsEEEET_T0_
pub fn stub_0x777e14() -> make_string_shared::MsStringBlock {
    // IDA 0x777e42-0x777e8e: `*a1 = 0; new 0x18; vtable; use = 1; weak = 1;
    // px = a2 (+12); empty-flag = 0 (+16)`. The pointee slot is filled later
    // by make_shared (0x777b70), so the block starts value-less.
    make_string_shared::MsStringBlock {
        use_count: 1,
        weak_count: 1,
        value: None,
        has_value: false,
        deleter: make_string_shared::MsStringDeleter,
    }
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::string *,rbx::detail::sp_ms_deleter<std::string>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSsN3rbx6detail13sp_ms_deleterISsEEED1Ev")]
// 0x777f18 — __ZN5boost6detail18sp_counted_impl_pdIPSsN3rbx6detail13sp_ms_deleterISsEEED1Ev
pub fn stub_0x777f18(block: &mut make_string_shared::MsStringBlock) {
    // IDA 0x777f2c: vtable reset (no vtable in Rust — layout is fixed).
    // IDA 0x777f2e-0x777f3c: `if (+16) { string::~string(+20); +16 = 0 }`.
    // D1 frees nothing; operator delete owns the block.
    if block.has_value {
        block.value.take();
        block.has_value = false;
    }
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::string *,rbx::detail::sp_ms_deleter<std::string>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSsN3rbx6detail13sp_ms_deleterISsEEED0Ev")]
// 0x777f44 — __ZN5boost6detail18sp_counted_impl_pdIPSsN3rbx6detail13sp_ms_deleterISsEEED0Ev
pub fn stub_0x777f44(block: make_string_shared::MsStringBlock) {
    // IDA 0x777f7c-0x777fac: D1 body, then 0x777fb2 `operator delete(a1)`.
    // By-value drop is dtor-then-free in the same order.
    let mut block = block;
    stub_0x777f18(&mut block);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::string *,rbx::detail::sp_ms_deleter<std::string>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSsN3rbx6detail13sp_ms_deleterISsEEE7disposeEv")]
// 0x777ffc — __ZN5boost6detail18sp_counted_impl_pdIPSsN3rbx6detail13sp_ms_deleterISsEEE7disposeEv
pub fn stub_0x777ffc(block: &mut make_string_shared::MsStringBlock) -> u8 {
    // IDA 0x778002-0x778014: `r = +16; if (r) { dtor(+20); +16 = 0; return 0 }
    // return r`. BUG (preserved): both arms return 0 — the flag is
    // unreadable from the result; only the side effects are observable.
    if block.has_value {
        block.value.take();
        block.has_value = false;
        return 0;
    }
    0
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::string *,rbx::detail::sp_ms_deleter<std::string>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSsN3rbx6detail13sp_ms_deleterISsEEE11get_deleterERKSt9type_info")]
// 0x778018 — __ZN5boost6detail18sp_counted_impl_pdIPSsN3rbx6detail13sp_ms_deleterISsEEE11get_deleterERKSt9type_info
pub fn stub_0x778018(
    block: &make_string_shared::MsStringBlock,
    type_name: &str,
) -> Option<make_string_shared::MsStringDeleter> {
    // IDA 0x77801c-0x77802e: `out = this+16; if (ti->name !=
    // "N3rbx6detail13sp_ms_deleterISsEE") return 0; return out`.
    if type_name == make_string_shared::MS_STRING_DELETER_TYPE_NAME {
        Some(block.deleter)
    } else {
        None
    }
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::string *,rbx::detail::sp_ms_deleter<std::string>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSsN3rbx6detail13sp_ms_deleterISsEEE19get_untyped_deleterEv")]
// 0x778030 — __ZN5boost6detail18sp_counted_impl_pdIPSsN3rbx6detail13sp_ms_deleterISsEEE19get_untyped_deleterEv
pub fn stub_0x778030(block: &make_string_shared::MsStringBlock) -> make_string_shared::MsStringDeleter {
    // IDA 0x778032: `return this+16` — unconditional.
    block.deleter
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::list6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SI_SK_SN_")]
// 0x778034 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SI_SK_SN_
pub fn stub_0x778034(
    debugger: debugger_hook::DebuggerTarget,
    hook: debugger_hook::DebuggerHookFn,
    adjust: debugger_hook::MemberFnAdjust,
    predicate: debugger_hook::HookPredicate,
    enabled: bool,
    source: SharedPtr<String>,
) -> debugger_hook::HookBindImage {
    // IDA 0x77805a-0x778064: temp bool-function + assign_to_own(predicate).
    let pred_tmp = predicate.clone();
    // IDA 0x7780a2: storage6 captures (debugger, arg1/arg2 call-time
    // placeholders, pred_tmp, enabled, source); 0x7780ae: clear temp.
    // (The free `bind(mf5, list6)` step folds hook/adjust in here.)
    stub_0x778108(debugger, hook, adjust, pred_tmp, enabled, source)
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::storage6(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SI_SK_SN_")]
// 0x778108 — __ZN5boost3_bi8storage6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEC2ES7_S9_SA_SI_SK_SN_
pub fn stub_0x778108(
    debugger: debugger_hook::DebuggerTarget,
    hook: debugger_hook::DebuggerHookFn,
    adjust: debugger_hook::MemberFnAdjust,
    predicate: debugger_hook::HookPredicate,
    enabled: bool,
    source: SharedPtr<String>,
) -> debugger_hook::HookBindImage {
    // IDA 0x77812e-0x778138: temp + assign_to_own(predicate).
    let pred_tmp = predicate.clone();
    // IDA 0x778172: storage5 captures the head five; 0x778188: source
    // reference stored at +24; 0x778180: clear temp.
    debugger_hook::HookBindImage {
        hook,
        adjust,
        args: debugger_hook::HookBindArgs {
            debugger,
            predicate: pred_tmp,
            enabled,
            source,
        },
    }
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>>::storage5(boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SA_SI_SK_")]
// 0x7781e0 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SA_SI_SK_
pub fn stub_0x7781e0(
    debugger: debugger_hook::DebuggerTarget,
    predicate: debugger_hook::HookPredicate,
    enabled: bool,
) -> debugger_hook::HookBindArgs5 {
    // IDA 0x77820e-0x778210: temp + assign_to_own(predicate).
    let pred_tmp = predicate.clone();
    // IDA 0x77821e-0x77824e: head words stored, predicate installed at +4
    // (the `a1[1] = 0` scratch init is overwritten by assign_to_own);
    // 0x77825a: clear temp; 0x778262: enabled stored at +20.
    debugger_hook::HookBindArgs5 {
        debugger,
        predicate: pred_tmp,
        enabled,
    }
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::swap(boost::function2<void,lua_State *,lua_Debug *>&)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE4swapERS5_")]
// 0x7782bc — __ZN5boost9function2IvP9lua_StateP9lua_DebugE4swapERS5_
pub fn stub_0x7782bc(a: &mut debugger_hook::HookSlot, b: &mut debugger_hook::HookSlot) {
    use debugger_hook::HookSlot;
    // IDA 0x77830a: self-swap guard.
    if std::ptr::eq(a as *const HookSlot, b as *const HookSlot) {
        return;
    }
    let mut tmp = HookSlot::Empty;
    stub_0x778398(&mut tmp, a);
    stub_0x778398(a, b);
    stub_0x778398(b, &mut tmp);
    // IDA 0x77833e: clear temp (already empty after the third move).
    drop(tmp);
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::move_assign(boost::function2<void,lua_State *,lua_Debug *>&)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE11move_assignERS5_")]
// 0x778398 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE11move_assignERS5_
pub fn stub_0x778398(dst: &mut debugger_hook::HookSlot, src: &mut debugger_hook::HookSlot) {
    use debugger_hook::HookSlot;
    // IDA 0x7783e6: self-assign guard.
    if std::ptr::eq(dst as *const HookSlot, src as *const HookSlot) {
        return;
    }
    // IDA 0x7783e8-0x77842c: steal the source image and null it (`*a2 = 0`).
    let taken = std::mem::replace(src, HookSlot::Empty);
    match taken {
        HookSlot::Bound(bind) => {
            // IDA 0x7783f2-0x778426: small-inline copy vs heap manager-move
            // collapse to one move — the 0x24 image is always boxed here.
            *dst = HookSlot::Bound(bind);
        }
        HookSlot::Empty => {
            // IDA 0x77840c: empty source clears the destination.
            *dst = HookSlot::Empty;
        }
    }
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
// 0x77849c — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE
pub fn stub_0x77849c(slot: &mut debugger_hook::HookSlot, bind: &debugger_hook::HookBindImage) {
    // IDA 0x7784c0-0x7784ec: bind image split into a temp (predicate via
    // assign_to_own at 0x7784da, refwords copied at 0x7784e0-0x7784ec).
    let tmp = bind.clone();
    // IDA 0x778520: function2 ctor installs it; 0x77852c: clear temp.
    stub_0x778588(slot, &tmp);
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
// 0x778588 — __ZN5boost9function2IvP9lua_StateP9lua_DebugEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE
pub fn stub_0x778588(slot: &mut debugger_hook::HookSlot, bind: &debugger_hook::HookBindImage) {
    use debugger_hook::HookSlot;
    // IDA 0x7785aa: `*a1 = 0` — the slot is zeroed before install.
    *slot = HookSlot::Empty;
    // IDA 0x7785b0-0x7785da: split into a temp; 0x77860e: assign_to;
    // 0x77861a: clear temp.
    let tmp = bind.clone();
    stub_0x778674(slot, &tmp);
}

#[doc(alias = "void boost::function2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>&)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEEvT_")]
// 0x778674 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS_8functionIFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS7_5list6INS7_5valueIPSD_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEEvT_
pub fn stub_0x778674(slot: &mut debugger_hook::HookSlot, bind: &debugger_hook::HookBindImage) {
    // IDA 0x7786aa-0x7786c4: temp predicate via assign_to_own (+12) plus the
    // +28/+32 refword copies.
    let tmp = bind.clone();
    // IDA 0x778706: basic_vtable2 assign installs the copy; 0x778712:
    // clear temp; 0x778716: `*a1` records the vtable marker (the Bound
    // state records it here).
    stub_0x7787b0(slot, &tmp);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEE6manageERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeE")]
// 0x778770 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEE6manageERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeE
pub fn stub_0x778770(
    slot: &mut debugger_hook::HookSlot,
    op: debugger_hook::HookFunctorOp,
    query_name: &str,
) -> debugger_hook::HookManageEffect {
    use debugger_hook::{HookFunctorOp, HookManageEffect, HOOK_BIND_TYPE_NAME};
    // IDA 0x778772-0x77878a: op 4 returns the bind_t typeinfo directly
    // without delegating to manager.
    if op == HookFunctorOp::GetType {
        return HookManageEffect::TypeName(HOOK_BIND_TYPE_NAME);
    }
    stub_0x778c34(slot, op, query_name)
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>,void,lua_State *,lua_Debug *>::invoke(boost::detail::function::function_buffer &,lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_")]
// 0x77878c — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEvSB_SD_E6invokeERNS1_15function_bufferESB_SD_
// was: invoker2::invoke unpacks the buffer and calls the bound functor.
// (Decompile unavailable at this EA; grounded in the disasm instead:
// buffer image at +8, list2{L, ar} built on stack, list6::operator() call.)
pub fn stub_0x77878c(
    slot: &debugger_hook::HookSlot,
    l: debugger_hook::LuaStatePtr,
    ar: debugger_hook::LuaDebugPtr,
) {
    use debugger_hook::HookSlot;
    // Disasm 0x778798-0x7787a2: image loaded from the buffer (+8);
    // 0x778792-0x7787a0: list2{L, ar} built on the stack; 0x7787a6 BLX into
    // list6::operator(); 0x7787aa-0x7787ac: epilogue. Empty slots hold no
    // image, so there is nothing to invoke.
    if let HookSlot::Bound(bind) = slot {
        stub_0x778a64(bind, l, ar);
    }
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>&,boost::detail::function::function_buffer&)")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEbT_RNS1_15function_bufferE")]
// 0x7787b0 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0x7787b0(slot: &mut debugger_hook::HookSlot, bind: &debugger_hook::HookBindImage) -> bool {
    use debugger_hook::HookSlot;
    // IDA 0x7787d0-0x778802: image fields copied into a temp (predicate via
    // assign_to_own at 0x7787f0, refwords at 0x7787f6-0x778802).
    let tmp = bind.clone();
    // IDA 0x778838: vtable assign installs the copy; 0x778844: clear temp;
    // 0x778864: return 1.
    *slot = HookSlot::Bound(Box::new(tmp));
    true
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>&,boost::detail::function::function_buffer&,boost::detail::function::function_obj_tag)")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x7788a0 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0x7788a0(slot: &mut debugger_hook::HookSlot, bind: &debugger_hook::HookBindImage) -> bool {
    // IDA 0x7788d4-0x7788ee: temp image (predicate assign_to_own at +12,
    // refwords at +28/+32); 0x778922: heap install via assign_functor.
    let boxed = stub_0x778988(bind);
    *slot = debugger_hook::HookSlot::Bound(boxed);
    // IDA 0x77892e: clear temp; 0x77894e: return 1.
    true
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,lua_State *,lua_Debug *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>&,boost::detail::function::function_buffer&,mpl_::bool_<false>)")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x778988 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StateP9lua_DebugE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES4_S6_NS_8functionIFbS4_S6_EEERbRNS_10shared_ptrISsEEEENS9_5list6INS9_5valueIPSF_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperIbEENSW_ISL_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_0x778988(bind: &debugger_hook::HookBindImage) -> Box<debugger_hook::HookBindImage> {
    // IDA 0x7789b0: `new 0x24`; 0x7789b8-0x778a14: qword/word field copies
    // (the first-qword store repeats at 0x778a14 — harmless, one clone
    // here); 0x7789fe: predicate assign_to_own at +12 (Arc clone);
    // 0x778a0a-0x778a10: +28/+32 refword copies; 0x778a1c: out = new image.
    Box::new(bind.clone())
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>::operator()<boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string>&>,boost::_bi::list2<lua_State *&,lua_Debug *&>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string>&> &,boost::_bi::list2<lua_State *&,lua_Debug *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SD_SF_SH_RbRSM_EENS0_5list2IRSD_RSF_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0x778a64 — __ZN5boost3_bi5list6INS0_5valueIPN3RBX9Scripting14ScriptDebuggerEEENS_3argILi1EEENS8_ILi2EEENS2_INS_8functionIFbP9lua_StateP9lua_DebugEEEEENS_17reference_wrapperIbEENSJ_INS_10shared_ptrISsEEEEEclINS_4_mfi3mf5IvS5_SD_SF_SH_RbRSM_EENS0_5list2IRSD_RSF_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0x778a64(bind: &debugger_hook::HookBindImage, l: debugger_hook::LuaStatePtr, ar: debugger_hook::LuaDebugPtr) {
    // IDA 0x778a88-0x778a90: call args (L, ar) resolved from the list2,
    // bound debugger from the list6 head.
    // IDA 0x778a9c: bound predicate assign_to_own into a temp copy.
    let pred_tmp = bind.args.predicate.clone();
    // IDA 0x778ae2: mf5::operator()(obj, L, ar, pred_tmp, enabled, source).
    stub_0x778b48(
        bind.hook,
        &bind.adjust,
        bind.args.debugger,
        l,
        ar,
        pred_tmp,
        bind.args.enabled,
        bind.args.source.clone(),
    );
    // IDA 0x778aee: clear temp (moved temp drops here).
}

#[doc(alias = "boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>::operator()(RBX::Scripting::ScriptDebugger*,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbS6_S8_EEERbRNS_10shared_ptrISsEEEclEPS4_S6_S8_SB_SC_SF_")]
// 0x778b48 — __ZNK5boost4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbS6_S8_EEERbRNS_10shared_ptrISsEEEclEPS4_S6_S8_SB_SC_SF_
pub fn stub_0x778b48(
    hook: debugger_hook::DebuggerHookFn,
    adjust: &debugger_hook::MemberFnAdjust,
    target: debugger_hook::DebuggerTarget,
    l: debugger_hook::LuaStatePtr,
    ar: debugger_hook::LuaDebugPtr,
    pred: debugger_hook::HookPredicate,
    enabled: bool,
    source: SharedPtr<String>,
) {
    // IDA 0x778b74-0x778b78: member pointer + encoding resolved.
    // IDA 0x778b98: dispatch slot for the virtual path.
    let _slot = adjust.dispatch_slot(target.0);
    // IDA 0x778ba4-0x778ba8: virtual-bit set indirects through the slot.
    // Monomorphic port: the resolved entry is `hook` either way.
    let _ = adjust.is_virtual_thunk();
    // IDA 0x778bb0-0x778bba: predicate assign_to_own into a local copy.
    let pred_local = pred.clone();
    // IDA 0x778bd2: the member call; 0x778bdc: clear local.
    hook(target, l, ar, pred_local, enabled, source);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,boost::shared_ptr<std::string> &>,boost::_bi::list6<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<bool ()(lua_State *,lua_Debug *)>>,boost::reference_wrapper<bool>,boost::reference_wrapper<boost::shared_ptr<std::string>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEE7managerERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x778c34 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbSB_SD_EEERbRNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEE7managerERKNS1_15function_bufferERS10_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0x778c34(
    slot: &mut debugger_hook::HookSlot,
    op: debugger_hook::HookFunctorOp,
    query_name: &str,
) -> debugger_hook::HookManageEffect {
    use debugger_hook::{HookFunctorOp, HookManageEffect, HookSlot, HOOK_BIND_TYPE_NAME};
    match op {
        // IDA 0x778ca8-0x778cf2 case 0: `new 0x24`, field-by-field copy
        // with assign_to_own at +12 and the +28/+32 refwords.
        HookFunctorOp::CloneBind => match slot {
            HookSlot::Bound(bind) => HookManageEffect::Cloned(Some((**bind).clone())),
            HookSlot::Empty => HookManageEffect::Cloned(None),
        },
        // IDA 0x778cf8-0x778cfc case 1: move the image pointer, null source.
        HookFunctorOp::MoveBind => {
            let taken = std::mem::replace(slot, HookSlot::Empty);
            match taken {
                HookSlot::Bound(bind) => HookManageEffect::Moved(Some(*bind)),
                HookSlot::Empty => HookManageEffect::Moved(None),
            }
        }
        // IDA 0x778d02-0x778d1e case 2: clear the inner bool-function
        // (+12), operator delete the image; empty stays empty.
        HookFunctorOp::DestroyBind => {
            *slot = HookSlot::Empty;
            HookManageEffect::Destroyed
        }
        // IDA 0x778d36-0x778d40 case 3: strcmp the queried type name
        // against the bind_t name; match stores the image, else null.
        HookFunctorOp::CheckType => {
            HookManageEffect::TypeMatch(query_name == HOOK_BIND_TYPE_NAME)
        }
        // IDA 0x778c92-0x778c96 default: out = typeid bind_t.
        HookFunctorOp::GetType => HookManageEffect::TypeName(HOOK_BIND_TYPE_NAME),
    }
}

#[cfg(test)]
mod kk_tests {
    use super::debugger_hook::*;
    use super::make_string_shared::*;
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    static HOOK_CALLS: Mutex<Vec<(usize, usize, usize, bool, bool, String)>> =
        Mutex::new(Vec::new());
    static PRED_CALLS: AtomicUsize = AtomicUsize::new(0);

    fn rec_hook(
        target: DebuggerTarget,
        l: LuaStatePtr,
        ar: LuaDebugPtr,
        pred: HookPredicate,
        enabled: bool,
        source: SharedPtr<String>,
    ) {
        let r = pred(l, ar);
        HOOK_CALLS
            .lock()
            .unwrap()
            .push((target.0, l, ar, enabled, r, source.to_string()));
    }

    fn test_pred() -> HookPredicate {
        SharedPtr::new(|l: LuaStatePtr, ar: LuaDebugPtr| {
            PRED_CALLS.fetch_add(1, Ordering::SeqCst);
            l == 7 && ar == 9
        })
    }

    fn sample_image() -> HookBindImage {
        stub_0x778034(
            DebuggerTarget(0x42),
            rec_hook,
            MemberFnAdjust {
                encoding: 0,
                vtable_offset: 0,
            },
            test_pred(),
            true,
            SharedPtr::new(String::from("chunk")),
        )
    }

    #[test]
    fn ms_string_block_lifecycle() {
        // IDA 0x777e14: fresh block, both counts 1, flag clear.
        let b = stub_0x777e14();
        assert_eq!((b.use_count, b.weak_count, b.has_value), (1, 1, false));
        assert!(b.value.is_none());
        // IDA 0x777ad4: make_shared constructs the inline string.
        let s = stub_0x777ad4("hello");
        assert_eq!(s.inner.lock().unwrap().value.as_deref(), Some("hello"));
        assert!(s.inner.lock().unwrap().has_value);
        // IDA 0x777c7c: deleter lookup hits the exact type name only.
        assert!(stub_0x777c7c(&s, MS_STRING_DELETER_TYPE_NAME).is_some());
        assert!(stub_0x777c7c(&s, "Nope").is_none());
        // IDA 0x778018/0x778030: same rule directly on the block.
        assert_eq!(
            stub_0x778018(&s.inner.lock().unwrap(), MS_STRING_DELETER_TYPE_NAME),
            Some(MsStringDeleter)
        );
        assert!(stub_0x778018(&s.inner.lock().unwrap(), "Nope").is_none());
        assert_eq!(stub_0x778030(&s.inner.lock().unwrap()), MsStringDeleter);
        // IDA 0x777ffc: dispose drops the string, clears the flag, returns 0.
        assert_eq!(stub_0x777ffc(&mut s.inner.lock().unwrap()), 0);
        assert!(!s.inner.lock().unwrap().has_value);
        assert!(s.inner.lock().unwrap().value.is_none());
        // IDA 0x777f18/0x777f44: D1 then D0 run without freeing live data.
        let mut b2 = stub_0x777e14();
        b2.value = Some(String::from("x"));
        b2.has_value = true;
        stub_0x777f18(&mut b2);
        assert!(!b2.has_value && b2.value.is_none());
        stub_0x777f44(b2);
        // IDA 0x777cd8: adopted strings skip the inline flag.
        let s2 = stub_0x777cd8(String::from("adopted"));
        assert_eq!(
            s2.inner.lock().unwrap().value.as_deref(),
            Some("adopted")
        );
        assert!(!s2.inner.lock().unwrap().has_value);
    }

    #[test]
    fn hook_bind_invoke_end_to_end() {
        HOOK_CALLS.lock().unwrap().clear();
        PRED_CALLS.store(0, Ordering::SeqCst);
        let img = sample_image();
        assert!(img.args.enabled);
        assert_eq!(img.args.source.as_str(), "chunk");
        // IDA 0x778588: ctor installs; 0x77878c: invoker dispatches.
        let mut slot = HookSlot::Empty;
        stub_0x778588(&mut slot, &img);
        assert!(matches!(slot, HookSlot::Bound(_)));
        stub_0x77878c(&slot, 7, 9);
        let calls = HOOK_CALLS.lock().unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0],
            (0x42, 7, 9, true, true, String::from("chunk"))
        );
        assert_eq!(PRED_CALLS.load(Ordering::SeqCst), 1);
        drop(calls);
        // Empty slots hold no image (invoker no-op).
        stub_0x77878c(&HookSlot::Empty, 7, 9);
        assert_eq!(HOOK_CALLS.lock().unwrap().len(), 1);
        // IDA 0x778b74-0x778ba8: member-pointer adjust math.
        let adj = MemberFnAdjust {
            encoding: 0b11,
            vtable_offset: 0,
        };
        assert!(adj.is_virtual_thunk());
        assert_eq!(adj.dispatch_slot(0x1000), 0x1001);
        assert!(!MemberFnAdjust {
            encoding: 0,
            vtable_offset: 0
        }
        .is_virtual_thunk());
    }

    #[test]
    fn hook_manager_clone_move_destroy_cycle() {
        let img = sample_image();
        let mut slot = HookSlot::Empty;
        // IDA 0x77849c: ctor installs the image.
        stub_0x77849c(&mut slot, &img);
        assert!(matches!(slot, HookSlot::Bound(_)));
        // IDA 0x778ca8-0x778cf2: clone duplicates the bound image.
        match stub_0x778c34(&mut slot, HookFunctorOp::CloneBind, "") {
            HookManageEffect::Cloned(Some(c)) => {
                assert_eq!(c.args.source.as_str(), "chunk")
            }
            _ => panic!("clone must duplicate the bound image"),
        }
        // IDA 0x778d36-0x778d40: check-type compares the queried name.
        match stub_0x778c34(&mut slot, HookFunctorOp::CheckType, HOOK_BIND_TYPE_NAME) {
            HookManageEffect::TypeMatch(true) => {}
            _ => panic!("check-type must match the monomorphic slot"),
        }
        match stub_0x778c34(&mut slot, HookFunctorOp::CheckType, "Bogus") {
            HookManageEffect::TypeMatch(false) => {}
            _ => panic!("check-type must reject foreign names"),
        }
        // IDA 0x778772: get-type bypasses the manager switch.
        match stub_0x778770(&mut slot, HookFunctorOp::GetType, "") {
            HookManageEffect::TypeName(n) => assert_eq!(n, HOOK_BIND_TYPE_NAME),
            _ => panic!("get-type must return the bind_t name"),
        }
        // IDA 0x778864/0x77894e: vtable installs report success.
        assert!(stub_0x7787b0(&mut slot, &img));
        assert!(stub_0x7788a0(&mut slot, &img));
        // IDA 0x7789b0-0x778a1c: heap clone carries the bound receiver.
        let boxed = stub_0x778988(&img);
        assert_eq!(boxed.args.debugger, DebuggerTarget(0x42));
        // IDA 0x778cf8-0x778cfc: move transfers and nulls the source.
        let mut dst = HookSlot::Empty;
        stub_0x778398(&mut dst, &mut slot);
        assert!(matches!(dst, HookSlot::Bound(_)));
        assert!(matches!(slot, HookSlot::Empty));
        // IDA 0x77840c: move from empty clears the destination.
        stub_0x778398(&mut dst, &mut slot);
        assert!(matches!(dst, HookSlot::Empty));
        // IDA 0x77831a-0x77833e: swap exchanges the images.
        let mut a = HookSlot::Empty;
        stub_0x778674(&mut a, &img);
        let mut b = HookSlot::Empty;
        stub_0x7782bc(&mut a, &mut b);
        assert!(matches!(a, HookSlot::Empty));
        assert!(matches!(b, HookSlot::Bound(_)));
        // IDA 0x7781e0: storage5 fragment keeps debugger/predicate/enabled.
        let frag = stub_0x7781e0(DebuggerTarget(7), test_pred(), false);
        assert_eq!(frag.debugger, DebuggerTarget(7));
        assert!(!frag.enabled);
        assert!(!frag.predicate.as_ref()(1, 2));
    }
}
