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
pub fn stub_477ed8() -> ! {
    todo!("0x477ed8 RBX::DebrisService::~DebrisService()")
}

// 0x477fe4 — __ZN3RBX13DebrisServiceD0Ev
// type: void __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "RBX::DebrisService::~DebrisService()")]
// was: __ZN3RBX13DebrisServiceD0Ev
pub fn stub_477fe4() -> ! {
    todo!("0x477fe4 RBX::DebrisService::~DebrisService()")
}

// 0x478100 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv
pub fn stub_478100() -> ! {
    todo!("0x478100 __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv")
}

// 0x478128 — __ZThn32_N3RBX13DebrisServiceD1Ev
// type: void __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// was: __ZThn32_N3RBX13DebrisServiceD1Ev
pub fn stub_478128() -> ! {
    todo!("0x478128 non-virtual thunk toRBX::DebrisService::~DebrisService()")
}

// 0x478234 — __ZThn32_N3RBX13DebrisServiceD0Ev
// type: void __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// was: __ZThn32_N3RBX13DebrisServiceD0Ev
pub fn stub_478234() -> ! {
    todo!("0x478234 non-virtual thunk toRBX::DebrisService::~DebrisService()")
}

// 0x478354 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv
pub fn stub_478354() -> ! {
    todo!("0x478354 __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv")
}

// 0x47837c — __ZThn36_N3RBX13DebrisServiceD1Ev
// type: void __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// was: __ZThn36_N3RBX13DebrisServiceD1Ev
pub fn stub_47837c() -> ! {
    todo!("0x47837c non-virtual thunk toRBX::DebrisService::~DebrisService()")
}

// 0x478484 — __ZThn36_N3RBX13DebrisServiceD0Ev
// type: void __fastcall(RBX::DebrisService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// was: __ZThn36_N3RBX13DebrisServiceD0Ev
pub fn stub_478484() -> ! {
    todo!("0x478484 non-virtual thunk toRBX::DebrisService::~DebrisService()")
}

// 0x4785a0 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE9push_backERKS4_
// type: int(void)
#[doc(alias = "std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::push_back(boost::weak_ptr<RBX::Instance> const&)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE9push_backERKS4_
pub fn stub_4785a0() -> ! {
    todo!("0x4785a0 std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::push_back(boost::weak_ptr<RBX::Instance> const&)")
}

// 0x478630 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE16_M_push_back_auxERKS4_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, void *, int)
#[doc(alias = "std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_push_back_aux(boost::weak_ptr<RBX::Instance> const&)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE16_M_push_back_auxERKS4_
pub fn stub_478630() -> ! {
    todo!("0x478630 std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_push_back_aux(boost::weak_ptr<RBX::Instance> const&)")
}

// 0x478814 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE22_M_reserve_map_at_backEm
// type: int(void)
#[doc(alias = "std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_reserve_map_at_back(unsigned long)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE22_M_reserve_map_at_backEm
pub fn stub_478814() -> ! {
    todo!("0x478814 std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_reserve_map_at_back(unsigned long)")
}

// 0x478830 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE17_M_reallocate_mapEmb
// type: int(void)
#[doc(alias = "std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_reallocate_map(unsigned long,bool)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE17_M_reallocate_mapEmb
pub fn stub_478830() -> ! {
    todo!("0x478830 std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_reallocate_map(unsigned long,bool)")
}

// 0x478908 — __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE15_M_allocate_mapEm
// type: int(void)
#[doc(alias = "std::_Deque_base<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_allocate_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE15_M_allocate_mapEm
pub fn stub_478908() -> ! {
    todo!("0x478908 std::_Deque_base<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_allocate_map(unsigned long)")
}

// 0x478920 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
pub fn stub_478920() -> ! {
    todo!("0x478920 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")
}

// 0x478a4c — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>)")]
// was: __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEEvT_
pub fn stub_478a4c() -> ! {
    todo!("0x478a4c void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>)")
}

// 0x478b84 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
pub fn stub_478b84() -> ! {
    todo!("0x478b84 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x478ba0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEvE6invokeERNS1_15function_bufferE
pub fn stub_478ba0() -> ! {
    todo!("0x478ba0 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x478bb4 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS5_5list1INS5_5valueISA_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS5_5list1INS5_5valueISA_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_478bb4() -> ! {
    todo!("0x478bb4 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")
}

// 0x478cd4 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS5_5list1INS5_5valueISA_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS5_5list1INS5_5valueISA_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_478cd4() -> ! {
    todo!("0x478cd4 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x478e50 — __ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX8InstanceEEEEEEclIPFvS6_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>::operator()<void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Instance>) &,boost::_bi::list0 &,int)")]
// was: __ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX8InstanceEEEEEEclIPFvS6_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
pub fn stub_478e50() -> ! {
    todo!("0x478e50 void boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>::operator()<void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Instance>) &,boost::_bi::list0 &,int)")
}

// 0x478f60 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEE12manage_smallERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: int(void)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEE12manage_smallERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
pub fn stub_478f60() -> ! {
    todo!("0x478f60 boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x479038 — __ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX8InstanceEEEEEEC2ES7_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>::list1(boost::_bi::value<boost::weak_ptr<RBX::Instance>>)")]
// was: __ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX8InstanceEEEEEEC2ES7_
pub fn stub_479038() -> ! {
    todo!("0x479038 boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>::list1(boost::_bi::value<boost::weak_ptr<RBX::Instance>>)")
}

// 0x479180 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE9pop_frontEv
// type: int(void)
#[doc(alias = "std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::pop_front(void)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE9pop_frontEv
pub fn stub_479180() -> ! {
    todo!("0x479180 std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::pop_front(void)")
}

// 0x4791ac — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE16_M_pop_front_auxEv
// type: int(void)
#[doc(alias = "std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_pop_front_aux(void)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE16_M_pop_front_auxEv
pub fn stub_4791ac() -> ! {
    todo!("0x4791ac std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_pop_front_aux(void)")
}

// 0x4791d8 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EEC2ERKS6_
#[doc(alias = "std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::deque(std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>> const&)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EEC2ERKS6_
pub fn stub_4791d8() -> ! {
    todo!("0x4791d8 std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::deque(std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>> const&)")
}

// 0x4792fc — __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EED2Ev
// type: int(void)
#[doc(alias = "std::_Deque_base<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::~_Deque_base()")]
// was: __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EED2Ev
pub fn stub_4792fc() -> ! {
    todo!("0x4792fc std::_Deque_base<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::~_Deque_base()")
}

// 0x479328 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost8weak_ptrIN3RBX8InstanceEEERKS5_PS6_ES0_IS5_RS5_PS5_EET0_T_SE_SD_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*> std::__uninitialized_copy_aux<std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance> const&,boost::weak_ptr<RBX::Instance> const*>,std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*>>(std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance> const&,boost::weak_ptr<RBX::Instance> const*>,std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance> const&,boost::weak_ptr<RBX::Instance> const*>,std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*>,std::__false_type)")]
// was: __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost8weak_ptrIN3RBX8InstanceEEERKS5_PS6_ES0_IS5_RS5_PS5_EET0_T_SE_SD_St12__false_type
pub fn stub_479328() -> ! {
    todo!("0x479328 std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*> std::__uninitialized_copy_aux<std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance> const&,boost::weak_ptr<RBX::Instance> const*>,std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*>>(std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance> const&,boost::weak_ptr<RBX::Instance> const*>,std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance> const&,boost::weak_ptr<RBX::Instance> const*>,std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*>,std::__false_type)")
}

// 0x479510 — __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE17_M_initialize_mapEm
// type: void __fastcall(int *, unsigned int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_initialize_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE17_M_initialize_mapEm
pub fn stub_479510() -> ! {
    todo!("0x479510 std::_Deque_base<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_initialize_map(unsigned long)")
}

// 0x479668 — __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE15_M_create_nodesEPPS4_S8_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_create_nodes(boost::weak_ptr<RBX::Instance>**,boost::weak_ptr<RBX::Instance>**)")]
// was: __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE15_M_create_nodesEPPS4_S8_
pub fn stub_479668() -> ! {
    todo!("0x479668 std::_Deque_base<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_create_nodes(boost::weak_ptr<RBX::Instance>**,boost::weak_ptr<RBX::Instance>**)")
}

// 0x47975c — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EED2Ev
#[doc(alias = "std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::~deque()")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EED2Ev
pub fn stub_47975c() -> ! {
    todo!("0x47975c std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::~deque()")
}

// 0x479844 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE19_M_destroy_data_auxESt15_Deque_iteratorIS4_RS4_PS4_ESA_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*>,std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*>)")]
// was: __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE19_M_destroy_data_auxESt15_Deque_iteratorIS4_RS4_PS4_ESA_
pub fn stub_479844() -> ! {
    todo!("0x479844 std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*>,std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*>)")
}

// 0x479984 — __ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_479984() -> ! {
    todo!("0x479984 __ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x479988 — __ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_479988() -> ! {
    todo!("0x479988 __ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x479a28 — __ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_479a28() -> ! {
    todo!("0x479a28 __ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x479a30 — __ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_479a30() -> ! {
    todo!("0x479a30 __ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x479ad4 — __ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_479ad4() -> ! {
    todo!("0x479ad4 __ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x479adc — __ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_479adc() -> ! {
    todo!("0x479adc __ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x479b80 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::BoundFuncDesc(void (RBX::DebrisService::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_479b80() -> ! {
    todo!("0x479b80 RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::BoundFuncDesc(void (RBX::DebrisService::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x479cf8 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_479cf8() -> ! {
    todo!("0x479cf8 RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x479d28 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EED0Ev
pub fn stub_479d28() -> ! {
    todo!("0x479d28 RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::~BoundFuncDesc()")
}

// 0x479dfc — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_479dfc() -> ! {
    todo!("0x479dfc RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x479e30 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EEC2EMS2_FvS6_dEPKcSC_SC_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, double, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::BoundFuncDesc(void (RBX::DebrisService::*)(boost::shared_ptr<RBX::Instance>,double),char const*,char const*,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EEC2EMS2_FvS6_dEPKcSC_SC_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_479e30() -> ! {
    todo!("0x479e30 RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::BoundFuncDesc(void (RBX::DebrisService::*)(boost::shared_ptr<RBX::Instance>,double),char const*,char const*,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x47a050 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
pub fn stub_47a050() -> ! {
    todo!("0x47a050 RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x47a09c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EED0Ev
pub fn stub_47a09c() -> ! {
    todo!("0x47a09c RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::~BoundFuncDesc()")
}

// 0x47a1c8 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_47a1c8() -> ! {
    todo!("0x47a1c8 RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x47a2cc — __ZN3RBX10Reflection11Call2HelperINS_13DebrisServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEdES6_dvE4callEPS2_S8_RNS0_7VariantERKS6_RKd
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DebrisService,void (RBX::DebrisService::*)(boost::shared_ptr<RBX::Instance>,double),boost::shared_ptr<RBX::Instance>,double,void>::call(RBX::DebrisService*,void (RBX::DebrisService::*)(boost::shared_ptr<RBX::Instance>,double),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,double const&)")]
// was: __ZN3RBX10Reflection11Call2HelperINS_13DebrisServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEdES6_dvE4callEPS2_S8_RNS0_7VariantERKS6_RKd
pub fn stub_47a2cc() -> ! {
    todo!("0x47a2cc RBX::Reflection::Call2Helper<RBX::DebrisService,void (RBX::DebrisService::*)(boost::shared_ptr<RBX::Instance>,double),boost::shared_ptr<RBX::Instance>,double,void>::call(RBX::DebrisService*,void (RBX::DebrisService::*)(boost::shared_ptr<RBX::Instance>,double),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,double const&)")
}

// 0x47a3c0 — __ZN3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::PropDescriptor<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>(char const*,char const*,int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_47a3c0() -> ! {
    todo!("0x47a3c0 RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::PropDescriptor<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>(char const*,char const*,int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x47a4d4 — __ZN3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiED0Ev
pub fn stub_47a4d4() -> ! {
    todo!("0x47a4d4 RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::~PropDescriptor()")
}

// 0x47a500 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
pub fn stub_47a500() -> ! {
    todo!("0x47a500 RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::isReadOnly(void)const")
}

// 0x47a504 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
pub fn stub_47a504() -> ! {
    todo!("0x47a504 RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::isWriteOnly(void)const")
}

// 0x47a508 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_47a508() -> ! {
    todo!("0x47a508 RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x47a528 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
pub fn stub_47a528() -> ! {
    todo!("0x47a528 RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::GetSetImpl<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x47a54c — __GLOBAL__I_a_181
#[doc(alias = "global constructor keyed to_a_181")]
// was: __GLOBAL__I_a_181
pub fn stub_47a54c() -> ! {
    todo!("0x47a54c global constructor keyed to_a_181")
}

// 0x47a87c — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEC2Ev
pub fn stub_47a87c() -> ! {
    todo!("0x47a87c RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::EnumDesc(void)")
}

// 0x47ab28 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEEC2Ev
pub fn stub_47ab28() -> ! {
    todo!("0x47ab28 RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::EnumDesc(void)")
}

// 0x47ad04 — __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEC2Ev
pub fn stub_47ad04() -> ! {
    todo!("0x47ad04 RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::EnumDesc(void)")
}

// 0x47aee0 — __ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEEC2Ev
pub fn stub_47aee0() -> ! {
    todo!("0x47aee0 RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::EnumDesc(void)")
}

// 0x47b0b8 — __ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEEC1Ev
pub fn stub_47b0b8() -> ! {
    todo!("0x47b0b8 RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::EnumDesc(void)")
}

// 0x47b0bc — __ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEEC2Ev
pub fn stub_47b0bc() -> ! {
    todo!("0x47b0bc RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::EnumDesc(void)")
}

// 0x47b2f4 — __ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEEC2Ev
pub fn stub_47b2f4() -> ! {
    todo!("0x47b2f4 RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::EnumDesc(void)")
}

// 0x47b4cc — __ZNK3RBX13DebugSettings20getVertexShaderModelEv
// type: int __fastcall(RBX::DebugSettings *this)
#[doc(alias = "RBX::DebugSettings::getVertexShaderModel(void)const")]
// was: __ZNK3RBX13DebugSettings20getVertexShaderModelEv
pub fn stub_47b4cc() -> ! {
    todo!("0x47b4cc RBX::DebugSettings::getVertexShaderModel(void)const")
}

// 0x47b4d0 — __ZNK3RBX13DebugSettings19getPixelShaderModelEv
// type: int __fastcall(RBX::DebugSettings *this)
#[doc(alias = "RBX::DebugSettings::getPixelShaderModel(void)const")]
// was: __ZNK3RBX13DebugSettings19getPixelShaderModelEv
pub fn stub_47b4d0() -> ! {
    todo!("0x47b4d0 RBX::DebugSettings::getPixelShaderModel(void)const")
}

// 0x47b4d4 — __ZNK3RBX13DebugSettings11videoMemoryEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::videoMemory(void)const")]
// was: __ZNK3RBX13DebugSettings11videoMemoryEv
pub fn stub_47b4d4() -> ! {
    todo!("0x47b4d4 RBX::DebugSettings::videoMemory(void)const")
}

// 0x47b564 — __ZNK3RBX13DebugSettings8cpuSpeedEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::cpuSpeed(void)const")]
// was: __ZNK3RBX13DebugSettings8cpuSpeedEv
pub fn stub_47b564() -> ! {
    todo!("0x47b564 RBX::DebugSettings::cpuSpeed(void)const")
}

// 0x47b5f4 — __ZNK3RBX13DebugSettings8cpuCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::cpuCount(void)const")]
// was: __ZNK3RBX13DebugSettings8cpuCountEv
pub fn stub_47b5f4() -> ! {
    todo!("0x47b5f4 RBX::DebugSettings::cpuCount(void)const")
}

// 0x47b684 — __ZNK3RBX13DebugSettings12osPlatformIdEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::osPlatformId(void)const")]
// was: __ZNK3RBX13DebugSettings12osPlatformIdEv
pub fn stub_47b684() -> ! {
    todo!("0x47b684 RBX::DebugSettings::osPlatformId(void)const")
}

// 0x47b688 — __ZNK3RBX13DebugSettings10osPlatformEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::osPlatform(void)const")]
// was: __ZNK3RBX13DebugSettings10osPlatformEv
pub fn stub_47b688() -> ! {
    todo!("0x47b688 RBX::DebugSettings::osPlatform(void)const")
}

// 0x47b6a4 — __ZNK3RBX13DebugSettings5osVerEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::osVer(void)const")]
// was: __ZNK3RBX13DebugSettings5osVerEv
pub fn stub_47b6a4() -> ! {
    todo!("0x47b6a4 RBX::DebugSettings::osVer(void)const")
}

// 0x47b6b0 — __ZNK3RBX13DebugSettings9osIs64BitEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::osIs64Bit(void)const")]
// was: __ZNK3RBX13DebugSettings9osIs64BitEv
pub fn stub_47b6b0() -> ! {
    todo!("0x47b6b0 RBX::DebugSettings::osIs64Bit(void)const")
}

// 0x47b6bc — __ZNK3RBX13DebugSettings17systemProductNameEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::systemProductName(void)const")]
// was: __ZNK3RBX13DebugSettings17systemProductNameEv
pub fn stub_47b6bc() -> ! {
    todo!("0x47b6bc RBX::DebugSettings::systemProductName(void)const")
}

// 0x47b6e4 — __ZNK3RBX13DebugSettings3cpuEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::cpu(void)const")]
// was: __ZNK3RBX13DebugSettings3cpuEv
pub fn stub_47b6e4() -> ! {
    todo!("0x47b6e4 RBX::DebugSettings::cpu(void)const")
}

// 0x47b894 — __ZNK3RBX13DebugSettings4simdEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::simd(void)const")]
// was: __ZNK3RBX13DebugSettings4simdEv
pub fn stub_47b894() -> ! {
    todo!("0x47b894 RBX::DebugSettings::simd(void)const")
}

// 0x47b9a4 — __ZNK3RBX13DebugSettings19totalPhysicalMemoryEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::totalPhysicalMemory(void)const")]
// was: __ZNK3RBX13DebugSettings19totalPhysicalMemoryEv
pub fn stub_47b9a4() -> ! {
    todo!("0x47b9a4 RBX::DebugSettings::totalPhysicalMemory(void)const")
}

// 0x47ba34 — __ZNK3RBX13DebugSettings10resolutionEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::resolution(void)const")]
// was: __ZNK3RBX13DebugSettings10resolutionEv
pub fn stub_47ba34() -> ! {
    todo!("0x47ba34 RBX::DebugSettings::resolution(void)const")
}

// 0x47bbb4 — __ZNK3RBX13DebugSettings23availablePhysicalMemoryEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::availablePhysicalMemory(void)const")]
// was: __ZNK3RBX13DebugSettings23availablePhysicalMemoryEv
pub fn stub_47bbb4() -> ! {
    todo!("0x47bbb4 RBX::DebugSettings::availablePhysicalMemory(void)const")
}

// 0x47bc44 — __ZNK3RBX13DebugSettings14getElapsedTimeEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getElapsedTime(void)const")]
// was: __ZNK3RBX13DebugSettings14getElapsedTimeEv
pub fn stub_47bc44() -> ! {
    todo!("0x47bc44 RBX::DebugSettings::getElapsedTime(void)const")
}

// 0x47bc50 — __ZNK3RBX13DebugSettings12processCoresEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::processCores(void)const")]
// was: __ZNK3RBX13DebugSettings12processCoresEv
pub fn stub_47bc50() -> ! {
    todo!("0x47bc50 RBX::DebugSettings::processCores(void)const")
}

// 0x47bc8c — __ZNK3RBX13DebugSettings18totalProcessorTimeEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::totalProcessorTime(void)const")]
// was: __ZNK3RBX13DebugSettings18totalProcessorTimeEv
pub fn stub_47bc8c() -> ! {
    todo!("0x47bc8c RBX::DebugSettings::totalProcessorTime(void)const")
}

// 0x47bcb0 — __ZNK3RBX13DebugSettings13processorTimeEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::processorTime(void)const")]
// was: __ZNK3RBX13DebugSettings13processorTimeEv
pub fn stub_47bcb0() -> ! {
    todo!("0x47bcb0 RBX::DebugSettings::processorTime(void)const")
}

// 0x47bcb8 — __ZNK3RBX13DebugSettings12privateBytesEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::privateBytes(void)const")]
// was: __ZNK3RBX13DebugSettings12privateBytesEv
pub fn stub_47bcb8() -> ! {
    todo!("0x47bcb8 RBX::DebugSettings::privateBytes(void)const")
}

// 0x47bcdc — __ZNK3RBX13DebugSettings22privateWorkingSetBytesEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::privateWorkingSetBytes(void)const")]
// was: __ZNK3RBX13DebugSettings22privateWorkingSetBytesEv
pub fn stub_47bcdc() -> ! {
    todo!("0x47bcdc RBX::DebugSettings::privateWorkingSetBytes(void)const")
}

// 0x47bcfc — __ZNK3RBX13DebugSettings15GetVirtualBytesEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::GetVirtualBytes(void)const")]
// was: __ZNK3RBX13DebugSettings15GetVirtualBytesEv
pub fn stub_47bcfc() -> ! {
    todo!("0x47bcfc RBX::DebugSettings::GetVirtualBytes(void)const")
}

// 0x47bd1c — __ZNK3RBX13DebugSettings16GetPageFileBytesEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::GetPageFileBytes(void)const")]
// was: __ZNK3RBX13DebugSettings16GetPageFileBytesEv
pub fn stub_47bd1c() -> ! {
    todo!("0x47bd1c RBX::DebugSettings::GetPageFileBytes(void)const")
}

// 0x47bd24 — __ZNK3RBX13DebugSettings22GetPageFaultsPerSecondEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::GetPageFaultsPerSecond(void)const")]
// was: __ZNK3RBX13DebugSettings22GetPageFaultsPerSecondEv
pub fn stub_47bd24() -> ! {
    todo!("0x47bd24 RBX::DebugSettings::GetPageFaultsPerSecond(void)const")
}

// 0x47bd50 — __ZNK3RBX13DebugSettings14getPlayerCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getPlayerCount(void)const")]
// was: __ZNK3RBX13DebugSettings14getPlayerCountEv
pub fn stub_47bd50() -> ! {
    todo!("0x47bd50 RBX::DebugSettings::getPlayerCount(void)const")
}

// 0x47bd60 — __ZNK3RBX13DebugSettings17getDataModelCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getDataModelCount(void)const")]
// was: __ZNK3RBX13DebugSettings17getDataModelCountEv
pub fn stub_47bd60() -> ! {
    todo!("0x47bd60 RBX::DebugSettings::getDataModelCount(void)const")
}

// 0x47bd70 — __ZNK3RBX13DebugSettings18getCdnSuccessCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getCdnSuccessCount(void)const")]
// was: __ZNK3RBX13DebugSettings18getCdnSuccessCountEv
pub fn stub_47bd70() -> ! {
    todo!("0x47bd70 RBX::DebugSettings::getCdnSuccessCount(void)const")
}

// 0x47bd80 — __ZNK3RBX13DebugSettings18getCdnFailureCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getCdnFailureCount(void)const")]
// was: __ZNK3RBX13DebugSettings18getCdnFailureCountEv
pub fn stub_47bd80() -> ! {
    todo!("0x47bd80 RBX::DebugSettings::getCdnFailureCount(void)const")
}

// 0x47bd90 — __ZNK3RBX13DebugSettings27getAlternateCdnSuccessCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getAlternateCdnSuccessCount(void)const")]
// was: __ZNK3RBX13DebugSettings27getAlternateCdnSuccessCountEv
pub fn stub_47bd90() -> ! {
    todo!("0x47bd90 RBX::DebugSettings::getAlternateCdnSuccessCount(void)const")
}

// 0x47bda0 — __ZNK3RBX13DebugSettings27getAlternateCdnFailureCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getAlternateCdnFailureCount(void)const")]
// was: __ZNK3RBX13DebugSettings27getAlternateCdnFailureCountEv
pub fn stub_47bda0() -> ! {
    todo!("0x47bda0 RBX::DebugSettings::getAlternateCdnFailureCount(void)const")
}

// 0x47bdb0 — __ZNK3RBX13DebugSettings20getBlockMeshMapCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getBlockMeshMapCount(void)const")]
// was: __ZNK3RBX13DebugSettings20getBlockMeshMapCountEv
pub fn stub_47bdb0() -> ! {
    todo!("0x47bdb0 RBX::DebugSettings::getBlockMeshMapCount(void)const")
}

// 0x47bdb4 — __ZNK3RBX13DebugSettings25getLastCdnFailureTimeSpanEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getLastCdnFailureTimeSpan(void)const")]
// was: __ZNK3RBX13DebugSettings25getLastCdnFailureTimeSpanEv
pub fn stub_47bdb4() -> ! {
    todo!("0x47bdb4 RBX::DebugSettings::getLastCdnFailureTimeSpan(void)const")
}

// 0x47bdcc — __ZNK3RBX13DebugSettings21getRobloxSuccessCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getRobloxSuccessCount(void)const")]
// was: __ZNK3RBX13DebugSettings21getRobloxSuccessCountEv
pub fn stub_47bdcc() -> ! {
    todo!("0x47bdcc RBX::DebugSettings::getRobloxSuccessCount(void)const")
}

// 0x47bddc — __ZNK3RBX13DebugSettings20getRobloxFalureCountEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getRobloxFalureCount(void)const")]
// was: __ZNK3RBX13DebugSettings20getRobloxFalureCountEv
pub fn stub_47bddc() -> ! {
    todo!("0x47bddc RBX::DebugSettings::getRobloxFalureCount(void)const")
}

// 0x47bdf0 — __ZNK3RBX13DebugSettings17getRobloxResponceEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getRobloxResponce(void)const")]
// was: __ZNK3RBX13DebugSettings17getRobloxResponceEv
pub fn stub_47bdf0() -> ! {
    todo!("0x47bdf0 RBX::DebugSettings::getRobloxResponce(void)const")
}

// 0x47be48 — __ZNK3RBX13DebugSettings13getCdnRespoceEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::getCdnRespoce(void)const")]
// was: __ZNK3RBX13DebugSettings13getCdnRespoceEv
pub fn stub_47be48() -> ! {
    todo!("0x47be48 RBX::DebugSettings::getCdnRespoce(void)const")
}

// 0x47bea0 — __ZN3RBX13DebugSettings21resetCdnFailureCountsEv
// type: _DWORD __fastcall(RBX::DebugSettings *__hidden this)
#[doc(alias = "RBX::DebugSettings::resetCdnFailureCounts(void)")]
// was: __ZN3RBX13DebugSettings21resetCdnFailureCountsEv
pub fn stub_47bea0() -> ! {
    todo!("0x47bea0 RBX::DebugSettings::resetCdnFailureCounts(void)")
}

// 0x47c2a8 — __ZN3RBX21TaskSchedulerSettings11addDummyJobEbd
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this, bool, double)
#[doc(alias = "RBX::TaskSchedulerSettings::addDummyJob(bool,double)")]
// was: __ZN3RBX21TaskSchedulerSettings11addDummyJobEbd
pub fn stub_47c2a8() -> ! {
    todo!("0x47c2a8 RBX::TaskSchedulerSettings::addDummyJob(bool,double)")
}

// 0x47c3f8 — __ZN3RBX13DebugSettings17setErrorReportingENS0_14ErrorReportingE
#[doc(alias = "RBX::DebugSettings::setErrorReporting(RBX::DebugSettings::ErrorReporting)")]
// was: __ZN3RBX13DebugSettings17setErrorReportingENS0_14ErrorReportingE
pub fn stub_47c3f8() -> ! {
    todo!("0x47c3f8 RBX::DebugSettings::setErrorReporting(RBX::DebugSettings::ErrorReporting)")
}

// 0x47c414 — __ZNK3RBX21TaskSchedulerSettings19getThreadPoolConfigEv
// type: _DWORD __fastcall(RBX::TaskSchedulerSettings *__hidden this)
#[doc(alias = "RBX::TaskSchedulerSettings::getThreadPoolConfig(void)const")]
// was: __ZNK3RBX21TaskSchedulerSettings19getThreadPoolConfigEv
pub fn stub_47c414() -> ! {
    todo!("0x47c414 RBX::TaskSchedulerSettings::getThreadPoolConfig(void)const")
}