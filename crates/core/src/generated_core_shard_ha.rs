//! core shard HA — 100 core stubs EA-sorted, 0xf55fd4..0xf56a54 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after GZ 0xf55fc4).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after GZ 0xf55fc4 (0xf55fd4..0xf56a54, 20414->20514 covered, 1404 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot> &)")]
// 0xf55fd4 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot> &)
pub fn stub_0xf55fd4() {
    // IDA 0xf55fd4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot::safe_static_do_get_mutex(void)")]
// 0xf55fe4 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0xf55fe4() {
    // IDA 0xf55fe4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::insert(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot *)")]
// 0xf55ff4 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE6insertEPNS6_4slotE
pub fn stub_0xf55ff4() {
    // IDA 0xf55ff4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::remove(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot *)")]
// 0xf56004 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE6removeEPNS6_4slotE
pub fn stub_0xf56004() {
    // IDA 0xf56004: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
// 0xf56014 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvS3_iiS4_EENS9_5list4INS9_5valueIPS3_EENS8_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEEENS0_10connectionERKT_
pub fn stub_0xf56014() {
    // IDA 0xf56014: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::connect<boost::function<void ()(int,int,RBX::FriendService::FriendStatus)>>(boost::function<void ()(int,int,RBX::FriendService::FriendStatus)> const&)")]
// 0xf56024 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
pub fn stub_0xf56024() {
    // IDA 0xf56024: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::on_error(std::exception &)")]
// 0xf56034 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE8on_errorERSt9exception
pub fn stub_0xf56034() {
    // IDA 0xf56034: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::disconnectAll(void)")]
// 0xf56044 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE13disconnectAllEv
pub fn stub_0xf56044() {
    // IDA 0xf56044: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::safe_static_do_get_mutex(void)")]
// 0xf56054 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE24safe_static_do_get_mutexEv
pub fn stub_0xf56054() {
    // IDA 0xf56054: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot> &)")]
// 0xf56064 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot> &)
pub fn stub_0xf56064() {
    // IDA 0xf56064: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot::safe_static_do_get_mutex(void)")]
// 0xf56074 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0xf56074() {
    // IDA 0xf56074: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::insert(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot *)")]
// 0xf56084 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE6insertEPNS6_4slotE
pub fn stub_0xf56084() {
    // IDA 0xf56084: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::remove(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot *)")]
// 0xf56094 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE6removeEPNS6_4slotE
pub fn stub_0xf56094() {
    // IDA 0xf56094: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType>,boost::_bi::list4<boost::_bi::value<RBX::FriendService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
// 0xf560a4 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvS3_iiS4_EENS9_5list4INS9_5valueIPS3_EENS8_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEEENS0_10connectionERKT_
pub fn stub_0xf560a4() {
    // IDA 0xf560a4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::connect<boost::function<void ()(int,int,RBX::FriendService::FriendEventType)>>(boost::function<void ()(int,int,RBX::FriendService::FriendEventType)> const&)")]
// 0xf560b4 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
pub fn stub_0xf560b4() {
    // IDA 0xf560b4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::on_error(std::exception &)")]
// 0xf560c4 — j___ZN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE8on_errorERSt9exception
pub fn stub_0xf560c4() {
    // IDA 0xf560c4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendStatus)>,3,void ()(int,int,RBX::FriendService::FriendStatus)>::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>*>(boost::function<void ()(int,int,RBX::FriendService::FriendStatus)> const&,rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>*)")]
// 0xf560d4 — j___ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
pub fn stub_0xf560d4() {
    // IDA 0xf560d4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot,boost::function<void ()(int,int,RBX::FriendService::FriendEventType)>,3,void ()(int,int,RBX::FriendService::FriendEventType)>::callable<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>*>(boost::function<void ()(int,int,RBX::FriendService::FriendEventType)> const&,rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>*)")]
// 0xf560e4 — j___ZN3rbx8callableINS_7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
pub fn stub_0xf560e4() {
    // IDA 0xf560e4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FriendService>::shared_ptr<RBX::FriendService>(rbx_core::WeakPtr<RBX::FriendService> const&,boost::detail::sp_nothrow_tag)")]
// 0xf560f4 — j___ZN5boost10shared_ptrIN3RBX13FriendServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::FriendService>::shared_ptr<RBX::FriendService>(boost::weak_ptr<RBX::FriendService> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0xf560f4() {
    // IDA 0xf560f4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>(std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>> *)")]
// 0xf56104 — j___ZN5boost10shared_ptrISt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS4_EEEEC2ISB_EEPT_
// was: boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>(std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>> *)
pub fn stub_0xf56104() {
    // IDA 0xf56104: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot>::operator=(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot*)")]
// 0xf56114 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot>::operator=(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot*)
pub fn stub_0xf56114() {
    // IDA 0xf56114: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot> const&)")]
// 0xf56124 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiN3RBX13FriendService12FriendStatusEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendStatus)>::slot> const&)
pub fn stub_0xf56124() {
    // IDA 0xf56124: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot>::operator=(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot*)")]
// 0xf56134 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot>::operator=(rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot*)
pub fn stub_0xf56134() {
    // IDA 0xf56134: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot> const&)")]
// 0xf56144 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiN3RBX13FriendService15FriendEventTypeEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int,RBX::FriendService::FriendEventType)>::slot> const&)
pub fn stub_0xf56144() {
    // IDA 0xf56144: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>)")]
// 0xf56154 — j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_INS_10shared_ptrISt3mapIiNS5_12FriendStatusESt4lessIiESaISt4pairIKiSB_EEEEEEEEC2ES7_S8_SK_
// was: boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::list3(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>)
pub fn stub_0xf56154() {
    // IDA 0xf56154: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::FriendService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus>,boost::_bi::list3<int &,int &,RBX::FriendService::FriendStatus&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendStatus> &,boost::_bi::list3<int &,int &,RBX::FriendService::FriendStatus&> &,int)")]
// 0xf56194 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX13FriendServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_iiNS4_12FriendStatusEEENS0_5list3IRiSI_RSF_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0xf56194() {
    // IDA 0xf56194: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::FriendService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType>,boost::_bi::list3<int &,int &,RBX::FriendService::FriendEventType&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::FriendService,int,int,RBX::FriendService::FriendEventType> &,boost::_bi::list3<int &,int &,RBX::FriendService::FriendEventType&> &,int)")]
// 0xf561a4 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX13FriendServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_iiNS4_15FriendEventTypeEEENS0_5list3IRiSI_RSF_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0xf561a4() {
    // IDA 0xf561a4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>)")]
// 0xf561b4 — j___ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_ISt3setIiSt4lessIiESaIiEEEENS_3argILi1EEENSF_ILi2EEEEC2ES7_S8_SE_SG_SH_
// was: boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>::list5(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>)
pub fn stub_0xf561b4() {
    // IDA 0xf561b4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>::operator()<void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0xf561c4 — j___ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_ISt3setIiSt4lessIiESaIiEEEENS_3argILi1EEENSF_ILi2EEEEclIPFvS6_iSD_PSsPSt9exceptionENS0_5list2IRSK_RSM_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>::operator()<void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)
pub fn stub_0xf561c4() {
    // IDA 0xf561c4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>)")]
// 0xf561f4 — j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEEEC2ES7_S8_
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>)
pub fn stub_0xf561f4() {
    // IDA 0xf561f4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>)")]
// 0xf56204 — j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_INS_10shared_ptrISt3mapIiNS5_12FriendStatusESt4lessIiESaISt4pairIKiSB_EEEEEEEEC2ES7_S8_SK_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>)
pub fn stub_0xf56204() {
    // IDA 0xf56204: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>)")]
// 0xf56214 — j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_ISt3setIiSt4lessIiESaIiEEEEEC2ES7_S8_SE_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>)
pub fn stub_0xf56214() {
    // IDA 0xf56214: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>)")]
// 0xf56224 — j___ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_ISt3setIiSt4lessIiESaIiEEEENS_3argILi1EEEEC2ES7_S8_SE_SG_
// was: boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>)
pub fn stub_0xf56224() {
    // IDA 0xf56224: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>)")]
// 0xf56234 — j___ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_ISt3setIiSt4lessIiESaIiEEEENS_3argILi1EEENSF_ILi2EEEEC2ES7_S8_SE_SG_SH_
// was: boost::_bi::storage5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>)
pub fn stub_0xf56234() {
    // IDA 0xf56234: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>(void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>)")]
// 0xf56284 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX13FriendServiceEEEiNS_10shared_ptrISt3mapIiNS3_12FriendStatusESt4lessIiESaISt4pairIKiS7_EEEEES4_iSF_EENS_3_bi6bind_tIT_PFSI_T0_T1_T2_ENSG_9list_av_3IT3_T4_T5_E4typeEEESN_SP_SQ_SR_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list_av_3<boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>::type> boost::bind<void,boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>(void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>)
pub fn stub_0xf56284() {
    // IDA 0xf56284: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *,rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>>(void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>)")]
// 0xf56294 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEEPSsPSt9exceptionS4_iS9_NS_3argILi1EEENSD_ILi2EEEEENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_T4_ENSG_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESP_SR_SS_ST_SU_SV_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list_av_5<boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *,boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>>(void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>)
pub fn stub_0xf56294() {
    // IDA 0xf56294: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>(std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>> *)")]
// 0xf562a4 — j___ZN5boost6detail12shared_countC2ISt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS6_EEEEEPT_
pub fn stub_0xf562a4() {
    // IDA 0xf562a4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf562e4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiNS_10shared_ptrISt3mapIiNS7_12FriendStatusESt4lessIiESaISt4pairIKiSB_EEEEEENS3_5list3INS3_5valueIS8_EENSN_IiEENSN_ISJ_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf562e4() {
    // IDA 0xf562e4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf562f4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEEPSsPSt9exceptionENS3_5list5INS3_5valueIS8_EENSK_IiEENSK_ISD_EENS_3argILi1EEENSO_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf562f4() {
    // IDA 0xf562f4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::FriendService>::weak_ptr<RBX::FriendService>(rbx_core::SharedPtr<RBX::FriendService> const&,boost::detail::sp_enable_if_convertible<RBX::FriendService,RBX::FriendService>::type)")]
// 0xf56354 — j___ZN5boost8weak_ptrIN3RBX13FriendServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// was: boost::weak_ptr<RBX::FriendService>::weak_ptr<RBX::FriendService>(boost::shared_ptr<RBX::FriendService> const&,boost::detail::sp_enable_if_convertible<RBX::FriendService,RBX::FriendService>::type)
pub fn stub_0xf56354() {
    // IDA 0xf56354: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>)")]
// 0xf56394 — j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEES1_S3_ENS6_5list5INS6_5valueISB_EENSK_IiEENSK_ISG_EENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_
// was: void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_0xf56394() {
    // IDA 0xf56394: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function3<void,int,int,RBX::FriendService::FriendStatus>::assign_to_own(boost::function3<void,int,int,RBX::FriendService::FriendStatus> const&)")]
// 0xf563c4 — j___ZN5boost9function3IviiN3RBX13FriendService12FriendStatusEE13assign_to_ownERKS4_
pub fn stub_0xf563c4() {
    // IDA 0xf563c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function3<void,int,int,RBX::FriendService::FriendStatus>::clear(void)")]
// 0xf563d4 — j___ZN5boost9function3IviiN3RBX13FriendService12FriendStatusEE5clearEv
pub fn stub_0xf563d4() {
    // IDA 0xf563d4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function3<void,int,int,RBX::FriendService::FriendEventType>::assign_to_own(boost::function3<void,int,int,RBX::FriendService::FriendEventType> const&)")]
// 0xf56404 — j___ZN5boost9function3IviiN3RBX13FriendService15FriendEventTypeEE13assign_to_ownERKS4_
pub fn stub_0xf56404() {
    // IDA 0xf56404: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function3<void,int,int,RBX::FriendService::FriendEventType>::clear(void)")]
// 0xf56414 — j___ZN5boost9function3IviiN3RBX13FriendService15FriendEventTypeEE5clearEv
pub fn stub_0xf56414() {
    // IDA 0xf56414: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf564b4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEES3_S5_ENS8_5list5INS8_5valueISD_EENSM_IiEENSM_ISI_EENS_3argILi1EEENSQ_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf564b4() {
    // IDA 0xf564b4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// 0xf564e4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEES3_S5_ENS8_5list5INS8_5valueISD_EENSM_IiEENSM_ISI_EENS_3argILi1EEENSQ_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf564e4() {
    // IDA 0xf564e4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf564f4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEES3_S5_ENS8_5list5INS8_5valueISD_EENSM_IiEENSM_ISI_EENS_3argILi1EEENSQ_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<std::set<int,std::less<int>,std::allocator<int>>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf564f4() {
    // IDA 0xf564f4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function3<void,int,int,RBX::FriendService::FriendStatus>::operator()(int,int,RBX::FriendService::FriendStatus)const")]
// 0xf56564 — j___ZNK5boost9function3IviiN3RBX13FriendService12FriendStatusEEclEiiS3_
pub fn stub_0xf56564() {
    // IDA 0xf56564: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function3<void,int,int,RBX::FriendService::FriendEventType>::operator()(int,int,RBX::FriendService::FriendEventType)const")]
// 0xf56574 — j___ZNK5boost9function3IviiN3RBX13FriendService15FriendEventTypeEEclEiiS3_
pub fn stub_0xf56574() {
    // IDA 0xf56574: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::_M_allocate(unsigned long)")]
// 0xf56584 — j___ZNSt12_Vector_baseIN3RBX13FriendService12FriendStatusESaIS2_EE11_M_allocateEm
pub fn stub_0xf56584() {
    // IDA 0xf56584: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::_M_allocate(unsigned long)")]
// 0xf56594 — j___ZNSt12_Vector_baseIN3RBX13FriendService15FriendEventTypeESaIS2_EE11_M_allocateEm
pub fn stub_0xf56594() {
    // IDA 0xf56594: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::FriendService::FriendStatus * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::FriendService::FriendStatus *,RBX::FriendService::FriendStatus *>(RBX::FriendService::FriendStatus *,RBX::FriendService::FriendStatus *,RBX::FriendService::FriendStatus *)")]
// 0xf565a4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13FriendService12FriendStatusES6_EET0_T_S8_S7_
pub fn stub_0xf565a4() {
    // IDA 0xf565a4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "RBX::FriendService::FriendEventType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::FriendService::FriendEventType *,RBX::FriendService::FriendEventType *>(RBX::FriendService::FriendEventType *,RBX::FriendService::FriendEventType *,RBX::FriendService::FriendEventType *)")]
// 0xf565b4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13FriendService15FriendEventTypeES6_EET0_T_S8_S7_
pub fn stub_0xf565b4() {
    // IDA 0xf565b4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::FriendService::FriendStatus,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::operator[](RBX::Name const* const&)")]
// 0xf565c4 — j___ZNSt3mapIPKN3RBX4NameENS0_13FriendService12FriendStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf565c4() {
    // IDA 0xf565c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::FriendService::FriendEventType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::operator[](RBX::Name const* const&)")]
// 0xf565d4 — j___ZNSt3mapIPKN3RBX4NameENS0_13FriendService15FriendEventTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf565d4() {
    // IDA 0xf565d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::operator[](int const&)")]
// 0xf565e4 — j___ZNSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS2_EEEixERS6_
pub fn stub_0xf565e4() {
    // IDA 0xf565e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<int,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::operator[](int const&)")]
// 0xf565f4 — j___ZNSt3mapIiS_IiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS2_EEES4_SaIS5_IS6_S9_EEEixERS6_
pub fn stub_0xf565f4() {
    // IDA 0xf565f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FriendService::FriendStatus*,std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>>,RBX::FriendService::FriendStatus const&)")]
// 0xf56604 — j___ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf56604() {
    // IDA 0xf56604: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FriendService::FriendStatus*,std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>>,unsigned long,RBX::FriendService::FriendStatus const&)")]
// 0xf56614 — j___ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf56614() {
    // IDA 0xf56614: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::resize(unsigned long,RBX::FriendService::FriendStatus)")]
// 0xf56624 — j___ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE6resizeEmS2_
pub fn stub_0xf56624() {
    // IDA 0xf56624: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendStatus,std::allocator<RBX::FriendService::FriendStatus>>::push_back(RBX::FriendService::FriendStatus const&)")]
// 0xf56634 — j___ZNSt6vectorIN3RBX13FriendService12FriendStatusESaIS2_EE9push_backERKS2_
pub fn stub_0xf56634() {
    // IDA 0xf56634: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FriendService::FriendEventType*,std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>>,RBX::FriendService::FriendEventType const&)")]
// 0xf56644 — j___ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf56644() {
    // IDA 0xf56644: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FriendService::FriendEventType*,std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>>,unsigned long,RBX::FriendService::FriendEventType const&)")]
// 0xf56654 — j___ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf56654() {
    // IDA 0xf56654: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::resize(unsigned long,RBX::FriendService::FriendEventType)")]
// 0xf56664 — j___ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE6resizeEmS2_
pub fn stub_0xf56664() {
    // IDA 0xf56664: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FriendService::FriendEventType,std::allocator<RBX::FriendService::FriendEventType>>::push_back(RBX::FriendService::FriendEventType const&)")]
// 0xf56674 — j___ZNSt6vectorIN3RBX13FriendService15FriendEventTypeESaIS2_EE9push_backERKS2_
pub fn stub_0xf56674() {
    // IDA 0xf56674: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus> const&)")]
// 0xf56684 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf56684() {
    // IDA 0xf56684: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus> const&)")]
// 0xf56694 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf56694() {
    // IDA 0xf56694: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus> const&)")]
// 0xf566a4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf566a4() {
    // IDA 0xf566a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType> const&)")]
// 0xf566b4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf566b4() {
    // IDA 0xf566b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType> const&)")]
// 0xf566c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf566c4() {
    // IDA 0xf566c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType> const&)")]
// 0xf566d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf566d4() {
    // IDA 0xf566d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::lower_bound(std::pair<int,int> const&)")]
// 0xf566e4 — j___ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE11lower_boundERKS1_
pub fn stub_0xf566e4() {
    // IDA 0xf566e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::upper_bound(std::pair<int,int> const&)")]
// 0xf566f4 — j___ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE11upper_boundERKS1_
pub fn stub_0xf566f4() {
    // IDA 0xf566f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::_M_insert_unique(std::pair<int,int> const&)")]
// 0xf56704 — j___ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE16_M_insert_uniqueERKS1_
pub fn stub_0xf56704() {
    // IDA 0xf56704: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::find(std::pair<int,int> const&)")]
// 0xf56714 — j___ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE4findERKS1_
pub fn stub_0xf56714() {
    // IDA 0xf56714: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::erase(std::pair<int,int> const&)")]
// 0xf56724 — j___ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE5eraseERKS1_
pub fn stub_0xf56724() {
    // IDA 0xf56724: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::erase(std::_Rb_tree_iterator<std::pair<int,int>>,std::_Rb_tree_iterator<std::pair<int,int>>)")]
// 0xf56734 — j___ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE5eraseESt17_Rb_tree_iteratorIS1_ES9_
pub fn stub_0xf56734() {
    // IDA 0xf56734: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::_M_erase(std::_Rb_tree_node<std::pair<int,int>> *)")]
// 0xf56744 — j___ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE8_M_eraseEPSt13_Rb_tree_nodeIS1_E
pub fn stub_0xf56744() {
    // IDA 0xf56744: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::pair<int,int>,std::pair<int,int>,std::_Identity<std::pair<int,int>>,std::less<std::pair<int,int>>,std::allocator<std::pair<int,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int,int> const&)")]
// 0xf56754 — j___ZNSt8_Rb_treeISt4pairIiiES1_St9_IdentityIS1_ESt4lessIS1_ESaIS1_EE9_M_insertEPSt18_Rb_tree_node_baseS9_RKS1_
pub fn stub_0xf56754() {
    // IDA 0xf56754: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::_M_insert_unique(std::pair<int const,RBX::FriendService::FriendStatus> const&)")]
// 0xf56764 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX13FriendService12FriendStatusEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_0xf56764() {
    // IDA 0xf56764: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,RBX::FriendService::FriendStatus>>,std::pair<int const,RBX::FriendService::FriendStatus> const&)")]
// 0xf56774 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX13FriendService12FriendStatusEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_0xf56774() {
    // IDA 0xf56774: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::_M_copy(std::_Rb_tree_node<std::pair<int const,RBX::FriendService::FriendStatus>> const*,std::_Rb_tree_node<std::pair<int const,RBX::FriendService::FriendStatus>>*)")]
// 0xf56784 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX13FriendService12FriendStatusEESt10_Select1stIS5_ESt4lessIiESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
pub fn stub_0xf56784() {
    // IDA 0xf56784: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,RBX::FriendService::FriendStatus>> *)")]
// 0xf56794 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX13FriendService12FriendStatusEESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0xf56794() {
    // IDA 0xf56794: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,RBX::FriendService::FriendStatus> const&)")]
// 0xf567a4 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX13FriendService12FriendStatusEESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_0xf567a4() {
    // IDA 0xf567a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::_Rb_tree(std::_Rb_tree<int,std::pair<int const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<int const,RBX::FriendService::FriendStatus>>,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>> const&)")]
// 0xf567b4 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX13FriendService12FriendStatusEESt10_Select1stIS5_ESt4lessIiESaIS5_EEC2ERKSB_
pub fn stub_0xf567b4() {
    // IDA 0xf567b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,std::_Select1st<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::_M_create_node(std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>> const&)")]
// 0xf567c4 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaIS0_IS1_S5_EEEESt10_Select1stISB_ES7_SaISB_EE14_M_create_nodeERKSB_
pub fn stub_0xf567c4() {
    // IDA 0xf567c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,std::_Select1st<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::_M_insert_unique(std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>> const&)")]
// 0xf567d4 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaIS0_IS1_S5_EEEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
pub fn stub_0xf567d4() {
    // IDA 0xf567d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,std::_Select1st<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>> const&)")]
// 0xf567e4 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaIS0_IS1_S5_EEEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
pub fn stub_0xf567e4() {
    // IDA 0xf567e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,std::_Select1st<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::erase(std::_Rb_tree_iterator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>)")]
// 0xf567f4 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaIS0_IS1_S5_EEEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_E
pub fn stub_0xf567f4() {
    // IDA 0xf567f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,std::_Select1st<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>> *)")]
// 0xf56804 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaIS0_IS1_S5_EEEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
pub fn stub_0xf56804() {
    // IDA 0xf56804: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,std::_Select1st<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>> const&)")]
// 0xf56814 — j___ZNSt8_Rb_treeIiSt4pairIKiSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaIS0_IS1_S5_EEEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
pub fn stub_0xf56814() {
    // IDA 0xf56814: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::equal_range(int const&)")]
// 0xf56824 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE11equal_rangeERKi
pub fn stub_0xf56824() {
    // IDA 0xf56824: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::erase(int const&)")]
// 0xf56834 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE5eraseERKi
pub fn stub_0xf56834() {
    // IDA 0xf56834: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::erase(std::_Rb_tree_iterator<int>,std::_Rb_tree_iterator<int>)")]
// 0xf56844 — j___ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE5eraseESt17_Rb_tree_iteratorIiES7_
pub fn stub_0xf56844() {
    // IDA 0xf56844: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::~GameBasicSettings()")]
// 0xf56984 — j___ZN3RBX17GameBasicSettingsD2Ev
pub fn stub_0xf56984() {
    // IDA 0xf56984: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>::_M_allocate(unsigned long)")]
// 0xf56a24 — j___ZNSt12_Vector_baseIN3RBX17GameBasicSettings11ControlModeESaIS2_EE11_M_allocateEm
pub fn stub_0xf56a24() {
    // IDA 0xf56a24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::_M_allocate(unsigned long)")]
// 0xf56a34 — j___ZNSt12_Vector_baseIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE11_M_allocateEm
pub fn stub_0xf56a34() {
    // IDA 0xf56a34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GameBasicSettings::ControlMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameBasicSettings::ControlMode *,RBX::GameBasicSettings::ControlMode *>(RBX::GameBasicSettings::ControlMode *,RBX::GameBasicSettings::ControlMode *,RBX::GameBasicSettings::ControlMode *)")]
// 0xf56a44 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17GameBasicSettings11ControlModeES6_EET0_T_S8_S7_
pub fn stub_0xf56a44() {
    // IDA 0xf56a44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GameBasicSettings::RenderQualitySetting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameBasicSettings::RenderQualitySetting *,RBX::GameBasicSettings::RenderQualitySetting *>(RBX::GameBasicSettings::RenderQualitySetting *,RBX::GameBasicSettings::RenderQualitySetting *,RBX::GameBasicSettings::RenderQualitySetting *)")]
// 0xf56a54 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17GameBasicSettings20RenderQualitySettingES6_EET0_T_S8_S7_
pub fn stub_0xf56a54() {
    // IDA 0xf56a54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
