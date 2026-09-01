//! core shard IS — 100 core stubs EA-sorted, 0x258fbc..0x369c54 (strict RBX|boost excluding Reflection|Instance|DataModel|Ogre|G3D|Rendering|Adorn|RakNet|Network|Replicat|Socket|Sound|Audio|FMOD|Script|Lua|ViewController|UIApplication|Platform|iOS, EA-sorted ascending, next 100 uncovered after 0x25104c prior 13946 remaining).
//! Source: `ida/export.json` filtered where demangled NOT containing Reflection|Instance|DataModel|Ogre|G3D|Rendering|Adorn|RakNet|Network|Replicat|Socket|Sound|Audio|FMOD|Script|Lua|lua|ViewController|UIApplication|Platform|iOS but containing RBX:: or boost::, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::resize(unsigned long,RBX::HttpService::HttpContentType)")]
// 0x258fbc — __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
pub fn stub_0x258fbc() -> ! {
    todo!("0x258fbc __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::push_back(RBX::HttpService::HttpContentType const&)")]
// 0x258ff0 — __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x258ff0() -> ! {
    todo!("0x258ff0 __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::HttpService::HttpContentType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::operator[](RBX::Name const* const&)")]
// 0x259018 — __ZNSt3mapIPKN3RBX4NameENS0_11HttpService15HttpContentTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
pub fn stub_0x259018() -> ! {
    todo!("0x259018 __ZNSt3mapIPKN3RBX4NameENS0_11HttpService15HttpContentTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
// 0x259070 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
pub fn stub_0x259070() -> ! {
    todo!("0x259070 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
// 0x259124 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_0x259124() -> ! {
    todo!("0x259124 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
// 0x25917c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
pub fn stub_0x25917c() -> ! {
    todo!("0x25917c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,RBX::HttpService::HttpContentType const&)")]
// 0x2591e4 — __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
pub fn stub_0x2591e4() -> ! {
    todo!("0x2591e4 __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_allocate(unsigned long)")]
// 0x2592c8 — __ZNSt12_Vector_baseIN3RBX11HttpService15HttpContentTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x2592c8() -> ! {
    todo!("0x2592c8 __ZNSt12_Vector_baseIN3RBX11HttpService15HttpContentTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::HttpService::HttpContentType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *>(RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *)")]
// 0x2592e0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11HttpService15HttpContentTypeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
pub fn stub_0x2592e0() -> ! {
    todo!("0x2592e0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11HttpService15HttpContentTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,unsigned long,RBX::HttpService::HttpContentType const&)")]
// 0x25931c — __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
pub fn stub_0x25931c() -> ! {
    todo!("0x25931c __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Heartbeat const&)>::operator()(RBX::Heartbeat const&)")]
// 0x361c20 — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9HeartbeatEEEclES5_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0x361c20() -> ! {
    todo!("0x361c20 __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9HeartbeatEEEclES5_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(double)>::operator()(double)")]
// 0x361d64 — __ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
pub fn stub_0x361d64() -> ! {
    todo!("0x361d64 __ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Stepped const&)>::operator()(RBX::Stepped const&)")]
// 0x361eb0 — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7SteppedEEEclES5_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0x361eb0() -> ! {
    todo!("0x361eb0 __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7SteppedEEEclES5_")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(double,double)>::operator()(double,double)")]
// 0x361ff4 — __ZN3rbx7signals16signal_with_argsILi2EFvddEEclEdd
// type: void __fastcall(_DWORD *, int, int, const void *, int)
pub fn stub_0x361ff4() -> ! {
    todo!("0x361ff4 __ZN3rbx7signals16signal_with_argsILi2EFvddEEclEdd")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::RunTransition)>::operator()(RBX::RunTransition)")]
// 0x362158 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_
pub fn stub_0x362158() -> ! {
    todo!("0x362158 __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::on_error(std::exception &)")]
// 0x362924 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE8on_errorERSt9exception
pub fn stub_0x362924() -> ! {
    todo!("0x362924 __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::safe_static_init_mutex(void)")]
// 0x3631a8 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE22safe_static_init_mutexEv
pub fn stub_0x3631a8() -> ! {
    todo!("0x3631a8 __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::on_error(std::exception &)")]
// 0x363384 — __ZN3rbx7signals6signalIFvddEE8on_errorERSt9exception
pub fn stub_0x363384() -> ! {
    todo!("0x363384 __ZN3rbx7signals6signalIFvddEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::safe_static_init_mutex(void)")]
// 0x3633d0 — __ZN3rbx7signals6signalIFvddEE22safe_static_init_mutexEv
pub fn stub_0x3633d0() -> ! {
    todo!("0x3633d0 __ZN3rbx7signals6signalIFvddEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::safe_static_do_get_mutex(void)")]
// 0x3633d4 — __ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv
// type: int()
pub fn stub_0x3633d4() -> ! {
    todo!("0x3633d4 __ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::on_error(std::exception &)")]
// 0x36362c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE8on_errorERSt9exception
pub fn stub_0x36362c() -> ! {
    todo!("0x36362c __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::safe_static_init_mutex(void)")]
// 0x363678 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE22safe_static_init_mutexEv
// type: int()
pub fn stub_0x363678() -> ! {
    todo!("0x363678 __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::safe_static_do_get_mutex(void)")]
// 0x36367c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv
pub fn stub_0x36367c() -> ! {
    todo!("0x36367c __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::on_error(std::exception &)")]
// 0x3638d4 — __ZN3rbx7signals6signalIFvdEE8on_errorERSt9exception
pub fn stub_0x3638d4() -> ! {
    todo!("0x3638d4 __ZN3rbx7signals6signalIFvdEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::safe_static_init_mutex(void)")]
// 0x363920 — __ZN3rbx7signals6signalIFvdEE22safe_static_init_mutexEv
pub fn stub_0x363920() -> ! {
    todo!("0x363920 __ZN3rbx7signals6signalIFvdEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::safe_static_do_get_mutex(void)")]
// 0x363924 — __ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv
pub fn stub_0x363924() -> ! {
    todo!("0x363924 __ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::on_error(std::exception &)")]
// 0x363b7c — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE8on_errorERSt9exception
pub fn stub_0x363b7c() -> ! {
    todo!("0x363b7c __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_init_mutex(void)")]
// 0x363ba8 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE22safe_static_init_mutexEv
pub fn stub_0x363ba8() -> ! {
    todo!("0x363ba8 __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE22safe_static_init_mutexEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::get_deleter(std::type_info const&)")]
// 0x363e88 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEE11get_deleterERKSt9type_info
pub fn stub_0x363e88() -> ! {
    todo!("0x363e88 __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::get_untyped_deleter(void)")]
// 0x363e8c — __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEE19get_untyped_deleterEv
pub fn stub_0x363e8c() -> ! {
    todo!("0x363e8c __ZN5boost6detail17sp_counted_impl_pIN3RBX13HeartbeatTaskEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::throw_exception<boost::bad_weak_ptr>(boost::bad_weak_ptr const&)")]
// 0x363e90 — __ZN5boost15throw_exceptionINS_12bad_weak_ptrEEEvRKT_
pub fn stub_0x363e90() -> ! {
    todo!("0x363e90 __ZN5boost15throw_exceptionINS_12bad_weak_ptrEEEvRKT_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
// 0x363f78 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev
// type: int __fastcall(int, int, int, int)
pub fn stub_0x363f78() -> ! {
    todo!("0x363f78 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone(void)const")]
// 0x363f90 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE5cloneEv
// type: int __fastcall(_DWORD *)
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone(void)const
pub fn stub_0x363f90() -> ! {
    todo!("0x363f90 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE5cloneEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsJob>::shared_ptr<RBX::PhysicsJob>(RBX::PhysicsJob *)")]
// 0x363f9c — __ZN5boost10shared_ptrIN3RBX10PhysicsJobEEC2IS2_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
// was: boost::shared_ptr<RBX::PhysicsJob>::shared_ptr<RBX::PhysicsJob>(RBX::PhysicsJob *)
pub fn stub_0x363f9c() -> ! {
    todo!("0x363f9c __ZN5boost10shared_ptrIN3RBX10PhysicsJobEEC2IS2_EEPT_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::PhysicsJob,RBX::PhysicsJob>(rbx_core::SharedPtr<RBX::PhysicsJob> const*,RBX::PhysicsJob *)const")]
// 0x364084 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10PhysicsJobES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::PhysicsJob,RBX::PhysicsJob>(boost::shared_ptr<RBX::PhysicsJob> const*,RBX::PhysicsJob *)const
pub fn stub_0x364084() -> ! {
    todo!("0x364084 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10PhysicsJobES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PhysicsJob>(RBX::PhysicsJob *)")]
// 0x364168 — __ZN5boost6detail12shared_countC2IN3RBX10PhysicsJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x364168() -> ! {
    todo!("0x364168 __ZN5boost6detail12shared_countC2IN3RBX10PhysicsJobEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::~sp_counted_impl_p()")]
// 0x364260 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEED1Ev
pub fn stub_0x364260() -> ! {
    todo!("0x364260 __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::~sp_counted_impl_p()")]
// 0x364264 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEED0Ev
pub fn stub_0x364264() -> ! {
    todo!("0x364264 __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::dispose(void)")]
// 0x364268 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEE7disposeEv
pub fn stub_0x364268() -> ! {
    todo!("0x364268 __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::get_deleter(std::type_info const&)")]
// 0x364278 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEE11get_deleterERKSt9type_info
pub fn stub_0x364278() -> ! {
    todo!("0x364278 __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::get_untyped_deleter(void)")]
// 0x36427c — __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEE19get_untyped_deleterEv
pub fn stub_0x36427c() -> ! {
    todo!("0x36427c __ZN5boost6detail17sp_counted_impl_pIN3RBX10PhysicsJobEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::disconnectAll(void)")]
// 0x364cdc — __ZN3rbx7signals6signalIFvdEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x364cdc() -> ! {
    todo!("0x364cdc __ZN3rbx7signals6signalIFvdEE13disconnectAllEv")
}

#[doc(alias = "rbx::bad_placement_any_cast::~bad_placement_any_cast()")]
// 0x364e58 — __ZN3rbx22bad_placement_any_castD1Ev
// type: void __fastcall(rbx::bad_placement_any_cast *__hidden this)
pub fn stub_0x364e58() -> ! {
    todo!("0x364e58 __ZN3rbx22bad_placement_any_castD1Ev")
}

#[doc(alias = "rbx::bad_placement_any_cast::what(void)const")]
// 0x364e60 — __ZNK3rbx22bad_placement_any_cast4whatEv
// type: _DWORD __fastcall(rbx::bad_placement_any_cast *__hidden this)
pub fn stub_0x364e60() -> ! {
    todo!("0x364e60 __ZNK3rbx22bad_placement_any_cast4whatEv")
}

#[doc(alias = "boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()")]
// 0x364e70 — __ZN5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x364e70() -> ! {
    todo!("0x364e70 __ZN5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED2Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()")]
// 0x364f28 — __ZThn4_N5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED1Ev
// was: non-virtual thunk toboost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()
pub fn stub_0x364f28() -> ! {
    todo!("0x364f28 __ZThn4_N5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED1Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()")]
// 0x364f30 — __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEED1Ev
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()
pub fn stub_0x364f30() -> ! {
    todo!("0x364f30 __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEED1Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()")]
// 0x364f38 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEED1Ev
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()
pub fn stub_0x364f38() -> ! {
    todo!("0x364f38 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone(void)const")]
// 0x364f48 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE5cloneEv
// type: char *__fastcall(int)
pub fn stub_0x364f48() -> ! {
    todo!("0x364f48 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE5cloneEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::rethrow(void)const")]
// 0x365008 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE7rethrowEv
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::rethrow(void)const
pub fn stub_0x365008() -> ! {
    todo!("0x365008 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE7rethrowEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()")]
// 0x365018 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEED0Ev
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()
pub fn stub_0x365018() -> ! {
    todo!("0x365018 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEED0Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()")]
// 0x365038 — __ZThn4_N5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED0Ev
// was: non-virtual thunk toboost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()
pub fn stub_0x365038() -> ! {
    todo!("0x365038 __ZThn4_N5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_tag)")]
// 0x365050 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEEC1ERKS6_NS6_9clone_tagE
pub fn stub_0x365050() -> ! {
    todo!("0x365050 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_impl(boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast> const&)")]
// 0x365188 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEEC1ERKS5_
pub fn stub_0x365188() -> ! {
    todo!("0x365188 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEEC1ERKS5_")
}

#[doc(alias = "boost::function1<void,double>::clear(void)")]
// 0x365520 — __ZN5boost9function1IvdE5clearEv
pub fn stub_0x365520() -> ! {
    todo!("0x365520 __ZN5boost9function1IvdE5clearEv")
}

#[doc(alias = "rbx::implementation::typed_holder<double>::singleton(void)")]
// 0x365550 — __ZN3rbx14implementation12typed_holderIdE9singletonEv
pub fn stub_0x365550() -> ! {
    todo!("0x365550 __ZN3rbx14implementation12typed_holderIdE9singletonEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(double)>::connect<boost::function<void ()(double)>>(boost::function<void ()(double)> const&)")]
// 0x366090 — __ZN3rbx7signals6signalIFvdEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x366090() -> ! {
    todo!("0x366090 __ZN3rbx7signals6signalIFvdEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::insert(rbx::signals::signal<void ()(double)>::slot *)")]
// 0x366184 — __ZN3rbx7signals6signalIFvdEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x366184() -> ! {
    todo!("0x366184 __ZN3rbx7signals6signalIFvdEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(double)>::slot>::operator=(rbx::signals::signal<void ()(double)>::slot*)")]
// 0x366390 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvdEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot>::operator=(rbx::signals::signal<void ()(double)>::slot*)
pub fn stub_0x366390() -> ! {
    todo!("0x366390 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvdEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::callable<rbx::signals::signal<void ()(double)>*>(boost::function<void ()(double)> const&,rbx::signals::signal<void ()(double)>*)")]
// 0x3663b4 — __ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
pub fn stub_0x3663b4() -> ! {
    todo!("0x3663b4 __ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::callable_slot<boost::function<void ()(double)>>::~callable_slot()")]
// 0x3664b0 — __ZN3rbx7signals6signalIFvdEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_0x3664b0() -> ! {
    todo!("0x3664b0 __ZN3rbx7signals6signalIFvdEE13callable_slotIN5boost8functionIS2_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::callable_slot<boost::function<void ()(double)>>::~callable_slot()")]
// 0x3665c0 — __ZN3rbx7signals6signalIFvdEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_0x3665c0() -> ! {
    todo!("0x3665c0 __ZN3rbx7signals6signalIFvdEE13callable_slotIN5boost8functionIS2_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::disconnect(void)")]
// 0x3666f0 — __ZN3rbx7signals6signalIFvdEE4slot10disconnectEv
pub fn stub_0x3666f0() -> ! {
    todo!("0x3666f0 __ZN3rbx7signals6signalIFvdEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::connected(void)const")]
// 0x366800 — __ZNK3rbx7signals6signalIFvdEE4slot9connectedEv
pub fn stub_0x366800() -> ! {
    todo!("0x366800 __ZNK3rbx7signals6signalIFvdEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::call(double)")]
// 0x36680c — __ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_E4callEd
pub fn stub_0x36680c() -> ! {
    todo!("0x36680c __ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_E4callEd")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::call(double)")]
// 0x366814 — __ZThn4_N3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_E4callEd
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::call(double)
pub fn stub_0x366814() -> ! {
    todo!("0x366814 __ZThn4_N3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_E4callEd")
}

#[doc(alias = "boost::function1<void,double>::operator()(double)const")]
// 0x36681c — __ZNK5boost9function1IvdEclEd
// type: void __fastcall(_DWORD *, int, int)
pub fn stub_0x36681c() -> ! {
    todo!("0x36681c __ZNK5boost9function1IvdEclEd")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::remove(rbx::signals::signal<void ()(double)>::slot *)")]
// 0x3668e8 — __ZN3rbx7signals6signalIFvdEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x3668e8() -> ! {
    todo!("0x3668e8 __ZN3rbx7signals6signalIFvdEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::safe_static_init_mutex(void)")]
// 0x3669d8 — __ZN3rbx7signals6signalIFvdEE4slot22safe_static_init_mutexEv
pub fn stub_0x3669d8() -> ! {
    todo!("0x3669d8 __ZN3rbx7signals6signalIFvdEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::safe_static_do_get_mutex(void)")]
// 0x3669dc — __ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x3669dc() -> ! {
    todo!("0x3669dc __ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::~callable()")]
// 0x366ad0 — __ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev
pub fn stub_0x366ad0() -> ! {
    todo!("0x366ad0 __ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::~callable()")]
// 0x366be0 — __ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev
pub fn stub_0x366be0() -> ! {
    todo!("0x366be0 __ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::~slot()")]
// 0x366d10 — __ZN3rbx7signals6signalIFvdEE4slotD1Ev
pub fn stub_0x366d10() -> ! {
    todo!("0x366d10 __ZN3rbx7signals6signalIFvdEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::~slot()")]
// 0x366d3c — __ZN3rbx7signals6signalIFvdEE4slotD0Ev
pub fn stub_0x366d3c() -> ! {
    todo!("0x366d3c __ZN3rbx7signals6signalIFvdEE4slotD0Ev")
}

#[doc(alias = "boost::function1<void,double>::assign_to_own(boost::function1<void,double> const&)")]
// 0x366e10 — __ZN5boost9function1IvdE13assign_to_ownERKS1_
pub fn stub_0x366e10() -> ! {
    todo!("0x366e10 __ZN5boost9function1IvdE13assign_to_ownERKS1_")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::disconnectAll(void)")]
// 0x3673ac — __ZN3rbx7signals6signalIFvddEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x3673ac() -> ! {
    todo!("0x3673ac __ZN3rbx7signals6signalIFvddEE13disconnectAllEv")
}

#[doc(alias = "boost::function2<void,double,double>::clear(void)")]
// 0x3677a8 — __ZN5boost9function2IvddE5clearEv
pub fn stub_0x3677a8() -> ! {
    todo!("0x3677a8 __ZN5boost9function2IvddE5clearEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(double,double)>::connect<boost::function<void ()(double,double)>>(boost::function<void ()(double,double)> const&)")]
// 0x367ef4 — __ZN3rbx7signals6signalIFvddEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x367ef4() -> ! {
    todo!("0x367ef4 __ZN3rbx7signals6signalIFvddEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::insert(rbx::signals::signal<void ()(double,double)>::slot *)")]
// 0x367fe8 — __ZN3rbx7signals6signalIFvddEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x367fe8() -> ! {
    todo!("0x367fe8 __ZN3rbx7signals6signalIFvddEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(double,double)>::slot>::operator=(rbx::signals::signal<void ()(double,double)>::slot*)")]
// 0x3681f4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvddEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot>::operator=(rbx::signals::signal<void ()(double,double)>::slot*)
pub fn stub_0x3681f4() -> ! {
    todo!("0x3681f4 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvddEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double,double)>::slot,boost::function<void ()(double,double)>,2,void ()(double,double)>::callable<rbx::signals::signal<void ()(double,double)>*>(boost::function<void ()(double,double)> const&,rbx::signals::signal<void ()(double,double)>*)")]
// 0x368218 — __ZN3rbx8callableINS_7signals6signalIFvddEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_
pub fn stub_0x368218() -> ! {
    todo!("0x368218 __ZN3rbx8callableINS_7signals6signalIFvddEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::callable_slot<boost::function<void ()(double,double)>>::~callable_slot()")]
// 0x368314 — __ZN3rbx7signals6signalIFvddEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x368314() -> ! {
    todo!("0x368314 __ZN3rbx7signals6signalIFvddEE13callable_slotIN5boost8functionIS2_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::callable_slot<boost::function<void ()(double,double)>>::~callable_slot()")]
// 0x368424 — __ZN3rbx7signals6signalIFvddEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x368424() -> ! {
    todo!("0x368424 __ZN3rbx7signals6signalIFvddEE13callable_slotIN5boost8functionIS2_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::disconnect(void)")]
// 0x368554 — __ZN3rbx7signals6signalIFvddEE4slot10disconnectEv
pub fn stub_0x368554() -> ! {
    todo!("0x368554 __ZN3rbx7signals6signalIFvddEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::connected(void)const")]
// 0x368664 — __ZNK3rbx7signals6signalIFvddEE4slot9connectedEv
pub fn stub_0x368664() -> ! {
    todo!("0x368664 __ZNK3rbx7signals6signalIFvddEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double,double)>::slot,boost::function<void ()(double,double)>,2,void ()(double,double)>::call(double,double)")]
// 0x368670 — __ZN3rbx8callableINS_7signals6signalIFvddEE4slotEN5boost8functionIS3_EELi2ES3_E4callEdd
pub fn stub_0x368670() -> ! {
    todo!("0x368670 __ZN3rbx8callableINS_7signals6signalIFvddEE4slotEN5boost8functionIS3_EELi2ES3_E4callEdd")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(double,double)>::slot,boost::function<void ()(double,double)>,2,void ()(double,double)>::call(double,double)")]
// 0x368688 — __ZThn4_N3rbx8callableINS_7signals6signalIFvddEE4slotEN5boost8functionIS3_EELi2ES3_E4callEdd
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(double,double)>::slot,boost::function<void ()(double,double)>,2,void ()(double,double)>::call(double,double)
pub fn stub_0x368688() -> ! {
    todo!("0x368688 __ZThn4_N3rbx8callableINS_7signals6signalIFvddEE4slotEN5boost8functionIS3_EELi2ES3_E4callEdd")
}

#[doc(alias = "boost::function2<void,double,double>::operator()(double,double)const")]
// 0x3686a0 — __ZNK5boost9function2IvddEclEdd
pub fn stub_0x3686a0() -> ! {
    todo!("0x3686a0 __ZNK5boost9function2IvddEclEdd")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::remove(rbx::signals::signal<void ()(double,double)>::slot *)")]
// 0x368778 — __ZN3rbx7signals6signalIFvddEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x368778() -> ! {
    todo!("0x368778 __ZN3rbx7signals6signalIFvddEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::safe_static_init_mutex(void)")]
// 0x368868 — __ZN3rbx7signals6signalIFvddEE4slot22safe_static_init_mutexEv
pub fn stub_0x368868() -> ! {
    todo!("0x368868 __ZN3rbx7signals6signalIFvddEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::safe_static_do_get_mutex(void)")]
// 0x36886c — __ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x36886c() -> ! {
    todo!("0x36886c __ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double,double)>::slot,boost::function<void ()(double,double)>,2,void ()(double,double)>::~callable()")]
// 0x36895c — __ZN3rbx8callableINS_7signals6signalIFvddEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev
pub fn stub_0x36895c() -> ! {
    todo!("0x36895c __ZN3rbx8callableINS_7signals6signalIFvddEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double,double)>::slot,boost::function<void ()(double,double)>,2,void ()(double,double)>::~callable()")]
// 0x368a6c — __ZN3rbx8callableINS_7signals6signalIFvddEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev
pub fn stub_0x368a6c() -> ! {
    todo!("0x368a6c __ZN3rbx8callableINS_7signals6signalIFvddEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::~slot()")]
// 0x368b9c — __ZN3rbx7signals6signalIFvddEE4slotD1Ev
pub fn stub_0x368b9c() -> ! {
    todo!("0x368b9c __ZN3rbx7signals6signalIFvddEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::~slot()")]
// 0x368bc8 — __ZN3rbx7signals6signalIFvddEE4slotD0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x368bc8() -> ! {
    todo!("0x368bc8 __ZN3rbx7signals6signalIFvddEE4slotD0Ev")
}

#[doc(alias = "boost::function2<void,double,double>::assign_to_own(boost::function2<void,double,double> const&)")]
// 0x368c9c — __ZN5boost9function2IvddE13assign_to_ownERKS1_
pub fn stub_0x368c9c() -> ! {
    todo!("0x368c9c __ZN5boost9function2IvddE13assign_to_ownERKS1_")
}

#[doc(alias = "RBX::HeartbeatTask::HeartbeatTask(rbx_core::SharedPtr<RBX::RunService>)")]
// 0x3690cc — __ZN3RBX13HeartbeatTaskC2EN5boost10shared_ptrINS_10RunServiceEEE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
// was: RBX::HeartbeatTask::HeartbeatTask(boost::shared_ptr<RBX::RunService>)
pub fn stub_0x3690cc() -> ! {
    todo!("0x3690cc __ZN3RBX13HeartbeatTaskC2EN5boost10shared_ptrINS_10RunServiceEEE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService>(rbx_core::WeakPtr<RBX::RunService> const&,boost::detail::sp_nothrow_tag)")]
// 0x369698 — __ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::RunService>::shared_ptr<RBX::RunService>(boost::weak_ptr<RBX::RunService> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0x369698() -> ! {
    todo!("0x369698 __ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "boost::detail::weak_count::weak_count(boost::detail::shared_count const&)")]
// 0x369718 — __ZN5boost6detail10weak_countC1ERKNS0_12shared_countE
// type: _DWORD __fastcall(boost::detail::weak_count *__hidden this, const boost::detail::shared_count *)
pub fn stub_0x369718() -> ! {
    todo!("0x369718 __ZN5boost6detail10weak_countC1ERKNS0_12shared_countE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::disconnectAll(void)")]
// 0x369c54 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
pub fn stub_0x369c54() -> ! {
    todo!("0x369c54 __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13disconnectAllEv")
}
