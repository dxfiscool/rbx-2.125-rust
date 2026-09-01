//! core shard GD — 100 core stubs EA-sorted, 0xf482f4..0xf49a44 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf482e4).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf482e4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xf482f4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_f482f4() -> ! {
    todo!("0xf482f4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0xf48304 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_f48304() -> ! {
    todo!("0xf48304 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xf48314 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_f48314() -> ! {
    todo!("0xf48314 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "void RBX::RunningAverage<double,double>::iter<RBX::Stats::JobStepWindowWriter>(RBX::Stats::JobStepWindowWriter &)const")]
// 0xf48354 — j___ZNK3RBX14RunningAverageIddE4iterINS_5Stats19JobStepWindowWriterEEEvRT_
pub fn stub_f48354() -> ! {
    todo!("0xf48354 j___ZNK3RBX14RunningAverageIddE4iterINS_5Stats19JobStepWindowWriterEEEvRT_")
}

#[doc(alias = "RBX::TotalCountTimeInterval<int,(RBX::Time::SampleMethod)1>::getCount(void)const")]
// 0xf48364 — j___ZNK3RBX22TotalCountTimeIntervalIiLNS_4Time12SampleMethodE1EE8getCountEv
pub fn stub_f48364() -> ! {
    todo!("0xf48364 j___ZNK3RBX22TotalCountTimeIntervalIiLNS_4Time12SampleMethodE1EE8getCountEv")
}

#[doc(alias = "boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>::operator()(RBX::Stats::StatsService*,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &)const")]
// 0xf483e4 — j___ZNK5boost4_mfi3mf3IvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS2_13TaskScheduler3JobEEENS5_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEclEPS4_S9_SF_SG_
// was: boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>::operator()(RBX::Stats::StatsService*,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &)const
pub fn stub_f483e4() -> ! {
    todo!("0xf483e4 j___ZNK5boost4_mfi3mf3IvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS2_13TaskScheduler3JobEEENS5_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEclEPS4_S9_SF_SG_")
}

#[doc(alias = "boost::function0<float>::operator()(void)const")]
// 0xf48444 — j___ZNK5boost9function0IfEclEv
pub fn stub_f48444() -> ! {
    todo!("0xf48444 j___ZNK5boost9function0IfEclEv")
}

#[doc(alias = "boost::function0<unsigned long>::operator()(void)const")]
// 0xf48454 — j___ZNK5boost9function0ImEclEv
pub fn stub_f48454() -> ! {
    todo!("0xf48454 j___ZNK5boost9function0ImEclEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xf48464 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSD_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS8_EEEEmRKT_RKT0_
pub fn stub_f48464() -> ! {
    todo!("0xf48464 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSD_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS8_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// 0xf48474 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
pub fn stub_f48474() -> ! {
    todo!("0xf48474 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0xf48484 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
pub fn stub_f48484() -> ! {
    todo!("0xf48484 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")
}

#[doc(alias = "std::map<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::operator[](std::string const&)")]
// 0xf48494 — j___ZNSt3mapISsSsSt4lessISsESaISt4pairIKSsSsEEEixERS3_
pub fn stub_f48494() -> ! {
    todo!("0xf48494 j___ZNSt3mapISsSsSt4lessISsESaISt4pairIKSsSsEEEixERS3_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>)")]
// 0xf484d4 — j___ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_5Stats12StatsServiceES8_NS3_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEENSE_5list4INSE_5valueIPSJ_EENS2_3argILi1EEENST_ISP_EENS2_17reference_wrapperIbEEEEEEET0_T_S14_S13_
// was: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>)
pub fn stub_f484d4() -> ! {
    todo!("0xf484d4 j___ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_5Stats12StatsServiceES8_NS3_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEENSE_5list4INSE_5valueIPSJ_EENS2_3argILi1EEENST_ISP_EENS2_17reference_wrapperIbEEEEEEET0_T_S14_S13_")
}

#[doc(alias = "RBX::SurfaceType * rbx::any_cast<RBX::SurfaceType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf48654 — j___ZN3rbx8any_castIN3RBX11SurfaceTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f48654() -> ! {
    todo!("0xf48654 j___ZN3rbx8any_castIN3RBX11SurfaceTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::LegacyController::InputType * rbx::any_cast<RBX::LegacyController::InputType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf48664 — j___ZN3rbx8any_castIN3RBX16LegacyController9InputTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f48664() -> ! {
    todo!("0xf48664 j___ZN3rbx8any_castIN3RBX16LegacyController9InputTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::SurfaceSelection::~SurfaceSelection()")]
// 0xf486e4 — j___ZN3RBX16SurfaceSelectionD1Ev
pub fn stub_f486e4() -> ! {
    todo!("0xf486e4 j___ZN3RBX16SurfaceSelectionD1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)")]
// 0xf48824 — j___ZN3RBX11shared_fromINS_4TeamEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Team> RBX::shared_from<RBX::Team>(RBX::Team*)
pub fn stub_f48824() -> ! {
    todo!("0xf48824 j___ZN3RBX11shared_fromINS_4TeamEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextBox> RBX::shared_from<RBX::TextBox>(RBX::TextBox*)")]
// 0xf489b4 — j___ZN3RBX11shared_fromINS_7TextBoxEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::TextBox> RBX::shared_from<RBX::TextBox>(RBX::TextBox*)
pub fn stub_f489b4() -> ! {
    todo!("0xf489b4 j___ZN3RBX11shared_fromINS_7TextBoxEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TextService>(void)")]
// 0xf489f4 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_11TextServiceEEEmv
pub fn stub_f489f4() -> ! {
    todo!("0xf489f4 j___ZN3RBX15ServiceProvider15doGetClassIndexINS_11TextServiceEEEmv")
}

#[doc(alias = "RBX::TextBox::~TextBox()")]
// 0xf48a94 — j___ZN3RBX7TextBoxD2Ev
pub fn stub_f48a94() -> ! {
    todo!("0xf48a94 j___ZN3RBX7TextBoxD2Ev")
}

#[doc(alias = "RBX::GuiObject::convertFontSize(RBX::TextService::FontSize)")]
// 0xf48ac4 — j___ZN3RBX9GuiObject15convertFontSizeENS_11TextService8FontSizeE
pub fn stub_f48ac4() -> ! {
    todo!("0xf48ac4 j___ZN3RBX9GuiObject15convertFontSizeENS_11TextService8FontSizeE")
}

#[doc(alias = "RBX::GuiObject::~GuiObject()")]
// 0xf48ad4 — j___ZN3RBX9GuiObjectD2Ev
pub fn stub_f48ad4() -> ! {
    todo!("0xf48ad4 j___ZN3RBX9GuiObjectD2Ev")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::XAlignment>(RBX::TextService::XAlignment const&)")]
// 0xf48ae4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10XAlignmentEEERS3_RKT_
pub fn stub_f48ae4() -> ! {
    todo!("0xf48ae4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10XAlignmentEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::YAlignment>(RBX::TextService::YAlignment const&)")]
// 0xf48af4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10YAlignmentEEERS3_RKT_
pub fn stub_f48af4() -> ! {
    todo!("0xf48af4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10YAlignmentEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::Font>(RBX::TextService::Font const&)")]
// 0xf48b04 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_
pub fn stub_f48b04() -> ! {
    todo!("0xf48b04 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::FontSize>(RBX::TextService::FontSize const&)")]
// 0xf48b14 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService8FontSizeEEERS3_RKT_
pub fn stub_f48b14() -> ! {
    todo!("0xf48b14 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService8FontSizeEEERS3_RKT_")
}

#[doc(alias = "rbx::remote_signal<void ()(RBX::UDim2)>::~remote_signal()")]
// 0xf48b24 — j___ZN3rbx13remote_signalIFvN3RBX5UDim2EEED2Ev
pub fn stub_f48b24() -> ! {
    todo!("0xf48b24 j___ZN3rbx13remote_signalIFvN3RBX5UDim2EEED2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(int,int)>::~remote_signal()")]
// 0xf48b34 — j___ZN3rbx13remote_signalIFviiEED2Ev
pub fn stub_f48b34() -> ! {
    todo!("0xf48b34 j___ZN3rbx13remote_signalIFviiEED2Ev")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::XAlignment>::singleton(void)")]
// 0xf48b44 — j___ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE9singletonEv
pub fn stub_f48b44() -> ! {
    todo!("0xf48b44 j___ZN3rbx14implementation12typed_holderIN3RBX11TextService10XAlignmentEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::YAlignment>::singleton(void)")]
// 0xf48b54 — j___ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE9singletonEv
pub fn stub_f48b54() -> ! {
    todo!("0xf48b54 j___ZN3rbx14implementation12typed_holderIN3RBX11TextService10YAlignmentEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::Font>::singleton(void)")]
// 0xf48b64 — j___ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE9singletonEv
pub fn stub_f48b64() -> ! {
    todo!("0xf48b64 j___ZN3rbx14implementation12typed_holderIN3RBX11TextService4FontEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TextService::FontSize>::singleton(void)")]
// 0xf48b74 — j___ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE9singletonEv
pub fn stub_f48b74() -> ! {
    todo!("0xf48b74 j___ZN3rbx14implementation12typed_holderIN3RBX11TextService8FontSizeEE9singletonEv")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(rbx_core::SharedPtr<RBX::TextBox>)>::operator()(rbx_core::SharedPtr<RBX::TextBox>)")]
// 0xf48b84 — j___ZN3rbx7signals16signal_with_argsILi1EFvN5boost10shared_ptrIN3RBX7TextBoxEEEEEclES6_
// was: rbx::signals::signal_with_args<1,void ()(boost::shared_ptr<RBX::TextBox>)>::operator()(boost::shared_ptr<RBX::TextBox>)
pub fn stub_f48b84() -> ! {
    todo!("0xf48b84 j___ZN3rbx7signals16signal_with_argsILi1EFvN5boost10shared_ptrIN3RBX7TextBoxEEEEEclES6_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::disconnectAll(void)")]
// 0xf48b94 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13disconnectAllEv
pub fn stub_f48b94() -> ! {
    todo!("0xf48b94 j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::safe_static_do_get_mutex(void)")]
// 0xf48ba4 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE24safe_static_do_get_mutexEv
pub fn stub_f48ba4() -> ! {
    todo!("0xf48ba4 j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot> &)")]
// 0xf48bb4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot> &)
pub fn stub_f48bb4() -> ! {
    todo!("0xf48bb4 j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::on_error(std::exception &)")]
// 0xf48bc4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE8on_errorERSt9exception
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::on_error(std::exception &)
pub fn stub_f48bc4() -> ! {
    todo!("0xf48bc4 j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::slot::safe_static_do_get_mutex(void)")]
// 0xf48bd4 — j___ZN3rbx7signals6signalIFvPKcbEE4slot24safe_static_do_get_mutexEv
pub fn stub_f48bd4() -> ! {
    todo!("0xf48bd4 j___ZN3rbx7signals6signalIFvPKcbEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::insert(rbx::signals::signal<void ()(char const*,bool)>::slot *)")]
// 0xf48be4 — j___ZN3rbx7signals6signalIFvPKcbEE6insertEPNS5_4slotE
pub fn stub_f48be4() -> ! {
    todo!("0xf48be4 j___ZN3rbx7signals6signalIFvPKcbEE6insertEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::remove(rbx::signals::signal<void ()(char const*,bool)>::slot *)")]
// 0xf48bf4 — j___ZN3rbx7signals6signalIFvPKcbEE6removeEPNS5_4slotE
pub fn stub_f48bf4() -> ! {
    todo!("0xf48bf4 j___ZN3rbx7signals6signalIFvPKcbEE6removeEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(char const*,bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0xf48c04 — j___ZN3rbx7signals6signalIFvPKcbEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_f48c04() -> ! {
    todo!("0xf48c04 j___ZN3rbx7signals6signalIFvPKcbEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::disconnectAll(void)")]
// 0xf48c14 — j___ZN3rbx7signals6signalIFviiEE13disconnectAllEv
pub fn stub_f48c14() -> ! {
    todo!("0xf48c14 j___ZN3rbx7signals6signalIFviiEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::safe_static_do_get_mutex(void)")]
// 0xf48c24 — j___ZN3rbx7signals6signalIFviiEE24safe_static_do_get_mutexEv
pub fn stub_f48c24() -> ! {
    todo!("0xf48c24 j___ZN3rbx7signals6signalIFviiEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "RBX::TextService::XAlignment const& rbx::any_cast<RBX::TextService::XAlignment const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf48c34 — j___ZN3rbx8any_castIRKN3RBX11TextService10XAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f48c34() -> ! {
    todo!("0xf48c34 j___ZN3rbx8any_castIRKN3RBX11TextService10XAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::TextService::YAlignment const& rbx::any_cast<RBX::TextService::YAlignment const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf48c44 — j___ZN3rbx8any_castIRKN3RBX11TextService10YAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f48c44() -> ! {
    todo!("0xf48c44 j___ZN3rbx8any_castIRKN3RBX11TextService10YAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::TextService::Font const& rbx::any_cast<RBX::TextService::Font const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf48c54 — j___ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f48c54() -> ! {
    todo!("0xf48c54 j___ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::TextService::FontSize const& rbx::any_cast<RBX::TextService::FontSize const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf48c64 — j___ZN3rbx8any_castIRKN3RBX11TextService8FontSizeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f48c64() -> ! {
    todo!("0xf48c64 j___ZN3rbx8any_castIRKN3RBX11TextService8FontSizeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "boost::scoped_ptr<RBX::GuiObject::Tween>::~scoped_ptr()")]
// 0xf48c74 — j___ZN5boost10scoped_ptrIN3RBX9GuiObject5TweenEED2Ev
pub fn stub_f48c74() -> ! {
    todo!("0xf48c74 j___ZN5boost10scoped_ptrIN3RBX9GuiObject5TweenEED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> const&)")]
// 0xf48ca4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSERKS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> const&)
pub fn stub_f48ca4() -> ! {
    todo!("0xf48ca4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(rbx::signals::signal<void ()(char const*,bool)>::slot*)")]
// 0xf48cb4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKcbEE4slotEEaSEPS8_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(rbx::signals::signal<void ()(char const*,bool)>::slot*)
pub fn stub_f48cb4() -> ! {
    todo!("0xf48cb4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKcbEE4slotEEaSEPS8_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot> const&)")]
// 0xf48cc4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot> const&)
pub fn stub_f48cc4() -> ! {
    todo!("0xf48cc4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiEE4slotEEaSERKS7_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::TextBox *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list2<char const*&,bool &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool> &,boost::_bi::list2<char const*&,bool &> &,int)")]
// 0xf48cd4 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX7TextBoxEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_PKcbEENS0_5list2IRSF_RbEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f48cd4() -> ! {
    todo!("0xf48cd4 j___ZN5boost3_bi5list3INS0_5valueIPN3RBX7TextBoxEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_PKcbEENS0_5list2IRSF_RbEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::clear(void)")]
// 0xf48d04 — j___ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5clearEv
pub fn stub_f48d04() -> ! {
    todo!("0xf48d04 j___ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5clearEv")
}

#[doc(alias = "RBX::TextService * RBX::ServiceProvider::find<RBX::TextService>(void)const")]
// 0xf48e64 — j___ZNK3RBX15ServiceProvider4findINS_11TextServiceEEEPT_v
pub fn stub_f48e64() -> ! {
    todo!("0xf48e64 j___ZNK3RBX15ServiceProvider4findINS_11TextServiceEEEPT_v")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::XAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>> *)")]
// 0xf48e94 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f48e94() -> ! {
    todo!("0xf48e94 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::YAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>> *)")]
// 0xf48ea4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f48ea4() -> ! {
    todo!("0xf48ea4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::Font>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::Font>> *)")]
// 0xf48eb4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f48eb4() -> ! {
    todo!("0xf48eb4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::FontSize>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::FontSize>> *)")]
// 0xf48ec4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f48ec4() -> ! {
    todo!("0xf48ec4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "boost::function0<void>::operator=(boost::function0<void> const&)")]
// 0xf49194 — j___ZN5boost9function0IvEaSERKS1_
pub fn stub_f49194() -> ! {
    todo!("0xf49194 j___ZN5boost9function0IvEaSERKS1_")
}

#[doc(alias = "std::_List_base<RBX::TimerService::Item,std::allocator<RBX::TimerService::Item>>::_M_clear(void)")]
// 0xf491a4 — j___ZNSt10_List_baseIN3RBX12TimerService4ItemESaIS2_EE8_M_clearEv
pub fn stub_f491a4() -> ! {
    todo!("0xf491a4 j___ZNSt10_List_baseIN3RBX12TimerService4ItemESaIS2_EE8_M_clearEv")
}

#[doc(alias = "std::list<RBX::TimerService::Item,std::allocator<RBX::TimerService::Item>>::_M_create_node(RBX::TimerService::Item const&)")]
// 0xf491b4 — j___ZNSt4listIN3RBX12TimerService4ItemESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_f491b4() -> ! {
    todo!("0xf491b4 j___ZNSt4listIN3RBX12TimerService4ItemESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "RBX::BackpackItem::BackpackItem(void)")]
// 0xf49234 — j___ZN3RBX12BackpackItemC2Ev
pub fn stub_f49234() -> ! {
    todo!("0xf49234 j___ZN3RBX12BackpackItemC2Ev")
}

#[doc(alias = "RBX::Tool::special_equipped_signal::~special_equipped_signal()")]
// 0xf492a4 — j___ZN3RBX4Tool23special_equipped_signalD2Ev
pub fn stub_f492a4() -> ! {
    todo!("0xf492a4 j___ZN3RBX4Tool23special_equipped_signalD2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ToolMouseCommand>::shared_ptr<RBX::ToolMouseCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf49304 — j___ZN5boost10shared_ptrIN3RBX16ToolMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ToolMouseCommand>::shared_ptr<RBX::ToolMouseCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_f49304() -> ! {
    todo!("0xf49304 j___ZN5boost10shared_ptrIN3RBX16ToolMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Mouse>::operator=(rbx_core::SharedPtr<RBX::Mouse> const&)")]
// 0xf49334 — j___ZN5boost10shared_ptrIN3RBX5MouseEEaSERKS3_
// was: boost::shared_ptr<RBX::Mouse>::operator=(boost::shared_ptr<RBX::Mouse> const&)
pub fn stub_f49334() -> ! {
    todo!("0xf49334 j___ZN5boost10shared_ptrIN3RBX5MouseEEaSERKS3_")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>)")]
// 0xf49354 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::list2(boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>)
pub fn stub_f49354() -> ! {
    todo!("0xf49354 j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>)")]
// 0xf493a4 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>)
pub fn stub_f493a4() -> ! {
    todo!("0xf493a4 j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf493d4 — j___ZN5boost6detail12shared_countC2IPN3RBX16ToolMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_f493d4() -> ! {
    todo!("0xf493d4 j___ZN5boost6detail12shared_countC2IPN3RBX16ToolMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::ToolMouseCommand,RBX::ToolMouseCommand>(rbx_core::SharedPtr<RBX::ToolMouseCommand> const*,RBX::ToolMouseCommand *)const")]
// 0xf494c4 — j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16ToolMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::ToolMouseCommand,RBX::ToolMouseCommand>(boost::shared_ptr<RBX::ToolMouseCommand> const*,RBX::ToolMouseCommand *)const
pub fn stub_f494c4() -> ! {
    todo!("0xf494c4 j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16ToolMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "RBX::ToolMouseCommand::~ToolMouseCommand()")]
// 0xf49554 — j___ZN3RBX16ToolMouseCommandD0Ev
pub fn stub_f49554() -> ! {
    todo!("0xf49554 j___ZN3RBX16ToolMouseCommandD0Ev")
}

#[doc(alias = "RBX::ToolMouseCommand::~ToolMouseCommand()")]
// 0xf49564 — j___ZN3RBX16ToolMouseCommandD2Ev
pub fn stub_f49564() -> ! {
    todo!("0xf49564 j___ZN3RBX16ToolMouseCommandD2Ev")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>> const&)")]
// 0xf49584 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_
pub fn stub_f49584() -> ! {
    todo!("0xf49584 j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ToolMouseCommand>,boost::_bi::list1<boost::_bi::value<RBX::ToolMouseCommand*>>>::operator()(void)")]
// 0xf49594 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
pub fn stub_f49594() -> ! {
    todo!("0xf49594 j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DecalTool> RBX::shared_from<RBX::DecalTool>(RBX::DecalTool*)")]
// 0xf495c4 — j___ZN3RBX11shared_fromINS_9DecalToolEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::DecalTool> RBX::shared_from<RBX::DecalTool>(RBX::DecalTool*)
pub fn stub_f495c4() -> ! {
    todo!("0xf495c4 j___ZN3RBX11shared_fromINS_9DecalToolEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "boost::scoped_ptr<RBX::TouchDebouncer>::~scoped_ptr()")]
// 0xf49624 — j___ZN5boost10scoped_ptrIN3RBX14TouchDebouncerEED2Ev
pub fn stub_f49624() -> ! {
    todo!("0xf49624 j___ZN5boost10scoped_ptrIN3RBX14TouchDebouncerEED2Ev")
}

#[doc(alias = "std::_Vector_base<RBX::TouchDebouncer::Item,std::allocator<RBX::TouchDebouncer::Item>>::_M_allocate(unsigned long)")]
// 0xf49644 — j___ZNSt12_Vector_baseIN3RBX14TouchDebouncer4ItemESaIS2_EE11_M_allocateEm
pub fn stub_f49644() -> ! {
    todo!("0xf49644 j___ZNSt12_Vector_baseIN3RBX14TouchDebouncer4ItemESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::TouchDebouncer::Item * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TouchDebouncer::Item *,RBX::TouchDebouncer::Item *>(RBX::TouchDebouncer::Item *,RBX::TouchDebouncer::Item *,RBX::TouchDebouncer::Item *)")]
// 0xf49654 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14TouchDebouncer4ItemES6_EET0_T_S8_S7_
pub fn stub_f49654() -> ! {
    todo!("0xf49654 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14TouchDebouncer4ItemES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::TouchDebouncer::Item,std::allocator<RBX::TouchDebouncer::Item>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TouchDebouncer::Item*,std::vector<RBX::TouchDebouncer::Item,std::allocator<RBX::TouchDebouncer::Item>>>,RBX::TouchDebouncer::Item const&)")]
// 0xf49664 — j___ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f49664() -> ! {
    todo!("0xf49664 j___ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::TouchDebouncer::Item,std::allocator<RBX::TouchDebouncer::Item>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TouchDebouncer::Item*,std::vector<RBX::TouchDebouncer::Item,std::allocator<RBX::TouchDebouncer::Item>>>,unsigned long,RBX::TouchDebouncer::Item const&)")]
// 0xf49674 — j___ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f49674() -> ! {
    todo!("0xf49674 j___ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::TouchDebouncer::Item,std::allocator<RBX::TouchDebouncer::Item>>::_M_erase_at_end(RBX::TouchDebouncer::Item*)")]
// 0xf49684 — j___ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE15_M_erase_at_endEPS2_
pub fn stub_f49684() -> ! {
    todo!("0xf49684 j___ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE15_M_erase_at_endEPS2_")
}

#[doc(alias = "std::vector<RBX::TouchDebouncer::Item,std::allocator<RBX::TouchDebouncer::Item>>::resize(unsigned long,RBX::TouchDebouncer::Item)")]
// 0xf49694 — j___ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE6resizeEmS2_
pub fn stub_f49694() -> ! {
    todo!("0xf49694 j___ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::TouchDebouncer::Item,std::allocator<RBX::TouchDebouncer::Item>>::push_back(RBX::TouchDebouncer::Item const&)")]
// 0xf496a4 — j___ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE9push_backERKS2_
pub fn stub_f496a4() -> ! {
    todo!("0xf496a4 j___ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::TouchDebouncer::Item,std::allocator<RBX::TouchDebouncer::Item>>::~vector()")]
// 0xf496b4 — j___ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EED2Ev
pub fn stub_f496b4() -> ! {
    todo!("0xf496b4 j___ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EED2Ev")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::TouchDebouncer::Item *,unsigned long,RBX::TouchDebouncer::Item>(RBX::TouchDebouncer::Item *,unsigned long,RBX::TouchDebouncer::Item const&,std::__false_type)")]
// 0xf496c4 — j___ZSt26__uninitialized_fill_n_auxIPN3RBX14TouchDebouncer4ItemEmS2_EvT_T0_RKT1_St12__false_type
pub fn stub_f496c4() -> ! {
    todo!("0xf496c4 j___ZSt26__uninitialized_fill_n_auxIPN3RBX14TouchDebouncer4ItemEmS2_EvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Controller> RBX::shared_from<RBX::Controller>(RBX::Controller*)")]
// 0xf49824 — j___ZN3RBX11shared_fromINS_10ControllerEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Controller> RBX::shared_from<RBX::Controller>(RBX::Controller*)
pub fn stub_f49824() -> ! {
    todo!("0xf49824 j___ZN3RBX11shared_fromINS_10ControllerEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::VehicleSeat> RBX::shared_from<RBX::VehicleSeat>(RBX::VehicleSeat*)")]
// 0xf49834 — j___ZN3RBX11shared_fromINS_11VehicleSeatEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::VehicleSeat> RBX::shared_from<RBX::VehicleSeat>(RBX::VehicleSeat*)
pub fn stub_f49834() -> ! {
    todo!("0xf49834 j___ZN3RBX11shared_fromINS_11VehicleSeatEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::GuiDrawImage::GuiDrawImage(void)")]
// 0xf49844 — j___ZN3RBX12GuiDrawImageC2Ev
pub fn stub_f49844() -> ! {
    todo!("0xf49844 j___ZN3RBX12GuiDrawImageC2Ev")
}

#[doc(alias = "RBX::ContentId::ContentId(void)")]
// 0xf49934 — j___ZN3RBX9ContentIdC2Ev
pub fn stub_f49934() -> ! {
    todo!("0xf49934 j___ZN3RBX9ContentIdC2Ev")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Controller::Button>(RBX::Controller::Button const&)")]
// 0xf49994 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Controller6ButtonEEERS3_RKT_
pub fn stub_f49994() -> ! {
    todo!("0xf49994 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Controller6ButtonEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Controller::Button>::singleton(void)")]
// 0xf499a4 — j___ZN3rbx14implementation12typed_holderIN3RBX10Controller6ButtonEE9singletonEv
pub fn stub_f499a4() -> ! {
    todo!("0xf499a4 j___ZN3rbx14implementation12typed_holderIN3RBX10Controller6ButtonEE9singletonEv")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Controller::Button)>::operator()(RBX::Controller::Button)")]
// 0xf499b4 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10Controller6ButtonEEEclES4_
pub fn stub_f499b4() -> ! {
    todo!("0xf499b4 j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10Controller6ButtonEEEclES4_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::disconnectAll(void)")]
// 0xf499c4 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE13disconnectAllEv
pub fn stub_f499c4() -> ! {
    todo!("0xf499c4 j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::safe_static_do_get_mutex(void)")]
// 0xf499d4 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE24safe_static_do_get_mutexEv
pub fn stub_f499d4() -> ! {
    todo!("0xf499d4 j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> &)")]
// 0xf499e4 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(RBX::Controller::Button)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Controller::Button)>::slot> &)
pub fn stub_f499e4() -> ! {
    todo!("0xf499e4 j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::slot::safe_static_do_get_mutex(void)")]
// 0xf499f4 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f499f4() -> ! {
    todo!("0xf499f4 j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::insert(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot *)")]
// 0xf49a04 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE6insertEPNS6_4slotE
pub fn stub_f49a04() -> ! {
    todo!("0xf49a04 j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::remove(rbx::signals::signal<void ()(RBX::Controller::Button)>::slot *)")]
// 0xf49a14 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE6removeEPNS6_4slotE
pub fn stub_f49a14() -> ! {
    todo!("0xf49a14 j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Controller::Button)>::connect<boost::function<void ()(RBX::Controller::Button)>>(boost::function<void ()(RBX::Controller::Button)> const&)")]
// 0xf49a24 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
pub fn stub_f49a24() -> ! {
    todo!("0xf49a24 j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Controller::Button)>::on_error(std::exception &)")]
// 0xf49a34 — j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE8on_errorERSt9exception
pub fn stub_f49a34() -> ! {
    todo!("0xf49a34 j___ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE8on_errorERSt9exception")
}

#[doc(alias = "RBX::Controller::Button * rbx::any_cast<RBX::Controller::Button,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf49a44 — j___ZN3rbx8any_castIN3RBX10Controller6ButtonENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f49a44() -> ! {
    todo!("0xf49a44 j___ZN3rbx8any_castIN3RBX10Controller6ButtonENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}
