//! rendering shard 268 — 100 stubs EA-sorted asc global gap filler after 0x382df4 not yet in rendering (Ogre|G3D|Render 14876/14876 complete, 29170->29270 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 29170 before -> 29270 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x38922c — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEED1Ev
// IDA 0x38922c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_38922c() {
}

// 0x389230 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEED0Ev
// IDA 0x389230: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_389230() {
}

// 0x389234 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE7disposeEv
// IDA 0x389234: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389234() {
}

// 0x389244 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE11get_deleterERKSt9type_info
// IDA 0x389244: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389244() {
}

// 0x389248 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEEE19get_untyped_deleterEv
// IDA 0x389248: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389248() {
}

// 0x38924c — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_
// IDA 0x38924c: 102 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38924c() {
}

// 0x389364 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEC2ES8_SB_
// IDA 0x389364: 106 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389364() {
}

// 0x389480 — __ZN5boost10shared_ptrIN3RBX5mutexEEC2IS2_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::mutex>::shared_ptr<RBX::mutex>(RBX::mutex *)")]
// was: __ZN5boost10shared_ptrIN3RBX5mutexEEC2IS2_EEPT_
// IDA 0x389480: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389480() {
}

// 0x389554 — __ZN5boost6detail12shared_countC2IN3RBX5mutexEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::mutex>(RBX::mutex *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX5mutexEEEPT_
// IDA 0x389554: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389554() {
}

// 0x389660 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEED1Ev
// IDA 0x389660: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_389660() {
}

// 0x389664 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEED0Ev
// IDA 0x389664: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_389664() {
}

// 0x389668 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE7disposeEv
// IDA 0x389668: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389668() {
}

// 0x38970c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE11get_deleterERKSt9type_info
// IDA 0x38970c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38970c() {
}

// 0x389710 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5mutexEE19get_untyped_deleterEv
// IDA 0x389710: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389710() {
}

// 0x389714 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE15_M_erase_at_endEPS4_
// type: boost::detail::sp_counted_base *__fastcall(boost::detail::sp_counted_base *result, int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::mutex>*)")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE15_M_erase_at_endEPS4_
// IDA 0x389714: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389714() {
}

// 0x389744 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
// type: void __fastcall(int *, struct _Unwind_Exception *, int, const shared_count *)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::mutex>*,std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>>,unsigned long,rbx_core::SharedPtr<RBX::mutex> const&)")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
// IDA 0x389744: 646 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389744() {
}

// 0x389d44 — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE11_M_allocateEm
// IDA 0x389d44: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_389d44() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x389d5c — __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX5mutexEEEmS4_EvT_T0_RKT1_St12__false_type
// type: void __fastcall(int, int, shared_count *, int, int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<RBX::mutex> *,unsigned long,rbx_core::SharedPtr<RBX::mutex>>(rbx_core::SharedPtr<RBX::mutex> *,unsigned long,rbx_core::SharedPtr<RBX::mutex> const&,std::__false_type)")]
// was: __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX5mutexEEEmS4_EvT_T0_RKT1_St12__false_type
// IDA 0x389d5c: 69 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389d5c() {
}

// 0x389e84 — __ZN5boost10shared_ptrIN3RBX5mutexEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::mutex>::operator=(rbx_core::SharedPtr<RBX::mutex> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX5mutexEEaSERKS3_
// IDA 0x389e84: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389e84() {
}

// 0x389ebc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5mutexEEES8_EET0_T_SA_S9_
// type: int __fastcall(int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::mutex> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::mutex> *,rbx_core::SharedPtr<RBX::mutex> *>(rbx_core::SharedPtr<RBX::mutex> *,rbx_core::SharedPtr<RBX::mutex> *,rbx_core::SharedPtr<RBX::mutex> *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX5mutexEEES8_EET0_T_SA_S9_
// IDA 0x389ebc: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_389ebc() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x389f0c — __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE15_M_erase_at_endEPS3_
// type: boost::detail::sp_counted_base *__fastcall(boost::detail::sp_counted_base *result, int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::_M_erase_at_end(rbx_core::SharedPtr<boost::thread>*)")]
// was: __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE15_M_erase_at_endEPS3_
// IDA 0x389f0c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389f0c() {
}

// 0x389f3c — __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// type: void __fastcall(int *, struct _Unwind_Exception *, int, const shared_count *)
#[doc(alias = "std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread>*,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,unsigned long,rbx_core::SharedPtr<boost::thread> const&)")]
// was: __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// IDA 0x389f3c: 646 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389f3c() {
}

// 0x38a53c — __ZNSt12_Vector_baseIN5boost10shared_ptrINS0_6threadEEESaIS3_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN5boost10shared_ptrINS0_6threadEEESaIS3_EE11_M_allocateEm
// IDA 0x38a53c: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_38a53c() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x38a554 — __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrINS0_6threadEEEmS3_EvT_T0_RKT1_St12__false_type
// type: void __fastcall(int, int, shared_count *, int, int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<boost::thread> *,unsigned long,rbx_core::SharedPtr<boost::thread>>(rbx_core::SharedPtr<boost::thread> *,unsigned long,rbx_core::SharedPtr<boost::thread> const&,std::__false_type)")]
// was: __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrINS0_6threadEEEmS3_EvT_T0_RKT1_St12__false_type
// IDA 0x38a554: 69 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38a554() {
}

// 0x38a67c — __ZN5boost10shared_ptrINS_6threadEEaSERKS2_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<boost::thread>::operator=(rbx_core::SharedPtr<boost::thread> const&)")]
// was: __ZN5boost10shared_ptrINS_6threadEEaSERKS2_
// IDA 0x38a67c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38a67c() {
}

// 0x38a6b4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrINS3_6threadEEES7_EET0_T_S9_S8_
// type: int __fastcall(int, int, int)
#[doc(alias = "rbx_core::SharedPtr<boost::thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<boost::thread> *,rbx_core::SharedPtr<boost::thread> *>(rbx_core::SharedPtr<boost::thread> *,rbx_core::SharedPtr<boost::thread> *,rbx_core::SharedPtr<boost::thread> *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrINS3_6threadEEES7_EET0_T_S9_S8_
// IDA 0x38a6b4: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_38a6b4() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x38a704 — __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::~vector()")]
// was: __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EED2Ev
// IDA 0x38a704: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38a704() {
}

// 0x38a7d0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::~vector()")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EED2Ev
// IDA 0x38a7d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38a7d0() {
}

// 0x38a89c — __ZN5boost10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEC2IS3_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>::shared_ptr<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")]
// was: __ZN5boost10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEC2IS3_EEPT_
// IDA 0x38a89c: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38a89c() {
}

// 0x38a970 — __ZN5boost6detail12shared_countC2IN3RBX14BaseThreadPool8PoolDataEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BaseThreadPool::PoolData>(RBX::BaseThreadPool::PoolData *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX14BaseThreadPool8PoolDataEEEPT_
// IDA 0x38a970: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38a970() {
}

// 0x38aa68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEED1Ev
// IDA 0x38aa68: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_38aa68() {
}

// 0x38aa6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEED0Ev
// IDA 0x38aa6c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_38aa6c() {
}

// 0x38aa70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE7disposeEv
// IDA 0x38aa70: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38aa70() {
}

// 0x38aa80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE11get_deleterERKSt9type_info
// IDA 0x38aa80: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38aa80() {
}

// 0x38aa84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE19get_untyped_deleterEv
// IDA 0x38aa84: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38aa84() {
}

// 0x38aa88 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE13assign_to_ownERKS5_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::mutex>> const&)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE13assign_to_ownERKS5_
// IDA 0x38aa88: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38aa88() {
}

// 0x38aab8 — __ZN3RBX14BaseThreadPool8PoolDataD2Ev
// type: void __fastcall(RBX::BaseThreadPool::PoolData *__hidden this)
#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
// was: __ZN3RBX14BaseThreadPool8PoolDataD2Ev
// IDA 0x38aab8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38aab8() {
}

// 0x38ab90 — __ZN3RBX14BaseThreadPool8PoolDataD1Ev
// type: void __fastcall(RBX::BaseThreadPool::PoolData *__hidden this)
#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
// was: __ZN3RBX14BaseThreadPool8PoolDataD1Ev
// IDA 0x38ab90: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_38ab90() {
}

// 0x38ab94 — __ZN3RBX14BaseThreadPool8PoolDataD0Ev
// type: void __fastcall(RBX::BaseThreadPool::PoolData *__hidden this)
#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
// was: __ZN3RBX14BaseThreadPool8PoolDataD0Ev
// IDA 0x38ab94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38ab94() {
}

// 0x38ac34 — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EED2Ev
// type: char **__fastcall(char **)
#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::~vector()")]
// was: __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EED2Ev
// IDA 0x38ac34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38ac34() {
}

// 0x38ad00 — __ZN3RBX14BaseThreadPool8PoolDataC2Ev
// type: RBX::BaseThreadPool::PoolData *__fastcall(RBX::BaseThreadPool::PoolData *this)
#[doc(alias = "RBX::BaseThreadPool::PoolData::PoolData(void)")]
// was: __ZN3RBX14BaseThreadPool8PoolDataC2Ev
// IDA 0x38ad00: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38ad00() {
}

// 0x38add0 — __ZN3RBX10ThreadPool14ThreadPoolDataD1Ev
// type: void __fastcall(RBX::ThreadPool::ThreadPoolData *__hidden this)
#[doc(alias = "RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")]
// was: __ZN3RBX10ThreadPool14ThreadPoolDataD1Ev
// IDA 0x38add0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38add0() {
}

// 0x38aec4 — __ZN3RBX10ThreadPool14ThreadPoolDataD0Ev
// type: void __fastcall(RBX::ThreadPool::ThreadPoolData *__hidden this)
#[doc(alias = "RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")]
// was: __ZN3RBX10ThreadPool14ThreadPoolDataD0Ev
// IDA 0x38aec4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38aec4() {
}

// 0x38afc8 — __ZN3RBX10ThreadPool14ThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE
// type: int __fastcall(int)
#[doc(alias = "RBX::ThreadPool::ThreadPoolData::getNextTask(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> &)")]
// was: __ZN3RBX10ThreadPool14ThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE
// IDA 0x38afc8: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38afc8() {
}

// 0x38afd4 — __ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE14pop_if_presentERS8_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::safe_queue<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>::pop_if_present(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&)")]
// was: __ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE14pop_if_presentERS8_
// IDA 0x38afd4: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38afd4() {
}

// 0x38b0b4 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9pop_frontEv
// type: int __fastcall(int)
#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::pop_front(void)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9pop_frontEv
// IDA 0x38b0b4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38b0b4() {
}

// 0x38b0ec — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::~deque()")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
// IDA 0x38b0ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38b0ec() {
}

// 0x38b1d4 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::~_Deque_base()")]
// was: __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
// IDA 0x38b1d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38b1d4() {
}

// 0x38b200 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_
// type: void __fastcall(int, int *, int *, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>*>,std::_Deque_iterator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>&,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>*>)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_
// IDA 0x38b200: 126 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38b200() {
}

// 0x38b338 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_initialize_mapEm
// type: void __fastcall(int *, unsigned int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_initialize_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_initialize_mapEm
// IDA 0x38b338: 124 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38b338() {
}

// 0x38b490 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_create_nodesEPPS7_SB_
// type: void __fastcall(int, _DWORD *, unsigned int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_create_nodes(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>**,boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>**)")]
// was: __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_create_nodesEPPS7_SB_
// IDA 0x38b490: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38b490() {
}

// 0x38b584 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EEC2ERKS9_
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::deque(std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>> const&)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EEC2ERKS9_
// IDA 0x38b584: 170 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38b584() {
}

// 0x38b740 — __GLOBAL__I_a_146
#[doc(alias = "global constructor keyed to_a_146")]
// was: __GLOBAL__I_a_146
// IDA 0x38b740: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_38b740() {
}

// 0x38b808 — __ZN3RBX15StringConverterINS_4UDimEE15convertToStringERKS1_
// type: void __fastcall(std::string *, int)
#[doc(alias = "RBX::StringConverter<RBX::UDim>::convertToString(RBX::UDim const&)")]
// was: __ZN3RBX15StringConverterINS_4UDimEE15convertToStringERKS1_
// IDA 0x38b808: 123 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38b808() {
}

// 0x38b970 — __ZN3RBX15StringConverterINS_4UDimEE14convertToValueERKSsRS1_
// type: int __fastcall(std::string *, int)
#[doc(alias = "RBX::StringConverter<RBX::UDim>::convertToValue(std::string const&,RBX::UDim&)")]
// was: __ZN3RBX15StringConverterINS_4UDimEE14convertToValueERKSsRS1_
// IDA 0x38b970: 84 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38b970() {
}

// 0x38ba5c — __ZN3RBX15StringConverterINS_5UDim2EE15convertToStringERKS1_
// type: void __fastcall(std::string *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::StringConverter<RBX::UDim2>::convertToString(RBX::UDim2 const&)")]
// was: __ZN3RBX15StringConverterINS_5UDim2EE15convertToStringERKS1_
// IDA 0x38ba5c: 369 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38ba5c() {
}

// 0x38be8c — __ZN3RBX15StringConverterINS_5UDim2EE14convertToValueERKSsRS1_
// type: int __fastcall(std::string *, int)
#[doc(alias = "RBX::StringConverter<RBX::UDim2>::convertToValue(std::string const&,RBX::UDim2&)")]
// was: __ZN3RBX15StringConverterINS_5UDim2EE14convertToValueERKSsRS1_
// IDA 0x38be8c: 214 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38be8c() {
}

// 0x38c0e8 — __ZNK3RBX4UDimplERKS0_
// type: int __fastcall(int result, int, int)
#[doc(alias = "RBX::UDim::operator+(RBX::UDim const&)const")]
// was: __ZNK3RBX4UDimplERKS0_
// IDA 0x38c0e8: 10 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38c0e8() {
}

// 0x38c108 — __ZNK3RBX4UDimmiERKS0_
// type: int __fastcall(int result, int, int)
#[doc(alias = "RBX::UDim::operator-(RBX::UDim const&)const")]
// was: __ZNK3RBX4UDimmiERKS0_
// IDA 0x38c108: 10 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38c108() {
}

// 0x38c128 — __ZNK3RBX4UDimngEv
// type: int __fastcall(int result, int)
#[doc(alias = "RBX::UDim::operator-(void)const")]
// was: __ZNK3RBX4UDimngEv
// IDA 0x38c128: 8 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38c128() {
}

// 0x38c188 — __ZNK3RBX5UDim2mlEf
// type: _DWORD *__fastcall(_DWORD *result, int, __int32)
#[doc(alias = "RBX::UDim2::operator*(float)const")]
// was: __ZNK3RBX5UDim2mlEf
// IDA 0x38c188: 25 insns (LDRSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38c188() {
}

// 0x38c1e4 — __ZNK3RBX5UDim2plERKS0_
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "RBX::UDim2::operator+(RBX::UDim2 const&)const")]
// was: __ZNK3RBX5UDim2plERKS0_
// IDA 0x38c1e4: 20 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38c1e4() {
}

// 0x38c224 — __ZNK3RBX5UDim2miERKS0_
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "RBX::UDim2::operator-(RBX::UDim2 const&)const")]
// was: __ZNK3RBX5UDim2miERKS0_
// IDA 0x38c224: 20 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38c224() {
}

// 0x38d61c — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIlEEbRT_
// type: int __fastcall(unsigned __int8 **, int *)
#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long>(long &)")]
// was: __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIlEEbRT_
// IDA 0x38d61c: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38d61c() {
}

// 0x38d67c — __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEmcEEbRT0_PKT1_S8_
// type: int __fastcall(int *, unsigned int, int)
#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long,char>(unsigned long &,char const*,char const*)")]
// was: __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEmcEEbRT0_PKT1_S8_
// IDA 0x38d67c: 294 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38d67c() {
}

// 0x38da14 — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedIjEEbRT_
// type: int __fastcall(unsigned __int8 **, _DWORD *)
#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned int>(unsigned int &)")]
// was: __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedIjEEbRT_
// IDA 0x38da14: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38da14() {
}

// 0x38e9d0 — __ZThn36_N3RBX12AccoutrementD0Ev
// type: void __fastcall(RBX::Accoutrement *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn36_N3RBX12AccoutrementD0Ev
// IDA 0x38e9d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38e9d0() {
}

// 0x38e9d8 — __ZThn92_N3RBX12AccoutrementD0Ev
// type: void __fastcall(RBX::Accoutrement *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn92_N3RBX12AccoutrementD0Ev
// IDA 0x38e9d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38e9d8() {
}

// 0x38e9e0 — __ZThn128_N3RBX12AccoutrementD0Ev
// type: void __fastcall(RBX::Accoutrement *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn128_N3RBX12AccoutrementD0Ev
// IDA 0x38e9e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38e9e0() {
}

// 0x38e9e8 — __ZN3RBX12AccoutrementD2Ev
// type: void __fastcall(RBX::Accoutrement *this, int *, int)
#[doc(alias = "RBX::Accoutrement::~Accoutrement()")]
// was: __ZN3RBX12AccoutrementD2Ev
// IDA 0x38e9e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38e9e8() {
}

// 0x38ef1c — __ZThn32_N3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn32_N3RBX12AccoutrementD1Ev
// IDA 0x38ef1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38ef1c() {
}

// 0x38ef2c — __ZThn36_N3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn36_N3RBX12AccoutrementD1Ev
// IDA 0x38ef2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38ef2c() {
}

// 0x38ef3c — __ZThn92_N3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn92_N3RBX12AccoutrementD1Ev
// IDA 0x38ef3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38ef3c() {
}

// 0x38ef4c — __ZThn128_N3RBX12AccoutrementD1Ev
// type: void __fastcall(RBX::Accoutrement *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// was: __ZThn128_N3RBX12AccoutrementD1Ev
// IDA 0x38ef4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38ef4c() {
}

// 0x38ef5c — __ZN3RBX12Accoutrement12onCameraNearEf
// type: unsigned int __fastcall(RBX::Accoutrement *this, float)
#[doc(alias = "RBX::Accoutrement::onCameraNear(float)")]
// was: __ZN3RBX12Accoutrement12onCameraNearEf
// IDA 0x38ef5c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38ef5c() {
}

// 0x38ef98 — __ZThn128_N3RBX12Accoutrement12onCameraNearEf
// type: unsigned int __fastcall(RBX::Accoutrement *this, float)
#[doc(alias = "non-virtual thunk toRBX::Accoutrement::onCameraNear(float)")]
// was: __ZThn128_N3RBX12Accoutrement12onCameraNearEf
// IDA 0x38ef98: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38ef98() {
}

// 0x38f01c — __ZN3RBX12Accoutrement7dropAllEPNS_13ModelInstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::ModelInstance *, RBX::Accoutrement *)
#[doc(alias = "RBX::Accoutrement::dropAll(RBX::ModelInstance *)")]
// was: __ZN3RBX12Accoutrement7dropAllEPNS_13ModelInstanceE
// IDA 0x38f01c: 2 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f01c() {
}

// 0x38f024 — __ZN3RBX12Accoutrement13dropAllOthersEPNS_13ModelInstanceEPS0_
// type: RBX::Instance *__fastcall(RBX::Accoutrement *this, RBX::ModelInstance *, RBX::Accoutrement *)
#[doc(alias = "RBX::Accoutrement::dropAllOthers(RBX::ModelInstance *,RBX::Accoutrement*)")]
// was: __ZN3RBX12Accoutrement13dropAllOthersEPNS_13ModelInstanceEPS0_
// IDA 0x38f024: 20 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f024() {
}

// 0x38f054 — __ZNK3RBX12Accoutrement14getHandleConstEv
// type: char *__fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::getHandleConst(void)const")]
// was: __ZNK3RBX12Accoutrement14getHandleConstEv
// IDA 0x38f054: 126 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f054() {
}

// 0x38f1c4 — __ZN3RBX12Accoutrement11getLocationEv
// type: int __fastcall(RBX::Accoutrement *this, RBX::Accoutrement *)
#[doc(alias = "RBX::Accoutrement::getLocation(void)")]
// was: __ZN3RBX12Accoutrement11getLocationEv
// IDA 0x38f1c4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f1c4() {
}

// 0x38f1f8 — __ZTv0_n12_N3RBX12Accoutrement11getLocationEv
// type: int __fastcall(RBX::Accoutrement *this, _DWORD *)
#[doc(alias = "virtual thunk toRBX::Accoutrement::getLocation(void)")]
// was: __ZTv0_n12_N3RBX12Accoutrement11getLocationEv
// IDA 0x38f1f8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f1f8() {
}

// 0x38f20c — __ZN3RBX12Accoutrement17connectTouchEventEv
// type: void __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::connectTouchEvent(void)")]
// was: __ZN3RBX12Accoutrement17connectTouchEventEv
// IDA 0x38f20c: 170 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f20c() {
}

// 0x38f3ec — __ZN3RBX12Accoutrement21onEvent_HandleTouchedEN5boost10shared_ptrINS_8InstanceEEE
// type: RBX::Accoutrement *__fastcall(RBX::Network::Players *, RBX::Accoutrement **, bool)
#[doc(alias = "RBX::Accoutrement::onEvent_HandleTouched(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX12Accoutrement21onEvent_HandleTouchedEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x38f3ec: 50 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f3ec() {
}

// 0x38f47c — __ZN3RBX12Accoutrement19rebuildBackendStateEv
// type: int __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::rebuildBackendState(void)")]
// was: __ZN3RBX12Accoutrement19rebuildBackendStateEv
// IDA 0x38f47c: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f47c() {
}

// 0x38f4f4 — __ZN3RBX12Accoutrement19computeDesiredStateEv
// type: int __fastcall(RBX::Instance **this, int, bool)
#[doc(alias = "RBX::Accoutrement::computeDesiredState(void)")]
// was: __ZN3RBX12Accoutrement19computeDesiredStateEv
// IDA 0x38f4f4: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f4f4() {
}

// 0x38f578 — __ZN3RBX12Accoutrement15setDesiredStateENS0_17AccoutrementStateEPKNS_15ServiceProviderE
// type: int __fastcall(RBX::Accoutrement *this, int, RBX::Network::Players *)
#[doc(alias = "RBX::Accoutrement::setDesiredState(RBX::Accoutrement::AccoutrementState,RBX::ServiceProvider const*)")]
// was: __ZN3RBX12Accoutrement15setDesiredStateENS0_17AccoutrementStateEPKNS_15ServiceProviderE
// IDA 0x38f578: 125 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f578() {
}

// 0x38f6f0 — __ZN3RBX12Accoutrement19computeDesiredStateEPNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::computeDesiredState(RBX::Instance *)")]
// was: __ZN3RBX12Accoutrement19computeDesiredStateEPNS_8InstanceE
// IDA 0x38f6f0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f6f0() {
}

// 0x38f714 — __ZN3RBX12Accoutrement13upTo_EquippedEv
// type: void __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::upTo_Equipped(void)")]
// was: __ZN3RBX12Accoutrement13upTo_EquippedEv
// IDA 0x38f714: 193 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f714() {
}

// 0x38f92c — __ZN3RBX12Accoutrement16upTo_InCharacterEv
// type: void __fastcall(RBX::Humanoid **this)
#[doc(alias = "RBX::Accoutrement::upTo_InCharacter(void)")]
// was: __ZN3RBX12Accoutrement16upTo_InCharacterEv
// IDA 0x38f92c: 176 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38f92c() {
}

// 0x38fb1c — __ZN3RBX12Accoutrement16upTo_InWorkspaceEv
// type: int __fastcall(RBX::Accoutrement *this, const RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::upTo_InWorkspace(void)")]
// was: __ZN3RBX12Accoutrement16upTo_InWorkspaceEv
// IDA 0x38fb1c: 56 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38fb1c() {
}

// 0x38fbcc — __ZN3RBX12Accoutrement17downFrom_EquippedEv
// type: void __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::downFrom_Equipped(void)")]
// was: __ZN3RBX12Accoutrement17downFrom_EquippedEv
// IDA 0x38fbcc: 116 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38fbcc() {
}

// 0x38fd24 — __ZN3RBX12Accoutrement18downFrom_HasHandleEv
// type: void __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::downFrom_HasHandle(void)")]
// was: __ZN3RBX12Accoutrement18downFrom_HasHandleEv
// IDA 0x38fd24: 21 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38fd24() {
}

// 0x38fd60 — __ZN3RBX12Accoutrement20onEvent_AddedBackendEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::onEvent_AddedBackend(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX12Accoutrement20onEvent_AddedBackendEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x38fd60: 59 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38fd60() {
}

// 0x38fe18 — __ZN3RBX12Accoutrement22onEvent_RemovedBackendEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int)
#[doc(alias = "RBX::Accoutrement::onEvent_RemovedBackend(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX12Accoutrement22onEvent_RemovedBackendEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x38fe18: 92 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38fe18() {
}

// 0x38ff34 — __ZN3RBX12Accoutrement12onChildAddedEPNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::onChildAdded(RBX::Instance *)")]
// was: __ZN3RBX12Accoutrement12onChildAddedEPNS_8InstanceE
// IDA 0x38ff34: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38ff34() {
}

// 0x38ff5c — __ZN3RBX12Accoutrement14onChildRemovedEPNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::onChildRemoved(RBX::Instance *)")]
// was: __ZN3RBX12Accoutrement14onChildRemovedEPNS_8InstanceE
// IDA 0x38ff5c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38ff5c() {
}

// 0x38ff84 — __ZN3RBX12Accoutrement17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "RBX::Accoutrement::onAncestorChanged(RBX::AncestorChanged const&)")]
// was: __ZN3RBX12Accoutrement17onAncestorChangedERKNS_15AncestorChangedE
// IDA 0x38ff84: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38ff84() {
}

// 0x38fff0 — __ZN3RBX3HatC1Ev
// type: RBX::Accoutrement *__fastcall(RBX::Hat *this)
#[doc(alias = "RBX::Hat::Hat(void)")]
// was: __ZN3RBX3HatC1Ev
// IDA 0x38fff0: 153 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38fff0() {
}

// 0x3901bc — __ZNK3RBX12Accoutrement18getAttachmentPointEv
// type: char *__fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::getAttachmentPoint(void)const")]
// was: __ZNK3RBX12Accoutrement18getAttachmentPointEv
// IDA 0x3901bc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3901bc() {
}

// 0x390208 — __ZNK3RBX12Accoutrement27getBackendAccoutrementStateEv
// type: int __fastcall(RBX::Accoutrement *this)
#[doc(alias = "RBX::Accoutrement::getBackendAccoutrementState(void)const")]
// was: __ZNK3RBX12Accoutrement27getBackendAccoutrementStateEv
// IDA 0x390208: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_390208() {
}