//! reflection — generated_bg_9 — 100 stubs EA-sorted asc global gap filler 0x3b108..0x3e2e8 not yet in crates/reflection (global 85545 funcs, 63680 gaps reflection before; 21865->21965 distinct)
//! Source: ida/export.json (85545 funcs) global EA asc not in crates/reflection/src — next 100 uncovered for reflection-bg sorted asc after 0x3b008
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// `ControllerService` declaration + class index (IDA 0x3b828/0x3b910,
/// cf. `RUNSERVICE_*` in bg_8).
pub(crate) static CONTROLLERSERVICE_DECLARED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub(crate) static CONTROLLERSERVICE_CLASS_INDEX: std::sync::LazyLock<usize> =
    std::sync::LazyLock::new(|| 1);

// 0x3b108 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x3b108() {
    // IDA 0x3b108: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3b110 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x3b110() {
    // IDA 0x3b110: `sp_counted_impl_pd<RunService*,Creatable::Deleter>::
    // dispose` runs `Instance::predelete` then deletes (same shape as
    // 0x32700). `Arc` drop glue covers it; no explicit body.
}

// 0x3b130 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x3b130() {
    // IDA 0x3b130: `sp_counted_impl_pd<RunService*,Creatable::Deleter>::
    // get_deleter` answers the deleter query by `type_info` (same shape
    // as 0x33454). `Arc` drop glue covers it; no explicit body.
}

// 0x3b148 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x3b148() {
    // IDA 0x3b148: `sp_counted_impl_pd<RunService*,Creatable::Deleter>::
    // get_untyped_deleter` returns the untyped deleter address (same
    // shape as 0x3346c). `Arc` drop glue covers it; no explicit body.
}

// 0x3b14c — __ZN5boost6detail12shared_countC2IN3RBX5Tasks8SequenceEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX5Tasks8SequenceEEEPT_")]
pub fn stub_0x3b14c() {
    // IDA 0x3b14c: `shared_count::shared_count<Sequence*>` allocates
    // the control block. `Arc` construction glue covers it; no explicit
    // body.
}

// 0x3b268 — __ZN3RBX5Tasks11Coordinator9onPreStepEPNS_13TaskScheduler3JobE
// type: void()
#[doc(alias = "RBX::Tasks::Coordinator::onPreStep(RBX::TaskScheduler::Job *)")]
#[doc(alias = "__ZN3RBX5Tasks11Coordinator9onPreStepEPNS_13TaskScheduler3JobE")]
pub fn stub_0x3b268() {
    // IDA 0x3b268: `Tasks::Coordinator::onPreStep` compiles to an empty
    // body (decompiled 0x3b268). No explicit body.
}

// 0x3b26c — __ZN3RBX5Tasks11Coordinator10onPostStepEPNS_13TaskScheduler3JobE
// type: void()
#[doc(alias = "RBX::Tasks::Coordinator::onPostStep(RBX::TaskScheduler::Job *)")]
#[doc(alias = "__ZN3RBX5Tasks11Coordinator10onPostStepEPNS_13TaskScheduler3JobE")]
pub fn stub_0x3b26c() {
    // IDA 0x3b26c: `Tasks::Coordinator::onPostStep` is the empty
    // post-step hook (same shape as 0x3b268). No explicit body.
}

// 0x3b270 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED1Ev")]
pub fn stub_0x3b270() {
    // IDA 0x3b270: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3b274 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED0Ev")]
pub fn stub_0x3b274() {
    // IDA 0x3b274: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3b278 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE7disposeEv")]
pub fn stub_0x3b278() {
    // IDA 0x3b278: `sp_counted_impl_p<Sequence>::dispose` deletes the
    // sequence. `Arc` drop glue covers it; no explicit body.
}

// 0x3b32c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE11get_deleterERKSt9type_info")]
pub fn stub_0x3b32c() {
    // IDA 0x3b32c: `sp_counted_impl_p<Sequence>::get_deleter` answers
    // null for the plain deleter. `Arc` drop glue covers it; no
    // explicit body.
}

// 0x3b330 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE19get_untyped_deleterEv")]
pub fn stub_0x3b330() {
    // IDA 0x3b330: `sp_counted_impl_p<Sequence>::get_untyped_deleter`
    // returns the untyped deleter address. `Arc` drop glue covers it;
    // no explicit body.
}

// 0x3b334 — __ZN5boost6detail12shared_countC2IN3RBX5Tasks17ExclusiveSequenceEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tasks::ExclusiveSequence>(RBX::Tasks::ExclusiveSequence *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX5Tasks17ExclusiveSequenceEEEPT_")]
pub fn stub_0x3b334() {
    // IDA 0x3b334: `shared_count::shared_count<ExclusiveSequence*>`
    // allocates the control block (same shape as 0x3b14c). `Arc`
    // construction glue covers it; no explicit body.
}

// 0x3b450 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED1Ev")]
pub fn stub_0x3b450() {
    // IDA 0x3b450: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3b454 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED0Ev")]
pub fn stub_0x3b454() {
    // IDA 0x3b454: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3b458 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE7disposeEv")]
pub fn stub_0x3b458() {
    // IDA 0x3b458: `sp_counted_impl_p<ExclusiveSequence>::dispose`
    // deletes the sequence (same shape as 0x3b278). `Arc` drop glue
    // covers it; no explicit body.
}

// 0x3b50c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE11get_deleterERKSt9type_info")]
pub fn stub_0x3b50c() {
    // IDA 0x3b50c: `sp_counted_impl_p<ExclusiveSequence>::get_deleter`
    // answers null for the plain deleter (same shape as 0x3b32c).
    // `Arc` drop glue covers it; no explicit body.
}

// 0x3b510 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE19get_untyped_deleterEv")]
pub fn stub_0x3b510() {
    // IDA 0x3b510: `sp_counted_impl_p<ExclusiveSequence>::
    // get_untyped_deleter` returns the untyped deleter address (same
    // shape as 0x3b330). `Arc` drop glue covers it; no explicit body.
}

// 0x3b518 — __ZNK3RBX15ServiceProvider4findINS_17ControllerServiceEEEPT_v
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::find<RBX::ControllerService>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17ControllerServiceEEEPT_v")]
pub fn stub_0x3b518(cached_present: bool, service_present: bool) -> bool {
    // IDA 0x3b518: `ServiceProvider::find<ControllerService>` returns
    // the cached service at the class index (0x3b578-0x3b5cc); on a miss
    // it resizes and, unless the class name is null (0x3bb58), declares
    // + `findServiceByClassName` (0x3b5d4-0x3b604). Presence reports
    // here.
    if cached_present {
        return true;
    }
    service_present
}

// 0x3b674 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17ControllerServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_17ControllerServiceEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x3b674(create_ok: bool) -> bool {
    // IDA 0x3b674: `Creatable<Instance>::create<ControllerService>`
    // runs the factory and returns the new instance (same shape as
    // 0x3a798). Factory glue; presence collapses to `bool`.
    create_ok
}

// 0x3b724 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17ControllerServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ControllerService>(rbx_core::SharedPtr<RBX::ControllerService> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17ControllerServiceEEERS3_RKNS0_IT_EE")]
pub fn stub_0x3b724() {
    // IDA 0x3b724: `shared_ptr<Instance>::operator=<ControllerService>`
    // cross-assigns the service (same shape as 0x3a2ec). `Arc`
    // conversion glue covers it; no explicit body.
}

// 0x3b7e0 — __ZN3RBX4Name7declareILZNS_18sControllerServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_18sControllerServiceEEEERKS0_v")]
pub fn stub_0x3b7e0() {
    // IDA 0x3b7e0: `Name::declare<sControllerService>` one-shots the
    // class-name declaration (same shape as 0x3add8). Idempotent
    // declare glue; no explicit body.
}

// 0x3b828 — __ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v")]
pub fn stub_0x3b828() {
    // IDA 0x3b828: `Name::doDeclare<sControllerService>` performs the
    // declaration (same shape as 0x3ae20). It records here.
    CONTROLLERSERVICE_DECLARED.store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x3b910 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17ControllerServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ControllerService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17ControllerServiceEEEmv")]
pub fn stub_0x3b910() -> usize {
    // IDA 0x3b910: `ServiceProvider::doGetClassIndex<ControllerService>`
    // returns the service class index (same shape as 0x3af08).
    *CONTROLLERSERVICE_CLASS_INDEX
}

// 0x3b9e8 — __ZN5boost10shared_ptrIN3RBX17ControllerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX17ControllerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x3b9e8() {
    // IDA 0x3b9e8: `shared_ptr<ControllerService>::shared_ptr<...,
    // Creatable::Deleter>` stores the pointer + deleter (same shape as
    // 0x3afe0). `Arc` construction glue covers it; no explicit body.
}

// 0x3ba10 — __ZN5boost6detail12shared_countC2IPN3RBX17ControllerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX17ControllerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x3ba10() {
    // IDA 0x3ba10: `shared_count::shared_count<ControllerService*,...>`
    // allocates the control block (same shape as 0x3b008). `Arc`
    // construction glue covers it; no explicit body.
}

// 0x3bb10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x3bb10() {
    // IDA 0x3bb10: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3bb18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x3bb18() {
    // IDA 0x3bb18: `sp_counted_impl_pd<ControllerService*,...>::dispose`
    // runs `Instance::predelete` then deletes (same shape as 0x3b110).
    // `Arc` drop glue covers it; no explicit body.
}

// 0x3bb38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x3bb38() {
    // IDA 0x3bb38: `sp_counted_impl_pd<ControllerService*,...>::
    // get_deleter` answers the deleter query by `type_info` (same shape
    // as 0x3b130). `Arc` drop glue covers it; no explicit body.
}

// 0x3bb50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x3bb50() {
    // IDA 0x3bb50: `sp_counted_impl_pd<ControllerService*,...>::
    // get_untyped_deleter` returns the untyped deleter address (same
    // shape as 0x3b148). `Arc` drop glue covers it; no explicit body.
}

// 0x3bb58 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv")]
pub fn stub_0x3bb58(class_name: Option<&str>) -> bool {
    // IDA 0x3bb58: `NonFactoryProduct<ControllerService>::
    // isNullClassName` asserts the empty-vs-null invariant in debug
    // (0x3bb7a-0x3bbac) and returns `sControllerService == NULL`
    // (0x3bbf4).
    class_name.is_none()
}

// 0x3bbf8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>::operator=(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSERKS3_")]
pub fn stub_0x3bbf8() {
    // IDA 0x3bbf8: `shared_ptr<Instance>::operator=(const&)`
    // copy-assigns the instance. `Arc` clone glue covers it; no explicit
    // body.
}

// 0x3bcb8 — __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0>::operator delete(void *)")]
#[doc(alias = "__ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EEdlEPv")]
pub fn stub_0x3bcb8() {
    // IDA 0x3bcb8: `intrusive_ptr_target<islot>::operator delete`
    // frees the slot. Drop glue covers it; no explicit body.
}

// 0x3be00 — __ZN3rbx7signals6signalIFvvEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::insert(rbx::signals::signal<void ()(void)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE6insertEPNS3_4slotE")]
pub fn stub_0x3be00() {
    // IDA 0x3be00: `signal<void()>::insert(slot *)` appends the slot
    // (same shape as 0x3d2f4). Slot-list glue; no explicit body.
}

// 0x3c010 — __ZN5boost26intrusive_ptr_add_weak_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// type: int __fastcall(_DWORD)
#[doc(alias = "void rbx_core::SharedPtr_add_weak_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
#[doc(alias = "__ZN5boost26intrusive_ptr_add_weak_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE")]
pub fn stub_0x3c010() {
    // IDA 0x3c010: `intrusive_ptr_add_weak_ref<islot>` bumps the weak
    // count. `Arc` downgrade glue covers it; no explicit body.
}

// 0x3c0c8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSEPS6_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot>::operator=(rbx::signals::signal<void ()(void)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSEPS6_")]
pub fn stub_0x3c0c8() {
    // IDA 0x3c0c8: `intrusive_ptr<slot>::operator=(slot*)`
    // copy-assigns the slot (same shape as 0x3d508). `Arc` clone glue
    // covers it; no explicit body.
}

// 0x3c170 — __ZN5boost5mutex6unlockEv
// type: _DWORD __fastcall(boost::mutex *__hidden this)
#[doc(alias = "boost::mutex::unlock(void)")]
#[doc(alias = "__ZN5boost5mutex6unlockEv")]
pub fn stub_0x3c170() {
    // IDA 0x3c170: `mutex::unlock` releases the mutex. Lock glue; no
    // explicit body.
}

// 0x3c2a0 — __ZN5boost15throw_exceptionINS_10lock_errorEEEvRKT_
// type: int __fastcall(std::string *)
#[doc(alias = "void boost::throw_exception<boost::lock_error>(boost::lock_error const&)")]
#[doc(alias = "__ZN5boost15throw_exceptionINS_10lock_errorEEEvRKT_")]
pub fn stub_0x3c2a0() -> ! {
    panic!("boost::lock_error (IDA 0x3c2a0)")
}

// 0x3c470 — __ZN5boost10lock_errorD0Ev
// type: void __fastcall(boost::lock_error *__hidden this)
#[doc(alias = "boost::lock_error::~lock_error()")]
#[doc(alias = "__ZN5boost10lock_errorD0Ev")]
pub fn stub_0x3c470() {
    // IDA 0x3c470: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3c4a0 — __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED2Ev
// type: int(void)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED2Ev")]
pub fn stub_0x3c4a0() {
    // IDA 0x3c4a0: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x3c4e0 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_10lock_errorEED1Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
#[doc(alias = "__ZThn20_N5boost16exception_detail19error_info_injectorINS_10lock_errorEED1Ev")]
pub fn stub_0x3c4e0() {
    // IDA 0x3c4e0: __ZThn20 thunk (D1 base dtor): `this -= 20`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3c528 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev")]
pub fn stub_0x3c528() {
    // IDA 0x3c528: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3c570 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev
// type: int(void)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev")]
pub fn stub_0x3c570() {
    // IDA 0x3c570: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3c5b8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE5cloneEv
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone(void)const")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE5cloneEv")]
pub fn stub_0x3c5b8() {
    // IDA 0x3c5b8: `clone_impl<error_info_injector<lock_error>>::clone`
    // heap-clones the exception. Exception-clone glue (`anyhow`
    // covers it); no explicit body.
}

// 0x3c678 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
#[doc(alias = "__ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev")]
pub fn stub_0x3c678() {
    // IDA 0x3c678: __ZThn20 thunk (D0 deleting dtor): `this -= 20`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3c680 — __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED0Ev
#[doc(alias = "boost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED0Ev")]
pub fn stub_0x3c680() {
    // IDA 0x3c680: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3c698 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEE5adoptEPS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::adopt(boost::exception_detail::error_info_container*)")]
#[doc(alias = "__ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEE5adoptEPS2_")]
pub fn stub_0x3c698() {
    // IDA 0x3c698: `refcount_ptr<error_info_container>::adopt` takes
    // ownership of the container. Refcount glue; no explicit body.
}

// 0x3c6c8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEEC1ERKS4_
// type: int __fastcall(int, int, int, int, std::exception *, std::string *, int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::lock_error> const&)")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEEC1ERKS4_")]
pub fn stub_0x3c6c8() {
    // IDA 0x3c6c8: `clone_impl<error_info_injector<lock_error>>::
    // clone_impl` copy-constructs the injector. Exception-clone glue;
    // no explicit body.
}

// 0x3c920 — __ZN3rbx7signals6signalIFvvEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(void)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE22safe_static_init_mutexEv")]
pub fn stub_0x3c920() {
    // IDA 0x3c920: `signal<void()>::safe_static_init_mutex` one-shots
    // the static signal mutex. One-shot init glue; no explicit body.
}

// 0x3c928 — __ZN5boost21thread_resource_errorD1Ev
// type: void __fastcall(boost::thread_resource_error *__hidden this)
#[doc(alias = "boost::thread_resource_error::~thread_resource_error()")]
#[doc(alias = "__ZN5boost21thread_resource_errorD1Ev")]
pub fn stub_0x3c928() {
    // IDA 0x3c928: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3c958 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED2Ev")]
pub fn stub_0x3c958() {
    // IDA 0x3c958: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x3c998 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
#[doc(alias = "__ZThn20_N5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev")]
pub fn stub_0x3c998() {
    // IDA 0x3c998: __ZThn20 thunk (D1 base dtor): `this -= 20`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3c9e0 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev")]
pub fn stub_0x3c9e0() {
    // IDA 0x3c9e0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3ca28 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev
// type: int(void)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev")]
pub fn stub_0x3ca28() {
    // IDA 0x3ca28: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3ca70 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv")]
pub fn stub_0x3ca70() {
    // IDA 0x3ca70: `clone_impl<error_info_injector<thread_resource_error>>::
    // clone` heap-clones the exception (same shape as 0x3c5b8).
    // Exception-clone glue; no explicit body.
}

// 0x3cb30 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
#[doc(alias = "__ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev")]
pub fn stub_0x3cb30() {
    // IDA 0x3cb30: __ZThn20 thunk (D0 deleting dtor): `this -= 20`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3cb38 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const")]
#[doc(alias = "__ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv")]
pub fn stub_0x3cb38() {
    // IDA 0x3cb38: virtual thunk to the `clone` above: this-adjust +
    // tail-call. Rust uses static dispatch; no thunk needed.
}

// 0x3cb48 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED0Ev
#[doc(alias = "boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED0Ev")]
pub fn stub_0x3cb48() {
    // IDA 0x3cb48: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3cb60 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS4_
// type: int __fastcall(int, int, int, int, std::exception *, std::string *, int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::thread_resource_error> const&)")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS4_")]
pub fn stub_0x3cb60() {
    // IDA 0x3cb60: `clone_impl<error_info_injector<thread_resource_error>>::
    // clone_impl` copy-constructs the injector (same shape as 0x3c6c8).
    // Exception-clone glue; no explicit body.
}

// 0x3cdb8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED1Ev")]
pub fn stub_0x3cdb8() {
    // IDA 0x3cdb8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3ce64 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED0Ev")]
pub fn stub_0x3ce64() {
    // IDA 0x3ce64: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3cf18 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x3cf18() {
    // IDA 0x3cf18: `callable<slot,bind_t<mf0<RobloxView>>>::call`
    // invokes the bound view method. Closure-call glue; no explicit
    // body.
}

// 0x3cf20 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x3cf20() {
    // IDA 0x3cf20: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView` — this/arg-adjust + tail-call (this += 12) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x3cf28 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv")]
pub fn stub_0x3cf28() {
    // IDA 0x3cf28: `bind_t<mf0<RobloxView>>::operator()` invokes the
    // view method. Closure-call glue; no explicit body.
}

// 0x3cf40 — __ZN3rbx7signals6signalIFvvEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::remove(rbx::signals::signal<void ()(void)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE6removeEPNS3_4slotE")]
pub fn stub_0x3cf40() {
    // IDA 0x3cf40: `signal<void()>::remove(slot *)` detaches the slot
    // (same shape as 0x3d848). Slot-list glue; no explicit body.
}

// 0x3d030 — __ZN3rbx7signals6signalIFvvEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x3d030() {
    // IDA 0x3d030: `signal<void()>::slot::safe_static_init_mutex`
    // one-shots the static slot mutex (same shape as 0x3c920). One-shot
    // init glue; no explicit body.
}

// 0x3d038 — __ZN3rbx7signals6signalIFvvEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE4slotD1Ev")]
pub fn stub_0x3d038() {
    // IDA 0x3d038: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3d0e4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED1Ev")]
pub fn stub_0x3d0e4() {
    // IDA 0x3d0e4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3d190 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED0Ev")]
pub fn stub_0x3d190() {
    // IDA 0x3d190: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3d240 — __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EE6countsC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0>::counts::counts(void)")]
#[doc(alias = "__ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EE6countsC2Ev")]
pub fn stub_0x3d240() {
    // IDA 0x3d240: `intrusive_ptr_target<islot>::counts::counts`
    // zero-inits the ref counts. Construction glue; no explicit body.
}

// 0x3db4c — __ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ViewBase>(RBX::ViewBase *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_")]
pub fn stub_0x3db4c() -> ! {
    todo!("0x3db4c boost::detail::shared_count::shared_count<RBX::ViewBase>(RBX::ViewBase *)")
}

// 0x3dc40 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED1Ev")]
pub fn stub_0x3dc40() {
    // IDA 0x3dc40: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3dc44 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED0Ev")]
pub fn stub_0x3dc44() {
    // IDA 0x3dc44: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3dc48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE7disposeEv")]
pub fn stub_0x3dc48() -> ! {
    todo!("0x3dc48 boost::detail::sp_counted_impl_p<RBX::ViewBase>::dispose(void)")
}

// 0x3dc58 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE11get_deleterERKSt9type_info")]
pub fn stub_0x3dc58() -> ! {
    todo!("0x3dc58 boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_deleter(std::type_info const&)")
}

// 0x3dc5c — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE19get_untyped_deleterEv")]
pub fn stub_0x3dc5c() -> ! {
    todo!("0x3dc5c boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_untyped_deleter(void)")
}

// 0x3dc60 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(rbx_core::SharedPtr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x3dc60() {
    // IDA 0x3dc60: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x3dd34 — __ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_")]
pub fn stub_0x3dd34() -> ! {
    todo!("0x3dd34 boost::detail::shared_count::shared_count<RobloxView::RenderJob>(RobloxView::RenderJob *)")
}

// 0x3de28 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED1Ev")]
pub fn stub_0x3de28() {
    // IDA 0x3de28: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3de2c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED0Ev")]
pub fn stub_0x3de2c() {
    // IDA 0x3de2c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3de30 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE7disposeEv")]
pub fn stub_0x3de30() -> ! {
    todo!("0x3de30 boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::dispose(void)")
}

// 0x3de40 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE11get_deleterERKSt9type_info")]
pub fn stub_0x3de40() -> ! {
    todo!("0x3de40 boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_deleter(std::type_info const&)")
}

// 0x3de44 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE19get_untyped_deleterEv")]
pub fn stub_0x3de44() -> ! {
    todo!("0x3de44 boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_untyped_deleter(void)")
}

// 0x3de48 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(rbx_core::SharedPtr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x3de48() {
    // IDA 0x3de48: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x3df1c — __ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_")]
pub fn stub_0x3df1c() -> ! {
    todo!("0x3df1c boost::detail::shared_count::shared_count<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")
}

// 0x3e010 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED1Ev")]
pub fn stub_0x3e010() {
    // IDA 0x3e010: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3e014 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEED0Ev")]
pub fn stub_0x3e014() {
    // IDA 0x3e014: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3e018 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE7disposeEv")]
pub fn stub_0x3e018() -> ! {
    todo!("0x3e018 boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::dispose(void)")
}

// 0x3e028 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE11get_deleterERKSt9type_info")]
pub fn stub_0x3e028() -> ! {
    todo!("0x3e028 boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_deleter(std::type_info const&)")
}

// 0x3e02c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN10RobloxView13ViewUpdateJobEE19get_untyped_deleterEv")]
pub fn stub_0x3e02c() -> ! {
    todo!("0x3e02c boost::detail::sp_counted_impl_p<RobloxView::ViewUpdateJob>::get_untyped_deleter(void)")
}

// 0x3e030 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x3e030() -> ! {
    todo!("0x3e030 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x3e090 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_0x3e090() -> ! {
    todo!("0x3e090 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x3e094 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")]
pub fn stub_0x3e094() -> ! {
    todo!("0x3e094 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>::operator()(void)")
}

// 0x3e190 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x3e190() {
    // IDA 0x3e190: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3e198 — __ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_0x3e198() -> ! {
    todo!("0x3e198 boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x3e1e8 — __ZN5boost14singleton_poolIN3RBX16OnDemandInstanceELj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX16OnDemandInstanceELj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_0x3e1e8() -> ! {
    todo!("0x3e1e8 boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x3e238 — __ZN5boost14singleton_poolI10XmlElementLj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<XmlElement,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolI10XmlElementLj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_0x3e238() -> ! {
    todo!("0x3e238 boost::singleton_pool<XmlElement,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x3e288 — __ZN5boost9function0IvE13assign_to_ownERKS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::function0<void>::assign_to_own(boost::function0<void> const&)")]
#[doc(alias = "__ZN5boost9function0IvE13assign_to_ownERKS1_")]
pub fn stub_0x3e288() -> ! {
    todo!("0x3e288 boost::function0<void>::assign_to_own(boost::function0<void> const&)")
}

// 0x3e2b8 — __ZN5boost16exception_detail14bad_exception_D1Ev
// type: void __fastcall(boost::exception_detail::bad_exception_ *__hidden this)
#[doc(alias = "boost::exception_detail::bad_exception_::~bad_exception_()")]
#[doc(alias = "__ZN5boost16exception_detail14bad_exception_D1Ev")]
pub fn stub_0x3e2b8() {
    // IDA 0x3e2b8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3e2e8 — __ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone(void)const")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv")]
pub fn stub_0x3e2e8() -> ! {
    todo!("0x3e2e8 boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone(void)const")
}
