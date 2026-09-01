//! core shard EE — 100 core stubs EA-sorted, lowest uncovered 0x8c2040..0x8d049c (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after ED 0x8c2040).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::function2<void,float,float>::operator()(float,float)const")]
// 0x8c2040 — __ZNK5boost9function2IvffEclEff
pub fn stub_8c2040() -> ! {
    todo!("0x8c2040 __ZNK5boost9function2IvffEclEff")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::remove(rbx::signals::signal<void ()(float,float)>::slot *)")]
// 0x8c2114 — __ZN3rbx7signals6signalIFvffEE6removeEPNS3_4slotE
pub fn stub_8c2114() -> ! {
    todo!("0x8c2114 __ZN3rbx7signals6signalIFvffEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::safe_static_init_mutex(void)")]
// 0x8c2204 — __ZN3rbx7signals6signalIFvffEE4slot22safe_static_init_mutexEv
pub fn stub_8c2204() -> ! {
    todo!("0x8c2204 __ZN3rbx7signals6signalIFvffEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::safe_static_do_get_mutex(void)")]
// 0x8c2208 — __ZN3rbx7signals6signalIFvffEE4slot24safe_static_do_get_mutexEv
pub fn stub_8c2208() -> ! {
    todo!("0x8c2208 __ZN3rbx7signals6signalIFvffEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::function<void ()(float,float)>,2,void ()(float,float)>::~callable()")]
// 0x8c22f8 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev
pub fn stub_8c22f8() -> ! {
    todo!("0x8c22f8 __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::function<void ()(float,float)>,2,void ()(float,float)>::~callable()")]
// 0x8c2408 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev
pub fn stub_8c2408() -> ! {
    todo!("0x8c2408 __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::~slot()")]
// 0x8c2538 — __ZN3rbx7signals6signalIFvffEE4slotD1Ev
pub fn stub_8c2538() -> ! {
    todo!("0x8c2538 __ZN3rbx7signals6signalIFvffEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::~slot()")]
// 0x8c2564 — __ZN3rbx7signals6signalIFvffEE4slotD0Ev
pub fn stub_8c2564() -> ! {
    todo!("0x8c2564 __ZN3rbx7signals6signalIFvffEE4slotD0Ev")
}

#[doc(alias = "boost::function2<void,float,float>::assign_to_own(boost::function2<void,float,float> const&)")]
// 0x8c2638 — __ZN5boost9function2IvffE13assign_to_ownERKS1_
pub fn stub_8c2638() -> ! {
    todo!("0x8c2638 __ZN5boost9function2IvffE13assign_to_ownERKS1_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::callable<rbx::signals::signal<void ()(void)>*>(boost::function<void ()(void)> const&,rbx::signals::signal<void ()(void)>*)")]
// 0x8c5040 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_EC2IPS4_EERKS8_T_
pub fn stub_8c5040() -> ! {
    todo!("0x8c5040 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::call(void)")]
// 0x8c5140 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_E4callEv
pub fn stub_8c5140() -> ! {
    todo!("0x8c5140 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_E4callEv")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::call(void)")]
// 0x8c5148 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_E4callEv
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::call(void)
pub fn stub_8c5148() -> ! {
    todo!("0x8c5148 __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_E4callEv")
}

#[doc(alias = "std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::resize(unsigned long,RBX::UserInputService::SwipeDirection)")]
// 0x8c5548 — __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE6resizeEmS2_
pub fn stub_8c5548() -> ! {
    todo!("0x8c5548 __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::push_back(RBX::UserInputService::SwipeDirection const&)")]
// 0x8c5580 — __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE9push_backERKS2_
pub fn stub_8c5580() -> ! {
    todo!("0x8c5580 __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::UserInputService::SwipeDirection,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::operator[](RBX::Name const* const&)")]
// 0x8c55ac — __ZNSt3mapIPKN3RBX4NameENS0_16UserInputService14SwipeDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_8c55ac() -> ! {
    todo!("0x8c55ac __ZNSt3mapIPKN3RBX4NameENS0_16UserInputService14SwipeDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection> const&)")]
// 0x8c5604 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_8c5604() -> ! {
    todo!("0x8c5604 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection> const&)")]
// 0x8c56b8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_8c56b8() -> ! {
    todo!("0x8c56b8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection> const&)")]
// 0x8c5710 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_8c5710() -> ! {
    todo!("0x8c5710 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::UserInputService::SwipeDirection*,std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>>,RBX::UserInputService::SwipeDirection const&)")]
// 0x8c577c — __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_8c577c() -> ! {
    todo!("0x8c577c __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::_M_allocate(unsigned long)")]
// 0x8c5860 — __ZNSt12_Vector_baseIN3RBX16UserInputService14SwipeDirectionESaIS2_EE11_M_allocateEm
pub fn stub_8c5860() -> ! {
    todo!("0x8c5860 __ZNSt12_Vector_baseIN3RBX16UserInputService14SwipeDirectionESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::UserInputService::SwipeDirection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::UserInputService::SwipeDirection *,RBX::UserInputService::SwipeDirection *>(RBX::UserInputService::SwipeDirection *,RBX::UserInputService::SwipeDirection *,RBX::UserInputService::SwipeDirection *)")]
// 0x8c5878 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16UserInputService14SwipeDirectionES6_EET0_T_S8_S7_
pub fn stub_8c5878() -> ! {
    todo!("0x8c5878 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16UserInputService14SwipeDirectionES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::UserInputService::SwipeDirection*,std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>>,unsigned long,RBX::UserInputService::SwipeDirection const&)")]
// 0x8c58b8 — __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_8c58b8() -> ! {
    todo!("0x8c58b8 __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::UserInputService::~UserInputService()")]
// 0x8c5a94 — __ZN3RBX16UserInputServiceD2Ev
pub fn stub_8c5a94() -> ! {
    todo!("0x8c5a94 __ZN3RBX16UserInputServiceD2Ev")
}

#[doc(alias = "RBX::UserInputService * RBX::ServiceProvider::create<RBX::UserInputService>(void)const")]
// 0x8c6288 — __ZNK3RBX15ServiceProvider6createINS_16UserInputServiceEEEPT_v
pub fn stub_8c6288() -> ! {
    todo!("0x8c6288 __ZNK3RBX15ServiceProvider6createINS_16UserInputServiceEEEPT_v")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::UserInputService>(void)")]
// 0x8c64a0 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_16UserInputServiceEEEmv
pub fn stub_8c64a0() -> ! {
    todo!("0x8c64a0 __ZN3RBX15ServiceProvider15doGetClassIndexINS_16UserInputServiceEEEmv")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(bool,void *,RBX::UIEvent)>::operator()(bool,void *,RBX::UIEvent)")]
// 0x8c65b4 — __ZN3rbx7signals16signal_with_argsILi3EFvbPvN3RBX7UIEventEEEclEbS2_S4_
pub fn stub_8c65b4() -> ! {
    todo!("0x8c65b4 __ZN3rbx7signals16signal_with_argsILi3EFvbPvN3RBX7UIEventEEEclEbS2_S4_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> &)")]
// 0x8c6764 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> &)
pub fn stub_8c6764() -> ! {
    todo!("0x8c6764 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::on_error(std::exception &)")]
// 0x8c68c4 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE8on_errorERSt9exception
pub fn stub_8c68c4() -> ! {
    todo!("0x8c68c4 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_init_mutex(void)")]
// 0x8c68f0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE22safe_static_init_mutexEv
pub fn stub_8c68f0() -> ! {
    todo!("0x8c68f0 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(char const*,bool)>::operator()(char const*,bool)")]
// 0x8c68f4 — __ZN3rbx7signals16signal_with_argsILi2EFvPKcbEEclES3_b
pub fn stub_8c68f4() -> ! {
    todo!("0x8c68f4 __ZN3rbx7signals16signal_with_argsILi2EFvPKcbEEclES3_b")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(char const*,bool)>::slot> &)")]
// 0x8c6a44 — __ZN3rbx7signals6signalIFvPKcbEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// was: rbx::signals::signal<void ()(char const*,bool)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(char const*,bool)>::slot> &)
pub fn stub_8c6a44() -> ! {
    todo!("0x8c6a44 __ZN3rbx7signals6signalIFvPKcbEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::on_error(std::exception &)")]
// 0x8c6ba4 — __ZN3rbx7signals6signalIFvPKcbEE8on_errorERSt9exception
pub fn stub_8c6ba4() -> ! {
    todo!("0x8c6ba4 __ZN3rbx7signals6signalIFvPKcbEE8on_errorERSt9exception")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(char const*,bool)>::slot> const&)")]
// 0x8c6bcc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKcbEE4slotEEaSERKS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(char const*,bool)>::slot> const&)
pub fn stub_8c6bcc() -> ! {
    todo!("0x8c6bcc __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKcbEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::safe_static_init_mutex(void)")]
// 0x8c6bf0 — __ZN3rbx7signals6signalIFvPKcbEE22safe_static_init_mutexEv
pub fn stub_8c6bf0() -> ! {
    todo!("0x8c6bf0 __ZN3rbx7signals6signalIFvPKcbEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::safe_static_do_get_mutex(void)")]
// 0x8c6bf4 — __ZN3rbx7signals6signalIFvPKcbEE24safe_static_do_get_mutexEv
pub fn stub_8c6bf4() -> ! {
    todo!("0x8c6bf4 __ZN3rbx7signals6signalIFvPKcbEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "RBX::UserInputServiceJob::~UserInputServiceJob()")]
// 0x8c6ea4 — __ZN3RBX19UserInputServiceJobD1Ev
pub fn stub_8c6ea4() -> ! {
    todo!("0x8c6ea4 __ZN3RBX19UserInputServiceJobD1Ev")
}

#[doc(alias = "RBX::UserInputServiceJob::~UserInputServiceJob()")]
// 0x8c6fc4 — __ZN3RBX19UserInputServiceJobD0Ev
pub fn stub_8c6fc4() -> ! {
    todo!("0x8c6fc4 __ZN3RBX19UserInputServiceJobD0Ev")
}

#[doc(alias = "RBX::UserInputServiceJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x8c70f8 — __ZN3RBX19UserInputServiceJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_8c70f8() -> ! {
    todo!("0x8c70f8 __ZN3RBX19UserInputServiceJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::UserInputServiceJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x8c7168 — __ZN3RBX19UserInputServiceJob5errorERKNS_13TaskScheduler3Job5StatsE
pub fn stub_8c7168() -> ! {
    todo!("0x8c7168 __ZN3RBX19UserInputServiceJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::UserInputServiceJob::processTasks(void)")]
// 0x8c7418 — __ZN3RBX19UserInputServiceJob12processTasksEv
pub fn stub_8c7418() -> ! {
    todo!("0x8c7418 __ZN3RBX19UserInputServiceJob12processTasksEv")
}

#[doc(alias = "RBX::IStepped::~IStepped()")]
// 0x8c8168 — __ZN3RBX8ISteppedD0Ev
pub fn stub_8c8168() -> ! {
    todo!("0x8c8168 __ZN3RBX8ISteppedD0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::disconnectAll(void)")]
// 0x8c8208 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13disconnectAllEv
pub fn stub_8c8208() -> ! {
    todo!("0x8c8208 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::disconnectAll(void)")]
// 0x8c8380 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13disconnectAllEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::disconnectAll(void)
pub fn stub_8c8380() -> ! {
    todo!("0x8c8380 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13disconnectAllEv")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot> const&)")]
// 0x8c84f8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSERKSB_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot> const&)
pub fn stub_8c84f8() -> ! {
    todo!("0x8c84f8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSERKSB_")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::safe_static_do_get_mutex(void)")]
// 0x8c8520 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE24safe_static_do_get_mutexEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::safe_static_do_get_mutex(void)
pub fn stub_8c8520() -> ! {
    todo!("0x8c8520 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::disconnectAll(void)")]
// 0x8c8618 — __ZN3rbx7signals6signalIFvPKcbEE13disconnectAllEv
pub fn stub_8c8618() -> ! {
    todo!("0x8c8618 __ZN3rbx7signals6signalIFvPKcbEE13disconnectAllEv")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>> *)")]
// 0x8c8790 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_8c8790() -> ! {
    todo!("0x8c8790 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "RBX::FWService::FWService(void)")]
// 0x8c8eac — __ZN3RBX9FWServiceC1Ev
pub fn stub_8c8eac() -> ! {
    todo!("0x8c8eac __ZN3RBX9FWServiceC1Ev")
}

#[doc(alias = "RBX::FWService::FWService(void)")]
// 0x8c8eb0 — __ZN3RBX9FWServiceC2Ev
pub fn stub_8c8eb0() -> ! {
    todo!("0x8c8eb0 __ZN3RBX9FWServiceC2Ev")
}

#[doc(alias = "RBX::FWService::~FWService()")]
// 0x8c9080 — __ZN3RBX9FWServiceD0Ev
pub fn stub_8c9080() -> ! {
    todo!("0x8c9080 __ZN3RBX9FWServiceD0Ev")
}

#[doc(alias = "RBX::FWService::~FWService()")]
// 0x8c9120 — __ZN3RBX9FWServiceD1Ev
pub fn stub_8c9120() -> ! {
    todo!("0x8c9120 __ZN3RBX9FWServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::FWService::~FWService()")]
// 0x8c9124 — __ZThn32_N3RBX9FWServiceD0Ev
// was: `non-virtual thunk to'RBX::FWService::~FWService()
pub fn stub_8c9124() -> ! {
    todo!("0x8c9124 __ZThn32_N3RBX9FWServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::FWService::~FWService()")]
// 0x8c912c — __ZThn36_N3RBX9FWServiceD0Ev
// was: `non-virtual thunk to'RBX::FWService::~FWService()
pub fn stub_8c912c() -> ! {
    todo!("0x8c912c __ZThn36_N3RBX9FWServiceD0Ev")
}

#[doc(alias = "RBX::FWService::~FWService()")]
// 0x8c9134 — __ZN3RBX9FWServiceD2Ev
pub fn stub_8c9134() -> ! {
    todo!("0x8c9134 __ZN3RBX9FWServiceD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::FWService::~FWService()")]
// 0x8c9210 — __ZThn32_N3RBX9FWServiceD1Ev
// was: `non-virtual thunk to'RBX::FWService::~FWService()
pub fn stub_8c9210() -> ! {
    todo!("0x8c9210 __ZThn32_N3RBX9FWServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::FWService::~FWService()")]
// 0x8c9218 — __ZThn36_N3RBX9FWServiceD1Ev
// was: `non-virtual thunk to'RBX::FWService::~FWService()
pub fn stub_8c9218() -> ! {
    todo!("0x8c9218 __ZThn36_N3RBX9FWServiceD1Ev")
}

#[doc(alias = "RBX::FWService::getUniqueSharedPtr(void)")]
// 0x8c9220 — __ZN3RBX9FWService18getUniqueSharedPtrEv
pub fn stub_8c9220() -> ! {
    todo!("0x8c9220 __ZN3RBX9FWService18getUniqueSharedPtrEv")
}

#[doc(alias = "non-virtual thunk toRBX::FWService::getUniqueSharedPtr(void)")]
// 0x8c92ec — __ZThn96_N3RBX9FWService18getUniqueSharedPtrEv
// was: `non-virtual thunk to'RBX::FWService::getUniqueSharedPtr(void)
pub fn stub_8c92ec() -> ! {
    todo!("0x8c92ec __ZThn96_N3RBX9FWService18getUniqueSharedPtrEv")
}

#[doc(alias = "RBX::FWService::isOwnedHolder(RBX::IFWHolder *)")]
// 0x8c92f8 — __ZN3RBX9FWService13isOwnedHolderEPNS_9IFWHolderE
pub fn stub_8c92f8() -> ! {
    todo!("0x8c92f8 __ZN3RBX9FWService13isOwnedHolderEPNS_9IFWHolderE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWService> RBX::shared_from<RBX::FWService>(RBX::FWService*)")]
// 0x8c9308 — __ZN3RBX11shared_fromINS_9FWServiceEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::FWService> RBX::shared_from<RBX::FWService>(RBX::FWService*)
pub fn stub_8c9308() -> ! {
    todo!("0x8c9308 __ZN3RBX11shared_fromINS_9FWServiceEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWBase>::operator=(rbx_core::SharedPtr<RBX::FWBase> const&)")]
// 0x8c99ac — __ZN5boost10shared_ptrIN3RBX6FWBaseEEaSERKS3_
// was: boost::shared_ptr<RBX::FWBase>::operator=(boost::shared_ptr<RBX::FWBase> const&)
pub fn stub_8c99ac() -> ! {
    todo!("0x8c99ac __ZN5boost10shared_ptrIN3RBX6FWBaseEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWBase>::shared_ptr<RBX::FWBase>(RBX::FWBase *)")]
// 0x8c99e8 — __ZN5boost10shared_ptrIN3RBX6FWBaseEEC2IS2_EEPT_
// was: boost::shared_ptr<RBX::FWBase>::shared_ptr<RBX::FWBase>(RBX::FWBase *)
pub fn stub_8c99e8() -> ! {
    todo!("0x8c99e8 __ZN5boost10shared_ptrIN3RBX6FWBaseEEC2IS2_EEPT_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::FWBase>::_internal_accept_owner<RBX::FWBase,RBX::FWBase>(rbx_core::SharedPtr<RBX::FWBase> const*,RBX::FWBase *)const")]
// 0x8c9ad0 — __ZNK5boost23enable_shared_from_thisIN3RBX6FWBaseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::FWBase>::_internal_accept_owner<RBX::FWBase,RBX::FWBase>(boost::shared_ptr<RBX::FWBase> const*,RBX::FWBase *)const
pub fn stub_8c9ad0() -> ! {
    todo!("0x8c9ad0 __ZNK5boost23enable_shared_from_thisIN3RBX6FWBaseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::FWBase>::get_untyped_deleter(void)")]
// 0x8c9bb8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6FWBaseEE19get_untyped_deleterEv
pub fn stub_8c9bb8() -> ! {
    todo!("0x8c9bb8 __ZN5boost6detail17sp_counted_impl_pIN3RBX6FWBaseEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::FWBase::~FWBase()")]
// 0x8c9c10 — __ZN3RBX6FWBaseD2Ev
pub fn stub_8c9c10() -> ! {
    todo!("0x8c9c10 __ZN3RBX6FWBaseD2Ev")
}

#[doc(alias = "RBX::FWBase::~FWBase()")]
// 0x8c9d28 — __ZN3RBX6FWBaseD1Ev
pub fn stub_8c9d28() -> ! {
    todo!("0x8c9d28 __ZN3RBX6FWBaseD1Ev")
}

#[doc(alias = "RBX::FWBase::FWBase(RBX::FWBase const&)")]
// 0x8c9d2c — __ZN3RBX6FWBaseC2ERKS0_
pub fn stub_8c9d2c() -> ! {
    todo!("0x8c9d2c __ZN3RBX6FWBaseC2ERKS0_")
}

#[doc(alias = "RBX::MarketplaceService::signalPromptProductPurchaseFinished(int,int,bool)")]
// 0x8ca664 — __ZN3RBX18MarketplaceService35signalPromptProductPurchaseFinishedEiib
pub fn stub_8ca664() -> ! {
    todo!("0x8ca664 __ZN3RBX18MarketplaceService35signalPromptProductPurchaseFinishedEiib")
}

#[doc(alias = "RBX::MarketplaceService::signalClientPurchaseSuccess(std::string,int,int)")]
// 0x8ca78c — __ZN3RBX18MarketplaceService27signalClientPurchaseSuccessESsii
pub fn stub_8ca78c() -> ! {
    todo!("0x8ca78c __ZN3RBX18MarketplaceService27signalClientPurchaseSuccessESsii")
}

#[doc(alias = "RBX::StringConverter<RBX::MarketplaceService::CurrencyType>::convertToValue(std::string const&,RBX::MarketplaceService::CurrencyType&)")]
// 0x8cbe38 — __ZN3RBX15StringConverterINS_18MarketplaceService12CurrencyTypeEE14convertToValueERKSsRS2_
pub fn stub_8cbe38() -> ! {
    todo!("0x8cbe38 __ZN3RBX15StringConverterINS_18MarketplaceService12CurrencyTypeEE14convertToValueERKSsRS2_")
}

#[doc(alias = "RBX::MarketplaceService::MarketplaceService(void)")]
// 0x8cbe84 — __ZN3RBX18MarketplaceServiceC1Ev
pub fn stub_8cbe84() -> ! {
    todo!("0x8cbe84 __ZN3RBX18MarketplaceServiceC1Ev")
}

#[doc(alias = "RBX::MarketplaceService::MarketplaceService(void)")]
// 0x8cbe88 — __ZN3RBX18MarketplaceServiceC2Ev
pub fn stub_8cbe88() -> ! {
    todo!("0x8cbe88 __ZN3RBX18MarketplaceServiceC2Ev")
}

#[doc(alias = "RBX::MarketplaceService::processPlayerOwnsAssetResponse(std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x8cc884 — __ZN3RBX18MarketplaceService30processPlayerOwnsAssetResponseEPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE
pub fn stub_8cc884() -> ! {
    todo!("0x8cc884 __ZN3RBX18MarketplaceService30processPlayerOwnsAssetResponseEPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE")
}

#[doc(alias = "RBX::MarketplaceService::setProductInfoUrl(std::string)")]
// 0x8cce78 — __ZN3RBX18MarketplaceService17setProductInfoUrlESs
pub fn stub_8cce78() -> ! {
    todo!("0x8cce78 __ZN3RBX18MarketplaceService17setProductInfoUrlESs")
}

#[doc(alias = "RBX::MarketplaceService::setPlayerOwnsAssetUrl(std::string)")]
// 0x8ccec0 — __ZN3RBX18MarketplaceService21setPlayerOwnsAssetUrlESs
pub fn stub_8ccec0() -> ! {
    todo!("0x8ccec0 __ZN3RBX18MarketplaceService21setPlayerOwnsAssetUrlESs")
}

#[doc(alias = "void RBX::MarketplaceService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0x8cdd2c — __ZN3RBX18MarketplaceService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
pub fn stub_8cdd2c() -> ! {
    todo!("0x8cdd2c __ZN3RBX18MarketplaceService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_5<RBX::MarketplaceService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,RBX::MarketplaceService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (RBX::MarketplaceService::*)(std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),RBX::MarketplaceService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x8cdfd8 — __ZN5boost4bindIvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENS6_IFvSsEEEPS2_NS_3argILi1EEENSC_ILi2EEES8_SA_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISH_T0_T1_T2_T3_T4_EENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSK_FSH_SL_SM_SN_SO_ESR_SS_ST_SU_SV_
pub fn stub_8cdfd8() -> ! {
    todo!("0x8cdfd8 __ZN5boost4bindIvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENS6_IFvSsEEEPS2_NS_3argILi1EEENSC_ILi2EEES8_SA_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISH_T0_T1_T2_T3_T4_EENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSK_FSH_SL_SM_SN_SO_ESR_SS_ST_SU_SV_")
}

#[doc(alias = "RBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce4a0 — __ZN3RBX18MarketplaceServiceD1Ev
pub fn stub_8ce4a0() -> ! {
    todo!("0x8ce4a0 __ZN3RBX18MarketplaceServiceD1Ev")
}

#[doc(alias = "RBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce4a4 — __ZN3RBX18MarketplaceServiceD0Ev
pub fn stub_8ce4a4() -> ! {
    todo!("0x8ce4a4 __ZN3RBX18MarketplaceServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce55c — __ZThn32_N3RBX18MarketplaceServiceD1Ev
// was: `non-virtual thunk to'RBX::MarketplaceService::~MarketplaceService()
pub fn stub_8ce55c() -> ! {
    todo!("0x8ce55c __ZThn32_N3RBX18MarketplaceServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce564 — __ZThn32_N3RBX18MarketplaceServiceD0Ev
// was: `non-virtual thunk to'RBX::MarketplaceService::~MarketplaceService()
pub fn stub_8ce564() -> ! {
    todo!("0x8ce564 __ZThn32_N3RBX18MarketplaceServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce618 — __ZThn36_N3RBX18MarketplaceServiceD1Ev
// was: `non-virtual thunk to'RBX::MarketplaceService::~MarketplaceService()
pub fn stub_8ce618() -> ! {
    todo!("0x8ce618 __ZThn36_N3RBX18MarketplaceServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce620 — __ZThn36_N3RBX18MarketplaceServiceD0Ev
// was: `non-virtual thunk to'RBX::MarketplaceService::~MarketplaceService()
pub fn stub_8ce620() -> ! {
    todo!("0x8ce620 __ZThn36_N3RBX18MarketplaceServiceD0Ev")
}

#[doc(alias = "RBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce6c8 — __ZN3RBX18MarketplaceServiceD2Ev
pub fn stub_8ce6c8() -> ! {
    todo!("0x8ce6c8 __ZN3RBX18MarketplaceServiceD2Ev")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::MarketplaceService::CurrencyType>(RBX::MarketplaceService::CurrencyType const&)")]
// 0x8ceaf8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_18MarketplaceService12CurrencyTypeEEERS3_RKT_
pub fn stub_8ceaf8() -> ! {
    todo!("0x8ceaf8 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_18MarketplaceService12CurrencyTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MarketplaceService::CurrencyType>::singleton(void)")]
// 0x8ceb48 — __ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE9singletonEv
pub fn stub_8ceb48() -> ! {
    todo!("0x8ceb48 __ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MarketplaceService::CurrencyType>::destruct_func(char *)")]
// 0x8cebb8 — __ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE13destruct_funcEPc
pub fn stub_8cebb8() -> ! {
    todo!("0x8cebb8 __ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::MarketplaceService::CurrencyType const& rbx::any_cast<RBX::MarketplaceService::CurrencyType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x8cebc0 — __ZN3rbx8any_castIRKN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_8cebc0() -> ! {
    todo!("0x8cebc0 __ZN3rbx8any_castIRKN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::safe_static_init_mutex(void)")]
// 0x8cf2f8 — __ZN3rbx7signals6signalIFvSsiiEE22safe_static_init_mutexEv
pub fn stub_8cf2f8() -> ! {
    todo!("0x8cf2f8 __ZN3rbx7signals6signalIFvSsiiEE22safe_static_init_mutexEv")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0x8cfa40 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_
pub fn stub_8cfa40() -> ! {
    todo!("0x8cfa40 __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x8cfbb0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
pub fn stub_8cfbb0() -> ! {
    todo!("0x8cfbb0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// 0x8cfbcc — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_
pub fn stub_8cfbcc() -> ! {
    todo!("0x8cfbcc __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0x8cfbf0 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_8cfbf0() -> ! {
    todo!("0x8cfbf0 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x8cfd50 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_8cfd50() -> ! {
    todo!("0x8cfd50 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x8cfeac — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_8cfeac() -> ! {
    todo!("0x8cfeac __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>> &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0x8cffb8 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_8cffb8() -> ! {
    todo!("0x8cffb8 __ZN5boost3_bi5list5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::operator()(RBX::MarketplaceService*,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)const")]
// 0x8d00c8 — __ZNK5boost4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_
pub fn stub_8d00c8() -> ! {
    todo!("0x8d00c8 __ZNK5boost4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x8d01e8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_8d01e8() -> ! {
    todo!("0x8d01e8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x8d03a0 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
pub fn stub_8d03a0() -> ! {
    todo!("0x8d03a0 __ZN5boost3_bi5list5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x8d049c — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
pub fn stub_8d049c() -> ! {
    todo!("0x8d049c __ZN5boost3_bi8storage5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")
}

