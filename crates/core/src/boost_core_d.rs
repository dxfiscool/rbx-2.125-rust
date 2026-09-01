//! core-D: 150 boost stubs — filtered boost:: namespace.
//! EA-ordered boost stubs (0x46e8cc..0x4b0264) so `cargo check` stays green.
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost", sorted by EA, next 150 after 4516 already covered.
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; signatures use `rbx_core::SharedPtr` not `boost::`.


#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~deque()")]
// 0x46e8cc — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EED2Ev
// was: std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~deque()
pub fn stub_46e8cc() -> ! {
    todo!("0x46e8cc __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EED2Ev")
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~_Deque_base()")]
// 0x46e9b4 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EED2Ev
// was: std::_Deque_base<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::~_Deque_base()
pub fn stub_46e9b4() -> ! {
    todo!("0x46e9b4 __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EED2Ev")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_destroy_data_aux(std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>)")]
// 0x46e9e0 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_
// was: std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*>)
pub fn stub_46e9e0() -> ! {
    todo!("0x46e9e0 __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_")
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_initialize_map(unsigned long)")]
// 0x46eb20 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE17_M_initialize_mapEm
// was: std::_Deque_base<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_initialize_map(unsigned long)
pub fn stub_46eb20() -> ! {
    todo!("0x46eb20 __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_create_nodes(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>**,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>**)")]
// 0x46ec78 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE15_M_create_nodesEPPS7_SB_
// was: std::_Deque_base<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_create_nodes(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>**,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>**)
pub fn stub_46ec78() -> ! {
    todo!("0x46ec78 __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE15_M_create_nodesEPPS7_SB_")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::deque(std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>> const&)")]
// 0x46ed6c — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EEC2ERKS9_
// was: std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::deque(std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>> const&)
pub fn stub_46ed6c() -> ! {
    todo!("0x46ed6c __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EEC2ERKS9_")
}

#[doc(alias = "std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>>(std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::__false_type)")]
// 0x46ee90 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEERKS8_PS9_ES0_IS8_RS8_PS8_EET0_T_SH_SG_St12__false_type
// was: std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*> std::__uninitialized_copy_aux<std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*>>(std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>*>,std::__false_type)
pub fn stub_46ee90() -> ! {
    todo!("0x46ee90 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEERKS8_PS9_ES0_IS8_RS8_PS8_EET0_T_SH_SG_St12__false_type")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::~thread_specific_ptr()")]
// 0x46f158 — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEED2Ev
// was: boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::~thread_specific_ptr()
pub fn stub_46f158() -> ! {
    todo!("0x46f158 __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEED2Ev")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()")]
// 0x46f24c — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataD1Ev
// was: boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()
pub fn stub_46f24c() -> ! {
    todo!("0x46f24c __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataD1Ev")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()")]
// 0x46f250 — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataD0Ev
// was: boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::~delete_data()
pub fn stub_46f250() -> ! {
    todo!("0x46f250 __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataD0Ev")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::operator()(void *)")]
// 0x46f254 — __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataclEPv
// was: boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data::operator()(void *)
pub fn stub_46f254() -> ! {
    todo!("0x46f254 __ZN5boost19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataclEPv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>(boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>)")]
// 0x46f260 — __ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS9_EEEET_T0_
// was: boost::detail::shared_count::shared_count<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>(boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>)
pub fn stub_46f260() -> ! {
    todo!("0x46f260 __ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS9_EEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()")]
// 0x46f358 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()
pub fn stub_46f358() -> ! {
    todo!("0x46f358 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()")]
// 0x46f35c — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::~sp_counted_impl_pd()
pub fn stub_46f35c() -> ! {
    todo!("0x46f35c __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::dispose(void)")]
// 0x46f360 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::dispose(void)
pub fn stub_46f360() -> ! {
    todo!("0x46f360 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_deleter(std::type_info const&)")]
// 0x46f370 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_deleter(std::type_info const&)
pub fn stub_46f370() -> ! {
    todo!("0x46f370 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_untyped_deleter(void)")]
// 0x46f388 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::DataModel::GenericJob *>::delete_data>>::get_untyped_deleter(void)
pub fn stub_46f388() -> ! {
    todo!("0x46f388 __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIPN3RBX9DataModel10GenericJobEE11delete_dataENS0_14do_heap_deleteIS8_EEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)")]
// 0x46f6b0 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)
pub fn stub_46f6b0() -> ! {
    todo!("0x46f6b0 __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::release(void)")]
// 0x46feb8 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE7releaseEv
// was: boost::thread_specific_ptr<RBX::Security::Context>::release(void)
pub fn stub_46feb8() -> ! {
    todo!("0x46feb8 __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE7releaseEv")
}

#[doc(alias = "RBX::DataModel::GenericJob::GenericJob(rbx_core::SharedPtr<RBX::DataModel>,char const*,RBX::DataModelJob::TaskType)")]
// 0x46ff84 — __ZN3RBX9DataModel10GenericJobC2EN5boost10shared_ptrIS0_EEPKcNS_12DataModelJob8TaskTypeE
// was: RBX::DataModel::GenericJob::GenericJob(boost::shared_ptr<RBX::DataModel>,char const*,RBX::DataModelJob::TaskType)
pub fn stub_46ff84() -> ! {
    todo!("0x46ff84 __ZN3RBX9DataModel10GenericJobC2EN5boost10shared_ptrIS0_EEPKcNS_12DataModelJob8TaskTypeE")
}

#[doc(alias = "RBX::DataModel::GenericJob::step(boost::function<void ()(RBX::DataModel*)> &)")]
// 0x470818 — __ZN3RBX9DataModel10GenericJob4stepERN5boost8functionIFvPS0_EEE
// was: RBX::DataModel::GenericJob::step(boost::function<void ()(RBX::DataModel*)> &)
pub fn stub_470818() -> ! {
    todo!("0x470818 __ZN3RBX9DataModel10GenericJob4stepERN5boost8functionIFvPS0_EEE")
}

#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~deque()")]
// 0x4708e0 — __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EED2Ev
// was: std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~deque()
pub fn stub_4708e0() -> ! {
    todo!("0x4708e0 __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EED2Ev")
}

#[doc(alias = "std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~_Deque_base()")]
// 0x4709c8 — __ZNSt11_Deque_baseIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EED2Ev
// was: std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::~_Deque_base()
pub fn stub_4709c8() -> ! {
    todo!("0x4709c8 __ZNSt11_Deque_baseIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EED2Ev")
}

#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::deque(std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>> const&)")]
// 0x4709f8 — __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EEC2ERKSC_
// was: std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>>::deque(std::deque<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>,std::allocator<rbx::implementation::timestamped_safe_queue_item<boost::function<void ()(RBX::DataModel *)>>>> const&)
pub fn stub_4709f8() -> ! {
    todo!("0x4709f8 __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN5boost8functionIFvPN3RBX9DataModelEEEEEESaISA_EEC2ERKSC_")
}

#[doc(alias = "RBX::DataModelJob::DataModelJob(char const*,RBX::DataModelJob::TaskType,bool,rbx_core::SharedPtr<RBX::DataModelArbiter>,RBX::Time::Interval)")]
// 0x4729dc — __ZN3RBX12DataModelJobC2EPKcNS0_8TaskTypeEbN5boost10shared_ptrINS_16DataModelArbiterEEENS_4Time8IntervalE
// was: RBX::DataModelJob::DataModelJob(char const*,RBX::DataModelJob::TaskType,bool,boost::shared_ptr<RBX::DataModelArbiter>,RBX::Time::Interval)
pub fn stub_4729dc() -> ! {
    todo!("0x4729dc __ZN3RBX12DataModelJobC2EPKcNS0_8TaskTypeEbN5boost10shared_ptrINS_16DataModelArbiterEEENS_4Time8IntervalE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Limits::Counter>::shared_ptr<RBX::Limits::Counter>(RBX::Limits::Counter *)")]
// 0x473d54 — __ZN5boost10shared_ptrIN3RBX6Limits7CounterEEC2IS3_EEPT_
// was: boost::shared_ptr<RBX::Limits::Counter>::shared_ptr<RBX::Limits::Counter>(RBX::Limits::Counter *)
pub fn stub_473d54() -> ! {
    todo!("0x473d54 __ZN5boost10shared_ptrIN3RBX6Limits7CounterEEC2IS3_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Limits::Counter>(RBX::Limits::Counter *)")]
// 0x473e2c — __ZN5boost6detail12shared_countC2IN3RBX6Limits7CounterEEEPT_
// was: boost::detail::shared_count::shared_count<RBX::Limits::Counter>(RBX::Limits::Counter *)
pub fn stub_473e2c() -> ! {
    todo!("0x473e2c __ZN5boost6detail12shared_countC2IN3RBX6Limits7CounterEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()")]
// 0x473f18 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()
pub fn stub_473f18() -> ! {
    todo!("0x473f18 __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()")]
// 0x473f1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED0Ev
// was: boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()
pub fn stub_473f1c() -> ! {
    todo!("0x473f1c __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::dispose(void)")]
// 0x473f20 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::dispose(void)
pub fn stub_473f20() -> ! {
    todo!("0x473f20 __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_deleter(std::type_info const&)")]
// 0x473f30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_deleter(std::type_info const&)
pub fn stub_473f30() -> ! {
    todo!("0x473f30 __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_untyped_deleter(void)")]
// 0x473f34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_untyped_deleter(void)
pub fn stub_473f34() -> ! {
    todo!("0x473f34 __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::DebrisService::addItem(rbx_core::SharedPtr<RBX::Instance>,double)")]
// 0x477264 — __ZN3RBX13DebrisService7addItemEN5boost10shared_ptrINS_8InstanceEEEd
// was: RBX::DebrisService::addItem(boost::shared_ptr<RBX::Instance>,double)
pub fn stub_477264() -> ! {
    todo!("0x477264 __ZN3RBX13DebrisService7addItemEN5boost10shared_ptrINS_8InstanceEEEd")
}

#[doc(alias = "cleanup(rbx_core::WeakPtr<RBX::Instance>)")]
// 0x477738 — __ZL7cleanupN5boost8weak_ptrIN3RBX8InstanceEEE
// was: cleanup(boost::weak_ptr<RBX::Instance>)
pub fn stub_477738() -> ! {
    todo!("0x477738 __ZL7cleanupN5boost8weak_ptrIN3RBX8InstanceEEE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::~BoundFuncDesc()")]
// 0x477a38 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::~BoundFuncDesc()
pub fn stub_477a38() -> ! {
    todo!("0x477a38 __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EED1Ev")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list_av_1<rbx_core::WeakPtr<RBX::Instance>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance>>(void (*)(rbx_core::WeakPtr<RBX::Instance>),rbx_core::WeakPtr<RBX::Instance>)")]
// 0x477b90 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX8InstanceEEES4_EENS_3_bi6bind_tIT_PFS7_T0_ENS5_9list_av_1IT1_E4typeEEESA_SC_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list_av_1<boost::weak_ptr<RBX::Instance>>::type> boost::bind<void,boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>>(void (*)(boost::weak_ptr<RBX::Instance>),boost::weak_ptr<RBX::Instance>)
pub fn stub_477b90() -> ! {
    todo!("0x477b90 __ZN5boost4bindIvNS_8weak_ptrIN3RBX8InstanceEEES4_EENS_3_bi6bind_tIT_PFS7_T0_ENS5_9list_av_1IT1_E4typeEEESA_SC_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TimerService>::operator=(rbx_core::SharedPtr<RBX::TimerService> const&)")]
// 0x477d30 — __ZN5boost10shared_ptrIN3RBX12TimerServiceEEaSERKS3_
// was: boost::shared_ptr<RBX::TimerService>::operator=(boost::shared_ptr<RBX::TimerService> const&)
pub fn stub_477d30() -> ! {
    todo!("0x477d30 __ZN5boost10shared_ptrIN3RBX12TimerServiceEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TimerService> RBX::shared_from<RBX::TimerService>(RBX::TimerService*)")]
// 0x477d68 — __ZN3RBX11shared_fromINS_12TimerServiceEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::TimerService> RBX::shared_from<RBX::TimerService>(RBX::TimerService*)
pub fn stub_477d68() -> ! {
    todo!("0x477d68 __ZN3RBX11shared_fromINS_12TimerServiceEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::push_back(rbx_core::WeakPtr<RBX::Instance> const&)")]
// 0x4785a0 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE9push_backERKS4_
// was: std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::push_back(boost::weak_ptr<RBX::Instance> const&)
pub fn stub_4785a0() -> ! {
    todo!("0x4785a0 __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE9push_backERKS4_")
}

#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_push_back_aux(rbx_core::WeakPtr<RBX::Instance> const&)")]
// 0x478630 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE16_M_push_back_auxERKS4_
// was: std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_push_back_aux(boost::weak_ptr<RBX::Instance> const&)
pub fn stub_478630() -> ! {
    todo!("0x478630 __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE16_M_push_back_auxERKS4_")
}

#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_reserve_map_at_back(unsigned long)")]
// 0x478814 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE22_M_reserve_map_at_backEm
// was: std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_reserve_map_at_back(unsigned long)
pub fn stub_478814() -> ! {
    todo!("0x478814 __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE22_M_reserve_map_at_backEm")
}

#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_reallocate_map(unsigned long,bool)")]
// 0x478830 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE17_M_reallocate_mapEmb
// was: std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_reallocate_map(unsigned long,bool)
pub fn stub_478830() -> ! {
    todo!("0x478830 __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::_Deque_base<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_allocate_map(unsigned long)")]
// 0x478908 — __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE15_M_allocate_mapEm
// was: std::_Deque_base<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_allocate_map(unsigned long)
pub fn stub_478908() -> ! {
    todo!("0x478908 __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE15_M_allocate_mapEm")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")]
// 0x478920 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
pub fn stub_478920() -> ! {
    todo!("0x478920 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>)")]
// 0x478a4c — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>)
pub fn stub_478a4c() -> ! {
    todo!("0x478a4c __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x478b84 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_478b84() -> ! {
    todo!("0x478b84 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x478ba0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_478ba0() -> ! {
    todo!("0x478ba0 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")]
// 0x478bb4 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS5_5list1INS5_5valueISA_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const
pub fn stub_478bb4() -> ! {
    todo!("0x478bb4 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS5_5list1INS5_5valueISA_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x478cd4 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS5_5list1INS5_5valueISA_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_478cd4() -> ! {
    todo!("0x478cd4 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS5_5list1INS5_5valueISA_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Instance>) &,boost::_bi::list0 &,int)")]
// 0x478e50 — __ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX8InstanceEEEEEEclIPFvS6_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>::operator()<void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Instance>) &,boost::_bi::list0 &,int)
pub fn stub_478e50() -> ! {
    todo!("0x478e50 __ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX8InstanceEEEEEEclIPFvS6_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x478f60 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEE12manage_smallERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_478f60() -> ! {
    todo!("0x478f60 __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueIS8_EEEEEEE12manage_smallERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>>::list1(boost::_bi::value<rbx_core::WeakPtr<RBX::Instance>>)")]
// 0x479038 — __ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX8InstanceEEEEEEC2ES7_
// was: boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::Instance>>>::list1(boost::_bi::value<boost::weak_ptr<RBX::Instance>>)
pub fn stub_479038() -> ! {
    todo!("0x479038 __ZN5boost3_bi5list1INS0_5valueINS_8weak_ptrIN3RBX8InstanceEEEEEEC2ES7_")
}

#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::pop_front(void)")]
// 0x479180 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE9pop_frontEv
// was: std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::pop_front(void)
pub fn stub_479180() -> ! {
    todo!("0x479180 __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE9pop_frontEv")
}

#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_pop_front_aux(void)")]
// 0x4791ac — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE16_M_pop_front_auxEv
// was: std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_pop_front_aux(void)
pub fn stub_4791ac() -> ! {
    todo!("0x4791ac __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE16_M_pop_front_auxEv")
}

#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::deque(std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>> const&)")]
// 0x4791d8 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EEC2ERKS6_
// was: std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::deque(std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>> const&)
pub fn stub_4791d8() -> ! {
    todo!("0x4791d8 __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EEC2ERKS6_")
}

#[doc(alias = "std::_Deque_base<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::~_Deque_base()")]
// 0x4792fc — __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EED2Ev
// was: std::_Deque_base<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::~_Deque_base()
pub fn stub_4792fc() -> ! {
    todo!("0x4792fc __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EED2Ev")
}

#[doc(alias = "std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance>&,rbx_core::WeakPtr<RBX::Instance>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance> const&,rbx_core::WeakPtr<RBX::Instance> const*>,std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance>&,rbx_core::WeakPtr<RBX::Instance>*>>(std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance> const&,rbx_core::WeakPtr<RBX::Instance> const*>,std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance> const&,rbx_core::WeakPtr<RBX::Instance> const*>,std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance>&,rbx_core::WeakPtr<RBX::Instance>*>,std::__false_type)")]
// 0x479328 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost8weak_ptrIN3RBX8InstanceEEERKS5_PS6_ES0_IS5_RS5_PS5_EET0_T_SE_SD_St12__false_type
// was: std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*> std::__uninitialized_copy_aux<std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance> const&,boost::weak_ptr<RBX::Instance> const*>,std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*>>(std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance> const&,boost::weak_ptr<RBX::Instance> const*>,std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance> const&,boost::weak_ptr<RBX::Instance> const*>,std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*>,std::__false_type)
pub fn stub_479328() -> ! {
    todo!("0x479328 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost8weak_ptrIN3RBX8InstanceEEERKS5_PS6_ES0_IS5_RS5_PS5_EET0_T_SE_SD_St12__false_type")
}

#[doc(alias = "std::_Deque_base<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_initialize_map(unsigned long)")]
// 0x479510 — __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE17_M_initialize_mapEm
// was: std::_Deque_base<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_initialize_map(unsigned long)
pub fn stub_479510() -> ! {
    todo!("0x479510 __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_create_nodes(rbx_core::WeakPtr<RBX::Instance>**,rbx_core::WeakPtr<RBX::Instance>**)")]
// 0x479668 — __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE15_M_create_nodesEPPS4_S8_
// was: std::_Deque_base<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_create_nodes(boost::weak_ptr<RBX::Instance>**,boost::weak_ptr<RBX::Instance>**)
pub fn stub_479668() -> ! {
    todo!("0x479668 __ZNSt11_Deque_baseIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE15_M_create_nodesEPPS4_S8_")
}

#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::~deque()")]
// 0x47975c — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EED2Ev
// was: std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::~deque()
pub fn stub_47975c() -> ! {
    todo!("0x47975c __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EED2Ev")
}

#[doc(alias = "std::deque<rbx_core::WeakPtr<RBX::Instance>,std::allocator<rbx_core::WeakPtr<RBX::Instance>>>::_M_destroy_data_aux(std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance>&,rbx_core::WeakPtr<RBX::Instance>*>,std::_Deque_iterator<rbx_core::WeakPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::Instance>&,rbx_core::WeakPtr<RBX::Instance>*>)")]
// 0x479844 — __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE19_M_destroy_data_auxESt15_Deque_iteratorIS4_RS4_PS4_ESA_
// was: std::deque<boost::weak_ptr<RBX::Instance>,std::allocator<boost::weak_ptr<RBX::Instance>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*>,std::_Deque_iterator<boost::weak_ptr<RBX::Instance>,boost::weak_ptr<RBX::Instance>&,boost::weak_ptr<RBX::Instance>*>)
pub fn stub_479844() -> ! {
    todo!("0x479844 __ZNSt5dequeIN5boost8weak_ptrIN3RBX8InstanceEEESaIS4_EE19_M_destroy_data_auxESt15_Deque_iteratorIS4_RS4_PS4_ESA_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::BoundFuncDesc(void (RBX::DebrisService::*)(rbx_core::SharedPtr<RBX::Instance>,double),char const*,char const*,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x479e30 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EEC2EMS2_FvS6_dEPKcSC_SC_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::BoundFuncDesc(void (RBX::DebrisService::*)(boost::shared_ptr<RBX::Instance>,double),char const*,char const*,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_479e30() -> ! {
    todo!("0x479e30 __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EEC2EMS2_FvS6_dEPKcSC_SC_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x47a050 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// was: RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
pub fn stub_47a050() -> ! {
    todo!("0x47a050 __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::~BoundFuncDesc()")]
// 0x47a09c — __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::~BoundFuncDesc()
pub fn stub_47a09c() -> ! {
    todo!("0x47a09c __ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x47a1c8 — __ZNK3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(boost::shared_ptr<RBX::Instance>,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_47a1c8() -> ! {
    todo!("0x47a1c8 __ZNK3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvN5boost10shared_ptrINS_8InstanceEEEdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DebrisService,void (RBX::DebrisService::*)(rbx_core::SharedPtr<RBX::Instance>,double),rbx_core::SharedPtr<RBX::Instance>,double,void>::call(RBX::DebrisService*,void (RBX::DebrisService::*)(rbx_core::SharedPtr<RBX::Instance>,double),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,double const&)")]
// 0x47a2cc — __ZN3RBX10Reflection11Call2HelperINS_13DebrisServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEdES6_dvE4callEPS2_S8_RNS0_7VariantERKS6_RKd
// was: RBX::Reflection::Call2Helper<RBX::DebrisService,void (RBX::DebrisService::*)(boost::shared_ptr<RBX::Instance>,double),boost::shared_ptr<RBX::Instance>,double,void>::call(RBX::DebrisService*,void (RBX::DebrisService::*)(boost::shared_ptr<RBX::Instance>,double),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,double const&)
pub fn stub_47a2cc() -> ! {
    todo!("0x47a2cc __ZN3RBX10Reflection11Call2HelperINS_13DebrisServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEdES6_dvE4callEPS2_S8_RNS0_7VariantERKS6_RKd")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()")]
// 0x47e018 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()
pub fn stub_47e018() -> ! {
    todo!("0x47e018 __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<DummyArbiter>::~shared_ptr()")]
// 0x47e640 — __ZN5boost10shared_ptrI12DummyArbiterED1Ev
// was: boost::shared_ptr<DummyArbiter>::~shared_ptr()
pub fn stub_47e640() -> ! {
    todo!("0x47e640 __ZN5boost10shared_ptrI12DummyArbiterED1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskSchedulerSettings> RBX::Creatable<RBX::Instance>::create<RBX::TaskSchedulerSettings>(void)")]
// 0x4828e0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_21TaskSchedulerSettingsEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::TaskSchedulerSettings> RBX::Creatable<RBX::Instance>::create<RBX::TaskSchedulerSettings>(void)
pub fn stub_4828e0() -> ! {
    todo!("0x4828e0 __ZN3RBX9CreatableINS_8InstanceEE6createINS_21TaskSchedulerSettingsEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x482990 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_482990() -> ! {
    todo!("0x482990 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x482998 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_482998() -> ! {
    todo!("0x482998 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4829b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4829b0() -> ! {
    todo!("0x4829b0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x482c00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_482c00() -> ! {
    todo!("0x482c00 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BlockMesh> RBX::Creatable<RBX::Instance>::create<RBX::BlockMesh>(void)")]
// 0x482c78 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9BlockMeshEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::BlockMesh> RBX::Creatable<RBX::Instance>::create<RBX::BlockMesh>(void)
pub fn stub_482c78() -> ! {
    todo!("0x482c78 __ZN3RBX9CreatableINS_8InstanceEE6createINS_9BlockMeshEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BlockMesh>::shared_ptr<RBX::BlockMesh,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4831f4 — __ZN5boost10shared_ptrIN3RBX9BlockMeshEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::BlockMesh>::shared_ptr<RBX::BlockMesh,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4831f4() -> ! {
    todo!("0x4831f4 __ZN5boost10shared_ptrIN3RBX9BlockMeshEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4832c0 — __ZN5boost6detail12shared_countC2IPN3RBX9BlockMeshENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4832c0() -> ! {
    todo!("0x4832c0 __ZN5boost6detail12shared_countC2IPN3RBX9BlockMeshENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4833c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4833c8() -> ! {
    todo!("0x4833c8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4833d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4833d0() -> ! {
    todo!("0x4833d0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4833f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4833f0() -> ! {
    todo!("0x4833f0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x483408 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_483408() -> ! {
    todo!("0x483408 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::destroy_content(boost::integral_constant<bool,true> const&)")]
// 0x4834f8 — __ZN5boost15circular_bufferIdSaIdEE15destroy_contentERKNS_17integral_constantIbLb1EEE
// was: boost::circular_buffer<double,std::allocator<double>>::destroy_content(boost::integral_constant<bool,true> const&)
pub fn stub_4834f8() -> ! {
    todo!("0x4834f8 __ZN5boost15circular_bufferIdSaIdEE15destroy_contentERKNS_17integral_constantIbLb1EEE")
}

#[doc(alias = "rbx_core::SharedPtr<DummyJob>::shared_ptr<DummyJob>(DummyJob *)")]
// 0x4839f0 — __ZN5boost10shared_ptrI8DummyJobEC2IS1_EEPT_
// was: boost::shared_ptr<DummyJob>::shared_ptr<DummyJob>(DummyJob *)
pub fn stub_4839f0() -> ! {
    todo!("0x4839f0 __ZN5boost10shared_ptrI8DummyJobEC2IS1_EEPT_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<DummyJob,DummyJob>(rbx_core::SharedPtr<DummyJob> const*,DummyJob *)const")]
// 0x483ad8 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerI8DummyJobS6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<DummyJob,DummyJob>(boost::shared_ptr<DummyJob> const*,DummyJob *)const
pub fn stub_483ad8() -> ! {
    todo!("0x483ad8 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerI8DummyJobS6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<DummyJob>(DummyJob *)")]
// 0x483bbc — __ZN5boost6detail12shared_countC2I8DummyJobEEPT_
// was: boost::detail::shared_count::shared_count<DummyJob>(DummyJob *)
pub fn stub_483bbc() -> ! {
    todo!("0x483bbc __ZN5boost6detail12shared_countC2I8DummyJobEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::~sp_counted_impl_p()")]
// 0x483cb4 — __ZN5boost6detail17sp_counted_impl_pI8DummyJobED1Ev
// was: boost::detail::sp_counted_impl_p<DummyJob>::~sp_counted_impl_p()
pub fn stub_483cb4() -> ! {
    todo!("0x483cb4 __ZN5boost6detail17sp_counted_impl_pI8DummyJobED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::~sp_counted_impl_p()")]
// 0x483cb8 — __ZN5boost6detail17sp_counted_impl_pI8DummyJobED0Ev
// was: boost::detail::sp_counted_impl_p<DummyJob>::~sp_counted_impl_p()
pub fn stub_483cb8() -> ! {
    todo!("0x483cb8 __ZN5boost6detail17sp_counted_impl_pI8DummyJobED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::dispose(void)")]
// 0x483cbc — __ZN5boost6detail17sp_counted_impl_pI8DummyJobE7disposeEv
// was: boost::detail::sp_counted_impl_p<DummyJob>::dispose(void)
pub fn stub_483cbc() -> ! {
    todo!("0x483cbc __ZN5boost6detail17sp_counted_impl_pI8DummyJobE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::get_deleter(std::type_info const&)")]
// 0x483ccc — __ZN5boost6detail17sp_counted_impl_pI8DummyJobE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<DummyJob>::get_deleter(std::type_info const&)
pub fn stub_483ccc() -> ! {
    todo!("0x483ccc __ZN5boost6detail17sp_counted_impl_pI8DummyJobE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::get_untyped_deleter(void)")]
// 0x483cd0 — __ZN5boost6detail17sp_counted_impl_pI8DummyJobE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<DummyJob>::get_untyped_deleter(void)
pub fn stub_483cd0() -> ! {
    todo!("0x483cd0 __ZN5boost6detail17sp_counted_impl_pI8DummyJobE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<DummyArbiter>::shared_ptr<DummyArbiter>(DummyArbiter *)")]
// 0x483cd4 — __ZN5boost10shared_ptrI12DummyArbiterEC2IS1_EEPT_
// was: boost::shared_ptr<DummyArbiter>::shared_ptr<DummyArbiter>(DummyArbiter *)
pub fn stub_483cd4() -> ! {
    todo!("0x483cd4 __ZN5boost10shared_ptrI12DummyArbiterEC2IS1_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<DummyArbiter>(DummyArbiter *)")]
// 0x483da8 — __ZN5boost6detail12shared_countC2I12DummyArbiterEEPT_
// was: boost::detail::shared_count::shared_count<DummyArbiter>(DummyArbiter *)
pub fn stub_483da8() -> ! {
    todo!("0x483da8 __ZN5boost6detail12shared_countC2I12DummyArbiterEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::~sp_counted_impl_p()")]
// 0x483e94 — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterED1Ev
// was: boost::detail::sp_counted_impl_p<DummyArbiter>::~sp_counted_impl_p()
pub fn stub_483e94() -> ! {
    todo!("0x483e94 __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::~sp_counted_impl_p()")]
// 0x483e98 — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterED0Ev
// was: boost::detail::sp_counted_impl_p<DummyArbiter>::~sp_counted_impl_p()
pub fn stub_483e98() -> ! {
    todo!("0x483e98 __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::dispose(void)")]
// 0x483e9c — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE7disposeEv
// was: boost::detail::sp_counted_impl_p<DummyArbiter>::dispose(void)
pub fn stub_483e9c() -> ! {
    todo!("0x483e9c __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::get_deleter(std::type_info const&)")]
// 0x483ea8 — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<DummyArbiter>::get_deleter(std::type_info const&)
pub fn stub_483ea8() -> ! {
    todo!("0x483ea8 __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::get_untyped_deleter(void)")]
// 0x483eac — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<DummyArbiter>::get_untyped_deleter(void)
pub fn stub_483eac() -> ! {
    todo!("0x483eac __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE19get_untyped_deleterEv")
}

#[doc(alias = "double RBX::Reflection::ArgHelper::getArg<double,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x487c88 — __ZN3RBX10Reflection9ArgHelper6getArgIdLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: double RBX::Reflection::ArgHelper::getArg<double,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_487c88() -> ! {
    todo!("0x487c88 __ZN3RBX10Reflection9ArgHelper6getArgIdLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "double RBX::Reflection::ArgHelper::getArg<double,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x489b38 — __ZN3RBX10Reflection9ArgHelper6getArgIdLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: double RBX::Reflection::ArgHelper::getArg<double,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_489b38() -> ! {
    todo!("0x489b38 __ZN3RBX10Reflection9ArgHelper6getArgIdLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x48a1ac — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EEC2EMS2_FS7_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_48a1ac() -> ! {
    todo!("0x48a1ac __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EEC2EMS2_FS7_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()")]
// 0x48a2b0 — __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()
pub fn stub_48a2b0() -> ! {
    todo!("0x48a2b0 __ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED0Ev")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4ac0f0 — __ZN5boost6detail12shared_countC2IPN3RBX13BindableEventENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4ac0f0() -> ! {
    todo!("0x4ac0f0 __ZN5boost6detail12shared_countC2IPN3RBX13BindableEventENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4ac1f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13BindableEventENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4ac1f8() -> ! {
    todo!("0x4ac1f8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13BindableEventENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4ac1fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13BindableEventENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4ac1fc() -> ! {
    todo!("0x4ac1fc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13BindableEventENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4ac200 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13BindableEventENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4ac200() -> ! {
    todo!("0x4ac200 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13BindableEventENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4ac220 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13BindableEventENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4ac220() -> ! {
    todo!("0x4ac220 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13BindableEventENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4ac238 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13BindableEventENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::BindableEvent *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4ac238() -> ! {
    todo!("0x4ac238 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13BindableEventENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BindableFunction> RBX::Creatable<RBX::Instance>::create<RBX::BindableFunction>(void)")]
// 0x4ac7dc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16BindableFunctionEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::BindableFunction> RBX::Creatable<RBX::Instance>::create<RBX::BindableFunction>(void)
pub fn stub_4ac7dc() -> ! {
    todo!("0x4ac7dc __ZN3RBX9CreatableINS_8InstanceEE6createINS_16BindableFunctionEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BindableFunction>::shared_ptr<RBX::BindableFunction,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4ad788 — __ZN5boost10shared_ptrIN3RBX16BindableFunctionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::BindableFunction>::shared_ptr<RBX::BindableFunction,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4ad788() -> ! {
    todo!("0x4ad788 __ZN5boost10shared_ptrIN3RBX16BindableFunctionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BindableFunction,RBX::BindableFunction>(rbx_core::SharedPtr<RBX::BindableFunction> const*,RBX::BindableFunction *)const")]
// 0x4ad850 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16BindableFunctionES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BindableFunction,RBX::BindableFunction>(boost::shared_ptr<RBX::BindableFunction> const*,RBX::BindableFunction *)const
pub fn stub_4ad850() -> ! {
    todo!("0x4ad850 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16BindableFunctionES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4ad938 — __ZN5boost6detail12shared_countC2IPN3RBX16BindableFunctionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4ad938() -> ! {
    todo!("0x4ad938 __ZN5boost6detail12shared_countC2IPN3RBX16BindableFunctionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4ada40 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BindableFunctionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4ada40() -> ! {
    todo!("0x4ada40 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BindableFunctionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4ada44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BindableFunctionENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4ada44() -> ! {
    todo!("0x4ada44 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BindableFunctionENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4ada48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BindableFunctionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4ada48() -> ! {
    todo!("0x4ada48 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BindableFunctionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4ada68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BindableFunctionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4ada68() -> ! {
    todo!("0x4ada68 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BindableFunctionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4ada80 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BindableFunctionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::BindableFunction *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4ada80() -> ! {
    todo!("0x4ada80 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BindableFunctionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Animation> RBX::Creatable<RBX::Instance>::create<RBX::Animation>(void)")]
// 0x4ae024 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9AnimationEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Animation> RBX::Creatable<RBX::Instance>::create<RBX::Animation>(void)
pub fn stub_4ae024() -> ! {
    todo!("0x4ae024 __ZN3RBX9CreatableINS_8InstanceEE6createINS_9AnimationEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Animation>::shared_ptr<RBX::Animation,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4ae0d4 — __ZN5boost10shared_ptrIN3RBX9AnimationEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Animation>::shared_ptr<RBX::Animation,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4ae0d4() -> ! {
    todo!("0x4ae0d4 __ZN5boost10shared_ptrIN3RBX9AnimationEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Animation,RBX::Animation>(rbx_core::SharedPtr<RBX::Animation> const*,RBX::Animation *)const")]
// 0x4ae19c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9AnimationES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Animation,RBX::Animation>(boost::shared_ptr<RBX::Animation> const*,RBX::Animation *)const
pub fn stub_4ae19c() -> ! {
    todo!("0x4ae19c __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9AnimationES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4ae284 — __ZN5boost6detail12shared_countC2IPN3RBX9AnimationENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4ae284() -> ! {
    todo!("0x4ae284 __ZN5boost6detail12shared_countC2IPN3RBX9AnimationENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4ae38c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9AnimationENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4ae38c() -> ! {
    todo!("0x4ae38c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9AnimationENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4ae390 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9AnimationENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4ae390() -> ! {
    todo!("0x4ae390 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9AnimationENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4ae394 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9AnimationENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4ae394() -> ! {
    todo!("0x4ae394 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9AnimationENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4ae3b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9AnimationENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4ae3b4() -> ! {
    todo!("0x4ae3b4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9AnimationENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4ae3cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9AnimationENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Animation *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4ae3cc() -> ! {
    todo!("0x4ae3cc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9AnimationENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ObjectValue> RBX::Creatable<RBX::Instance>::create<RBX::ObjectValue>(void)")]
// 0x4ae724 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11ObjectValueEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ObjectValue> RBX::Creatable<RBX::Instance>::create<RBX::ObjectValue>(void)
pub fn stub_4ae724() -> ! {
    todo!("0x4ae724 __ZN3RBX9CreatableINS_8InstanceEE6createINS_11ObjectValueEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ObjectValue>::shared_ptr<RBX::ObjectValue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4ae8f4 — __ZN5boost10shared_ptrIN3RBX11ObjectValueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ObjectValue>::shared_ptr<RBX::ObjectValue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4ae8f4() -> ! {
    todo!("0x4ae8f4 __ZN5boost10shared_ptrIN3RBX11ObjectValueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4ae9bc — __ZN5boost6detail12shared_countC2IPN3RBX11ObjectValueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4ae9bc() -> ! {
    todo!("0x4ae9bc __ZN5boost6detail12shared_countC2IPN3RBX11ObjectValueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StringValue> RBX::Creatable<RBX::Instance>::create<RBX::StringValue>(void)")]
// 0x4aeac4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11StringValueEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::StringValue> RBX::Creatable<RBX::Instance>::create<RBX::StringValue>(void)
pub fn stub_4aeac4() -> ! {
    todo!("0x4aeac4 __ZN3RBX9CreatableINS_8InstanceEE6createINS_11StringValueEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StringValue>::shared_ptr<RBX::StringValue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4aec94 — __ZN5boost10shared_ptrIN3RBX11StringValueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::StringValue>::shared_ptr<RBX::StringValue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4aec94() -> ! {
    todo!("0x4aec94 __ZN5boost10shared_ptrIN3RBX11StringValueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4aed5c — __ZN5boost6detail12shared_countC2IPN3RBX11StringValueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4aed5c() -> ! {
    todo!("0x4aed5c __ZN5boost6detail12shared_countC2IPN3RBX11StringValueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Sparkles> RBX::Creatable<RBX::Instance>::create<RBX::Sparkles>(void)")]
// 0x4af0b0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_8SparklesEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Sparkles> RBX::Creatable<RBX::Instance>::create<RBX::Sparkles>(void)
pub fn stub_4af0b0() -> ! {
    todo!("0x4af0b0 __ZN3RBX9CreatableINS_8InstanceEE6createINS_8SparklesEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Sparkles>::shared_ptr<RBX::Sparkles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4af160 — __ZN5boost10shared_ptrIN3RBX8SparklesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Sparkles>::shared_ptr<RBX::Sparkles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4af160() -> ! {
    todo!("0x4af160 __ZN5boost10shared_ptrIN3RBX8SparklesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Sparkles,RBX::Sparkles>(rbx_core::SharedPtr<RBX::Sparkles> const*,RBX::Sparkles *)const")]
// 0x4af228 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8SparklesES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Sparkles,RBX::Sparkles>(boost::shared_ptr<RBX::Sparkles> const*,RBX::Sparkles *)const
pub fn stub_4af228() -> ! {
    todo!("0x4af228 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8SparklesES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4af310 — __ZN5boost6detail12shared_countC2IPN3RBX8SparklesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4af310() -> ! {
    todo!("0x4af310 __ZN5boost6detail12shared_countC2IPN3RBX8SparklesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4af418 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8SparklesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4af418() -> ! {
    todo!("0x4af418 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8SparklesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4af41c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8SparklesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4af41c() -> ! {
    todo!("0x4af41c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8SparklesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4af420 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8SparklesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4af420() -> ! {
    todo!("0x4af420 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8SparklesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4af440 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8SparklesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4af440() -> ! {
    todo!("0x4af440 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8SparklesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4af458 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8SparklesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Sparkles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4af458() -> ! {
    todo!("0x4af458 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8SparklesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BasicPartInstance> RBX::Creatable<RBX::Instance>::create<RBX::BasicPartInstance>(void)")]
// 0x4af9fc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17BasicPartInstanceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::BasicPartInstance> RBX::Creatable<RBX::Instance>::create<RBX::BasicPartInstance>(void)
pub fn stub_4af9fc() -> ! {
    todo!("0x4af9fc __ZN3RBX9CreatableINS_8InstanceEE6createINS_17BasicPartInstanceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BasicPartInstance>::shared_ptr<RBX::BasicPartInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4afab0 — __ZN5boost10shared_ptrIN3RBX17BasicPartInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::BasicPartInstance>::shared_ptr<RBX::BasicPartInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4afab0() -> ! {
    todo!("0x4afab0 __ZN5boost10shared_ptrIN3RBX17BasicPartInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BasicPartInstance,RBX::BasicPartInstance>(rbx_core::SharedPtr<RBX::BasicPartInstance> const*,RBX::BasicPartInstance *)const")]
// 0x4afb78 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17BasicPartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BasicPartInstance,RBX::BasicPartInstance>(boost::shared_ptr<RBX::BasicPartInstance> const*,RBX::BasicPartInstance *)const
pub fn stub_4afb78() -> ! {
    todo!("0x4afb78 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17BasicPartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4afc60 — __ZN5boost6detail12shared_countC2IPN3RBX17BasicPartInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4afc60() -> ! {
    todo!("0x4afc60 __ZN5boost6detail12shared_countC2IPN3RBX17BasicPartInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4afd68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17BasicPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4afd68() -> ! {
    todo!("0x4afd68 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17BasicPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x4afd6c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17BasicPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_4afd6c() -> ! {
    todo!("0x4afd6c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17BasicPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x4afd70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17BasicPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_4afd70() -> ! {
    todo!("0x4afd70 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17BasicPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4afd90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17BasicPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_4afd90() -> ! {
    todo!("0x4afd90 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17BasicPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x4afda8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17BasicPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::BasicPartInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_4afda8() -> ! {
    todo!("0x4afda8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17BasicPartInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ForceField>::shared_ptr<RBX::ForceField,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x4b019c — __ZN5boost10shared_ptrIN3RBX10ForceFieldEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ForceField>::shared_ptr<RBX::ForceField,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ForceField *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_4b019c() -> ! {
    todo!("0x4b019c __ZN5boost10shared_ptrIN3RBX10ForceFieldEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ForceField,RBX::ForceField>(rbx_core::SharedPtr<RBX::ForceField> const*,RBX::ForceField *)const")]
// 0x4b0264 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ForceFieldES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ForceField,RBX::ForceField>(boost::shared_ptr<RBX::ForceField> const*,RBX::ForceField *)const
pub fn stub_4b0264() -> ! {
    todo!("0x4b0264 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ForceFieldES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}
