//! rendering shard 268 — 100 stubs EA-sorted asc global gap filler after 0x382df4 not yet in rendering (Ogre|G3D|Render 14876/14876 complete, 29170->29270 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 29170 before -> 29270 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x38922c — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEED1Ev
pub fn stub_38922c() -> ! {
    todo!("0x38922c boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::~sp_counted_impl_p()")
}

// 0x389230 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEED0Ev
pub fn stub_389230() -> ! {
    todo!("0x389230 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::~sp_counted_impl_p()")
}

// 0x389234 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE7disposeEv
pub fn stub_389234() -> ! {
    todo!("0x389234 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::dispose(void)")
}

// 0x389244 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE11get_deleterERKSt9type_info
pub fn stub_389244() -> ! {
    todo!("0x389244 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::get_deleter(std::type_info const&)")
}

// 0x389248 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE19get_untyped_deleterEv
pub fn stub_389248() -> ! {
    todo!("0x389248 boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>::get_untyped_deleter(void)")
}

// 0x38924c — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_
pub fn stub_38924c() -> ! {
    todo!("0x38924c boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>)")
}

// 0x389364 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_
pub fn stub_389364() -> ! {
    todo!("0x389364 boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>)")
}

// 0x389480 — __ZN5boost10shared_ptrIN3RBX5mutexEEC2IS2_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::mutex>::shared_ptr<RBX::mutex>(RBX::mutex *)")]
// was: __ZN5boost10shared_ptrIN3RBX5mutexEEC2IS2_EEPT_
pub fn stub_389480() -> ! {
    todo!("0x389480 boost::shared_ptr<RBX::mutex>::shared_ptr<RBX::mutex>(RBX::mutex *)")
}

// 0x389554 — __ZN5boost6detail12shared_countC2IN3RBX5mutexEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::mutex>(RBX::mutex *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX5mutexEEEPT_
pub fn stub_389554() -> ! {
    todo!("0x389554 boost::detail::shared_count::shared_count<RBX::mutex>(RBX::mutex *)")
}

// 0x389660 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEED1Ev
pub fn stub_389660() -> ! {
    todo!("0x389660 boost::detail::sp_counted_impl_p<RBX::mutex>::~sp_counted_impl_p()")
}

// 0x389664 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEED0Ev
pub fn stub_389664() -> ! {
    todo!("0x389664 boost::detail::sp_counted_impl_p<RBX::mutex>::~sp_counted_impl_p()")
}

// 0x389668 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE7disposeEv
pub fn stub_389668() -> ! {
    todo!("0x389668 boost::detail::sp_counted_impl_p<RBX::mutex>::dispose(void)")
}

// 0x38970c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE11get_deleterERKSt9type_info
pub fn stub_38970c() -> ! {
    todo!("0x38970c boost::detail::sp_counted_impl_p<RBX::mutex>::get_deleter(std::type_info const&)")
}

// 0x389710 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE19get_untyped_deleterEv
pub fn stub_389710() -> ! {
    todo!("0x389710 boost::detail::sp_counted_impl_p<RBX::mutex>::get_untyped_deleter(void)")
}

// 0x389714 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE15_M_erase_at_endEPS4_
// type: boost::detail::sp_counted_base *__fastcall(boost::detail::sp_counted_base *result, int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_erase_at_end(boost::shared_ptr<RBX::mutex>*)")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE15_M_erase_at_endEPS4_
pub fn stub_389714() -> ! {
    todo!("0x389714 std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_erase_at_end(boost::shared_ptr<RBX::mutex>*)")
}

// 0x389744 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
// type: void __fastcall(int *, struct _Unwind_Exception *, int, const shared_count *)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::mutex>*,std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>>,unsigned long,boost::shared_ptr<RBX::mutex> const&)")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
pub fn stub_389744() -> ! {
    todo!("0x389744 std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::mutex>*,std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>>,unsigned long,boost::shared_ptr<RBX::mutex> const&)")
}

// 0x389d44 — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE11_M_allocateEm
pub fn stub_389d44() -> ! {
    todo!("0x389d44 std::_Vector_base<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::_M_allocate(unsigned long)")
}

// 0x389d5c — __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX5mutexEEEmS4_EvT_T0_RKT1_St12__false_type
// type: void __fastcall(int, int, shared_count *, int, int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_aux<boost::shared_ptr<RBX::mutex> *,unsigned long,boost::shared_ptr<RBX::mutex>>(boost::shared_ptr<RBX::mutex> *,unsigned long,boost::shared_ptr<RBX::mutex> const&,std::__false_type)")]
// was: __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX5mutexEEEmS4_EvT_T0_RKT1_St12__false_type
pub fn stub_389d5c() -> ! {
    todo!("0x389d5c void std::__uninitialized_fill_n_aux<boost::shared_ptr<RBX::mutex> *,unsigned long,boost::shared_ptr<RBX::mutex>>(boost::shared_ptr<RBX::mutex> *,unsigned long,boost::shared_ptr<RBX::mutex> const&,std::__false_type)")
}

// 0x389e84 — __ZN5boost10shared_ptrIN3RBX5mutexEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::mutex>::operator=(boost::shared_ptr<RBX::mutex> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX5mutexEEaSERKS3_
pub fn stub_389e84() -> ! {
    todo!("0x389e84 boost::shared_ptr<RBX::mutex>::operator=(boost::shared_ptr<RBX::mutex> const&)")
}

// 0x389ebc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5mutexEEES8_EET0_T_SA_S9_
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::mutex> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *>(boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5mutexEEES8_EET0_T_SA_S9_
pub fn stub_389ebc() -> ! {
    todo!("0x389ebc boost::shared_ptr<RBX::mutex> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *>(boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *,boost::shared_ptr<RBX::mutex> *)")
}

// 0x389f0c — __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE15_M_erase_at_endEPS3_
// type: boost::detail::sp_counted_base *__fastcall(boost::detail::sp_counted_base *result, int)
#[doc(alias = "std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_erase_at_end(boost::shared_ptr<boost::thread>*)")]
// was: __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE15_M_erase_at_endEPS3_
pub fn stub_389f0c() -> ! {
    todo!("0x389f0c std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_erase_at_end(boost::shared_ptr<boost::thread>*)")
}

// 0x389f3c — __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// type: void __fastcall(int *, struct _Unwind_Exception *, int, const shared_count *)
#[doc(alias = "std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread>*,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,unsigned long,boost::shared_ptr<boost::thread> const&)")]
// was: __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
pub fn stub_389f3c() -> ! {
    todo!("0x389f3c std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread>*,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,unsigned long,boost::shared_ptr<boost::thread> const&)")
}

// 0x38a53c — __ZNSt12_Vector_baseIN5boost10shared_ptrINS0_6threadEEESaIS3_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN5boost10shared_ptrINS0_6threadEEESaIS3_EE11_M_allocateEm
pub fn stub_38a53c() -> ! {
    todo!("0x38a53c std::_Vector_base<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::_M_allocate(unsigned long)")
}

// 0x38a554 — __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrINS0_6threadEEEmS3_EvT_T0_RKT1_St12__false_type
// type: void __fastcall(int, int, shared_count *, int, int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_aux<boost::shared_ptr<boost::thread> *,unsigned long,boost::shared_ptr<boost::thread>>(boost::shared_ptr<boost::thread> *,unsigned long,boost::shared_ptr<boost::thread> const&,std::__false_type)")]
// was: __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrINS0_6threadEEEmS3_EvT_T0_RKT1_St12__false_type
pub fn stub_38a554() -> ! {
    todo!("0x38a554 void std::__uninitialized_fill_n_aux<boost::shared_ptr<boost::thread> *,unsigned long,boost::shared_ptr<boost::thread>>(boost::shared_ptr<boost::thread> *,unsigned long,boost::shared_ptr<boost::thread> const&,std::__false_type)")
}

// 0x38a67c — __ZN5boost10shared_ptrINS_6threadEEaSERKS2_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<boost::thread>::operator=(boost::shared_ptr<boost::thread> const&)")]
// was: __ZN5boost10shared_ptrINS_6threadEEaSERKS2_
pub fn stub_38a67c() -> ! {
    todo!("0x38a67c boost::shared_ptr<boost::thread>::operator=(boost::shared_ptr<boost::thread> const&)")
}

// 0x38a6b4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrINS3_6threadEEES7_EET0_T_S9_S8_
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::shared_ptr<boost::thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *>(boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrINS3_6threadEEES7_EET0_T_S9_S8_
pub fn stub_38a6b4() -> ! {
    todo!("0x38a6b4 boost::shared_ptr<boost::thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *>(boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *,boost::shared_ptr<boost::thread> *)")
}

// 0x38a704 — __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::~vector()")]
// was: __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EED2Ev
pub fn stub_38a704() -> ! {
    todo!("0x38a704 std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::~vector()")
}

// 0x38a7d0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::~vector()")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EED2Ev
pub fn stub_38a7d0() -> ! {
    todo!("0x38a7d0 std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::~vector()")
}

// 0x38a89c — __ZN5boost10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEC2IS3_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::BaseThreadPool::PoolData>::shared_ptr<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")]
// was: __ZN5boost10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEC2IS3_EEPT_
pub fn stub_38a89c() -> ! {
    todo!("0x38a89c boost::shared_ptr<RBX::BaseThreadPool::PoolData>::shared_ptr<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")
}

// 0x38a970 — __ZN5boost6detail12shared_countC2IN3RBX14BaseThreadPool8PoolDataEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX14BaseThreadPool8PoolDataEEEPT_
pub fn stub_38a970() -> ! {
    todo!("0x38a970 boost::detail::shared_count::shared_count<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")
}

// 0x38aa68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEED1Ev
pub fn stub_38aa68() -> ! {
    todo!("0x38aa68 boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::~sp_counted_impl_p()")
}

// 0x38aa6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEED0Ev
pub fn stub_38aa6c() -> ! {
    todo!("0x38aa6c boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::~sp_counted_impl_p()")
}

// 0x38aa70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE7disposeEv
pub fn stub_38aa70() -> ! {
    todo!("0x38aa70 boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::dispose(void)")
}

// 0x38aa80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE11get_deleterERKSt9type_info
pub fn stub_38aa80() -> ! {
    todo!("0x38aa80 boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_deleter(std::type_info const&)")
}

// 0x38aa84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE19get_untyped_deleterEv
pub fn stub_38aa84() -> ! {
    todo!("0x38aa84 boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_untyped_deleter(void)")
}

// 0x38aa88 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE13assign_to_ownERKS5_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::mutex>> const&)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE13assign_to_ownERKS5_
pub fn stub_38aa88() -> ! {
    todo!("0x38aa88 boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::mutex>> const&)")
}

// 0x38aab8 — __ZN3RBX14BaseThreadPool8PoolDataD2Ev
// type: void __fastcall(RBX::BaseThreadPool::PoolData *__hidden this)
#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
// was: __ZN3RBX14BaseThreadPool8PoolDataD2Ev
pub fn stub_38aab8() -> ! {
    todo!("0x38aab8 RBX::BaseThreadPool::PoolData::~PoolData()")
}

// 0x38ab90 — __ZN3RBX14BaseThreadPool8PoolDataD1Ev
// type: void __fastcall(RBX::BaseThreadPool::PoolData *__hidden this)
#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
// was: __ZN3RBX14BaseThreadPool8PoolDataD1Ev
pub fn stub_38ab90() -> ! {
    todo!("0x38ab90 RBX::BaseThreadPool::PoolData::~PoolData()")
}

// 0x38ab94 — __ZN3RBX14BaseThreadPool8PoolDataD0Ev
// type: void __fastcall(RBX::BaseThreadPool::PoolData *__hidden this)
#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
// was: __ZN3RBX14BaseThreadPool8PoolDataD0Ev
pub fn stub_38ab94() -> ! {
    todo!("0x38ab94 RBX::BaseThreadPool::PoolData::~PoolData()")
}

// 0x38ac34 — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EED2Ev
// type: char **__fastcall(char **)
#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::~vector()")]
// was: __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EED2Ev
pub fn stub_38ac34() -> ! {
    todo!("0x38ac34 std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::~vector()")
}

// 0x38ad00 — __ZN3RBX14BaseThreadPool8PoolDataC2Ev
// type: RBX::BaseThreadPool::PoolData *__fastcall(RBX::BaseThreadPool::PoolData *this)
#[doc(alias = "RBX::BaseThreadPool::PoolData::PoolData(void)")]
// was: __ZN3RBX14BaseThreadPool8PoolDataC2Ev
pub fn stub_38ad00() -> ! {
    todo!("0x38ad00 RBX::BaseThreadPool::PoolData::PoolData(void)")
}

// 0x38add0 — __ZN3RBX10ThreadPool14ThreadPoolDataD1Ev
// type: void __fastcall(RBX::ThreadPool::ThreadPoolData *__hidden this)
#[doc(alias = "RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")]
// was: __ZN3RBX10ThreadPool14ThreadPoolDataD1Ev
pub fn stub_38add0() -> ! {
    todo!("0x38add0 RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")
}

// 0x38aec4 — __ZN3RBX10ThreadPool14ThreadPoolDataD0Ev
// type: void __fastcall(RBX::ThreadPool::ThreadPoolData *__hidden this)
#[doc(alias = "RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")]
// was: __ZN3RBX10ThreadPool14ThreadPoolDataD0Ev
pub fn stub_38aec4() -> ! {
    todo!("0x38aec4 RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")
}

// 0x38afc8 — __ZN3RBX10ThreadPool14ThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE
// type: int __fastcall(int)
#[doc(alias = "RBX::ThreadPool::ThreadPoolData::getNextTask(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> &)")]
// was: __ZN3RBX10ThreadPool14ThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE
pub fn stub_38afc8() -> ! {
    todo!("0x38afc8 RBX::ThreadPool::ThreadPoolData::getNextTask(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> &)")
}

// 0x38afd4 — __ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE14pop_if_presentERS8_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::safe_queue<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>::pop_if_present(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&)")]
// was: __ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE14pop_if_presentERS8_
pub fn stub_38afd4() -> ! {
    todo!("0x38afd4 rbx::safe_queue<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>::pop_if_present(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&)")
}

// 0x38b0b4 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9pop_frontEv
// type: int __fastcall(int)
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::pop_front(void)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9pop_frontEv
pub fn stub_38b0b4() -> ! {
    todo!("0x38b0b4 std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::pop_front(void)")
}

// 0x38b0ec — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::~deque()")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
pub fn stub_38b0ec() -> ! {
    todo!("0x38b0ec std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::~deque()")
}

// 0x38b1d4 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::~_Deque_base()")]
// was: __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
pub fn stub_38b1d4() -> ! {
    todo!("0x38b1d4 std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::~_Deque_base()")
}

// 0x38b200 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_
// type: void __fastcall(int, int *, int *, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>*>,std::_Deque_iterator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>*>)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_
pub fn stub_38b200() -> ! {
    todo!("0x38b200 std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>*>,std::_Deque_iterator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>*>)")
}

// 0x38b338 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_initialize_mapEm
// type: void __fastcall(int *, unsigned int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_initialize_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_initialize_mapEm
pub fn stub_38b338() -> ! {
    todo!("0x38b338 std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_initialize_map(unsigned long)")
}

// 0x38b490 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_create_nodesEPPS7_SB_
// type: void __fastcall(int, _DWORD *, unsigned int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_create_nodes(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>**,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>**)")]
// was: __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_create_nodesEPPS7_SB_
pub fn stub_38b490() -> ! {
    todo!("0x38b490 std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_create_nodes(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>**,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>**)")
}

// 0x38b584 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EEC2ERKS9_
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::deque(std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>> const&)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EEC2ERKS9_
pub fn stub_38b584() -> ! {
    todo!("0x38b584 std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::deque(std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>> const&)")
}

// 0x38b740 — __GLOBAL__I_a_146
#[doc(alias = "global constructor keyed to_a_146")]
// was: __GLOBAL__I_a_146
pub fn stub_38b740() -> ! {
    todo!("0x38b740 global constructor keyed to_a_146")
}

// 0x38b808 — __ZN3RBX15StringConverterINS_4UDimEE15convertToStringERKS1_
// type: void __fastcall(std::string *, int)
#[doc(alias = "RBX::StringConverter<RBX::UDim>::convertToString(RBX::UDim const&)")]
// was: __ZN3RBX15StringConverterINS_4UDimEE15convertToStringERKS1_
pub fn stub_38b808() -> ! {
    todo!("0x38b808 RBX::StringConverter<RBX::UDim>::convertToString(RBX::UDim const&)")
}

// 0x38b970 — __ZN3RBX15StringConverterINS_4UDimEE14convertToValueERKSsRS1_
// type: int __fastcall(std::string *, int)
#[doc(alias = "RBX::StringConverter<RBX::UDim>::convertToValue(std::string const&,RBX::UDim&)")]
// was: __ZN3RBX15StringConverterINS_4UDimEE14convertToValueERKSsRS1_
pub fn stub_38b970() -> ! {
    todo!("0x38b970 RBX::StringConverter<RBX::UDim>::convertToValue(std::string const&,RBX::UDim&)")
}

// 0x38ba5c — __ZN3RBX15StringConverterINS_5UDim2EE15convertToStringERKS1_
// type: void __fastcall(std::string *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::StringConverter<RBX::UDim2>::convertToString(RBX::UDim2 const&)")]
// was: __ZN3RBX15StringConverterINS_5UDim2EE15convertToStringERKS1_
pub fn stub_38ba5c() -> ! {
    todo!("0x38ba5c RBX::StringConverter<RBX::UDim2>::convertToString(RBX::UDim2 const&)")
}

// 0x38be8c — __ZN3RBX15StringConverterINS_5UDim2EE14convertToValueERKSsRS1_
// type: int __fastcall(std::string *, int)
#[doc(alias = "RBX::StringConverter<RBX::UDim2>::convertToValue(std::string const&,RBX::UDim2&)")]
// was: __ZN3RBX15StringConverterINS_5UDim2EE14convertToValueERKSsRS1_
pub fn stub_38be8c() -> ! {
    todo!("0x38be8c RBX::StringConverter<RBX::UDim2>::convertToValue(std::string const&,RBX::UDim2&)")
}

// 0x38c0e8 — __ZNK3RBX4UDimplERKS0_
// type: int __fastcall(int result, int, int)
#[doc(alias = "RBX::UDim::operator+(RBX::UDim const&)const")]
// was: __ZNK3RBX4UDimplERKS0_
pub fn stub_38c0e8() -> ! {
    todo!("0x38c0e8 RBX::UDim::operator+(RBX::UDim const&)const")
}

// 0x38c108 — __ZNK3RBX4UDimmiERKS0_
// type: int __fastcall(int result, int, int)
#[doc(alias = "RBX::UDim::operator-(RBX::UDim const&)const")]
// was: __ZNK3RBX4UDimmiERKS0_
pub fn stub_38c108() -> ! {
    todo!("0x38c108 RBX::UDim::operator-(RBX::UDim const&)const")
}

// 0x38c128 — __ZNK3RBX4UDimngEv
// type: int __fastcall(int result, int)
#[doc(alias = "RBX::UDim::operator-(void)const")]
// was: __ZNK3RBX4UDimngEv
pub fn stub_38c128() -> ! {
    todo!("0x38c128 RBX::UDim::operator-(void)const")
}

// 0x38c188 — __ZNK3RBX5UDim2mlEf
// type: _DWORD *__fastcall(_DWORD *result, int, __int32)
#[doc(alias = "RBX::UDim2::operator*(float)const")]
// was: __ZNK3RBX5UDim2mlEf
pub fn stub_38c188() -> ! {
    todo!("0x38c188 RBX::UDim2::operator*(float)const")
}

// 0x38c1e4 — __ZNK3RBX5UDim2plERKS0_
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "RBX::UDim2::operator+(RBX::UDim2 const&)const")]
// was: __ZNK3RBX5UDim2plERKS0_
pub fn stub_38c1e4() -> ! {
    todo!("0x38c1e4 RBX::UDim2::operator+(RBX::UDim2 const&)const")
}

// 0x38c224 — __ZNK3RBX5UDim2miERKS0_
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "RBX::UDim2::operator-(RBX::UDim2 const&)const")]
// was: __ZNK3RBX5UDim2miERKS0_
pub fn stub_38c224() -> ! {
    todo!("0x38c224 RBX::UDim2::operator-(RBX::UDim2 const&)const")
}

// 0x38d61c — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIlEEbRT_
// type: int __fastcall(unsigned __int8 **, int *)
#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long>(long &)")]
// was: __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIlEEbRT_
pub fn stub_38d61c() -> ! {
    todo!("0x38d61c bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long>(long &)")
}

// 0x38d67c — __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEmcEEbRT0_PKT1_S8_
// type: int __fastcall(int *, unsigned int, int)
#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long,char>(unsigned long &,char const*,char const*)")]
// was: __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEmcEEbRT0_PKT1_S8_
pub fn stub_38d67c() -> ! {
    todo!("0x38d67c bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long,char>(unsigned long &,char const*,char const*)")
}

// 0x38da14 — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedIjEEbRT_
// type: int __fastcall(unsigned __int8 **, _DWORD *)
#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned int>(unsigned int &)")]
// was: __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedIjEEbRT_
pub fn stub_38da14() -> ! {
    todo!("0x38da14 bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned int>(unsigned int &)")
}

// 0x38e9d0 — __ZThn36_N3RBX12AccoutrementD0Ev
// type: void __fastcall(RBX::Accoutrement *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn36_N3RBX12AccoutrementD0Ev
pub fn stub_38e9d0() -> ! {
    todo!("0x38e9d0 non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38e9d8 — __ZThn92_N3RBX12AccoutrementD0Ev
// type: void __fastcall(RBX::Accoutrement *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn92_N3RBX12AccoutrementD0Ev
pub fn stub_38e9d8() -> ! {
    todo!("0x38e9d8 non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38e9e0 — __ZThn128_N3RBX12AccoutrementD0Ev
// type: void __fastcall(RBX::Accoutrement *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn128_N3RBX12AccoutrementD0Ev
pub fn stub_38e9e0() -> ! {
    todo!("0x38e9e0 non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38e9e8 — __ZN3RBX12AccoutrementD2Ev
// type: void __fastcall(RBX::Accoutrement *this, int *, int)
#[doc(alias = "RBX::Accoutrement::~Accoutrement()")]
// was: __ZN3RBX12AccoutrementD2Ev
pub fn stub_38e9e8() -> ! {
    todo!("0x38e9e8 RBX::Accoutrement::~Accoutrement()")
}

// 0x38ef1c — __ZThn32_N3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn32_N3RBX12AccoutrementD1Ev
pub fn stub_38ef1c() -> ! {
    todo!("0x38ef1c non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38ef2c — __ZThn36_N3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn36_N3RBX12AccoutrementD1Ev
pub fn stub_38ef2c() -> ! {
    todo!("0x38ef2c non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38ef3c — __ZThn92_N3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn92_N3RBX12AccoutrementD1Ev
pub fn stub_38ef3c() -> ! {
    todo!("0x38ef3c non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38ef4c — __ZThn128_N3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn128_N3RBX12AccoutrementD1Ev
pub fn stub_38ef4c() -> ! {
    todo!("0x38ef4c non-virtual thunk toRBX::Accoutrement::~Accoutrement()")
}

// 0x38ef5c — __ZN3RBX12Accoutrement12onCameraNearEf
// type: unsigned int __fastcall(RBX::Accoutrement *this, float)
#[doc(alias = "RBX::Accoutrement::onCameraNear(float)")]
// was: __ZN3RBX12Accoutrement12onCameraNearEf
pub fn stub_38ef5c() -> ! {
    todo!("0x38ef5c RBX::Accoutrement::onCameraNear(float)")
}

// 0x38ef98 — __ZThn128_N3RBX12Accoutrement12onCameraNearEf
// type: unsigned int __fastcall(RBX::Accoutrement *this, float)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::onCameraNear(float)")]
// was: __ZThn128_N3RBX12Accoutrement12onCameraNearEf
pub fn stub_38ef98() -> ! {
    todo!("0x38ef98 non-virtual thunk toRBX::Accoutrement::onCameraNear(float)")
}

// 0x38f01c — __ZN3RBX12Accoutrement7dropAllEPNS_13ModelInstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::ModelInstance *, RBX::Accoutrement *)
#[doc(alias = "RBX::Accoutrement::dropAll(RBX::ModelInstance *)")]
// was: __ZN3RBX12Accoutrement7dropAllEPNS_13ModelInstanceE
pub fn stub_38f01c() -> ! {
    todo!("0x38f01c RBX::Accoutrement::dropAll(RBX::ModelInstance *)")
}

// 0x38f024 — __ZN3RBX12Accoutrement13dropAllOthersEPNS_13ModelInstanceEPS0_
// type: RBX::Instance *__fastcall(RBX::Accoutrement *this, RBX::ModelInstance *, RBX::Accoutrement *)
#[doc(alias = "RBX::Accoutrement::dropAllOthers(RBX::ModelInstance *,RBX::Accoutrement*)")]
// was: __ZN3RBX12Accoutrement13dropAllOthersEPNS_13ModelInstanceEPS0_
pub fn stub_38f024() -> ! {
    todo!("0x38f024 RBX::Accoutrement::dropAllOthers(RBX::ModelInstance *,RBX::Accoutrement*)")
}

// 0x38f054 — __ZNK3RBX12Accoutrement14getHandleConstEv
// type: char *__fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::getHandleConst(void)const")]
// was: __ZNK3RBX12Accoutrement14getHandleConstEv
pub fn stub_38f054() -> ! {
    todo!("0x38f054 RBX::Accoutrement::getHandleConst(void)const")
}

// 0x38f1c4 — __ZN3RBX12Accoutrement11getLocationEv
// type: int __fastcall(RBX::Accoutrement *this, RBX::Accoutrement *)
#[doc(alias = "RBX::Accoutrement::getLocation(void)")]
// was: __ZN3RBX12Accoutrement11getLocationEv
pub fn stub_38f1c4() -> ! {
    todo!("0x38f1c4 RBX::Accoutrement::getLocation(void)")
}

// 0x38f1f8 — __ZTv0_n12_N3RBX12Accoutrement11getLocationEv
// type: int __fastcall(RBX::Accoutrement *this, _DWORD *)
#[doc(alias = "virtual thunk toRBX::Accoutrement::getLocation(void)")]
// was: __ZTv0_n12_N3RBX12Accoutrement11getLocationEv
pub fn stub_38f1f8() -> ! {
    todo!("0x38f1f8 virtual thunk toRBX::Accoutrement::getLocation(void)")
}

// 0x38f20c — __ZN3RBX12Accoutrement17connectTouchEventEv
// type: void __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::connectTouchEvent(void)")]
// was: __ZN3RBX12Accoutrement17connectTouchEventEv
pub fn stub_38f20c() -> ! {
    todo!("0x38f20c RBX::Accoutrement::connectTouchEvent(void)")
}

// 0x38f3ec — __ZN3RBX12Accoutrement21onEvent_HandleTouchedEN5boost10shared_ptrINS_8InstanceEEE
// type: RBX::Accoutrement *__fastcall(RBX::Network::Players *, RBX::Accoutrement **, bool)
#[doc(alias = "RBX::Accoutrement::onEvent_HandleTouched(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX12Accoutrement21onEvent_HandleTouchedEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_38f3ec() -> ! {
    todo!("0x38f3ec RBX::Accoutrement::onEvent_HandleTouched(boost::shared_ptr<RBX::Instance>)")
}

// 0x38f47c — __ZN3RBX12Accoutrement19rebuildBackendStateEv
// type: int __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::rebuildBackendState(void)")]
// was: __ZN3RBX12Accoutrement19rebuildBackendStateEv
pub fn stub_38f47c() -> ! {
    todo!("0x38f47c RBX::Accoutrement::rebuildBackendState(void)")
}

// 0x38f4f4 — __ZN3RBX12Accoutrement19computeDesiredStateEv
// type: int __fastcall(RBX::Instance **this, int, bool)
#[doc(alias = "RBX::Accoutrement::computeDesiredState(void)")]
// was: __ZN3RBX12Accoutrement19computeDesiredStateEv
pub fn stub_38f4f4() -> ! {
    todo!("0x38f4f4 RBX::Accoutrement::computeDesiredState(void)")
}

// 0x38f578 — __ZN3RBX12Accoutrement15setDesiredStateENS0_17AccoutrementStateEPKNS_15ServiceProviderE
// type: int __fastcall(RBX::Accoutrement *this, int, RBX::Network::Players *)
#[doc(alias = "RBX::Accoutrement::setDesiredState(RBX::Accoutrement::AccoutrementState,RBX::ServiceProvider const*)")]
// was: __ZN3RBX12Accoutrement15setDesiredStateENS0_17AccoutrementStateEPKNS_15ServiceProviderE
pub fn stub_38f578() -> ! {
    todo!("0x38f578 RBX::Accoutrement::setDesiredState(RBX::Accoutrement::AccoutrementState,RBX::ServiceProvider const*)")
}

// 0x38f6f0 — __ZN3RBX12Accoutrement19computeDesiredStateEPNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::computeDesiredState(RBX::Instance *)")]
// was: __ZN3RBX12Accoutrement19computeDesiredStateEPNS_8InstanceE
pub fn stub_38f6f0() -> ! {
    todo!("0x38f6f0 RBX::Accoutrement::computeDesiredState(RBX::Instance *)")
}

// 0x38f714 — __ZN3RBX12Accoutrement13upTo_EquippedEv
// type: void __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::upTo_Equipped(void)")]
// was: __ZN3RBX12Accoutrement13upTo_EquippedEv
pub fn stub_38f714() -> ! {
    todo!("0x38f714 RBX::Accoutrement::upTo_Equipped(void)")
}

// 0x38f92c — __ZN3RBX12Accoutrement16upTo_InCharacterEv
// type: void __fastcall(RBX::Humanoid **this)
#[doc(alias = "RBX::Accoutrement::upTo_InCharacter(void)")]
// was: __ZN3RBX12Accoutrement16upTo_InCharacterEv
pub fn stub_38f92c() -> ! {
    todo!("0x38f92c RBX::Accoutrement::upTo_InCharacter(void)")
}

// 0x38fb1c — __ZN3RBX12Accoutrement16upTo_InWorkspaceEv
// type: int __fastcall(RBX::Accoutrement *this, const RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::upTo_InWorkspace(void)")]
// was: __ZN3RBX12Accoutrement16upTo_InWorkspaceEv
pub fn stub_38fb1c() -> ! {
    todo!("0x38fb1c RBX::Accoutrement::upTo_InWorkspace(void)")
}

// 0x38fbcc — __ZN3RBX12Accoutrement17downFrom_EquippedEv
// type: void __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::downFrom_Equipped(void)")]
// was: __ZN3RBX12Accoutrement17downFrom_EquippedEv
pub fn stub_38fbcc() -> ! {
    todo!("0x38fbcc RBX::Accoutrement::downFrom_Equipped(void)")
}

// 0x38fd24 — __ZN3RBX12Accoutrement18downFrom_HasHandleEv
// type: void __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::downFrom_HasHandle(void)")]
// was: __ZN3RBX12Accoutrement18downFrom_HasHandleEv
pub fn stub_38fd24() -> ! {
    todo!("0x38fd24 RBX::Accoutrement::downFrom_HasHandle(void)")
}

// 0x38fd60 — __ZN3RBX12Accoutrement20onEvent_AddedBackendEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::onEvent_AddedBackend(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX12Accoutrement20onEvent_AddedBackendEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_38fd60() -> ! {
    todo!("0x38fd60 RBX::Accoutrement::onEvent_AddedBackend(boost::shared_ptr<RBX::Instance>)")
}

// 0x38fe18 — __ZN3RBX12Accoutrement22onEvent_RemovedBackendEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int)
#[doc(alias = "RBX::Accoutrement::onEvent_RemovedBackend(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX12Accoutrement22onEvent_RemovedBackendEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_38fe18() -> ! {
    todo!("0x38fe18 RBX::Accoutrement::onEvent_RemovedBackend(boost::shared_ptr<RBX::Instance>)")
}

// 0x38ff34 — __ZN3RBX12Accoutrement12onChildAddedEPNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::onChildAdded(RBX::Instance *)")]
// was: __ZN3RBX12Accoutrement12onChildAddedEPNS_8InstanceE
pub fn stub_38ff34() -> ! {
    todo!("0x38ff34 RBX::Accoutrement::onChildAdded(RBX::Instance *)")
}

// 0x38ff5c — __ZN3RBX12Accoutrement14onChildRemovedEPNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::onChildRemoved(RBX::Instance *)")]
// was: __ZN3RBX12Accoutrement14onChildRemovedEPNS_8InstanceE
pub fn stub_38ff5c() -> ! {
    todo!("0x38ff5c RBX::Accoutrement::onChildRemoved(RBX::Instance *)")
}

// 0x38ff84 — __ZN3RBX12Accoutrement17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "RBX::Accoutrement::onAncestorChanged(RBX::AncestorChanged const&)")]
// was: __ZN3RBX12Accoutrement17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_38ff84() -> ! {
    todo!("0x38ff84 RBX::Accoutrement::onAncestorChanged(RBX::AncestorChanged const&)")
}

// 0x38fff0 — __ZN3RBX3HatC1Ev
// type: RBX::Accoutrement *__fastcall(RBX::Hat *this)
#[doc(alias = "RBX::Hat::Hat(void)")]
// was: __ZN3RBX3HatC1Ev
pub fn stub_38fff0() -> ! {
    todo!("0x38fff0 RBX::Hat::Hat(void)")
}

// 0x3901bc — __ZNK3RBX12Accoutrement18getAttachmentPointEv
// type: char *__fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::getAttachmentPoint(void)const")]
// was: __ZNK3RBX12Accoutrement18getAttachmentPointEv
pub fn stub_3901bc() -> ! {
    todo!("0x3901bc RBX::Accoutrement::getAttachmentPoint(void)const")
}

// 0x390208 — __ZNK3RBX12Accoutrement27getBackendAccoutrementStateEv
// type: int __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::getBackendAccoutrementState(void)const")]
// was: __ZNK3RBX12Accoutrement27getBackendAccoutrementStateEv
pub fn stub_390208() -> ! {
    todo!("0x390208 RBX::Accoutrement::getBackendAccoutrementState(void)const")
}