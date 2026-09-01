//! rendering shard 263 — 100 stubs EA-sorted asc global gap filler after 0x3631a8 not yet in rendering (Ogre|G3D|Render 15420/15420 complete, 28520->28620 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3631b0 — __ZN5boost21intrusive_ptr_add_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
#[doc(alias = "void boost::intrusive_ptr_add_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
// was: __ZN5boost21intrusive_ptr_add_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
pub fn stub_3631b0() -> ! {
    todo!("0x3631b0 void boost::intrusive_ptr_add_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")
}

// 0x363224 — __ZN3rbx7signals6signalIFvddEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(double,double)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvddEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_363224() -> ! {
    todo!("0x363224 rbx::signals::signal<void ()(double,double)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot> &)")
}

// 0x363384 — __ZN3rbx7signals6signalIFvddEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(double,double)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvddEE8on_errorERSt9exception
pub fn stub_363384() -> ! {
    todo!("0x363384 rbx::signals::signal<void ()(double,double)>::on_error(std::exception &)")
}

// 0x3633ac — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvddEE4slotEEaSERKS7_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvddEE4slotEEaSERKS7_
pub fn stub_3633ac() -> ! {
    todo!("0x3633ac boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot> const&)")
}

// 0x3633d0 — __ZN3rbx7signals6signalIFvddEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(double,double)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvddEE22safe_static_init_mutexEv
pub fn stub_3633d0() -> ! {
    todo!("0x3633d0 rbx::signals::signal<void ()(double,double)>::safe_static_init_mutex(void)")
}

// 0x3633d4 — __ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(double,double)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv
pub fn stub_3633d4() -> ! {
    todo!("0x3633d4 rbx::signals::signal<void ()(double,double)>::safe_static_do_get_mutex(void)")
}

// 0x3634cc — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
pub fn stub_3634cc() -> ! {
    todo!("0x3634cc rbx::signals::signal<void ()(RBX::Stepped const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> &)")
}

// 0x36362c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE8on_errorERSt9exception
pub fn stub_36362c() -> ! {
    todo!("0x36362c rbx::signals::signal<void ()(RBX::Stepped const&)>::on_error(std::exception &)")
}

// 0x363654 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotEEaSERKSB_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotEEaSERKSB_
pub fn stub_363654() -> ! {
    todo!("0x363654 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> const&)")
}

// 0x363678 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE22safe_static_init_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE22safe_static_init_mutexEv
pub fn stub_363678() -> ! {
    todo!("0x363678 rbx::signals::signal<void ()(RBX::Stepped const&)>::safe_static_init_mutex(void)")
}

// 0x36367c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv
pub fn stub_36367c() -> ! {
    todo!("0x36367c rbx::signals::signal<void ()(RBX::Stepped const&)>::safe_static_do_get_mutex(void)")
}

// 0x363774 — __ZN3rbx7signals6signalIFvdEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(double)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvdEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_363774() -> ! {
    todo!("0x363774 rbx::signals::signal<void ()(double)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot> &)")
}

// 0x3638d4 — __ZN3rbx7signals6signalIFvdEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(double)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvdEE8on_errorERSt9exception
pub fn stub_3638d4() -> ! {
    todo!("0x3638d4 rbx::signals::signal<void ()(double)>::on_error(std::exception &)")
}

// 0x3638fc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvdEE4slotEEaSERKS7_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvdEE4slotEEaSERKS7_
pub fn stub_3638fc() -> ! {
    todo!("0x3638fc boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot> const&)")
}

// 0x363920 — __ZN3rbx7signals6signalIFvdEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(double)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvdEE22safe_static_init_mutexEv
pub fn stub_363920() -> ! {
    todo!("0x363920 rbx::signals::signal<void ()(double)>::safe_static_init_mutex(void)")
}

// 0x363924 — __ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(double)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv
pub fn stub_363924() -> ! {
    todo!("0x363924 rbx::signals::signal<void ()(double)>::safe_static_do_get_mutex(void)")
}

// 0x363a1c — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
pub fn stub_363a1c() -> ! {
    todo!("0x363a1c rbx::signals::signal<void ()(RBX::Heartbeat const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> &)")
}

// 0x363b7c — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE8on_errorERSt9exception
pub fn stub_363b7c() -> ! {
    todo!("0x363b7c rbx::signals::signal<void ()(RBX::Heartbeat const&)>::on_error(std::exception &)")
}

// 0x363ba8 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE22safe_static_init_mutexEv
pub fn stub_363ba8() -> ! {
    todo!("0x363ba8 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_init_mutex(void)")
}

// 0x363bac — __ZN5boost10shared_ptrIN3RBX13HeartbeatTaskEEC2IS2_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::HeartbeatTask>::shared_ptr<RBX::HeartbeatTask>(RBX::HeartbeatTask *)")]
// was: __ZN5boost10shared_ptrIN3RBX13HeartbeatTaskEEC2IS2_EEPT_
pub fn stub_363bac() -> ! {
    todo!("0x363bac boost::shared_ptr<RBX::HeartbeatTask>::shared_ptr<RBX::HeartbeatTask>(RBX::HeartbeatTask *)")
}

// 0x363c94 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_13HeartbeatTaskES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::HeartbeatTask,RBX::HeartbeatTask>(boost::shared_ptr<RBX::HeartbeatTask> const*,RBX::HeartbeatTask *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_13HeartbeatTaskES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_363c94() -> ! {
    todo!("0x363c94 void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::HeartbeatTask,RBX::HeartbeatTask>(boost::shared_ptr<RBX::HeartbeatTask> const*,RBX::HeartbeatTask *)const")
}

// 0x363d78 — __ZN5boost6detail12shared_countC2IN3RBX13HeartbeatTaskEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HeartbeatTask>(RBX::HeartbeatTask *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX13HeartbeatTaskEEEPT_
pub fn stub_363d78() -> ! {
    todo!("0x363d78 boost::detail::shared_count::shared_count<RBX::HeartbeatTask>(RBX::HeartbeatTask *)")
}

// 0x363e70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEED1Ev
pub fn stub_363e70() -> ! {
    todo!("0x363e70 boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::~sp_counted_impl_p()")
}

// 0x363e74 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEED0Ev
pub fn stub_363e74() -> ! {
    todo!("0x363e74 boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::~sp_counted_impl_p()")
}

// 0x363e78 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEE7disposeEv
pub fn stub_363e78() -> ! {
    todo!("0x363e78 boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::dispose(void)")
}

// 0x363e88 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEE11get_deleterERKSt9type_info
pub fn stub_363e88() -> ! {
    todo!("0x363e88 boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::get_deleter(std::type_info const&)")
}

// 0x363e8c — __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEE19get_untyped_deleterEv
pub fn stub_363e8c() -> ! {
    todo!("0x363e8c boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::get_untyped_deleter(void)")
}

// 0x363e90 — __ZN5boost15throw_exceptionINS_12bad_weak_ptrEEEvRKT_
#[doc(alias = "void boost::throw_exception<boost::bad_weak_ptr>(boost::bad_weak_ptr const&)")]
// was: __ZN5boost15throw_exceptionINS_12bad_weak_ptrEEEvRKT_
pub fn stub_363e90() -> ! {
    todo!("0x363e90 void boost::throw_exception<boost::bad_weak_ptr>(boost::bad_weak_ptr const&)")
}

// 0x363f78 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev
pub fn stub_363f78() -> ! {
    todo!("0x363f78 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")
}

// 0x363f90 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE5cloneEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone(void)const")]
// was: __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE5cloneEv
pub fn stub_363f90() -> ! {
    todo!("0x363f90 `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone(void)const")
}

// 0x363f9c — __ZN5boost10shared_ptrIN3RBX10PhysicsJobEEC2IS2_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::PhysicsJob>::shared_ptr<RBX::PhysicsJob>(RBX::PhysicsJob *)")]
// was: __ZN5boost10shared_ptrIN3RBX10PhysicsJobEEC2IS2_EEPT_
pub fn stub_363f9c() -> ! {
    todo!("0x363f9c boost::shared_ptr<RBX::PhysicsJob>::shared_ptr<RBX::PhysicsJob>(RBX::PhysicsJob *)")
}

// 0x364084 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10PhysicsJobES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::PhysicsJob,RBX::PhysicsJob>(boost::shared_ptr<RBX::PhysicsJob> const*,RBX::PhysicsJob *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10PhysicsJobES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_364084() -> ! {
    todo!("0x364084 void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::PhysicsJob,RBX::PhysicsJob>(boost::shared_ptr<RBX::PhysicsJob> const*,RBX::PhysicsJob *)const")
}

// 0x364168 — __ZN5boost6detail12shared_countC2IN3RBX10PhysicsJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PhysicsJob>(RBX::PhysicsJob *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX10PhysicsJobEEEPT_
pub fn stub_364168() -> ! {
    todo!("0x364168 boost::detail::shared_count::shared_count<RBX::PhysicsJob>(RBX::PhysicsJob *)")
}

// 0x364260 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEED1Ev
pub fn stub_364260() -> ! {
    todo!("0x364260 boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::~sp_counted_impl_p()")
}

// 0x364264 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEED0Ev
pub fn stub_364264() -> ! {
    todo!("0x364264 boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::~sp_counted_impl_p()")
}

// 0x364268 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEE7disposeEv
pub fn stub_364268() -> ! {
    todo!("0x364268 boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::dispose(void)")
}

// 0x364278 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEE11get_deleterERKSt9type_info
pub fn stub_364278() -> ! {
    todo!("0x364278 boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::get_deleter(std::type_info const&)")
}

// 0x36427c — __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEE19get_untyped_deleterEv
pub fn stub_36427c() -> ! {
    todo!("0x36427c boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::get_untyped_deleter(void)")
}

// 0x364280 — __ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_364280() -> ! {
    todo!("0x364280 __ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x36439c — __ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_36439c() -> ! {
    todo!("0x36439c __ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3643a0 — __ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3643a0() -> ! {
    todo!("0x3643a0 __ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x364440 — __ZThn32_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_364440() -> ! {
    todo!("0x364440 __ZThn32_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x364448 — __ZThn32_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_364448() -> ! {
    todo!("0x364448 __ZThn32_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3644ec — __ZThn36_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3644ec() -> ! {
    todo!("0x3644ec __ZThn36_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3644f4 — __ZThn36_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3644f4() -> ! {
    todo!("0x3644f4 __ZThn36_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x364598 — __ZN3RBX10Reflection15ClassDescriptor14rootDescriptorEv
// type: _DWORD __fastcall(RBX::Reflection::ClassDescriptor *__hidden this)
#[doc(alias = "RBX::Reflection::ClassDescriptor::rootDescriptor(void)")]
// was: __ZN3RBX10Reflection15ClassDescriptor14rootDescriptorEv
pub fn stub_364598() -> ! {
    todo!("0x364598 RBX::Reflection::ClassDescriptor::rootDescriptor(void)")
}

// 0x364688 — __ZN5boost20dynamic_pointer_castIN3RBX9DataModelENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE
#[doc(alias = "boost::shared_ptr<RBX::DataModel> boost::dynamic_pointer_cast<RBX::DataModel,RBX::Reflection::DescribedBase>(boost::shared_ptr<RBX::Reflection::DescribedBase> const&)")]
// was: __ZN5boost20dynamic_pointer_castIN3RBX9DataModelENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE
pub fn stub_364688() -> ! {
    todo!("0x364688 boost::shared_ptr<RBX::DataModel> boost::dynamic_pointer_cast<RBX::DataModel,RBX::Reflection::DescribedBase>(boost::shared_ptr<RBX::Reflection::DescribedBase> const&)")
}

// 0x3646d0 — __ZN3RBX10Reflection13BoundFuncDescINS_10RunServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RunService,void ()(void),0>::BoundFuncDesc(void (RBX::RunService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10RunServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3646d0() -> ! {
    todo!("0x3646d0 RBX::Reflection::BoundFuncDesc<RBX::RunService,void ()(void),0>::BoundFuncDesc(void (RBX::RunService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3647d4 — __ZN3RBX10Reflection13BoundFuncDescINS_10RunServiceEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RunService,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10RunServiceEFvvELi0EED0Ev
pub fn stub_3647d4() -> ! {
    todo!("0x3647d4 RBX::Reflection::BoundFuncDesc<RBX::RunService,void ()(void),0>::~BoundFuncDesc()")
}

// 0x364888 — __ZNK3RBX10Reflection13BoundFuncDescINS_10RunServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RunService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_10RunServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_364888() -> ! {
    todo!("0x364888 RBX::Reflection::BoundFuncDesc<RBX::RunService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x3648ac — __ZN3RBX10Reflection9EventDescINS_10RunServiceEFvdEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::EventDesc(rbx::signal<void ()(double)> RBX::RunService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_10RunServiceEFvdEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3648ac() -> ! {
    todo!("0x3648ac RBX::Reflection::EventDesc<RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::EventDesc(rbx::signal<void ()(double)> RBX::RunService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x364a30 — __ZN3RBX10Reflection9EventDescINS_10RunServiceEFvdEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10RunServiceEFvdEN3rbx6signalIS3_EEMS2_S6_ED0Ev
pub fn stub_364a30() -> ! {
    todo!("0x364a30 RBX::Reflection::EventDesc<RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::~EventDesc()")
}

// 0x364ae4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10RunServiceEFvdEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_10RunServiceEFvdEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_364ae4() -> ! {
    todo!("0x364ae4 RBX::Reflection::EventDescImpl<1,RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x364c38 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10RunServiceEFvdEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_10RunServiceEFvdEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
pub fn stub_364c38() -> ! {
    todo!("0x364c38 RBX::Reflection::EventDescImpl<1,RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x364cc8 — __ZNK3RBX10Reflection13EventDescBaseINS_10RunServiceEFvdEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_10RunServiceEFvdEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_364cc8() -> ! {
    todo!("0x364cc8 RBX::Reflection::EventDescBase<RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x364cdc — __ZN3rbx7signals6signalIFvdEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(double)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvdEE13disconnectAllEv
pub fn stub_364cdc() -> ! {
    todo!("0x364cdc rbx::signals::signal<void ()(double)>::disconnectAll(void)")
}

// 0x364e58 — __ZN3rbx22bad_placement_any_castD1Ev
// type: void __fastcall(rbx::bad_placement_any_cast *__hidden this)
#[doc(alias = "rbx::bad_placement_any_cast::~bad_placement_any_cast()")]
// was: __ZN3rbx22bad_placement_any_castD1Ev
pub fn stub_364e58() -> ! {
    todo!("0x364e58 rbx::bad_placement_any_cast::~bad_placement_any_cast()")
}

// 0x364e60 — __ZNK3rbx22bad_placement_any_cast4whatEv
// type: _DWORD __fastcall(rbx::bad_placement_any_cast *__hidden this)
#[doc(alias = "rbx::bad_placement_any_cast::what(void)const")]
// was: __ZNK3rbx22bad_placement_any_cast4whatEv
pub fn stub_364e60() -> ! {
    todo!("0x364e60 rbx::bad_placement_any_cast::what(void)const")
}

// 0x364e70 — __ZN5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED2Ev
pub fn stub_364e70() -> ! {
    todo!("0x364e70 boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()")
}

// 0x364f28 — __ZThn4_N5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED1Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()")]
// was: __ZThn4_N5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED1Ev
pub fn stub_364f28() -> ! {
    todo!("0x364f28 `non-virtual thunk to'boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()")
}

// 0x364f30 — __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEED1Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()")]
// was: __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEED1Ev
pub fn stub_364f30() -> ! {
    todo!("0x364f30 `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()")
}

// 0x364f38 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEED1Ev
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()")]
// was: __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEED1Ev
pub fn stub_364f38() -> ! {
    todo!("0x364f38 `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()")
}

// 0x364f48 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE5cloneEv
// type: char *__fastcall(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone(void)const")]
// was: __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE5cloneEv
pub fn stub_364f48() -> ! {
    todo!("0x364f48 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone(void)const")
}

// 0x365008 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE7rethrowEv
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::rethrow(void)const")]
// was: __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE7rethrowEv
pub fn stub_365008() -> ! {
    todo!("0x365008 `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::rethrow(void)const")
}

// 0x365018 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEED0Ev
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()")]
// was: __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEED0Ev
pub fn stub_365018() -> ! {
    todo!("0x365018 `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()")
}

// 0x365038 — __ZThn4_N5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED0Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()")]
// was: __ZThn4_N5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED0Ev
pub fn stub_365038() -> ! {
    todo!("0x365038 `non-virtual thunk to'boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()")
}

// 0x365050 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEEC1ERKS6_NS6_9clone_tagE
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_tag)")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEEC1ERKS6_NS6_9clone_tagE
pub fn stub_365050() -> ! {
    todo!("0x365050 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_tag)")
}

// 0x365188 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEEC1ERKS5_
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_impl(boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast> const&)")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEEC1ERKS5_
pub fn stub_365188() -> ! {
    todo!("0x365188 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_impl(boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast> const&)")
}

// 0x3652c0 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKdNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,double const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(double const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKdNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_
pub fn stub_3652c0() -> ! {
    todo!("0x3652c0 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,double const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(double const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")
}

// 0x3653dc — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IdEEvRKT_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<double>(double const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute1IdEEvRKT_
pub fn stub_3653dc() -> ! {
    todo!("0x3653dc void RBX::Reflection::GenericSlotWrapper::execute1<double>(double const&)")
}

// 0x365520 — __ZN5boost9function1IvdE5clearEv
#[doc(alias = "boost::function1<void,double>::clear(void)")]
// was: __ZN5boost9function1IvdE5clearEv
pub fn stub_365520() -> ! {
    todo!("0x365520 boost::function1<void,double>::clear(void)")
}

// 0x365550 — __ZN3rbx14implementation12typed_holderIdE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<double>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIdE9singletonEv
pub fn stub_365550() -> ! {
    todo!("0x365550 rbx::implementation::typed_holder<double>::singleton(void)")
}

// 0x3655c0 — __ZNSt6vectorIN3RBX10Reflection7VariantESaIS2_EEC2EmRKS2_RKS3_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::vector(unsigned long,RBX::Reflection::Variant const&,std::allocator<RBX::Reflection::Variant> const&)")]
// was: __ZNSt6vectorIN3RBX10Reflection7VariantESaIS2_EEC2EmRKS2_RKS3_
pub fn stub_3655c0() -> ! {
    todo!("0x3655c0 std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::vector(unsigned long,RBX::Reflection::Variant const&,std::allocator<RBX::Reflection::Variant> const&)")
}

// 0x365690 — __ZSt26__uninitialized_fill_n_auxIPN3RBX10Reflection7VariantEmS2_EvT_T0_RKT1_St12__false_type
// type: void __fastcall(int, int, _DWORD *, int, int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::Reflection::Variant *,unsigned long,RBX::Reflection::Variant>(RBX::Reflection::Variant *,unsigned long,RBX::Reflection::Variant const&,std::__false_type)")]
// was: __ZSt26__uninitialized_fill_n_auxIPN3RBX10Reflection7VariantEmS2_EvT_T0_RKT1_St12__false_type
pub fn stub_365690() -> ! {
    todo!("0x365690 void std::__uninitialized_fill_n_aux<RBX::Reflection::Variant *,unsigned long,RBX::Reflection::Variant>(RBX::Reflection::Variant *,unsigned long,RBX::Reflection::Variant const&,std::__false_type)")
}

// 0x3657d0 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEEEC2ES8_SA_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>::list2(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEEEC2ES8_SA_
pub fn stub_3657d0() -> ! {
    todo!("0x3657d0 boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>::list2(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>)")
}

// 0x3658a0 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEEEC2ES8_SA_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEEEC2ES8_SA_
pub fn stub_3658a0() -> ! {
    todo!("0x3658a0 boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>)")
}

// 0x365980 — __ZN5boost8functionIFvdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
pub fn stub_365980() -> ! {
    todo!("0x365980 __ZN5boost8functionIFvdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0x365a64 — __ZN5boost9function1IvdEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvdEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvdEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_365a64() -> ! {
    todo!("0x365a64 __ZN5boost9function1IvdEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

// 0x365b4c — __ZN5boost9function1IvdE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,double>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// was: __ZN5boost9function1IvdE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
pub fn stub_365b4c() -> ! {
    todo!("0x365b4c void boost::function1<void,double>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")
}

// 0x365c44 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
pub fn stub_365c44() -> ! {
    todo!("0x365c44 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x365c60 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvdE6invokeERNS1_15function_bufferEd
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,double>::invoke(boost::detail::function::function_buffer &,double)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvdE6invokeERNS1_15function_bufferEd
pub fn stub_365c60() -> ! {
    todo!("0x365c60 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,double>::invoke(boost::detail::function::function_buffer &,double)")
}

// 0x365c80 — __ZNK5boost6detail8function13basic_vtable1IvdE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,double>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvdE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_365c80() -> ! {
    todo!("0x365c80 bool boost::detail::function::basic_vtable1<void,double>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x365d68 — __ZNK5boost6detail8function13basic_vtable1IvdE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,double>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvdE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_365d68() -> ! {
    todo!("0x365d68 bool boost::detail::function::basic_vtable1<void,double>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x365e4c — __ZNK5boost6detail8function13basic_vtable1IvdE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,double>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvdE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_365e4c() -> ! {
    todo!("0x365e4c void boost::detail::function::basic_vtable1<void,double>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x365f20 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIdEEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<double>(double &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIdEEvRT_
pub fn stub_365f20() -> ! {
    todo!("0x365f20 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<double>(double &)")
}

// 0x365f38 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_365f38() -> ! {
    todo!("0x365f38 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x366090 — __ZN3rbx7signals6signalIFvdEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(double)>::connect<boost::function<void ()(double)>>(boost::function<void ()(double)> const&)")]
// was: __ZN3rbx7signals6signalIFvdEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_366090() -> ! {
    todo!("0x366090 rbx::signals::connection rbx::signals::signal<void ()(double)>::connect<boost::function<void ()(double)>>(boost::function<void ()(double)> const&)")
}

// 0x366184 — __ZN3rbx7signals6signalIFvdEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(double)>::insert(rbx::signals::signal<void ()(double)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvdEE6insertEPNS3_4slotE
pub fn stub_366184() -> ! {
    todo!("0x366184 rbx::signals::signal<void ()(double)>::insert(rbx::signals::signal<void ()(double)>::slot *)")
}

// 0x366390 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvdEE4slotEEaSEPS6_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot>::operator=(rbx::signals::signal<void ()(double)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvdEE4slotEEaSEPS6_
pub fn stub_366390() -> ! {
    todo!("0x366390 boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot>::operator=(rbx::signals::signal<void ()(double)>::slot*)")
}

// 0x3663b4 — __ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::callable<rbx::signals::signal<void ()(double)>*>(boost::function<void ()(double)> const&,rbx::signals::signal<void ()(double)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
pub fn stub_3663b4() -> ! {
    todo!("0x3663b4 rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::callable<rbx::signals::signal<void ()(double)>*>(boost::function<void ()(double)> const&,rbx::signals::signal<void ()(double)>*)")
}

// 0x3664b0 — __ZN3rbx7signals6signalIFvdEE13callable_slotIN5boost8functionIS2_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(double)>::callable_slot<boost::function<void ()(double)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvdEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_3664b0() -> ! {
    todo!("0x3664b0 rbx::signals::signal<void ()(double)>::callable_slot<boost::function<void ()(double)>>::~callable_slot()")
}

// 0x3665c0 — __ZN3rbx7signals6signalIFvdEE13callable_slotIN5boost8functionIS2_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(double)>::callable_slot<boost::function<void ()(double)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvdEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_3665c0() -> ! {
    todo!("0x3665c0 rbx::signals::signal<void ()(double)>::callable_slot<boost::function<void ()(double)>>::~callable_slot()")
}

// 0x3666f0 — __ZN3rbx7signals6signalIFvdEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvdEE4slot10disconnectEv
pub fn stub_3666f0() -> ! {
    todo!("0x3666f0 rbx::signals::signal<void ()(double)>::slot::disconnect(void)")
}

// 0x366800 — __ZNK3rbx7signals6signalIFvdEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvdEE4slot9connectedEv
pub fn stub_366800() -> ! {
    todo!("0x366800 rbx::signals::signal<void ()(double)>::slot::connected(void)const")
}

// 0x36680c — __ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_E4callEd
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::call(double)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_E4callEd
pub fn stub_36680c() -> ! {
    todo!("0x36680c rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::call(double)")
}

// 0x366814 — __ZThn4_N3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_E4callEd
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::call(double)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_E4callEd
pub fn stub_366814() -> ! {
    todo!("0x366814 `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::call(double)")
}

// 0x36681c — __ZNK5boost9function1IvdEclEd
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "boost::function1<void,double>::operator()(double)const")]
// was: __ZNK5boost9function1IvdEclEd
pub fn stub_36681c() -> ! {
    todo!("0x36681c boost::function1<void,double>::operator()(double)const")
}

// 0x3668e8 — __ZN3rbx7signals6signalIFvdEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(double)>::remove(rbx::signals::signal<void ()(double)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvdEE6removeEPNS3_4slotE
pub fn stub_3668e8() -> ! {
    todo!("0x3668e8 rbx::signals::signal<void ()(double)>::remove(rbx::signals::signal<void ()(double)>::slot *)")
}

// 0x3669d8 — __ZN3rbx7signals6signalIFvdEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvdEE4slot22safe_static_init_mutexEv
pub fn stub_3669d8() -> ! {
    todo!("0x3669d8 rbx::signals::signal<void ()(double)>::slot::safe_static_init_mutex(void)")
}

// 0x3669dc — __ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv
pub fn stub_3669dc() -> ! {
    todo!("0x3669dc rbx::signals::signal<void ()(double)>::slot::safe_static_do_get_mutex(void)")
}

