// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x2a6c90..0x2b7e68 | total filtered 10215, remaining 6734 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; `'` stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x2a6c90 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS4_S6_EE
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::erase(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance>*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
// was: std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::erase(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance>*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)
pub fn stub_0x2a6c90() -> ! {
    todo!("0x2a6c90 std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::erase(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance>*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")
}

// 0x2a6e68 — __ZN3rbx7signals16signal_with_argsILi3EFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEEclESsSsS6_
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::signals::signal_with_args<3,void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>::operator()(std::string,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2a6e68() -> ! {
    todo!("0x2a6e68 rbx::signals::signal_with_args<3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2a7c60 — __ZN3RBX15ServiceProvider4findINS_13ScriptContextEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::ScriptContext * RBX::ServiceProvider::find<RBX::ScriptContext>(RBX::Instance const*)")]
pub fn stub_0x2a7c60() -> ! {
    todo!("0x2a7c60 RBX::ScriptContext * RBX::ServiceProvider::find<RBX::ScriptContext>(RBX::Instance const*)")
}

// 0x2a8210 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting13DebuggerWatchEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerWatch>(void)")]
// was: boost::shared_ptr<RBX::Scripting::DebuggerWatch> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerWatch>(void)
pub fn stub_0x2a8210() -> ! {
    todo!("0x2a8210 rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerWatch>(void)")
}

// 0x2a8610 — __ZN5boost10shared_ptrIN3RBX9Scripting13DebuggerWatchEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch>::shared_ptr<RBX::Scripting::DebuggerWatch,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Scripting::DebuggerWatch>::shared_ptr<RBX::Scripting::DebuggerWatch,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x2a8610() -> ! {
    todo!("0x2a8610 rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch>::shared_ptr<RBX::Scripting::DebuggerWatch,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2a87c4 — __ZN5boost6detail12shared_countC2IPN3RBX9Scripting13DebuggerWatchENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2a87c4() -> ! {
    todo!("0x2a87c4 boost::detail::shared_count::shared_count<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2a88cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting13DebuggerWatchENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2a88cc() -> ! {
    todo!("0x2a88cc boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2a88d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting13DebuggerWatchENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2a88d0() -> ! {
    todo!("0x2a88d0 boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2a88d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting13DebuggerWatchENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2a88d4() -> ! {
    todo!("0x2a88d4 boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x2a88f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting13DebuggerWatchENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2a88f4() -> ! {
    todo!("0x2a88f4 boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x2a890c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting13DebuggerWatchENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2a890c() -> ! {
    todo!("0x2a890c boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x2a8d70 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting14ScriptDebuggerEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::ScriptDebugger>(void)")]
// was: boost::shared_ptr<RBX::Scripting::ScriptDebugger> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::ScriptDebugger>(void)
pub fn stub_0x2a8d70() -> ! {
    todo!("0x2a8d70 rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::ScriptDebugger>(void)")
}

// 0x2a9830 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::disconnectAll(void)
pub fn stub_0x2a9830() -> ! {
    todo!("0x2a9830 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")
}

// 0x2a99a8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSERKSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot> const&)
pub fn stub_0x2a99a8() -> ! {
    todo!("0x2a99a8 boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")
}

// 0x2a99d0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::safe_static_do_get_mutex(void)
pub fn stub_0x2a99d0() -> ! {
    todo!("0x2a99d0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")
}

// 0x2a9e50 — __ZN5boost10shared_ptrIN3RBX9Scripting14ScriptDebuggerEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger>::shared_ptr<RBX::Scripting::ScriptDebugger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Scripting::ScriptDebugger>::shared_ptr<RBX::Scripting::ScriptDebugger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x2a9e50() -> ! {
    todo!("0x2a9e50 rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger>::shared_ptr<RBX::Scripting::ScriptDebugger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2aa004 — __ZN5boost6detail12shared_countC2IPN3RBX9Scripting14ScriptDebuggerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2aa004() -> ! {
    todo!("0x2aa004 boost::detail::shared_count::shared_count<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2aa10c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2aa10c() -> ! {
    todo!("0x2aa10c boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2aa110 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2aa110() -> ! {
    todo!("0x2aa110 boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2aa114 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2aa114() -> ! {
    todo!("0x2aa114 boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x2aa134 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2aa134() -> ! {
    todo!("0x2aa134 boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x2aa14c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2aa14c() -> ! {
    todo!("0x2aa14c boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x2aa518 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11LocalScriptEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::LocalScript> RBX::Creatable<RBX::Instance>::create<RBX::LocalScript>(void)")]
// was: boost::shared_ptr<RBX::LocalScript> RBX::Creatable<RBX::Instance>::create<RBX::LocalScript>(void)
pub fn stub_0x2aa518() -> ! {
    todo!("0x2aa518 rbx_core::SharedPtr<RBX::LocalScript> RBX::Creatable<RBX::Instance>::create<RBX::LocalScript>(void)")
}

// 0x2aa5c8 — __ZN5boost10shared_ptrIN3RBX11LocalScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::LocalScript>::shared_ptr<RBX::LocalScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::LocalScript>::shared_ptr<RBX::LocalScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x2aa5c8() -> ! {
    todo!("0x2aa5c8 rbx_core::SharedPtr<RBX::LocalScript>::shared_ptr<RBX::LocalScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2aa780 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LocalScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2aa780() -> ! {
    todo!("0x2aa780 boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2aab78 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE15_M_erase_at_endEPS4_
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::Instance>*)")]
// was: std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_erase_at_end(boost::shared_ptr<RBX::Instance>*)
pub fn stub_0x2aab78() -> ! {
    todo!("0x2aab78 std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::Instance>*)")
}

// 0x2aaba8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance>*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,unsigned long,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance>*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,unsigned long,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0x2aaba8() -> ! {
    todo!("0x2aaba8 std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance>*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,unsigned long,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x2ab1a8 — __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX8InstanceEEEmS4_EvT_T0_RKT1_St12__false_type
#[doc(alias = "void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<RBX::Instance> *,unsigned long,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> *,unsigned long,rbx_core::SharedPtr<RBX::Instance> const&,std::__false_type)")]
// was: void std::__uninitialized_fill_n_aux<boost::shared_ptr<RBX::Instance> *,unsigned long,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> *,unsigned long,boost::shared_ptr<RBX::Instance> const&,std::__false_type)
pub fn stub_0x2ab1a8() -> ! {
    todo!("0x2ab1a8 void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<RBX::Instance> *,unsigned long,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> *,unsigned long,rbx_core::SharedPtr<RBX::Instance> const&,std::__false_type)")
}

// 0x2abdd8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ScriptContextEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptContext> RBX::Creatable<RBX::Instance>::create<RBX::ScriptContext>(void)")]
// was: boost::shared_ptr<RBX::ScriptContext> RBX::Creatable<RBX::Instance>::create<RBX::ScriptContext>(void)
pub fn stub_0x2abdd8() -> ! {
    todo!("0x2abdd8 rbx_core::SharedPtr<RBX::ScriptContext> RBX::Creatable<RBX::Instance>::create<RBX::ScriptContext>(void)")
}

// 0x2abe90 — __ZN5boost6detail12shared_countC2IPN3RBX13ScriptContextENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2abe90() -> ! {
    todo!("0x2abe90 boost::detail::shared_count::shared_count<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2abf98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2abf98() -> ! {
    todo!("0x2abf98 boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2abfa0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2abfa0() -> ! {
    todo!("0x2abfa0 boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x2abfc0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2abfc0() -> ! {
    todo!("0x2abfc0 boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x2abfd8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ScriptContextENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2abfd8() -> ! {
    todo!("0x2abfd8 boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x2adb24 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")]
// was: rbx::signals::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>::slot> &)
pub fn stub_0x2adb24() -> ! {
    todo!("0x2adb24 rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")
}

// 0x2adc84 — __ZN3rbx7signals16signal_with_argsILi3EFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE8fireItemEPNS0_6signalIS7_E4slotESsSsS6_
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::signals::signal_with_args<3,void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>::slot *,std::string,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2adc84() -> ! {
    todo!("0x2adc84 rbx::signals::signal_with_args<3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2ade8c — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
// was: rbx::signals::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>::on_error(std::exception &)
pub fn stub_0x2ade8c() -> ! {
    todo!("0x2ade8c rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")
}

// 0x2adeb4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSERKSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>::slot> const&)
pub fn stub_0x2adeb4() -> ! {
    todo!("0x2adeb4 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")
}

// 0x2aded8 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>::safe_static_init_mutex(void)
pub fn stub_0x2aded8() -> ! {
    todo!("0x2aded8 rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")
}

// 0x2adedc — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>::safe_static_do_get_mutex(void)
pub fn stub_0x2adedc() -> ! {
    todo!("0x2adedc rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")
}

// 0x2ae3c8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13DebugSettingsEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::DebugSettings> RBX::Creatable<RBX::Instance>::create<RBX::DebugSettings>(void)")]
// was: boost::shared_ptr<RBX::DebugSettings> RBX::Creatable<RBX::Instance>::create<RBX::DebugSettings>(void)
pub fn stub_0x2ae3c8() -> ! {
    todo!("0x2ae3c8 rbx_core::SharedPtr<RBX::DebugSettings> RBX::Creatable<RBX::Instance>::create<RBX::DebugSettings>(void)")
}

// 0x2ae478 — __ZN5boost10shared_ptrIN3RBX13DebugSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::DebugSettings>::shared_ptr<RBX::DebugSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::DebugSettings>::shared_ptr<RBX::DebugSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x2ae478() -> ! {
    todo!("0x2ae478 rbx_core::SharedPtr<RBX::DebugSettings>::shared_ptr<RBX::DebugSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2ae630 — __ZN5boost6detail12shared_countC2IPN3RBX13DebugSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2ae630() -> ! {
    todo!("0x2ae630 boost::detail::shared_count::shared_count<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2ae738 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2ae738() -> ! {
    todo!("0x2ae738 boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2ae740 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2ae740() -> ! {
    todo!("0x2ae740 boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x2ae760 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2ae760() -> ! {
    todo!("0x2ae760 boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x2ae778 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2ae778() -> ! {
    todo!("0x2ae778 boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x2aef18 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX8InstanceEEES8_EET0_T_SA_S9_
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *>(rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *)")]
// was: boost::shared_ptr<RBX::Instance> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::Instance> *,boost::shared_ptr<RBX::Instance> *>(boost::shared_ptr<RBX::Instance> *,boost::shared_ptr<RBX::Instance> *,boost::shared_ptr<RBX::Instance> *)
pub fn stub_0x2aef18() -> ! {
    todo!("0x2aef18 rbx_core::SharedPtr<RBX::Instance> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *>(rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *)")
}

// 0x2af428 — __ZN5boost10shared_ptrIN3RBX10CoreScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::CoreScript>::shared_ptr<RBX::CoreScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::CoreScript>::shared_ptr<RBX::CoreScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x2af428() -> ! {
    todo!("0x2af428 rbx_core::SharedPtr<RBX::CoreScript>::shared_ptr<RBX::CoreScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2af5dc — __ZN5boost6detail12shared_countC2IPN3RBX10CoreScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2af5dc() -> ! {
    todo!("0x2af5dc boost::detail::shared_count::shared_count<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2af6e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2af6e4() -> ! {
    todo!("0x2af6e4 boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2af6e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2af6e8() -> ! {
    todo!("0x2af6e8 boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2af6ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2af6ec() -> ! {
    todo!("0x2af6ec boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x2af70c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2af70c() -> ! {
    todo!("0x2af70c boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x2af724 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2af724() -> ! {
    todo!("0x2af724 boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x2af728 — __ZN5boost10shared_ptrIN3RBX13StarterScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::StarterScript>::shared_ptr<RBX::StarterScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::StarterScript>::shared_ptr<RBX::StarterScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x2af728() -> ! {
    todo!("0x2af728 rbx_core::SharedPtr<RBX::StarterScript>::shared_ptr<RBX::StarterScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2af8dc — __ZN5boost6detail12shared_countC2IPN3RBX13StarterScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2af8dc() -> ! {
    todo!("0x2af8dc boost::detail::shared_count::shared_count<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2af9e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2af9e4() -> ! {
    todo!("0x2af9e4 boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2af9e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2af9e8() -> ! {
    todo!("0x2af9e8 boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2af9ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2af9ec() -> ! {
    todo!("0x2af9ec boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x2afa0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2afa0c() -> ! {
    todo!("0x2afa0c boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x2afa24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2afa24() -> ! {
    todo!("0x2afa24 boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x2b07d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2b07d0() -> ! {
    todo!("0x2b07d0 boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2b0e50 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5Stats12StatsServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::StatsService> RBX::Creatable<RBX::Instance>::create<RBX::Stats::StatsService>(void)")]
// was: boost::shared_ptr<RBX::Stats::StatsService> RBX::Creatable<RBX::Instance>::create<RBX::Stats::StatsService>(void)
pub fn stub_0x2b0e50() -> ! {
    todo!("0x2b0e50 rbx_core::SharedPtr<RBX::Stats::StatsService> RBX::Creatable<RBX::Instance>::create<RBX::Stats::StatsService>(void)")
}

// 0x2b0f00 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_5Stats12StatsServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Stats::StatsService>(rbx_core::SharedPtr<RBX::Stats::StatsService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::Stats::StatsService>(boost::shared_ptr<RBX::Stats::StatsService> const&)
pub fn stub_0x2b0f00() -> ! {
    todo!("0x2b0f00 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Stats::StatsService>(rbx_core::SharedPtr<RBX::Stats::StatsService> const&)")
}

// 0x2b1220 — __ZN5boost10shared_ptrIN3RBX5Stats12StatsServiceEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::StatsService>::shared_ptr<RBX::Stats::StatsService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Stats::StatsService>::shared_ptr<RBX::Stats::StatsService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0x2b1220() -> ! {
    todo!("0x2b1220 rbx_core::SharedPtr<RBX::Stats::StatsService>::shared_ptr<RBX::Stats::StatsService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2b13d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2b13d8() -> ! {
    todo!("0x2b13d8 boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2b1828 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15ContentProviderEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ContentProvider> RBX::Creatable<RBX::Instance>::create<RBX::ContentProvider>(void)")]
// was: boost::shared_ptr<RBX::ContentProvider> RBX::Creatable<RBX::Instance>::create<RBX::ContentProvider>(void)
pub fn stub_0x2b1828() -> ! {
    todo!("0x2b1828 rbx_core::SharedPtr<RBX::ContentProvider> RBX::Creatable<RBX::Instance>::create<RBX::ContentProvider>(void)")
}

// 0x2b18d8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_15ContentProviderEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentProvider>(rbx_core::SharedPtr<RBX::ContentProvider> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ContentProvider>(boost::shared_ptr<RBX::ContentProvider> const&)
pub fn stub_0x2b18d8() -> ! {
    todo!("0x2b18d8 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentProvider>(rbx_core::SharedPtr<RBX::ContentProvider> const&)")
}

// 0x2b1920 — __ZN5boost6detail12shared_countC2IPN3RBX15ContentProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2b1920() -> ! {
    todo!("0x2b1920 boost::detail::shared_count::shared_count<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2b1a28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2b1a28() -> ! {
    todo!("0x2b1a28 boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2b1a30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2b1a30() -> ! {
    todo!("0x2b1a30 boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x2b1a50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2b1a50() -> ! {
    todo!("0x2b1a50 boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x2b1a68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2b1a68() -> ! {
    todo!("0x2b1a68 boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x2b4c64 — __ZN5boost10shared_ptrIKSt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_EEPT_
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>::shared_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *)")]
// was: boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *)
pub fn stub_0x2b4c64() -> ! {
    todo!("0x2b4c64 rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>::shared_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *)")
}

// 0x2b4d38 — __ZN5boost6detail12shared_countC2ISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *)")]
// was: boost::detail::shared_count::shared_count<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *)
pub fn stub_0x2b4d38() -> ! {
    todo!("0x2b4d38 boost::detail::shared_count::shared_count<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *)")
}

// 0x2b4e48 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2ERKS6_
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::vector(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
// was: std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::vector(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&)
pub fn stub_0x2b4e48() -> ! {
    todo!("0x2b4e48 std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::vector(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")
}

// 0x2b57e0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE6insertEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_0x2b57e0() -> ! {
    todo!("0x2b57e0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")
}

// 0x2b59ec — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot*)
pub fn stub_0x2b59ec() -> ! {
    todo!("0x2b59ec boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")
}

// 0x2b5a10 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotEEaSERKSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot> const&)
pub fn stub_0x2b5a10() -> ! {
    todo!("0x2b5a10 boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")
}

// 0x2b5a34 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::safe_static_init_mutex(void)
pub fn stub_0x2b5a34() -> ! {
    todo!("0x2b5a34 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")
}

// 0x2b5a38 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::safe_static_do_get_mutex(void)
pub fn stub_0x2b5a38() -> ! {
    todo!("0x2b5a38 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")
}

// 0x2b5b30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_13ScriptContextES6_SsS6_EENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()
pub fn stub_0x2b5b30() -> ! {
    todo!("0x2b5b30 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}

// 0x2b5b5c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_13ScriptContextES6_SsS6_EENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()
pub fn stub_0x2b5b5c() -> ! {
    todo!("0x2b5b5c rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}

// 0x2b5c30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::disconnect(void)
pub fn stub_0x2b5c30() -> ! {
    todo!("0x2b5c30 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")
}

// 0x2b5d40 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::connected(void)const
pub fn stub_0x2b5d40() -> ! {
    todo!("0x2b5d40 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")
}

// 0x2b5d4c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callES7_SsS7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2b5d4c() -> ! {
    todo!("0x2b5d4c rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2b5d68 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callES7_SsS7_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2b5d68() -> ! {
    todo!("0x2b5d68 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2b5d84 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_NS_10shared_ptrINS3_8InstanceEEESsSH_EENS0_5list3IRSH_RSsSK_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: void boost::_bi::list4<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_0x2b5d84() -> ! {
    todo!("0x2b5d84 void boost::_bi::list4<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")
}

// 0x2b5f3c — __ZNK5boost4_mfi3mf3IvN3RBX13ScriptContextENS_10shared_ptrINS2_8InstanceEEESsS6_EclEPS3_S6_SsS6_
#[doc(alias = "boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ScriptContext*,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>::operator()(RBX::ScriptContext*,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)const
pub fn stub_0x2b5f3c() -> ! {
    todo!("0x2b5f3c boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ScriptContext*,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0x2b6104 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE6removeEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_0x2b6104() -> ! {
    todo!("0x2b6104 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")
}

// 0x2b61f4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_init_mutex(void)
pub fn stub_0x2b61f4() -> ! {
    todo!("0x2b61f4 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")
}

// 0x2b61f8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)
pub fn stub_0x2b61f8() -> ! {
    todo!("0x2b61f8 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")
}

// 0x2b62e8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_0x2b62e8() -> ! {
    todo!("0x2b62e8 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")
}

// 0x2b6314 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub fn stub_0x2b6314() -> ! {
    todo!("0x2b6314 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")
}

// 0x2b63e8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_0x2b63e8() -> ! {
    todo!("0x2b63e8 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0x2b6414 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_13ScriptContextES7_SsS7_EENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_0x2b6414() -> ! {
    todo!("0x2b6414 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0x2b7698 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_9WorkspaceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Workspace>(void)")]
pub fn stub_0x2b7698() -> ! {
    todo!("0x2b7698 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Workspace>(void)")
}

// 0x2b7770 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13pushNewObjectIS5_EEPS5_P9lua_StateT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>* RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::pushNewObject<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::shared_ptr<RBX::Instance>* RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::pushNewObject<boost::shared_ptr<RBX::Instance>>(lua_State *,boost::shared_ptr<RBX::Instance>)
pub fn stub_0x2b7770() -> ! {
    todo!("0x2b7770 rbx_core::SharedPtr<RBX::Instance>* RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::pushNewObject<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2b7e68 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_index(lua_State *)
pub fn stub_0x2b7e68() -> ! {
    todo!("0x2b7e68 RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_index(lua_State *)")
}
