//! core shard ED — 100 core stubs EA-sorted, lowest uncovered 0x8b2048..0x8c2038 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after EC 0x8b2048).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::IStepped::~IStepped()")]
// 0x8b2048 — __ZN3RBX8ISteppedD2Ev
pub fn stub_8b2048() -> ! {
    todo!("0x8b2048 __ZN3RBX8ISteppedD2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::UserInputServiceJob>::operator=(rbx_core::SharedPtr<RBX::UserInputServiceJob> const&)")]
// 0x8b212c — __ZN5boost10shared_ptrIN3RBX19UserInputServiceJobEEaSERKS3_
// was: boost::shared_ptr<RBX::UserInputServiceJob>::operator=(boost::shared_ptr<RBX::UserInputServiceJob> const&)
pub fn stub_8b212c() -> ! {
    todo!("0x8b212c __ZN5boost10shared_ptrIN3RBX19UserInputServiceJobEEaSERKS3_")
}

#[doc(alias = "RBX::RbxRay::~RbxRay()")]
// 0x8b2430 — __ZN3RBX6RbxRayD1Ev
pub fn stub_8b2430() -> ! {
    todo!("0x8b2430 __ZN3RBX6RbxRayD1Ev")
}

#[doc(alias = "RBX::UserInputService::~UserInputService()")]
// 0x8b2e18 — __ZN3RBX16UserInputServiceD1Ev
pub fn stub_8b2e18() -> ! {
    todo!("0x8b2e18 __ZN3RBX16UserInputServiceD1Ev")
}

#[doc(alias = "RBX::UserInputService::~UserInputService()")]
// 0x8b2e1c — __ZN3RBX16UserInputServiceD0Ev
pub fn stub_8b2e1c() -> ! {
    todo!("0x8b2e1c __ZN3RBX16UserInputServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::UserInputService::~UserInputService()")]
// 0x8b2ed0 — __ZThn32_N3RBX16UserInputServiceD1Ev
// was: non-virtual thunk toRBX::UserInputService::~UserInputService()
pub fn stub_8b2ed0() -> ! {
    todo!("0x8b2ed0 __ZThn32_N3RBX16UserInputServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::UserInputService::~UserInputService()")]
// 0x8b2ed8 — __ZThn32_N3RBX16UserInputServiceD0Ev
// was: non-virtual thunk toRBX::UserInputService::~UserInputService()
pub fn stub_8b2ed8() -> ! {
    todo!("0x8b2ed8 __ZThn32_N3RBX16UserInputServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::UserInputService::~UserInputService()")]
// 0x8b2ef0 — __ZThn36_N3RBX16UserInputServiceD1Ev
// was: non-virtual thunk toRBX::UserInputService::~UserInputService()
pub fn stub_8b2ef0() -> ! {
    todo!("0x8b2ef0 __ZThn36_N3RBX16UserInputServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::UserInputService::~UserInputService()")]
// 0x8b2ef8 — __ZThn36_N3RBX16UserInputServiceD0Ev
// was: non-virtual thunk toRBX::UserInputService::~UserInputService()
pub fn stub_8b2ef8() -> ! {
    todo!("0x8b2ef8 __ZThn36_N3RBX16UserInputServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::UserInputService::~UserInputService()")]
// 0x8b2f04 — __ZThn96_N3RBX16UserInputServiceD1Ev
// was: non-virtual thunk toRBX::UserInputService::~UserInputService()
pub fn stub_8b2f04() -> ! {
    todo!("0x8b2f04 __ZThn96_N3RBX16UserInputServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::UserInputService::~UserInputService()")]
// 0x8b2f0c — __ZThn96_N3RBX16UserInputServiceD0Ev
// was: non-virtual thunk toRBX::UserInputService::~UserInputService()
pub fn stub_8b2f0c() -> ! {
    todo!("0x8b2f0c __ZThn96_N3RBX16UserInputServiceD0Ev")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::UserInputService::SwipeDirection>(RBX::UserInputService::SwipeDirection const&)")]
// 0x8b3330 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16UserInputService14SwipeDirectionEEERS3_RKT_
pub fn stub_8b3330() -> ! {
    todo!("0x8b3330 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16UserInputService14SwipeDirectionEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UserInputService::SwipeDirection>::singleton(void)")]
// 0x8b3380 — __ZN3rbx14implementation12typed_holderIN3RBX16UserInputService14SwipeDirectionEE9singletonEv
pub fn stub_8b3380() -> ! {
    todo!("0x8b3380 __ZN3rbx14implementation12typed_holderIN3RBX16UserInputService14SwipeDirectionEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UserInputService::SwipeDirection>::construct_func(char const*,char *)")]
// 0x8b33ec — __ZN3rbx14implementation12typed_holderIN3RBX16UserInputService14SwipeDirectionEE14construct_funcEPKcPc
pub fn stub_8b33ec() -> ! {
    todo!("0x8b33ec __ZN3rbx14implementation12typed_holderIN3RBX16UserInputService14SwipeDirectionEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UserInputService::SwipeDirection>::destruct_func(char *)")]
// 0x8b33f8 — __ZN3rbx14implementation12typed_holderIN3RBX16UserInputService14SwipeDirectionEE13destruct_funcEPc
pub fn stub_8b33f8() -> ! {
    todo!("0x8b33f8 __ZN3rbx14implementation12typed_holderIN3RBX16UserInputService14SwipeDirectionEE13destruct_funcEPc")
}

#[doc(alias = "RBX::UserInputService::SwipeDirection const& rbx::any_cast<RBX::UserInputService::SwipeDirection const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x8b34c8 — __ZN3rbx8any_castIRKN3RBX16UserInputService14SwipeDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_8b34c8() -> ! {
    todo!("0x8b34c8 __ZN3rbx8any_castIRKN3RBX16UserInputService14SwipeDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::UserInputServiceJob>::shared_ptr<RBX::UserInputServiceJob>(RBX::UserInputServiceJob *)")]
// 0x8bb998 — __ZN5boost10shared_ptrIN3RBX19UserInputServiceJobEEC2IS2_EEPT_
// was: boost::shared_ptr<RBX::UserInputServiceJob>::shared_ptr<RBX::UserInputServiceJob>(RBX::UserInputServiceJob *)
pub fn stub_8bb998() -> ! {
    todo!("0x8bb998 __ZN5boost10shared_ptrIN3RBX19UserInputServiceJobEEC2IS2_EEPT_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::UserInputServiceJob,RBX::UserInputServiceJob>(rbx_core::SharedPtr<RBX::UserInputServiceJob> const*,RBX::UserInputServiceJob *)const")]
// 0x8bba80 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_19UserInputServiceJobES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::UserInputServiceJob,RBX::UserInputServiceJob>(boost::shared_ptr<RBX::UserInputServiceJob> const*,RBX::UserInputServiceJob *)const
pub fn stub_8bba80() -> ! {
    todo!("0x8bba80 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_19UserInputServiceJobES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::UserInputServiceJob>(RBX::UserInputServiceJob *)")]
// 0x8bbb64 — __ZN5boost6detail12shared_countC2IN3RBX19UserInputServiceJobEEEPT_
pub fn stub_8bbb64() -> ! {
    todo!("0x8bbb64 __ZN5boost6detail12shared_countC2IN3RBX19UserInputServiceJobEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UserInputServiceJob>::~sp_counted_impl_p()")]
// 0x8bbc5c — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UserInputServiceJobEED1Ev
pub fn stub_8bbc5c() -> ! {
    todo!("0x8bbc5c __ZN5boost6detail17sp_counted_impl_pIN3RBX19UserInputServiceJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UserInputServiceJob>::~sp_counted_impl_p()")]
// 0x8bbc60 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UserInputServiceJobEED0Ev
pub fn stub_8bbc60() -> ! {
    todo!("0x8bbc60 __ZN5boost6detail17sp_counted_impl_pIN3RBX19UserInputServiceJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UserInputServiceJob>::dispose(void)")]
// 0x8bbc64 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UserInputServiceJobEE7disposeEv
pub fn stub_8bbc64() -> ! {
    todo!("0x8bbc64 __ZN5boost6detail17sp_counted_impl_pIN3RBX19UserInputServiceJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UserInputServiceJob>::get_deleter(std::type_info const&)")]
// 0x8bbc74 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UserInputServiceJobEE11get_deleterERKSt9type_info
pub fn stub_8bbc74() -> ! {
    todo!("0x8bbc74 __ZN5boost6detail17sp_counted_impl_pIN3RBX19UserInputServiceJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::UserInputServiceJob>::get_untyped_deleter(void)")]
// 0x8bbc78 — __ZN5boost6detail17sp_counted_impl_pIN3RBX19UserInputServiceJobEE19get_untyped_deleterEv
pub fn stub_8bbc78() -> ! {
    todo!("0x8bbc78 __ZN5boost6detail17sp_counted_impl_pIN3RBX19UserInputServiceJobEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::disconnectAll(void)")]
// 0x8bc3d8 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE13disconnectAllEv
pub fn stub_8bc3d8() -> ! {
    todo!("0x8bc3d8 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE13disconnectAllEv")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::InputObject)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::InputObject)>::slot> const&)")]
// 0x8bc550 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slotEEaSERKS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::InputObject)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::InputObject)>::slot> const&)
pub fn stub_8bc550() -> ! {
    todo!("0x8bc550 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::safe_static_init_mutex(void)")]
// 0x8bc574 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE22safe_static_init_mutexEv
pub fn stub_8bc574() -> ! {
    todo!("0x8bc574 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::safe_static_do_get_mutex(void)")]
// 0x8bc578 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE24safe_static_do_get_mutexEv
pub fn stub_8bc578() -> ! {
    todo!("0x8bc578 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::InputObject)>::operator()(RBX::InputObject)")]
// 0x8bc674 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX11InputObjectEEEclES3_
pub fn stub_8bc674() -> ! {
    todo!("0x8bc674 __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX11InputObjectEEEclES3_")
}

#[doc(alias = "RBX::InputObject const& rbx::any_cast<RBX::InputObject const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x8bc7e0 — __ZN3rbx8any_castIRKN3RBX11InputObjectENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_8bc7e0() -> ! {
    todo!("0x8bc7e0 __ZN3rbx8any_castIRKN3RBX11InputObjectENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::InputObject)>::slot> &)")]
// 0x8bc8d0 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// was: rbx::signals::signal<void ()(RBX::InputObject)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::InputObject)>::slot> &)
pub fn stub_8bc8d0() -> ! {
    todo!("0x8bc8d0 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::on_error(std::exception &)")]
// 0x8bca30 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE8on_errorERSt9exception
pub fn stub_8bca30() -> ! {
    todo!("0x8bca30 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE8on_errorERSt9exception")
}

#[doc(alias = "boost::function1<void,RBX::InputObject>::clear(void)")]
// 0x8bccb8 — __ZN5boost9function1IvN3RBX11InputObjectEE5clearEv
pub fn stub_8bccb8() -> ! {
    todo!("0x8bccb8 __ZN5boost9function1IvN3RBX11InputObjectEE5clearEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject>::singleton(void)")]
// 0x8bcce8 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObjectEE9singletonEv
pub fn stub_8bcce8() -> ! {
    todo!("0x8bcce8 __ZN3rbx14implementation12typed_holderIN3RBX11InputObjectEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject>::destruct_func(char *)")]
// 0x8bcd58 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObjectEE13destruct_funcEPc
pub fn stub_8bcd58() -> ! {
    todo!("0x8bcd58 __ZN3rbx14implementation12typed_holderIN3RBX11InputObjectEE13destruct_funcEPc")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::InputObject)>::connect<boost::function<void ()(RBX::InputObject)>>(boost::function<void ()(RBX::InputObject)> const&)")]
// 0x8bd454 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
pub fn stub_8bd454() -> ! {
    todo!("0x8bd454 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0>::operator new(unsigned long)")]
// 0x8bd548 — __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EEnwEm
pub fn stub_8bd548() -> ! {
    todo!("0x8bd548 __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EEnwEm")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::insert(rbx::signals::signal<void ()(RBX::InputObject)>::slot *)")]
// 0x8bd568 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE6insertEPNS5_4slotE
pub fn stub_8bd568() -> ! {
    todo!("0x8bd568 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE6insertEPNS5_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::InputObject)>::slot>::operator=(rbx::signals::signal<void ()(RBX::InputObject)>::slot*)")]
// 0x8bd774 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slotEEaSEPS8_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::InputObject)>::slot>::operator=(rbx::signals::signal<void ()(RBX::InputObject)>::slot*)
pub fn stub_8bd774() -> ! {
    todo!("0x8bd774 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slotEEaSEPS8_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::InputObject)>::slot,boost::function<void ()(RBX::InputObject)>,1,void ()(RBX::InputObject)>::callable<rbx::signals::signal<void ()(RBX::InputObject)>*>(boost::function<void ()(RBX::InputObject)> const&,rbx::signals::signal<void ()(RBX::InputObject)>*)")]
// 0x8bd798 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX11InputObjectEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
pub fn stub_8bd798() -> ! {
    todo!("0x8bd798 __ZN3rbx8callableINS_7signals6signalIFvN3RBX11InputObjectEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::callable_slot<boost::function<void ()(RBX::InputObject)>>::~callable_slot()")]
// 0x8bd894 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE13callable_slotIN5boost8functionIS4_EEED1Ev
pub fn stub_8bd894() -> ! {
    todo!("0x8bd894 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE13callable_slotIN5boost8functionIS4_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::callable_slot<boost::function<void ()(RBX::InputObject)>>::~callable_slot()")]
// 0x8bd9a4 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE13callable_slotIN5boost8functionIS4_EEED0Ev
pub fn stub_8bd9a4() -> ! {
    todo!("0x8bd9a4 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE13callable_slotIN5boost8functionIS4_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::slot::disconnect(void)")]
// 0x8bdad4 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slot10disconnectEv
pub fn stub_8bdad4() -> ! {
    todo!("0x8bdad4 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::slot::connected(void)const")]
// 0x8bdbe4 — __ZNK3rbx7signals6signalIFvN3RBX11InputObjectEEE4slot9connectedEv
pub fn stub_8bdbe4() -> ! {
    todo!("0x8bdbe4 __ZNK3rbx7signals6signalIFvN3RBX11InputObjectEEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::InputObject)>::slot,boost::function<void ()(RBX::InputObject)>,1,void ()(RBX::InputObject)>::call(RBX::InputObject)")]
// 0x8bdbf0 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX11InputObjectEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
pub fn stub_8bdbf0() -> ! {
    todo!("0x8bdbf0 __ZN3rbx8callableINS_7signals6signalIFvN3RBX11InputObjectEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::InputObject)>::slot,boost::function<void ()(RBX::InputObject)>,1,void ()(RBX::InputObject)>::call(RBX::InputObject)")]
// 0x8bdc18 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX11InputObjectEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::InputObject)>::slot,boost::function<void ()(RBX::InputObject)>,1,void ()(RBX::InputObject)>::call(RBX::InputObject)
pub fn stub_8bdc18() -> ! {
    todo!("0x8bdc18 __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX11InputObjectEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")
}

#[doc(alias = "boost::function1<void,RBX::InputObject>::operator()(RBX::InputObject)const")]
// 0x8bdc40 — __ZNK5boost9function1IvN3RBX11InputObjectEEclES2_
pub fn stub_8bdc40() -> ! {
    todo!("0x8bdc40 __ZNK5boost9function1IvN3RBX11InputObjectEEclES2_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::remove(rbx::signals::signal<void ()(RBX::InputObject)>::slot *)")]
// 0x8bdd18 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE6removeEPNS5_4slotE
pub fn stub_8bdd18() -> ! {
    todo!("0x8bdd18 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE6removeEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::slot::safe_static_init_mutex(void)")]
// 0x8bde08 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slot22safe_static_init_mutexEv
pub fn stub_8bde08() -> ! {
    todo!("0x8bde08 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::slot::safe_static_do_get_mutex(void)")]
// 0x8bde0c — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slot24safe_static_do_get_mutexEv
pub fn stub_8bde0c() -> ! {
    todo!("0x8bde0c __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::InputObject)>::slot,boost::function<void ()(RBX::InputObject)>,1,void ()(RBX::InputObject)>::~callable()")]
// 0x8bdf00 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX11InputObjectEEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
pub fn stub_8bdf00() -> ! {
    todo!("0x8bdf00 __ZN3rbx8callableINS_7signals6signalIFvN3RBX11InputObjectEEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::InputObject)>::slot,boost::function<void ()(RBX::InputObject)>,1,void ()(RBX::InputObject)>::~callable()")]
// 0x8be010 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX11InputObjectEEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
pub fn stub_8be010() -> ! {
    todo!("0x8be010 __ZN3rbx8callableINS_7signals6signalIFvN3RBX11InputObjectEEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::slot::~slot()")]
// 0x8be140 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slotD1Ev
pub fn stub_8be140() -> ! {
    todo!("0x8be140 __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::InputObject)>::slot::~slot()")]
// 0x8be16c — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slotD0Ev
pub fn stub_8be16c() -> ! {
    todo!("0x8be16c __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slotD0Ev")
}

#[doc(alias = "boost::function1<void,RBX::InputObject>::assign_to_own(boost::function1<void,RBX::InputObject> const&)")]
// 0x8be240 — __ZN5boost9function1IvN3RBX11InputObjectEE13assign_to_ownERKS3_
pub fn stub_8be240() -> ! {
    todo!("0x8be240 __ZN5boost9function1IvN3RBX11InputObjectEE13assign_to_ownERKS3_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::disconnectAll(void)")]
// 0x8be69c — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE13disconnectAllEv
pub fn stub_8be69c() -> ! {
    todo!("0x8be69c __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE13disconnectAllEv")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot> const&)")]
// 0x8be814 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot> const&)
pub fn stub_8be814() -> ! {
    todo!("0x8be814 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEEaSERKSA_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::safe_static_init_mutex(void)")]
// 0x8be838 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE22safe_static_init_mutexEv
pub fn stub_8be838() -> ! {
    todo!("0x8be838 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::safe_static_do_get_mutex(void)")]
// 0x8be83c — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE24safe_static_do_get_mutexEv
pub fn stub_8be83c() -> ! {
    todo!("0x8be83c __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::UserInputService::SwipeDirection)>::operator()(RBX::UserInputService::SwipeDirection)")]
// 0x8be934 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX16UserInputService14SwipeDirectionEEEclES4_
pub fn stub_8be934() -> ! {
    todo!("0x8be934 __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX16UserInputService14SwipeDirectionEEEclES4_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot> &)")]
// 0x8bea78 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot> &)
pub fn stub_8bea78() -> ! {
    todo!("0x8bea78 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::on_error(std::exception &)")]
// 0x8bebd8 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE8on_errorERSt9exception
pub fn stub_8bebd8() -> ! {
    todo!("0x8bebd8 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE8on_errorERSt9exception")
}

#[doc(alias = "boost::function1<void,RBX::UserInputService::SwipeDirection>::clear(void)")]
// 0x8bee60 — __ZN5boost9function1IvN3RBX16UserInputService14SwipeDirectionEE5clearEv
pub fn stub_8bee60() -> ! {
    todo!("0x8bee60 __ZN5boost9function1IvN3RBX16UserInputService14SwipeDirectionEE5clearEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::connect<boost::function<void ()(RBX::UserInputService::SwipeDirection)>>(boost::function<void ()(RBX::UserInputService::SwipeDirection)> const&)")]
// 0x8bf590 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
pub fn stub_8bf590() -> ! {
    todo!("0x8bf590 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::insert(rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot *)")]
// 0x8bf684 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE6insertEPNS6_4slotE
pub fn stub_8bf684() -> ! {
    todo!("0x8bf684 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot*)")]
// 0x8bf890 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot*)
pub fn stub_8bf890() -> ! {
    todo!("0x8bf890 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot,boost::function<void ()(RBX::UserInputService::SwipeDirection)>,1,void ()(RBX::UserInputService::SwipeDirection)>::callable<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>*>(boost::function<void ()(RBX::UserInputService::SwipeDirection)> const&,rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>*)")]
// 0x8bf8b4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
pub fn stub_8bf8b4() -> ! {
    todo!("0x8bf8b4 __ZN3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::callable_slot<boost::function<void ()(RBX::UserInputService::SwipeDirection)>>::~callable_slot()")]
// 0x8bf9b0 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE13callable_slotIN5boost8functionIS5_EEED1Ev
pub fn stub_8bf9b0() -> ! {
    todo!("0x8bf9b0 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE13callable_slotIN5boost8functionIS5_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::callable_slot<boost::function<void ()(RBX::UserInputService::SwipeDirection)>>::~callable_slot()")]
// 0x8bfac0 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE13callable_slotIN5boost8functionIS5_EEED0Ev
pub fn stub_8bfac0() -> ! {
    todo!("0x8bfac0 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE13callable_slotIN5boost8functionIS5_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot::disconnect(void)")]
// 0x8bfbf0 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot10disconnectEv
pub fn stub_8bfbf0() -> ! {
    todo!("0x8bfbf0 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot::connected(void)const")]
// 0x8bfd00 — __ZNK3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot9connectedEv
pub fn stub_8bfd00() -> ! {
    todo!("0x8bfd00 __ZNK3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot,boost::function<void ()(RBX::UserInputService::SwipeDirection)>,1,void ()(RBX::UserInputService::SwipeDirection)>::call(RBX::UserInputService::SwipeDirection)")]
// 0x8bfd0c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
pub fn stub_8bfd0c() -> ! {
    todo!("0x8bfd0c __ZN3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot,boost::function<void ()(RBX::UserInputService::SwipeDirection)>,1,void ()(RBX::UserInputService::SwipeDirection)>::call(RBX::UserInputService::SwipeDirection)")]
// 0x8bfd14 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot,boost::function<void ()(RBX::UserInputService::SwipeDirection)>,1,void ()(RBX::UserInputService::SwipeDirection)>::call(RBX::UserInputService::SwipeDirection)
pub fn stub_8bfd14() -> ! {
    todo!("0x8bfd14 __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_")
}

#[doc(alias = "boost::function1<void,RBX::UserInputService::SwipeDirection>::operator()(RBX::UserInputService::SwipeDirection)const")]
// 0x8bfd1c — __ZNK5boost9function1IvN3RBX16UserInputService14SwipeDirectionEEclES3_
pub fn stub_8bfd1c() -> ! {
    todo!("0x8bfd1c __ZNK5boost9function1IvN3RBX16UserInputService14SwipeDirectionEEclES3_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::remove(rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot *)")]
// 0x8bfde0 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE6removeEPNS6_4slotE
pub fn stub_8bfde0() -> ! {
    todo!("0x8bfde0 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot::safe_static_init_mutex(void)")]
// 0x8bfed0 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot22safe_static_init_mutexEv
pub fn stub_8bfed0() -> ! {
    todo!("0x8bfed0 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot::safe_static_do_get_mutex(void)")]
// 0x8bfed4 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot24safe_static_do_get_mutexEv
pub fn stub_8bfed4() -> ! {
    todo!("0x8bfed4 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot,boost::function<void ()(RBX::UserInputService::SwipeDirection)>,1,void ()(RBX::UserInputService::SwipeDirection)>::~callable()")]
// 0x8bffc4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
pub fn stub_8bffc4() -> ! {
    todo!("0x8bffc4 __ZN3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot,boost::function<void ()(RBX::UserInputService::SwipeDirection)>,1,void ()(RBX::UserInputService::SwipeDirection)>::~callable()")]
// 0x8c00d4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
pub fn stub_8c00d4() -> ! {
    todo!("0x8c00d4 __ZN3rbx8callableINS_7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot::~slot()")]
// 0x8c0204 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotD1Ev
pub fn stub_8c0204() -> ! {
    todo!("0x8c0204 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UserInputService::SwipeDirection)>::slot::~slot()")]
// 0x8c0230 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotD0Ev
pub fn stub_8c0230() -> ! {
    todo!("0x8c0230 __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slotD0Ev")
}

#[doc(alias = "boost::function1<void,RBX::UserInputService::SwipeDirection>::assign_to_own(boost::function1<void,RBX::UserInputService::SwipeDirection> const&)")]
// 0x8c0304 — __ZN5boost9function1IvN3RBX16UserInputService14SwipeDirectionEE13assign_to_ownERKS4_
pub fn stub_8c0304() -> ! {
    todo!("0x8c0304 __ZN5boost9function1IvN3RBX16UserInputService14SwipeDirectionEE13assign_to_ownERKS4_")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::disconnectAll(void)")]
// 0x8c07dc — __ZN3rbx7signals6signalIFvffEE13disconnectAllEv
pub fn stub_8c07dc() -> ! {
    todo!("0x8c07dc __ZN3rbx7signals6signalIFvffEE13disconnectAllEv")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float)>::slot> const&)")]
// 0x8c0954 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(float,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float)>::slot> const&)
pub fn stub_8c0954() -> ! {
    todo!("0x8c0954 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::safe_static_init_mutex(void)")]
// 0x8c0978 — __ZN3rbx7signals6signalIFvffEE22safe_static_init_mutexEv
pub fn stub_8c0978() -> ! {
    todo!("0x8c0978 __ZN3rbx7signals6signalIFvffEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::safe_static_do_get_mutex(void)")]
// 0x8c097c — __ZN3rbx7signals6signalIFvffEE24safe_static_do_get_mutexEv
pub fn stub_8c097c() -> ! {
    todo!("0x8c097c __ZN3rbx7signals6signalIFvffEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(float,float)>::operator()(float,float)")]
// 0x8c0a74 — __ZN3rbx7signals16signal_with_argsILi2EFvffEEclEff
pub fn stub_8c0a74() -> ! {
    todo!("0x8c0a74 __ZN3rbx7signals16signal_with_argsILi2EFvffEEclEff")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float)>::slot> &)")]
// 0x8c0bcc — __ZN3rbx7signals6signalIFvffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(float,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float)>::slot> &)
pub fn stub_8c0bcc() -> ! {
    todo!("0x8c0bcc __ZN3rbx7signals6signalIFvffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::on_error(std::exception &)")]
// 0x8c0d2c — __ZN3rbx7signals6signalIFvffEE8on_errorERSt9exception
pub fn stub_8c0d2c() -> ! {
    todo!("0x8c0d2c __ZN3rbx7signals6signalIFvffEE8on_errorERSt9exception")
}

#[doc(alias = "boost::function2<void,float,float>::clear(void)")]
// 0x8c0fd8 — __ZN5boost9function2IvffE5clearEv
pub fn stub_8c0fd8() -> ! {
    todo!("0x8c0fd8 __ZN5boost9function2IvffE5clearEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float)>::connect<boost::function<void ()(float,float)>>(boost::function<void ()(float,float)> const&)")]
// 0x8c18b4 — __ZN3rbx7signals6signalIFvffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_8c18b4() -> ! {
    todo!("0x8c18b4 __ZN3rbx7signals6signalIFvffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::insert(rbx::signals::signal<void ()(float,float)>::slot *)")]
// 0x8c19a8 — __ZN3rbx7signals6signalIFvffEE6insertEPNS3_4slotE
pub fn stub_8c19a8() -> ! {
    todo!("0x8c19a8 __ZN3rbx7signals6signalIFvffEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float)>::slot*)")]
// 0x8c1bb4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float)>::slot*)
pub fn stub_8c1bb4() -> ! {
    todo!("0x8c1bb4 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::function<void ()(float,float)>,2,void ()(float,float)>::callable<rbx::signals::signal<void ()(float,float)>*>(boost::function<void ()(float,float)> const&,rbx::signals::signal<void ()(float,float)>*)")]
// 0x8c1bd8 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_
pub fn stub_8c1bd8() -> ! {
    todo!("0x8c1bd8 __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::callable_slot<boost::function<void ()(float,float)>>::~callable_slot()")]
// 0x8c1cd4 — __ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_8c1cd4() -> ! {
    todo!("0x8c1cd4 __ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost8functionIS2_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::callable_slot<boost::function<void ()(float,float)>>::~callable_slot()")]
// 0x8c1de4 — __ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_8c1de4() -> ! {
    todo!("0x8c1de4 __ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost8functionIS2_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::disconnect(void)")]
// 0x8c1f14 — __ZN3rbx7signals6signalIFvffEE4slot10disconnectEv
pub fn stub_8c1f14() -> ! {
    todo!("0x8c1f14 __ZN3rbx7signals6signalIFvffEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::connected(void)const")]
// 0x8c2024 — __ZNK3rbx7signals6signalIFvffEE4slot9connectedEv
pub fn stub_8c2024() -> ! {
    todo!("0x8c2024 __ZNK3rbx7signals6signalIFvffEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::function<void ()(float,float)>,2,void ()(float,float)>::call(float,float)")]
// 0x8c2030 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_E4callEff
pub fn stub_8c2030() -> ! {
    todo!("0x8c2030 __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_E4callEff")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::function<void ()(float,float)>,2,void ()(float,float)>::call(float,float)")]
// 0x8c2038 — __ZThn4_N3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_E4callEff
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::function<void ()(float,float)>,2,void ()(float,float)>::call(float,float)
pub fn stub_8c2038() -> ! {
    todo!("0x8c2038 __ZThn4_N3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_E4callEff")
}
