//! Auto-generated skeletons for rbx-script — script gap filler EA asc 0x31bf0..0x45aa0
//! Filter: Script|Lua|LuaBridge|Yield|ProtectedString (4921 filtered, 0 remaining not yet in script — gap filler global EA asc distinct not yet in crates/script/src)
//! Source: ida/export.json (85545 funcs, base 0x4000, size 0x13a8efc)
//! Batch: +120 stubs | range 0x31bf0..0x45aa0 | script 23461->23581 total (EA-sorted asc distinct not yet in crates/script/src, rbx_core::SharedPtr not boost, // 0xADDR mangled + #[doc(alias)] + todo!("0xADDR"))
//! Remaining not in script before batch: 62084 -> after: 61964 (filtered Script|Lua exhausted, global gap filler EA asc)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use std::sync::LazyLock;
/// `bind_t<openUrlWindow,id,SEL,string>` typeinfo answer for the
/// `functor_manager` glue (IDA 0x32270, cf. reflection bg_8).
pub const BIND_OPEN_URL_TYPEINFO: &str = "bind_t<openUrlWindow,id,SEL,string>";
/// `bind_t<childAdded,id,SEL,SharedPtr<Instance>>` typeinfo answer for the
/// `functor_manager` glue (IDA 0x31cd0, cf. reflection bg_7).
pub const BIND_CHILD_ADDED_TYPEINFO: &str = "bind_t<childAdded,id,SEL,SharedPtr<Instance>>";
/// Opaque `signal<string>` static mutex handle (IDA 0x31ec8, cf. reflection bg_7).
static SIGNAL_STR_MUTEX: LazyLock<u32> = LazyLock::new(|| 1);
/// `bind_t<RobloxView *, signed char>` typeinfo answer for the
/// `functor_manager` glue (IDA 0x33470).
pub const BIND_ROBLOXVIEW_SCHAR_TYPEINFO: &str = "bind_t<RobloxView,signed char>";
/// `bind_t<objc_object *, objc_selector *>` typeinfo answer for the
/// `functor_manager` glue (IDA 0x334dc).
pub const BIND_OBJC_VOID_TYPEINFO: &str = "bind_t<objc_object,objc_selector>";
/// `bind_t<PlaceLauncher *, string x3>` typeinfo answer for the
/// `functor_manager` glue (IDA 0x34b40).
pub const BIND_PLACELAUNCHER_TYPEINFO: &str = "bind_t<PlaceLauncher,string,string,string>";

// 0x31bf0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x31bf0]")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x31bf0() {
    // IDA 0x31bf0: D0 deleting dtor — storage release only (`B.W __ZdlPv`).
    // `Arc` drop glue covers it; no explicit body.
}

// 0x31bf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x31bf4() {
    // IDA 0x31bf4: `dispose` runs `predelete` then the deleter
    // virtual-delete (cf. reflection bg_7 0x31bf4). `Arc` drop glue covers
    // it; no explicit body.
}

// 0x31c14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x31c14(type_matches: bool) -> usize {
    // IDA 0x31c14: `get_deleter` returns the deleter address on typeinfo
    // match, else null (cf. reflection bg_7). The nonzero cookie stands in
    // for the address.
    if type_matches { 1 } else { 0 }
}

// 0x31c2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x31c2c() -> usize {
    // IDA 0x31c2c: `get_untyped_deleter` answers null (`MOVS;BX`, cf.
    // reflection bg_7). Returns 0.
    0
}

// 0x31c30 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv")]
pub fn stub_0x31c30() -> bool {
    // IDA 0x31c30: `NonFactoryProduct<Instance,sLoginService>::
    // isNullClassName` — `sLoginService` derefs to "LoginService" (0x11f55fc,
    // non-null; cf. `rbx_datamodel::instance::stub_0x31c30`). Never null.
    false
}

// 0x31cd0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x31cd0(get_typeinfo: bool) -> &'static str {
    // IDA 0x31cd0: `functor_manager<bind_t<childAdded...>>::manage` answers
    // op 4 with the `bind_t` typeinfo (cf. reflection bg_7). Other ops are
    // vtable glue.
    if get_typeinfo {
        BIND_CHILD_ADDED_TYPEINFO
    } else {
        ""
    }
}

// 0x31d30 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")]
pub fn stub_0x31d30() {
    // IDA 0x31d30: `void_function_obj_invoker1<bind_t<childAdded...>>::
    // invoke` runs the bound `childAdded:` slot over `list3::operator()`
    // (0x31d48, cf. reflection bg_7). Closure-call glue; no explicit body.
}

// 0x31d48 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int, int, sp_counted_base **), const shared_count **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x31d48() {
    // IDA 0x31d48: `list3<id,SEL,arg<1>>::operator()` unwraps target +
    // selector + instance and invokes `childAdded:` (cf. reflection bg_7).
    // Closure-call glue; no explicit body.
}

// 0x31e24 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsEE4slotEEaSEPS6_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsEE4slotEEaSEPS6_")]
pub fn stub_0x31e24() {
    // IDA 0x31e24: `intrusive_ptr<signal<string>::slot>::operator=` swaps
    // the slot with add_ref/release (cf. reflection bg_7). `Arc`
    // assignment glue covers it; no explicit body.
}

// 0x31ec8 — __ZN3rbx7signals6signalIFvSsEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsEE24safe_static_do_get_mutexEv")]
pub fn stub_0x31ec8() -> u32 {
    // IDA 0x31ec8: `signal<string>::safe_static_do_get_mutex` one-shots the
    // static signal mutex (cf. reflection bg_7). The opaque handle records
    // once.
    *SIGNAL_STR_MUTEX
}

// 0x31fc0 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::callable<rbx::signals::signal<void ()(std::string)>*>(boost::function<void ()(std::string)> const&,rbx::signals::signal<void ()(std::string)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_")]
pub fn stub_0x31fc0() {
    // IDA 0x31fc0: `callable<signal<string>::slot,function,1>::callable`
    // copies the function via `assign_to_own` (cf. reflection bg_7).
    // Slot-construction glue; no explicit body.
}

// 0x320bc — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost8functionIS2_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::function<void ()(std::string)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost8functionIS2_EEED0Ev")]
pub fn stub_0x320bc() {
    // IDA 0x320bc: D0 deleting dtor: reset vtables, destroy members,
    // `operator delete` (cf. reflection bg_7). `Arc` Drop glue covers it;
    // no explicit body.
}

// 0x32194 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_E4callESs
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::call(std::string)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_E4callESs")]
pub fn stub_0x32194() {
    // IDA 0x32194: non-virtual thunk to `callable<...>::call(string)` —
    // this/arg-adjust + tail-call (cf. reflection bg_7). Static dispatch
    // needs no thunk; no explicit body.
}

// 0x3219c — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev")]
pub fn stub_0x3219c() {
    // IDA 0x3219c: D1 complete-object dtor: reset vtable, destroy owned
    // member. Rust Drop glue covers it; no explicit body.
}

// 0x32270 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x32270(get_typeinfo: bool) -> &'static str {
    // IDA 0x32270: `functor_manager<bind_t<openUrl...>>::manage` answers op
    // 4 with the `bind_t` typeinfo (cf. reflection bg_8). Other ops are
    // vtable glue.
    if get_typeinfo {
        BIND_OPEN_URL_TYPEINFO
    } else {
        ""
    }
}

// 0x322d0 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs")]
pub fn stub_0x322d0() {
    // IDA 0x322d0: `void_function_obj_invoker1<bind_t<openUrl...>>::invoke`
    // runs the bound `openUrlWindow:` slot (cf. reflection bg_8).
    // Closure-call glue; no explicit body.
}

// 0x322e8 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_SsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int, int, int *), const std::string **)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,std::string),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,std::string) &,boost::_bi::list1<std::string &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_SsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x322e8() {
    // IDA 0x322e8: `list3<id,SEL,arg<1>>::operator()` copies the string arg
    // and invokes `openUrlWindow:` (cf. reflection bg_8). Closure-call
    // glue; no explicit body.
}

// 0x32408 — __ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv")]
pub fn stub_0x32408() {
    // IDA 0x32408: `Name::callDoDeclare<sGuiService>` forwards to
    // `doDeclare` (1 insn, cf. reflection bg_8). Trampoline glue; no
    // explicit body.
}

// 0x3240c — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10GuiServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GuiService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10GuiServiceEEEvv")]
pub fn stub_0x3240c() {
    // IDA 0x3240c: `ServiceProvider::callDoGetClassIndex<GuiService>`
    // forwards to `doGetClassIndex` (1 insn, cf. reflection bg_8).
    // Trampoline glue; no explicit body.
}

// 0x32410 — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x32410(constructed: bool) -> &'static str {
    // IDA 0x32410: `FactoryProduct<TaskSchedulerSettings,...>::Creator::
    // getClassName` asserts `wasConstructed()` and returns the declared
    // `TaskSchedulerSettings` name (cf. reflection bg_8).
    assert!(constructed, "wasConstructed() (IDA 0x32410)");
    "TaskSchedulerSettings"
}

// 0x3247c — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x3247c(constructed: bool, create_ok: bool) -> bool {
    // IDA 0x3247c: `FactoryProduct<TaskSchedulerSettings,...>::Creator::
    // create` asserts `wasConstructed()`, runs `Creatable::create` and
    // returns the new instance (cf. reflection bg_8). Factory glue;
    // presence collapses to `bool`.
    assert!(constructed, "wasConstructed() (IDA 0x3247c)");
    create_ok
}

// 0x324fc — __ZN5boost10shared_ptrIN3RBX21TaskSchedulerSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int(void)
#[doc(alias = "boost::shared_ptr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX21TaskSchedulerSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x324fc() {
    // IDA 0x324fc: `shared_ptr<TaskSchedulerSettings>::shared_ptr<...,
    // Creatable::Deleter>` stores the pointer + deleter (cf. reflection
    // bg_8). `Arc` construction glue covers it; no explicit body.
}

// 0x32520 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_21TaskSchedulerSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TaskSchedulerSettings,RBX::TaskSchedulerSettings>(boost::shared_ptr<RBX::TaskSchedulerSettings> const*,RBX::TaskSchedulerSettings *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_21TaskSchedulerSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x32520() {
    // IDA 0x32520: `enable_shared_from_this<DescribedBase>::
    // _internal_accept_owner` — weak-owner install. `SharedPtr`/`Weak`
    // covers it; no explicit body.
}

// 0x325fc — __ZN5boost6detail12shared_countC2IPN3RBX21TaskSchedulerSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX21TaskSchedulerSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x325fc() {
    // IDA 0x325fc: `shared_count` ctor allocates the control block (cf.
    // reflection bg_8). Refcount owned by `SharedPtr` (`Arc`); no explicit
    // body.
}

// 0x326fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x326fc() {
    // IDA 0x326fc: D1 complete-object dtor, empty. `Arc` drop glue covers
    // it; no explicit body.
}

// 0x32700 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x32700() {
    // IDA 0x32700: `dispose` runs `Instance::predelete` then deletes (cf.
    // reflection bg_8 0x32700). `Arc` drop glue covers it; no explicit
    // body.
}

// 0x32720 — __ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v")]
pub fn stub_0x32720() {
    // IDA 0x32720: `Name::declare<sTaskSchedulerSettings>` one-shots the
    // class-name declaration (`call_once`, cf. reflection bg_8).
    // Idempotent declare glue; no explicit body.
}

// 0x32764 — __ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv")]
pub fn stub_0x32764() {
    // IDA 0x32764: `Name::callDoDeclare<sTaskSchedulerSettings>` forwards
    // to `doDeclare` (1 insn, cf. reflection bg_8). Trampoline glue; no
    // explicit body.
}

// 0x327d4 — __ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEES2_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<RobloxView *,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,RobloxView *,boost::shared_ptr<RBX::Game>,RobloxView *,boost::shared_ptr<RBX::Game>>(void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),RobloxView *,boost::shared_ptr<RBX::Game>)")]
#[doc(alias = "__ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEES2_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_")]
pub fn stub_0x327d4() {
    // IDA 0x327d4: bind thunk; binds are plain closures (cf. generated_166
    // bind family) — carrier no-op.
}

// 0x328bc — __ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_")]
pub fn stub_0x328bc() {
    // IDA 0x328bc: bind argument-list packing; captures fold into closures
    // (cf. generated_166 bind family) — carrier no-op.
}

// 0x32984 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x32984() {
    // IDA 0x32984: function-bind ctor/assign thunk; binds are plain
    // closures — carrier no-op.
}

// 0x32a68 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x32a68() {
    // IDA 0x32a68: function-bind ctor/assign thunk; binds are plain
    // closures — carrier no-op.
}

// 0x32b50 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)")]
#[doc(alias = "__ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEEvT_")]
pub fn stub_0x32b50() {
    // IDA 0x32b50: function-bind ctor/assign thunk; binds are plain
    // closures — carrier no-op.
}

// 0x32c48 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x32c48() {
    // IDA 0x32c48: functor_manager thunk; closure buffer ops fold into
    // Box<dyn Fn> — carrier no-op.
}

// 0x32c64 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_0x32c64() {
    // IDA 0x32c64: invoker thunk; dispatches a stored closure — carrier
    // no-op.
}

// 0x32c78 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x32c78() {
    // IDA 0x32c78: vtable assign thunk; closure buffer ops fold into
    // Box<dyn Fn> — carrier no-op.
}

// 0x32d60 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x32d60() {
    // IDA 0x32d60: vtable assign thunk; closure buffer ops fold into
    // Box<dyn Fn> — carrier no-op.
}

// 0x32e74 — __ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x32e74() {
    // IDA 0x32e74: bind argument-list dispatch; captures fold into closures
    // — carrier no-op.
}

// 0x32f4c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x32f4c() {
    // IDA 0x32f4c: functor_manager thunk; closure buffer ops fold into
    // Box<dyn Fn> — carrier no-op.
}
// 0x33080 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEENS1_26device_close_all_operationIS5_EENSD_ISB_EEEENS1_14execute_traitsIT_NS_9result_ofIFSH_vEE4typeEE11result_typeESH_T0_T1_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>)")]
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEENS1_26device_close_all_operationIS5_EENSD_ISB_EEEENS1_14execute_traitsIT_NS_9result_ofIFSH_vEE4typeEE11result_typeESH_T0_T1_")]
pub fn stub_0x33080(src: &[u8], dst: &mut Vec<u8>, close_dst: &mut dyn FnMut()) -> usize {
    // IDA 0x33080: `execute_all<copy_operation, close_all(src), close_all(dst)>`
    // runs the copy via 0x33188 then closes the sink (vtable call at 0x330f4).
    // MODEL: the sink close is a caller callback; the byte count is observed.
    let n = stub_0x33188(src, dst);
    close_dst();
    n
}

// 0x33188 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEENS1_26device_close_all_operationIS5_EEEENS1_14execute_traitsIT_NS_9result_ofIFSG_vEE4typeEE11result_typeESG_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>)")]
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEENS1_26device_close_all_operationIS5_EEEENS1_14execute_traitsIT_NS_9result_ofIFSG_vEE4typeEE11result_typeESG_T0_")]
pub fn stub_0x33188(src: &[u8], dst: &mut Vec<u8>) -> usize {
    // IDA 0x33188: `execute_all<copy_operation, close_all(src)>` forwards to
    // `copy_impl` (0x33200). MODEL: default chunk buffer; count observed.
    stub_0x33250(src, dst, 4096)
}

// 0x33250 — __ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESH_
// type: int __fastcall(int, int, unsigned int, int, int, void *, int, int, int, int)
#[doc(alias = "int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>(boost::reference_wrapper<std::istream> &,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESH_")]
pub fn stub_0x33250(src: &[u8], dst: &mut Vec<u8>, buffer: usize) -> usize {
    // IDA 0x33250: `copy_impl<istream, ostringstream>` allocates a `buffer`
    // (0x33282), loops `read` (0x332cc) / fully-`write` (0x332fc) until the
    // read reports 0 (mapped to -1 break at 0x332d0), frees the buffer
    // (0x33312), and returns the total (0x33332).
    if buffer == 0 {
        return 0;
    }
    let mut total = 0;
    let mut offset = 0;
    while offset < src.len() {
        let end = (offset + buffer).min(src.len());
        dst.extend_from_slice(&src[offset..end]);
        total += end - offset;
        offset = end;
    }
    total
}

/// `RBX::Http` endpoint state (IDA 0x33368).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HttpEndpoint {
    /// Request URL copied from the `char const*` ctor arg (0x333e6).
    pub url: String,
    /// `RBX::Http::defaultApi` snapshot (0x333b8); empty until observed.
    pub api_base: String,
}
// 0x33368 — __ZN3RBX4HttpC2EPKc
// type: RBX::Http *__fastcall(RBX::Http *this, const char *)
#[doc(alias = "RBX::Http::Http(char const*)")]
pub fn stub_0x33368(url: &str) -> HttpEndpoint {
    // IDA 0x33368: `Http::Http(char const*)` stores `defaultApi` (0x333b8),
    // copies the URL (0x333e6), and zeroes the remaining fields
    // (0x333f4..0x33406). MODEL: only the observable strings are kept.
    HttpEndpoint { url: url.to_owned(), api_base: String::new() }
}

// 0x33454 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x33454(type_matches: bool) -> usize {
    // IDA 0x33454: `get_deleter` returns the deleter address on typeinfo
    // match, else null (cf. 0x31c14). The nonzero cookie stands in for
    // the address.
    if type_matches { 1 } else { 0 }
}

// 0x3346c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x3346c() -> usize {
    // IDA 0x3346c: `get_untyped_deleter` returns `this + 16` (0x3346e) —
    // unconditionally non-null, unlike the 0x31c2c sibling. The nonzero
    // cookie stands in for the address.
    1
}

// 0x33470 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewaENS3_5list2INS3_5valueIS6_EENSA_IaEEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewaENS3_5list2INS3_5valueIS6_EENSA_IaEEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x33470(get_typeinfo: bool) -> &'static str {
    // IDA 0x33470: `functor_manager<bind_t<RobloxView,schar>>::manage`
    // answers op 4 with the `bind_t` typeinfo (0x334ca), clones on op 0/1
    // (0x33480), checks the name on op 3 (0x334aa), else no-op (op 2).
    // Other ops are closure-buffer glue.
    if get_typeinfo { BIND_ROBLOXVIEW_SCHAR_TYPEINFO } else { "" }
}
// 0x334d0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP10RobloxViewaENS3_5list2INS3_5valueIS6_EENSA_IaEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP10RobloxViewaENS3_5list2INS3_5valueIS6_EENSA_IaEEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_0x334d0() {
    // IDA 0x334d0: `void_function_obj_invoker0<bind_t<RobloxView,schar>>::
    // invoke` calls the stored slot with the bound view + char.
    // Closure-call glue; no explicit body.
}
// 0x334dc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorENS3_5list2INS3_5valueIS6_EENSB_IS7_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *),boost::_bi::list2<boost::_bi::value<objc_object *>,boost::_bi::list2<objc_selector>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *),boost::_bi::list2<boost::_bi::value<objc_object *>,boost::_bi::list2<objc_selector>>>>&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorENS3_5list2INS3_5valueIS6_EENSB_IS7_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x334dc(get_typeinfo: bool) -> &'static str {
    // IDA 0x334dc: `functor_manager<bind_t<objc_object,objc_selector>>::
    // manage` answers op 4 with the `bind_t` typeinfo (0x33536), clones on
    // op 0/1 (0x334ec), checks the name on op 3 (0x33516), else no-op.
    if get_typeinfo { BIND_OBJC_VOID_TYPEINFO } else { "" }
}
// 0x3353c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorENS3_5list2INS3_5valueIS6_EENSB_IS7_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *),boost::_bi::list2<boost::_bi::value<objc_object *>,boost::_bi::list2<objc_selector>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorENS3_5list2INS3_5valueIS6_EENSB_IS7_EEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_0x3353c() {
    // IDA 0x3353c: `void_function_obj_invoker0<bind_t<objc_object,
    // objc_selector>>::invoke` calls the stored slot with the bound
    // target + selector. Closure-call glue; no explicit body.
}
// 0x33548 — __ZN10TeleporterD1Ev
// type: void __fastcall(Teleporter *__hidden this)
#[doc(alias = "Teleporter::~Teleporter()")]
#[doc(alias = "__ZN10TeleporterD1Ev")]
pub fn stub_0x33548() {
    // IDA 0x33548: D1 complete-object dtor, empty. Drop glue covers it;
    // no explicit body.
}
// 0x3354c — __ZN10TeleporterD0Ev
// type: void __fastcall(Teleporter *__hidden this)
#[doc(alias = "Teleporter::~Teleporter() [0x3354c]")]
#[doc(alias = "__ZN10TeleporterD0Ev")]
pub fn stub_0x3354c() {
    // IDA 0x3354c: D0 deleting dtor — `operator delete` only. `Arc` drop
    // glue covers it; no explicit body.
}
/// `Teleporter::doTeleport`/`teleportImpl` string triple (IDA 0x33550/0x33d00).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeleportArgs {
    pub place: String,
    pub auth: String,
    pub script: String,
}
// 0x33550 — __ZN10Teleporter10doTeleportERKSsS1_S1_
// type: _DWORD __fastcall(Teleporter *__hidden this, const std::string *, const std::string *, const std::string *)
#[doc(alias = "Teleporter::doTeleport(std::string const&,std::string const&,std::string const&)")]
#[doc(alias = "__ZN10Teleporter10doTeleportERKSsS1_S1_")]
pub fn stub_0x33550(place: &str, auth: &str, script: &str) -> TeleportArgs {
    // IDA 0x33550: copies the three strings (0x3357a..0x335be), binds
    // `teleportImpl` (0x335e0), and posts it as a `function<void()>` on
    // the worker queue. MODEL: scheduling folds into the caller; the
    // bound triple is observed.
    TeleportArgs { place: place.to_owned(), auth: auth.to_owned(), script: script.to_owned() }
}
// 0x33920 — __ZNK10Teleporter17isTeleportEnabledEv
// type: _DWORD __fastcall(Teleporter *__hidden this)
#[doc(alias = "Teleporter::isTeleportEnabled(void)const")]
#[doc(alias = "__ZNK10Teleporter17isTeleportEnabledEv")]
pub fn stub_0x33920() -> bool {
    // IDA 0x33920: `Teleporter::isTeleportEnabled` returns 1 (0x33922).
    true
}
// 0x33924 — __ZN5boost4bindIvP13PlaceLauncherSsSsSsS2_SsSsSsEENS_3_bi6bind_tIT_PFS5_T0_T1_T2_T3_ENS3_9list_av_4IT4_T5_T6_T7_E4typeEEESB_SD_SE_SF_SG_
// type: int __fastcall(int, int, int, std::string *, std::string *, std::string *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list_av_4<PlaceLauncher *,std::string,std::string,std::string>::type> boost::bind<void,PlaceLauncher *,std::string,std::string,std::string,PlaceLauncher *,std::string,std::string,std::string>(void (*)(PlaceLauncher *,std::string,std::string,std::string),PlaceLauncher *,std::string,std::string,std::string)")]
#[doc(alias = "__ZN5boost4bindIvP13PlaceLauncherSsSsSsS2_SsSsSsEENS_3_bi6bind_tIT_PFS5_T0_T1_T2_T3_ENS3_9list_av_4IT4_T5_T6_T7_E4typeEEESB_SD_SE_SF_SG_")]
pub fn stub_0x33924() {
    // IDA 0x33924: bind thunk; binds are plain closures (cf. generated_166
    // bind family) — carrier no-op.
}
// 0x33d00 — __ZN10Teleporter12teleportImplEP13PlaceLauncherSsSsSs
#[doc(alias = "Teleporter::teleportImpl(PlaceLauncher *,std::string,std::string,std::string)")]
#[doc(alias = "__ZN10Teleporter12teleportImplEP13PlaceLauncherSsSsSs")]
pub fn stub_0x33d00(place: &str, auth: &str, script: &str) -> TeleportArgs {
    // IDA 0x33d00: `teleportImpl` converts the three `std::string`s to
    // `NSString` (0x33d32..0x33d8a) and issues
    // `teleport:withAuthentication:withScript:` (0x33dac). MODEL: ObjC
    // plumbing not modeled; the dispatch triple is observed.
    TeleportArgs { place: place.to_owned(), auth: auth.to_owned(), script: script.to_owned() }
}
// 0x33db0 — __ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EC2ES5_S6_S6_S6_
// type: int __fastcall(int, int, std::string *, int, std::string *)
#[doc(alias = "boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::list4(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EC2ES5_S6_S6_S6_")]
pub fn stub_0x33db0() {
    // IDA 0x33db0: `list4<PlaceLauncher,string x3>::list4` packs the bind
    // arguments (copies each string, forwards to `storage4` at 0x33e2c).
    // Captures fold into closures — carrier no-op.
}
// 0x33fe0 — __ZN5boost3_bi8storage4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EC2ES5_S6_S6_S6_
// type: int __fastcall(int, int, std::string *, int, std::string *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage4(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EC2ES5_S6_S6_S6_")]
pub fn stub_0x33fe0() {
    // IDA 0x33fe0: `storage4<PlaceLauncher,string x3>::storage4` stores the
    // launcher + first string and forwards the rest to `storage3`
    // (0x3404c). Captures fold into closures — carrier no-op.
}
// 0x341ac — __ZN5boost3_bi8storage3INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_EC2ES5_S6_S6_
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_EC2ES5_S6_S6_")]
pub fn stub_0x341ac() {
    // IDA 0x341ac: `storage3<PlaceLauncher,string x2>::storage3` stores the
    // launcher + first string (0x341e4) and copies the second (0x3423c).
    // Captures fold into closures — carrier no-op.
}

// 0x342f4 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x342f4() {
    // IDA 0x342f4: `function<void()>::function<bind_t<PlaceLauncher,...>>`
    // ctor; binds are plain closures — carrier no-op.
}

// 0x345b0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x345b0() {
    // IDA 0x345b0: `function0<void>::function0<bind_t<PlaceLauncher,...>>`
    // ctor; binds are plain closures — carrier no-op.
}

// 0x34870 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEEvT_
// type: int(void)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>)")]
#[doc(alias = "__ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEEvT_")]
pub fn stub_0x34870() {
    // IDA 0x34870: `function0<void>::assign_to<bind_t<PlaceLauncher,...>>`;
    // closure-buffer store folds into `Box<dyn Fn>` — carrier no-op.
}

// 0x34b40 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x34b40(get_typeinfo: bool) -> &'static str {
    // IDA 0x34b40: `functor_manager<bind_t<PlaceLauncher,...>>::manage`
    // answers op 4 with the `bind_t` typeinfo (0x34b56), else delegates to
    // `manager` (0x34b44). Other ops are closure-buffer glue.
    if get_typeinfo { BIND_PLACELAUNCHER_TYPEINFO } else { "" }
}

// 0x34b5c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_0x34b5c() {
    // IDA 0x34b5c: `void_function_obj_invoker0<bind_t<PlaceLauncher,...>>::
    // invoke` dispatches through `list4::operator()` (0x34b6e).
    // Closure-call glue; no explicit body.
}

// 0x34b70 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEbT_RNS1_15function_bufferE
// type: int(void)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x34b70() {
    // IDA 0x34b70: `basic_vtable0<void>::assign_to<bind_t<PlaceLauncher,...>>`
    // copies the bound triple and stores it in the closure buffer (0x34b94..
    // 0x34c04). Buffer ops fold into `Box<dyn Fn>` — carrier no-op.
}

// 0x34e30 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x34e30() {
    // IDA 0x34e30: `basic_vtable0<void>::assign_to<bind_t<PlaceLauncher,...>>
    // (function_obj_tag)` copies the bound triple and forwards to
    // `assign_functor` (0x34ec0). Buffer ops fold into `Box<dyn Fn>` —
    // carrier no-op.
}

// 0x350ec — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x350ec() {
    // IDA 0x350ec: `basic_vtable0<void>::assign_functor<bind_t<PlaceLauncher,
    // ...>>` heap-allocates the bound copy (`operator new(0x14)` at 0x35114,
    // copies at 0x3511a..0x3518c) and stores it (0x35192). Buffer ops fold
    // into `Box<dyn Fn>` — carrier no-op.
}

// 0x35200 — __ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EclIPFvS4_SsSsSsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::operator()<void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(PlaceLauncher *,std::string,std::string,std::string) &,boost::_bi::list0 &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EclIPFvS4_SsSsSsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x35200(
    place: &str,
    auth: &str,
    script: &str,
    invoke: &mut dyn FnMut(&str, &str, &str),
) {
    // IDA 0x35200: `list4<PlaceLauncher,string x3>::operator()` copies the
    // three bound strings (0x35230..0x35276) and calls `teleportImpl`
    // (0x35288), releasing the copies after (0x35298..). MODEL: the copies
    // are borrows; the dispatch is a caller callback.
    invoke(place, auth, script);
}

// 0x35438 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE7managerERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE7managerERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x35438() {
    // IDA 0x35438: `functor_manager<bind_t<PlaceLauncher,...>>::manager`
    // switch: clone (0x354ac), move (0x3550e), destroy (0x35516), type
    // check (0x35550), typeinfo (default). Closure-buffer ops fold into
    // `Box<dyn Fn>` — carrier no-op.
}

/// `__GLOBAL__I_a_8` one-shot latch (IDA 0x355c8): boost categories,
/// `ios_base::Init`, exception statics, singleton pools, and the
/// `FactoryProduct` creators run once behind `__cxa_atexit` guards.
static GLOBAL_A8_INIT: LazyLock<u32> = LazyLock::new(|| 1);
// 0x355c8 — __GLOBAL__I_a_8
#[doc(alias = "global constructor keyed to_a_8")]
#[doc(alias = "__GLOBAL__I_a_8")]
pub fn stub_0x355c8() -> u32 {
    // IDA 0x355c8: static-init ctor keyed to `a_8` — `generic_category` /
    // `system_category` (0x355cc..0x355e6), `ios_base::Init` (0x355ea),
    // `exception_ptr` statics, `singleton_pool` pools (XmlAttribute,
    // XmlElement, FWInstance, OnDemandInstance, OnDemandPVInstance), and
    // `FactoryProduct` creators (ScriptContext, TaskSchedulerSettings,
    // Camera, UserInputService), each guarded (0x35618..0x3585a).
    // MODEL: runtime statics self-initialize; the once latch is observed.
    *GLOBAL_A8_INIT
}

/// `Reachability` observable state (IDA 0x3588c..0x35ce4, Reachability.m).
/// System objects (`SCNetworkReachabilityRef`, run loop, autorelease pool)
/// live on the host; only the observable latches are modeled here.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReachabilityState {
    /// Probed target: hostname or dotted-quad address (IDA 0x35a00/0x35a80).
    pub target: String,
    /// `localWiFiRef` ivar (IDA 0x35a78/0x35b96): routes status through
    /// `localWiFiStatusForFlags:` instead of `networkStatusForFlags:`.
    pub local_wifi_ref: bool,
    /// Notifier scheduled on the run loop (IDA 0x3588c/0x35970).
    pub notifier_running: bool,
    /// `kNetworkReachabilityChangedNotification` posts via
    /// `ReachabilityCallback` (IDA 0x358ec).
    pub notifications_posted: u32,
}
/// `-[Reachability networkStatusForFlags:]` outcomes (IDA 0x35c6c).
pub const REACHABILITY_NOT_REACHABLE: u32 = 0;
pub const REACHABILITY_VIA_WIFI: u32 = 1;
pub const REACHABILITY_VIA_WWAN: u32 = 2;
/// `kNetworkReachabilityChangedNotification` name posted by
/// `ReachabilityCallback` (IDA 0x358ec).
pub const REACHABILITY_CHANGED_NOTIFICATION: &str = "kNetworkReachabilityChangedNotification";
// 0x3588c — -[Reachability startNotifier]
// type: char __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability startNotifier]")]
pub fn stub_0x3588c(
    state: &mut ReachabilityState,
    callback_ok: bool,
    schedule_ok: bool,
) -> bool {
    // IDA 0x3588c: `startNotifier` installs `ReachabilityCallback` via
    // `SCNetworkReachabilitySetCallback` (0x358ba) and schedules on the
    // current run loop (0x358ce); returns nonzero only if both succeed
    // (0x358e6). MODEL: host installs fold into the caller; the latch and
    // the combined outcome are observed.
    state.notifier_running = callback_ok && schedule_ok;
    state.notifier_running
}

// 0x358ec — _ReachabilityCallback
// type: id __fastcall(int, int, int)
#[doc(alias = "_ReachabilityCallback")]
pub fn stub_0x358ec(state: &mut ReachabilityState) -> &'static str {
    // IDA 0x358ec: `ReachabilityCallback` drains into an autorelease pool
    // (0x3590a..0x3591e) and posts `kNetworkReachabilityChangedNotification`
    // (0x35938..0x35954). MODEL: pool/release fold into the host; the post
    // is counted and the notification name observed.
    state.notifications_posted += 1;
    REACHABILITY_CHANGED_NOTIFICATION
}

// 0x35970 — -[Reachability stopNotifier]
// type: void __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability stopNotifier]")]
pub fn stub_0x35970(state: &mut ReachabilityState) {
    // IDA 0x35970: `stopNotifier` unschedules from the run loop when the
    // ref is non-null (0x35984..0x359a4). MODEL: the ref folds into host
    // ownership; the latch always clears.
    state.notifier_running = false;
}

// 0x359a8 — -[Reachability dealloc]
// type: void __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability dealloc]")]
pub fn stub_0x359a8(state: &mut ReachabilityState) {
    // IDA 0x359a8: `dealloc` runs `stopNotifier` (0x359be), `CFRelease`s
    // the ref when non-null (0x359ce..0x359d4), then super `dealloc`
    // (0x359ec..0x359f6). MODEL: releases fold into host ownership; the
    // state resets to default.
    *state = ReachabilityState::default();
}

// 0x35a00 — +[Reachability reachabilityWithHostName:]
// type: id __cdecl(id, SEL, id)
#[doc(alias = "+[Reachability reachabilityWithHostName:]")]
pub fn stub_0x35a00(host: &str, create_ok: bool) -> Option<ReachabilityState> {
    // IDA 0x35a00: `reachabilityWithHostName:` takes the UTF-8 hostname
    // (0x35a14), creates the ref (0x35a22), and on success stores it with
    // `localWiFiRef = 0` (0x35a72..0x35a78); failure returns null
    // (0x35a24). MODEL: the ref folds into host ownership; `None` is the
    // null answer.
    create_ok.then(|| ReachabilityState {
        target: host.to_owned(),
        ..ReachabilityState::default()
    })
}

// 0x35a80 — +[Reachability reachabilityWithAddress:]
// type: id __cdecl(id, SEL, const sockaddr_in *)
#[doc(alias = "+[Reachability reachabilityWithAddress:]")]
pub fn stub_0x35a80(octets: [u8; 4], create_ok: bool) -> Option<ReachabilityState> {
    // IDA 0x35a80: `reachabilityWithAddress:` creates the ref from the
    // `sockaddr` (0x35a9a) and on success stores it with `localWiFiRef = 0`
    // (0x35aee..0x35af4); failure returns null (0x35a9c). MODEL: the
    // sockaddr folds into a dotted-quad target string.
    create_ok.then(|| ReachabilityState {
        target: format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3]),
        ..ReachabilityState::default()
    })
}

// 0x35af8 — +[Reachability reachabilityForInternetConnection]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[Reachability reachabilityForInternetConnection]")]
pub fn stub_0x35af8(create_ok: bool) -> Option<ReachabilityState> {
    // IDA 0x35af8: `reachabilityForInternetConnection` builds a zeroed
    // `sockaddr` (0x35b22) and forwards to `reachabilityWithAddress:`
    // (0x35b3e).
    stub_0x35a80([0, 0, 0, 0], create_ok)
}

// 0x35b44 — +[Reachability reachabilityForLocalWiFi]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[Reachability reachabilityForLocalWiFi]")]
pub fn stub_0x35b44(create_ok: bool) -> Option<ReachabilityState> {
    // IDA 0x35b44: `reachabilityForLocalWiFi` builds the link-local
    // `sockaddr` (0x35b6e..0x35b80), forwards to `reachabilityWithAddress:`
    // (0x35b82), and sets `localWiFiRef = 1` (0x35b96).
    stub_0x35a80([169, 254, 0, 0], create_ok).map(|mut state| {
        state.local_wifi_ref = true;
        state
    })
}

// 0x35ba8 — -[Reachability localWiFiStatusForFlags:]
// type: int __cdecl(Reachability *self, SEL, unsigned int)
#[doc(alias = "-[Reachability localWiFiStatusForFlags:]")]
pub fn stub_0x35ba8(flags: u32) -> bool {
    // IDA 0x35ba8: `localWiFiStatusForFlags:` logs the flags (0x35bba) and
    // returns `(flags & 0x20002) == 131074` (0x35bcc), i.e. reachable
    // (bit 1) and direct (bit 17) together.
    flags & 0x20002 == 0x20002
}

// 0x35bd0 — _PrintReachabilityFlags
#[doc(alias = "_PrintReachabilityFlags")]
pub fn stub_0x35bd0(flags: u32) -> String {
    // IDA 0x35bd0: `PrintReachabilityFlags` maps each flag bit to a letter
    // (`-` when clear) and `NSLog`s the code with the caller label
    // (0x35c64): `d` = 0x20000, `l` = 0x10000, `D` = 0x20, `i` = 0x10,
    // `C` = 8, `c` = 4, `W` = 0x40000, `R` = 2, `t` = 1. MODEL: the log
    // sink folds into the host; the code is returned in `NSLog` arg order.
    const BITS: [(u32, char); 9] = [
        (0x40000, 'W'),
        (0x2, 'R'),
        (0x1, 't'),
        (0x4, 'c'),
        (0x8, 'C'),
        (0x10, 'i'),
        (0x20, 'D'),
        (0x10000, 'l'),
        (0x20000, 'd'),
    ];
    BITS.iter()
        .map(|(bit, letter)| if flags & bit != 0 { *letter } else { '-' })
        .collect()
}

// 0x35c6c — -[Reachability networkStatusForFlags:]
// type: int __cdecl(Reachability *self, SEL, unsigned int)
#[doc(alias = "-[Reachability networkStatusForFlags:]")]
pub fn stub_0x35c6c(flags: u32) -> u32 {
    // IDA 0x35c6c: `networkStatusForFlags:` logs the flags (0x35c7e),
    // returns 0 when unreachable (bit 1 clear, 0x35c88), 2 on WWAN
    // (0x40000, 0x35cae), else 1 when no connection is required (bit 2
    // clear) or the link is direct without intervention (0x28 set with
    // bit 4 clear), else 0 (0x35c94..0x35cb2).
    if flags & 0x2 == 0 {
        return REACHABILITY_NOT_REACHABLE;
    }
    if flags & 0x40000 != 0 {
        return REACHABILITY_VIA_WWAN;
    }
    let direct = flags & 0x28 != 0;
    let mut wifi = flags & 0x4 == 0;
    if direct && flags & 0x10 == 0 {
        wifi = true;
    }
    if wifi { REACHABILITY_VIA_WIFI } else { REACHABILITY_NOT_REACHABLE }
}

// 0x35cb8 — -[Reachability connectionRequired]
// type: char __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability connectionRequired]")]
pub fn stub_0x35cb8(flags: u32, get_ok: bool) -> bool {
    // IDA 0x35cb8: `connectionRequired` reads the flags (0x35cd2) and
    // answers bit 2 (0x35cdc); a failed read answers 0 (0x35cd4).
    get_ok && flags & 0x4 != 0
}

// 0x35ce4 — -[Reachability currentReachabilityStatus]
// type: int __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability currentReachabilityStatus]")]
pub fn stub_0x35ce4(state: &ReachabilityState, flags: u32, get_ok: bool) -> u32 {
    // IDA 0x35ce4: `currentReachabilityStatus` reads the flags (0x35d00),
    // answers 0 on failure (0x35d02), else dispatches on `localWiFiRef`
    // to `localWiFiStatusForFlags:` (0x35d20) or `networkStatusForFlags:`
    // (0x35d2c).
    if !get_ok {
        return REACHABILITY_NOT_REACHABLE;
    }
    if state.local_wifi_ref {
        u32::from(stub_0x35ba8(flags))
    } else {
        stub_0x35c6c(flags)
    }
}

/// `RobloxAlert` queued request (IDA 0x35d3c/0x35e90, RobloxAlert.m).
/// The `dispatch_async` to the main queue folds into the caller; the
/// request is observed here.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AlertRequest {
    pub message: String,
    pub has_delegate: bool,
}
/// `UIAlertView` built by the alert blocks (IDA 0x35d8c/0x35ee4).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AlertView {
    /// Localized title (`RobloxWord`, 0x35f5e).
    pub title: String,
    pub message: String,
    /// Cancel button (`OkWord` without delegate, `CancelWord` with).
    pub cancel: String,
    /// `OkWord` other button, present only with a delegate (0x35fcc).
    pub other: Option<String>,
    pub has_delegate: bool,
}
// 0x35d3c — +[RobloxAlert RobloxAlertWithMessage:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxAlert RobloxAlertWithMessage:]")]
pub fn stub_0x35d3c(message: &str) -> AlertRequest {
    // IDA 0x35d3c: `RobloxAlertWithMessage:` captures the message in a
    // stack block (0x35d70..0x35d80) and `dispatch_async`s it to the main
    // queue (0x35d82). MODEL: the queue hop folds into the caller; the
    // queued request is observed.
    AlertRequest { message: message.to_owned(), has_delegate: false }
}

// 0x35d8c — ___38+[RobloxAlert RobloxAlertWithMessage:]_block_invoke
#[doc(alias = "___38+[RobloxAlert RobloxAlertWithMessage:]_block_invoke")]
pub fn stub_0x35d8c(message: &str) -> AlertView {
    // IDA 0x35d8c: the `RobloxAlertWithMessage:` block builds a
    // `UIAlertView` with the localized title (0x35dd2..0x35e06), the
    // message, no delegate, and the `OkWord` cancel button (0x35e0e..
    // 0x35e4c), shows it (0x35e5c), and releases it. MODEL: alloc/show/
    // release fold into host ownership; the shown view is observed.
    AlertView {
        title: "Roblox".to_string(),
        message: message.to_owned(),
        cancel: "OK".to_string(),
        other: None,
        has_delegate: false,
    }
}

// 0x35e7c — ___copy_helper_block__5
#[doc(alias = "___copy_helper_block__5")]
pub fn stub_0x35e7c() {
    // IDA 0x35e7c: `__copy_helper_block__5` retains the captured message
    // (`_Block_object_assign`, 0x35e82). Block retain glue; no explicit
    // body.
}

// 0x35e88 — ___destroy_helper_block__5
#[doc(alias = "___destroy_helper_block__5")]
pub fn stub_0x35e88() {
    // IDA 0x35e88: `__destroy_helper_block__5` releases the captured
    // message (`_Block_object_dispose`, 0x35e8c). Block release glue; no
    // explicit body.
}

// 0x35e90 — +[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]
// type: void __cdecl(id, SEL, id, id)
#[doc(alias = "+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]")]
pub fn stub_0x35e90(message: &str, has_delegate: bool) -> AlertRequest {
    // IDA 0x35e90: `RobloxAlertWithMessageAndDelegate:Delegate:` captures
    // message + delegate in a stack block (0x35ec4..0x35eda) and
    // `dispatch_async`s it to the main queue (0x35edc). MODEL: the queue
    // hop folds into the caller; the queued request is observed.
    AlertRequest { message: message.to_owned(), has_delegate }
}

// 0x35ee4 — ___58+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]_block_invoke
#[doc(alias = "___58+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]_block_invoke")]
pub fn stub_0x35ee4(message: &str, has_delegate: bool) -> AlertView {
    // IDA 0x35ee4: the delegate block builds a `UIAlertView` with the
    // `RobloxWord` title (0x35f5e), the message (0x35f64), the delegate
    // (0x35f6a), the `CancelWord` cancel button (0x35f88) and the `OkWord`
    // other button (0x35fa6..0x35fcc), shows it (0x35fdc), and releases
    // it. MODEL: alloc/show/release fold into host ownership; the shown
    // view is observed.
    AlertView {
        title: "Roblox".to_string(),
        message: message.to_owned(),
        cancel: "Cancel".to_string(),
        other: Some("OK".to_string()),
        has_delegate,
    }
}

// 0x35ffc — ___copy_helper_block_19
#[doc(alias = "___copy_helper_block_19")]
pub fn stub_0x35ffc() {
    // IDA 0x35ffc: `__copy_helper_block_19` retains both captures
    // (message at 0x3600c, delegate at 0x3601c). Block retain glue; no
    // explicit body.
}

// 0x36020 — ___destroy_helper_block_20
#[doc(alias = "___destroy_helper_block_20")]
pub fn stub_0x36020() -> ! {
    todo!("0x36020 ___destroy_helper_block_20")
}

// 0x3603c — __Z18getUserAgentStringv
// type: id __fastcall()
#[doc(alias = "getUserAgentString(void)")]
#[doc(alias = "__Z18getUserAgentStringv")]
pub fn stub_0x3603c() -> ! {
    todo!("0x3603c getUserAgentString(void)")
}

// 0x36058 — +[RobloxInfo getDeviceType]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getDeviceType]")]
pub fn stub_0x36058() -> ! {
    todo!("0x36058 +[RobloxInfo getDeviceType]")
}

// 0x36114 — +[RobloxInfo getDeviceModelNumber]
// type: int __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getDeviceModelNumber]")]
pub fn stub_0x36114() -> ! {
    todo!("0x36114 +[RobloxInfo getDeviceModelNumber]")
}

// 0x3622c — +[RobloxInfo thisDeviceIsATablet]
// type: char __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo thisDeviceIsATablet]")]
pub fn stub_0x3622c() -> ! {
    todo!("0x3622c +[RobloxInfo thisDeviceIsATablet]")
}

// 0x36290 — +[RobloxInfo deviceType]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo deviceType]")]
pub fn stub_0x36290() -> ! {
    todo!("0x36290 +[RobloxInfo deviceType]")
}

// 0x362fc — +[RobloxInfo deviceOSVersion]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo deviceOSVersion]")]
pub fn stub_0x362fc() -> ! {
    todo!("0x362fc +[RobloxInfo deviceOSVersion]")
}

// 0x36330 — +[RobloxInfo appVersion]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo appVersion]")]
pub fn stub_0x36330() -> ! {
    todo!("0x36330 +[RobloxInfo appVersion]")
}

// 0x36370 — +[RobloxInfo friendlyDeviceName]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo friendlyDeviceName]")]
pub fn stub_0x36370() -> ! {
    todo!("0x36370 +[RobloxInfo friendlyDeviceName]")
}

// 0x3683c — +[RobloxInfo getUserAgentString]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getUserAgentString]")]
pub fn stub_0x3683c() -> ! {
    todo!("0x3683c +[RobloxInfo getUserAgentString]")
}

// 0x36918 — +[RobloxInfo getBaseUrl]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getBaseUrl]")]
pub fn stub_0x36918() -> ! {
    todo!("0x36918 +[RobloxInfo getBaseUrl]")
}

// 0x369c0 — +[RobloxInfo getApiBaseUrl]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getApiBaseUrl]")]
pub fn stub_0x369c0() -> ! {
    todo!("0x369c0 +[RobloxInfo getApiBaseUrl]")
}

// 0x36ab0 — +[RobloxInfo getDomainString]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getDomainString]")]
pub fn stub_0x36ab0() -> ! {
    todo!("0x36ab0 +[RobloxInfo getDomainString]")
}

// 0x36bc8 — +[RobloxInfo getBaseUrlChangedNotification]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo getBaseUrlChangedNotification]")]
pub fn stub_0x36bc8() -> ! {
    todo!("0x36bc8 +[RobloxInfo getBaseUrlChangedNotification]")
}

// 0x36bd4 — +[RobloxInfo setBaseUrl:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxInfo setBaseUrl:]")]
pub fn stub_0x36bd4() -> ! {
    todo!("0x36bd4 +[RobloxInfo setBaseUrl:]")
}

// 0x36de4 — ___25+[RobloxInfo setBaseUrl:]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___25+[RobloxInfo setBaseUrl:]_block_invoke")]
pub fn stub_0x36de4() -> ! {
    todo!("0x36de4 ___25+[RobloxInfo setBaseUrl:]_block_invoke")
}

// 0x36e04 — +[RobloxInfo searchUrl]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxInfo searchUrl]")]
pub fn stub_0x36e04() -> ! {
    todo!("0x36e04 +[RobloxInfo searchUrl]")
}

// 0x36e80 — __GLOBAL__I_a_9
#[doc(alias = "global constructor keyed to_a_9")]
#[doc(alias = "__GLOBAL__I_a_9")]
pub fn stub_0x36e80() -> ! {
    todo!("0x36e80 global constructor keyed to_a_9")
}

// 0x37068 — __ZN10RobloxView37requestStopRenderingForBackgroundModeEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::requestStopRenderingForBackgroundMode(void)")]
#[doc(alias = "__ZN10RobloxView37requestStopRenderingForBackgroundModeEv")]
pub fn stub_0x37068() -> ! {
    todo!("0x37068 RobloxView::requestStopRenderingForBackgroundMode(void)")
}

// 0x37378 — __ZN10RobloxView22requestResumeRenderingEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::requestResumeRendering(void)")]
#[doc(alias = "__ZN10RobloxView22requestResumeRenderingEv")]
pub fn stub_0x37378() -> ! {
    todo!("0x37378 RobloxView::requestResumeRendering(void)")
}

// 0x375b4 — __Z13macBundlePathv
// type: _DWORD __fastcall()
#[doc(alias = "macBundlePath(void)")]
#[doc(alias = "__Z13macBundlePathv")]
pub fn stub_0x375b4() -> ! {
    todo!("0x375b4 macBundlePath(void)")
}

// 0x37628 — __ZN10RobloxViewC2EjjSsSsSs
#[doc(alias = "RobloxView::RobloxView(unsigned int,unsigned int,std::string,std::string,std::string)")]
#[doc(alias = "__ZN10RobloxViewC2EjjSsSsSs")]
pub fn stub_0x37628() -> ! {
    todo!("0x37628 RobloxView::RobloxView(unsigned int,unsigned int,std::string,std::string,std::string)")
}

// 0x37b3c — __ZN10RobloxView16completeViewPrepEN5boost10shared_ptrIN3RBX4GameEEE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, void *, char, int, int, int, int)
#[doc(alias = "RobloxView::completeViewPrep(boost::shared_ptr<RBX::Game>)")]
#[doc(alias = "__ZN10RobloxView16completeViewPrepEN5boost10shared_ptrIN3RBX4GameEEE")]
pub fn stub_0x37b3c() -> ! {
    todo!("0x37b3c RobloxView::completeViewPrep(boost::shared_ptr<RBX::Game>)")
}

// 0x380a4 — __ZN10RobloxView13bindWorkspaceEN5boost10shared_ptrIN3RBX8ViewBaseEEENS1_INS2_9DataModelEEENS1_INS2_16OverlayDataModelEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int)
#[doc(alias = "RobloxView::bindWorkspace(boost::shared_ptr<RBX::ViewBase>,boost::shared_ptr<RBX::DataModel>,boost::shared_ptr<RBX::OverlayDataModel>)")]
#[doc(alias = "__ZN10RobloxView13bindWorkspaceEN5boost10shared_ptrIN3RBX8ViewBaseEEENS1_INS2_9DataModelEEENS1_INS2_16OverlayDataModelEEE")]
pub fn stub_0x380a4() -> ! {
    todo!("0x380a4 RobloxView::bindWorkspace(boost::shared_ptr<RBX::ViewBase>,boost::shared_ptr<RBX::DataModel>,boost::shared_ptr<RBX::OverlayDataModel>)")
}

// 0x382b0 — __ZN10RobloxView22defineConcurrencyRulesEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::defineConcurrencyRules(void)")]
#[doc(alias = "__ZN10RobloxView22defineConcurrencyRulesEv")]
pub fn stub_0x382b0() -> ! {
    todo!("0x382b0 RobloxView::defineConcurrencyRules(void)")
}

// 0x386d0 — __ZN10RobloxView16restartDataModelEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::restartDataModel(void)")]
#[doc(alias = "__ZN10RobloxView16restartDataModelEv")]
pub fn stub_0x386d0() -> ! {
    todo!("0x386d0 RobloxView::restartDataModel(void)")
}

// 0x38720 — __ZN10RobloxView15newGameDidStartEv
// type: _DWORD __fastcall(RobloxView *__hidden this)
#[doc(alias = "RobloxView::newGameDidStart(void)")]
#[doc(alias = "__ZN10RobloxView15newGameDidStartEv")]
pub fn stub_0x38720() -> ! {
    todo!("0x38720 RobloxView::newGameDidStart(void)")
}

// 0x45808 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSERKSA_")]
pub fn stub_0x45808() -> ! {
    todo!("0x45808 boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> const&)")
}

// 0x458ac — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv")]
pub fn stub_0x458ac() -> ! {
    todo!("0x458ac rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_do_get_mutex(void)")
}

// 0x459a4 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_")]
pub fn stub_0x459a4() -> ! {
    todo!("0x459a4 rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&,rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>*)")
}

// 0x45aa0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13callable_slotIN5boost8functionIS5_EEED1Ev")]
pub fn stub_0x45aa0() -> ! {
    todo!("0x45aa0 rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::callable_slot<boost::function<void ()(bool,void *,RBX::UIEvent)>>::~callable_slot()")
}

#[cfg(test)]
mod login_service_tail_tests {
    use super::*;

    #[test]
    fn refcount_and_class_name_shape() {
        assert!(!stub_0x31c30());
        assert_eq!(stub_0x31c14(true), 1);
        assert_eq!(stub_0x31c14(false), 0);
        assert_eq!(stub_0x31c2c(), 0);
        stub_0x31bf0();
        stub_0x31bf4();
    }

    #[test]
    fn child_added_and_signal_glue_shape() {
        assert_eq!(stub_0x31cd0(true), BIND_CHILD_ADDED_TYPEINFO);
        assert_eq!(stub_0x31cd0(false), "");
        assert_eq!(stub_0x31ec8(), *SIGNAL_STR_MUTEX);
        stub_0x31d30();
        stub_0x31d48();
        stub_0x31e24();
        stub_0x31fc0();
        stub_0x320bc();
        stub_0x32194();
        stub_0x3219c();
    }
}

#[cfg(test)]
mod factory_bind_tests {
    use super::*;

    #[test]
    fn open_url_and_factory_shape() {
        assert_eq!(stub_0x32270(true), BIND_OPEN_URL_TYPEINFO);
        assert_eq!(stub_0x32270(false), "");
        stub_0x322d0();
        stub_0x322e8();
        stub_0x32408();
        stub_0x3240c();
        assert_eq!(stub_0x32410(true), "TaskSchedulerSettings");
        assert!(stub_0x3247c(true, true));
        assert!(!stub_0x3247c(true, false));
        stub_0x324fc();
        stub_0x32520();
        stub_0x325fc();
        stub_0x326fc();
        stub_0x32700();
        stub_0x32720();
        stub_0x32764();
        stub_0x327d4();
    }

    #[test]
    fn view_game_bind_carriers_are_noops() {
        stub_0x328bc();
        stub_0x32984();
        stub_0x32a68();
        stub_0x32b50();
        stub_0x32c48();
        stub_0x32c64();
        stub_0x32c78();
        stub_0x32d60();
        stub_0x32e74();
        stub_0x32f4c();
    }
}

#[cfg(test)]
mod stream_teleport_batch_tests {
    use super::*;

    #[test]
    fn copy_impl_moves_bytes_in_chunks() {
        let src = b"hello world";
        let mut dst = Vec::new();
        assert_eq!(stub_0x33250(src, &mut dst, 4), src.len());
        assert_eq!(dst, src);
        let mut empty = Vec::new();
        assert_eq!(stub_0x33250(b"", &mut empty, 4), 0);
        assert!(empty.is_empty());
        let mut zero = Vec::new();
        assert_eq!(stub_0x33250(src, &mut zero, 0), 0);
        assert!(zero.is_empty());
    }

    #[test]
    fn execute_all_copies_and_closes() {
        let mut dst = Vec::new();
        assert_eq!(stub_0x33188(b"abc", &mut dst), 3);
        assert_eq!(dst, b"abc");
        let mut dst2 = Vec::new();
        let mut closed = false;
        let n = stub_0x33080(b"xy", &mut dst2, &mut || closed = true);
        assert_eq!(n, 2);
        assert_eq!(dst2, b"xy");
        assert!(closed);
    }

    #[test]
    fn http_ctor_and_players_deleter_shape() {
        let ep = stub_0x33368("https://example.com");
        assert_eq!(ep.url, "https://example.com");
        assert_eq!(stub_0x33454(true), 1);
        assert_eq!(stub_0x33454(false), 0);
        assert_ne!(stub_0x3346c(), 0);
    }

    #[test]
    fn functor_typeinfo_answers() {
        assert_eq!(stub_0x33470(true), BIND_ROBLOXVIEW_SCHAR_TYPEINFO);
        assert_eq!(stub_0x33470(false), "");
        assert_eq!(stub_0x334dc(true), BIND_OBJC_VOID_TYPEINFO);
        assert_eq!(stub_0x334dc(false), "");
        assert_eq!(stub_0x34b40(true), BIND_PLACELAUNCHER_TYPEINFO);
        assert_eq!(stub_0x34b40(false), "");
        stub_0x334d0();
        stub_0x3353c();
        stub_0x34b5c();
    }

    #[test]
    fn teleporter_round_trip() {
        assert!(stub_0x33920());
        stub_0x33548();
        stub_0x3354c();
        let args = stub_0x33550("place", "auth", "script");
        assert_eq!(args.place, "place");
        assert_eq!(args.auth, "auth");
        assert_eq!(args.script, "script");
        let dispatch = stub_0x33d00("place", "auth", "script");
        assert_eq!(dispatch, args);
    }

    #[test]
    fn placelauncher_bind_carriers_are_noops() {
        stub_0x33924();
        stub_0x33db0();
        stub_0x33fe0();
        stub_0x341ac();
        stub_0x342f4();
        stub_0x345b0();
        stub_0x34870();
        stub_0x34b70();
    }
}

#[cfg(test)]
mod reachability_alert_batch_tests {
    use super::*;

    #[test]
    fn placelauncher_tail_carriers_and_global_init() {
        stub_0x34e30();
        stub_0x350ec();
        stub_0x35438();
        let mut seen = Vec::new();
        stub_0x35200("p", "a", "s", &mut |p, a, s| {
            seen.push((p.to_owned(), a.to_owned(), s.to_owned()));
        });
        assert_eq!(seen, [("p".to_string(), "a".to_string(), "s".to_string())]);
        assert_eq!(stub_0x355c8(), 1);
        assert_eq!(stub_0x355c8(), 1);
    }

    #[test]
    fn notifier_lifecycle() {
        let mut state = ReachabilityState::default();
        assert!(stub_0x3588c(&mut state, true, true));
        assert!(state.notifier_running);
        assert!(!stub_0x3588c(&mut state, true, false));
        assert!(!state.notifier_running);
        stub_0x3588c(&mut state, true, true);
        assert_eq!(stub_0x358ec(&mut state), REACHABILITY_CHANGED_NOTIFICATION);
        assert_eq!(state.notifications_posted, 1);
        stub_0x35970(&mut state);
        assert!(!state.notifier_running);
        stub_0x359a8(&mut state);
        assert_eq!(state, ReachabilityState::default());
    }

    #[test]
    fn constructors_chain() {
        assert!(stub_0x35a00("apple.com", false).is_none());
        let host = stub_0x35a00("apple.com", true).unwrap();
        assert_eq!(host.target, "apple.com");
        assert!(!host.local_wifi_ref);
        assert!(stub_0x35a80([1, 2, 3, 4], false).is_none());
        let addr = stub_0x35a80([10, 0, 0, 1], true).unwrap();
        assert_eq!(addr.target, "10.0.0.1");
        let internet = stub_0x35af8(true).unwrap();
        assert_eq!(internet.target, "0.0.0.0");
        assert!(!internet.local_wifi_ref);
        assert!(stub_0x35af8(false).is_none());
        let wifi = stub_0x35b44(true).unwrap();
        assert!(wifi.local_wifi_ref);
        assert!(stub_0x35b44(false).is_none());
    }

    #[test]
    fn flag_predicates_match_ida() {
        assert_eq!(stub_0x35bd0(0), "---------");
        assert_eq!(stub_0x35bd0(0x20002), "-R------d");
        assert_eq!(stub_0x35bd0(0xFFFFFFFF), "WRtcCiDld");
        assert!(stub_0x35ba8(0x20002));
        assert!(!stub_0x35ba8(0x2));
        assert!(!stub_0x35ba8(0x20000));
        assert_eq!(stub_0x35c6c(0), REACHABILITY_NOT_REACHABLE);
        assert_eq!(stub_0x35c6c(0x2), REACHABILITY_VIA_WIFI);
        assert_eq!(stub_0x35c6c(0x40002), REACHABILITY_VIA_WWAN);
        assert_eq!(stub_0x35c6c(0x2 | 0x4), REACHABILITY_NOT_REACHABLE);
        assert!(!stub_0x35cb8(0x4, false));
        assert!(stub_0x35cb8(0x4, true));
        assert!(!stub_0x35cb8(0x0, true));
    }

    #[test]
    fn current_status_dispatches_on_wifi_ref() {
        let plain = ReachabilityState::default();
        let wifi = ReachabilityState { local_wifi_ref: true, ..ReachabilityState::default() };
        assert_eq!(stub_0x35ce4(&plain, 0x2, false), REACHABILITY_NOT_REACHABLE);
        assert_eq!(stub_0x35ce4(&plain, 0x2, true), REACHABILITY_VIA_WIFI);
        assert_eq!(stub_0x35ce4(&wifi, 0x20002, true), 1);
        assert_eq!(stub_0x35ce4(&wifi, 0x2, true), 0);
    }

    #[test]
    fn alert_request_and_view_shapes() {
        let req = stub_0x35d3c("hello");
        assert_eq!(req.message, "hello");
        assert!(!req.has_delegate);
        let view = stub_0x35d8c("hello");
        assert_eq!(view.title, "Roblox");
        assert_eq!(view.message, "hello");
        assert_eq!(view.cancel, "OK");
        assert_eq!(view.other, None);
        stub_0x35e7c();
        stub_0x35e88();
        let dreq = stub_0x35e90("bye", true);
        assert!(dreq.has_delegate);
        let dview = stub_0x35ee4("bye", true);
        assert_eq!(dview.cancel, "Cancel");
        assert_eq!(dview.other, Some("OK".to_string()));
        assert!(dview.has_delegate);
        stub_0x35ffc();
    }
}
