//! boost_core_e — 150 boost stubs (EA-ordered, next uncovered after boost_core_d).
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost", sorted by EA, next 150 uncovered.
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; sanitized alias uses `rbx_core::SharedPtr` not `boost::`.
//! Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr.

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4b034c — __ZN5boost6detail12shared_countC2IPN3RBX10ForceFieldENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4b034c() -> ! {
    todo!("0x4b034c __ZN5boost6detail12shared_countC2IPN3RBX10ForceFieldENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEvent> RBX::Creatable<RBX::Instance>::create<RBX::CustomEvent>(void)")]
// 0x4b08a4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11CustomEventEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::CustomEvent> RBX::Creatable<RBX::Instance>::create<RBX::CustomEvent>(void)
pub fn stub_4b08a4() -> ! {
    todo!("0x4b08a4 __ZN3RBX9CreatableINS_8InstanceEE6createINS_11CustomEventEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEvent> RBX::shared_from<RBX::CustomEvent>(RBX::CustomEvent*)")]
// 0x4b1244 — __ZN3RBX11shared_fromINS_11CustomEventEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::CustomEvent> RBX::shared_from<RBX::CustomEvent>(RBX::CustomEvent*)
pub fn stub_4b1244() -> ! {
    todo!("0x4b1244 __ZN3RBX11shared_fromINS_11CustomEventEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEventReceiver>::shared_ptr<RBX::CustomEventReceiver>(rbx_core::WeakPtr<RBX::CustomEventReceiver> const&,boost::detail::sp_nothrow_tag)")]
// 0x4b13b4 — __ZN5boost10shared_ptrIN3RBX19CustomEventReceiverEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::CustomEventReceiver>::shared_ptr<RBX::CustomEventReceiver>(boost::weak_ptr<RBX::CustomEventReceiver> const&,boost::detail::sp_nothrow_tag)
pub fn stub_4b13b4() -> ! {
    todo!("0x4b13b4 __ZN5boost10shared_ptrIN3RBX19CustomEventReceiverEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::list(std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>> const&)")]
// 0x4b1430 — __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EEC2ERKS6_
// was: std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::list(std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>> const&)
pub fn stub_4b1430() -> ! {
    todo!("0x4b1430 __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EEC2ERKS6_")
}

#[doc(alias = "void std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::_M_initialize_dispatch<std::_List_const_iterator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>(std::_List_const_iterator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>,std::_List_const_iterator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>,std::__false_type)")]
// 0x4b14f8 — __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE22_M_initialize_dispatchISt20_List_const_iteratorIS4_EEEvT_SA_St12__false_type
// was: void std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_initialize_dispatch<std::_List_const_iterator<boost::weak_ptr<RBX::CustomEventReceiver>>>(std::_List_const_iterator<boost::weak_ptr<RBX::CustomEventReceiver>>,std::_List_const_iterator<boost::weak_ptr<RBX::CustomEventReceiver>>,std::__false_type)
pub fn stub_4b14f8() -> ! {
    todo!("0x4b14f8 __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE22_M_initialize_dispatchISt20_List_const_iteratorIS4_EEEvT_SA_St12__false_type")
}

#[doc(alias = "std::_List_base<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::_M_clear(void)")]
// 0x4b151c — __ZNSt10_List_baseIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE8_M_clearEv
// was: std::_List_base<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_clear(void)
pub fn stub_4b151c() -> ! {
    todo!("0x4b151c __ZNSt10_List_baseIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE8_M_clearEv")
}

#[doc(alias = "std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::_M_create_node(rbx_core::WeakPtr<RBX::CustomEventReceiver> const&)")]
// 0x4b1544 — __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE14_M_create_nodeERKS4_
// was: std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_create_node(boost::weak_ptr<RBX::CustomEventReceiver> const&)
pub fn stub_4b1544() -> ! {
    todo!("0x4b1544 __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE14_M_create_nodeERKS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEvent>::shared_ptr<RBX::CustomEvent,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4b1984 — __ZN5boost10shared_ptrIN3RBX11CustomEventEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::CustomEvent>::shared_ptr<RBX::CustomEvent,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4b1984() -> ! {
    todo!("0x4b1984 __ZN5boost10shared_ptrIN3RBX11CustomEventEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CustomEvent,RBX::CustomEvent>(rbx_core::SharedPtr<RBX::CustomEvent> const*,RBX::CustomEvent *)const")]
// 0x4b1a4c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11CustomEventES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CustomEvent,RBX::CustomEvent>(boost::shared_ptr<RBX::CustomEvent> const*,RBX::CustomEvent *)const
pub fn stub_4b1a4c() -> ! {
    todo!("0x4b1a4c __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11CustomEventES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4b1b34 — __ZN5boost6detail12shared_countC2IPN3RBX11CustomEventENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4b1b34() -> ! {
    todo!("0x4b1b34 __ZN5boost6detail12shared_countC2IPN3RBX11CustomEventENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4b1c3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4b1c3c() -> ! {
    todo!("0x4b1c3c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4b1c40 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_4b1c40() -> ! {
    todo!("0x4b1c40 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4b1c44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4b1c44() -> ! {
    todo!("0x4b1c44 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4b1c64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4b1c64() -> ! {
    todo!("0x4b1c64 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4b1c7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4b1c7c() -> ! {
    todo!("0x4b1c7c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEventReceiver> RBX::Creatable<RBX::Instance>::create<RBX::CustomEventReceiver>(void)")]
// 0x4b2220 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19CustomEventReceiverEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::CustomEventReceiver> RBX::Creatable<RBX::Instance>::create<RBX::CustomEventReceiver>(void)
pub fn stub_4b2220() -> ! {
    todo!("0x4b2220 __ZN3RBX9CreatableINS_8InstanceEE6createINS_19CustomEventReceiverEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>> const&)")]
// 0x4b2680 — __ZN3rbx7signals6signalIFvfEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_4b2680() -> ! {
    todo!("0x4b2680 __ZN3rbx7signals6signalIFvfEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot> const&)")]
// 0x4b29b4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSERKS7_
pub fn stub_4b29b4() -> ! {
    todo!("0x4b29b4 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx::signals::signal<void ()(float)>::slot*)")]
// 0x4b2ce0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSEPS6_
pub fn stub_4b2ce0() -> ! {
    todo!("0x4b2ce0 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot()")]
// 0x4b2d04 — __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
pub fn stub_4b2d04() -> ! {
    todo!("0x4b2d04 __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot()")]
// 0x4b2d30 — __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
pub fn stub_4b2d30() -> ! {
    todo!("0x4b2d30 __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)")]
// 0x4b2f20 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf
pub fn stub_4b2f20() -> ! {
    todo!("0x4b2f20 __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)")]
// 0x4b2f34 — __ZThn4_N3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf
pub fn stub_4b2f34() -> ! {
    todo!("0x4b2f34 __ZThn4_N3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>::operator()<float>(float &)")]
// 0x4b2f48 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIfEEvRT_
pub fn stub_4b2f48() -> ! {
    todo!("0x4b2f48 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIfEEvRT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable()")]
// 0x4b324c — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev
pub fn stub_4b324c() -> ! {
    todo!("0x4b324c __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable()")]
// 0x4b3278 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev
pub fn stub_4b3278() -> ! {
    todo!("0x4b3278 __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEventReceiver>::shared_ptr<RBX::CustomEventReceiver,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4b3668 — __ZN5boost10shared_ptrIN3RBX19CustomEventReceiverEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::CustomEventReceiver>::shared_ptr<RBX::CustomEventReceiver,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4b3668() -> ! {
    todo!("0x4b3668 __ZN5boost10shared_ptrIN3RBX19CustomEventReceiverEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CustomEventReceiver,RBX::CustomEventReceiver>(rbx_core::SharedPtr<RBX::CustomEventReceiver> const*,RBX::CustomEventReceiver *)const")]
// 0x4b3730 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CustomEventReceiverES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CustomEventReceiver,RBX::CustomEventReceiver>(boost::shared_ptr<RBX::CustomEventReceiver> const*,RBX::CustomEventReceiver *)const
pub fn stub_4b3730() -> ! {
    todo!("0x4b3730 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CustomEventReceiverES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4b3818 — __ZN5boost6detail12shared_countC2IPN3RBX19CustomEventReceiverENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4b3818() -> ! {
    todo!("0x4b3818 __ZN5boost6detail12shared_countC2IPN3RBX19CustomEventReceiverENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4b3920 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4b3920() -> ! {
    todo!("0x4b3920 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4b3924 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_4b3924() -> ! {
    todo!("0x4b3924 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4b3928 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4b3928() -> ! {
    todo!("0x4b3928 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4b3948 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4b3948() -> ! {
    todo!("0x4b3948 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4b3960 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4b3960() -> ! {
    todo!("0x4b3960 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::function1<void,std::exception &>::swap(boost::function1<void,std::exception &>&)")]
// 0x4d5a08 — __ZN5boost9function1IvRSt9exceptionE4swapERS3_
pub fn stub_4d5a08() -> ! {
    todo!("0x4d5a08 __ZN5boost9function1IvRSt9exceptionE4swapERS3_")
}

#[doc(alias = "boost::function1<void,std::exception &>::clear(void)")]
// 0x4d5ae4 — __ZN5boost9function1IvRSt9exceptionE5clearEv
pub fn stub_4d5ae4() -> ! {
    todo!("0x4d5ae4 __ZN5boost9function1IvRSt9exceptionE5clearEv")
}

#[doc(alias = "boost::function1<void,std::exception &>::move_assign(boost::function1<void,std::exception &>&)")]
// 0x4d5b10 — __ZN5boost9function1IvRSt9exceptionE11move_assignERS3_
pub fn stub_4d5b10() -> ! {
    todo!("0x4d5b10 __ZN5boost9function1IvRSt9exceptionE11move_assignERS3_")
}

#[doc(alias = "boost::detail::function::functor_manager<void (*)(std::exception &)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x4d5c14 — __ZN5boost6detail8function15functor_managerIPFvRSt9exceptionEE6manageERKNS1_15function_bufferERS8_NS1_30functor_manager_operation_typeE
pub fn stub_4d5c14() -> ! {
    todo!("0x4d5c14 __ZN5boost6detail8function15functor_managerIPFvRSt9exceptionEE6manageERKNS1_15function_bufferERS8_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_invoker1<void (*)(std::exception &),void,std::exception &>::invoke(boost::detail::function::function_buffer &,std::exception &)")]
// 0x4d5c70 — __ZN5boost6detail8function22void_function_invoker1IPFvRSt9exceptionEvS4_E6invokeERNS1_15function_bufferES4_
pub fn stub_4d5c70() -> ! {
    todo!("0x4d5c70 __ZN5boost6detail8function22void_function_invoker1IPFvRSt9exceptionEvS4_E6invokeERNS1_15function_bufferES4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VelocityMotor> RBX::Creatable<RBX::Instance>::create<RBX::VelocityMotor>(void)")]
// 0x4e6e68 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13VelocityMotorEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::VelocityMotor> RBX::Creatable<RBX::Instance>::create<RBX::VelocityMotor>(void)
pub fn stub_4e6e68() -> ! {
    todo!("0x4e6e68 __ZN3RBX9CreatableINS_8InstanceEE6createINS_13VelocityMotorEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Hole>::operator=(rbx_core::SharedPtr<RBX::Hole> const&)")]
// 0x4e6f18 — __ZN5boost10shared_ptrIN3RBX4HoleEEaSERKS3_
// was: boost::shared_ptr<RBX::Hole>::operator=(boost::shared_ptr<RBX::Hole> const&)
pub fn stub_4e6f18() -> ! {
    todo!("0x4e6f18 __ZN5boost10shared_ptrIN3RBX4HoleEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Hole> RBX::shared_from<RBX::Hole>(RBX::Hole*)")]
// 0x4e6f50 — __ZN3RBX11shared_fromINS_4HoleEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Hole> RBX::shared_from<RBX::Hole>(RBX::Hole*)
pub fn stub_4e6f50() -> ! {
    todo!("0x4e6f50 __ZN3RBX11shared_fromINS_4HoleEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>> const&)")]
// 0x4e70c0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvNS4_13VelocityMotorEEENSA_5list1INSA_5valueIPSE_EEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>> const&)
pub fn stub_4e70c0() -> ! {
    todo!("0x4e70c0 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvNS4_13VelocityMotorEEENSA_5list1INSA_5valueIPSE_EEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MotorFeature> RBX::Creatable<RBX::Instance>::create<RBX::MotorFeature>(void)")]
// 0x4e7e78 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12MotorFeatureEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::MotorFeature> RBX::Creatable<RBX::Instance>::create<RBX::MotorFeature>(void)
pub fn stub_4e7e78() -> ! {
    todo!("0x4e7e78 __ZN3RBX9CreatableINS_8InstanceEE6createINS_12MotorFeatureEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MotorFeature>::shared_ptr<RBX::MotorFeature,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4e7f28 — __ZN5boost10shared_ptrIN3RBX12MotorFeatureEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::MotorFeature>::shared_ptr<RBX::MotorFeature,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4e7f28() -> ! {
    todo!("0x4e7f28 __ZN5boost10shared_ptrIN3RBX12MotorFeatureEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::MotorFeature,RBX::MotorFeature>(rbx_core::SharedPtr<RBX::MotorFeature> const*,RBX::MotorFeature *)const")]
// 0x4e7ff0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12MotorFeatureES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::MotorFeature,RBX::MotorFeature>(boost::shared_ptr<RBX::MotorFeature> const*,RBX::MotorFeature *)const
pub fn stub_4e7ff0() -> ! {
    todo!("0x4e7ff0 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12MotorFeatureES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4e80d8 — __ZN5boost6detail12shared_countC2IPN3RBX12MotorFeatureENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4e80d8() -> ! {
    todo!("0x4e80d8 __ZN5boost6detail12shared_countC2IPN3RBX12MotorFeatureENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4e81e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MotorFeatureENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4e81e0() -> ! {
    todo!("0x4e81e0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MotorFeatureENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4e81e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MotorFeatureENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_4e81e4() -> ! {
    todo!("0x4e81e4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MotorFeatureENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4e81e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MotorFeatureENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4e81e8() -> ! {
    todo!("0x4e81e8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MotorFeatureENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4e8208 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MotorFeatureENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4e8208() -> ! {
    todo!("0x4e8208 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MotorFeatureENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MotorFeature *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4e8220 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MotorFeatureENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4e8220() -> ! {
    todo!("0x4e8220 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MotorFeatureENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Hole> RBX::Creatable<RBX::Instance>::create<RBX::Hole>(void)")]
// 0x4e8828 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4HoleEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Hole> RBX::Creatable<RBX::Instance>::create<RBX::Hole>(void)
pub fn stub_4e8828() -> ! {
    todo!("0x4e8828 __ZN3RBX9CreatableINS_8InstanceEE6createINS_4HoleEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Hole>::shared_ptr<RBX::Hole,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4e88d8 — __ZN5boost10shared_ptrIN3RBX4HoleEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Hole>::shared_ptr<RBX::Hole,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4e88d8() -> ! {
    todo!("0x4e88d8 __ZN5boost10shared_ptrIN3RBX4HoleEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Hole,RBX::Hole>(rbx_core::SharedPtr<RBX::Hole> const*,RBX::Hole *)const")]
// 0x4e89a0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4HoleES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Hole,RBX::Hole>(boost::shared_ptr<RBX::Hole> const*,RBX::Hole *)const
pub fn stub_4e89a0() -> ! {
    todo!("0x4e89a0 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4HoleES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4e8a88 — __ZN5boost6detail12shared_countC2IPN3RBX4HoleENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4e8a88() -> ! {
    todo!("0x4e8a88 __ZN5boost6detail12shared_countC2IPN3RBX4HoleENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4e8b90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HoleENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4e8b90() -> ! {
    todo!("0x4e8b90 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HoleENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4e8b94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HoleENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_4e8b94() -> ! {
    todo!("0x4e8b94 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HoleENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4e8b98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HoleENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4e8b98() -> ! {
    todo!("0x4e8b98 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HoleENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4e8bb8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HoleENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4e8bb8() -> ! {
    todo!("0x4e8bb8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HoleENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hole *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4e8bd0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HoleENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4e8bd0() -> ! {
    todo!("0x4e8bd0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HoleENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>>::~callable_slot()")]
// 0x4e9054 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvNS4_13VelocityMotorEEENSA_5list1INSA_5valueIPSE_EEEEEEED1Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>>::~callable_slot()
pub fn stub_4e9054() -> ! {
    todo!("0x4e9054 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvNS4_13VelocityMotorEEENSA_5list1INSA_5valueIPSE_EEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>>::~callable_slot()")]
// 0x4e9080 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvNS4_13VelocityMotorEEENSA_5list1INSA_5valueIPSE_EEEEEEED0Ev
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>>::~callable_slot()
pub fn stub_4e9080() -> ! {
    todo!("0x4e9080 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvNS4_13VelocityMotorEEENSA_5list1INSA_5valueIPSE_EEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x4e9154 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_13VelocityMotorEEENSB_5list1INSB_5valueIPSF_EEEEEELi2ES8_E4callES7_S7_
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_4e9154() -> ! {
    todo!("0x4e9154 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_13VelocityMotorEEENSB_5list1INSB_5valueIPSF_EEEEEELi2ES8_E4callES7_S7_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x4e916c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_13VelocityMotorEEENSB_5list1INSB_5valueIPSF_EEEEEELi2ES8_E4callES7_S7_
// was: `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_4e916c() -> ! {
    todo!("0x4e916c __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_13VelocityMotorEEENSB_5list1INSB_5valueIPSF_EEEEEELi2ES8_E4callES7_S7_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// 0x4e9184 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_13VelocityMotorEEENSB_5list1INSB_5valueIPSF_EEEEEELi2ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_4e9184() -> ! {
    todo!("0x4e9184 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_13VelocityMotorEEENSB_5list1INSB_5valueIPSF_EEEEEELi2ES8_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// 0x4e91b0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_13VelocityMotorEEENSB_5list1INSB_5valueIPSF_EEEEEELi2ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::VelocityMotor>,boost::_bi::list1<boost::_bi::value<RBX::VelocityMotor*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_4e91b0() -> ! {
    todo!("0x4e91b0 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_13VelocityMotorEEENSB_5list1INSB_5valueIPSF_EEEEEELi2ES8_ED0Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VelocityMotor>::shared_ptr<RBX::VelocityMotor,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4e9480 — __ZN5boost10shared_ptrIN3RBX13VelocityMotorEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::VelocityMotor>::shared_ptr<RBX::VelocityMotor,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4e9480() -> ! {
    todo!("0x4e9480 __ZN5boost10shared_ptrIN3RBX13VelocityMotorEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::VelocityMotor,RBX::VelocityMotor>(rbx_core::SharedPtr<RBX::VelocityMotor> const*,RBX::VelocityMotor *)const")]
// 0x4e9548 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13VelocityMotorES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::VelocityMotor,RBX::VelocityMotor>(boost::shared_ptr<RBX::VelocityMotor> const*,RBX::VelocityMotor *)const
pub fn stub_4e9548() -> ! {
    todo!("0x4e9548 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13VelocityMotorES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4e9630 — __ZN5boost6detail12shared_countC2IPN3RBX13VelocityMotorENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4e9630() -> ! {
    todo!("0x4e9630 __ZN5boost6detail12shared_countC2IPN3RBX13VelocityMotorENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4e9738 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13VelocityMotorENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4e9738() -> ! {
    todo!("0x4e9738 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13VelocityMotorENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4e973c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13VelocityMotorENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_4e973c() -> ! {
    todo!("0x4e973c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13VelocityMotorENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4e9740 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13VelocityMotorENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4e9740() -> ! {
    todo!("0x4e9740 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13VelocityMotorENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4e9760 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13VelocityMotorENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4e9760() -> ! {
    todo!("0x4e9760 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13VelocityMotorENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4e9778 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13VelocityMotorENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4e9778() -> ! {
    todo!("0x4e9778 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13VelocityMotorENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::FilterDescendents::FilterDescendents(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x4ef18c — __ZN3RBX17FilterDescendentsC1EN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::FilterDescendents::FilterDescendents(boost::shared_ptr<RBX::Instance>)
pub fn stub_4ef18c() -> ! {
    todo!("0x4ef18c __ZN3RBX17FilterDescendentsC1EN5boost10shared_ptrINS_8InstanceEEE")
}

#[doc(alias = "RBX::FilterDescendents::FilterDescendents(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x4ef190 — __ZN3RBX17FilterDescendentsC2EN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::FilterDescendents::FilterDescendents(boost::shared_ptr<RBX::Instance>)
pub fn stub_4ef190() -> ! {
    todo!("0x4ef190 __ZN3RBX17FilterDescendentsC2EN5boost10shared_ptrINS_8InstanceEEE")
}

#[doc(alias = "RBX::FilterDescendentsList::FilterDescendentsList(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const*)")]
// 0x4ef28c — __ZN3RBX21FilterDescendentsListC1EPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EE
// was: RBX::FilterDescendentsList::FilterDescendentsList(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const*)
pub fn stub_4ef28c() -> ! {
    todo!("0x4ef28c __ZN3RBX21FilterDescendentsListC1EPKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ModelInstance>::operator=(rbx_core::SharedPtr<RBX::ModelInstance> const&)")]
// 0x4ef3ec — __ZN5boost10shared_ptrIN3RBX13ModelInstanceEEaSERKS3_
// was: boost::shared_ptr<RBX::ModelInstance>::operator=(boost::shared_ptr<RBX::ModelInstance> const&)
pub fn stub_4ef3ec() -> ! {
    todo!("0x4ef3ec __ZN5boost10shared_ptrIN3RBX13ModelInstanceEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Fire> RBX::Creatable<RBX::Instance>::create<RBX::Fire>(void)")]
// 0x4f0004 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4FireEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Fire> RBX::Creatable<RBX::Instance>::create<RBX::Fire>(void)
pub fn stub_4f0004() -> ! {
    todo!("0x4f0004 __ZN3RBX9CreatableINS_8InstanceEE6createINS_4FireEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Fire>::shared_ptr<RBX::Fire,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4f00b4 — __ZN5boost10shared_ptrIN3RBX4FireEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Fire>::shared_ptr<RBX::Fire,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4f00b4() -> ! {
    todo!("0x4f00b4 __ZN5boost10shared_ptrIN3RBX4FireEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Fire,RBX::Fire>(rbx_core::SharedPtr<RBX::Fire> const*,RBX::Fire *)const")]
// 0x4f017c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4FireES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Fire,RBX::Fire>(boost::shared_ptr<RBX::Fire> const*,RBX::Fire *)const
pub fn stub_4f017c() -> ! {
    todo!("0x4f017c __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4FireES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4f0264 — __ZN5boost6detail12shared_countC2IPN3RBX4FireENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4f0264() -> ! {
    todo!("0x4f0264 __ZN5boost6detail12shared_countC2IPN3RBX4FireENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4f036c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4f036c() -> ! {
    todo!("0x4f036c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4f0370 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_4f0370() -> ! {
    todo!("0x4f0370 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4f0374 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4f0374() -> ! {
    todo!("0x4f0374 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4f0394 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4f0394() -> ! {
    todo!("0x4f0394 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Fire *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4f03ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4f03ac() -> ! {
    todo!("0x4f03ac __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FireENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Flag::onEvent_flagTouched(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x4f1da0 — __ZN3RBX4Flag19onEvent_flagTouchedEN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::Flag::onEvent_flagTouched(boost::shared_ptr<RBX::Instance>)
pub fn stub_4f1da0() -> ! {
    todo!("0x4f1da0 __ZN3RBX4Flag19onEvent_flagTouchedEN5boost10shared_ptrINS_8InstanceEEE")
}

#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Flag,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Flag*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Flag,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Flag*>,boost::arg<1>>>)")]
// 0x4f1eac — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_4FlagENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
// was: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Flag*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Flag*>,boost::arg<1>>>)
pub fn stub_4f1eac() -> ! {
    todo!("0x4f1eac __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_4FlagENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Flag> RBX::Creatable<RBX::Instance>::create<RBX::Flag>(void)")]
// 0x4f22bc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4FlagEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Flag> RBX::Creatable<RBX::Instance>::create<RBX::Flag>(void)
pub fn stub_4f22bc() -> ! {
    todo!("0x4f22bc __ZN3RBX9CreatableINS_8InstanceEE6createINS_4FlagEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Flag>::shared_ptr<RBX::Flag,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4f2370 — __ZN5boost10shared_ptrIN3RBX4FlagEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Flag>::shared_ptr<RBX::Flag,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4f2370() -> ! {
    todo!("0x4f2370 __ZN5boost10shared_ptrIN3RBX4FlagEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Flag,RBX::Flag>(rbx_core::SharedPtr<RBX::Flag> const*,RBX::Flag *)const")]
// 0x4f2438 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4FlagES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Flag,RBX::Flag>(boost::shared_ptr<RBX::Flag> const*,RBX::Flag *)const
pub fn stub_4f2438() -> ! {
    todo!("0x4f2438 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4FlagES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4f2520 — __ZN5boost6detail12shared_countC2IPN3RBX4FlagENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4f2520() -> ! {
    todo!("0x4f2520 __ZN5boost6detail12shared_countC2IPN3RBX4FlagENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4f2628 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4f2628() -> ! {
    todo!("0x4f2628 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4f262c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_4f262c() -> ! {
    todo!("0x4f262c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4f2630 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4f2630() -> ! {
    todo!("0x4f2630 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4f2650 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4f2650() -> ! {
    todo!("0x4f2650 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Flag *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4f2668 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4f2668() -> ! {
    todo!("0x4f2668 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4FlagENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Flag,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Flag*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x4f2a08 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Flag*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_4f2a08() -> ! {
    todo!("0x4f2a08 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Flag,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Flag*>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x4f2a68 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Flag*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
pub fn stub_4f2a68() -> ! {
    todo!("0x4f2a68 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Flag *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Flag,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Flag,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// 0x4f2a84 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4FlagEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<RBX::Flag *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_4f2a84() -> ! {
    todo!("0x4f2a84 __ZN5boost3_bi5list2INS0_5valueIPN3RBX4FlagEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::Flag,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Flag*,rbx_core::SharedPtr<RBX::Instance>)const")]
// 0x4f2b5c — __ZNK5boost4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// was: boost::_mfi::mf1<void,RBX::Flag,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Flag*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_4f2b5c() -> ! {
    todo!("0x4f2b5c __ZNK5boost4_mfi3mf1IvN3RBX4FlagENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_")
}

#[doc(alias = "RBX::FlagStand::onEvent_standTouched(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x4f39a4 — __ZN3RBX9FlagStand20onEvent_standTouchedEN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::FlagStand::onEvent_standTouched(boost::shared_ptr<RBX::Instance>)
pub fn stub_4f39a4() -> ! {
    todo!("0x4f39a4 __ZN3RBX9FlagStand20onEvent_standTouchedEN5boost10shared_ptrINS_8InstanceEEE")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::~EventDesc()")]
// 0x4f4574 — __ZN3RBX10Reflection9EventDescINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
// was: RBX::Reflection::EventDesc<RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::~EventDesc()
pub fn stub_4f4574() -> ! {
    todo!("0x4f4574 __ZN3RBX10Reflection9EventDescINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev")
}

#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>)")]
// 0x4f47b4 — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_9FlagStandENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
// was: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>)
pub fn stub_4f47b4() -> ! {
    todo!("0x4f47b4 __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_9FlagStandENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Flag>::operator=(rbx_core::SharedPtr<RBX::Flag> const&)")]
// 0x4f4934 — __ZN5boost10shared_ptrIN3RBX4FlagEEaSERKS3_
// was: boost::shared_ptr<RBX::Flag>::operator=(boost::shared_ptr<RBX::Flag> const&)
pub fn stub_4f4934() -> ! {
    todo!("0x4f4934 __ZN5boost10shared_ptrIN3RBX4FlagEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Flag> RBX::shared_from<RBX::Flag>(RBX::Flag*)")]
// 0x4f496c — __ZN3RBX11shared_fromINS_4FlagEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Flag> RBX::shared_from<RBX::Flag>(RBX::Flag*)
pub fn stub_4f496c() -> ! {
    todo!("0x4f496c __ZN3RBX11shared_fromINS_4FlagEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FlagStand> RBX::Creatable<RBX::Instance>::create<RBX::FlagStand>(void)")]
// 0x4f5524 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9FlagStandEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::FlagStand> RBX::Creatable<RBX::Instance>::create<RBX::FlagStand>(void)
pub fn stub_4f5524() -> ! {
    todo!("0x4f5524 __ZN3RBX9CreatableINS_8InstanceEE6createINS_9FlagStandEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FlagStand>::shared_ptr<RBX::FlagStand,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4f55d8 — __ZN5boost10shared_ptrIN3RBX9FlagStandEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::FlagStand>::shared_ptr<RBX::FlagStand,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4f55d8() -> ! {
    todo!("0x4f55d8 __ZN5boost10shared_ptrIN3RBX9FlagStandEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FlagStand,RBX::FlagStand>(rbx_core::SharedPtr<RBX::FlagStand> const*,RBX::FlagStand *)const")]
// 0x4f56a0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9FlagStandES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FlagStand,RBX::FlagStand>(boost::shared_ptr<RBX::FlagStand> const*,RBX::FlagStand *)const
pub fn stub_4f56a0() -> ! {
    todo!("0x4f56a0 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9FlagStandES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4f5788 — __ZN5boost6detail12shared_countC2IPN3RBX9FlagStandENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4f5788() -> ! {
    todo!("0x4f5788 __ZN5boost6detail12shared_countC2IPN3RBX9FlagStandENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4f5890 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4f5890() -> ! {
    todo!("0x4f5890 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4f5894 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_4f5894() -> ! {
    todo!("0x4f5894 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4f5898 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4f5898() -> ! {
    todo!("0x4f5898 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4f58b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4f58b8() -> ! {
    todo!("0x4f58b8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStand *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4f58d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4f58d0() -> ! {
    todo!("0x4f58d0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9FlagStandENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FlagStandService> RBX::Creatable<RBX::Instance>::create<RBX::FlagStandService>(void)")]
// 0x4f6440 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16FlagStandServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::FlagStandService> RBX::Creatable<RBX::Instance>::create<RBX::FlagStandService>(void)
pub fn stub_4f6440() -> ! {
    todo!("0x4f6440 __ZN3RBX9CreatableINS_8InstanceEE6createINS_16FlagStandServiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::FlagStandService>(rbx_core::SharedPtr<RBX::FlagStandService> const&)")]
// 0x4f64f0 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_16FlagStandServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FlagStandService>(boost::shared_ptr<RBX::FlagStandService> const&)
pub fn stub_4f64f0() -> ! {
    todo!("0x4f64f0 __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_16FlagStandServiceEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FlagStandService>::shared_ptr<RBX::FlagStandService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4f6600 — __ZN5boost10shared_ptrIN3RBX16FlagStandServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::FlagStandService>::shared_ptr<RBX::FlagStandService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4f6600() -> ! {
    todo!("0x4f6600 __ZN5boost10shared_ptrIN3RBX16FlagStandServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FlagStandService,RBX::FlagStandService>(rbx_core::SharedPtr<RBX::FlagStandService> const*,RBX::FlagStandService *)const")]
// 0x4f66c8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16FlagStandServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FlagStandService,RBX::FlagStandService>(boost::shared_ptr<RBX::FlagStandService> const*,RBX::FlagStandService *)const
pub fn stub_4f66c8() -> ! {
    todo!("0x4f66c8 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16FlagStandServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4f67b0 — __ZN5boost6detail12shared_countC2IPN3RBX16FlagStandServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4f67b0() -> ! {
    todo!("0x4f67b0 __ZN5boost6detail12shared_countC2IPN3RBX16FlagStandServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4f68b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4f68b8() -> ! {
    todo!("0x4f68b8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4f68bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_4f68bc() -> ! {
    todo!("0x4f68bc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4f68c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4f68c0() -> ! {
    todo!("0x4f68c0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4f68e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4f68e0() -> ! {
    todo!("0x4f68e0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4f68f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4f68f8() -> ! {
    todo!("0x4f68f8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x4f697c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_4f697c() -> ! {
    todo!("0x4f697c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// 0x4f69dc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
pub fn stub_4f69dc() -> ! {
    todo!("0x4f69dc __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::FlagStand *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// 0x4f69f8 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX9FlagStandEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<RBX::FlagStand *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_4f69f8() -> ! {
    todo!("0x4f69f8 __ZN5boost3_bi5list2INS0_5valueIPN3RBX9FlagStandEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::FlagStand*,rbx_core::SharedPtr<RBX::Instance>)const")]
// 0x4f6ad0 — __ZNK5boost4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// was: boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>::operator()(RBX::FlagStand*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_4f6ad0() -> ! {
    todo!("0x4f6ad0 __ZNK5boost4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x4f6f10 — __ZN3RBX10Reflection9EventDescINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_4f6f10() -> ! {
    todo!("0x4f6f10 __ZN3RBX10Reflection9EventDescINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::~EventDesc()")]
// 0x4f7094 — __ZN3RBX10Reflection9EventDescINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
// was: RBX::Reflection::EventDesc<RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::~EventDesc()
pub fn stub_4f7094() -> ! {
    todo!("0x4f7094 __ZN3RBX10Reflection9EventDescINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x4f7148 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<1,RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_4f7148() -> ! {
    todo!("0x4f7148 __ZNK3RBX10Reflection13EventDescImplILi1ENS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x4f729c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// was: RBX::Reflection::EventDescImpl<1,RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_4f729c() -> ! {
    todo!("0x4f729c __ZNK3RBX10Reflection13EventDescImplILi1ENS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x4f73fc — __ZNK3RBX10Reflection13EventDescBaseINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// was: RBX::Reflection::EventDescBase<RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_4f73fc() -> ! {
    todo!("0x4f73fc __ZNK3RBX10Reflection13EventDescBaseINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")
}

#[doc(alias = "RBX::renderForceField(rbx_core::SharedPtr<RBX::Instance>,RBX::Adorn *,int,int)")]
// 0x4f80a0 — __ZN3RBX16renderForceFieldEN5boost10shared_ptrINS_8InstanceEEEPNS_5AdornEii
// was: RBX::renderForceField(boost::shared_ptr<RBX::Instance>,RBX::Adorn *,int,int)
pub fn stub_4f80a0() -> ! {
    todo!("0x4f80a0 __ZN3RBX16renderForceFieldEN5boost10shared_ptrINS_8InstanceEEEPNS_5AdornEii")
}

#[doc(alias = "RBX::Game::setDataModel(rbx_core::SharedPtr<RBX::DataModel>)")]
// 0x4fc0c8 — __ZN3RBX4Game12setDataModelEN5boost10shared_ptrINS_9DataModelEEE
// was: RBX::Game::setDataModel(boost::shared_ptr<RBX::DataModel>)
pub fn stub_4fc0c8() -> ! {
    todo!("0x4fc0c8 __ZN3RBX4Game12setDataModelEN5boost10shared_ptrINS_9DataModelEEE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ProfanityFilter>::~shared_ptr()")]
// 0x4fc774 — __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEED1Ev
// was: boost::shared_ptr<RBX::ProfanityFilter>::~shared_ptr()
pub fn stub_4fc774() -> ! {
    todo!("0x4fc774 __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEED1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ProfanityFilter>::operator=(rbx_core::SharedPtr<RBX::ProfanityFilter> const&)")]
// 0x4fc788 — __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEaSERKS3_
// was: boost::shared_ptr<RBX::ProfanityFilter>::operator=(boost::shared_ptr<RBX::ProfanityFilter> const&)
pub fn stub_4fc788() -> ! {
    todo!("0x4fc788 __ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEaSERKS3_")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::CommonVerbs>::reset<RBX::CommonVerbs>(RBX::CommonVerbs *)")]
// 0x4fcd04 — __ZN5boost10shared_ptrIN3RBX11CommonVerbsEE5resetIS2_EEvPT_
// was: void boost::shared_ptr<RBX::CommonVerbs>::reset<RBX::CommonVerbs>(RBX::CommonVerbs *)
pub fn stub_4fcd04() -> ! {
    todo!("0x4fcd04 __ZN5boost10shared_ptrIN3RBX11CommonVerbsEE5resetIS2_EEvPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::OverlayDataModel>::operator=(rbx_core::SharedPtr<RBX::OverlayDataModel> const&)")]
// 0x4fcd5c — __ZN5boost10shared_ptrIN3RBX16OverlayDataModelEEaSERKS3_
// was: boost::shared_ptr<RBX::OverlayDataModel>::operator=(boost::shared_ptr<RBX::OverlayDataModel> const&)
pub fn stub_4fcd5c() -> ! {
    todo!("0x4fcd5c __ZN5boost10shared_ptrIN3RBX16OverlayDataModelEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>::operator=(rbx_core::SharedPtr<RBX::DataModel> const&)")]
// 0x4fcd94 — __ZN5boost10shared_ptrIN3RBX9DataModelEEaSERKS3_
// was: boost::shared_ptr<RBX::DataModel>::operator=(boost::shared_ptr<RBX::DataModel> const&)
pub fn stub_4fcd94() -> ! {
    todo!("0x4fcd94 __ZN5boost10shared_ptrIN3RBX9DataModelEEaSERKS3_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list_av_2<RBX::Game*,std::string>::type> boost::bind<void,RBX::Game,std::string const&,RBX::Game*,std::string>(void (RBX::Game::*)(std::string const&),RBX::Game*,std::string)")]
// 0x4fcdcc — __ZN5boost4bindIvN3RBX4GameERKSsPS2_SsEENS_3_bi6bind_tIT_NS_4_mfi3mf1IS8_T0_T1_EENS6_9list_av_2IT2_T3_E4typeEEEMSB_FS8_SC_ESF_SG_
pub fn stub_4fcdcc() -> ! {
    todo!("0x4fcdcc __ZN5boost4bindIvN3RBX4GameERKSsPS2_SsEENS_3_bi6bind_tIT_NS_4_mfi3mf1IS8_T0_T1_EENS6_9list_av_2IT2_T3_E4typeEEEMSB_FS8_SC_ESF_SG_")
}

#[doc(alias = "void RBX::shutdownDM<RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> &)")]
// 0x4fd0c8 — __ZN3RBX10shutdownDMINS_9DataModelEEEvRN5boost10shared_ptrIT_EE
// was: void RBX::shutdownDM<RBX::DataModel>(boost::shared_ptr<RBX::DataModel> &)
pub fn stub_4fd0c8() -> ! {
    todo!("0x4fd0c8 __ZN3RBX10shutdownDMINS_9DataModelEEEvRN5boost10shared_ptrIT_EE")
}

#[doc(alias = "void RBX::shutdownDM<RBX::OverlayDataModel>(rbx_core::SharedPtr<RBX::OverlayDataModel> &)")]
// 0x4fd1e8 — __ZN3RBX10shutdownDMINS_16OverlayDataModelEEEvRN5boost10shared_ptrIT_EE
// was: void RBX::shutdownDM<RBX::OverlayDataModel>(boost::shared_ptr<RBX::OverlayDataModel> &)
pub fn stub_4fd1e8() -> ! {
    todo!("0x4fd1e8 __ZN3RBX10shutdownDMINS_16OverlayDataModelEEEvRN5boost10shared_ptrIT_EE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> &)")]
// 0x4fd44c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
pub fn stub_4fd44c() -> ! {
    todo!("0x4fd44c __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS7_5list2INS7_5valueIPSB_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x4fd5d4 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS7_5list2INS7_5valueIPSB_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_4fd5d4() -> ! {
    todo!("0x4fd5d4 __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS7_5list2INS7_5valueIPSB_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS6_5list2INS6_5valueIPSA_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x4fd70c — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS6_5list2INS6_5valueIPSA_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
pub fn stub_4fd70c() -> ! {
    todo!("0x4fd70c __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS6_5list2INS6_5valueIPSA_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}
