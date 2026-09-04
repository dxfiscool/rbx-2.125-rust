//! core shard GY — 100 core stubs EA-sorted, 0x46094..0x1c4b48 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after GX 0x45fa4).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after GX 0x45fa4 (0x46094..0x1c4b48, 18121->18221 covered, 3697 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
// 0x46094 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
pub fn stub_0x46094() {
    // IDA 0x46094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot,boost::function<void ()(bool,void *,RBX::UIEvent)>,3,void ()(bool,void *,RBX::UIEvent)>::~callable()")]
// 0x46168 — __ZN3rbx8callableINS_7signals6signalIFvbPvN3RBX7UIEventEEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
pub fn stub_0x46168() {
    // IDA 0x46168: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
// 0x46240 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD1Ev
pub fn stub_0x46240() {
    // IDA 0x46240: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot::~slot()")]
// 0x462ec — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotD0Ev
pub fn stub_0x462ec() {
    // IDA 0x462ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::assign_to_own(boost::function3<void,bool,void *,RBX::UIEvent> const&)")]
// 0x4639c — __ZN5boost9function3IvbPvN3RBX7UIEventEE13assign_to_ownERKS4_
pub fn stub_0x4639c() {
    // IDA 0x4639c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>>&,boost::detail::function::functor_manager_operation_type)")]
// 0x463cc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
pub fn stub_0x463cc() {
    // IDA 0x463cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,bool,void *,RBX::UIEvent),boost::_bi::list5<boost::_bi::value<objc_object *>,boost::_bi::list5<objc_selector>,boost::arg<1>,boost::_bi::list5<objc_selector><2>,boost::_bi::list5<objc_selector><3>>>,void,bool,objc_selector *,RBX>::invoke(boost::detail::function::function_buffer &,bool,objc_selector *,RBX)")]
// 0x4642c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorbPvN3RBX7UIEventEENS3_5list5INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEvbS8_SA_E6invokeERNS1_15function_bufferEbS8_SA_
pub fn stub_0x4642c() {
    // IDA 0x4642c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,bool,void *,RBX::UIEvent>::clear(void)")]
// 0x46464 — __ZN5boost9function3IvbPvN3RBX7UIEventEE5clearEv
pub fn stub_0x46464() {
    // IDA 0x46464: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&)")]
// 0x49f64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&)
pub fn stub_0x49f64() {
    // IDA 0x49f64: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
// 0x4a28c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6insertEPNS8_4slotE
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)
pub fn stub_0x4a28c() {
    // IDA 0x4a28c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot*)")]
// 0x4a49c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot*)
pub fn stub_0x4a49c() {
    // IDA 0x4a49c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::safe_static_init_mutex(void)")]
// 0x4a540 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::safe_static_init_mutex(void)
pub fn stub_0x4a540() {
    // IDA 0x4a540: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>*)")]
// 0x4a544 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_EC2IPS9_EERKSC_T_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>*>(boost::function<void ()(boost::shared_ptr<RBX::TextBox>)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>*)
pub fn stub_0x4a544() {
    // IDA 0x4a544: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")]
// 0x4a640 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>::~callable_slot()
pub fn stub_0x4a640() {
    // IDA 0x4a640: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>>::~callable_slot()")]
// 0x4a714 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>>::~callable_slot()
pub fn stub_0x4a714() {
    // IDA 0x4a714: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::disconnect(void)")]
// 0x4a7ec — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot10disconnectEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::disconnect(void)
pub fn stub_0x4a7ec() {
    // IDA 0x4a7ec: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::connected(void)const")]
// 0x4a8fc — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot9connectedEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::connected(void)const
pub fn stub_0x4a8fc() {
    // IDA 0x4a8fc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
// 0x4a908 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)
pub fn stub_0x4a908() {
    // IDA 0x4a908: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::call(rbx_core::SharedPtr<RBX::TextBox>)")]
// 0x4a9dc — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::call(boost::shared_ptr<RBX::TextBox>)
pub fn stub_0x4a9dc() {
    // IDA 0x4a9dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::operator()(rbx_core::SharedPtr<RBX::TextBox>)const")]
// 0x4a9e4 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEEclES4_
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::operator()(boost::shared_ptr<RBX::TextBox>)const
pub fn stub_0x4a9e4() {
    // IDA 0x4a9e4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot *)")]
// 0x4aaf4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE6removeEPNS8_4slotE
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot *)
pub fn stub_0x4aaf4() {
    // IDA 0x4aaf4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)")]
// 0x4abe4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot22safe_static_init_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_init_mutex(void)
pub fn stub_0x4abe4() {
    // IDA 0x4abe4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)")]
// 0x4abe8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slot24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::safe_static_do_get_mutex(void)
pub fn stub_0x4abe8() {
    // IDA 0x4abe8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")]
// 0x4acd8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::~callable()
pub fn stub_0x4acd8() {
    // IDA 0x4acd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::TextBox>)>,1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::~callable()")]
// 0x4adac — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::TextBox>)>,1,void ()(boost::shared_ptr<RBX::TextBox>)>::~callable()
pub fn stub_0x4adac() {
    // IDA 0x4adac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")]
// 0x4ae84 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::~slot()
pub fn stub_0x4ae84() {
    // IDA 0x4ae84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot::~slot()")]
// 0x4af30 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4slotD0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot::~slot()
pub fn stub_0x4af30() {
    // IDA 0x4af30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>> const&)")]
// 0x4afe0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE13assign_to_ownERKS5_
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::TextBox>> const&)
pub fn stub_0x4afe0() {
    // IDA 0x4afe0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
// 0x4b010 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x4b010() {
    // IDA 0x4b010: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::TextBox>::invoke(boost::detail::function::function_buffer &,RBX::TextBox)")]
// 0x4b070 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX7TextBoxEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::TextBox>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::TextBox>::invoke(boost::detail::function::function_buffer &,RBX::TextBox)
pub fn stub_0x4b070() {
    // IDA 0x4b070: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)")]
// 0x4b088 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX7TextBoxEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>),boost::_bi::list1<RBX::TextBox&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::TextBox>) &,boost::_bi::list1<RBX::TextBox&> &,int)
pub fn stub_0x4b088() {
    // IDA 0x4b088: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::TextBox>>::clear(void)")]
// 0x4c008 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX7TextBoxEEEE5clearEv
// was: boost::function1<void,boost::shared_ptr<RBX::TextBox>>::clear(void)
pub fn stub_0x4c008() {
    // IDA 0x4c008: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox>&&)")]
// 0x4d238 — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSEOS3_
// was: boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox>&&)
pub fn stub_0x4d238() {
    // IDA 0x4d238: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox>::operator=(rbx_core::SharedPtr<RBX::TextBox> const&)")]
// 0x4d2dc — __ZN5boost10shared_ptrIN3RBX7TextBoxEEaSERKS3_
// was: boost::shared_ptr<RBX::TextBox>::operator=(boost::shared_ptr<RBX::TextBox> const&)
pub fn stub_0x4d2dc() {
    // IDA 0x4d2dc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string)>::slot> &)")]
// 0x4ee0c — __ZN3rbx7signals6signalIFvSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot> &)
pub fn stub_0x4ee0c() {
    // IDA 0x4ee0c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::vector<void *,std::allocator<void *>>::~vector()")]
// 0x62f08 — __ZNSt6vectorIPvSaIS0_EED1Ev
pub fn stub_0x62f08() {
    // IDA 0x62f08: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::vector<void *,std::allocator<void *>>::push_back(void * const&)")]
// 0x62f1c — __ZNSt6vectorIPvSaIS0_EE9push_backERKS0_
pub fn stub_0x62f1c() {
    // IDA 0x62f1c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::vector<void *,std::allocator<void *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<void **,std::vector<void *,std::allocator<void *>>>,void * const&)")]
// 0x62f48 — __ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_
pub fn stub_0x62f48() {
    // IDA 0x62f48: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::_Vector_base<void *,std::allocator<void *>>::_M_allocate(unsigned long)")]
// 0x63028 — __ZNSt12_Vector_baseIPvSaIS0_EE11_M_allocateEm
pub fn stub_0x63028() {
    // IDA 0x63028: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::connect<boost::function<void ()(RBX::StandardOutMessage const&)>>(boost::function<void ()(RBX::StandardOutMessage const&)> const&)")]
// 0x64bc0 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE7connectIN5boost8functionIS6_EEEENS0_10connectionERKT_
pub fn stub_0x64bc0() {
    // IDA 0x64bc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::insert(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
// 0x64ca8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6insertEPNS7_4slotE
pub fn stub_0x64ca8() {
    // IDA 0x64ca8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)")]
// 0x64eb8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)
pub fn stub_0x64eb8() {
    // IDA 0x64eb8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)")]
// 0x64f5c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSERKSB_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)
pub fn stub_0x64f5c() {
    // IDA 0x64f5c: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_init_mutex(void)")]
// 0x65000 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE22safe_static_init_mutexEv
pub fn stub_0x65000() {
    // IDA 0x65000: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_do_get_mutex(void)")]
// 0x65004 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv
pub fn stub_0x65004() {
    // IDA 0x65004: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*>(boost::function<void ()(RBX::StandardOutMessage const&)> const&,rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*)")]
// 0x650fc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_EC2IPS8_EERKSC_T_
pub fn stub_0x650fc() {
    // IDA 0x650fc: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")]
// 0x651f8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED1Ev
pub fn stub_0x651f8() {
    // IDA 0x651f8: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")]
// 0x652cc — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED0Ev
pub fn stub_0x652cc() {
    // IDA 0x652cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::disconnect(void)")]
// 0x653a4 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot10disconnectEv
pub fn stub_0x653a4() {
    // IDA 0x653a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::connected(void)const")]
// 0x654b4 — __ZNK3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot9connectedEv
pub fn stub_0x654b4() {
    // IDA 0x654b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")]
// 0x654c0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_
pub fn stub_0x654c0() {
    // IDA 0x654c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")]
// 0x654c8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)
pub fn stub_0x654c8() {
    // IDA 0x654c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::operator()(RBX::StandardOutMessage const&)const")]
// 0x654d0 — __ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_
pub fn stub_0x654d0() {
    // IDA 0x654d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::remove(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
// 0x65594 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6removeEPNS7_4slotE
pub fn stub_0x65594() {
    // IDA 0x65594: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_init_mutex(void)")]
// 0x65684 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot22safe_static_init_mutexEv
pub fn stub_0x65684() {
    // IDA 0x65684: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_do_get_mutex(void)")]
// 0x65688 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x65688() {
    // IDA 0x65688: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")]
// 0x65778 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED1Ev
pub fn stub_0x65778() {
    // IDA 0x65778: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")]
// 0x6584c — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED0Ev
pub fn stub_0x6584c() {
    // IDA 0x6584c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")]
// 0x65924 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD1Ev
pub fn stub_0x65924() {
    // IDA 0x65924: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")]
// 0x659d0 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD0Ev
pub fn stub_0x659d0() {
    // IDA 0x659d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::assign_to_own(boost::function1<void,RBX::StandardOutMessage const&> const&)")]
// 0x65a80 — __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE13assign_to_ownERKS5_
pub fn stub_0x65a80() {
    // IDA 0x65a80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
// 0x65ab0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorRKN3RBX18StandardOutMessageEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
pub fn stub_0x65ab0() {
    // IDA 0x65ab0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::StandardOutMessage const&),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::StandardOutMessage const>::invoke(boost::detail::function::function_buffer &,RBX::StandardOutMessage const)")]
// 0x65b10 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorRKN3RBX18StandardOutMessageEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
pub fn stub_0x65b10() {
    // IDA 0x65b10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::clear(void)")]
// 0x65b20 — __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE5clearEv
pub fn stub_0x65b20() {
    // IDA 0x65b20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::find(int const&)")]
// 0x109b9c — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE4findERS1_
pub fn stub_0x109b9c() {
    // IDA 0x109b9c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>> const&,std::less<int> const&)")]
// 0x109bf8 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_
pub fn stub_0x109bf8() {
    // IDA 0x109bf8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_Rb_tree_impl<std::less<std::string>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<std::string const,FITAG *>>> const&,std::less<std::string> const&)")]
// 0x109c38 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_
pub fn stub_0x109c38() {
    // IDA 0x109c38: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::lower_bound(int const&)")]
// 0x109c78 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE11lower_boundERS1_
pub fn stub_0x109c78() {
    // IDA 0x109c78: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_create_node(std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *> const&)")]
// 0x109d98 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE14_M_create_nodeERKSC_
pub fn stub_0x109d98() {
    // IDA 0x109d98: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *> const&)")]
// 0x109dc8 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSC_
pub fn stub_0x109dc8() {
    // IDA 0x109dc8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_insert_unique(std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *> const&)")]
// 0x109e4c — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueERKSC_
pub fn stub_0x109e4c() {
    // IDA 0x109e4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_create_node(std::pair<std::string const,FITAG *> const&)")]
// 0x109f3c — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE14_M_create_nodeERKS4_
pub fn stub_0x109f3c() {
    // IDA 0x109f3c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,FITAG *>> *)")]
// 0x10a03c — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
pub fn stub_0x10a03c() {
    // IDA 0x10a03c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,FITAG *>> *)")]
// 0x10a0e4 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
pub fn stub_0x10a0e4() {
    // IDA 0x10a0e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>> *)")]
// 0x10a124 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
pub fn stub_0x10a124() {
    // IDA 0x10a124: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,FITAG *>>)")]
// 0x10a160 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseESt17_Rb_tree_iteratorIS4_E
pub fn stub_0x10a160() {
    // IDA 0x10a160: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::erase(std::_Rb_tree_iterator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>)")]
// 0x10a198 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE5eraseESt17_Rb_tree_iteratorISC_E
pub fn stub_0x10a198() {
    // IDA 0x10a198: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>,std::_Select1st<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>,std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *> const&)")]
// 0x10a1c8 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS0_IKSsS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
pub fn stub_0x10a1c8() {
    // IDA 0x10a1c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::upper_bound(std::string const&)")]
// 0x10a2ec — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11upper_boundERS1_
pub fn stub_0x10a2ec() {
    // IDA 0x10a2ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::lower_bound(std::string const&)")]
// 0x10a334 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11lower_boundERS1_
pub fn stub_0x10a334() {
    // IDA 0x10a334: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::equal_range(std::string const&)")]
// 0x10a37c — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE11equal_rangeERS1_
pub fn stub_0x10a37c() {
    // IDA 0x10a37c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::find(std::string const&)")]
// 0x10a3c4 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE4findERS1_
pub fn stub_0x10a3c4() {
    // IDA 0x10a3c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,FITAG *> const&)")]
// 0x10a43c — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
pub fn stub_0x10a43c() {
    // IDA 0x10a43c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_insert_unique(std::pair<std::string const,FITAG *> const&)")]
// 0x10a4c0 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueERKS4_
pub fn stub_0x10a4c0() {
    // IDA 0x10a4c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,FITAG *>>,std::pair<std::string const,FITAG *> const&)")]
// 0x10a584 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
pub fn stub_0x10a584() {
    // IDA 0x10a584: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,FITAG *>>,std::_Rb_tree_iterator<std::pair<std::string const,FITAG *>>)")]
// 0x10a6e4 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseESt17_Rb_tree_iteratorIS4_ESC_
pub fn stub_0x10a6e4() {
    // IDA 0x10a6e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,FITAG *>,std::_Select1st<std::pair<std::string const,FITAG *>>,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::erase(std::string const&)")]
// 0x10a760 — __ZNSt8_Rb_treeISsSt4pairIKSsP5FITAGESt10_Select1stIS4_ESt4lessISsESaIS4_EE5eraseERS1_
pub fn stub_0x10a760() {
    // IDA 0x10a760: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>>::operator[](std::string const&)")]
// 0x10a7a8 — __ZNSt3mapISsP5FITAGSt4lessISsESaISt4pairIKSsS1_EEEixERS5_
pub fn stub_0x10a7a8() {
    // IDA 0x10a7a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<int,std::map*<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG>>>,FITAG *<int>,std::allocator<std::less<std::string><int const,std::map*<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG>>>>>>::operator[](int const&)")]
// 0x10a8e4 — __ZNSt3mapIiPS_ISsP5FITAGSt4lessISsESaISt4pairIKSsS1_EEES2_IiESaIS4_IKiS9_EEEixERSB_
pub fn stub_0x10a8e4() {
    // IDA 0x10a8e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::find(int const&)")]
// 0x1118a0 — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE4findERS1_
pub fn stub_0x1118a0() {
    // IDA 0x1118a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,PluginNode *>>> const&,std::less<int> const&)")]
// 0x1118fc — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_
pub fn stub_0x1118fc() {
    // IDA 0x1118fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::lower_bound(int const&)")]
// 0x11193c — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE11lower_boundERS1_
pub fn stub_0x11193c() {
    // IDA 0x11193c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,PluginNode *>> *)")]
// 0x111970 — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
pub fn stub_0x111970() {
    // IDA 0x111970: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const,PluginNode *>>>::allocate(unsigned long,void const*)")]
// 0x1119ac — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiP10PluginNodeEEE8allocateEmPKv
pub fn stub_0x1119ac() {
    // IDA 0x1119ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_create_node(std::pair<int const,PluginNode *> const&)")]
// 0x1119dc — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE14_M_create_nodeERKS4_
pub fn stub_0x1119dc() {
    // IDA 0x1119dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,PluginNode *> const&)")]
// 0x111a0c — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
pub fn stub_0x111a0c() {
    // IDA 0x111a0c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert_unique(std::pair<int const,PluginNode *> const&)")]
// 0x111a90 — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE16_M_insert_uniqueERKS4_
pub fn stub_0x111a90() {
    // IDA 0x111a90: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,PluginNode *>>,std::pair<int const,PluginNode *> const&)")]
// 0x111b50 — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
pub fn stub_0x111b50() {
    // IDA 0x111b50: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<int,PluginNode *,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::operator[](int const&)")]
// 0x111c74 — __ZNSt3mapIiP10PluginNodeSt4lessIiESaISt4pairIKiS1_EEEixERS5_
pub fn stub_0x111c74() {
    // IDA 0x111c74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>> const&,std::less<int> const&)")]
// 0x1c4b48 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_
pub fn stub_0x1c4b48() {
    // IDA 0x1c4b48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
