//! core shard GC — 100 core stubs EA-sorted, 0xf46b34..0xf482e4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf46b14).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf46b14.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::disconnectAll(void)")]
// 0xf46b34 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE13disconnectAllEv
pub fn stub_f46b34() -> ! {
    todo!("0xf46b34 j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::safe_static_do_get_mutex(void)")]
// 0xf46b44 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE24safe_static_do_get_mutexEv
pub fn stub_f46b44() -> ! {
    todo!("0xf46b44 j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot> &)")]
// 0xf46b54 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// was: rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot> &)
pub fn stub_f46b54() -> ! {
    todo!("0xf46b54 j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot::safe_static_do_get_mutex(void)")]
// 0xf46b64 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f46b64() -> ! {
    todo!("0xf46b64 j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::insert(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot *)")]
// 0xf46b74 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE6insertEPNS7_4slotE
pub fn stub_f46b74() -> ! {
    todo!("0xf46b74 j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE6insertEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::remove(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot *)")]
// 0xf46b84 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE6removeEPNS7_4slotE
pub fn stub_f46b84() -> ! {
    todo!("0xf46b84 j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE6removeEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::SelectionChanged const&>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::SelectionChanged const&>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::arg<1>>> const&)")]
// 0xf46b94 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9SelectionES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_f46b94() -> ! {
    todo!("0xf46b94 j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9SelectionES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::on_error(std::exception &)")]
// 0xf46ba4 — j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE8on_errorERSt9exception
pub fn stub_f46ba4() -> ! {
    todo!("0xf46ba4 j___ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot*)")]
// 0xf46bb4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slotEEaSEPSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot*)
pub fn stub_f46bb4() -> ! {
    todo!("0xf46bb4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slotEEaSEPSA_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot> const&)")]
// 0xf46bc4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slotEEaSERKSB_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::SelectionChanged const&)>::slot> const&)
pub fn stub_f46bc4() -> ! {
    todo!("0xf46bc4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slotEEaSERKSB_")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Selection,RBX::SelectionChanged const&>,boost::_bi::list2<boost::_bi::value<RBX::Selection*>,boost::arg<1>>>::operator()<RBX::SelectionChanged>(RBX::SelectionChanged const&)")]
// 0xf46be4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9SelectionERKNS4_16SelectionChangedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
pub fn stub_f46be4() -> ! {
    todo!("0xf46be4 j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9SelectionERKNS4_16SelectionChangedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_")
}

#[doc(alias = "std::_Vector_base<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::_M_allocate(unsigned long)")]
// 0xf46bf4 — j___ZNSt12_Vector_baseIPN3RBX14ISelectionBaseESaIS2_EE11_M_allocateEm
pub fn stub_f46bf4() -> ! {
    todo!("0xf46bf4 j___ZNSt12_Vector_baseIPN3RBX14ISelectionBaseESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase * const&)")]
// 0xf46c14 — j___ZNSt6vectorIPN3RBX14ISelectionBaseESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f46c14() -> ! {
    todo!("0xf46c14 j___ZNSt6vectorIPN3RBX14ISelectionBaseESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>::push_back(RBX::ISelectionBase * const&)")]
// 0xf46c24 — j___ZNSt6vectorIPN3RBX14ISelectionBaseESaIS2_EE9push_backERKS2_
pub fn stub_f46c24() -> ! {
    todo!("0xf46c24 j___ZNSt6vectorIPN3RBX14ISelectionBaseESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase *>(__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,__gnu_cxx::__normal_iterator<RBX::ISelectionBase **,std::vector<RBX::ISelectionBase *,std::allocator<RBX::ISelectionBase *>>>,RBX::ISelectionBase * const&,std::random_access_iterator_tag)")]
// 0xf46c44 — j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX14ISelectionBaseESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
pub fn stub_f46c44() -> ! {
    todo!("0xf46c44 j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX14ISelectionBaseESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag")
}

#[doc(alias = "RBX::SelectionBox::~SelectionBox()")]
// 0xf46c64 — j___ZN3RBX12SelectionBoxD1Ev
pub fn stub_f46c64() -> ! {
    todo!("0xf46c64 j___ZN3RBX12SelectionBoxD1Ev")
}

#[doc(alias = "RBX::SelectionLasso::~SelectionLasso()")]
// 0xf46db4 — j___ZN3RBX14SelectionLassoD1Ev
pub fn stub_f46db4() -> ! {
    todo!("0xf46db4 j___ZN3RBX14SelectionLassoD1Ev")
}

#[doc(alias = "RBX::SelectionPartLasso::~SelectionPartLasso()")]
// 0xf46dc4 — j___ZN3RBX18SelectionPartLassoD1Ev
pub fn stub_f46dc4() -> ! {
    todo!("0xf46dc4 j___ZN3RBX18SelectionPartLassoD1Ev")
}

#[doc(alias = "RBX::SelectionPointLasso::~SelectionPointLasso()")]
// 0xf46dd4 — j___ZN3RBX19SelectionPointLassoD1Ev
pub fn stub_f46dd4() -> ! {
    todo!("0xf46dd4 j___ZN3RBX19SelectionPointLassoD1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Humanoid>::shared_ptr<RBX::Humanoid>(rbx_core::WeakPtr<RBX::Humanoid> const&,boost::detail::sp_nothrow_tag)")]
// 0xf46e54 — j___ZN5boost10shared_ptrIN3RBX8HumanoidEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::Humanoid>::shared_ptr<RBX::Humanoid>(boost::weak_ptr<RBX::Humanoid> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f46e54() -> ! {
    todo!("0xf46e54 j___ZN5boost10shared_ptrIN3RBX8HumanoidEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::SkateboardPlatform> RBX::shared_from<RBX::SkateboardPlatform>(RBX::SkateboardPlatform*)")]
// 0xf46f04 — j___ZN3RBX11shared_fromINS_18SkateboardPlatformEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::SkateboardPlatform> RBX::shared_from<RBX::SkateboardPlatform>(RBX::SkateboardPlatform*)
pub fn stub_f46f04() -> ! {
    todo!("0xf46f04 j___ZN3RBX11shared_fromINS_18SkateboardPlatformEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::SkateboardController::~SkateboardController()")]
// 0xf46f44 — j___ZN3RBX20SkateboardControllerD1Ev
pub fn stub_f46f44() -> ! {
    todo!("0xf46f44 j___ZN3RBX20SkateboardControllerD1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::SkateboardPlatform>::shared_ptr<RBX::SkateboardPlatform>(rbx_core::WeakPtr<RBX::SkateboardPlatform> const&,boost::detail::sp_nothrow_tag)")]
// 0xf46f64 — j___ZN5boost10shared_ptrIN3RBX18SkateboardPlatformEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::SkateboardPlatform>::shared_ptr<RBX::SkateboardPlatform>(boost::weak_ptr<RBX::SkateboardPlatform> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f46f64() -> ! {
    todo!("0xf46f64 j___ZN5boost10shared_ptrIN3RBX18SkateboardPlatformEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::SkateboardController> RBX::shared_from<RBX::SkateboardController>(RBX::SkateboardController*)")]
// 0xf470d4 — j___ZN3RBX11shared_fromINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::SkateboardController> RBX::shared_from<RBX::SkateboardController>(RBX::SkateboardController*)
pub fn stub_f470d4() -> ! {
    todo!("0xf470d4 j___ZN3RBX11shared_fromINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SkateboardPlatform,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::SkateboardPlatform*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SkateboardPlatform,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::SkateboardPlatform*>,boost::arg<1>>>,RBX::Primitive *)")]
// 0xf471c4 — j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_18SkateboardPlatformEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_
pub fn stub_f471c4() -> ! {
    todo!("0xf471c4 j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_18SkateboardPlatformEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::operator()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)")]
// 0xf47204 — j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX18SkateboardPlatform9MoveStateES4_EEclES4_S4_
pub fn stub_f47204() -> ! {
    todo!("0xf47204 j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX18SkateboardPlatform9MoveStateES4_EEclES4_S4_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::disconnectAll(void)")]
// 0xf47214 — j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE13disconnectAllEv
pub fn stub_f47214() -> ! {
    todo!("0xf47214 j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::safe_static_do_get_mutex(void)")]
// 0xf47224 — j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE24safe_static_do_get_mutexEv
pub fn stub_f47224() -> ! {
    todo!("0xf47224 j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot> &)")]
// 0xf47234 — j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot> &)
pub fn stub_f47234() -> ! {
    todo!("0xf47234 j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4nextERN5boost13intrusive_ptrINS6_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot::safe_static_do_get_mutex(void)")]
// 0xf47244 — j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slot24safe_static_do_get_mutexEv
pub fn stub_f47244() -> ! {
    todo!("0xf47244 j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::insert(rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot *)")]
// 0xf47254 — j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE6insertEPNS6_4slotE
pub fn stub_f47254() -> ! {
    todo!("0xf47254 j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::remove(rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot *)")]
// 0xf47264 — j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE6removeEPNS6_4slotE
pub fn stub_f47264() -> ! {
    todo!("0xf47264 j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::connect<boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>>(boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> const&)")]
// 0xf47274 — j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
pub fn stub_f47274() -> ! {
    todo!("0xf47274 j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::on_error(std::exception &)")]
// 0xf47284 — j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE8on_errorERSt9exception
pub fn stub_f47284() -> ! {
    todo!("0xf47284 j___ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE8on_errorERSt9exception")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot,boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::callable<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>*>(boost::function<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> const&,rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>*)")]
// 0xf472b4 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_
pub fn stub_f472b4() -> ! {
    todo!("0xf472b4 j___ZN3rbx8callableINS_7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES5_EE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::SkateboardController>::operator=(rbx_core::SharedPtr<RBX::SkateboardController> const&)")]
// 0xf472f4 — j___ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEaSERKS3_
// was: boost::shared_ptr<RBX::SkateboardController>::operator=(boost::shared_ptr<RBX::SkateboardController> const&)
pub fn stub_f472f4() -> ! {
    todo!("0xf472f4 j___ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot>::operator=(rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot*)")]
// 0xf47304 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES6_EE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot>::operator=(rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot*)
pub fn stub_f47304() -> ! {
    todo!("0xf47304 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES6_EE4slotEEaSEPS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot> const&)")]
// 0xf47314 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES6_EE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot> const&)
pub fn stub_f47314() -> ! {
    todo!("0xf47314 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES6_EE4slotEEaSERKSA_")
}

#[doc(alias = "boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::assign_to_own(boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState> const&)")]
// 0xf47444 — j___ZN5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_E13assign_to_ownERKS4_
pub fn stub_f47444() -> ! {
    todo!("0xf47444 j___ZN5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_E13assign_to_ownERKS4_")
}

#[doc(alias = "boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::clear(void)")]
// 0xf47454 — j___ZN5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_E5clearEv
pub fn stub_f47454() -> ! {
    todo!("0xf47454 j___ZN5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_E5clearEv")
}

#[doc(alias = "boost::function2<void,RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState>::operator()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)const")]
// 0xf47584 — j___ZNK5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_EclES3_S3_
pub fn stub_f47584() -> ! {
    todo!("0xf47584 j___ZNK5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_EclES3_S3_")
}

#[doc(alias = "std::_Vector_base<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::_M_allocate(unsigned long)")]
// 0xf47594 — j___ZNSt12_Vector_baseIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE11_M_allocateEm
pub fn stub_f47594() -> ! {
    todo!("0xf47594 j___ZNSt12_Vector_baseIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::SkateboardPlatform::MoveState * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SkateboardPlatform::MoveState *,RBX::SkateboardPlatform::MoveState *>(RBX::SkateboardPlatform::MoveState *,RBX::SkateboardPlatform::MoveState *,RBX::SkateboardPlatform::MoveState *)")]
// 0xf475a4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18SkateboardPlatform9MoveStateES6_EET0_T_S8_S7_
pub fn stub_f475a4() -> ! {
    todo!("0xf475a4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18SkateboardPlatform9MoveStateES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::SkateboardPlatform::MoveState,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::operator[](RBX::Name const* const&)")]
// 0xf475b4 — j___ZNSt3mapIPKN3RBX4NameENS0_18SkateboardPlatform9MoveStateESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f475b4() -> ! {
    todo!("0xf475b4 j___ZNSt3mapIPKN3RBX4NameENS0_18SkateboardPlatform9MoveStateESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SkateboardPlatform::MoveState*,std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>>,RBX::SkateboardPlatform::MoveState const&)")]
// 0xf475c4 — j___ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f475c4() -> ! {
    todo!("0xf475c4 j___ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SkateboardPlatform::MoveState*,std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>>,unsigned long,RBX::SkateboardPlatform::MoveState const&)")]
// 0xf475d4 — j___ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f475d4() -> ! {
    todo!("0xf475d4 j___ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::resize(unsigned long,RBX::SkateboardPlatform::MoveState)")]
// 0xf475e4 — j___ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE6resizeEmS2_
pub fn stub_f475e4() -> ! {
    todo!("0xf475e4 j___ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::SkateboardPlatform::MoveState,std::allocator<RBX::SkateboardPlatform::MoveState>>::push_back(RBX::SkateboardPlatform::MoveState const&)")]
// 0xf475f4 — j___ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE9push_backERKS2_
pub fn stub_f475f4() -> ! {
    todo!("0xf475f4 j___ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState> const&)")]
// 0xf47604 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f47604() -> ! {
    todo!("0xf47604 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState> const&)")]
// 0xf47614 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f47614() -> ! {
    todo!("0xf47614 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState> const&)")]
// 0xf47624 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f47624() -> ! {
    todo!("0xf47624 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "void RBX::SocialService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0xf477f4 — j___ZN3RBX13SocialService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
pub fn stub_f477f4() -> ! {
    todo!("0xf477f4 j___ZN3RBX13SocialService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")
}

#[doc(alias = "void RBX::SocialService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xf47804 — j___ZN3RBX13SocialService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
pub fn stub_f47804() -> ! {
    todo!("0xf47804 j___ZN3RBX13SocialService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")
}

#[doc(alias = "void RBX::SocialService::dispatchRequest<int>(std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0xf47814 — j___ZN3RBX13SocialService15dispatchRequestIiEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
pub fn stub_f47814() -> ! {
    todo!("0xf47814 j___ZN3RBX13SocialService15dispatchRequestIiEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")
}

#[doc(alias = "RBX::SocialService::~SocialService()")]
// 0xf47824 — j___ZN3RBX13SocialServiceD1Ev
pub fn stub_f47824() -> ! {
    todo!("0xf47824 j___ZN3RBX13SocialServiceD1Ev")
}

#[doc(alias = "std::_Vector_base<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_allocate(unsigned long)")]
// 0xf47844 — j___ZNSt12_Vector_baseIN3RBX13SocialService9StuffTypeESaIS2_EE11_M_allocateEm
pub fn stub_f47844() -> ! {
    todo!("0xf47844 j___ZNSt12_Vector_baseIN3RBX13SocialService9StuffTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::SocialService::StuffType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SocialService::StuffType *,RBX::SocialService::StuffType *>(RBX::SocialService::StuffType *,RBX::SocialService::StuffType *,RBX::SocialService::StuffType *)")]
// 0xf47854 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13SocialService9StuffTypeES6_EET0_T_S8_S7_
pub fn stub_f47854() -> ! {
    todo!("0xf47854 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13SocialService9StuffTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::SocialService::StuffType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::operator[](RBX::Name const* const&)")]
// 0xf47864 — j___ZNSt3mapIPKN3RBX4NameENS0_13SocialService9StuffTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f47864() -> ! {
    todo!("0xf47864 j___ZNSt3mapIPKN3RBX4NameENS0_13SocialService9StuffTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SocialService::StuffType*,std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>>,RBX::SocialService::StuffType const&)")]
// 0xf47874 — j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f47874() -> ! {
    todo!("0xf47874 j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SocialService::StuffType*,std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>>,unsigned long,RBX::SocialService::StuffType const&)")]
// 0xf47884 — j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f47884() -> ! {
    todo!("0xf47884 j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::resize(unsigned long,RBX::SocialService::StuffType)")]
// 0xf47894 — j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE6resizeEmS2_
pub fn stub_f47894() -> ! {
    todo!("0xf47894 j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::push_back(RBX::SocialService::StuffType const&)")]
// 0xf478a4 — j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE9push_backERKS2_
pub fn stub_f478a4() -> ! {
    todo!("0xf478a4 j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)")]
// 0xf478b4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f478b4() -> ! {
    todo!("0xf478b4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)")]
// 0xf478c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f478c4() -> ! {
    todo!("0xf478c4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)")]
// 0xf478d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f478d4() -> ! {
    todo!("0xf478d4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "RBX::Sparkles::~Sparkles()")]
// 0xf47914 — j___ZN3RBX8SparklesD0Ev
pub fn stub_f47914() -> ! {
    todo!("0xf47914 j___ZN3RBX8SparklesD0Ev")
}

#[doc(alias = "RBX::Sparkles::~Sparkles()")]
// 0xf47924 — j___ZN3RBX8SparklesD2Ev
pub fn stub_f47924() -> ! {
    todo!("0xf47924 j___ZN3RBX8SparklesD2Ev")
}

#[doc(alias = "RBX::SpawnerService * RBX::ServiceProvider::create<RBX::SpawnerService>(void)const")]
// 0xf47a84 — j___ZNK3RBX15ServiceProvider6createINS_14SpawnerServiceEEEPT_v
pub fn stub_f47a84() -> ! {
    todo!("0xf47a84 j___ZNK3RBX15ServiceProvider6createINS_14SpawnerServiceEEEPT_v")
}

#[doc(alias = "std::_Vector_base<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_allocate(unsigned long)")]
// 0xf47ac4 — j___ZNSt12_Vector_baseIPN3RBX13SpawnLocationESaIS2_EE11_M_allocateEm
pub fn stub_f47ac4() -> ! {
    todo!("0xf47ac4 j___ZNSt12_Vector_baseIPN3RBX13SpawnLocationESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::list<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::remove(RBX::SpawnLocation * const&)")]
// 0xf47ad4 — j___ZNSt4listIPN3RBX13SpawnLocationESaIS2_EE6removeERKS2_
pub fn stub_f47ad4() -> ! {
    todo!("0xf47ad4 j___ZNSt4listIPN3RBX13SpawnLocationESaIS2_EE6removeERKS2_")
}

#[doc(alias = "std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpawnLocation **,std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>>,RBX::SpawnLocation * const&)")]
// 0xf47ae4 — j___ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f47ae4() -> ! {
    todo!("0xf47ae4 j___ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::push_back(RBX::SpawnLocation * const&)")]
// 0xf47af4 — j___ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE9push_backERKS2_
pub fn stub_f47af4() -> ! {
    todo!("0xf47af4 j___ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_allocate(unsigned long)")]
// 0xf47c74 — j___ZNSt12_Vector_baseIN3RBX12SpecialShape8MeshTypeESaIS2_EE11_M_allocateEm
pub fn stub_f47c74() -> ! {
    todo!("0xf47c74 j___ZNSt12_Vector_baseIN3RBX12SpecialShape8MeshTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::SpecialShape::MeshType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *>(RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *)")]
// 0xf47c84 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12SpecialShape8MeshTypeES6_EET0_T_S8_S7_
pub fn stub_f47c84() -> ! {
    todo!("0xf47c84 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12SpecialShape8MeshTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::SpecialShape::MeshType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::operator[](RBX::Name const* const&)")]
// 0xf47c94 — j___ZNSt3mapIPKN3RBX4NameENS0_12SpecialShape8MeshTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f47c94() -> ! {
    todo!("0xf47c94 j___ZNSt3mapIPKN3RBX4NameENS0_12SpecialShape8MeshTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpecialShape::MeshType*,std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>>,RBX::SpecialShape::MeshType const&)")]
// 0xf47ca4 — j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f47ca4() -> ! {
    todo!("0xf47ca4 j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpecialShape::MeshType*,std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>>,unsigned long,RBX::SpecialShape::MeshType const&)")]
// 0xf47cb4 — j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f47cb4() -> ! {
    todo!("0xf47cb4 j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::resize(unsigned long,RBX::SpecialShape::MeshType)")]
// 0xf47cc4 — j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE6resizeEmS2_
pub fn stub_f47cc4() -> ! {
    todo!("0xf47cc4 j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::push_back(RBX::SpecialShape::MeshType const&)")]
// 0xf47cd4 — j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE9push_backERKS2_
pub fn stub_f47cd4() -> ! {
    todo!("0xf47cd4 j___ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)")]
// 0xf47ce4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f47ce4() -> ! {
    todo!("0xf47ce4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)")]
// 0xf47cf4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f47cf4() -> ! {
    todo!("0xf47cf4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)")]
// 0xf47d04 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f47d04() -> ! {
    todo!("0xf47d04 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "RBX::Stats::StatsService::~StatsService()")]
// 0xf47f24 — j___ZN3RBX5Stats12StatsServiceD2Ev
pub fn stub_f47f24() -> ! {
    todo!("0xf47f24 j___ZN3RBX5Stats12StatsServiceD2Ev")
}

#[doc(alias = "RBX::Stats::JobStepWindowWriter::operator()(double)")]
// 0xf47f34 — j___ZN3RBX5Stats19JobStepWindowWriterclEd
pub fn stub_f47f34() -> ! {
    todo!("0xf47f34 j___ZN3RBX5Stats19JobStepWindowWriterclEd")
}

#[doc(alias = "double const& rbx::any_cast<double const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf47ff4 — j___ZN3rbx8any_castIRKdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f47ff4() -> ! {
    todo!("0xf47ff4 j___ZN3rbx8any_castIRKdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
// 0xf48074 — j___ZN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEC2IS5_EEPT_
// was: boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> *)
pub fn stub_f48074() -> ! {
    todo!("0xf48074 j___ZN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEC2IS5_EEPT_")
}

#[doc(alias = "boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::list4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)")]
// 0xf480d4 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_
// was: boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::list4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)
pub fn stub_f480d4() -> ! {
    todo!("0xf480d4 j___ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::operator()<boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&> &,int)")]
// 0xf480e4 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEclINS_4_mfi3mf3IvS5_NSA_IKNS3_13TaskScheduler3JobEEESG_RbEENS0_5list1IRSR_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::operator()<boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &> &,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job const>&> &,int)
pub fn stub_f480e4() -> ! {
    todo!("0xf480e4 j___ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEclINS_4_mfi3mf3IvS5_NSA_IKNS3_13TaskScheduler3JobEEESG_RbEENS0_5list1IRSR_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::storage4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)")]
// 0xf48124 — j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_
// was: boost::_bi::storage4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::storage4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)
pub fn stub_f48124() -> ! {
    todo!("0xf48124 j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list_av_4<RBX::Stats::StatsService*,boost::arg<1>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>>::type> boost::bind<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &,RBX::Stats::StatsService*,boost::arg<1>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>>(void (RBX::Stats::StatsService::*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &),RBX::Stats::StatsService*,boost::arg<1>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>)")]
// 0xf48134 — j___ZN5boost4bindIvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS1_13TaskScheduler3JobEEENS4_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbPS3_NS_3argILi1EEESE_NS_17reference_wrapperIbEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISN_T0_T1_T2_T3_EENSL_9list_av_4IT4_T5_T6_T7_E4typeEEEMSQ_FSN_SR_SS_ST_ESW_SX_SY_SZ_
// was: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list_av_4<RBX::Stats::StatsService*,boost::arg<1>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>>::type> boost::bind<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &,RBX::Stats::StatsService*,boost::arg<1>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>>(void (RBX::Stats::StatsService::*)(boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &),RBX::Stats::StatsService*,boost::arg<1>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>)
pub fn stub_f48134() -> ! {
    todo!("0xf48134 j___ZN5boost4bindIvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS1_13TaskScheduler3JobEEENS4_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbPS3_NS_3argILi1EEESE_NS_17reference_wrapperIbEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISN_T0_T1_T2_T3_EENSL_9list_av_4IT4_T5_T6_T7_E4typeEEEMSQ_FSN_SR_SS_ST_ESW_SX_SY_SZ_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
// 0xf481d4 — j___ZN5boost6detail12shared_countC2ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEPT_
pub fn stub_f481d4() -> ! {
    todo!("0xf481d4 j___ZN5boost6detail12shared_countC2ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEPT_")
}

#[doc(alias = "boost::function0<float>::clear(void)")]
// 0xf48224 — j___ZN5boost9function0IfE5clearEv
pub fn stub_f48224() -> ! {
    todo!("0xf48224 j___ZN5boost9function0IfE5clearEv")
}

#[doc(alias = "boost::function0<unsigned long>::clear(void)")]
// 0xf48234 — j___ZN5boost9function0ImE5clearEv
pub fn stub_f48234() -> ! {
    todo!("0xf48234 j___ZN5boost9function0ImE5clearEv")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// 0xf48284 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
pub fn stub_f48284() -> ! {
    todo!("0xf48284 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0xf48294 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISE_EEPNS1_10ptr_bucketE
pub fn stub_f48294() -> ! {
    todo!("0xf48294 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISE_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)")]
// 0xf482a4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEixERS5_
pub fn stub_f482a4() -> ! {
    todo!("0xf482a4 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEixERS5_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::construct_with_value<boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>>(boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>> const&)")]
// 0xf482b4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE20construct_with_valueINS1_13emplace_args3INS0_21piecewise_construct_tENS_6tuples5tupleISsNSF_9null_typeESH_SH_SH_SH_SH_SH_SH_SH_EENSG_ISH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EEEEEEvRKT_
pub fn stub_f482b4() -> ! {
    todo!("0xf482b4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE20construct_with_valueINS1_13emplace_args3INS0_21piecewise_construct_tENS_6tuples5tupleISsNSF_9null_typeESH_SH_SH_SH_SH_SH_SH_SH_EENSG_ISH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EEEEEEvRKT_")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::construct(void)")]
// 0xf482c4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE9constructEv
pub fn stub_f482c4() -> ! {
    todo!("0xf482c4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::~node_constructor()")]
// 0xf482d4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEED2Ev
pub fn stub_f482d4() -> ! {
    todo!("0xf482d4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEED2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0xf482e4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_f482e4() -> ! {
    todo!("0xf482e4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

