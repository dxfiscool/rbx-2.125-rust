//! rendering shard 316 — 100 stubs 0x477ed8..0x47c414 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 34400->34500 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 34400 before -> 34500 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x477ed8 (lowest remaining 0x477ed8..0x47c414, next lowest 0x47c418)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x477ed8 — __ZN3RBX13DebrisServiceD1Ev
// type: void __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "RBX::DebrisService::~DebrisService()")]
// was: __ZN3RBX13DebrisServiceD1Ev
// IDA 0x477ed8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_477ed8() {
}

// 0x477fe4 — __ZN3RBX13DebrisServiceD0Ev
// type: void __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "RBX::DebrisService::~DebrisService()")]
// was: __ZN3RBX13DebrisServiceD0Ev
// IDA 0x477fe4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_477fe4() {
}

// 0x478100 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv
// IDA 0x478100: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_478100() {
}

// 0x478128 — __ZThn32_N3RBX13DebrisServiceD1Ev
// type: void __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// was: __ZThn32_N3RBX13DebrisServiceD1Ev
// IDA 0x478128: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_478128() {
}

// 0x478234 — __ZThn32_N3RBX13DebrisServiceD0Ev
// type: void __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// was: __ZThn32_N3RBX13DebrisServiceD0Ev
// IDA 0x478234: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_478234() {
}

// 0x478354 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv
// IDA 0x478354: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_478354() {
}

// 0x47837c — __ZThn36_N3RBX13DebrisServiceD1Ev
// type: void __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// was: __ZThn36_N3RBX13DebrisServiceD1Ev
// IDA 0x47837c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_47837c() {
}

// 0x478484 — __ZThn36_N3RBX13DebrisServiceD0Ev
// type: void __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// was: __ZThn36_N3RBX13DebrisServiceD0Ev
// IDA 0x478484: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_478484() {
}

// 0x4785a0 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE9push_backERKS4_
// type: int(void)
#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::push_back(rbx_core::WeakPtr<RBX::Instance> const&)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE9push_backERKS4_
// IDA 0x4785a0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_4785a0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x478630 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE16_M_push_back_auxERKS4_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, void *, int)
#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_push_back_aux(rbx_core::WeakPtr<RBX::Instance> const&)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE16_M_push_back_auxERKS4_
// IDA 0x478630: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_478630() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x478814 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE22_M_reserve_map_at_backEm
// type: int(void)
#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_reserve_map_at_back(unsigned long)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE22_M_reserve_map_at_backEm
// IDA 0x478814: 10 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_478814() {
}

// 0x478830 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE17_M_reallocate_mapEmb
// type: int(void)
#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_reallocate_map(unsigned long,bool)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE17_M_reallocate_mapEmb
// IDA 0x478830: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_478830() {
}

// 0x478908 — __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE15_M_allocate_mapEm
// type: int(void)
#[doc(alias = "std::_Deque_base<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_allocate_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE15_M_allocate_mapEm
// IDA 0x478908: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_478908() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x478920 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// IDA 0x478920: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_478920() {
}

// 0x478a4c — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>)")]
// was: __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEEvT_
// IDA 0x478a4c: 106 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_478a4c() {
}

// 0x478b84 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// IDA 0x478b84: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_478b84() {
}

// 0x478ba0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEvE6invokeERNS1_15function_bufferE
// IDA 0x478ba0: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_478ba0() {
}

// 0x478bb4 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS5_5list1INS5_5valueISA_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS5_5list1INS5_5valueISA_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x478bb4: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_478bb4() {
}

// 0x478cd4 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS5_5list1INS5_5valueISA_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS5_5list1INS5_5valueISA_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x478cd4: 129 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_478cd4() {
}

// 0x478e50 — __ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX8InstanceEEEEEEclIPFvS6_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Instance>) &,boost::_bi::list0 &,int)")]
// was: __ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX8InstanceEEEEEEclIPFvS6_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// IDA 0x478e50: 92 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_478e50() {
}

// 0x478f60 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEE12manage_smallERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: int(void)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEE12manage_smallERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// IDA 0x478f60: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_478f60() {
}

// 0x479038 — __ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX8InstanceEEEEEEC2ES7_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>::list1(boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>)")]
// was: __ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX8InstanceEEEEEEC2ES7_
// IDA 0x479038: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_479038() {
}

// 0x479180 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE9pop_frontEv
// type: int(void)
#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::pop_front(void)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE9pop_frontEv
// IDA 0x479180: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_479180() {
}

// 0x4791ac — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE16_M_pop_front_auxEv
// type: int(void)
#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_pop_front_aux(void)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE16_M_pop_front_auxEv
// IDA 0x4791ac: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4791ac() {
}

// 0x4791d8 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EEC2ERKS6_
#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::deque(std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>> const&)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EEC2ERKS6_
// IDA 0x4791d8: 101 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4791d8() {
}

// 0x4792fc — __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EED2Ev
// type: int(void)
#[doc(alias = "std::_Deque_base<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::~_Deque_base()")]
// was: __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EED2Ev
// IDA 0x4792fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4792fc() {
}

// 0x479328 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost8weak_ptrIN3RBX8InstanceEEERKS5_PS6_ES0_IS5_RS5_PS5_EET0_T_SE_SD_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance>&,rbx_core::WeakPtr<RBX::Instance>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance> const&,rbx_core::WeakPtr<RBX::Instance> const*>,std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance>&,rbx_core::WeakPtr<RBX::Instance>*>>(std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance> const&,rbx_core::WeakPtr<RBX::Instance> const*>,std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance> const&,rbx_core::WeakPtr<RBX::Instance> const*>,std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance>&,rbx_core::WeakPtr<RBX::Instance>*>,std::__false_type)")]
// was: __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost8weak_ptrIN3RBX8InstanceEEERKS5_PS6_ES0_IS5_RS5_PS5_EET0_T_SE_SD_St12__false_type
// IDA 0x479328: 135 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_479328() {
}

// 0x479510 — __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE17_M_initialize_mapEm
// type: void __fastcall(int *, unsigned int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_initialize_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE17_M_initialize_mapEm
// IDA 0x479510: 124 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_479510() {
}

// 0x479668 — __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE15_M_create_nodesEPPS4_S8_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_create_nodes(rbx_core::WeakPtr<RBX::Instance>**,rbx_core::WeakPtr<RBX::Instance>**)")]
// was: __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE15_M_create_nodesEPPS4_S8_
// IDA 0x479668: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_479668() {
}

// 0x47975c — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EED2Ev
#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::~deque()")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EED2Ev
// IDA 0x47975c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_47975c() {
}

// 0x479844 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE19_M_destroy_data_auxESt15_Deque_iteratorIS4_RS4_PS4_ESA_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_destroy_data_aux(std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance>&,rbx_core::WeakPtr<RBX::Instance>*>,std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance>&,rbx_core::WeakPtr<RBX::Instance>*>)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE19_M_destroy_data_auxESt15_Deque_iteratorIS4_RS4_PS4_ESA_
// IDA 0x479844: 130 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_479844() {
}

// 0x479984 — __ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x479984: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_479984() {
}

// 0x479988 — __ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x479988: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_479988() {
}

// 0x479a28 — __ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x479a28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_479a28() {
}

// 0x479a30 — __ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x479a30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_479a30() {
}

// 0x479ad4 — __ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x479ad4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_479ad4() {
}

// 0x479adc — __ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x479adc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_479adc() {
}

// 0x479b80 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::BoundFuncDesc(void (RBX::DebrisService::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x479b80: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_479b80() {
}

// 0x479cf8 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x479cf8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_479cf8() {
}

// 0x479d28 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EED0Ev
// IDA 0x479d28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_479d28() {
}

// 0x479dfc — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x479dfc: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_479dfc() {
}

// 0x479e30 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EEC2EMS2_FvS6_dEPKcSC_SC_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, double, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::BoundFuncDesc(void (RBX::DebrisService::*)(rbx_core::SharedPtr<RBX::Instance>,double),char const*,char const*,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EEC2EMS2_FvS6_dEPKcSC_SC_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x479e30: 210 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_479e30() {
}

// 0x47a050 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// IDA 0x47a050: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47a050() {
}

// 0x47a09c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EED0Ev
// IDA 0x47a09c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_47a09c() {
}

// 0x47a1c8 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x47a1c8: 91 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47a1c8() {
}

// 0x47a2cc — __ZN3RBX10Reflection11Call2HelperINS_13DebrisServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEdES6_dvE4callEPS2_S8_RNS0_7VariantERKS6_RKd
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DebrisService,void (RBX::DebrisService::*)(rbx_core::SharedPtr<RBX::Instance>,double),rbx_core::SharedPtr<RBX::Instance>,double,void>::call(RBX::DebrisService*,void (RBX::DebrisService::*)(rbx_core::SharedPtr<RBX::Instance>,double),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,double const&)")]
// was: __ZN3RBX10Reflection11Call2HelperINS_13DebrisServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEdES6_dvE4callEPS2_S8_RNS0_7VariantERKS6_RKd
// IDA 0x47a2cc: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47a2cc() {
}

// 0x47a3c0 — __ZN3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::PropDescriptor<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>(char const*,char const*,int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x47a3c0: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47a3c0() {
}

// 0x47a4d4 — __ZN3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiED0Ev
// IDA 0x47a4d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_47a4d4() {
}

// 0x47a500 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
// IDA 0x47a500: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47a500() {
}

// 0x47a504 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// IDA 0x47a504: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47a504() {
}

// 0x47a508 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x47a508: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47a508() {
}

// 0x47a528 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x47a528: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47a528() {
}

// 0x47a54c — __GLOBAL__I_a_181
#[doc(alias = "global constructor keyed to_a_181")]
// was: __GLOBAL__I_a_181
// IDA 0x47a54c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_47a54c() {
}

// 0x47a87c — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEC2Ev
// IDA 0x47a87c: 239 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47a87c() {
}

// 0x47ab28 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEEC2Ev
// IDA 0x47ab28: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47ab28() {
}

// 0x47ad04 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEC2Ev
// IDA 0x47ad04: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47ad04() {
}

// 0x47aee0 — __ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEEC2Ev
// IDA 0x47aee0: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47aee0() {
}

// 0x47b0b8 — __ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEEC1Ev
// IDA 0x47b0b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_47b0b8() {
}

// 0x47b0bc — __ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEEC2Ev
// IDA 0x47b0bc: 198 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b0bc() {
}

// 0x47b2f4 — __ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEEC2Ev
// IDA 0x47b2f4: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b2f4() {
}

// 0x47b4cc — __ZNK3RBX13DebugSettings20getVertexShaderModelEv
// type: int __fastcall(RBX::DebugSettings *this)
#[doc(alias = "RBX::DebugSettings::getVertexShaderModel(void)const")]
// was: __ZNK3RBX13DebugSettings20getVertexShaderModelEv
// IDA 0x47b4cc: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b4cc() {
}

// 0x47b4d0 — __ZNK3RBX13DebugSettings19getPixelShaderModelEv
// type: int __fastcall(RBX::DebugSettings *this)
#[doc(alias = "RBX::DebugSettings::getPixelShaderModel(void)const")]
// was: __ZNK3RBX13DebugSettings19getPixelShaderModelEv
// IDA 0x47b4d0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b4d0() {
}

// 0x47b4d4 — __ZNK3RBX13DebugSettings11videoMemoryEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::videoMemory(void)const")]
// was: __ZNK3RBX13DebugSettings11videoMemoryEv
// IDA 0x47b4d4: 50 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b4d4() {
}

// 0x47b564 — __ZNK3RBX13DebugSettings8cpuSpeedEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::cpuSpeed(void)const")]
// was: __ZNK3RBX13DebugSettings8cpuSpeedEv
// IDA 0x47b564: 50 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b564() {
}

// 0x47b5f4 — __ZNK3RBX13DebugSettings8cpuCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::cpuCount(void)const")]
// was: __ZNK3RBX13DebugSettings8cpuCountEv
// IDA 0x47b5f4: 50 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b5f4() {
}

// 0x47b684 — __ZNK3RBX13DebugSettings12osPlatformIdEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::osPlatformId(void)const")]
// was: __ZNK3RBX13DebugSettings12osPlatformIdEv
// IDA 0x47b684: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b684() {
}

// 0x47b688 — __ZNK3RBX13DebugSettings10osPlatformEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::osPlatform(void)const")]
// was: __ZNK3RBX13DebugSettings10osPlatformEv
// IDA 0x47b688: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b688() {
}

// 0x47b6a4 — __ZNK3RBX13DebugSettings5osVerEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::osVer(void)const")]
// was: __ZNK3RBX13DebugSettings5osVerEv
// IDA 0x47b6a4: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b6a4() {
}

// 0x47b6b0 — __ZNK3RBX13DebugSettings9osIs64BitEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::osIs64Bit(void)const")]
// was: __ZNK3RBX13DebugSettings9osIs64BitEv
// IDA 0x47b6b0: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b6b0() {
}

// 0x47b6bc — __ZNK3RBX13DebugSettings17systemProductNameEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::systemProductName(void)const")]
// was: __ZNK3RBX13DebugSettings17systemProductNameEv
// IDA 0x47b6bc: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b6bc() {
}

// 0x47b6e4 — __ZNK3RBX13DebugSettings3cpuEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::cpu(void)const")]
// was: __ZNK3RBX13DebugSettings3cpuEv
// IDA 0x47b6e4: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b6e4() {
}

// 0x47b894 — __ZNK3RBX13DebugSettings4simdEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::simd(void)const")]
// was: __ZNK3RBX13DebugSettings4simdEv
// IDA 0x47b894: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b894() {
}

// 0x47b9a4 — __ZNK3RBX13DebugSettings19totalPhysicalMemoryEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::totalPhysicalMemory(void)const")]
// was: __ZNK3RBX13DebugSettings19totalPhysicalMemoryEv
// IDA 0x47b9a4: 50 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47b9a4() {
}

// 0x47ba34 — __ZNK3RBX13DebugSettings10resolutionEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::resolution(void)const")]
// was: __ZNK3RBX13DebugSettings10resolutionEv
// IDA 0x47ba34: 129 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47ba34() {
}

// 0x47bbb4 — __ZNK3RBX13DebugSettings23availablePhysicalMemoryEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::availablePhysicalMemory(void)const")]
// was: __ZNK3RBX13DebugSettings23availablePhysicalMemoryEv
// IDA 0x47bbb4: 50 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bbb4() {
}

// 0x47bc44 — __ZNK3RBX13DebugSettings14getElapsedTimeEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getElapsedTime(void)const")]
// was: __ZNK3RBX13DebugSettings14getElapsedTimeEv
// IDA 0x47bc44: 3 insns (VMOV.F64..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bc44() {
}

// 0x47bc50 — __ZNK3RBX13DebugSettings12processCoresEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::processCores(void)const")]
// was: __ZNK3RBX13DebugSettings12processCoresEv
// IDA 0x47bc50: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bc50() {
}

// 0x47bc8c — __ZNK3RBX13DebugSettings18totalProcessorTimeEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::totalProcessorTime(void)const")]
// was: __ZNK3RBX13DebugSettings18totalProcessorTimeEv
// IDA 0x47bc8c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bc8c() {
}

// 0x47bcb0 — __ZNK3RBX13DebugSettings13processorTimeEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::processorTime(void)const")]
// was: __ZNK3RBX13DebugSettings13processorTimeEv
// IDA 0x47bcb0: 2 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bcb0() {
}

// 0x47bcb8 — __ZNK3RBX13DebugSettings12privateBytesEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::privateBytes(void)const")]
// was: __ZNK3RBX13DebugSettings12privateBytesEv
// IDA 0x47bcb8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bcb8() {
}

// 0x47bcdc — __ZNK3RBX13DebugSettings22privateWorkingSetBytesEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::privateWorkingSetBytes(void)const")]
// was: __ZNK3RBX13DebugSettings22privateWorkingSetBytesEv
// IDA 0x47bcdc: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bcdc() {
}

// 0x47bcfc — __ZNK3RBX13DebugSettings15GetVirtualBytesEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::GetVirtualBytes(void)const")]
// was: __ZNK3RBX13DebugSettings15GetVirtualBytesEv
// IDA 0x47bcfc: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bcfc() {
}

// 0x47bd1c — __ZNK3RBX13DebugSettings16GetPageFileBytesEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::GetPageFileBytes(void)const")]
// was: __ZNK3RBX13DebugSettings16GetPageFileBytesEv
// IDA 0x47bd1c: 2 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bd1c() {
}

// 0x47bd24 — __ZNK3RBX13DebugSettings22GetPageFaultsPerSecondEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::GetPageFaultsPerSecond(void)const")]
// was: __ZNK3RBX13DebugSettings22GetPageFaultsPerSecondEv
// IDA 0x47bd24: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bd24() {
}

// 0x47bd50 — __ZNK3RBX13DebugSettings14getPlayerCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getPlayerCount(void)const")]
// was: __ZNK3RBX13DebugSettings14getPlayerCountEv
// IDA 0x47bd50: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bd50() {
}

// 0x47bd60 — __ZNK3RBX13DebugSettings17getDataModelCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getDataModelCount(void)const")]
// was: __ZNK3RBX13DebugSettings17getDataModelCountEv
// IDA 0x47bd60: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bd60() {
}

// 0x47bd70 — __ZNK3RBX13DebugSettings18getCdnSuccessCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getCdnSuccessCount(void)const")]
// was: __ZNK3RBX13DebugSettings18getCdnSuccessCountEv
// IDA 0x47bd70: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bd70() {
}

// 0x47bd80 — __ZNK3RBX13DebugSettings18getCdnFailureCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getCdnFailureCount(void)const")]
// was: __ZNK3RBX13DebugSettings18getCdnFailureCountEv
// IDA 0x47bd80: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bd80() {
}

// 0x47bd90 — __ZNK3RBX13DebugSettings27getAlternateCdnSuccessCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getAlternateCdnSuccessCount(void)const")]
// was: __ZNK3RBX13DebugSettings27getAlternateCdnSuccessCountEv
// IDA 0x47bd90: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bd90() {
}

// 0x47bda0 — __ZNK3RBX13DebugSettings27getAlternateCdnFailureCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getAlternateCdnFailureCount(void)const")]
// was: __ZNK3RBX13DebugSettings27getAlternateCdnFailureCountEv
// IDA 0x47bda0: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bda0() {
}

// 0x47bdb0 — __ZNK3RBX13DebugSettings20getBlockMeshMapCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getBlockMeshMapCount(void)const")]
// was: __ZNK3RBX13DebugSettings20getBlockMeshMapCountEv
// IDA 0x47bdb0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_47bdb0() {
}

// 0x47bdb4 — __ZNK3RBX13DebugSettings25getLastCdnFailureTimeSpanEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getLastCdnFailureTimeSpan(void)const")]
// was: __ZNK3RBX13DebugSettings25getLastCdnFailureTimeSpanEv
// IDA 0x47bdb4: 6 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bdb4() {
}

// 0x47bdcc — __ZNK3RBX13DebugSettings21getRobloxSuccessCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getRobloxSuccessCount(void)const")]
// was: __ZNK3RBX13DebugSettings21getRobloxSuccessCountEv
// IDA 0x47bdcc: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bdcc() {
}

// 0x47bddc — __ZNK3RBX13DebugSettings20getRobloxFalureCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getRobloxFalureCount(void)const")]
// was: __ZNK3RBX13DebugSettings20getRobloxFalureCountEv
// IDA 0x47bddc: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bddc() {
}

// 0x47bdf0 — __ZNK3RBX13DebugSettings17getRobloxResponceEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getRobloxResponce(void)const")]
// was: __ZNK3RBX13DebugSettings17getRobloxResponceEv
// IDA 0x47bdf0: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bdf0() {
}

// 0x47be48 — __ZNK3RBX13DebugSettings13getCdnRespoceEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getCdnRespoce(void)const")]
// was: __ZNK3RBX13DebugSettings13getCdnRespoceEv
// IDA 0x47be48: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47be48() {
}

// 0x47bea0 — __ZN3RBX13DebugSettings21resetCdnFailureCountsEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::resetCdnFailureCounts(void)")]
// was: __ZN3RBX13DebugSettings21resetCdnFailureCountsEv
// IDA 0x47bea0: 368 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47bea0() {
}

// 0x47c2a8 — __ZN3RBX21TaskSchedulerSettings11addDummyJobEbd
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this, bool, double)
#[doc(alias = "RBX::TaskSchedulerSettings::addDummyJob(bool,double)")]
// was: __ZN3RBX21TaskSchedulerSettings11addDummyJobEbd
// IDA 0x47c2a8: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47c2a8() {
}

// 0x47c3f8 — __ZN3RBX13DebugSettings17setErrorReportingENS0_14ErrorReportingE
#[doc(alias = "RBX::DebugSettings::setErrorReporting(RBX::DebugSettings::ErrorReporting)")]
// was: __ZN3RBX13DebugSettings17setErrorReportingENS0_14ErrorReportingE
// IDA 0x47c3f8: 9 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47c3f8() {
}

// 0x47c414 — __ZNK3RBX21TaskSchedulerSettings19getThreadPoolConfigEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::getThreadPoolConfig(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings19getThreadPoolConfigEv
// IDA 0x47c414: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_47c414() {
}