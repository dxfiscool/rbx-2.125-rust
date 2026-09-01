//! core shard BD — 100 core stubs EA-sorted, next uncovered after BC 0x434768 (strict RBX|boost|std earliest gap, after BC 0x411db4..0x434768).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x434768.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::StarterGuiService * RBX::ServiceProvider::create<RBX::StarterGuiService>(void)const")]
// 0x434850 — __ZNK3RBX15ServiceProvider6createINS_17StarterGuiServiceEEEPT_v — RBX::StarterGuiService * RBX::ServiceProvider::create<RBX::StarterGuiService>(void)const
pub fn stub_0x434850() -> ! {
    todo!("0x434850 __ZNK3RBX15ServiceProvider6createINS_17StarterGuiServiceEEEPT_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CoreGuiService>::operator=(rbx_core::SharedPtr<RBX::CoreGuiService> const&)")]
// 0x434a18 — __ZN5boost10shared_ptrIN3RBX14CoreGuiServiceEEaSERKS3_ — rbx_core::SharedPtr<RBX::CoreGuiService>::operator=(rbx_core::SharedPtr<RBX::CoreGuiService> const&)
pub fn stub_0x434a18() -> ! {
    todo!("0x434a18 __ZN5boost10shared_ptrIN3RBX14CoreGuiServiceEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CoreGuiService> RBX::shared_from<RBX::CoreGuiService>(RBX::CoreGuiService*)")]
// 0x434a50 — __ZN3RBX11shared_fromINS_14CoreGuiServiceEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::CoreGuiService> RBX::shared_from<RBX::CoreGuiService>(RBX::CoreGuiService*)
pub fn stub_0x434a50() -> ! {
    todo!("0x434a50 __ZN3RBX11shared_fromINS_14CoreGuiServiceEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::CoreGuiService * RBX::ServiceProvider::create<RBX::CoreGuiService>(void)const")]
// 0x434b38 — __ZNK3RBX15ServiceProvider6createINS_14CoreGuiServiceEEEPT_v — RBX::CoreGuiService * RBX::ServiceProvider::create<RBX::CoreGuiService>(void)const
pub fn stub_0x434b38() -> ! {
    todo!("0x434b38 __ZNK3RBX15ServiceProvider6createINS_14CoreGuiServiceEEEPT_v")
}

#[doc(alias = "RBX::JointsService * RBX::ServiceProvider::create<RBX::JointsService>(void)const")]
// 0x434f50 — __ZNK3RBX15ServiceProvider6createINS_13JointsServiceEEEPT_v — RBX::JointsService * RBX::ServiceProvider::create<RBX::JointsService>(void)const
pub fn stub_0x434f50() -> ! {
    todo!("0x434f50 __ZNK3RBX15ServiceProvider6createINS_13JointsServiceEEEPT_v")
}

#[doc(alias = "RBX::CollectionService * RBX::ServiceProvider::create<RBX::CollectionService>(void)const")]
// 0x435118 — __ZNK3RBX15ServiceProvider6createINS_17CollectionServiceEEEPT_v — RBX::CollectionService * RBX::ServiceProvider::create<RBX::CollectionService>(void)const
pub fn stub_0x435118() -> ! {
    todo!("0x435118 __ZNK3RBX15ServiceProvider6createINS_17CollectionServiceEEEPT_v")
}

#[doc(alias = "RBX::PhysicsService * RBX::ServiceProvider::create<RBX::PhysicsService>(void)const")]
// 0x4352e0 — __ZNK3RBX15ServiceProvider6createINS_14PhysicsServiceEEEPT_v — RBX::PhysicsService * RBX::ServiceProvider::create<RBX::PhysicsService>(void)const
pub fn stub_0x4352e0() -> ! {
    todo!("0x4352e0 __ZNK3RBX15ServiceProvider6createINS_14PhysicsServiceEEEPT_v")
}

#[doc(alias = "RBX::BadgeService * RBX::ServiceProvider::create<RBX::BadgeService>(void)const")]
// 0x4354a8 — __ZNK3RBX15ServiceProvider6createINS_12BadgeServiceEEEPT_v — RBX::BadgeService * RBX::ServiceProvider::create<RBX::BadgeService>(void)const
pub fn stub_0x4354a8() -> ! {
    todo!("0x4354a8 __ZNK3RBX15ServiceProvider6createINS_12BadgeServiceEEEPT_v")
}

#[doc(alias = "RBX::GeometryService * RBX::ServiceProvider::create<RBX::GeometryService>(void)const")]
// 0x435684 — __ZNK3RBX15ServiceProvider6createINS_15GeometryServiceEEEPT_v — RBX::GeometryService * RBX::ServiceProvider::create<RBX::GeometryService>(void)const
pub fn stub_0x435684() -> ! {
    todo!("0x435684 __ZNK3RBX15ServiceProvider6createINS_15GeometryServiceEEEPT_v")
}

#[doc(alias = "RBX::FriendService * RBX::ServiceProvider::create<RBX::FriendService>(void)const")]
// 0x43584c — __ZNK3RBX15ServiceProvider6createINS_13FriendServiceEEEPT_v — RBX::FriendService * RBX::ServiceProvider::create<RBX::FriendService>(void)const
pub fn stub_0x43584c() -> ! {
    todo!("0x43584c __ZNK3RBX15ServiceProvider6createINS_13FriendServiceEEEPT_v")
}

#[doc(alias = "RBX::RenderHooksService * RBX::ServiceProvider::create<RBX::RenderHooksService>(void)const")]
// 0x435a28 — __ZNK3RBX15ServiceProvider6createINS_18RenderHooksServiceEEEPT_v — RBX::RenderHooksService * RBX::ServiceProvider::create<RBX::RenderHooksService>(void)const
pub fn stub_0x435a28() -> ! {
    todo!("0x435a28 __ZNK3RBX15ServiceProvider6createINS_18RenderHooksServiceEEEPT_v")
}

#[doc(alias = "RBX::InsertService * RBX::ServiceProvider::create<RBX::InsertService>(void)const")]
// 0x435bf0 — __ZNK3RBX15ServiceProvider6createINS_13InsertServiceEEEPT_v — RBX::InsertService * RBX::ServiceProvider::create<RBX::InsertService>(void)const
pub fn stub_0x435bf0() -> ! {
    todo!("0x435bf0 __ZNK3RBX15ServiceProvider6createINS_13InsertServiceEEEPT_v")
}

#[doc(alias = "RBX::SocialService * RBX::ServiceProvider::create<RBX::SocialService>(void)const")]
// 0x435dcc — __ZNK3RBX15ServiceProvider6createINS_13SocialServiceEEEPT_v — RBX::SocialService * RBX::ServiceProvider::create<RBX::SocialService>(void)const
pub fn stub_0x435dcc() -> ! {
    todo!("0x435dcc __ZNK3RBX15ServiceProvider6createINS_13SocialServiceEEEPT_v")
}

#[doc(alias = "RBX::GamePassService * RBX::ServiceProvider::create<RBX::GamePassService>(void)const")]
// 0x435f94 — __ZNK3RBX15ServiceProvider6createINS_15GamePassServiceEEEPT_v — RBX::GamePassService * RBX::ServiceProvider::create<RBX::GamePassService>(void)const
pub fn stub_0x435f94() -> ! {
    todo!("0x435f94 __ZNK3RBX15ServiceProvider6createINS_15GamePassServiceEEEPT_v")
}

#[doc(alias = "RBX::DebrisService * RBX::ServiceProvider::create<RBX::DebrisService>(void)const")]
// 0x43615c — __ZNK3RBX15ServiceProvider6createINS_13DebrisServiceEEEPT_v — RBX::DebrisService * RBX::ServiceProvider::create<RBX::DebrisService>(void)const
pub fn stub_0x43615c() -> ! {
    todo!("0x43615c __ZNK3RBX15ServiceProvider6createINS_13DebrisServiceEEEPT_v")
}

#[doc(alias = "RBX::CookiesService * RBX::ServiceProvider::create<RBX::CookiesService>(void)const")]
// 0x436324 — __ZNK3RBX15ServiceProvider6createINS_14CookiesServiceEEEPT_v — RBX::CookiesService * RBX::ServiceProvider::create<RBX::CookiesService>(void)const
pub fn stub_0x436324() -> ! {
    todo!("0x436324 __ZNK3RBX15ServiceProvider6createINS_14CookiesServiceEEEPT_v")
}

#[doc(alias = "RBX::TeleportService * RBX::ServiceProvider::create<RBX::TeleportService>(void)const")]
// 0x4364ec — __ZNK3RBX15ServiceProvider6createINS_15TeleportServiceEEEPT_v — RBX::TeleportService * RBX::ServiceProvider::create<RBX::TeleportService>(void)const
pub fn stub_0x4364ec() -> ! {
    todo!("0x4364ec __ZNK3RBX15ServiceProvider6createINS_15TeleportServiceEEEPT_v")
}

#[doc(alias = "RBX::PersonalServerService * RBX::ServiceProvider::create<RBX::PersonalServerService>(void)const")]
// 0x4366b4 — __ZNK3RBX15ServiceProvider6createINS_21PersonalServerServiceEEEPT_v — RBX::PersonalServerService * RBX::ServiceProvider::create<RBX::PersonalServerService>(void)const
pub fn stub_0x4366b4() -> ! {
    todo!("0x4366b4 __ZNK3RBX15ServiceProvider6createINS_21PersonalServerServiceEEEPT_v")
}

#[doc(alias = "RBX::FWService * RBX::ServiceProvider::create<RBX::FWService>(void)const")]
// 0x43687c — __ZNK3RBX15ServiceProvider6createINS_9FWServiceEEEPT_v — RBX::FWService * RBX::ServiceProvider::create<RBX::FWService>(void)const
pub fn stub_0x43687c() -> ! {
    todo!("0x43687c __ZNK3RBX15ServiceProvider6createINS_9FWServiceEEEPT_v")
}

#[doc(alias = "RBX::ContextActionService * RBX::ServiceProvider::create<RBX::ContextActionService>(void)const")]
// 0x436a48 — __ZNK3RBX15ServiceProvider6createINS_20ContextActionServiceEEEPT_v — RBX::ContextActionService * RBX::ServiceProvider::create<RBX::ContextActionService>(void)const
pub fn stub_0x436a48() -> ! {
    todo!("0x436a48 __ZNK3RBX15ServiceProvider6createINS_20ContextActionServiceEEEPT_v")
}

#[doc(alias = "RBX::AssetService * RBX::ServiceProvider::create<RBX::AssetService>(void)const")]
// 0x436dd8 — __ZNK3RBX15ServiceProvider6createINS_12AssetServiceEEEPT_v — RBX::AssetService * RBX::ServiceProvider::create<RBX::AssetService>(void)const
pub fn stub_0x436dd8() -> ! {
    todo!("0x436dd8 __ZNK3RBX15ServiceProvider6createINS_12AssetServiceEEEPT_v")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>::type> boost::bind<void,std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>,boost::arg<1>,boost::arg<2>,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>(void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::arg<1>,boost::arg<2>,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0x437060 — __ZN5boost4bindIvPSsPSt9exceptionNS_8functionIFvSsEEES6_NS_3argILi1EEENS7_ILi2EEES6_S6_EENS_3_bi6bind_tIT_PFSC_T0_T1_T2_T3_ENSA_9list_av_4IT4_T5_T6_T7_E4typeEEESI_SK_SL_SM_SN_ — boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>::type> boost::bind<void,std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>,boost::arg<1>,boost::arg<2>,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>(void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::arg<1>,boost::arg<2>,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
pub fn stub_0x437060() -> ! {
    todo!("0x437060 __ZN5boost4bindIvPSsPSt9exceptionNS_8functionIFvSsEEES6_NS_3argILi1EEENS7_ILi2EEES6_S6_EENS_3_bi6bind_tIT_PFSC_T0_T1_T2_T3_ENSA_9list_av_4IT4_T5_T6_T7_E4typeEEESI_SK_SL_SM_SN_")
}

#[doc(alias = "boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list_av_1<std::string>::type> boost::bind<std::string,std::string const&,std::string>(std::string (*)(std::string const&),std::string)")]
// 0x437214 — __ZN5boost4bindISsRKSsSsEENS_3_bi6bind_tIT_PFS5_T0_ENS3_9list_av_1IT1_E4typeEEES8_SA_ — boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list_av_1<std::string>::type> boost::bind<std::string,std::string const&,std::string>(std::string (*)(std::string const&),std::string)
pub fn stub_0x437214() -> ! {
    todo!("0x437214 __ZN5boost4bindISsRKSsSsEENS_3_bi6bind_tIT_PFS5_T0_ENS3_9list_av_1IT1_E4typeEEES8_SA_")
}

#[doc(alias = "boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list_av_2<std::string,std::string>::type> boost::bind<std::string,std::string const&,std::string const&,std::string,std::string>(std::string (*)(std::string const&,std::string const&),std::string,std::string)")]
// 0x4373bc — __ZN5boost4bindISsRKSsS2_SsSsEENS_3_bi6bind_tIT_PFS5_T0_T1_ENS3_9list_av_2IT2_T3_E4typeEEES9_SB_SC_ — boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list_av_2<std::string,std::string>::type> boost::bind<std::string,std::string const&,std::string const&,std::string,std::string>(std::string (*)(std::string const&,std::string const&),std::string,std::string)
pub fn stub_0x4373bc() -> ! {
    todo!("0x4373bc __ZN5boost4bindISsRKSsS2_SsSsEENS_3_bi6bind_tIT_PFS5_T0_T1_ENS3_9list_av_2IT2_T3_E4typeEEES9_SB_SC_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::UIEvent const&)>::operator()(RBX::UIEvent const&)")]
// 0x4376b0 — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7UIEventEEEclES5_ — rbx::signals::signal_with_args<1,void ()(RBX::UIEvent const&)>::operator()(RBX::UIEvent const&)
pub fn stub_0x4376b0() -> ! {
    todo!("0x4376b0 __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7UIEventEEEclES5_")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::str(void)const")]
// 0x437c38 — __ZNK5boost12basic_formatIcSt11char_traitsIcESaIcEE3strEv — boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::str(void)const
pub fn stub_0x437c38() -> ! {
    todo!("0x437c38 __ZNK5boost12basic_formatIcSt11char_traitsIcESaIcEE3strEv")
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)1>::rate(void)const")]
// 0x437e68 — __ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE4rateEv — RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)1>::rate(void)const
pub fn stub_0x437e68() -> ! {
    todo!("0x437e68 __ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE4rateEv")
}

#[doc(alias = "RBX::Kernel::numBodies(void)const")]
// 0x437ec8 — __ZNK3RBX6Kernel9numBodiesEv — RBX::Kernel::numBodies(void)const
pub fn stub_0x437ec8() -> ! {
    todo!("0x437ec8 __ZNK3RBX6Kernel9numBodiesEv")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(std::string const&)>::operator()(std::string const&)")]
// 0x4384b4 — __ZN3rbx7signals16signal_with_argsILi1EFvRKSsEEclES3_ — rbx::signals::signal_with_args<1,void ()(std::string const&)>::operator()(std::string const&)
pub fn stub_0x4384b4() -> ! {
    todo!("0x4384b4 __ZN3rbx7signals16signal_with_argsILi1EFvRKSsEEclES3_")
}

#[doc(alias = "RBX::CoreGuiService * RBX::ServiceProvider::find<RBX::CoreGuiService>(void)const")]
// 0x4385f8 — __ZNK3RBX15ServiceProvider4findINS_14CoreGuiServiceEEEPT_v — RBX::CoreGuiService * RBX::ServiceProvider::find<RBX::CoreGuiService>(void)const
pub fn stub_0x4385f8() -> ! {
    todo!("0x4385f8 __ZNK3RBX15ServiceProvider4findINS_14CoreGuiServiceEEEPT_v")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::CoreGuiService>(void)")]
// 0x43b7a0 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14CoreGuiServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::CoreGuiService>(void)
pub fn stub_0x43b7a0() -> ! {
    todo!("0x43b7a0 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14CoreGuiServiceEEEvv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string const&)>::slot> &)")]
// 0x43b7a4 — __ZN3rbx7signals6signalIFvRKSsEE4nextERN5boost13intrusive_ptrINS5_4slotEEE — rbx::signals::signal<void ()(std::string const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string const&)>::slot> &)
pub fn stub_0x43b7a4() -> ! {
    todo!("0x43b7a4 __ZN3rbx7signals6signalIFvRKSsEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string const&)>::on_error(std::exception &)")]
// 0x43b904 — __ZN3rbx7signals6signalIFvRKSsEE8on_errorERSt9exception — rbx::signals::signal<void ()(std::string const&)>::on_error(std::exception &)
pub fn stub_0x43b904() -> ! {
    todo!("0x43b904 __ZN3rbx7signals6signalIFvRKSsEE8on_errorERSt9exception")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string const&)>::slot> const&)")]
// 0x43b92c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKSsEE4slotEEaSERKS9_ — rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string const&)>::slot> const&)
pub fn stub_0x43b92c() -> ! {
    todo!("0x43b92c __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKSsEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string const&)>::safe_static_init_mutex(void)")]
// 0x43b950 — __ZN3rbx7signals6signalIFvRKSsEE22safe_static_init_mutexEv — rbx::signals::signal<void ()(std::string const&)>::safe_static_init_mutex(void)
pub fn stub_0x43b950() -> ! {
    todo!("0x43b950 __ZN3rbx7signals6signalIFvRKSsEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string const&)>::safe_static_do_get_mutex(void)")]
// 0x43b954 — __ZN3rbx7signals6signalIFvRKSsEE24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(std::string const&)>::safe_static_do_get_mutex(void)
pub fn stub_0x43b954() -> ! {
    todo!("0x43b954 __ZN3rbx7signals6signalIFvRKSsEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,float const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,float const&)")]
// 0x43c330 — __ZN5boost2io6detail4feedIcSt11char_traitsIcESaIcERKfEERNS_12basic_formatIT_T0_T1_EESD_T2_ — boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,float const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,float const&)
pub fn stub_0x43c330() -> ! {
    todo!("0x43c330 __ZN5boost2io6detail4feedIcSt11char_traitsIcESaIcERKfEERNS_12basic_formatIT_T0_T1_EESD_T2_")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::clear(void)")]
// 0x43c38c — __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEE5clearEv — boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::clear(void)
pub fn stub_0x43c38c() -> ! {
    todo!("0x43c38c __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEE5clearEv")
}

#[doc(alias = "void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,float const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,float const&)")]
// 0x43c450 — __ZN5boost2io6detail10distributeIcSt11char_traitsIcESaIcERKfEEvRNS_12basic_formatIT_T0_T1_EET2_ — void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,float const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,float const&)
pub fn stub_0x43c450() -> ! {
    todo!("0x43c450 __ZN5boost2io6detail10distributeIcSt11char_traitsIcESaIcERKfEEvRNS_12basic_formatIT_T0_T1_EET2_")
}

#[doc(alias = "void boost::throw_exception<boost::io::too_many_args>(boost::io::too_many_args const&)")]
// 0x43c570 — __ZN5boost15throw_exceptionINS_2io13too_many_argsEEEvRKT_ — void boost::throw_exception<boost::io::too_many_args>(boost::io::too_many_args const&)
pub fn stub_0x43c570() -> ! {
    todo!("0x43c570 __ZN5boost15throw_exceptionINS_2io13too_many_argsEEEvRKT_")
}

#[doc(alias = "boost::io::too_many_args::~too_many_args()")]
// 0x43c660 — __ZN5boost2io13too_many_argsD1Ev — boost::io::too_many_args::~too_many_args()
pub fn stub_0x43c660() -> ! {
    todo!("0x43c660 __ZN5boost2io13too_many_argsD1Ev")
}

#[doc(alias = "void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,float const&>(float const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)")]
// 0x43c664 — __ZN5boost2io6detail3putIcSt11char_traitsIcESaIcERKfEEvT2_RKNS1_11format_itemIT_T0_T1_EERNS_12basic_formatISA_SB_SC_E11string_typeERNSH_20internal_streambuf_tEPSt6locale — void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,float const&>(float const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)
pub fn stub_0x43c664() -> ! {
    todo!("0x43c664 __ZN5boost2io6detail3putIcSt11char_traitsIcESaIcERKfEEvT2_RKNS1_11format_itemIT_T0_T1_EERNS_12basic_formatISA_SB_SC_E11string_typeERNSH_20internal_streambuf_tEPSt6locale")
}

#[doc(alias = "boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::basic_oaltstringstream(boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *)")]
// 0x43cb88 — __ZN5boost2io22basic_oaltstringstreamIcSt11char_traitsIcESaIcEEC1EPNS0_18basic_altstringbufIcS3_S4_EE — boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::basic_oaltstringstream(boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *)
pub fn stub_0x43cb88() -> ! {
    todo!("0x43cb88 __ZN5boost2io22basic_oaltstringstreamIcSt11char_traitsIcESaIcEEC1EPNS0_18basic_altstringbufIcS3_S4_EE")
}

#[doc(alias = "boost::io::detail::stream_format_state<char,std::char_traits<char>>::apply_on(std::basic_ios<char,std::char_traits<char>> &,std::locale *)const")]
// 0x43ccd8 — __ZNK5boost2io6detail19stream_format_stateIcSt11char_traitsIcEE8apply_onERSt9basic_iosIcS4_EPSt6locale — boost::io::detail::stream_format_state<char,std::char_traits<char>>::apply_on(std::basic_ios<char,std::char_traits<char>> &,std::locale *)const
pub fn stub_0x43ccd8() -> ! {
    todo!("0x43ccd8 __ZNK5boost2io6detail19stream_format_stateIcSt11char_traitsIcEE8apply_onERSt9basic_iosIcS4_EPSt6locale")
}

#[doc(alias = "void boost::io::detail::mk_str<char,std::char_traits<char>,std::allocator<char>>(std::basic_string<char,std::char_traits<char>,std::allocator<char>> &,char const*,std::basic_string<char,std::char_traits<char>,std::allocator<char>>::size_type,int,char,std::_Ios_Fmtflags,char,bool)")]
// 0x43cd68 — __ZN5boost2io6detail6mk_strIcSt11char_traitsIcESaIcEEEvRSbIT_T0_T1_EPKS6_NS9_9size_typeEiS6_St13_Ios_FmtflagsS6_b — void boost::io::detail::mk_str<char,std::char_traits<char>,std::allocator<char>>(std::basic_string<char,std::char_traits<char>,std::allocator<char>> &,char const*,std::basic_string<char,std::char_traits<char>,std::allocator<char>>::size_type,int,char,std::_Ios_Fmtflags,char,bool)
pub fn stub_0x43cd68() -> ! {
    todo!("0x43cd68 __ZN5boost2io6detail6mk_strIcSt11char_traitsIcESaIcEEEvRSbIT_T0_T1_EPKS6_NS9_9size_typeEiS6_St13_Ios_FmtflagsS6_b")
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::clear_buffer(void)")]
// 0x43ce40 — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE12clear_bufferEv — boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::clear_buffer(void)
pub fn stub_0x43ce40() -> ! {
    todo!("0x43ce40 __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE12clear_bufferEv")
}

#[doc(alias = "boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_oaltstringstream()")]
// 0x43cf00 — __ZN5boost2io22basic_oaltstringstreamIcSt11char_traitsIcESaIcEED0Ev — boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_oaltstringstream()
pub fn stub_0x43cf00() -> ! {
    todo!("0x43cf00 __ZN5boost2io22basic_oaltstringstreamIcSt11char_traitsIcESaIcEED0Ev")
}

#[doc(alias = "boost::optional_detail::optional_base<std::locale>::is_initialized(void)const")]
// 0x43cfd4 — __ZNK5boost15optional_detail13optional_baseISt6localeE14is_initializedEv — boost::optional_detail::optional_base<std::locale>::is_initialized(void)const
pub fn stub_0x43cfd4() -> ! {
    todo!("0x43cfd4 __ZNK5boost15optional_detail13optional_baseISt6localeE14is_initializedEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>(boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op)")]
// 0x43cfd8 — __ZN5boost6detail12shared_countC2IPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS3_22basic_oaltstringstreamIcS6_S7_E5No_OpEEET_T0_ — boost::detail::shared_count::shared_count<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>(boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op)
pub fn stub_0x43cfd8() -> ! {
    todo!("0x43cfd8 __ZN5boost6detail12shared_countC2IPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS3_22basic_oaltstringstreamIcS6_S7_E5No_OpEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::~sp_counted_impl_pd()")]
// 0x43d0b8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEED1Ev — boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::~sp_counted_impl_pd()
pub fn stub_0x43d0b8() -> ! {
    todo!("0x43d0b8 __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::~sp_counted_impl_pd()")]
// 0x43d0bc — __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEED0Ev — boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::~sp_counted_impl_pd()
pub fn stub_0x43d0bc() -> ! {
    todo!("0x43d0bc __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::dispose(void)")]
// 0x43d0c0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEE7disposeEv — boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::dispose(void)
pub fn stub_0x43d0c0() -> ! {
    todo!("0x43d0c0 __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::get_deleter(std::type_info const&)")]
// 0x43d0c4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::get_deleter(std::type_info const&)
pub fn stub_0x43d0c4() -> ! {
    todo!("0x43d0c4 __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::get_untyped_deleter(void)")]
// 0x43d0dc — __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::get_untyped_deleter(void)
pub fn stub_0x43d0dc() -> ! {
    todo!("0x43d0dc __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::io::too_many_args::~too_many_args()")]
// 0x43d0e0 — __ZN5boost2io13too_many_argsD0Ev — boost::io::too_many_args::~too_many_args()
pub fn stub_0x43d0e0() -> ! {
    todo!("0x43d0e0 __ZN5boost2io13too_many_argsD0Ev")
}

#[doc(alias = "boost::io::too_many_args::what(void)const")]
// 0x43d0f4 — __ZNK5boost2io13too_many_args4whatEv — boost::io::too_many_args::what(void)const
pub fn stub_0x43d0f4() -> ! {
    todo!("0x43d0f4 __ZNK5boost2io13too_many_args4whatEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
// 0x43d100 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED1Ev — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()
pub fn stub_0x43d100() -> ! {
    todo!("0x43d100 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")]
// 0x43d110 — __ZN5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED1Ev — boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()
pub fn stub_0x43d110() -> ! {
    todo!("0x43d110 __ZN5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")]
// 0x43d114 — __ZN5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED2Ev — boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()
pub fn stub_0x43d114() -> ! {
    todo!("0x43d114 __ZN5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED2Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
// 0x43d1d0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED0Ev — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()
pub fn stub_0x43d1d0() -> ! {
    todo!("0x43d1d0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::rethrow(void)const")]
// 0x43d1e8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE7rethrowEv — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::rethrow(void)const
pub fn stub_0x43d1e8() -> ! {
    todo!("0x43d1e8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE7rethrowEv")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
// 0x43d318 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED0Ev — non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()
pub fn stub_0x43d318() -> ! {
    todo!("0x43d318 __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED0Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone(void)const")]
// 0x43d330 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE5cloneEv — virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone(void)const
pub fn stub_0x43d330() -> ! {
    todo!("0x43d330 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE5cloneEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::rethrow(void)const")]
// 0x43d33c — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE7rethrowEv — virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::rethrow(void)const
pub fn stub_0x43d33c() -> ! {
    todo!("0x43d33c __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE7rethrowEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
// 0x43d34c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED0Ev — virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()
pub fn stub_0x43d34c() -> ! {
    todo!("0x43d34c __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")]
// 0x43d368 — __ZN5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED0Ev — boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()
pub fn stub_0x43d368() -> ! {
    todo!("0x43d368 __ZN5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED0Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")]
// 0x43d37c — __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED0Ev — non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()
pub fn stub_0x43d37c() -> ! {
    todo!("0x43d37c __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::too_many_args> const&)")]
// 0x43d398 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEEC1ERKS5_ — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::too_many_args> const&)
pub fn stub_0x43d398() -> ! {
    todo!("0x43d398 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEEC1ERKS5_")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::size(void)const")]
// 0x43d4d8 — __ZNK5boost12basic_formatIcSt11char_traitsIcESaIcEE4sizeEv — boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::size(void)const
pub fn stub_0x43d4d8() -> ! {
    todo!("0x43d4d8 __ZNK5boost12basic_formatIcSt11char_traitsIcESaIcEE4sizeEv")
}

#[doc(alias = "boost::io::too_few_args::~too_few_args()")]
// 0x43d528 — __ZN5boost2io12too_few_argsD0Ev — boost::io::too_few_args::~too_few_args()
pub fn stub_0x43d528() -> ! {
    todo!("0x43d528 __ZN5boost2io12too_few_argsD0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
// 0x43d540 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED1Ev — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()
pub fn stub_0x43d540() -> ! {
    todo!("0x43d540 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")]
// 0x43d550 — __ZN5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED1Ev — boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()
pub fn stub_0x43d550() -> ! {
    todo!("0x43d550 __ZN5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")]
// 0x43d554 — __ZN5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED2Ev — boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()
pub fn stub_0x43d554() -> ! {
    todo!("0x43d554 __ZN5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED2Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")]
// 0x43d60c — __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED1Ev — non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()
pub fn stub_0x43d60c() -> ! {
    todo!("0x43d60c __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED1Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
// 0x43d614 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED1Ev — non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()
pub fn stub_0x43d614() -> ! {
    todo!("0x43d614 __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED1Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
// 0x43d61c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED1Ev — virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()
pub fn stub_0x43d61c() -> ! {
    todo!("0x43d61c __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
// 0x43d628 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED0Ev — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()
pub fn stub_0x43d628() -> ! {
    todo!("0x43d628 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone(void)const")]
// 0x43d63c — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE5cloneEv — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone(void)const
pub fn stub_0x43d63c() -> ! {
    todo!("0x43d63c __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::rethrow(void)const")]
// 0x43d6f8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE7rethrowEv — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::rethrow(void)const
pub fn stub_0x43d6f8() -> ! {
    todo!("0x43d6f8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE7rethrowEv")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
// 0x43d828 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED0Ev — non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()
pub fn stub_0x43d828() -> ! {
    todo!("0x43d828 __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED0Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone(void)const")]
// 0x43d840 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE5cloneEv — virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone(void)const
pub fn stub_0x43d840() -> ! {
    todo!("0x43d840 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")]
// 0x43d850 — __ZN5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED0Ev — boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()
pub fn stub_0x43d850() -> ! {
    todo!("0x43d850 __ZN5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_tag)")]
// 0x43d864 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEEC1ERKS6_NS6_9clone_tagE — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_tag)
pub fn stub_0x43d864() -> ! {
    todo!("0x43d864 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::too_few_args> const&)")]
// 0x43d9a0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEEC1ERKS5_ — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::too_few_args> const&)
pub fn stub_0x43d9a0() -> ! {
    todo!("0x43d9a0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEEC1ERKS5_")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,int const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,int const&)")]
// 0x43dadc — __ZN5boost2io6detail4feedIcSt11char_traitsIcESaIcERKiEERNS_12basic_formatIT_T0_T1_EESD_T2_ — boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,int const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,int const&)
pub fn stub_0x43dadc() -> ! {
    todo!("0x43dadc __ZN5boost2io6detail4feedIcSt11char_traitsIcESaIcERKiEERNS_12basic_formatIT_T0_T1_EESD_T2_")
}

#[doc(alias = "void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,int const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,int const&)")]
// 0x43db38 — __ZN5boost2io6detail10distributeIcSt11char_traitsIcESaIcERKiEEvRNS_12basic_formatIT_T0_T1_EET2_ — void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,int const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,int const&)
pub fn stub_0x43db38() -> ! {
    todo!("0x43db38 __ZN5boost2io6detail10distributeIcSt11char_traitsIcESaIcERKiEEvRNS_12basic_formatIT_T0_T1_EET2_")
}

#[doc(alias = "void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,int const&>(int const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)")]
// 0x43dc58 — __ZN5boost2io6detail3putIcSt11char_traitsIcESaIcERKiEEvT2_RKNS1_11format_itemIT_T0_T1_EERNS_12basic_formatISA_SB_SC_E11string_typeERNSH_20internal_streambuf_tEPSt6locale — void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,int const&>(int const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)
pub fn stub_0x43dc58() -> ! {
    todo!("0x43dc58 __ZN5boost2io6detail3putIcSt11char_traitsIcESaIcERKiEEvT2_RKNS1_11format_itemIT_T0_T1_EERNS_12basic_formatISA_SB_SC_E11string_typeERNSH_20internal_streambuf_tEPSt6locale")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,double const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,double const&)")]
// 0x43e15c — __ZN5boost2io6detail4feedIcSt11char_traitsIcESaIcERKdEERNS_12basic_formatIT_T0_T1_EESD_T2_ — boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,double const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,double const&)
pub fn stub_0x43e15c() -> ! {
    todo!("0x43e15c __ZN5boost2io6detail4feedIcSt11char_traitsIcESaIcERKdEERNS_12basic_formatIT_T0_T1_EESD_T2_")
}

#[doc(alias = "void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,double const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,double const&)")]
// 0x43e1b8 — __ZN5boost2io6detail10distributeIcSt11char_traitsIcESaIcERKdEEvRNS_12basic_formatIT_T0_T1_EET2_ — void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,double const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,double const&)
pub fn stub_0x43e1b8() -> ! {
    todo!("0x43e1b8 __ZN5boost2io6detail10distributeIcSt11char_traitsIcESaIcERKdEEvRNS_12basic_formatIT_T0_T1_EET2_")
}

#[doc(alias = "void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,double const&>(double const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)")]
// 0x43e2d8 — __ZN5boost2io6detail3putIcSt11char_traitsIcESaIcERKdEEvT2_RKNS1_11format_itemIT_T0_T1_EERNS_12basic_formatISA_SB_SC_E11string_typeERNSH_20internal_streambuf_tEPSt6locale — void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,double const&>(double const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)
pub fn stub_0x43e2d8() -> ! {
    todo!("0x43e2d8 __ZN5boost2io6detail3putIcSt11char_traitsIcESaIcERKdEEvT2_RKNS1_11format_itemIT_T0_T1_EERNS_12basic_formatISA_SB_SC_E11string_typeERNSH_20internal_streambuf_tEPSt6locale")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::basic_format(char const*)")]
// 0x43e7f0 — __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEEC2EPKc — boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::basic_format(char const*)
pub fn stub_0x43e7f0() -> ! {
    todo!("0x43e7f0 __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEEC2EPKc")
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::~basic_altstringbuf()")]
// 0x43ea00 — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEED1Ev — boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::~basic_altstringbuf()
pub fn stub_0x43ea00() -> ! {
    todo!("0x43ea00 __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEED1Ev")
}

#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::~vector()")]
// 0x43ea3c — __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EED2Ev — std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::~vector()
pub fn stub_0x43ea3c() -> ! {
    todo!("0x43ea3c __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EED2Ev")
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::dealloc(void)")]
// 0x43ea8c — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE7deallocEv — boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::dealloc(void)
pub fn stub_0x43ea8c() -> ! {
    todo!("0x43ea8c __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE7deallocEv")
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::~basic_altstringbuf()")]
// 0x43eabc — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEED0Ev — boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::~basic_altstringbuf()
pub fn stub_0x43eabc() -> ! {
    todo!("0x43eabc __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEED0Ev")
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::underflow(void)")]
// 0x43eb00 — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE9underflowEv — boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::underflow(void)
pub fn stub_0x43eb00() -> ! {
    todo!("0x43eb00 __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE9underflowEv")
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::pbackfail(int)")]
// 0x43eb48 — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE9pbackfailEi — boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::pbackfail(int)
pub fn stub_0x43eb48() -> ! {
    todo!("0x43eb48 __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE9pbackfailEi")
}

#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::overflow(int)")]
// 0x43eb98 — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE8overflowEi — boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::overflow(int)
pub fn stub_0x43eb98() -> ! {
    todo!("0x43eb98 __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE8overflowEi")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::getloc(void)const")]
// 0x43ecd4 — __ZNK5boost12basic_formatIcSt11char_traitsIcESaIcEE6getlocEv — boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::getloc(void)const
pub fn stub_0x43ecd4() -> ! {
    todo!("0x43ecd4 __ZNK5boost12basic_formatIcSt11char_traitsIcESaIcEE6getlocEv")
}

#[doc(alias = "std::ctype<char>::widen(char)const")]
// 0x43ecfc — __ZNKSt5ctypeIcE5widenEc — std::ctype<char>::widen(char)const
pub fn stub_0x43ecfc() -> ! {
    todo!("0x43ecfc __ZNKSt5ctypeIcE5widenEc")
}

