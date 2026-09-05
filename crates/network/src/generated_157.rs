//! Auto-generated skeletons for rbx-network — global EA-sorted filler (RakNet|Network|Replicat|Socket filtered exhausted)
//! Filter: RakNet|Network|Replicat|Socket -> 5198 funcs (cs), 5282 (ci), 3 remaining before batch (next 0xecd6e8 _TFCreateCrashSocket); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x312d0..0x35e90 | existing 17509 -> 17609 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

/// ObjC block captured-object slot index for the +20 byte field (word 5).
pub const BLOCK_CAPTURE_WORD: usize = 5;

/// Apple reachability watcher state (IDA 0x3588c).
#[derive(Clone, Copy, Debug, Default)]
pub struct Reachability {
    pub handle: usize,
    pub scheduled: bool,
    pub is_wifi: bool,
}

/// `boost::_bi::bind_t` capture: target fn plus `(RobloxView, flag)` (IDA 0x33470).
#[derive(Clone, Copy, Debug, Default)]
pub struct BindViewFlag {
    pub target: usize,
    pub view: usize,
    pub flag: i8,
}

/// Teleporter request state (IDA 0x33550).
#[derive(Clone, Debug, Default)]
pub struct Teleporter {
    pub place_id: i32,
    pub url: String,
}

/// `boost::_bi::bind_t` capture: target fn plus `(launcher, string x3)` (IDA 0x33924).
#[derive(Clone, Debug, Default)]
pub struct BindLauncherStrings {
    pub target: usize,
    pub launcher: usize,
    pub s0: String,
    pub s1: String,
    pub s2: String,
}

/// `boost::_bi::list4` head: launcher plus two strings (IDA 0x341ac).
#[derive(Clone, Debug, Default)]
pub struct List3LauncherStrings {
    pub launcher: usize,
    pub s0: String,
    pub s1: String,
}

/// `boost::function0<void>` holding one launcher functor (IDA 0x342f4).
#[derive(Clone, Debug, Default)]
pub struct VoidLauncherCallback {
    pub bound: Option<BindLauncherStrings>,
}

/// `RBX::Http` request state (IDA 0x33368).
#[derive(Clone, Debug, Default)]
pub struct Http {
    pub url: String,
}

/// `rbx::signals` callable slot for `signal<void(std::string)>` (IDA 0x31fc0).
#[derive(Clone, Debug, Default)]
pub struct CallableSlot {
    pub id: u64,
    pub conn: u64,
    pub has_fn: bool,
}

/// `boost::shared_ptr` value slot for `Game` (IDA 0x327d4 shape).
#[derive(Clone, Copy, Debug, Default)]
pub struct GameSlot {
    pub ptr: usize,
    pub counted: usize,
}

/// `boost::_bi::bind_t` capture: target fn plus `(RobloxView, game)` (IDA 0x327d4).
#[derive(Clone, Debug, Default)]
pub struct BindViewGame {
    pub target: usize,
    pub view: usize,
    pub game: GameSlot,
}

/// `boost::function0<void>` holding one `(view, game)` functor (IDA 0x32984).
#[derive(Clone, Debug, Default)]
pub struct VoidViewCallback {
    pub bound: Option<BindViewGame>,
}

/// `boost::shared_ptr` value slot for `LoginService` (IDA 0x319ec shape).
#[derive(Clone, Copy, Debug, Default)]
pub struct LoginShared {
    pub ptr: usize,
    pub counted: usize,
}

/// `boost::_bi::bind_t` capture: target fn plus `(RobloxView, flag, FunctionMarshaller)`
/// (IDA 0x312d0).
#[derive(Clone, Copy, Debug, Default)]
pub struct BindViewFlagMarshaller {
    pub target: usize,
    pub view: usize,
    pub flag: i8,
    pub marshaller: usize,
}

/// `boost::_bi::bind_t` capture: target fn plus `(objc, sel)` (IDA 0x31cd0).
#[derive(Clone, Copy, Debug, Default)]
pub struct BindObjc {
    pub target: usize,
    pub obj: usize,
    pub sel: usize,
}
// 0x312d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operat")]
pub fn stub_312d0(op: i32, src: &mut Option<BindViewFlagMarshaller>, dst: &mut Option<BindViewFlagMarshaller>) -> bool {
    // IDA 0x312d0: 0 clone (new 0x10, field copy); 1 move; 2 destroy; 3 check type (tail follows
    // the 0x2d964 shape).
    match op {
        0 => {
            *dst = *src;
            true
        }
        1 => {
            *dst = src.take();
            true
        }
        2 => {
            *dst = None;
            true
        }
        3 => true,
        _ => false,
    }
}

// 0x31348 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn stub_31348(bound: &BindViewFlagMarshaller, invoke: &mut dyn FnMut(usize, i8, usize)) {
    // IDA 0x31348: invoker1: F from the buffer; F(view, flag, marshaller).
    invoke(bound.view, bound.flag, bound.marshaller);
}

// 0x31358 — __ZNK3RBX15ServiceProvider6createINS_12LoginServiceEEEPT_v
// demangled: RBX::LoginService * RBX::ServiceProvider::create<RBX::LoginService>(void)const
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "RBX::LoginService * RBX::ServiceProvider::create<RBX::LoginService>(void)const")]
pub fn stub_31358(cell: &mut Option<usize>, create: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x31358: locked create-or-find of the LoginService singleton (asserts below truncation).
    if let Some(v) = *cell {
        return v;
    }
    let v = create();
    if cell.is_none() {
        *cell = Some(v);
    }
    v
}

// 0x3151c — __ZNK3RBX15ServiceProvider4findINS_12LoginServiceEEEPT_v
// demangled: RBX::LoginService * RBX::ServiceProvider::find<RBX::LoginService>(void)const
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "RBX::LoginService * RBX::ServiceProvider::find<RBX::LoginService>(void)const")]
pub fn stub_3151c(cache: &mut Vec<usize>, class_index: &mut dyn FnMut() -> usize, find_service_by_class_name: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x3151c: call_once(doGetClassIndex<LoginService>); cached slot hit; else
    // findServiceByClassName, store, return.
    let idx = class_index();
    if idx + 1 <= cache.len() {
        let hit = cache[idx];
        if hit != 0 {
            return hit;
        }
    } else {
        cache.resize(idx + 1, 0);
    }
    let found = find_service_by_class_name();
    if found != 0 {
        cache[idx] = found;
    }
    found
}

// 0x31678 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LoginServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)")]
pub fn stub_31678(alloc: &mut dyn FnMut() -> usize, construct: &mut dyn FnMut(usize), share: &mut dyn FnMut(usize) -> LoginShared) -> LoginShared {
    // IDA 0x31678: new LoginService (0x70); ctor; shared_ptr with Creatable deleter.
    let raw = alloc();
    construct(raw);
    share(raw)
}

// 0x31728 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12LoginServiceEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const&)")]
pub fn stub_31728(dst: &mut LoginShared, src: &LoginShared, release: &mut dyn FnMut(usize)) {
    // IDA 0x31728: shared_count copy; swap into dst; release the old count.
    let old = *dst;
    *dst = *src;
    if old.counted != 0 {
        release(old.counted);
    }
}

// 0x317e4 — __ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v
// demangled: __ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v")]
pub fn stub_317e4(cell: &mut Option<usize>, declare: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x317e4: sLoginService name declare (guarded once) with null-name check.
    *cell.get_or_insert_with(|| declare())
}

// 0x31828 — __ZN3RBX4Name13callDoDeclareILZNS_13sLoginServiceEEEEvv
// demangled: __ZN3RBX4Name13callDoDeclareILZNS_13sLoginServiceEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sLoginServiceEEEEvv")]
pub fn stub_31828(declare: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x31828: thunk tail-calls doDeclare<LoginService> shim.
    declare()
}

// 0x3182c — __ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v
// demangled: __ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v
// type: 
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v")]
pub fn stub_3182c(cell: &mut Option<usize>, declare: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x3182c: guard-checked once Name::declare(sLoginService).
    *cell.get_or_insert_with(|| declare())
}

// 0x31910 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12LoginServiceEEEvv
// demangled: void RBX::ServiceProvider::callDoGetClassIndex<RBX::LoginService>(void)
// type: 
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::LoginService>(void)")]
pub fn stub_31910(index: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x31910: thunk tail-calls doGetClassIndex<LoginService>.
    index()
}

// 0x31914 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12LoginServiceEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LoginService>(void)
// type: 
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LoginService>(void)")]
pub fn stub_31914(cell: &mut Option<usize>, new_index: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x31914: guard-checked once ServiceProvider::newIndex.
    *cell.get_or_insert_with(|| new_index())
}

// 0x319ec — __ZN5boost10shared_ptrIN3RBX12LoginServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int(void)
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_319ec(slot: &mut LoginShared, raw: usize, counted: usize, accept_owner: &mut dyn FnMut(usize)) {
    // IDA 0x319ec: shared_ptr<LoginService> ctor: px stored, shared_count ctor; nonzero px →
    // accept_owner.
    slot.ptr = raw;
    slot.counted = counted;
    if raw != 0 {
        accept_owner(raw);
    }
}

// 0x31a10 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LoginServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LoginService,RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const*,RBX::LoginService *)const
// type: 
// was: boost::shared_ptr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LoginService,RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const*,RBX::LoginService *)const")]
pub fn stub_31a10(owner: &mut usize, weak: &mut usize, candidate_owner: usize, candidate_weak: usize, use_count: usize) {
    // IDA 0x31a10: if !weak || !use_count(weak): owner = candidate; weak = candidate copy.
    if *weak == 0 || use_count == 0 {
        *owner = candidate_owner;
        *weak = candidate_weak;
    }
}

// 0x31aec — __ZN5boost6detail12shared_countC2IPN3RBX12LoginServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_31aec(slot: &mut LoginShared, raw: usize, control: usize) {
    // IDA 0x31aec: *a1 = 0; new sp_counted_impl_pd<LoginService>(px) uses/weaks 1; *a1 = it.
    slot.ptr = raw;
    slot.counted = control;
}

// 0x31bec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_31bec() {
    // IDA 0x31bec: empty dtor body (single BX LR).
}

// 0x31bf0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_31bf0(block: usize, free: &mut dyn FnMut(usize)) {
    // IDA 0x31bf0: deleting-dtor thunk tail-calls operator delete.
    free(block);
}

// 0x31bf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_31bf4(payload: usize, predelete: &mut dyn FnMut(usize) -> usize, destroy: &mut dyn FnMut(usize) -> usize) -> usize {
    // IDA 0x31bf4: px = block[12]; predelete(px); px ? px->deleter(px) (vtable + 8) : predelete result.
    let r = predelete(payload);
    if payload != 0 {
        destroy(payload)
    } else {
        r
    }
}

// 0x31c14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_31c14(block: usize, type_name: &str) -> usize {
    // IDA 0x31c14: match "N3RBX9CreatableINS_8InstanceEE7DeleterE" → block + 16, else 0.
    if type_name == "N3RBX9CreatableINS_8InstanceEE7DeleterE" {
        block + 16
    } else {
        0
    }
}

// 0x31c2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_31c2c(block: usize) -> usize {
    // IDA 0x31c2c: return block + 16.
    block + 16
}

// 0x31c30 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv
// demangled: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv")]
pub fn stub_31c30(class_name_empty: bool, s_class_name_null: bool, release_assert: &mut dyn FnMut()) -> bool {
    // IDA 0x31c30: ReleaseAssert(className().empty() == (sClassName == NULL), Object.h:360); return it.
    let ok = class_name_empty == s_class_name_null;
    if !ok {
        release_assert();
    }
    ok
}

// 0x31cd0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,b")]
pub fn stub_31cd0(op: i32, src: &mut Option<BindObjc>, dst: &mut Option<BindObjc>) -> bool {
    // IDA 0x31cd0: 4 → typeinfo; ≤1 → copy-or-move words; 2 → destroy; 3 → check type (tail follows
    // the 0x2d964 shape).
    match op {
        4 => true,
        0 => {
            *dst = *src;
            true
        }
        1 => {
            *dst = src.take();
            true
        }
        2 => {
            *dst = None;
            true
        }
        3 => true,
        _ => false,
    }
}

// 0x31d30 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)
// type: int __fastcall(int, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)")]
pub fn stub_31d30(bound: &BindObjc, instance: usize, invoke: &mut dyn FnMut(usize, usize, usize)) {
    // IDA 0x31d30: invoker: functor f from the buffer; list3::operator()<F, list1<Instance&>> calls
    // f(obj, sel, instance).
    invoke(bound.obj, bound.sel, instance);
}

// 0x31d48 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)
// type: void __fastcall(int *, void (__fastcall **)(int, int, sp_counted_base **), const shared_count **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)")]
pub fn stub_31d48(bound: &BindObjc, instance: usize, invoke: &mut dyn FnMut(usize, usize, usize)) {
    // IDA 0x31d48: F = stored target; shared_count copied for the call; F(obj, sel, instance); released.
    invoke(bound.obj, bound.sel, instance);
}

// 0x31e24 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsEE4slotEEaSEPS6_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string)>::slot*)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string)>::slot*)")]
pub fn stub_31e24(slot: &mut usize, new: usize, add_ref: &mut dyn FnMut(usize), release: &mut dyn FnMut(usize)) {
    // IDA 0x31e24: add_ref(new); swap in; release(old).
    if new != 0 {
        add_ref(new);
    }
    let old = *slot;
    *slot = new;
    if old != 0 {
        release(old);
    }
}

// 0x31ec8 — __ZN3rbx7signals6signalIFvSsEE24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(std::string)>::safe_static_do_get_mutex(void)
// type: 
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::safe_static_do_get_mutex(void)")]
pub fn stub_31ec8(cell: &mut Option<usize>, alloc_mutex: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x31ec8: guard-checked once mutex alloc for the string-signal slot static.
    *cell.get_or_insert_with(|| alloc_mutex())
}

// 0x31fc0 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
// demangled: rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::callable<rbx::signals::signal<void ()(std::string)>*>(boost::function<void ()(std::string)> const&,rbx::signals::signal<void ()(std::string)>*)
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::callable<rbx::signals::signal<void ()(std::string)>*>(boost::function<void ()(std::string)> const&,rbx::signals::signal<void ()(std::string)>*)")]
pub fn stub_31fc0(slots: &mut Vec<CallableSlot>, conn: u64) -> u64 {
    // IDA 0x31fc0: vtable install; function1::assign_to_own into +4; return the slot.
    let id = slots.len() as u64;
    slots.push(CallableSlot { id, conn, has_fn: true });
    id
}

// 0x320bc — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost8functionIS2_EEED0Ev
// demangled: rbx::signals::signal<void ()(std::string)>::callable_slot<boost::function<void ()(std::string)>>::~callable_slot()
// type: 
// was: boost::shared_ptr
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::function<void ()(std::string)>>::~callable_slot()")]
pub fn stub_320bc(slots: &mut Vec<CallableSlot>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x320bc: callable_slot dtor: vtable resets; function clear; intrusive release; delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.conn);
    }
}

// 0x32194 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_E4callESs
// demangled: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::call(std::string)
// type: 
// was: boost::shared_ptr
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::call(std::string)")]
pub fn stub_32194(this: usize, call: &mut dyn FnMut(usize) -> usize) -> usize {
    // IDA 0x32194: non-virtual thunk adjusts this-4 then tail-calls.
    call(this.wrapping_sub(4))
}

// 0x3219c — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()
// type: int __fastcall(int)
// was: boost::shared_ptr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()")]
pub fn stub_3219c(slots: &mut Vec<CallableSlot>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3219c: D1: vtable resets; function clear; intrusive release (no delete).
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.conn);
    }
}

// 0x32270 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_b")]
pub fn stub_32270(op: i32, src: &mut Option<BindObjc>, dst: &mut Option<BindObjc>) -> bool {
    // IDA 0x32270: 4 → typeinfo; ≤1 → copy-or-move words; 2 → destroy; 3 → check type.
    match op {
        4 => true,
        0 => {
            *dst = *src;
            true
        }
        1 => {
            *dst = src.take();
            true
        }
        2 => {
            *dst = None;
            true
        }
        3 => true,
        _ => false,
    }
}

// 0x322d0 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorSsENS3_5list3INS3_5valueIS6_EENSB_IS7_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)
// type: int __fastcall(int, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,std::string),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
pub fn stub_322d0(bound: &BindObjc, arg: &str, invoke: &mut dyn FnMut(usize, usize, &str)) {
    // IDA 0x322d0: invoker1: list3::operator()<F, list1<string&>> calls F(obj, sel, arg).
    invoke(bound.obj, bound.sel, arg);
}

// 0x322e8 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_SsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,std::string),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,std::string) &,boost::_bi::list1<std::string &> &,int)
// type: void __fastcall(int *, void (__fastcall **)(int, int, int *), const std::string **)
// was: boost::shared_ptr
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,std::string),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,std::string) &,boost::_bi::list1<std::string &> &,int)")]
pub fn stub_322e8(bound: &BindObjc, arg: String, invoke: &mut dyn FnMut(usize, usize, String)) {
    // IDA 0x322e8: F = stored target; F(obj, sel, string copy).
    invoke(bound.obj, bound.sel, arg);
}

// 0x32408 — __ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv
// demangled: __ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv")]
pub fn stub_32408(declare: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x32408: thunk tail-calls doDeclare<GuiService> shim.
    declare()
}

// 0x3240c — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10GuiServiceEEEvv
// demangled: void RBX::ServiceProvider::callDoGetClassIndex<RBX::GuiService>(void)
// type: 
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GuiService>(void)")]
pub fn stub_3240c(index: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x3240c: thunk tail-calls doGetClassIndex<GuiService>.
    index()
}

// 0x32410 — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv
// demangled: __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_32410(constructed: bool, declare: &mut dyn FnMut() -> usize, release_assert: &mut dyn FnMut()) -> usize {
    // IDA 0x32410: ReleaseAssert(wasConstructed(), Object.h:236); return Name::declare.
    if !constructed {
        release_assert();
    }
    declare()
}

// 0x3247c — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv
// demangled: __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv")]
pub fn stub_3247c(constructed: bool, create: &mut dyn FnMut() -> usize, release_assert: &mut dyn FnMut()) -> usize {
    // IDA 0x3247c: ReleaseAssert(wasConstructed()); create the instance (below truncation).
    if !constructed {
        release_assert();
    }
    create()
}

// 0x324fc — __ZN5boost10shared_ptrIN3RBX21TaskSchedulerSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int(void)
// was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_324fc(slot: &mut LoginShared, raw: usize, counted: usize, accept_owner: &mut dyn FnMut(usize)) {
    // IDA 0x324fc: shared_ptr<TaskSchedulerSettings> ctor: px stored, shared_count ctor; nonzero px
    // → accept_owner.
    slot.ptr = raw;
    slot.counted = counted;
    if raw != 0 {
        accept_owner(raw);
    }
}

// 0x32520 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_21TaskSchedulerSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TaskSchedulerSettings,RBX::TaskSchedulerSettings>(boost::shared_ptr<RBX::TaskSchedulerSettings> const*,RBX::TaskSchedulerSettings *)const
// type: 
// was: boost::shared_ptr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TaskSchedulerSettings,RBX::TaskSchedulerSettings>(rbx_core::SharedPtr<RBX::TaskSchedulerSettings> const*,RBX::TaskSchedulerSettings *)const")]
pub fn stub_32520(owner: &mut usize, weak: &mut usize, candidate_owner: usize, candidate_weak: usize, use_count: usize) {
    // IDA 0x32520: if !weak || !use_count(weak): owner = candidate; weak = candidate copy.
    if *weak == 0 || use_count == 0 {
        *owner = candidate_owner;
        *weak = candidate_weak;
    }
}

// 0x325fc — __ZN5boost6detail12shared_countC2IPN3RBX21TaskSchedulerSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_325fc(slot: &mut LoginShared, raw: usize, control: usize) {
    // IDA 0x325fc: *a1 = 0; new sp_counted_impl_pd<TaskSchedulerSettings>(px) uses/weaks 1; *a1 = it.
    slot.ptr = raw;
    slot.counted = control;
}

// 0x326fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_326fc() {
    // IDA 0x326fc: empty dtor body.
}

// 0x32700 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_32700(payload: usize, predelete: &mut dyn FnMut(usize) -> usize, destroy: &mut dyn FnMut(usize) -> usize) -> usize {
    // IDA 0x32700: px = block[12]; predelete(px); px ? px->deleter(px) (vtable + 8) : predelete result.
    let r = predelete(payload);
    if payload != 0 {
        destroy(payload)
    } else {
        r
    }
}

// 0x32720 — __ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v
// demangled: __ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v")]
pub fn stub_32720(has_name: bool, cell: &mut Option<usize>, call_once: &mut dyn FnMut(), declare: &mut dyn FnMut() -> usize, null_name: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x32720: null sClassName → getNullName; else call_once(callDoDeclare) + doDeclare.
    if !has_name {
        return null_name();
    }
    if cell.is_none() {
        call_once();
        *cell = Some(declare());
    }
    cell.unwrap()
}

// 0x32764 — __ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv
// demangled: __ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv")]
pub fn stub_32764(declare: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x32764: thunk tail-calls doDeclare<TaskSchedulerSettings> shim.
    declare()
}

// 0x32768 — __ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv
// demangled: __ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv")]
pub fn stub_32768(constructed: bool, declare: &mut dyn FnMut() -> usize, release_assert: &mut dyn FnMut()) -> usize {
    // IDA 0x32768: ReleaseAssert(wasConstructed(), Object.h:236); return Name::declare.
    if !constructed {
        release_assert();
    }
    declare()
}

// 0x327d4 — __ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEES2_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// demangled: boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<RobloxView *,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,RobloxView *,boost::shared_ptr<RBX::Game>,RobloxView *,boost::shared_ptr<RBX::Game>>(void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),RobloxView *,boost::shared_ptr<RBX::Game>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<RobloxView *,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,RobloxView *,rbx_core::SharedPtr<RBX::Game>,RobloxView *,rbx_core::SharedPtr<RBX::Game>>(void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),RobloxView *,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_327d4(target: usize, view: usize, game: GameSlot) -> BindViewGame {
    // IDA 0x327d4: list2<value<RobloxView*>, value<shared_ptr<Game>>> ctor; bind_t pack.
    BindViewGame { target, view, game }
}

// 0x328bc — __ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
// demangled: boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_328bc(view: usize, game: GameSlot) -> BindViewGame {
    // IDA 0x328bc: list2 ctor: view stored, game ptr + shared_count copied in.
    BindViewGame { target: 0, view, game }
}

// 0x32984 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// demangled: __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
pub fn stub_32984(bound: BindViewGame) -> VoidViewCallback {
    // IDA 0x32984: function<void()> ctor: bind_t copied to temp, forwarded to function0 ctor.
    VoidViewCallback { bound: Some(bound) }
}

// 0x32a68 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// demangled: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_32a68(bound: BindViewGame) -> VoidViewCallback {
    // IDA 0x32a68: function0 ctor: *a1 = 0, then assign_to; temp released.
    let mut cb = VoidViewCallback::default();
    stub_32b50(&mut cb, bound);
    cb
}

// 0x32b50 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEEvT_
// demangled: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
pub fn stub_32b50(cb: &mut VoidViewCallback, bound: BindViewGame) {
    // IDA 0x32b50: function0::assign_to: functor + shared_count copied, stored vtable.
    cb.bound = Some(bound);
}

// 0x32c48 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_32c48(op: i32, out_type: &mut usize, out_flags: &mut u16) -> usize {
    // IDA 0x32c48: op != 4: tail-call functor_manager::manager table; else store the
    // bind_t<view, game> typeinfo, clear flags, return it.
    const MANAGER_TABLE: usize = 0x32c4c;
    const BIND_T_TYPEINFO: usize = 0x32c5e;
    if op != 4 {
        return MANAGER_TABLE;
    }
    *out_type = BIND_T_TYPEINFO;
    *out_flags = 0;
    BIND_T_TYPEINFO
}

// 0x32c64 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
// demangled: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_32c64(bound: &BindViewGame, invoke: &mut dyn FnMut(usize, usize)) {
    // IDA 0x32c64: void_function_obj_invoker0::invoke: functor f from the buffer;
    // list2::operator()<F(view, game), list0> calls f(view, game).
    invoke(bound.view, bound.game.ptr);
}

// 0x32c78 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<")]
pub fn stub_32c78(cb: &mut VoidViewCallback, bound: BindViewGame) -> bool {
    // IDA 0x32c78: basic_vtable0::assign_to: functor + shared_count copied, stored vtable; true.
    cb.bound = Some(bound);
    true
}

// 0x32d60 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)c
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<")]
pub fn stub_32d60(cb: &mut VoidViewCallback, bound: BindViewGame) -> bool {
    // IDA 0x32d60: tagged assign_to overload: vetted functor stored directly; true.
    cb.bound = Some(bound);
    true
}

// 0x32e74 — __ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn stub_32e74(bound: &BindViewGame, invoke: &mut dyn FnMut(usize, usize)) {
    // IDA 0x32e74: F = stored target; shared_count copied for the call; F(view, game); temp released.
    invoke(bound.view, bound.game.ptr);
}

// 0x32f4c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_32f4c(op: i32, src: &mut Option<BindViewGame>, dst: &mut Option<BindViewGame>, release: &mut dyn FnMut(usize)) -> bool {
    // IDA 0x32f4c: 0 clone (new 0x10, field + shared_count copy, store); 1 move; 2 destroy (release,
    // delete, clear); 3 check type.
    match op {
        0 => {
            *dst = src.clone();
            true
        }
        1 => {
            *dst = src.take();
            true
        }
        2 => {
            if let Some(bound) = dst.take() {
                release(bound.game.counted);
            }
            true
        }
        3 => true,
        _ => false,
    }
}

// 0x33080 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEENS1_26device_close_all_operationIS5_EENSD_ISB_EEEENS1_14execute_traitsIT_NS_9result_ofIFSH_vEE4typeEE11result_typeESH_T0_T1_
// demangled: boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::ios
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::shared_ptr
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,st")]
pub fn stub_33080(src: &[u8], chunk_size: usize, copy_chunk: &mut dyn FnMut(&[u8]) -> Vec<u8>, close: &mut dyn FnMut()) -> usize {
    // IDA 0x33080: execute_all: copy istream→ostringstream in 4096 chunks, then device_close_all on
    // both; total bytes.
    let mut total = 0;
    for piece in src.chunks(chunk_size.max(1)) {
        total += copy_chunk(piece).len();
    }
    close();
    close();
    total
}

// 0x33188 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEENS1_26device_close_all_operationIS5_EEEENS1_14execute_traitsIT_NS_9result_ofIFSG_vEE4typeEE11result_typeESG_T0_
// demangled: boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::ios
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::shared_ptr
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,st")]
pub fn stub_33188(src: &[u8], chunk_size: usize, copy_chunk: &mut dyn FnMut(&[u8]) -> Vec<u8>, close: &mut dyn FnMut()) -> usize {
    // IDA 0x33188: execute_all with a single close_all op: copy in chunks, close, total bytes.
    let mut total = 0;
    for piece in src.chunks(chunk_size.max(1)) {
        total += copy_chunk(piece).len();
    }
    close();
    total
}

// 0x33250 — __ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_ISt19basic_ostringstreamIcSt11char_traitsIcESaIcEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESH_
// demangled: int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>(boost::reference_wrapper<std::istream> &,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)
// type: int __fastcall(int, int, unsigned int, int, int, void *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::istream>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>(boost::reference_wrapper<std::istream> &,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)")]
pub fn stub_33250(src: &[u8], buf_size: usize, write: &mut dyn FnMut(&[u8])) -> usize {
    // IDA 0x33250: heap buffer of bufsize; read/write loop to EOF; total bytes.
    let mut total = 0;
    for piece in src.chunks(buf_size.max(1)) {
        write(piece);
        total += piece.len();
    }
    total
}

// 0x33368 — __ZN3RBX4HttpC2EPKc
// demangled: RBX::Http::Http(char const*)
// type: RBX::Http *__fastcall(RBX::Http *this, const char *)
#[doc(alias = "RBX::Http::Http(char const*)")]
pub fn stub_33368(url: &str) -> Http {
    // IDA 0x33368: Http ctor stores the URL (below truncation).
    Http { url: url.to_owned() }
}

// 0x33470 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewaENS3_5list2INS3_5valueIS6_EENSA_IaEEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_33470(op: i32, src: &mut Option<BindViewFlag>, dst: &mut Option<BindViewFlag>) -> bool {
    // IDA 0x33470: 0 clone; 1 move; 2 destroy; 3 check type.
    match op {
        0 => {
            *dst = *src;
            true
        }
        1 => {
            *dst = src.take();
            true
        }
        2 => {
            *dst = None;
            true
        }
        3 => true,
        _ => false,
    }
}

// 0x334d0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP10RobloxViewaENS3_5list2INS3_5valueIS6_EENSA_IaEEEEEEvE6invokeERNS1_15function_bufferE
// demangled: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>>>,void>::invoke(boost::detail::function::function_buffer &)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_334d0(bound: &BindViewFlag, invoke: &mut dyn FnMut(usize, i8)) {
    // IDA 0x334d0: F = stored target; F(view, flag).
    invoke(bound.view, bound.flag);
}

// 0x334dc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorENS3_5list2INS3_5valueIS6_EENSB_IS7_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *),boost::_bi::list2<boost::_bi::value<objc_object *>,boost::_bi::list2<objc_selector>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *),boost::_bi::list2<boost::_bi::value<objc_object *>,boost::_bi::list2<objc_selector>>>>&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *),boost::_bi::list2<boost::_bi::value<objc_object *>,boost::_bi::list2<objc_selector>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *),boost::_bi::list2<boost::_bi::value<objc_object")]
pub fn stub_334dc(op: i32, src: &mut Option<BindObjc>, dst: &mut Option<BindObjc>) -> bool {
    // IDA 0x334dc: 4 → typeinfo; ≤1 → copy-or-move words; 2 → destroy; 3 → check type.
    match op {
        4 => true,
        0 => {
            *dst = *src;
            true
        }
        1 => {
            *dst = src.take();
            true
        }
        2 => {
            *dst = None;
            true
        }
        3 => true,
        _ => false,
    }
}

// 0x3353c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorENS3_5list2INS3_5valueIS6_EENSB_IS7_EEEEEEvE6invokeERNS1_15function_bufferE
// demangled: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *),boost::_bi::list2<boost::_bi::value<objc_object *>,boost::_bi::list2<objc_selector>>>,void>::invoke(boost::detail::function::function_buffer &)
// type: int __fastcall(int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *),boost::_bi::list2<boost::_bi::value<objc_object *>,boost::_bi::list2<objc_selector>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_3353c(bound: &BindObjc, invoke: &mut dyn FnMut(usize, usize)) {
    // IDA 0x3353c: F = stored target; F(obj, sel).
    invoke(bound.obj, bound.sel);
}

// 0x33548 — __ZN10TeleporterD1Ev
// demangled: Teleporter::~Teleporter()
// type: void __fastcall(Teleporter *__hidden this)
#[doc(alias = "Teleporter::~Teleporter()")]
pub fn stub_33548() {
    // IDA 0x33548: empty Teleporter dtor body.
}

// 0x3354c — __ZN10TeleporterD0Ev
// demangled: Teleporter::~Teleporter()
// type: void __fastcall(Teleporter *__hidden this)
#[doc(alias = "Teleporter::~Teleporter()")]
pub fn stub_3354c(_teleporter: Teleporter) {
    // IDA 0x3354c: deleting dtor (drop on take).
    let _ = _teleporter;
}

// 0x33550 — __ZN10Teleporter10doTeleportERKSsS1_S1_
// demangled: Teleporter::doTeleport(std::string const&,std::string const&,std::string const&)
// type: _DWORD __fastcall(Teleporter *__hidden this, const std::string *, const std::string *, const std::string *)
#[doc(alias = "Teleporter::doTeleport(std::string const&,std::string const&,std::string const&)")]
pub fn stub_33550(t: &Teleporter, place: &str, ticket: &str, teleport: &mut dyn FnMut(&str, &str, &str)) {
    // IDA 0x33550: build the teleport request from the three strings; dispatch (below truncation).
    teleport(&t.url, place, ticket);
}

// 0x33920 — __ZNK10Teleporter17isTeleportEnabledEv
// demangled: Teleporter::isTeleportEnabled(void)const
// type: _DWORD __fastcall(Teleporter *__hidden this)
#[doc(alias = "Teleporter::isTeleportEnabled(void)const")]
pub fn stub_33920() -> bool {
    // IDA 0x33920: always enabled (MOVS-equivalent constant return).
    true
}

// 0x33924 — __ZN5boost4bindIvP13PlaceLauncherSsSsSsS2_SsSsSsEENS_3_bi6bind_tIT_PFS5_T0_T1_T2_T3_ENS3_9list_av_4IT4_T5_T6_T7_E4typeEEESB_SD_SE_SF_SG_
// demangled: boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list_av_4<PlaceLauncher *,std::string,std::string,std::string>::type> boost::bind<void,PlaceLauncher *,std::string,std::string,std::string,PlaceLauncher *,std::string,std::string,std::string>(void (*)(PlaceLauncher *,std::string,std::string,std::string),PlaceLauncher *,std::string,std::string,std::string)
// type: int __fastcall(int, int, int, std::string *, std::string *, std::string *)
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list_av_4<PlaceLauncher *,std::string,std::string,std::string>::type> boost::bind<void,PlaceLauncher *,std::string,std::string,std::string,PlaceLauncher *,std::string,std::string,std::string>(void (*)(PlaceLauncher *,std::string,std::string,std::string),PlaceLauncher *,std::string,std::string,std::str")]
pub fn stub_33924(target: usize, launcher: usize, s0: &str, s1: &str, s2: &str) -> BindLauncherStrings {
    // IDA 0x33924: three std::string copies into list4; bind_t pack; temps released.
    BindLauncherStrings { target, launcher, s0: s0.to_owned(), s1: s1.to_owned(), s2: s2.to_owned() }
}

// 0x33d00 — __ZN10Teleporter12teleportImplEP13PlaceLauncherSsSsSs
// demangled: Teleporter::teleportImpl(PlaceLauncher *,std::string,std::string,std::string)
// type: 
#[doc(alias = "Teleporter::teleportImpl(PlaceLauncher *,std::string,std::string,std::string)")]
pub fn stub_33d00(url: &str, place: &str, ticket: &str, open_url: &mut dyn FnMut(&str, &str, &str) -> usize) -> usize {
    // IDA 0x33d00: NSString conversions (defaultCStringEncoding) then the teleport open call.
    open_url(url, place, ticket)
}

// 0x33db0 — __ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EC2ES5_S6_S6_S6_
// demangled: boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::list4(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)
// type: int __fastcall(int, int, std::string *, int, std::string *)
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::list4(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_33db0(launcher: usize, s0: &str, s1: &str, s2: &str) -> BindLauncherStrings {
    // IDA 0x33db0: list4 ctor: launcher stored, three strings copied in.
    BindLauncherStrings { target: 0, launcher, s0: s0.to_owned(), s1: s1.to_owned(), s2: s2.to_owned() }
}

// 0x33fe0 — __ZN5boost3_bi8storage4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EC2ES5_S6_S6_S6_
// demangled: boost::_bi::storage4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage4(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)
// type: int __fastcall(int, int, std::string *, int, std::string *)
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage4(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_33fe0(launcher: usize, s0: &str, s1: &str, s2: &str) -> BindLauncherStrings {
    // IDA 0x33fe0: storage4 ctor: launcher + three strings stored.
    stub_33db0(launcher, s0, s1, s2)
}

// 0x341ac — __ZN5boost3_bi8storage3INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_EC2ES5_S6_S6_
// demangled: boost::_bi::storage3<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)
// type: int __fastcall(int, int, std::string *)
// was: boost::shared_ptr
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_341ac(launcher: usize, s0: &str, s1: &str) -> List3LauncherStrings {
    // IDA 0x341ac: storage3 ctor: launcher + two strings stored.
    List3LauncherStrings { launcher, s0: s0.to_owned(), s1: s1.to_owned() }
}

// 0x342f4 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// demangled: __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")]
pub fn stub_342f4(bound: BindLauncherStrings) -> VoidLauncherCallback {
    // IDA 0x342f4: function<void()> ctor: bind_t copied to temp, forwarded to function0 ctor.
    VoidLauncherCallback { bound: Some(bound) }
}

// 0x345b0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE
// demangled: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")]
pub fn stub_345b0(bound: BindLauncherStrings) -> VoidLauncherCallback {
    // IDA 0x345b0: function0 ctor: *a1 = 0, then assign_to; temp released.
    let mut cb = VoidLauncherCallback::default();
    stub_34870(&mut cb, bound);
    cb
}

// 0x34870 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEEvT_
// demangled: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::st
// type: int(void)
// was: boost::shared_ptr
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<Plac")]
pub fn stub_34870(cb: &mut VoidLauncherCallback, bound: BindLauncherStrings) {
    // IDA 0x34870: function0::assign_to: functor + strings copied, stored vtable.
    cb.bound = Some(bound);
}

// 0x34b40 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::f")]
pub fn stub_34b40(op: i32, out_type: &mut usize, out_flags: &mut u16) -> usize {
    // IDA 0x34b40: op != 4: tail-call functor_manager::manager table; else store the
    // bind_t<launcher> typeinfo, clear flags, return it.
    const MANAGER_TABLE: usize = 0x34b44;
    const BIND_T_TYPEINFO: usize = 0x34b56;
    if op != 4 {
        return MANAGER_TABLE;
    }
    *out_type = BIND_T_TYPEINFO;
    *out_flags = 0;
    BIND_T_TYPEINFO
}

// 0x34b5c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEvE6invokeERNS1_15function_bufferE
// demangled: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)
// type: 
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_34b5c(bound: &BindLauncherStrings, invoke: &mut dyn FnMut(usize, &str, &str, &str)) {
    // IDA 0x34b5c: void_function_obj_invoker0::invoke: functor f from the buffer;
    // list4::operator()<F, list0> calls f(launcher, s0, s1, s2).
    invoke(bound.launcher, &bound.s0, &bound.s1, &bound.s2);
}

// 0x34b70 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boo
// type: int(void)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<")]
pub fn stub_34b70(cb: &mut VoidLauncherCallback, bound: BindLauncherStrings) -> bool {
    // IDA 0x34b70: basic_vtable0::assign_to: functor + strings copied, stored vtable; true.
    cb.bound = Some(bound);
    true
}

// 0x34e30 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boo
// type: int __fastcall(int, int, int)
// was: boost::shared_ptr
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<")]
pub fn stub_34e30(cb: &mut VoidLauncherCallback, bound: BindLauncherStrings) -> bool {
    // IDA 0x34e30: tagged assign_to overload: vetted functor stored directly; true.
    cb.bound = Some(bound);
    true
}

// 0x350ec — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// demangled: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::l")]
pub fn stub_350ec(bound: &BindLauncherStrings) -> BindLauncherStrings {
    // IDA 0x350ec: assign_functor: new 0x14 + field/string copies (heap clone).
    bound.clone()
}

// 0x35200 — __ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EclIPFvS4_SsSsSsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::operator()<void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(PlaceLauncher *,std::string,std::string,std::string) &,boost::_bi::list0 &,int)
// type: int(void)
// was: boost::shared_ptr
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::operator()<void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(PlaceLauncher *,std::string,std::string,std::string) &,boost::_bi::list0 &,int)")]
pub fn stub_35200(bound: &BindLauncherStrings, invoke: &mut dyn FnMut(usize, &str, &str, &str)) {
    // IDA 0x35200: F = stored target; string copies; F(launcher, s0, s1, s2); temps released.
    invoke(bound.launcher, &bound.s0.clone(), &bound.s1.clone(), &bound.s2.clone());
}

// 0x35438 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE7managerERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
// was: boost::shared_ptr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::")]
pub fn stub_35438(op: i32, src: &mut Option<BindLauncherStrings>, dst: &mut Option<BindLauncherStrings>) -> bool {
    // IDA 0x35438: 0 clone (new 0x14, fields + string copies); 1 move; 2 destroy; 3 check type.
    match op {
        0 => {
            *dst = src.clone();
            true
        }
        1 => {
            *dst = src.take();
            true
        }
        2 => {
            dst.take();
            true
        }
        3 => true,
        _ => false,
    }
}

// 0x355c8 — __GLOBAL__I_a_8
// demangled: global constructor keyed to_a_8
// type: 
#[doc(alias = "global constructor keyed to_a_8")]
pub fn stub_355c8(init: &mut dyn FnMut()) {
    // IDA 0x355c8: __GLOBAL__I_a_8 static initializer (boost system categories, ios_base::Init,
    // atexit, exception guards).
    init();
}

// 0x3588c — -[Reachability startNotifier]
// demangled: -[Reachability startNotifier]
// type: char __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability startNotifier]")]
pub fn stub_3588c(r: &mut Reachability, set_callback: &mut dyn FnMut() -> bool, schedule: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x3588c: SetCallback (fail → false); ScheduleWithRunLoop; result.
    if !set_callback() {
        return false;
    }
    let ok = schedule();
    r.scheduled = ok;
    ok
}

// 0x358ec — _ReachabilityCallback
// demangled: _ReachabilityCallback
// type: id __fastcall(int, int, int)
#[doc(alias = "_ReachabilityCallback")]
pub fn stub_358ec(notify: &mut dyn FnMut(&str)) {
    // IDA 0x358ec: autorelease pool; post kNetworkReachabilityChangedNotification; drain pool.
    notify("kNetworkReachabilityChangedNotification");
}

// 0x35970 — -[Reachability stopNotifier]
// demangled: -[Reachability stopNotifier]
// type: void __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability stopNotifier]")]
pub fn stub_35970(r: &mut Reachability, unschedule: &mut dyn FnMut()) {
    // IDA 0x35970: ref set → unschedule from run loop.
    if r.scheduled {
        unschedule();
        r.scheduled = false;
    }
}

// 0x359a8 — -[Reachability dealloc]
// demangled: -[Reachability dealloc]
// type: void __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability dealloc]")]
pub fn stub_359a8(r: &mut Reachability, stop: &mut dyn FnMut(&mut Reachability), release: &mut dyn FnMut()) {
    // IDA 0x359a8: stopNotifier; CFRelease the ref; super dealloc.
    stop(r);
    release();
}

// 0x35a00 — +[Reachability reachabilityWithHostName:]
// demangled: +[Reachability reachabilityWithHostName:]
// type: id __cdecl(id, SEL, id)
#[doc(alias = "+[Reachability reachabilityWithHostName:]")]
pub fn stub_35a00(host: &str, create: &mut dyn FnMut(&str) -> Option<usize>) -> Option<Reachability> {
    // IDA 0x35a00: CreateWithName; null → null; else alloc/init/autorelease + store ref.
    create(host).map(|h| Reachability { handle: h, scheduled: false, is_wifi: false })
}

// 0x35a80 — +[Reachability reachabilityWithAddress:]
// demangled: +[Reachability reachabilityWithAddress:]
// type: id __cdecl(id, SEL, const sockaddr_in *)
#[doc(alias = "+[Reachability reachabilityWithAddress:]")]
pub fn stub_35a80(ip: u32, create: &mut dyn FnMut(u32) -> Option<usize>) -> Option<Reachability> {
    // IDA 0x35a80: CreateWithAddress; null → null; else alloc/init/autorelease + store ref.
    create(ip).map(|h| Reachability { handle: h, scheduled: false, is_wifi: false })
}

// 0x35af8 — +[Reachability reachabilityForInternetConnection]
// demangled: +[Reachability reachabilityForInternetConnection]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[Reachability reachabilityForInternetConnection]")]
pub fn stub_35af8(create: &mut dyn FnMut(u32) -> Option<usize>) -> Option<Reachability> {
    // IDA 0x35af8: zero sockaddr (family/len word 528) + reachabilityWithAddress.
    stub_35a80(0, create)
}

// 0x35b44 — +[Reachability reachabilityForLocalWiFi]
// demangled: +[Reachability reachabilityForLocalWiFi]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[Reachability reachabilityForLocalWiFi]")]
pub fn stub_35b44(create: &mut dyn FnMut(u32) -> Option<usize>) -> Option<Reachability> {
    // IDA 0x35b44: link-local sockaddr (169.254.0.0) + reachabilityWithAddress; mark local-WiFi.
    stub_35a80(0xA9FE0000, create).map(|mut r| {
        r.is_wifi = true;
        r
    })
}

// 0x35ba8 — -[Reachability localWiFiStatusForFlags:]
// demangled: -[Reachability localWiFiStatusForFlags:]
// type: int __cdecl(Reachability *self, SEL, unsigned int)
#[doc(alias = "-[Reachability localWiFiStatusForFlags:]")]
pub fn stub_35ba8(flags: u32, print: &mut dyn FnMut(u32)) -> bool {
    // IDA 0x35ba8: PrintReachabilityFlags; (flags & 0x20002) == 0x20002.
    print(flags);
    flags & 0x20002 == 0x20002
}

// 0x35bd0 — _PrintReachabilityFlags
// demangled: _PrintReachabilityFlags
// type: 
#[doc(alias = "_PrintReachabilityFlags")]
pub fn stub_35bd0(flags: u32) -> String {
    // IDA 0x35bd0: flag letters d/l/D/i/C/c ('-' otherwise; two trailing chars below truncation).
    let mut s = String::with_capacity(8);
    s.push(if flags & 0x20000 != 0 { 'd' } else { '-' });
    s.push(if flags & 0x10000 != 0 { 'l' } else { '-' });
    s.push(if flags & 0x20 != 0 { 'D' } else { '-' });
    s.push(if flags & 0x10 != 0 { 'i' } else { '-' });
    s.push(if flags & 8 != 0 { 'C' } else { '-' });
    s.push(if flags & 4 != 0 { 'c' } else { '-' });
    s
}

// 0x35cb8 — -[Reachability connectionRequired]
// demangled: -[Reachability connectionRequired]
// type: char __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability connectionRequired]")]
pub fn stub_35cb8(get_flags: &mut dyn FnMut() -> Option<u32>) -> u32 {
    // IDA 0x35cb8: GetFlags fail → 0; else flags & 4 (connection required).
    match get_flags() {
        Some(f) => f & 4,
        None => 0,
    }
}

// 0x35ce4 — -[Reachability currentReachabilityStatus]
// demangled: -[Reachability currentReachabilityStatus]
// type: int __cdecl(Reachability *self, SEL)
#[doc(alias = "-[Reachability currentReachabilityStatus]")]
pub fn stub_35ce4(is_wifi: bool, get_flags: &mut dyn FnMut() -> Option<u32>, status_for_flags: &mut dyn FnMut(bool, u32) -> i32) -> i32 {
    // IDA 0x35ce4: GetFlags fail → 0; else localWiFiStatusForFlags/networkStatusForFlags.
    match get_flags() {
        Some(f) => status_for_flags(is_wifi, f),
        None => 0,
    }
}

// 0x35d3c — +[RobloxAlert RobloxAlertWithMessage:]
// demangled: +[RobloxAlert RobloxAlertWithMessage:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxAlert RobloxAlertWithMessage:]")]
pub fn stub_35d3c(message: &str, dispatch_main: &mut dyn FnMut(String)) {
    // IDA 0x35d3c: completion block capturing the message; dispatch_async(main).
    dispatch_main(message.to_owned());
}

// 0x35d8c — ___38+[RobloxAlert RobloxAlertWithMessage:]_block_invoke
// demangled: ___38+[RobloxAlert RobloxAlertWithMessage:]_block_invoke
// type: 
#[doc(alias = "___38+[RobloxAlert RobloxAlertWithMessage:]_block_invoke")]
pub fn stub_35d8c(message: &str, title: &str, show_alert: &mut dyn FnMut(&str, &str)) {
    // IDA 0x35d8c: UIAlertView with localized title/message; show.
    show_alert(title, message);
}

// 0x35e7c — ___copy_helper_block__5
// demangled: ___copy_helper_block__5
// type: 
#[doc(alias = "___copy_helper_block__5")]
pub fn stub_35e7c(dst: &mut [usize], src: &[usize]) {
    // IDA 0x35e7c: _Block_object_assign(dst + 20, src[20], BLOCK_FIELD_IS_OBJECT).
    let retained = src.get(BLOCK_CAPTURE_WORD).copied().unwrap_or(0);
    if let Some(slot) = dst.get_mut(BLOCK_CAPTURE_WORD) {
        *slot = retained;
    }
}

// 0x35e88 — ___destroy_helper_block__5
// demangled: ___destroy_helper_block__5
// type: 
#[doc(alias = "___destroy_helper_block__5")]
pub fn stub_35e88(block: &[usize], release: &mut dyn FnMut(usize)) {
    // IDA 0x35e88: _Block_object_dispose(block[20], BLOCK_FIELD_IS_OBJECT).
    release(block.get(BLOCK_CAPTURE_WORD).copied().unwrap_or(0));
}

// 0x35e90 — +[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]
// demangled: +[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]
// type: void __cdecl(id, SEL, id, id)
#[doc(alias = "+[RobloxAlert RobloxAlertWithMessageAndDelegate:Delegate:]")]
pub fn stub_35e90(message: &str, delegate: usize, dispatch_main: &mut dyn FnMut(String, usize)) {
    // IDA 0x35e90: completion block capturing message + delegate; dispatch_async(main).
    dispatch_main(message.to_owned(), delegate);
}
