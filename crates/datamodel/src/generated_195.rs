// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel|RBX::Workspace (10215 filtered, 0 remaining) — EA-sorted asc next 100 DM gaps not yet in crates/datamodel/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x4a27c..0x4e998 | dm distinct before 20066, after 20166, dm missing 65479->65379
// Shard: 195 EA-sorted asc next 100 DM gaps after 194 (filtered exhausted, global filler 0x4a27c..0x4e998)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; `'` stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4a27c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPKN3RBX10Reflection18PropertyDescriptorEENS3_5list3INS3_5valueIS6_EENSG_IS7_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Reflection::PropertyDescriptor const>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::PropertyDescriptor const)")]
pub fn stub_4a27c() -> ! {
    todo!("0x4a27c boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::Reflection::PropertyDescriptor const*),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Reflection::PropertyDescriptor const>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::PropertyDescriptor const)")
}

// 0x4a28c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
pub fn stub_4a28c() -> ! {
    todo!("0x4a28c rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")
}

// 0x4a49c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot*)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot*)")]
pub fn stub_4a49c() -> ! {
    todo!("0x4a49c boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot*)")
}

// 0x4a540 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::safe_static_init_mutex(void)")]
pub fn stub_4a540() -> ! {
    todo!("0x4a540 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::safe_static_init_mutex(void)")
}

// 0x4a544 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>*>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>*)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*)")]
pub fn stub_4a544() -> ! {
    todo!("0x4a544 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*)")
}

// 0x4a640 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")]
pub fn stub_4a640() -> ! {
    todo!("0x4a640 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")
}

// 0x4a714 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")]
pub fn stub_4a714() -> ! {
    todo!("0x4a714 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")
}

// 0x4a7ec — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot10disconnectEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::disconnect(void)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::disconnect(void)")]
pub fn stub_4a7ec() -> ! {
    todo!("0x4a7ec rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::disconnect(void)")
}

// 0x4a8fc — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot9connectedEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::connected(void)const
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::connected(void)const")]
pub fn stub_4a8fc() -> ! {
    todo!("0x4a8fc rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::connected(void)const")
}

// 0x4a908 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
pub fn stub_4a908() -> ! {
    todo!("0x4a908 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")
}

// 0x4a9dc — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// was: `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
pub fn stub_4a9dc() -> ! {
    todo!("0x4a9dc `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")
}

// 0x4a9e4 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_
// type: int(void)
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::operator()(boost::shared_ptr<RBX::TextBox>)const
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::operator()(rbx_core::SharedPtr<RBX::TextBox>)const")]
pub fn stub_4a9e4() -> ! {
    todo!("0x4a9e4 boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::operator()(rbx_core::SharedPtr<RBX::TextBox>)const")
}

// 0x4aaf4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
pub fn stub_4aaf4() -> ! {
    todo!("0x4aaf4 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")
}

// 0x4abe4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)")]
pub fn stub_4abe4() -> ! {
    todo!("0x4abe4 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)")
}

// 0x4abe8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_4abe8() -> ! {
    todo!("0x4abe8 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)")
}

// 0x4acd8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")]
pub fn stub_4acd8() -> ! {
    todo!("0x4acd8 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")
}

// 0x4adac — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")]
pub fn stub_4adac() -> ! {
    todo!("0x4adac rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")
}

// 0x4ae84 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::~slot()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")]
pub fn stub_4ae84() -> ! {
    todo!("0x4ae84 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")
}

// 0x4af30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::~slot()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")]
pub fn stub_4af30() -> ! {
    todo!("0x4af30 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")
}

// 0x4afe0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_
// type: int(void)
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::TextBox>> const&)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>> const&)")]
pub fn stub_4afe0() -> ! {
    todo!("0x4afe0 boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>> const&)")
}

// 0x4b010 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_4b010() -> ! {
    todo!("0x4b010 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")
}

// 0x4b070 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// type: int __fastcall(int, int)
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::TextBox>::invoke(boost::detail::function::function_buffer &,RBX::TextBox)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::TextBox>::invoke(boost::detail::function::function_buffer &,RBX::TextBox)")]
pub fn stub_4b070() -> ! {
    todo!("0x4b070 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::TextBox>::invoke(boost::detail::function::function_buffer &,RBX::TextBox)")
}

// 0x4b088 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX7TextBoxEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int, int, sp_counted_base **), const shared_count **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)")]
pub fn stub_4b088() -> ! {
    todo!("0x4b088 void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)")
}

// 0x4bfdc — __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::clear(void)")]
pub fn stub_4bfdc() -> ! {
    todo!("0x4bfdc boost::function1<void,RBX::Reflection::PropertyDescriptor const*>::clear(void)")
}

// 0x4c008 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv
// type: int __fastcall(int *)
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::clear(void)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::clear(void)")]
pub fn stub_4c008() -> ! {
    todo!("0x4c008 boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::clear(void)")
}

// 0x4c034 — __GLOBAL__I_a_18
#[doc(alias = "global constructor keyed to_a_18")]
pub fn stub_4c034() -> ! {
    todo!("0x4c034 `global constructor keyed to_a_18")
}

// 0x4c3f4 — -[GameInputViewController dealloc]
// type: void __cdecl(GameInputViewController *self, SEL)
#[doc(alias = "-[GameInputViewController dealloc]")]
pub fn stub_4c3f4() -> ! {
    todo!("0x4c3f4 -[GameInputViewController dealloc]")
}

// 0x4c440 — -[GameInputViewController viewDidLoad]
// type: void __cdecl(GameInputViewController *self, SEL)
#[doc(alias = "-[GameInputViewController viewDidLoad]")]
pub fn stub_4c440() -> ! {
    todo!("0x4c440 -[GameInputViewController viewDidLoad]")
}

// 0x4c46c — -[GameInputViewController viewDidUnload]
// type: void __cdecl(GameInputViewController *self, SEL)
#[doc(alias = "-[GameInputViewController viewDidUnload]")]
pub fn stub_4c46c() -> ! {
    todo!("0x4c46c -[GameInputViewController viewDidUnload]")
}

// 0x4c498 — __GLOBAL__I_a_19
#[doc(alias = "global constructor keyed to_a_19")]
pub fn stub_4c498() -> ! {
    todo!("0x4c498 `global constructor keyed to_a_19")
}

// 0x4c71c — -[GameKeyboard init]
// type: GameKeyboard *__cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard init]")]
pub fn stub_4c71c() -> ! {
    todo!("0x4c71c -[GameKeyboard init]")
}

// 0x4ca18 — -[GameKeyboard dealloc]
// type: void __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard dealloc]")]
pub fn stub_4ca18() -> ! {
    todo!("0x4ca18 -[GameKeyboard dealloc]")
}

// 0x4ca64 — -[GameKeyboard hideKeyboard]
// type: void __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard hideKeyboard]")]
pub fn stub_4ca64() -> ! {
    todo!("0x4ca64 -[GameKeyboard hideKeyboard]")
}

// 0x4cb80 — -[GameKeyboard keyboardWillHide:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard keyboardWillHide:]")]
pub fn stub_4cb80() -> ! {
    todo!("0x4cb80 -[GameKeyboard keyboardWillHide:]")
}

// 0x4cbbc — -[GameKeyboard keyboardWillChangeFrame:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard keyboardWillChangeFrame:]")]
pub fn stub_4cbbc() -> ! {
    todo!("0x4cbbc -[GameKeyboard keyboardWillChangeFrame:]")
}

// 0x4cbc0 — -[GameKeyboard setDefaultString:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard setDefaultString:]")]
pub fn stub_4cbc0() -> ! {
    todo!("0x4cbc0 -[GameKeyboard setDefaultString:]")
}

// 0x4cbe0 — -[GameKeyboard setParentView:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard setParentView:]")]
pub fn stub_4cbe0() -> ! {
    todo!("0x4cbe0 -[GameKeyboard setParentView:]")
}

// 0x4cbf8 — -[GameKeyboard showKeyboard:]
// type: bool __cdecl(GameKeyboard *self, SEL, const char *)
#[doc(alias = "-[GameKeyboard showKeyboard:]")]
pub fn stub_4cbf8() -> ! {
    todo!("0x4cbf8 -[GameKeyboard showKeyboard:]")
}

// 0x4cc78 — ___29-[GameKeyboard showKeyboard:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___29-[GameKeyboard showKeyboard:]_block_invoke")]
pub fn stub_4cc78() -> ! {
    todo!("0x4cc78 ___29-[GameKeyboard showKeyboard:]_block_invoke")
}

// 0x4ce30 — ___copy_helper_block__9
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__9")]
pub fn stub_4ce30() -> ! {
    todo!("0x4ce30 ___copy_helper_block__9")
}

// 0x4ce3c — ___destroy_helper_block__9
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__9")]
pub fn stub_4ce3c() -> ! {
    todo!("0x4ce3c ___destroy_helper_block__9")
}

// 0x4ce44 — -[GameKeyboard showKeyboardWithTextBox:]
// type: bool __cdecl(GameKeyboard *self, SEL, shared_ptr<RBX::TextBox>)
#[doc(alias = "-[GameKeyboard showKeyboardWithTextBox:]")]
pub fn stub_4ce44() -> ! {
    todo!("0x4ce44 -[GameKeyboard showKeyboardWithTextBox:]")
}

// 0x4cfbc — -[GameKeyboard getText]
// type: id __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard getText]")]
pub fn stub_4cfbc() -> ! {
    todo!("0x4cfbc -[GameKeyboard getText]")
}

// 0x4cfdc — -[GameKeyboard textFieldShouldReturn:]
// type: char __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard textFieldShouldReturn:]")]
pub fn stub_4cfdc() -> ! {
    todo!("0x4cfdc -[GameKeyboard textFieldShouldReturn:]")
}

// 0x4d07c — ___38-[GameKeyboard textFieldShouldReturn:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___38-[GameKeyboard textFieldShouldReturn:]_block_invoke")]
pub fn stub_4d07c() -> ! {
    todo!("0x4d07c ___38-[GameKeyboard textFieldShouldReturn:]_block_invoke")
}

// 0x4d090 — ___copy_helper_block_82
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_82")]
pub fn stub_4d090() -> ! {
    todo!("0x4d090 ___copy_helper_block_82")
}

// 0x4d09c — ___destroy_helper_block_83
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_83")]
pub fn stub_4d09c() -> ! {
    todo!("0x4d09c ___destroy_helper_block_83")
}

// 0x4d0a4 — -[GameKeyboard textFieldDidEndEditing:]
// type: void __cdecl(GameKeyboard *self, SEL, id)
#[doc(alias = "-[GameKeyboard textFieldDidEndEditing:]")]
pub fn stub_4d0a4() -> ! {
    todo!("0x4d0a4 -[GameKeyboard textFieldDidEndEditing:]")
}

// 0x4d15c — ___39-[GameKeyboard textFieldDidEndEditing:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___39-[GameKeyboard textFieldDidEndEditing:]_block_invoke")]
pub fn stub_4d15c() -> ! {
    todo!("0x4d15c ___39-[GameKeyboard textFieldDidEndEditing:]_block_invoke")
}

// 0x4d170 — ___copy_helper_block_87
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_87")]
pub fn stub_4d170() -> ! {
    todo!("0x4d170 ___copy_helper_block_87")
}

// 0x4d17c — ___destroy_helper_block_88
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_88")]
pub fn stub_4d17c() -> ! {
    todo!("0x4d17c ___destroy_helper_block_88")
}

// 0x4d184 — -[GameKeyboard .cxx_destruct]
// type: void __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard .cxx_destruct]")]
pub fn stub_4d184() -> ! {
    todo!("0x4d184 -[GameKeyboard .cxx_destruct]")
}

// 0x4d220 — -[GameKeyboard .cxx_construct]
// type: id __cdecl(GameKeyboard *self, SEL)
#[doc(alias = "-[GameKeyboard .cxx_construct]")]
pub fn stub_4d220() -> ! {
    todo!("0x4d220 -[GameKeyboard .cxx_construct]")
}

// 0x4d238 — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(_DWORD *, __int64 *)
// was: boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox>&&)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox>&&)")]
pub fn stub_4d238() -> ! {
    todo!("0x4d238 rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox>&&)")
}

// 0x4d2dc — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, const shared_count *)
// was: boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox> const&)")]
pub fn stub_4d2dc() -> ! {
    todo!("0x4d2dc rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox> const&)")
}

// 0x4d398 — __GLOBAL__I_a_20
#[doc(alias = "global constructor keyed to_a_20")]
pub fn stub_4d398() -> ! {
    todo!("0x4d398 `global constructor keyed to_a_20")
}

// 0x4d5ac — -[GameView initWithFrame:]
// type: GameView *__cdecl(GameView *self, SEL, CGRect)
#[doc(alias = "-[GameView initWithFrame:]")]
pub fn stub_4d5ac() -> ! {
    todo!("0x4d5ac -[GameView initWithFrame:]")
}

// 0x4d5e4 — -[GameView layoutSubviews]
// type: void __cdecl(GameView *self, SEL)
#[doc(alias = "-[GameView layoutSubviews]")]
pub fn stub_4d5e4() -> ! {
    todo!("0x4d5e4 -[GameView layoutSubviews]")
}

// 0x4d6d4 — __GLOBAL__I_a_21
// type: int()
#[doc(alias = "global constructor keyed to_a_21")]
pub fn stub_4d6d4() -> ! {
    todo!("0x4d6d4 `global constructor keyed to_a_21")
}

// 0x4d70c — -[GameViewController initWithNibName:bundle:]
// type: GameViewController *__cdecl(GameViewController *self, SEL, id, id)
#[doc(alias = "-[GameViewController initWithNibName:bundle:]")]
pub fn stub_4d70c() -> ! {
    todo!("0x4d70c -[GameViewController initWithNibName:bundle:]")
}

// 0x4d8cc — -[GameViewController dealloc]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController dealloc]")]
pub fn stub_4d8cc() -> ! {
    todo!("0x4d8cc -[GameViewController dealloc]")
}

// 0x4d978 — -[GameViewController viewWillAppear:]
// type: void __cdecl(GameViewController *self, SEL, char)
#[doc(alias = "-[GameViewController viewWillAppear:]")]
pub fn stub_4d978() -> ! {
    todo!("0x4d978 -[GameViewController viewWillAppear:]")
}

// 0x4d9d4 — -[GameViewController viewDidAppear:]
// type: void __cdecl(GameViewController *self, SEL, char)
#[doc(alias = "-[GameViewController viewDidAppear:]")]
pub fn stub_4d9d4() -> ! {
    todo!("0x4d9d4 -[GameViewController viewDidAppear:]")
}

// 0x4da00 — -[GameViewController viewDidLoad]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController viewDidLoad]")]
pub fn stub_4da00() -> ! {
    todo!("0x4da00 -[GameViewController viewDidLoad]")
}

// 0x4dab8 — -[GameViewController didReceiveMemoryWarning]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController didReceiveMemoryWarning]")]
pub fn stub_4dab8() -> ! {
    todo!("0x4dab8 -[GameViewController didReceiveMemoryWarning]")
}

// 0x4dae4 — -[GameViewController resizeGameView]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController resizeGameView]")]
pub fn stub_4dae4() -> ! {
    todo!("0x4dae4 -[GameViewController resizeGameView]")
}

// 0x4db04 — -[GameViewController shouldAutorotate]
// type: char __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController shouldAutorotate]")]
pub fn stub_4db04() -> ! {
    todo!("0x4db04 -[GameViewController shouldAutorotate]")
}

// 0x4db08 — -[GameViewController supportedInterfaceOrientations]
// type: unsigned int __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController supportedInterfaceOrientations]")]
pub fn stub_4db08() -> ! {
    todo!("0x4db08 -[GameViewController supportedInterfaceOrientations]")
}

// 0x4db0c — -[GameViewController shouldAutorotateToInterfaceOrientation:]
// type: char __cdecl(GameViewController *self, SEL, int)
#[doc(alias = "-[GameViewController shouldAutorotateToInterfaceOrientation:]")]
pub fn stub_4db0c() -> ! {
    todo!("0x4db0c -[GameViewController shouldAutorotateToInterfaceOrientation:]")
}

// 0x4db20 — -[GameViewController getControlView]
// type: id __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController getControlView]")]
pub fn stub_4db20() -> ! {
    todo!("0x4db20 -[GameViewController getControlView]")
}

// 0x4db9c — -[GameViewController webView:shouldStartLoadWithRequest:navigationType:]
// type: char __cdecl(GameViewController *self, SEL, id, id, int)
#[doc(alias = "-[GameViewController webView:shouldStartLoadWithRequest:navigationType:]")]
pub fn stub_4db9c() -> ! {
    todo!("0x4db9c -[GameViewController webView:shouldStartLoadWithRequest:navigationType:]")
}

// 0x4dc08 — -[GameViewController closeUrlWindow:]
// type: void __cdecl(GameViewController *self, SEL, id)
#[doc(alias = "-[GameViewController closeUrlWindow:]")]
pub fn stub_4dc08() -> ! {
    todo!("0x4dc08 -[GameViewController closeUrlWindow:]")
}

// 0x4de58 — ___37-[GameViewController closeUrlWindow:]_block_invoke
// type: id __fastcall(_DWORD *)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke")]
pub fn stub_4de58() -> ! {
    todo!("0x4de58 ___37-[GameViewController closeUrlWindow:]_block_invoke")
}

// 0x4df1c — ___37-[GameViewController closeUrlWindow:]_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke_2")]
pub fn stub_4df1c() -> ! {
    todo!("0x4df1c ___37-[GameViewController closeUrlWindow:]_block_invoke_2")
}

// 0x4dfd8 — ___copy_helper_block__10
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__10")]
pub fn stub_4dfd8() -> ! {
    todo!("0x4dfd8 ___copy_helper_block__10")
}

// 0x4dfe4 — ___destroy_helper_block__10
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__10")]
pub fn stub_4dfe4() -> ! {
    todo!("0x4dfe4 ___destroy_helper_block__10")
}

// 0x4dfec — ___37-[GameViewController closeUrlWindow:]_block_invoke93
// type: id __fastcall(int)
#[doc(alias = "___37-[GameViewController closeUrlWindow:]_block_invoke93")]
pub fn stub_4dfec() -> ! {
    todo!("0x4dfec ___37-[GameViewController closeUrlWindow:]_block_invoke93")
}

// 0x4e01c — ___copy_helper_block_94
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_94")]
pub fn stub_4e01c() -> ! {
    todo!("0x4e01c ___copy_helper_block_94")
}

// 0x4e028 — ___destroy_helper_block_95
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_95")]
pub fn stub_4e028() -> ! {
    todo!("0x4e028 ___destroy_helper_block_95")
}

// 0x4e030 — ___copy_helper_block_100
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_100")]
pub fn stub_4e030() -> ! {
    todo!("0x4e030 ___copy_helper_block_100")
}

// 0x4e054 — ___destroy_helper_block_101
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_101")]
pub fn stub_4e054() -> ! {
    todo!("0x4e054 ___destroy_helper_block_101")
}

// 0x4e070 — -[GameViewController closeUrlWindow]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController closeUrlWindow]")]
pub fn stub_4e070() -> ! {
    todo!("0x4e070 -[GameViewController closeUrlWindow]")
}

// 0x4e084 — -[GameViewController openUrlWindow:]
// type: void __cdecl(GameViewController *self, SEL, basic_string<char, std::char_traits<char>, std::allocator<char> >)
#[doc(alias = "-[GameViewController openUrlWindow:]")]
pub fn stub_4e084() -> ! {
    todo!("0x4e084 -[GameViewController openUrlWindow:]")
}

// 0x4e2ac — ___36-[GameViewController openUrlWindow:]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke")]
pub fn stub_4e2ac() -> ! {
    todo!("0x4e2ac ___36-[GameViewController openUrlWindow:]_block_invoke")
}

// 0x4e4c8 — ___copy_helper_block_133
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_133")]
pub fn stub_4e4c8() -> ! {
    todo!("0x4e4c8 ___copy_helper_block_133")
}

// 0x4e4d4 — ___destroy_helper_block_134
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_134")]
pub fn stub_4e4d4() -> ! {
    todo!("0x4e4d4 ___destroy_helper_block_134")
}

// 0x4e4dc — ___36-[GameViewController openUrlWindow:]_block_invoke136
// type: id __fastcall(int)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke136")]
pub fn stub_4e4dc() -> ! {
    todo!("0x4e4dc ___36-[GameViewController openUrlWindow:]_block_invoke136")
}

// 0x4e5fc — ___36-[GameViewController openUrlWindow:]_block_invoke_2
// type: id __fastcall(_DWORD *)
#[doc(alias = "___36-[GameViewController openUrlWindow:]_block_invoke_2")]
pub fn stub_4e5fc() -> ! {
    todo!("0x4e5fc ___36-[GameViewController openUrlWindow:]_block_invoke_2")
}

// 0x4e6dc — ___copy_helper_block_148
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_148")]
pub fn stub_4e6dc() -> ! {
    todo!("0x4e6dc ___copy_helper_block_148")
}

// 0x4e6e8 — ___destroy_helper_block_149
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_149")]
pub fn stub_4e6e8() -> ! {
    todo!("0x4e6e8 ___destroy_helper_block_149")
}

// 0x4e6f0 — ___copy_helper_block_153
// type: int __fastcall(int, int)
#[doc(alias = "___copy_helper_block_153")]
pub fn stub_4e6f0() -> ! {
    todo!("0x4e6f0 ___copy_helper_block_153")
}

// 0x4e714 — ___destroy_helper_block_154
// type: int __fastcall(int)
#[doc(alias = "___destroy_helper_block_154")]
pub fn stub_4e714() -> ! {
    todo!("0x4e714 ___destroy_helper_block_154")
}

// 0x4e730 — -[GameViewController handlePromptLoginSignal]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController handlePromptLoginSignal]")]
pub fn stub_4e730() -> ! {
    todo!("0x4e730 -[GameViewController handlePromptLoginSignal]")
}

// 0x4e780 — ___45-[GameViewController handlePromptLoginSignal]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___45-[GameViewController handlePromptLoginSignal]_block_invoke")]
pub fn stub_4e780() -> ! {
    todo!("0x4e780 ___45-[GameViewController handlePromptLoginSignal]_block_invoke")
}

// 0x4e854 — ___copy_helper_block_174
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_174")]
pub fn stub_4e854() -> ! {
    todo!("0x4e854 ___copy_helper_block_174")
}

// 0x4e860 — ___destroy_helper_block_175
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_175")]
pub fn stub_4e860() -> ! {
    todo!("0x4e860 ___destroy_helper_block_175")
}

// 0x4e868 — -[GameViewController handlePromptSignupSignal]
// type: void __cdecl(GameViewController *self, SEL)
#[doc(alias = "-[GameViewController handlePromptSignupSignal]")]
pub fn stub_4e868() -> ! {
    todo!("0x4e868 -[GameViewController handlePromptSignupSignal]")
}

// 0x4e8b8 — ___46-[GameViewController handlePromptSignupSignal]_block_invoke
// type: id __fastcall(int)
#[doc(alias = "___46-[GameViewController handlePromptSignupSignal]_block_invoke")]
pub fn stub_4e8b8() -> ! {
    todo!("0x4e8b8 ___46-[GameViewController handlePromptSignupSignal]_block_invoke")
}

// 0x4e98c — ___copy_helper_block_179
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_179")]
pub fn stub_4e98c() -> ! {
    todo!("0x4e98c ___copy_helper_block_179")
}

// 0x4e998 — ___destroy_helper_block_180
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_180")]
pub fn stub_4e998() -> ! {
    todo!("0x4e998 ___destroy_helper_block_180")
}
