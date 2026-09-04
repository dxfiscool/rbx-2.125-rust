//! core shard GZ — 100 core stubs EA-sorted, 0xf555b4..0xf55fc4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after GY 0x1c4b48).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after GY 0x1c4b48 (0xf555b4..0xf55fc4, 20314->20414 covered, 1504 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>)")]
// 0xf555b4 — j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEEEC2ES8_
// was: boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>)
pub fn stub_0xf555b4() {
    // IDA 0xf555b4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>) &,boost::_bi::list0 &,int)")]
// 0xf555c4 — j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEEEclIPFvS7_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>::operator()<void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>) &,boost::_bi::list0 &,int)
pub fn stub_0xf555c4() {
    // IDA 0xf555c4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>)")]
// 0xf555d4 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_ISsEEEC2ES8_S9_
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>)
pub fn stub_0xf555d4() {
    // IDA 0xf555d4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>::operator()<void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string) &,boost::_bi::list0 &,int)")]
// 0xf555e4 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_ISsEEEclIPFvS7_SsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>::operator()<void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string) &,boost::_bi::list0 &,int)
pub fn stub_0xf555e4() {
    // IDA 0xf555e4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>)")]
// 0xf555f4 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_IiEEEC2ES8_S9_
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>::list2(boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>)
pub fn stub_0xf555f4() {
    // IDA 0xf555f4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>::operator()<void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int) &,boost::_bi::list0 &,int)")]
// 0xf55604 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_IiEEEclIPFvS7_iENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>::operator()<void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int) &,boost::_bi::list0 &,int)
pub fn stub_0xf55604() {
    // IDA 0xf55604: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::LibraryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>&> &,int)")]
// 0xf55614 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX14LibraryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS4_18LibraryStateObjectEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<RBX::LibraryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list1<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>> &,boost::_bi::list1<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>&> &,int)
pub fn stub_0xf55614() {
    // IDA 0xf55614: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::LibraryService *>,boost::arg<1>,boost::_bi::value<std::string>>::list3(boost::_bi::value<RBX::LibraryService *>,boost::arg<1>,boost::_bi::value<std::string>)")]
// 0xf55624 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX14LibraryServiceEEENS_3argILi1EEENS2_ISsEEEC2ES6_S8_S9_
pub fn stub_0xf55624() {
    // IDA 0xf55624: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::LibraryService *>,boost::arg<1>,boost::_bi::value<std::string>>::operator()<boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list1<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>&> &,int)")]
// 0xf55634 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX14LibraryServiceEEENS_3argILi1EEENS2_ISsEEEclINS_4_mfi3mf2IvS4_NS_10shared_ptrINS4_18LibraryStateObjectEEERKSsEENS0_5list1IRSG_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<RBX::LibraryService *>,boost::arg<1>,boost::_bi::value<std::string>>::operator()<boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list1<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&> &,boost::_bi::list1<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>&> &,int)
pub fn stub_0xf55634() {
    // IDA 0xf55634: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>)")]
// 0xf55664 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_ISsEEEC2ES8_S9_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>)
pub fn stub_0xf55664() {
    // IDA 0xf55664: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>)")]
// 0xf55674 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_IiEEEC2ES8_S9_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>)
pub fn stub_0xf55674() {
    // IDA 0xf55674: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list_av_3<RBX::LibraryService*,boost::arg<1>,std::string>::type> boost::bind<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&,RBX::LibraryService*,boost::arg<1>,std::string>(void (RBX::LibraryService::*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&),RBX::LibraryService*,boost::arg<1>,std::string)")]
// 0xf556e4 — j___ZN5boost4bindIvN3RBX14LibraryServiceENS_10shared_ptrINS2_18LibraryStateObjectEEERKSsPS2_NS_3argILi1EEESsEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISD_T0_T1_T2_EENSB_9list_av_3IT3_T4_T5_E4typeEEEMSG_FSD_SH_SI_ESL_SM_SN_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list_av_3<RBX::LibraryService*,boost::arg<1>,std::string>::type> boost::bind<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&,RBX::LibraryService*,boost::arg<1>,std::string>(void (RBX::LibraryService::*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&),RBX::LibraryService*,boost::arg<1>,std::string)
pub fn stub_0xf556e4() {
    // IDA 0xf556e4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>(void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")]
// 0xf556f4 — j___ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEES5_EENS_3_bi6bind_tIT_PFS8_T0_ENS6_9list_av_1IT1_E4typeEEESB_SD_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list_av_1<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>::type> boost::bind<void,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>(void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::shared_ptr<RBX::LibraryService::LibraryStateObject>)
pub fn stub_0xf556f4() {
    // IDA 0xf556f4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string>::type> boost::bind<void,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string>(void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string)")]
// 0xf55704 — j___ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsS5_SsEENS_3_bi6bind_tIT_PFS8_T0_T1_ENS6_9list_av_2IT2_T3_E4typeEEESC_SE_SF_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list_av_2<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string>::type> boost::bind<void,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string>(void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string)
pub fn stub_0xf55704() {
    // IDA 0xf55704: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int>::type> boost::bind<void,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int>(void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int)")]
// 0xf55714 — j___ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiS5_iEENS_3_bi6bind_tIT_PFS8_T0_T1_ENS6_9list_av_2IT2_T3_E4typeEEESC_SE_SF_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list_av_2<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int>::type> boost::bind<void,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int>(void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int)
pub fn stub_0xf55714() {
    // IDA 0xf55714: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LibraryService::LibraryStateObject>(RBX::LibraryService::LibraryStateObject *)")]
// 0xf55734 — j___ZN5boost6detail12shared_countC2IN3RBX14LibraryService18LibraryStateObjectEEEPT_
pub fn stub_0xf55734() {
    // IDA 0xf55734: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf55744 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf55744() {
    // IDA 0xf55744: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf55754 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf55754() {
    // IDA 0xf55754: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0xf55774 — j___ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEE12manage_smallERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0xf55774() {
    // IDA 0xf55774: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>)")]
// 0xf557d4 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>)
pub fn stub_0xf557d4() {
    // IDA 0xf557d4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>)")]
// 0xf557e4 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>)
pub fn stub_0xf557e4() {
    // IDA 0xf557e4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>)")]
// 0xf557f4 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>)
pub fn stub_0xf557f4() {
    // IDA 0xf557f4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::string>> const&)")]
// 0xf55854 — j___ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeISsEEE
pub fn stub_0xf55854() {
    // IDA 0xf55854: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>::destroy(std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>*)")]
// 0xf55864 — j___ZN9__gnu_cxx13new_allocatorISt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS9_EEEE7destroyEPSC_
// was: __gnu_cxx::new_allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>::destroy(std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>*)
pub fn stub_0xf55864() {
    // IDA 0xf55864: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>::operator()(RBX::LibraryService*,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)const")]
// 0xf55874 — j___ZNK5boost4_mfi3mf1IvN3RBX14LibraryServiceENS_10shared_ptrINS3_18LibraryStateObjectEEEEclEPS3_S6_
// was: boost::_mfi::mf1<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>::operator()(RBX::LibraryService*,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>)const
pub fn stub_0xf55874() {
    // IDA 0xf55874: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>::operator()(RBX::LibraryService*,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&)const")]
// 0xf55884 — j___ZNK5boost4_mfi3mf2IvN3RBX14LibraryServiceENS_10shared_ptrINS3_18LibraryStateObjectEEERKSsEclEPS3_S6_S8_
// was: boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&>::operator()(RBX::LibraryService*,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&)const
pub fn stub_0xf55884() {
    // IDA 0xf55884: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::hash<RBX::ProtectedString>::operator()(RBX::ProtectedString const&)const")]
// 0xf55894 — j___ZNK5boost4hashIN3RBX15ProtectedStringEEclERKS2_
pub fn stub_0xf55894() {
    // IDA 0xf55894: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf558a4 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS5_5list2INS5_5valueISB_EENSF_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf558a4() {
    // IDA 0xf558a4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf558b4 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS5_5list2INS5_5valueISB_EENSF_IiEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf558b4() {
    // IDA 0xf558b4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,boost::detail::function::function_buffer &)const")]
// 0xf558c4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS5_5list1INS5_5valueISB_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf558c4() {
    // IDA 0xf558c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf558d4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS5_5list1INS5_5valueISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf558d4() {
    // IDA 0xf558d4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// 0xf558e4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS5_5list2INS5_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf558e4() {
    // IDA 0xf558e4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf558f4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS5_5list2INS5_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf558f4() {
    // IDA 0xf558f4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &)const")]
// 0xf55904 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS5_5list2INS5_5valueISB_EENSF_IiEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf55904() {
    // IDA 0xf55904: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf55914 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS5_5list2INS5_5valueISB_EENSF_IiEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf55914() {
    // IDA 0xf55914: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::string>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xf55954 — j___ZNK5boost9unordered6detail10table_implINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsS8_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISsEEEEmRKT_RKT0_
pub fn stub_0xf55954() {
    // IDA 0xf55954: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0xf55964 — j___ZNK5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE9find_nodeERKSs
pub fn stub_0xf55964() {
    // IDA 0xf55964: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_List_base<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::_M_clear(void)")]
// 0xf55974 — j___ZNSt10_List_baseIN5boost8functionIFvvEEESaIS3_EE8_M_clearEv
pub fn stub_0xf55974() {
    // IDA 0xf55974: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_List_base<std::string,std::allocator<std::string>>::_M_clear(void)")]
// 0xf55984 — j___ZNSt10_List_baseISsSaISsEE8_M_clearEv
pub fn stub_0xf55984() {
    // IDA 0xf55984: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::map<std::string,RBX::LibraryService::LibraryDefinition,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::operator[](std::string const&)")]
// 0xf55994 — j___ZNSt3mapISsN3RBX14LibraryService17LibraryDefinitionESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
pub fn stub_0xf55994() {
    // IDA 0xf55994: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<std::string,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::operator[](std::string const&)")]
// 0xf559b4 — j___ZNSt3mapISsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS6_EESt4lessISsESaISt4pairIKSsS8_EEEixERSC_
// was: std::map<std::string,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::operator[](std::string const&)
pub fn stub_0xf559b4() {
    // IDA 0xf559b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::_M_create_node(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject> const&)")]
// 0xf559c4 — j___ZNSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE14_M_create_nodeERKS5_
// was: std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>::_M_create_node(boost::shared_ptr<RBX::LibraryService::LibraryStateObject> const&)
pub fn stub_0xf559c4() {
    // IDA 0xf559c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::_M_initialize_dispatch<std::_List_const_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>(std::_List_const_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,std::_List_const_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,std::__false_type)")]
// 0xf559d4 — j___ZNSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE22_M_initialize_dispatchISt20_List_const_iteratorIS5_EEEvT_SB_St12__false_type
// was: void std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>::_M_initialize_dispatch<std::_List_const_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>(std::_List_const_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,std::_List_const_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,std::__false_type)
pub fn stub_0xf559d4() {
    // IDA 0xf559d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::list(std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>> const&)")]
// 0xf559e4 — j___ZNSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EEC2ERKS7_
// was: std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>::list(std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>> const&)
pub fn stub_0xf559e4() {
    // IDA 0xf559e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::_M_create_node(boost::function<void ()(void)> const&)")]
// 0xf559f4 — j___ZNSt4listIN5boost8functionIFvvEEESaIS3_EE14_M_create_nodeERKS3_
pub fn stub_0xf559f4() {
    // IDA 0xf559f4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::list<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::_M_initialize_dispatch<std::_List_const_iterator<boost::function<void ()(void)>>>(std::_List_const_iterator<boost::function<void ()(void)>>,std::_List_const_iterator<boost::function<void ()(void)>>,std::__false_type)")]
// 0xf55a04 — j___ZNSt4listIN5boost8functionIFvvEEESaIS3_EE22_M_initialize_dispatchISt20_List_const_iteratorIS3_EEEvT_S9_St12__false_type
pub fn stub_0xf55a04() {
    // IDA 0xf55a04: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::list(std::list<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>> const&)")]
// 0xf55a14 — j___ZNSt4listIN5boost8functionIFvvEEESaIS3_EEC2ERKS5_
pub fn stub_0xf55a14() {
    // IDA 0xf55a14: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<std::string,std::allocator<std::string>>::_M_create_node(std::string const&)")]
// 0xf55a24 — j___ZNSt4listISsSaISsEE14_M_create_nodeERKSs
pub fn stub_0xf55a24() {
    // IDA 0xf55a24: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::pair<std::string const,RBX::LibraryService::LibraryDefinition>::pair(std::string const&,RBX::LibraryService::LibraryDefinition const&)")]
// 0xf55a34 — j___ZNSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEEC2ERS0_RKS3_
pub fn stub_0xf55a34() {
    // IDA 0xf55a34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>::pair(std::string const&,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>> const&)")]
// 0xf55a54 — j___ZNSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS7_EEEC2ERS0_RKS9_
// was: std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>::pair(std::string const&,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>> const&)
pub fn stub_0xf55a54() {
    // IDA 0xf55a54: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::lower_bound(std::string const&)")]
// 0xf55a64 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
pub fn stub_0xf55a64() {
    // IDA 0xf55a64: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_create_node(std::pair<std::string const,RBX::LibraryService::LibraryDefinition> const&)")]
// 0xf55a74 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_0xf55a74() {
    // IDA 0xf55a74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_insert_unique(std::pair<std::string const,RBX::LibraryService::LibraryDefinition> const&)")]
// 0xf55a84 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_0xf55a84() {
    // IDA 0xf55a84: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::pair<std::string const,RBX::LibraryService::LibraryDefinition> const&)")]
// 0xf55a94 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_0xf55a94() {
    // IDA 0xf55a94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::find(std::string const&)")]
// 0xf55aa4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
pub fn stub_0xf55aa4() {
    // IDA 0xf55aa4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::LibraryService::LibraryDefinition> const&)")]
// 0xf55ab4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_0xf55ab4() {
    // IDA 0xf55ab4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::lower_bound(std::string const&)")]
// 0xf55b24 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE11lower_boundERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::lower_bound(std::string const&)
pub fn stub_0xf55b24() {
    // IDA 0xf55b24: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::upper_bound(std::string const&)")]
// 0xf55b34 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE11upper_boundERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::upper_bound(std::string const&)
pub fn stub_0xf55b34() {
    // IDA 0xf55b34: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_create_node(std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>> const&)")]
// 0xf55b44 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE14_M_create_nodeERKSB_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_create_node(std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>> const&)
pub fn stub_0xf55b44() {
    // IDA 0xf55b44: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_insert_unique(std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>> const&)")]
// 0xf55b54 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE16_M_insert_uniqueERKSB_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_insert_unique(std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>> const&)
pub fn stub_0xf55b54() {
    // IDA 0xf55b54: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>> const&)")]
// 0xf55b64 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>> const&)
pub fn stub_0xf55b64() {
    // IDA 0xf55b64: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::erase(std::string const&)")]
// 0xf55b74 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE5eraseERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::erase(std::string const&)
pub fn stub_0xf55b74() {
    // IDA 0xf55b74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>)")]
// 0xf55b84 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE5eraseESt17_Rb_tree_iteratorISB_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>)
pub fn stub_0xf55b84() {
    // IDA 0xf55b84: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::_Rb_tree_iterator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>)")]
// 0xf55b94 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESJ_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::_Rb_tree_iterator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>)
pub fn stub_0xf55b94() {
    // IDA 0xf55b94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>> *)")]
// 0xf55ba4 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>> *)
pub fn stub_0xf55ba4() {
    // IDA 0xf55ba4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>> const&)")]
// 0xf55bb4 — j___ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSB_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>> const&)
pub fn stub_0xf55bb4() {
    // IDA 0xf55bb4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>>)")]
// 0xf55bc4 — j___ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf1IvS4_S6_EENS8_5list2INS8_5valueIPS4_EENS1_3argILi1EEEEEEEET0_T_SM_SL_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>> std::for_each<std::_List_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>>>(std::_List_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,std::_List_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>>)
pub fn stub_0xf55bc4() {
    // IDA 0xf55bc4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>>)")]
// 0xf55bd4 — j___ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf2IvS4_S6_RKSsEENS8_5list3INS8_5valueIPS4_EENS1_3argILi1EEENSG_ISsEEEEEEET0_T_SP_SO_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>> std::for_each<std::_List_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>>>(std::_List_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,std::_List_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>>)
pub fn stub_0xf55bd4() {
    // IDA 0xf55bd4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void (*)(boost::function<void ()(void)>) std::for_each<std::_List_iterator<boost::function<void ()(void)>>,void (*)(boost::function<void ()(void)>)>(std::_List_iterator<boost::function<void ()(void)>>,std::_List_iterator<boost::function<void ()(void)>>,void (*)(boost::function<void ()(void)>))")]
// 0xf55be4 — j___ZSt8for_eachISt14_List_iteratorIN5boost8functionIFvvEEEEPFvS4_EET0_T_S9_S8_
pub fn stub_0xf55be4() {
    // IDA 0xf55be4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Hook::remove(void)")]
// 0xf55c04 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E4Hook6removeEv
pub fn stub_0xf55c04() {
    // IDA 0xf55c04: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::insert(RobloxExtraSpace&)")]
// 0xf55c14 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E6insertERS2_
pub fn stub_0xf55c14() {
    // IDA 0xf55c14: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::erase(RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator)")]
// 0xf55c54 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E5eraseENS3_8IteratorE
pub fn stub_0xf55c54() {
    // IDA 0xf55c54: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator*(void)")]
// 0xf55c64 — j___ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratordeEv
pub fn stub_0xf55c64() {
    // IDA 0xf55c64: boost template instantiation (mangled-only context). Per Boost map (AGENTS.md section 4) — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RobloxExtraSpace::Shared>::shared_ptr<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *)")]
// 0xf55c74 — j___ZN5boost10shared_ptrIN16RobloxExtraSpace6SharedEEC2IS2_EEPT_
// was: boost::shared_ptr<RobloxExtraSpace::Shared>::shared_ptr<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *)
pub fn stub_0xf55c74() {
    // IDA 0xf55c74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *)")]
// 0xf55c84 — j___ZN5boost6detail12shared_countC2IN16RobloxExtraSpace6SharedEEEPT_
pub fn stub_0xf55c84() {
    // IDA 0xf55c84: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,rbx_core::WeakPtr<RBX::GuiObject>,std::_Identity<rbx_core::WeakPtr<RBX::GuiObject>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<rbx_core::WeakPtr<RBX::GuiObject>>>::_M_create_node(rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// 0xf55c94 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE14_M_create_nodeERKS4_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_create_node(boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_0xf55c94() {
    // IDA 0xf55c94: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,rbx_core::WeakPtr<RBX::GuiObject>,std::_Identity<rbx_core::WeakPtr<RBX::GuiObject>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<rbx_core::WeakPtr<RBX::GuiObject>>>::_M_destroy_node(std::_Rb_tree_node<rbx_core::WeakPtr<RBX::GuiObject>> *)")]
// 0xf55ca4 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_destroy_node(std::_Rb_tree_node<boost::weak_ptr<RBX::GuiObject>> *)
pub fn stub_0xf55ca4() {
    // IDA 0xf55ca4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,rbx_core::WeakPtr<RBX::GuiObject>,std::_Identity<rbx_core::WeakPtr<RBX::GuiObject>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<rbx_core::WeakPtr<RBX::GuiObject>>>::_M_insert_unique(rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// 0xf55cb4 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE16_M_insert_uniqueERKS4_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_insert_unique(boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_0xf55cb4() {
    // IDA 0xf55cb4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,rbx_core::WeakPtr<RBX::GuiObject>,std::_Identity<rbx_core::WeakPtr<RBX::GuiObject>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<rbx_core::WeakPtr<RBX::GuiObject>>>::find(rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// 0xf55cc4 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE4findERKS4_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::find(boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_0xf55cc4() {
    // IDA 0xf55cc4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,rbx_core::WeakPtr<RBX::GuiObject>,std::_Identity<rbx_core::WeakPtr<RBX::GuiObject>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<rbx_core::WeakPtr<RBX::GuiObject>>>::_M_erase(std::_Rb_tree_node<rbx_core::WeakPtr<RBX::GuiObject>> *)")]
// 0xf55cd4 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_erase(std::_Rb_tree_node<boost::weak_ptr<RBX::GuiObject>> *)
pub fn stub_0xf55cd4() {
    // IDA 0xf55cd4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,rbx_core::WeakPtr<RBX::GuiObject>,std::_Identity<rbx_core::WeakPtr<RBX::GuiObject>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<rbx_core::WeakPtr<RBX::GuiObject>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// 0xf55ce4 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_0xf55ce4() {
    // IDA 0xf55ce4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::NotificationObject>::weak_ptr<RBX::NotificationObject>(rbx_core::SharedPtr<RBX::NotificationObject> const&,boost::detail::sp_enable_if_convertible<RBX::NotificationObject,RBX::NotificationObject>::type)")]
// 0xf55d34 — j___ZN5boost8weak_ptrIN3RBX18NotificationObjectEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// was: boost::weak_ptr<RBX::NotificationObject>::weak_ptr<RBX::NotificationObject>(boost::shared_ptr<RBX::NotificationObject> const&,boost::detail::sp_enable_if_convertible<RBX::NotificationObject,RBX::NotificationObject>::type)
pub fn stub_0xf55d34() {
    // IDA 0xf55d34: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::NotificationObject>,std::allocator<rbx_core::SharedPtr<RBX::NotificationObject>>>::_M_clear(void)")]
// 0xf55d54 — j___ZNSt10_List_baseIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_clearEv
// was: std::_List_base<boost::shared_ptr<RBX::NotificationObject>,std::allocator<boost::shared_ptr<RBX::NotificationObject>>>::_M_clear(void)
pub fn stub_0xf55d54() {
    // IDA 0xf55d54: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::NotificationObject>,std::allocator<rbx_core::SharedPtr<RBX::NotificationObject>>>::_M_create_node(rbx_core::SharedPtr<RBX::NotificationObject> const&)")]
// 0xf55d64 — j___ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE14_M_create_nodeERKS4_
// was: std::list<boost::shared_ptr<RBX::NotificationObject>,std::allocator<boost::shared_ptr<RBX::NotificationObject>>>::_M_create_node(boost::shared_ptr<RBX::NotificationObject> const&)
pub fn stub_0xf55d64() {
    // IDA 0xf55d64: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::NotificationObject>,std::allocator<rbx_core::SharedPtr<RBX::NotificationObject>>>::remove(rbx_core::SharedPtr<RBX::NotificationObject> const&)")]
// 0xf55d74 — j___ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE6removeERKS4_
// was: std::list<boost::shared_ptr<RBX::NotificationObject>,std::allocator<boost::shared_ptr<RBX::NotificationObject>>>::remove(boost::shared_ptr<RBX::NotificationObject> const&)
pub fn stub_0xf55d74() {
    // IDA 0xf55d74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::NotificationObject>,std::allocator<rbx_core::SharedPtr<RBX::NotificationObject>>>::_M_erase(std::_List_iterator<rbx_core::SharedPtr<RBX::NotificationObject>>)")]
// 0xf55d84 — j___ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
// was: std::list<boost::shared_ptr<RBX::NotificationObject>,std::allocator<boost::shared_ptr<RBX::NotificationObject>>>::_M_erase(std::_List_iterator<boost::shared_ptr<RBX::NotificationObject>>)
pub fn stub_0xf55d84() {
    // IDA 0xf55d84: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::NotificationObject::~NotificationObject()")]
// 0xf55df4 — j___ZN3RBX18NotificationObjectD2Ev
pub fn stub_0xf55df4() {
    // IDA 0xf55df4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ImageLabel>::operator=(rbx_core::SharedPtr<RBX::ImageLabel> const&)")]
// 0xf55e34 — j___ZN5boost10shared_ptrIN3RBX10ImageLabelEEaSERKS3_
// was: boost::shared_ptr<RBX::ImageLabel>::operator=(boost::shared_ptr<RBX::ImageLabel> const&)
pub fn stub_0xf55e34() {
    // IDA 0xf55e34: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiImageButton>::operator=(rbx_core::SharedPtr<RBX::GuiImageButton> const&)")]
// 0xf55e44 — j___ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEaSERKS3_
// was: boost::shared_ptr<RBX::GuiImageButton>::operator=(boost::shared_ptr<RBX::GuiImageButton> const&)
pub fn stub_0xf55e44() {
    // IDA 0xf55e44: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextLabel>::operator=(rbx_core::SharedPtr<RBX::TextLabel> const&)")]
// 0xf55e54 — j___ZN5boost10shared_ptrIN3RBX9TextLabelEEaSERKS3_
// was: boost::shared_ptr<RBX::TextLabel>::operator=(boost::shared_ptr<RBX::TextLabel> const&)
pub fn stub_0xf55e54() {
    // IDA 0xf55e54: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::FriendService::~FriendService()")]
// 0xf55f34 — j___ZN3RBX13FriendServiceD2Ev
pub fn stub_0xf55f34() {
    // IDA 0xf55f34: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::FriendService> RBX::weak_from<RBX::FriendService>(RBX::FriendService*)")]
// 0xf55f44 — j___ZN3RBX9weak_fromINS_13FriendServiceEEEN5boost8weak_ptrIT_EEPS4_
// was: boost::weak_ptr<RBX::FriendService> RBX::weak_from<RBX::FriendService>(RBX::FriendService*)
pub fn stub_0xf55f44() {
    // IDA 0xf55f44: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,RBX::FriendService::FriendStatus)>::remote_signal(void)")]
// 0xf55f54 — j___ZN3rbx13remote_signalIFviiN3RBX13FriendService12FriendStatusEEEC2Ev
pub fn stub_0xf55f54() {
    // IDA 0xf55f54: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,RBX::FriendService::FriendStatus)>::~remote_signal()")]
// 0xf55f64 — j___ZN3rbx13remote_signalIFviiN3RBX13FriendService12FriendStatusEEED2Ev
pub fn stub_0xf55f64() {
    // IDA 0xf55f64: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,RBX::FriendService::FriendEventType)>::remote_signal(void)")]
// 0xf55f74 — j___ZN3rbx13remote_signalIFviiN3RBX13FriendService15FriendEventTypeEEEC2Ev
pub fn stub_0xf55f74() {
    // IDA 0xf55f74: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,RBX::FriendService::FriendEventType)>::~remote_signal()")]
// 0xf55f84 — j___ZN3rbx13remote_signalIFviiN3RBX13FriendService15FriendEventTypeEEED2Ev
pub fn stub_0xf55f84() {
    // IDA 0xf55f84: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(int,int,RBX::FriendService::FriendStatus)>::operator()(int,int,RBX::FriendService::FriendStatus)")]
// 0xf55f94 — j___ZN3rbx7signals16signal_with_argsILi3EFviiN3RBX13FriendService12FriendStatusEEEclEiiS4_
pub fn stub_0xf55f94() {
    // IDA 0xf55f94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(int,int,RBX::FriendService::FriendEventType)>::operator()(int,int,RBX::FriendService::FriendEventType)")]
// 0xf55fa4 — j___ZN3rbx7signals16signal_with_argsILi3EFviiN3RBX13FriendService15FriendEventTypeEEEclEiiS4_
pub fn stub_0xf55fa4() {
    // IDA 0xf55fa4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::disconnectAll(void)")]
// 0xf55fb4 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE13disconnectAllEv
pub fn stub_0xf55fb4() {
    // IDA 0xf55fb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::safe_static_do_get_mutex(void)")]
// 0xf55fc4 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE24safe_static_do_get_mutexEv
pub fn stub_0xf55fc4() {
    // IDA 0xf55fc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

