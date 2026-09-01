//! core shard FE — 100 core stubs EA-sorted, lowest uncovered 0xf28814..0xf2a0a4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after FD 0xf28804).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf28804.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.


#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::set_capacity(unsigned long)")]
// 0xf28814 — j___ZN5boost15circular_bufferIdSaIdEE12set_capacityEm
pub fn stub_f28814() -> ! {
    todo!("0xf28814 j___ZN5boost15circular_bufferIdSaIdEE12set_capacityEm")
}

#[doc(alias = "void boost::throw_exception<boost::gregorian::bad_month>(boost::gregorian::bad_month const&)")]
// 0xf28824 — j___ZN5boost15throw_exceptionINS_9gregorian9bad_monthEEEvRKT_
pub fn stub_f28824() -> ! {
    todo!("0xf28824 j___ZN5boost15throw_exceptionINS_9gregorian9bad_monthEEEvRKT_")
}

#[doc(alias = "void boost::throw_exception<std::runtime_error>(std::runtime_error const&)")]
// 0xf28834 — j___ZN5boost15throw_exceptionISt13runtime_errorEEvRKT_
pub fn stub_f28834() -> ! {
    todo!("0xf28834 j___ZN5boost15throw_exceptionISt13runtime_errorEEvRKT_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month> const&)")]
// 0xf28844 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEEC1ERKS5_
pub fn stub_f28844() -> ! {
    todo!("0xf28844 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEEC1ERKS5_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_day_of_month>>::clone_tag)")]
// 0xf28854 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEEC1ERKS6_NS6_9clone_tagE
pub fn stub_f28854() -> ! {
    todo!("0xf28854 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian16bad_day_of_monthEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_impl(boost::exception_detail::error_info_injector<boost::gregorian::bad_year> const&)")]
// 0xf28864 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEEC1ERKS5_
pub fn stub_f28864() -> ! {
    todo!("0xf28864 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEEC1ERKS5_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_year>>::clone_tag)")]
// 0xf28874 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEEC1ERKS6_NS6_9clone_tagE
pub fn stub_f28874() -> ! {
    todo!("0xf28874 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian8bad_yearEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>> const&)")]
// 0xf28884 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS6_
pub fn stub_f28884() -> ! {
    todo!("0xf28884 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS6_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::clone_tag)")]
// 0xf28894 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS6_NS6_9clone_tagE
pub fn stub_f28894() -> ! {
    todo!("0xf28894 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&)")]
// 0xf288a4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_
pub fn stub_f288a4() -> ! {
    todo!("0xf288a4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::length_error>>::clone_tag)")]
// 0xf288b4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_NS5_9clone_tagE
pub fn stub_f288b4() -> ! {
    todo!("0xf288b4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt12length_errorEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::runtime_error>>::clone_tag)")]
// 0xf288c4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEEC1ERKS5_NS5_9clone_tagE
pub fn stub_f288c4() -> ! {
    todo!("0xf288c4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt13runtime_errorEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::TaskScheduler::Job *>::reset(RBX::TaskScheduler::Job **)")]
// 0xf288d4 — j___ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE5resetEPS4_
pub fn stub_f288d4() -> ! {
    todo!("0xf288d4 j___ZN5boost19thread_specific_ptrIPN3RBX13TaskScheduler3JobEE5resetEPS4_")
}

#[doc(alias = "boost::CV::simple_exception_policy<unsigned short,(unsigned short)1400,(unsigned short)10000,boost::gregorian::bad_year>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)")]
// 0xf288e4 — j___ZN5boost2CV23simple_exception_policyItLt1400ELt10000ENS_9gregorian8bad_yearEE8on_errorEttNS0_14violation_enumE
pub fn stub_f288e4() -> ! {
    todo!("0xf288e4 j___ZN5boost2CV23simple_exception_policyItLt1400ELt10000ENS_9gregorian8bad_yearEE8on_errorEttNS0_14violation_enumE")
}

#[doc(alias = "boost::CV::simple_exception_policy<unsigned short,(unsigned short)1,(unsigned short)12,boost::gregorian::bad_month>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)")]
// 0xf288f4 — j___ZN5boost2CV23simple_exception_policyItLt1ELt12ENS_9gregorian9bad_monthEE8on_errorEttNS0_14violation_enumE
pub fn stub_f288f4() -> ! {
    todo!("0xf288f4 j___ZN5boost2CV23simple_exception_policyItLt1ELt12ENS_9gregorian9bad_monthEE8on_errorEttNS0_14violation_enumE")
}

#[doc(alias = "boost::CV::simple_exception_policy<unsigned short,(unsigned short)1,(unsigned short)31,boost::gregorian::bad_day_of_month>::on_error(unsigned short,unsigned short,boost::CV::violation_enum)")]
// 0xf28904 — j___ZN5boost2CV23simple_exception_policyItLt1ELt31ENS_9gregorian16bad_day_of_monthEE8on_errorEttNS0_14violation_enumE
pub fn stub_f28904() -> ! {
    todo!("0xf28904 j___ZN5boost2CV23simple_exception_policyItLt1ELt31ENS_9gregorian16bad_day_of_monthEE8on_errorEttNS0_14violation_enumE")
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>)")]
// 0xf28914 — j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13TaskScheduler6ThreadEEEEEEC2ES8_
// was: boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>)
pub fn stub_f28914() -> ! {
    todo!("0xf28914 j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13TaskScheduler6ThreadEEEEEEC2ES8_")
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<double>>::operator()<void (*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double),boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,double) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job>&> &,int)")]
// 0xf28924 — j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIdEEEclIPFvNS_10shared_ptrIN3RBX13TaskScheduler3JobEEEdENS0_5list1IRSC_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<double>>::operator()<void (*)(boost::shared_ptr<RBX::TaskScheduler::Job>,double),boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::TaskScheduler::Job>,double) &,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job>&> &,int)
pub fn stub_f28924() -> ! {
    todo!("0xf28924 j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIdEEEclIPFvNS_10shared_ptrIN3RBX13TaskScheduler3JobEEEdENS0_5list1IRSC_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>::type> boost::bind<void,RBX::TaskScheduler::Thread,rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>(void (RBX::TaskScheduler::Thread::*)(void),rbx_core::SharedPtr<RBX::TaskScheduler::Thread>)")]
// 0xf28934 — j___ZN5boost4bindIvN3RBX13TaskScheduler6ThreadENS_10shared_ptrIS3_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS8_T0_EENS6_9list_av_1IT1_E4typeEEEMSB_FS8_vESE_
// was: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list_av_1<boost::shared_ptr<RBX::TaskScheduler::Thread>>::type> boost::bind<void,RBX::TaskScheduler::Thread,boost::shared_ptr<RBX::TaskScheduler::Thread>>(void (RBX::TaskScheduler::Thread::*)(void),boost::shared_ptr<RBX::TaskScheduler::Thread>)
pub fn stub_f28934() -> ! {
    todo!("0xf28934 j___ZN5boost4bindIvN3RBX13TaskScheduler6ThreadENS_10shared_ptrIS3_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS8_T0_EENS6_9list_av_1IT1_E4typeEEEMSB_FS8_vESE_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskScheduler::Thread>(RBX::TaskScheduler::Thread *)")]
// 0xf28944 — j___ZN5boost6detail12shared_countC2IN3RBX13TaskScheduler6ThreadEEEPT_
pub fn stub_f28944() -> ! {
    todo!("0xf28944 j___ZN5boost6detail12shared_countC2IN3RBX13TaskScheduler6ThreadEEEPT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf28954 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_f28954() -> ! {
    todo!("0xf28954 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::thread::timed_join(boost::posix_time::ptime const&)")]
// 0xf28964 — j___ZN5boost6thread10timed_joinERKNS_10posix_time5ptimeE
pub fn stub_f28964() -> ! {
    todo!("0xf28964 j___ZN5boost6thread10timed_joinERKNS_10posix_time5ptimeE")
}

#[doc(alias = "boost::thread::do_try_join_until(timespec const&)")]
// 0xf28974 — j___ZN5boost6thread17do_try_join_untilERK8timespec
pub fn stub_f28974() -> ! {
    todo!("0xf28974 j___ZN5boost6thread17do_try_join_untilERK8timespec")
}

#[doc(alias = "boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::subtract_times(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&)")]
// 0xf28984 — j___ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE14subtract_timesERKS5_S8_
pub fn stub_f28984() -> ! {
    todo!("0xf28984 j___ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE14subtract_timesERKS5_S8_")
}

#[doc(alias = "boost::date_time::counted_time_system<boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>>::add_time_duration(boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config> const&,boost::posix_time::time_duration)")]
// 0xf28994 — j___ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE17add_time_durationERKS5_NS3_13time_durationE
pub fn stub_f28994() -> ! {
    todo!("0xf28994 j___ZN5boost9date_time19counted_time_systemINS0_16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEEE17add_time_durationERKS5_NS3_13time_durationE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>)")]
// 0xf289a4 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>)
pub fn stub_f289a4() -> ! {
    todo!("0xf289a4 j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEEvT_")
}

#[doc(alias = "boost::gregorian::bad_day_of_month::bad_day_of_month(void)")]
// 0xf289c4 — j___ZN5boost9gregorian16bad_day_of_monthC2Ev
pub fn stub_f289c4() -> ! {
    todo!("0xf289c4 j___ZN5boost9gregorian16bad_day_of_monthC2Ev")
}

#[doc(alias = "boost::gregorian::date::date(boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day)")]
// 0xf289d4 — j___ZN5boost9gregorian4dateC2ENS0_9greg_yearENS0_10greg_monthENS0_8greg_dayE
pub fn stub_f289d4() -> ! {
    todo!("0xf289d4 j___ZN5boost9gregorian4dateC2ENS0_9greg_yearENS0_10greg_monthENS0_8greg_dayE")
}

#[doc(alias = "boost::gregorian::bad_year::bad_year(void)")]
// 0xf289e4 — j___ZN5boost9gregorian8bad_yearC2Ev
pub fn stub_f289e4() -> ! {
    todo!("0xf289e4 j___ZN5boost9gregorian8bad_yearC2Ev")
}

#[doc(alias = "boost::gregorian::bad_month::bad_month(void)")]
// 0xf289f4 — j___ZN5boost9gregorian9bad_monthC2Ev
pub fn stub_f289f4() -> ! {
    todo!("0xf289f4 j___ZN5boost9gregorian9bad_monthC2Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::gregorian::bad_month>>::rethrow(void)const")]
// 0xf28a04 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE7rethrowEv
pub fn stub_f28a04() -> ! {
    todo!("0xf28a04 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9gregorian9bad_monthEEEE7rethrowEv")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const")]
// 0xf28a14 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler6ThreadEE22_internal_accept_ownerIS3_S3_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Thread>::_internal_accept_owner<RBX::TaskScheduler::Thread,RBX::TaskScheduler::Thread>(boost::shared_ptr<RBX::TaskScheduler::Thread> const*,RBX::TaskScheduler::Thread *)const
pub fn stub_f28a14() -> ! {
    todo!("0xf28a14 j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler6ThreadEE22_internal_accept_ownerIS3_S3_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const")]
// 0xf28a24 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &)const
pub fn stub_f28a24() -> ! {
    todo!("0xf28a24 j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf28a34 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::TaskScheduler::Thread>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_f28a34() -> ! {
    todo!("0xf28a34 j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13TaskScheduler6ThreadEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *)")]
// 0xf28a44 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESA_EET0_T_SC_SB_
// was: boost::shared_ptr<RBX::TaskScheduler::Job const> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *>(boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *,boost::shared_ptr<RBX::TaskScheduler::Job const> *)
pub fn stub_f28a44() -> ! {
    todo!("0xf28a44 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESA_EET0_T_SC_SB_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *,rbx_core::SharedPtr<RBX::TaskScheduler::Job> *)")]
// 0xf28a54 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::TaskScheduler::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *>(boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *,boost::shared_ptr<RBX::TaskScheduler::Job> *)
pub fn stub_f28a54() -> ! {
    todo!("0xf28a54 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEES9_EET0_T_SB_SA_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *)")]
// 0xf28a64 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::TaskScheduler::Thread> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *>(boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *)
pub fn stub_f28a64() -> ! {
    todo!("0xf28a64 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *>(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> *)")]
// 0xf28a74 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::TaskScheduler::Thread> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *>(boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *,boost::shared_ptr<RBX::TaskScheduler::Thread> *)
pub fn stub_f28a74() -> ! {
    todo!("0xf28a74 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEES9_EET0_T_SB_SA_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&)")]
// 0xf28a84 — j___ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::shared_ptr<RBX::TaskScheduler::Job const> const&)
pub fn stub_f28a84() -> ! {
    todo!("0xf28a84 j___ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job const> const&)")]
// 0xf28a94 — j___ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE9push_backERKS6_
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Job const> const&)
pub fn stub_f28a94() -> ! {
    todo!("0xf28a94 j___ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EE9push_backERKS6_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
// 0xf28aa4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>>,boost::shared_ptr<RBX::TaskScheduler::Job> const&)
pub fn stub_f28aa4() -> ! {
    todo!("0xf28aa4 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
// 0xf28ab4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE9push_backERKS5_
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Job>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Job> const&)
pub fn stub_f28ab4() -> ! {
    todo!("0xf28ab4 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEESaIS5_EE9push_backERKS5_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>*,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>>,rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&)")]
// 0xf28ac4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Thread>*,std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>>,boost::shared_ptr<RBX::TaskScheduler::Thread> const&)
pub fn stub_f28ac4() -> ! {
    todo!("0xf28ac4 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Thread>>>::push_back(rbx_core::SharedPtr<RBX::TaskScheduler::Thread> const&)")]
// 0xf28ad4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE9push_backERKS5_
// was: std::vector<boost::shared_ptr<RBX::TaskScheduler::Thread>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Thread>>>::push_back(boost::shared_ptr<RBX::TaskScheduler::Thread> const&)
pub fn stub_f28ad4() -> ! {
    todo!("0xf28ad4 j___ZNSt6vectorIN5boost10shared_ptrIN3RBX13TaskScheduler6ThreadEEESaIS5_EE9push_backERKS5_")
}

#[doc(alias = "RBX::TCritical(unsigned int,RBX::Confidence)")]
// 0xf28ae4 — j___ZN3RBX9TCriticalEjNS_10ConfidenceE
pub fn stub_f28ae4() -> ! {
    todo!("0xf28ae4 j___ZN3RBX9TCriticalEjNS_10ConfidenceE")
}

#[doc(alias = "void boost::algorithm::trim_if<std::string,bool (*)(char)>(std::string &,bool (*)(char))")]
// 0xf28af4 — j___ZN5boost9algorithm7trim_ifISsPFbcEEEvRT_T0_
pub fn stub_f28af4() -> ! {
    todo!("0xf28af4 j___ZN5boost9algorithm7trim_ifISsPFbcEEEvRT_T0_")
}

#[doc(alias = "RBX::HttpService::HttpContentType * rbx::any_cast<RBX::HttpService::HttpContentType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf28c54 — j___ZN3rbx8any_castIN3RBX11HttpService15HttpContentTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f28c54() -> ! {
    todo!("0xf28c54 j___ZN3rbx8any_castIN3RBX11HttpService15HttpContentTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::HttpService::HttpContentType & rbx::any_cast<RBX::HttpService::HttpContentType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf28c64 — j___ZN3rbx8any_castIRN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f28c64() -> ! {
    todo!("0xf28c64 j___ZN3rbx8any_castIRN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Vector_base<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_allocate(unsigned long)")]
// 0xf28cb4 — j___ZNSt12_Vector_baseIN3RBX11HttpService15HttpContentTypeESaIS2_EE11_M_allocateEm
pub fn stub_f28cb4() -> ! {
    todo!("0xf28cb4 j___ZNSt12_Vector_baseIN3RBX11HttpService15HttpContentTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::HttpService::HttpContentType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *>(RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *,RBX::HttpService::HttpContentType *)")]
// 0xf28cc4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11HttpService15HttpContentTypeES6_EET0_T_S8_S7_
pub fn stub_f28cc4() -> ! {
    todo!("0xf28cc4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11HttpService15HttpContentTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::HttpService::HttpContentType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::operator[](RBX::Name const* const&)")]
// 0xf28cd4 — j___ZNSt3mapIPKN3RBX4NameENS0_11HttpService15HttpContentTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f28cd4() -> ! {
    todo!("0xf28cd4 j___ZNSt3mapIPKN3RBX4NameENS0_11HttpService15HttpContentTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,RBX::HttpService::HttpContentType const&)")]
// 0xf28ce4 — j___ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f28ce4() -> ! {
    todo!("0xf28ce4 j___ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HttpService::HttpContentType*,std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>>,unsigned long,RBX::HttpService::HttpContentType const&)")]
// 0xf28cf4 — j___ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f28cf4() -> ! {
    todo!("0xf28cf4 j___ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::resize(unsigned long,RBX::HttpService::HttpContentType)")]
// 0xf28d04 — j___ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE6resizeEmS2_
pub fn stub_f28d04() -> ! {
    todo!("0xf28d04 j___ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::HttpService::HttpContentType,std::allocator<RBX::HttpService::HttpContentType>>::push_back(RBX::HttpService::HttpContentType const&)")]
// 0xf28d14 — j___ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE9push_backERKS2_
pub fn stub_f28d14() -> ! {
    todo!("0xf28d14 j___ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
// 0xf28d24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f28d24() -> ! {
    todo!("0xf28d24 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
// 0xf28d34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f28d34() -> ! {
    todo!("0xf28d34 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType> const&)")]
// 0xf28d44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f28d44() -> ! {
    todo!("0xf28d44 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "boost::singleton_pool<XmlAttribute,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf28f34 — j___ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f28f34() -> ! {
    todo!("0xf28f34 j___ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "void boost::throw_exception<boost::thread_resource_error>(boost::thread_resource_error const&)")]
// 0xf28f44 — j___ZN5boost15throw_exceptionINS_21thread_resource_errorEEEvRKT_
pub fn stub_f28f44() -> ! {
    todo!("0xf28f44 j___ZN5boost15throw_exceptionINS_21thread_resource_errorEEEvRKT_")
}

#[doc(alias = "boost::exception_detail::copy_boost_exception(boost::exception *,boost::exception const*)")]
// 0xf28f54 — j___ZN5boost16exception_detail20copy_boost_exceptionEPNS_9exceptionEPKS1_
pub fn stub_f28f54() -> ! {
    todo!("0xf28f54 j___ZN5boost16exception_detail20copy_boost_exceptionEPNS_9exceptionEPKS1_")
}

#[doc(alias = "boost::detail::shared_count::shared_count(boost::detail::shared_count const&)")]
// 0xf28f64 — j___ZN5boost6detail12shared_countC1ERKS1_
pub fn stub_f28f64() -> ! {
    todo!("0xf28f64 j___ZN5boost6detail12shared_countC1ERKS1_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::rethrow(void)const")]
// 0xf28f84 — j___ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE7rethrowEv
pub fn stub_f28f84() -> ! {
    todo!("0xf28f84 j___ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE7rethrowEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> *)")]
// 0xf28ff4 — j___ZN5boost6detail12shared_countC2INS_16exception_detail10clone_implINS3_10bad_alloc_EEEEEPT_
pub fn stub_f28ff4() -> ! {
    todo!("0xf28ff4 j___ZN5boost6detail12shared_countC2INS_16exception_detail10clone_implINS3_10bad_alloc_EEEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_base::release(void)")]
// 0xf29004 — j___ZN5boost6detail15sp_counted_base7releaseEv
pub fn stub_f29004() -> ! {
    todo!("0xf29004 j___ZN5boost6detail15sp_counted_base7releaseEv")
}

#[doc(alias = "boost::exception_detail::bad_alloc_::~bad_alloc_()")]
// 0xf29194 — j___ZN5boost16exception_detail10bad_alloc_D2Ev
pub fn stub_f29194() -> ! {
    todo!("0xf29194 j___ZN5boost16exception_detail10bad_alloc_D2Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_tag)")]
// 0xf291a4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS5_NS5_9clone_tagE
pub fn stub_f291a4() -> ! {
    todo!("0xf291a4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::bad_exception_::~bad_exception_()")]
// 0xf291b4 — j___ZN5boost16exception_detail14bad_exception_D2Ev
pub fn stub_f291b4() -> ! {
    todo!("0xf291b4 j___ZN5boost16exception_detail14bad_exception_D2Ev")
}

#[doc(alias = "boost::exception_ptr boost::exception_detail::get_static_exception_object<boost::exception_detail::bad_alloc_>(void)")]
// 0xf291c4 — j___ZN5boost16exception_detail27get_static_exception_objectINS0_10bad_alloc_EEENS_13exception_ptrEv
pub fn stub_f291c4() -> ! {
    todo!("0xf291c4 j___ZN5boost16exception_detail27get_static_exception_objectINS0_10bad_alloc_EEENS_13exception_ptrEv")
}

#[doc(alias = "boost::exception_ptr boost::exception_detail::get_static_exception_object<boost::exception_detail::bad_exception_>(void)")]
// 0xf291d4 — j___ZN5boost16exception_detail27get_static_exception_objectINS0_14bad_exception_EEENS_13exception_ptrEv
pub fn stub_f291d4() -> ! {
    todo!("0xf291d4 j___ZN5boost16exception_detail27get_static_exception_objectINS0_14bad_exception_EEENS_13exception_ptrEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_> *)")]
// 0xf291e4 — j___ZN5boost6detail12shared_countC2INS_16exception_detail10clone_implINS3_14bad_exception_EEEEEPT_
pub fn stub_f291e4() -> ! {
    todo!("0xf291e4 j___ZN5boost6detail12shared_countC2INS_16exception_detail10clone_implINS3_14bad_exception_EEEEEPT_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::rethrow(void)const")]
// 0xf29424 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE7rethrowEv
pub fn stub_f29424() -> ! {
    todo!("0xf29424 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE7rethrowEv")
}

#[doc(alias = "XmlElement::XmlElement(RBX::Name const&)")]
// 0xf296c4 — j___ZN10XmlElementC2ERKN3RBX4NameE
pub fn stub_f296c4() -> ! {
    todo!("0xf296c4 j___ZN10XmlElementC2ERKN3RBX4NameE")
}

#[doc(alias = "XmlAttribute::XmlAttribute<RBX::Name const*>(RBX::Name const&,RBX::Name const*)")]
// 0xf296d4 — j___ZN12XmlAttributeC2IPKN3RBX4NameEEERS3_T_
pub fn stub_f296d4() -> ! {
    todo!("0xf296d4 j___ZN12XmlAttributeC2IPKN3RBX4NameEEERS3_T_")
}

#[doc(alias = "RBX::Allocator<XmlElement>::Allocator(void)")]
// 0xf29704 — j___ZN3RBX9AllocatorI10XmlElementEC2Ev
pub fn stub_f29704() -> ! {
    todo!("0xf29704 j___ZN3RBX9AllocatorI10XmlElementEC2Ev")
}

#[doc(alias = "RBX::Allocator<XmlElement>::operator new(unsigned long)")]
// 0xf29714 — j___ZN3RBX9AllocatorI10XmlElementEnwEm
pub fn stub_f29714() -> ! {
    todo!("0xf29714 j___ZN3RBX9AllocatorI10XmlElementEnwEm")
}

#[doc(alias = "RBX::Allocator<XmlAttribute>::Allocator(void)")]
// 0xf29724 — j___ZN3RBX9AllocatorI12XmlAttributeEC2Ev
pub fn stub_f29724() -> ! {
    todo!("0xf29724 j___ZN3RBX9AllocatorI12XmlAttributeEC2Ev")
}

#[doc(alias = "RBX::Allocator<XmlAttribute>::operator new(unsigned long)")]
// 0xf29734 — j___ZN3RBX9AllocatorI12XmlAttributeEnwEm
pub fn stub_f29734() -> ! {
    todo!("0xf29734 j___ZN3RBX9AllocatorI12XmlAttributeEnwEm")
}

#[doc(alias = "boost::singleton_pool<XmlAttribute,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf29744 — j___ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f29744() -> ! {
    todo!("0xf29744 j___ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "boost::simple_segregated_storage<unsigned long>::segregate(void *,unsigned long,unsigned long,void *)")]
// 0xf29754 — j___ZN5boost25simple_segregated_storageImE9segregateEPvmmS2_
pub fn stub_f29754() -> ! {
    todo!("0xf29754 j___ZN5boost25simple_segregated_storageImE9segregateEPvmmS2_")
}

#[doc(alias = "boost::pool<boost::default_user_allocator_malloc_free>::release_memory(void)")]
// 0xf29764 — j___ZN5boost4poolINS_34default_user_allocator_malloc_freeEE14release_memoryEv
pub fn stub_f29764() -> ! {
    todo!("0xf29764 j___ZN5boost4poolINS_34default_user_allocator_malloc_freeEE14release_memoryEv")
}

#[doc(alias = "std::_Vector_base<bool (*)(void),std::allocator<bool (*)(void)>>::_M_allocate(unsigned long)")]
// 0xf29824 — j___ZNSt12_Vector_baseIPFbvESaIS1_EE11_M_allocateEm
pub fn stub_f29824() -> ! {
    todo!("0xf29824 j___ZNSt12_Vector_baseIPFbvESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::_M_insert_aux(__gnu_cxx::__normal_iterator<bool (**)(void),std::vector<bool (*)(void),std::allocator<bool (*)(void)>>>,bool (* const&)(void))")]
// 0xf29854 — j___ZNSt6vectorIPFbvESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_f29854() -> ! {
    todo!("0xf29854 j___ZNSt6vectorIPFbvESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "boost::multi_index::detail::auto_space<unsigned long,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::auto_space(std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&,unsigned long)")]
// 0xf299d4 — j___ZN5boost11multi_index6detail10auto_spaceImSaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES8_EEEEC2ERKSC_m
pub fn stub_f299d4() -> ! {
    todo!("0xf299d4 j___ZN5boost11multi_index6detail10auto_spaceImSaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES8_EEEEC2ERKSC_m")
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::link_point(boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const&,boost::multi_index::detail::hashed_index_node_impl<std::allocator<char>> *&,boost::multi_index::detail::hashed_unique_tag)")]
// 0xf299e4 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE10link_pointERKSC_RPNS1_22hashed_index_node_implISaIcEEEST_
pub fn stub_f299e4() -> ! {
    todo!("0xf299e4 j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE10link_pointERKSC_RPNS1_22hashed_index_node_implISaIcEEEST_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject>(RBX::InputObject const&)")]
// 0xf29d14 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObjectEEERS3_RKT_
pub fn stub_f29d14() -> ! {
    todo!("0xf29d14 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObjectEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Region3int16>(RBX::Region3int16 const&)")]
// 0xf29d24 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12Region3int16EEERS3_RKT_
pub fn stub_f29d24() -> ! {
    todo!("0xf29d24 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12Region3int16EEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CellID>(RBX::CellID const&)")]
// 0xf29d44 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6CellIDEEERS3_RKT_
pub fn stub_f29d44() -> ! {
    todo!("0xf29d44 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6CellIDEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Region3int16>::singleton(void)")]
// 0xf29d74 — j___ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE9singletonEv
pub fn stub_f29d74() -> ! {
    todo!("0xf29d74 j___ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CellID>::singleton(void)")]
// 0xf29d84 — j___ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE9singletonEv
pub fn stub_f29d84() -> ! {
    todo!("0xf29d84 j___ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::RbxRay>::singleton(void)")]
// 0xf29d94 — j___ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE9singletonEv
pub fn stub_f29d94() -> ! {
    todo!("0xf29d94 j___ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE9singletonEv")
}

#[doc(alias = "RBX::Region3int16 const& rbx::any_cast<RBX::Region3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf29dc4 — j___ZN3rbx8any_castIRKN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f29dc4() -> ! {
    todo!("0xf29dc4 j___ZN3rbx8any_castIRKN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::ProtectedString const& rbx::any_cast<RBX::ProtectedString const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf29dd4 — j___ZN3rbx8any_castIRKN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f29dd4() -> ! {
    todo!("0xf29dd4 j___ZN3rbx8any_castIRKN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Axes const& rbx::any_cast<RBX::Axes const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf29df4 — j___ZN3rbx8any_castIRKN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f29df4() -> ! {
    todo!("0xf29df4 j___ZN3rbx8any_castIRKN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::UDim const& rbx::any_cast<RBX::UDim const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf29e04 — j___ZN3rbx8any_castIRKN3RBX4UDimENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f29e04() -> ! {
    todo!("0xf29e04 j___ZN3rbx8any_castIRKN3RBX4UDimENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CellID const& rbx::any_cast<RBX::CellID const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf29e14 — j___ZN3rbx8any_castIRKN3RBX6CellIDENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f29e14() -> ! {
    todo!("0xf29e14 j___ZN3rbx8any_castIRKN3RBX6CellIDENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Region3 const& rbx::any_cast<RBX::Region3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf29e24 — j___ZN3rbx8any_castIRKN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE
pub fn stub_f29e24() -> ! {
    todo!("0xf29e24 j___ZN3rbx8any_castIRKN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::ContentId const& rbx::any_cast<RBX::ContentId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf29e34 — j___ZN3rbx8any_castIRKN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f29e34() -> ! {
    todo!("0xf29e34 j___ZN3rbx8any_castIRKN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "long const& rbx::any_cast<long const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf29e94 — j___ZN3rbx8any_castIRKlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f29e94() -> ! {
    todo!("0xf29e94 j___ZN3rbx8any_castIRKlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::at(unsigned long)const")]
// 0xf2a0a4 — j___ZNKSt6vectorIN3RBX10BrickColorESaIS1_EE2atEm
pub fn stub_f2a0a4() -> ! {
    todo!("0xf2a0a4 j___ZNKSt6vectorIN3RBX10BrickColorESaIS1_EE2atEm")
}
