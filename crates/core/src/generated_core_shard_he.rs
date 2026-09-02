//! core shard HE — 100 core stubs EA-sorted, 0xf59124..0xf5a714 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HD 0xf590c4).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HD 0xf590c4 (0xf59124..0xf5a714, 20814->20914 covered, 1004 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "void RBX::GamePassService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xf59124 — j___ZN3RBX15GamePassService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
pub fn stub_0xf59124() -> ! {
    todo!("0xf59124 j___ZN3RBX15GamePassService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::UserInputService>(void)")]
// 0xf59204 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_16UserInputServiceEEEmv
pub fn stub_0xf59204() -> ! {
    todo!("0xf59204 j___ZN3RBX15ServiceProvider15doGetClassIndexINS_16UserInputServiceEEEmv")
}

#[doc(alias = "RBX::UserInputService::~UserInputService()")]
// 0xf59214 — j___ZN3RBX16UserInputServiceD0Ev
pub fn stub_0xf59214() -> ! {
    todo!("0xf59214 j___ZN3RBX16UserInputServiceD0Ev")
}

#[doc(alias = "RBX::UserInputService::~UserInputService()")]
// 0xf59224 — j___ZN3RBX16UserInputServiceD2Ev
pub fn stub_0xf59224() -> ! {
    todo!("0xf59224 j___ZN3RBX16UserInputServiceD2Ev")
}

#[doc(alias = "RBX::UserInputServiceJob::processTasks(void)")]
// 0xf59234 — j___ZN3RBX19UserInputServiceJob12processTasksEv
pub fn stub_0xf59234() -> ! {
    todo!("0xf59234 j___ZN3RBX19UserInputServiceJob12processTasksEv")
}

#[doc(alias = "RBX::IStepped::~IStepped()")]
// 0xf59294 — j___ZN3RBX8ISteppedD2Ev
pub fn stub_0xf59294() -> ! {
    todo!("0xf59294 j___ZN3RBX8ISteppedD2Ev")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::UserInputService::SwipeDirection>(RBX::UserInputService::SwipeDirection const&)")]
// 0xf592c4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16UserInputService14SwipeDirectionEEERS3_RKT_
pub fn stub_0xf592c4() -> ! {
    todo!("0xf592c4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16UserInputService14SwipeDirectionEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject>::singleton(void)")]
// 0xf592d4 — j___ZN3rbx14implementation12typed_holderIN3RBX11InputObjectEE9singletonEv
pub fn stub_0xf592d4() -> ! {
    todo!("0xf592d4 j___ZN3rbx14implementation12typed_holderIN3RBX11InputObjectEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UserInputService::SwipeDirection>::singleton(void)")]
// 0xf592e4 — j___ZN3rbx14implementation12typed_holderIN3RBX16UserInputService14SwipeDirectionEE9singletonEv
pub fn stub_0xf592e4() -> ! {
    todo!("0xf592e4 j___ZN3rbx14implementation12typed_holderIN3RBX16UserInputService14SwipeDirectionEE9singletonEv")
}

#[doc(alias = "rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0>::operator new(unsigned long)")]
// 0xf59304 — j___ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EEnwEm
pub fn stub_0xf59304() -> ! {
    todo!("0xf59304 j___ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EEnwEm")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::InputObject)>::operator()(RBX::InputObject)")]
// 0xf59344 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX11InputObjectEEEclES3_
pub fn stub_0xf59344() -> ! {
    todo!("0xf59344 j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX11InputObjectEEEclES3_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::UserInputService::SwipeDirection)>::operator()(RBX::UserInputService::SwipeDirection)")]
// 0xf59354 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX16UserInputService14SwipeDirectionEEEclES4_
pub fn stub_0xf59354() -> ! {
    todo!("0xf59354 j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX16UserInputService14SwipeDirectionEEEclES4_")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(char const*,bool)>::operator()(char const*,bool)")]
// 0xf59364 — j___ZN3rbx7signals16signal_with_argsILi2EFvPKcbEEclES3_b
pub fn stub_0xf59364() -> ! {
    todo!("0xf59364 j___ZN3rbx7signals16signal_with_argsILi2EFvPKcbEEclES3_b")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(float,float)>::operator()(float,float)")]
// 0xf59374 — j___ZN3rbx7signals16signal_with_argsILi2EFvffEEclEff
pub fn stub_0xf59374() -> ! {
    todo!("0xf59374 j___ZN3rbx7signals16signal_with_argsILi2EFvffEEclEff")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(bool,void *,RBX::UIEvent)>::operator()(bool,void *,RBX::UIEvent)")]
// 0xf59384 — j___ZN3rbx7signals16signal_with_argsILi3EFvbPvN3RBX7UIEventEEEclEbS2_S4_
pub fn stub_0xf59384() -> ! {
    todo!("0xf59384 j___ZN3rbx7signals16signal_with_argsILi3EFvbPvN3RBX7UIEventEEEclEbS2_S4_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::disconnectAll(void)")]
// 0xf59414 — j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE13disconnectAllEv
pub fn stub_0xf59414() -> ! {
    todo!("0xf59414 j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::safe_static_do_get_mutex(void)")]
// 0xf59424 — j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE24safe_static_do_get_mutexEv
pub fn stub_0xf59424() -> ! {
    todo!("0xf59424 j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::InputObject)>::slot> &)")]
// 0xf59434 — j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
pub fn stub_0xf59434() -> ! {
    todo!("0xf59434 j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::slot::safe_static_do_get_mutex(void)")]
// 0xf59444 — j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0xf59444() -> ! {
    todo!("0xf59444 j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::insert(rbx::signals::signal<void ()(RBX::InputObject)>::slot *)")]
// 0xf59454 — j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE6insertEPNS5_4slotE
pub fn stub_0xf59454() -> ! {
    todo!("0xf59454 j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE6insertEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::remove(rbx::signals::signal<void ()(RBX::InputObject)>::slot *)")]
// 0xf59464 — j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE6removeEPNS5_4slotE
pub fn stub_0xf59464() -> ! {
    todo!("0xf59464 j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE6removeEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::InputObject)>::connect<boost::function<void ()(RBX::InputObject)>>(boost::function<void ()(RBX::InputObject)> const&)")]
// 0xf59474 — j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
pub fn stub_0xf59474() -> ! {
    todo!("0xf59474 j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::on_error(std::exception &)")]
// 0xf59484 — j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE8on_errorERSt9exception
pub fn stub_0xf59484() -> ! {
    todo!("0xf59484 j___ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::disconnectAll(void)")]
// 0xf59494 — j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE13disconnectAllEv
pub fn stub_0xf59494() -> ! {
    todo!("0xf59494 j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::safe_static_do_get_mutex(void)")]
// 0xf594a4 — j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE24safe_static_do_get_mutexEv
pub fn stub_0xf594a4() -> ! {
    todo!("0xf594a4 j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot> &)")]
// 0xf594b4 — j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
pub fn stub_0xf594b4() -> ! {
    todo!("0xf594b4 j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot::safe_static_do_get_mutex(void)")]
// 0xf594c4 — j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0xf594c4() -> ! {
    todo!("0xf594c4 j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::insert(rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot *)")]
// 0xf594d4 — j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE6insertEPNS6_4slotE
pub fn stub_0xf594d4() -> ! {
    todo!("0xf594d4 j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::remove(rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot *)")]
// 0xf594e4 — j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE6removeEPNS6_4slotE
pub fn stub_0xf594e4() -> ! {
    todo!("0xf594e4 j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::connect<boost::function<void ()(RBX::UserInputService::SwipeDirection)>>(boost::function<void ()(RBX::UserInputService::SwipeDirection)> const&)")]
// 0xf594f4 — j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
pub fn stub_0xf594f4() -> ! {
    todo!("0xf594f4 j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::on_error(std::exception &)")]
// 0xf59504 — j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE8on_errorERSt9exception
pub fn stub_0xf59504() -> ! {
    todo!("0xf59504 j___ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::disconnectAll(void)")]
// 0xf59514 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13disconnectAllEv
pub fn stub_0xf59514() -> ! {
    todo!("0xf59514 j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::safe_static_do_get_mutex(void)")]
// 0xf59524 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE24safe_static_do_get_mutexEv
pub fn stub_0xf59524() -> ! {
    todo!("0xf59524 j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::disconnectAll(void)")]
// 0xf59534 — j___ZN3rbx7signals6signalIFvPKcbEE13disconnectAllEv
pub fn stub_0xf59534() -> ! {
    todo!("0xf59534 j___ZN3rbx7signals6signalIFvPKcbEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::safe_static_do_get_mutex(void)")]
// 0xf59544 — j___ZN3rbx7signals6signalIFvPKcbEE24safe_static_do_get_mutexEv
pub fn stub_0xf59544() -> ! {
    todo!("0xf59544 j___ZN3rbx7signals6signalIFvPKcbEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(char const*,bool)>::slot> &)")]
// 0xf59554 — j___ZN3rbx7signals6signalIFvPKcbEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
pub fn stub_0xf59554() -> ! {
    todo!("0xf59554 j___ZN3rbx7signals6signalIFvPKcbEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::on_error(std::exception &)")]
// 0xf59564 — j___ZN3rbx7signals6signalIFvPKcbEE8on_errorERSt9exception
pub fn stub_0xf59564() -> ! {
    todo!("0xf59564 j___ZN3rbx7signals6signalIFvPKcbEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::disconnectAll(void)")]
// 0xf59574 — j___ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13disconnectAllEv
pub fn stub_0xf59574() -> ! {
    todo!("0xf59574 j___ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> &)")]
// 0xf59584 — j___ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
pub fn stub_0xf59584() -> ! {
    todo!("0xf59584 j___ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::on_error(std::exception &)")]
// 0xf59594 — j___ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE8on_errorERSt9exception
pub fn stub_0xf59594() -> ! {
    todo!("0xf59594 j___ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::disconnectAll(void)")]
// 0xf595a4 — j___ZN3rbx7signals6signalIFvffEE13disconnectAllEv
pub fn stub_0xf595a4() -> ! {
    todo!("0xf595a4 j___ZN3rbx7signals6signalIFvffEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::safe_static_do_get_mutex(void)")]
// 0xf595b4 — j___ZN3rbx7signals6signalIFvffEE24safe_static_do_get_mutexEv
pub fn stub_0xf595b4() -> ! {
    todo!("0xf595b4 j___ZN3rbx7signals6signalIFvffEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float)>::slot> &)")]
// 0xf595c4 — j___ZN3rbx7signals6signalIFvffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
pub fn stub_0xf595c4() -> ! {
    todo!("0xf595c4 j___ZN3rbx7signals6signalIFvffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::safe_static_do_get_mutex(void)")]
// 0xf595d4 — j___ZN3rbx7signals6signalIFvffEE4slot24safe_static_do_get_mutexEv
pub fn stub_0xf595d4() -> ! {
    todo!("0xf595d4 j___ZN3rbx7signals6signalIFvffEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::insert(rbx::signals::signal<void ()(float,float)>::slot *)")]
// 0xf595e4 — j___ZN3rbx7signals6signalIFvffEE6insertEPNS3_4slotE
pub fn stub_0xf595e4() -> ! {
    todo!("0xf595e4 j___ZN3rbx7signals6signalIFvffEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::remove(rbx::signals::signal<void ()(float,float)>::slot *)")]
// 0xf595f4 — j___ZN3rbx7signals6signalIFvffEE6removeEPNS3_4slotE
pub fn stub_0xf595f4() -> ! {
    todo!("0xf595f4 j___ZN3rbx7signals6signalIFvffEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float)>::connect<boost::function<void ()(float,float)>>(boost::function<void ()(float,float)> const&)")]
// 0xf59604 — j___ZN3rbx7signals6signalIFvffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_0xf59604() -> ! {
    todo!("0xf59604 j___ZN3rbx7signals6signalIFvffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::on_error(std::exception &)")]
// 0xf59614 — j___ZN3rbx7signals6signalIFvffEE8on_errorERSt9exception
pub fn stub_0xf59614() -> ! {
    todo!("0xf59614 j___ZN3rbx7signals6signalIFvffEE8on_errorERSt9exception")
}

#[doc(alias = "RBX::InputObject const& rbx::any_cast<RBX::InputObject const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf59634 — j___ZN3rbx8any_castIRKN3RBX11InputObjectENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0xf59634() -> ! {
    todo!("0xf59634 j___ZN3rbx8any_castIRKN3RBX11InputObjectENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::UserInputService::SwipeDirection const& rbx::any_cast<RBX::UserInputService::SwipeDirection const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf59644 — j___ZN3rbx8any_castIRKN3RBX16UserInputService14SwipeDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0xf59644() -> ! {
    todo!("0xf59644 j___ZN3rbx8any_castIRKN3RBX16UserInputService14SwipeDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::InputObject)>::slot,boost::function<void ()(RBX::InputObject)>,1,void ()(RBX::InputObject)>::callable<rbx::signals::signal<void ()(RBX::InputObject)>*>(boost::function<void ()(RBX::InputObject)> const&,rbx::signals::signal<void ()(RBX::InputObject)>*)")]
// 0xf59664 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX11InputObjectEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
pub fn stub_0xf59664() -> ! {
    todo!("0xf59664 j___ZN3rbx8callableINS_7signals6signalIFvN3RBX11InputObjectEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot,boost::function<void ()(RBX::UserInputService::SwipeDirection)>,1,void ()(RBX::UserInputService::SwipeDirection)>::callable<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>*>(boost::function<void ()(RBX::UserInputService::SwipeDirection)> const&,rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>*)")]
// 0xf59674 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
pub fn stub_0xf59674() -> ! {
    todo!("0xf59674 j___ZN3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::function<void ()(float,float)>,2,void ()(float,float)>::callable<rbx::signals::signal<void ()(float,float)>*>(boost::function<void ()(float,float)> const&,rbx::signals::signal<void ()(float,float)>*)")]
// 0xf59684 — j___ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_
pub fn stub_0xf59684() -> ! {
    todo!("0xf59684 j___ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::callable<rbx::signals::signal<void ()(void)>*>(boost::function<void ()(void)> const&,rbx::signals::signal<void ()(void)>*)")]
// 0xf59694 — j___ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_EC2IPS4_EERKS8_T_
pub fn stub_0xf59694() -> ! {
    todo!("0xf59694 j___ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::UserInputServiceJob>::shared_ptr<RBX::UserInputServiceJob>(RBX::UserInputServiceJob *)")]
// 0xf596c4 — j___ZN5boost10shared_ptrIN3RBX19UserInputServiceJobEEC2IS2_EEPT_
pub fn stub_0xf596c4() -> ! {
    todo!("0xf596c4 j___ZN5boost10shared_ptrIN3RBX19UserInputServiceJobEEC2IS2_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::UserInputServiceJob>::operator=(rbx_core::SharedPtr<RBX::UserInputServiceJob> const&)")]
// 0xf596d4 — j___ZN5boost10shared_ptrIN3RBX19UserInputServiceJobEEaSERKS3_
pub fn stub_0xf596d4() -> ! {
    todo!("0xf596d4 j___ZN5boost10shared_ptrIN3RBX19UserInputServiceJobEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::InputObject)>::slot>::operator=(rbx::signals::signal<void ()(RBX::InputObject)>::slot*)")]
// 0xf59714 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slotEEaSEPS8_
pub fn stub_0xf59714() -> ! {
    todo!("0xf59714 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slotEEaSEPS8_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::InputObject)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::InputObject)>::slot> const&)")]
// 0xf59724 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slotEEaSERKS9_
pub fn stub_0xf59724() -> ! {
    todo!("0xf59724 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot*)")]
// 0xf59734 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEEaSEPS9_
pub fn stub_0xf59734() -> ! {
    todo!("0xf59734 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot> const&)")]
// 0xf59744 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEEaSERKSA_
pub fn stub_0xf59744() -> ! {
    todo!("0xf59744 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEEaSERKSA_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot> const&)")]
// 0xf59754 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSERKSB_
pub fn stub_0xf59754() -> ! {
    todo!("0xf59754 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSERKSB_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(char const*,bool)>::slot> const&)")]
// 0xf59764 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKcbEE4slotEEaSERKS9_
pub fn stub_0xf59764() -> ! {
    todo!("0xf59764 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKcbEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float)>::slot*)")]
// 0xf59774 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffEE4slotEEaSEPS6_
pub fn stub_0xf59774() -> ! {
    todo!("0xf59774 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float)>::slot> const&)")]
// 0xf59784 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffEE4slotEEaSERKS7_
pub fn stub_0xf59784() -> ! {
    todo!("0xf59784 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffEE4slotEEaSERKS7_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::UserInputServiceJob>(RBX::UserInputServiceJob *)")]
// 0xf59a54 — j___ZN5boost6detail12shared_countC2IN3RBX19UserInputServiceJobEEEPT_
pub fn stub_0xf59a54() -> ! {
    todo!("0xf59a54 j___ZN5boost6detail12shared_countC2IN3RBX19UserInputServiceJobEEEPT_")
}

#[doc(alias = "boost::function1<void,RBX::InputObject>::assign_to_own(boost::function1<void,RBX::InputObject> const&)")]
// 0xf59c74 — j___ZN5boost9function1IvN3RBX11InputObjectEE13assign_to_ownERKS3_
pub fn stub_0xf59c74() -> ! {
    todo!("0xf59c74 j___ZN5boost9function1IvN3RBX11InputObjectEE13assign_to_ownERKS3_")
}

#[doc(alias = "boost::function1<void,RBX::InputObject>::clear(void)")]
// 0xf59c84 — j___ZN5boost9function1IvN3RBX11InputObjectEE5clearEv
pub fn stub_0xf59c84() -> ! {
    todo!("0xf59c84 j___ZN5boost9function1IvN3RBX11InputObjectEE5clearEv")
}

#[doc(alias = "boost::function1<void,RBX::UserInputService::SwipeDirection>::assign_to_own(boost::function1<void,RBX::UserInputService::SwipeDirection> const&)")]
// 0xf59cb4 — j___ZN5boost9function1IvN3RBX16UserInputService14SwipeDirectionEE13assign_to_ownERKS4_
pub fn stub_0xf59cb4() -> ! {
    todo!("0xf59cb4 j___ZN5boost9function1IvN3RBX16UserInputService14SwipeDirectionEE13assign_to_ownERKS4_")
}

#[doc(alias = "boost::function1<void,RBX::UserInputService::SwipeDirection>::clear(void)")]
// 0xf59cc4 — j___ZN5boost9function1IvN3RBX16UserInputService14SwipeDirectionEE5clearEv
pub fn stub_0xf59cc4() -> ! {
    todo!("0xf59cc4 j___ZN5boost9function1IvN3RBX16UserInputService14SwipeDirectionEE5clearEv")
}

#[doc(alias = "boost::function2<void,float,float>::assign_to_own(boost::function2<void,float,float> const&)")]
// 0xf59e24 — j___ZN5boost9function2IvffE13assign_to_ownERKS1_
pub fn stub_0xf59e24() -> ! {
    todo!("0xf59e24 j___ZN5boost9function2IvffE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::function2<void,float,float>::clear(void)")]
// 0xf59e34 — j___ZN5boost9function2IvffE5clearEv
pub fn stub_0xf59e34() -> ! {
    todo!("0xf59e34 j___ZN5boost9function2IvffE5clearEv")
}

#[doc(alias = "RBX::UserInputService * RBX::ServiceProvider::create<RBX::UserInputService>(void)const")]
// 0xf59ea4 — j___ZNK3RBX15ServiceProvider6createINS_16UserInputServiceEEEPT_v
pub fn stub_0xf59ea4() -> ! {
    todo!("0xf59ea4 j___ZNK3RBX15ServiceProvider6createINS_16UserInputServiceEEEPT_v")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::UserInputServiceJob,RBX::UserInputServiceJob>(rbx_core::SharedPtr<RBX::UserInputServiceJob> const*,RBX::UserInputServiceJob *)const")]
// 0xf59ed4 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_19UserInputServiceJobES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_0xf59ed4() -> ! {
    todo!("0xf59ed4 j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_19UserInputServiceJobES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::function1<void,RBX::InputObject>::operator()(RBX::InputObject)const")]
// 0xf5a154 — j___ZNK5boost9function1IvN3RBX11InputObjectEEclES2_
pub fn stub_0xf5a154() -> ! {
    todo!("0xf5a154 j___ZNK5boost9function1IvN3RBX11InputObjectEEclES2_")
}

#[doc(alias = "boost::function1<void,RBX::UserInputService::SwipeDirection>::operator()(RBX::UserInputService::SwipeDirection)const")]
// 0xf5a164 — j___ZNK5boost9function1IvN3RBX16UserInputService14SwipeDirectionEEclES3_
pub fn stub_0xf5a164() -> ! {
    todo!("0xf5a164 j___ZNK5boost9function1IvN3RBX16UserInputService14SwipeDirectionEEclES3_")
}

#[doc(alias = "boost::function2<void,float,float>::operator()(float,float)const")]
// 0xf5a174 — j___ZNK5boost9function2IvffEclEff
pub fn stub_0xf5a174() -> ! {
    todo!("0xf5a174 j___ZNK5boost9function2IvffEclEff")
}

#[doc(alias = "std::_Vector_base<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::_M_allocate(unsigned long)")]
// 0xf5a1b4 — j___ZNSt12_Vector_baseIN3RBX16UserInputService14SwipeDirectionESaIS2_EE11_M_allocateEm
pub fn stub_0xf5a1b4() -> ! {
    todo!("0xf5a1b4 j___ZNSt12_Vector_baseIN3RBX16UserInputService14SwipeDirectionESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::UserInputService::SwipeDirection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::UserInputService::SwipeDirection *,RBX::UserInputService::SwipeDirection *>(RBX::UserInputService::SwipeDirection *,RBX::UserInputService::SwipeDirection *,RBX::UserInputService::SwipeDirection *)")]
// 0xf5a1c4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16UserInputService14SwipeDirectionES6_EET0_T_S8_S7_
pub fn stub_0xf5a1c4() -> ! {
    todo!("0xf5a1c4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16UserInputService14SwipeDirectionES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::UserInputService::SwipeDirection,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::operator[](RBX::Name const* const&)")]
// 0xf5a1d4 — j___ZNSt3mapIPKN3RBX4NameENS0_16UserInputService14SwipeDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf5a1d4() -> ! {
    todo!("0xf5a1d4 j___ZNSt3mapIPKN3RBX4NameENS0_16UserInputService14SwipeDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::UserInputService::SwipeDirection*,std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>>,RBX::UserInputService::SwipeDirection const&)")]
// 0xf5a254 — j___ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf5a254() -> ! {
    todo!("0xf5a254 j___ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::UserInputService::SwipeDirection*,std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>>,unsigned long,RBX::UserInputService::SwipeDirection const&)")]
// 0xf5a264 — j___ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf5a264() -> ! {
    todo!("0xf5a264 j___ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::resize(unsigned long,RBX::UserInputService::SwipeDirection)")]
// 0xf5a274 — j___ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE6resizeEmS2_
pub fn stub_0xf5a274() -> ! {
    todo!("0xf5a274 j___ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::push_back(RBX::UserInputService::SwipeDirection const&)")]
// 0xf5a284 — j___ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE9push_backERKS2_
pub fn stub_0xf5a284() -> ! {
    todo!("0xf5a284 j___ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection> const&)")]
// 0xf5a2a4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf5a2a4() -> ! {
    todo!("0xf5a2a4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection> const&)")]
// 0xf5a2b4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf5a2b4() -> ! {
    todo!("0xf5a2b4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>> *)")]
// 0xf5a2c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0xf5a2c4() -> ! {
    todo!("0xf5a2c4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection> const&)")]
// 0xf5a2d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf5a2d4() -> ! {
    todo!("0xf5a2d4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWService> RBX::shared_from<RBX::FWService>(RBX::FWService*)")]
// 0xf5a2f4 — j___ZN3RBX11shared_fromINS_9FWServiceEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_0xf5a2f4() -> ! {
    todo!("0xf5a2f4 j___ZN3RBX11shared_fromINS_9FWServiceEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::FWBase::FWBase(RBX::FWBase const&)")]
// 0xf5a304 — j___ZN3RBX6FWBaseC2ERKS0_
pub fn stub_0xf5a304() -> ! {
    todo!("0xf5a304 j___ZN3RBX6FWBaseC2ERKS0_")
}

#[doc(alias = "RBX::FWBase::~FWBase()")]
// 0xf5a314 — j___ZN3RBX6FWBaseD2Ev
pub fn stub_0xf5a314() -> ! {
    todo!("0xf5a314 j___ZN3RBX6FWBaseD2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWBase>::shared_ptr<RBX::FWBase>(RBX::FWBase *)")]
// 0xf5a324 — j___ZN5boost10shared_ptrIN3RBX6FWBaseEEC2IS2_EEPT_
pub fn stub_0xf5a324() -> ! {
    todo!("0xf5a324 j___ZN5boost10shared_ptrIN3RBX6FWBaseEEC2IS2_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWBase>::operator=(rbx_core::SharedPtr<RBX::FWBase> const&)")]
// 0xf5a334 — j___ZN5boost10shared_ptrIN3RBX6FWBaseEEaSERKS3_
pub fn stub_0xf5a334() -> ! {
    todo!("0xf5a334 j___ZN5boost10shared_ptrIN3RBX6FWBaseEEaSERKS3_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::FWBase>::_internal_accept_owner<RBX::FWBase,RBX::FWBase>(rbx_core::SharedPtr<RBX::FWBase> const*,RBX::FWBase *)const")]
// 0xf5a3a4 — j___ZNK5boost23enable_shared_from_thisIN3RBX6FWBaseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_0xf5a3a4() -> ! {
    todo!("0xf5a3a4 j___ZNK5boost23enable_shared_from_thisIN3RBX6FWBaseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "void RBX::MarketplaceService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0xf5a624 — j___ZN3RBX18MarketplaceService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
pub fn stub_0xf5a624() -> ! {
    todo!("0xf5a624 j___ZN3RBX18MarketplaceService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")
}

#[doc(alias = "RBX::MarketplaceService::~MarketplaceService()")]
// 0xf5a634 — j___ZN3RBX18MarketplaceServiceD2Ev
pub fn stub_0xf5a634() -> ! {
    todo!("0xf5a634 j___ZN3RBX18MarketplaceServiceD2Ev")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::MarketplaceService::CurrencyType>(RBX::MarketplaceService::CurrencyType const&)")]
// 0xf5a674 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_18MarketplaceService12CurrencyTypeEEERS3_RKT_
pub fn stub_0xf5a674() -> ! {
    todo!("0xf5a674 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_18MarketplaceService12CurrencyTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,int,int)>::~remote_signal()")]
// 0xf5a6e4 — j___ZN3rbx13remote_signalIFvSsiiEED2Ev
pub fn stub_0xf5a6e4() -> ! {
    todo!("0xf5a6e4 j___ZN3rbx13remote_signalIFvSsiiEED2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,bool)>::remote_signal(void)")]
// 0xf5a6f4 — j___ZN3rbx13remote_signalIFviibEEC2Ev
pub fn stub_0xf5a6f4() -> ! {
    todo!("0xf5a6f4 j___ZN3rbx13remote_signalIFviibEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(int,int,bool)>::~remote_signal()")]
// 0xf5a704 — j___ZN3rbx13remote_signalIFviibEED2Ev
pub fn stub_0xf5a704() -> ! {
    todo!("0xf5a704 j___ZN3rbx13remote_signalIFviibEED2Ev")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MarketplaceService::CurrencyType>::singleton(void)")]
// 0xf5a714 — j___ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE9singletonEv
pub fn stub_0xf5a714() -> ! {
    todo!("0xf5a714 j___ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE9singletonEv")
}
