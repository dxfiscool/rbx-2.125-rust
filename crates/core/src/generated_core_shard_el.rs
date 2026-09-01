//! core shard EL — 100 core stubs EA-sorted, lowest uncovered 0x9742b8..0x98dd5c (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EK 0x940250).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,std::string)>::slot,boost::function<void ()(std::string,int,std::string)>,3,void ()(std::string,int,std::string)>::~callable()")]
// 0x9742b8 — __ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
pub fn stub_9742b8() -> ! {
    todo!("0x9742b8 __ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,std::string)>::slot,boost::function<void ()(std::string,int,std::string)>,3,void ()(std::string,int,std::string)>::~callable()")]
// 0x9742c4 — __ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
pub fn stub_9742c4() -> ! {
    todo!("0x9742c4 __ZN3rbx8callableINS_7signals6signalIFvSsiSsEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::slot::~slot()")]
// 0x974378 — __ZN3rbx7signals6signalIFvSsiSsEE4slotD1Ev
pub fn stub_974378() -> ! {
    todo!("0x974378 __ZN3rbx7signals6signalIFvSsiSsEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::slot::~slot()")]
// 0x9743d4 — __ZN3rbx7signals6signalIFvSsiSsEE4slotD0Ev
pub fn stub_9743d4() -> ! {
    todo!("0x9743d4 __ZN3rbx7signals6signalIFvSsiSsEE4slotD0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::slot::safe_static_init_mutex(void)")]
// 0x975048 — __ZN3rbx7signals6signalIFvSsEE4slot22safe_static_init_mutexEv
pub fn stub_975048() -> ! {
    todo!("0x975048 __ZN3rbx7signals6signalIFvSsEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::slot::~slot()")]
// 0x975130 — __ZN3rbx7signals6signalIFvSsEE4slotD0Ev
pub fn stub_975130() -> ! {
    todo!("0x975130 __ZN3rbx7signals6signalIFvSsEE4slotD0Ev")
}

#[doc(alias = "boost::date_time::microsec_clock<boost::posix_time::ptime>::create_time(tm * (*)(long const*,tm *))")]
// 0x978ba8 — __ZN5boost9date_time14microsec_clockINS_10posix_time5ptimeEE11create_timeEPFP2tmPKlS6_E
pub fn stub_978ba8() -> ! {
    todo!("0x978ba8 __ZN5boost9date_time14microsec_clockINS_10posix_time5ptimeEE11create_timeEPFP2tmPKlS6_E")
}

#[doc(alias = "boost::date_time::c_time::gmtime(long const*,tm *)")]
// 0x978e10 — __ZN5boost9date_time6c_time6gmtimeEPKlP2tm
pub fn stub_978e10() -> ! {
    todo!("0x978e10 __ZN5boost9date_time6c_time6gmtimeEPKlP2tm")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()")]
// 0x978f28 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED0Ev
pub fn stub_978f28() -> ! {
    todo!("0x978f28 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::rethrow(void)const")]
// 0x978fe8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE7rethrowEv
pub fn stub_978fe8() -> ! {
    todo!("0x978fe8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE7rethrowEv")
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()")]
// 0x979128 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED0Ev
// was: `non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()`
pub fn stub_979128() -> ! {
    todo!("0x979128 __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED0Ev")
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::rethrow(void)const")]
// 0x9791e8 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE7rethrowEv
// was: `virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::rethrow(void)const`
pub fn stub_9791e8() -> ! {
    todo!("0x9791e8 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEE7rethrowEv")
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()")]
// 0x9791f8 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED0Ev
// was: `virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::~clone_impl()`
pub fn stub_9791f8() -> ! {
    todo!("0x9791f8 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEED0Ev")
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()")]
// 0x9792d0 — __ZThn8_N5boost16exception_detail19error_info_injectorISt13runtime_errorED0Ev
// was: `non-virtual thunk to boost::exception_detail::error_info_injector<std::runtime_error>::~error_info_injector()`
pub fn stub_9792d0() -> ! {
    todo!("0x9792d0 __ZThn8_N5boost16exception_detail19error_info_injectorISt13runtime_errorED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_impl(boost::exception_detail::error_info_injector<std::runtime_error> const&)")]
// 0x979390 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEEC1ERKS4_
pub fn stub_979390() -> ! {
    todo!("0x979390 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEEC1ERKS4_")
}

#[doc(alias = "void boost::throw_exception<boost::gregorian::bad_day_of_month>(boost::gregorian::bad_day_of_month const&)")]
// 0x979518 — __ZN5boost15throw_exceptionINS_9gregorian16bad_day_of_monthEEEvRKT_
pub fn stub_979518() -> ! {
    todo!("0x979518 __ZN5boost15throw_exceptionINS_9gregorian16bad_day_of_monthEEEvRKT_")
}

#[doc(alias = "boost::gregorian::bad_day_of_month::~bad_day_of_month()")]
// 0x979668 — __ZN5boost9gregorian16bad_day_of_monthD0Ev
pub fn stub_979668() -> ! {
    todo!("0x979668 __ZN5boost9gregorian16bad_day_of_monthD0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::~clone_impl()")]
// 0x979680 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED0Ev
pub fn stub_979680() -> ! {
    todo!("0x979680 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::rethrow(void)const")]
// 0x979740 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE7rethrowEv
pub fn stub_979740() -> ! {
    todo!("0x979740 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEE7rethrowEv")
}

#[doc(alias = "boost::gregorian::bad_month::~bad_month()")]
// 0x979898 — __ZN5boost9gregorian9bad_monthD1Ev
pub fn stub_979898() -> ! {
    todo!("0x979898 __ZN5boost9gregorian9bad_monthD1Ev")
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
// 0x9798a8 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED0Ev
// was: `non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()`
pub fn stub_9798a8() -> ! {
    todo!("0x9798a8 __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED0Ev")
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::rethrow(void)const")]
// 0x979968 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE7rethrowEv
// was: `virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::rethrow(void)const`
pub fn stub_979968() -> ! {
    todo!("0x979968 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE7rethrowEv")
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()")]
// 0x979978 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED0Ev
// was: `virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::~clone_impl()`
pub fn stub_979978() -> ! {
    todo!("0x979978 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEED0Ev")
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()")]
// 0x979a50 — __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED0Ev
// was: `non-virtual thunk to boost::exception_detail::error_info_injector<boost::gregorian::bad_month>::~error_info_injector()`
pub fn stub_979a50() -> ! {
    todo!("0x979a50 __ZThn8_N5boost16exception_detail19error_info_injectorINS_9gregorian9bad_monthEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_month> const&)")]
// 0x979b10 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS5_
pub fn stub_979b10() -> ! {
    todo!("0x979b10 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS5_")
}

#[doc(alias = "void boost::throw_exception<boost::gregorian::bad_year>(boost::gregorian::bad_year const&)")]
// 0x979ca8 — __ZN5boost15throw_exceptionINS_9gregorian8bad_yearEEEvRKT_
pub fn stub_979ca8() -> ! {
    todo!("0x979ca8 __ZN5boost15throw_exceptionINS_9gregorian8bad_yearEEEvRKT_")
}

#[doc(alias = "boost::gregorian::bad_year::~bad_year()")]
// 0x979df8 — __ZN5boost9gregorian8bad_yearD0Ev
pub fn stub_979df8() -> ! {
    todo!("0x979df8 __ZN5boost9gregorian8bad_yearD0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::~clone_impl()")]
// 0x979e10 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED0Ev
pub fn stub_979e10() -> ! {
    todo!("0x979e10 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::rethrow(void)const")]
// 0x979ed0 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE7rethrowEv
pub fn stub_979ed0() -> ! {
    todo!("0x979ed0 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEE7rethrowEv")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::JointsService>(void)")]
// 0x981b60 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13JointsServiceEEEvv
pub fn stub_981b60() -> ! {
    todo!("0x981b60 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13JointsServiceEEEvv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,std::string)>::slot> &)")]
// 0x9828f0 — __ZN3rbx7signals6signalIFvSsiSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_9828f0() -> ! {
    todo!("0x9828f0 __ZN3rbx7signals6signalIFvSsiSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::mutex(void)")]
// 0x982af8 — __ZN3rbx7signals6signalIFvSsiSsEE5mutexEv
pub fn stub_982af8() -> ! {
    todo!("0x982af8 __ZN3rbx7signals6signalIFvSsiSsEE5mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,std::string)>::safe_static_init_mutex(void)")]
// 0x982c10 — __ZN3rbx7signals6signalIFvSsiSsEE22safe_static_init_mutexEv
pub fn stub_982c10() -> ! {
    todo!("0x982c10 __ZN3rbx7signals6signalIFvSsiSsEE22safe_static_init_mutexEv")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned int>::deref(unsigned int const*)")]
// 0x9842a8 — __ZN3RBX5Stats14TypedStatsItemIjE5derefEPKj
pub fn stub_9842a8() -> ! {
    todo!("0x9842a8 __ZN3RBX5Stats14TypedStatsItemIjE5derefEPKj")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x9842b0 — __ZN3RBX5Stats14TypedStatsItemIjED0Ev
pub fn stub_9842b0() -> ! {
    todo!("0x9842b0 __ZN3RBX5Stats14TypedStatsItemIjED0Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned int>::update(void)")]
// 0x984350 — __ZN3RBX5Stats14TypedStatsItemIjE6updateEv
pub fn stub_984350() -> ! {
    todo!("0x984350 __ZN3RBX5Stats14TypedStatsItemIjE6updateEv")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x9844d0 — __ZThn32_N3RBX5Stats14TypedStatsItemIjED0Ev
// was: `non-virtual thunk to RBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()`
pub fn stub_9844d0() -> ! {
    todo!("0x9844d0 __ZThn32_N3RBX5Stats14TypedStatsItemIjED0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()")]
// 0x984578 — __ZThn36_N3RBX5Stats14TypedStatsItemIjED0Ev
// was: `non-virtual thunk to RBX::Stats::TypedStatsItem<unsigned int>::~TypedStatsItem()`
pub fn stub_984578() -> ! {
    todo!("0x984578 __ZThn36_N3RBX5Stats14TypedStatsItemIjED0Ev")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<unsigned long long>(char const*,unsigned long long const&)")]
// 0x984b88 — __ZN3RBX5Stats4Item20createBoundChildItemIyEEPS1_PKcRKT_
pub fn stub_984b88() -> ! {
    todo!("0x984b88 __ZN3RBX5Stats4Item20createBoundChildItemIyEEPS1_PKcRKT_")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<float>(char const*,float const&)")]
// 0x9851e0 — __ZN3RBX5Stats4Item20createBoundChildItemIfEEPS1_PKcRKT_
pub fn stub_9851e0() -> ! {
    todo!("0x9851e0 __ZN3RBX5Stats4Item20createBoundChildItemIfEEPS1_PKcRKT_")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<float>::~TypedStatsItem()")]
// 0x985ad8 — __ZThn36_N3RBX5Stats14TypedStatsItemIfED1Ev
// was: `non-virtual thunk to RBX::Stats::TypedStatsItem<float>::~TypedStatsItem()`
pub fn stub_985ad8() -> ! {
    todo!("0x985ad8 __ZThn36_N3RBX5Stats14TypedStatsItemIfED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<float>::~TypedStatsItem()")]
// 0x985ae8 — __ZN3RBX5Stats14TypedStatsItemIfED2Ev
pub fn stub_985ae8() -> ! {
    todo!("0x985ae8 __ZN3RBX5Stats14TypedStatsItemIfED2Ev")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<float const&,float const& (*)(float const*),boost::_bi::list1<boost::_bi::value<float const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x985cc0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKfPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
pub fn stub_985cc0() -> ! {
    todo!("0x985cc0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKfPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<float const&,float const& (*)(float const*),boost::_bi::list1<boost::_bi::value<float const*>>>,float>::invoke(boost::detail::function::function_buffer &)")]
// 0x985d20 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKfPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEfE6invokeERNS1_15function_bufferE
pub fn stub_985d20() -> ! {
    todo!("0x985d20 __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKfPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEfE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<double>::deref(double const*)")]
// 0x986000 — __ZN3RBX5Stats14TypedStatsItemIdE5derefEPKd
pub fn stub_986000() -> ! {
    todo!("0x986000 __ZN3RBX5Stats14TypedStatsItemIdE5derefEPKd")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<double>::update(void)")]
// 0x986008 — __ZN3RBX5Stats14TypedStatsItemIdE6updateEv
pub fn stub_986008() -> ! {
    todo!("0x986008 __ZN3RBX5Stats14TypedStatsItemIdE6updateEv")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
// 0x986188 — __ZThn36_N3RBX5Stats14TypedStatsItemIdED0Ev
// was: `non-virtual thunk to RBX::Stats::TypedStatsItem<double>::~TypedStatsItem()`
pub fn stub_986188() -> ! {
    todo!("0x986188 __ZThn36_N3RBX5Stats14TypedStatsItemIdED0Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<bool>::deref(bool const*)")]
// 0x986258 — __ZN3RBX5Stats14TypedStatsItemIbE5derefEPKb
pub fn stub_986258() -> ! {
    todo!("0x986258 __ZN3RBX5Stats14TypedStatsItemIbE5derefEPKb")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<bool>::update(void)")]
// 0x986260 — __ZN3RBX5Stats14TypedStatsItemIbE6updateEv
pub fn stub_986260() -> ! {
    todo!("0x986260 __ZN3RBX5Stats14TypedStatsItemIbE6updateEv")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()")]
// 0x9863e0 — __ZThn32_N3RBX5Stats14TypedStatsItemIbED0Ev
// was: `non-virtual thunk to RBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()`
pub fn stub_9863e0() -> ! {
    todo!("0x9863e0 __ZThn32_N3RBX5Stats14TypedStatsItemIbED0Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long long>::deref(unsigned long long const*)")]
// 0x9864b0 — __ZN3RBX5Stats14TypedStatsItemIyE5derefEPKy
pub fn stub_9864b0() -> ! {
    todo!("0x9864b0 __ZN3RBX5Stats14TypedStatsItemIyE5derefEPKy")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x9864b8 — __ZN3RBX5Stats14TypedStatsItemIyED0Ev
pub fn stub_9864b8() -> ! {
    todo!("0x9864b8 __ZN3RBX5Stats14TypedStatsItemIyED0Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long long>::update(void)")]
// 0x986558 — __ZN3RBX5Stats14TypedStatsItemIyE6updateEv
pub fn stub_986558() -> ! {
    todo!("0x986558 __ZN3RBX5Stats14TypedStatsItemIyE6updateEv")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x9866d8 — __ZThn32_N3RBX5Stats14TypedStatsItemIyED0Ev
// was: `non-virtual thunk to RBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()`
pub fn stub_9866d8() -> ! {
    todo!("0x9866d8 __ZThn32_N3RBX5Stats14TypedStatsItemIyED0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()")]
// 0x986780 — __ZThn36_N3RBX5Stats14TypedStatsItemIyED0Ev
// was: `non-virtual thunk to RBX::Stats::TypedStatsItem<unsigned long long>::~TypedStatsItem()`
pub fn stub_986780() -> ! {
    todo!("0x986780 __ZThn36_N3RBX5Stats14TypedStatsItemIyED0Ev")
}

#[doc(alias = "boost::iostreams::stream<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>>::stream<char const*,unsigned int>(char const* &,unsigned int const&,boost::disable_if<boost::is_same<char const*,boost::iostreams::basic_array_source<char>>,void>::type *)")]
// 0x98b63c — __ZN5boost9iostreams6streamINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcEEC1IPKcjEERT_RKT0_PNS_10disable_ifINS_7is_sameISB_S3_EEvE4typeE
pub fn stub_98b63c() -> ! {
    todo!("0x98b63c __ZN5boost9iostreams6streamINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcEEC1IPKcjEERT_RKT0_PNS_10disable_ifINS_7is_sameISB_S3_EEvE4typeE")
}

#[doc(alias = "boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_streambuf()")]
// 0x98b844 — __ZN5boost9iostreams19filtering_streambufINS0_5inputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev
pub fn stub_98b844() -> ! {
    todo!("0x98b844 __ZN5boost9iostreams19filtering_streambufINS0_5inputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev")
}

#[doc(alias = "boost::iostreams::stream<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>>::~stream()")]
// 0x98b850 — __ZN5boost9iostreams6streamINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcEED1Ev
pub fn stub_98b850() -> ! {
    todo!("0x98b850 __ZN5boost9iostreams6streamINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcEED1Ev")
}

#[doc(alias = "boost::iostreams::stream<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>>::stream<char *,unsigned int>(char * const&,unsigned int const&,boost::disable_if<boost::is_same<char *,boost::iostreams::basic_array_source<char>>,void>::type *)")]
// 0x98b930 — __ZN5boost9iostreams6streamINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcEEC1IPcjEERKT_RKT0_PNS_10disable_ifINS_7is_sameISA_S3_EEvE4typeE
pub fn stub_98b930() -> ! {
    todo!("0x98b930 __ZN5boost9iostreams6streamINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcEEC1IPcjEERKT_RKT0_PNS_10disable_ifINS_7is_sameISA_S3_EEvE4typeE")
}

#[doc(alias = "boost::unique_lock<boost::mutex>::~unique_lock()")]
// 0x98bc50 — __ZN5boost11unique_lockINS_5mutexEED1Ev
pub fn stub_98bc50() -> ! {
    todo!("0x98bc50 __ZN5boost11unique_lockINS_5mutexEED1Ev")
}

#[doc(alias = "void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::push_impl<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>>(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> const&,int,int)")]
// 0x98be30 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_23basic_gzip_decompressorIS7_EEEEvRKT_ii
pub fn stub_98be30() -> ! {
    todo!("0x98be30 __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_23basic_gzip_decompressorIS7_EEEEvRKT_ii")
}

#[doc(alias = "void boost::throw_exception<std::logic_error>(std::logic_error const&)")]
// 0x98c378 — __ZN5boost15throw_exceptionISt11logic_errorEEvRKT_
pub fn stub_98c378() -> ! {
    todo!("0x98c378 __ZN5boost15throw_exceptionISt11logic_errorEEvRKT_")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open_impl(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> const&,int,int)")]
// 0x98c4c8 — __ZN5boost9iostreams13stream_bufferINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES3_NS0_5inputEE9open_implERKS4_ii
pub fn stub_98c4c8() -> ! {
    todo!("0x98c4c8 __ZN5boost9iostreams13stream_bufferINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES3_NS0_5inputEE9open_implERKS4_ii")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()")]
// 0x98c5e8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEED2Ev
pub fn stub_98c5e8() -> ! {
    todo!("0x98c5e8 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEED2Ev")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0x98c78c — __ZN5boost9iostreams13stream_bufferINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES3_NS0_5inputEED1Ev
pub fn stub_98c78c() -> ! {
    todo!("0x98c78c __ZN5boost9iostreams13stream_bufferINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES3_NS0_5inputEED1Ev")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0x98c798 — __ZN5boost9iostreams13stream_bufferINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES3_NS0_5inputEED0Ev
pub fn stub_98c798() -> ! {
    todo!("0x98c798 __ZN5boost9iostreams13stream_bufferINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES3_NS0_5inputEED0Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::imbue(std::locale const&)")]
// 0x98c838 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE5imbueERKSt6locale
pub fn stub_98c838() -> ! {
    todo!("0x98c838 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE5imbueERKSt6locale")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x98c900 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
pub fn stub_98c900() -> ! {
    todo!("0x98c900 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
// 0x98c918 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
pub fn stub_98c918() -> ! {
    todo!("0x98c918 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::sync(void)")]
// 0x98c968 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE4syncEv
pub fn stub_98c968() -> ! {
    todo!("0x98c968 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE4syncEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::underflow(void)")]
// 0x98ca60 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE9underflowEv
pub fn stub_98ca60() -> ! {
    todo!("0x98ca60 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE9underflowEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::pbackfail(int)")]
// 0x98caec — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE9pbackfailEi
pub fn stub_98caec() -> ! {
    todo!("0x98caec __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE9pbackfailEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::overflow(int)")]
// 0x98cc04 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE8overflowEi
pub fn stub_98cc04() -> ! {
    todo!("0x98cc04 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE8overflowEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x98ccb0 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE8set_nextEPNS1_16linked_streambufIcS7_EE
pub fn stub_98ccb0() -> ! {
    todo!("0x98ccb0 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE8set_nextEPNS1_16linked_streambufIcS7_EE")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::close_impl(std::_Ios_Openmode)")]
// 0x98ccb4 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE10close_implESt13_Ios_Openmode
pub fn stub_98ccb4() -> ! {
    todo!("0x98ccb4 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE10close_implESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::auto_close(void)const")]
// 0x98ccdc — __ZNK5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE10auto_closeEv
pub fn stub_98ccdc() -> ! {
    todo!("0x98ccdc __ZNK5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE10auto_closeEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_auto_close(bool)")]
// 0x98cce8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE14set_auto_closeEb
pub fn stub_98cce8() -> ! {
    todo!("0x98cce8 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE14set_auto_closeEb")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::strict_sync(void)")]
// 0x98ccfc — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE11strict_syncEv
pub fn stub_98ccfc() -> ! {
    todo!("0x98ccfc __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE11strict_syncEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_type(void)const")]
// 0x98cdf4 — __ZNK5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE14component_typeEv
pub fn stub_98cdf4() -> ! {
    todo!("0x98cdf4 __ZNK5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE14component_typeEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_impl(void)")]
// 0x98ce04 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE14component_implEv
pub fn stub_98ce04() -> ! {
    todo!("0x98ce04 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE14component_implEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_get_area(void)")]
// 0x98ce08 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE13init_get_areaEv
pub fn stub_98ce08() -> ! {
    todo!("0x98ce08 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE13init_get_areaEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_put_area(void)")]
// 0x98ce14 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE13init_put_areaEv
pub fn stub_98ce14() -> ! {
    todo!("0x98ce14 __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE13init_put_areaEv")
}

#[doc(alias = "int boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char const*,int)")]
// 0x98ce3c — __ZN5boost9iostreams23basic_gzip_decompressorISaIcEE5writeINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci
pub fn stub_98ce3c() -> ! {
    todo!("0x98ce3c __ZN5boost9iostreams23basic_gzip_decompressorISaIcEE5writeINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci")
}

#[doc(alias = "int boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char const*,int)")]
// 0x98d104 — __ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E5writeINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci
pub fn stub_98d104() -> ! {
    todo!("0x98d104 __ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E5writeINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci")
}

#[doc(alias = "void boost::throw_exception<boost::iostreams::gzip_error>(boost::iostreams::gzip_error const&)")]
// 0x98d25c — __ZN5boost15throw_exceptionINS_9iostreams10gzip_errorEEEvRKT_
pub fn stub_98d25c() -> ! {
    todo!("0x98d25c __ZN5boost15throw_exceptionINS_9iostreams10gzip_errorEEEvRKT_")
}

#[doc(alias = "boost::iostreams::gzip_error::~gzip_error()")]
// 0x98d3b8 — __ZN5boost9iostreams10gzip_errorD1Ev
pub fn stub_98d3b8() -> ! {
    todo!("0x98d3b8 __ZN5boost9iostreams10gzip_errorD1Ev")
}

#[doc(alias = "void boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::close<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,std::_Ios_Openmode)")]
// 0x98d3c4 — __ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E5closeINS2_16linked_streambufIcSt11char_traitsIcEEEEEvRT_St13_Ios_Openmode
pub fn stub_98d3c4() -> ! {
    todo!("0x98d3c4 __ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E5closeINS2_16linked_streambufIcSt11char_traitsIcEEEEEvRT_St13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::gzip_error::~gzip_error()")]
// 0x98d5e4 — __ZN5boost9iostreams10gzip_errorD0Ev
pub fn stub_98d5e4() -> ! {
    todo!("0x98d5e4 __ZN5boost9iostreams10gzip_errorD0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::~clone_impl()")]
// 0x98d5f8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEED1Ev
pub fn stub_98d5f8() -> ! {
    todo!("0x98d5f8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>::~error_info_injector()")]
// 0x98d6b0 — __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10gzip_errorEED1Ev
pub fn stub_98d6b0() -> ! {
    todo!("0x98d6b0 __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10gzip_errorEED1Ev")
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>::~error_info_injector()")]
// 0x98d768 — __ZThn16_N5boost16exception_detail19error_info_injectorINS_9iostreams10gzip_errorEED1Ev
// was: `non-virtual thunk to boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>::~error_info_injector()`
pub fn stub_98d768() -> ! {
    todo!("0x98d768 __ZThn16_N5boost16exception_detail19error_info_injectorINS_9iostreams10gzip_errorEED1Ev")
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::~clone_impl()")]
// 0x98d820 — __ZThn16_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEED1Ev
// was: `non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::~clone_impl()`
pub fn stub_98d820() -> ! {
    todo!("0x98d820 __ZThn16_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEED1Ev")
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::~clone_impl()")]
// 0x98d8d8 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEED1Ev
// was: `virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::~clone_impl()`
pub fn stub_98d8d8() -> ! {
    todo!("0x98d8d8 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::~clone_impl()")]
// 0x98d9a4 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEED0Ev
pub fn stub_98d9a4() -> ! {
    todo!("0x98d9a4 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone(void)const")]
// 0x98da60 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEE5cloneEv
pub fn stub_98da60() -> ! {
    todo!("0x98da60 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::rethrow(void)const")]
// 0x98db1c — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEE7rethrowEv
pub fn stub_98db1c() -> ! {
    todo!("0x98db1c __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEE7rethrowEv")
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::~clone_impl()")]
// 0x98dbcc — __ZThn16_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEED0Ev
// was: `non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::~clone_impl()`
pub fn stub_98dbcc() -> ! {
    todo!("0x98dbcc __ZThn16_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEED0Ev")
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone(void)const")]
// 0x98dc88 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEE5cloneEv
// was: `virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone(void)const`
pub fn stub_98dc88() -> ! {
    todo!("0x98dc88 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEE5cloneEv")
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::rethrow(void)const")]
// 0x98dd4c — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEE7rethrowEv
// was: `virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::rethrow(void)const`
pub fn stub_98dd4c() -> ! {
    todo!("0x98dd4c __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEE7rethrowEv")
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::~clone_impl()")]
// 0x98dd5c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEED0Ev
// was: `virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::~clone_impl()`
pub fn stub_98dd5c() -> ! {
    todo!("0x98dd5c __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEED0Ev")
}