//! core shard GW — 100 core stubs EA-sorted, 0x350ec..0x3df1c (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap (0x350ec..0x3df1c, 17921->18021 covered, 3897 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x350ec — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_0x350ec() -> ! {
    todo!("0x350ec __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::operator()<void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(PlaceLauncher *,std::string,std::string,std::string) &,boost::_bi::list0 &,int)")]
// 0x35200 — __ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EclIPFvS4_SsSsSsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0x35200() -> ! {
    todo!("0x35200 __ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EclIPFvS4_SsSsSsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x35438 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE7managerERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0x35438() -> ! {
    todo!("0x35438 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE7managerERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "RobloxView::completeViewPrep(rbx_core::SharedPtr<RBX::Game>)")]
// 0x37b3c — __ZN10RobloxView16completeViewPrepEN5boost10shared_ptrIN3RBX4GameEEE
// was: RobloxView::completeViewPrep(boost::shared_ptr<RBX::Game>)
pub fn stub_0x37b3c() -> ! {
    todo!("0x37b3c __ZN10RobloxView16completeViewPrepEN5boost10shared_ptrIN3RBX4GameEEE")
}

#[doc(alias = "RobloxView::create_view(rbx_core::SharedPtr<RBX::Game>,unsigned int,unsigned int,std::string,std::string,std::string)")]
// 0x39674 — __ZN10RobloxView11create_viewEN5boost10shared_ptrIN3RBX4GameEEEjjSsSsSs
// was: RobloxView::create_view(boost::shared_ptr<RBX::Game>,unsigned int,unsigned int,std::string,std::string,std::string)
pub fn stub_0x39674() -> ! {
    todo!("0x39674 __ZN10RobloxView11create_viewEN5boost10shared_ptrIN3RBX4GameEEEjjSsSsSs")
}

#[doc(alias = "std::domain_error::~domain_error()")]
// 0x39be0 — __ZNSt12domain_errorD0Ev
pub fn stub_0x39be0() -> ! {
    todo!("0x39be0 __ZNSt12domain_errorD0Ev")
}

#[doc(alias = "std::domain_error::~domain_error()")]
// 0x39bf8 — __ZNSt12domain_errorD2Ev
pub fn stub_0x39bf8() -> ! {
    todo!("0x39bf8 __ZNSt12domain_errorD2Ev")
}

#[doc(alias = "std::invalid_argument::~invalid_argument()")]
// 0x39c00 — __ZNSt16invalid_argumentD1Ev
pub fn stub_0x39c00() -> ! {
    todo!("0x39c00 __ZNSt16invalid_argumentD1Ev")
}

#[doc(alias = "std::length_error::~length_error()")]
// 0x39c08 — __ZNSt12length_errorD0Ev
pub fn stub_0x39c08() -> ! {
    todo!("0x39c08 __ZNSt12length_errorD0Ev")
}

#[doc(alias = "std::out_of_range::~out_of_range()")]
// 0x39c20 — __ZNSt12out_of_rangeD1Ev
pub fn stub_0x39c20() -> ! {
    todo!("0x39c20 __ZNSt12out_of_rangeD1Ev")
}

#[doc(alias = "std::range_error::~range_error()")]
// 0x39c28 — __ZNSt11range_errorD0Ev
pub fn stub_0x39c28() -> ! {
    todo!("0x39c28 __ZNSt11range_errorD0Ev")
}

#[doc(alias = "std::range_error::~range_error()")]
// 0x39c40 — __ZNSt11range_errorD2Ev
pub fn stub_0x39c40() -> ! {
    todo!("0x39c40 __ZNSt11range_errorD2Ev")
}

#[doc(alias = "std::overflow_error::~overflow_error()")]
// 0x39c48 — __ZNSt14overflow_errorD1Ev
pub fn stub_0x39c48() -> ! {
    todo!("0x39c48 __ZNSt14overflow_errorD1Ev")
}

#[doc(alias = "std::underflow_error::~underflow_error()")]
// 0x39c50 — __ZNSt15underflow_errorD0Ev
pub fn stub_0x39c50() -> ! {
    todo!("0x39c50 __ZNSt15underflow_errorD0Ev")
}

#[doc(alias = "std::underflow_error::~underflow_error()")]
// 0x39c68 — __ZNSt15underflow_errorD2Ev
pub fn stub_0x39c68() -> ! {
    todo!("0x39c68 __ZNSt15underflow_errorD2Ev")
}

#[doc(alias = "RBX::TaskScheduler::removeBlocking(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,boost::function<void ()(void)>)")]
// 0x39c6c — __ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEENS1_8functionIFvvEEE
// was: RBX::TaskScheduler::removeBlocking(boost::shared_ptr<RBX::TaskScheduler::Job>,boost::function<void ()(void)>)
pub fn stub_0x39c6c() -> ! {
    todo!("0x39c6c __ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEENS1_8functionIFvvEEE")
}

#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::reset(void)")]
// 0x39d7c — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEE5resetEv
// was: boost::shared_ptr<RobloxView::RenderJob>::reset(void)
pub fn stub_0x39d7c() -> ! {
    todo!("0x39d7c __ZN5boost10shared_ptrIN10RobloxView9RenderJobEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::reset(void)")]
// 0x39e10 — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv
// was: boost::shared_ptr<RobloxView::ViewUpdateJob>::reset(void)
pub fn stub_0x39e10() -> ! {
    todo!("0x39e10 __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::operator=(rbx_core::SharedPtr<RobloxView::ViewUpdateJob>&&)")]
// 0x39ea8 — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_
// was: boost::shared_ptr<RobloxView::ViewUpdateJob>::operator=(boost::shared_ptr<RobloxView::ViewUpdateJob>&&)
pub fn stub_0x39ea8() -> ! {
    todo!("0x39ea8 __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::shared_ptr<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
// 0x39f4c — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_
// was: boost::shared_ptr<RobloxView::ViewUpdateJob>::shared_ptr<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)
pub fn stub_0x39f4c() -> ! {
    todo!("0x39f4c __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::operator=(rbx_core::SharedPtr<RobloxView::RenderJob>&&)")]
// 0x3a030 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEaSEOS3_
// was: boost::shared_ptr<RobloxView::RenderJob>::operator=(boost::shared_ptr<RobloxView::RenderJob>&&)
pub fn stub_0x3a030() -> ! {
    todo!("0x3a030 __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEaSEOS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::shared_ptr<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
// 0x3a0d4 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEC1IS2_EEPT_
// was: boost::shared_ptr<RobloxView::RenderJob>::shared_ptr<RobloxView::RenderJob>(RobloxView::RenderJob *)
pub fn stub_0x3a0d4() -> ! {
    todo!("0x3a0d4 __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEC1IS2_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Game>::operator=(rbx_core::SharedPtr<RBX::Game> const&)")]
// 0x3a1bc — __ZN5boost10shared_ptrIN3RBX4GameEEaSERKS3_
// was: boost::shared_ptr<RBX::Game>::operator=(boost::shared_ptr<RBX::Game> const&)
pub fn stub_0x3a1bc() -> ! {
    todo!("0x3a1bc __ZN5boost10shared_ptrIN3RBX4GameEEaSERKS3_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>> const&)")]
// 0x3a390 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_
pub fn stub_0x3a390() -> ! {
    todo!("0x3a390 __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::Tasks::Sequence>::reset<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)")]
// 0x3a5bc — __ZN5boost10shared_ptrIN3RBX5Tasks8SequenceEE5resetIS3_EEvPT_
// was: void boost::shared_ptr<RBX::Tasks::Sequence>::reset<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)
pub fn stub_0x3a5bc() -> ! {
    todo!("0x3a5bc __ZN5boost10shared_ptrIN3RBX5Tasks8SequenceEE5resetIS3_EEvPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ViewBase>::reset(void)")]
// 0x3a660 — __ZN5boost10shared_ptrIN3RBX8ViewBaseEE5resetEv
// was: boost::shared_ptr<RBX::ViewBase>::reset(void)
pub fn stub_0x3a660() -> ! {
    todo!("0x3a660 __ZN5boost10shared_ptrIN3RBX8ViewBaseEE5resetEv")
}

#[doc(alias = "boost::exception_ptr::~exception_ptr()")]
// 0x3a6f8 — __ZN5boost13exception_ptrD1Ev
pub fn stub_0x3a6f8() -> ! {
    todo!("0x3a6f8 __ZN5boost13exception_ptrD1Ev")
}

#[doc(alias = "boost::detail::sp_counted_base::weak_release(void)")]
// 0x3a850 — __ZN5boost6detail15sp_counted_base12weak_releaseEv
pub fn stub_0x3a850() -> ! {
    todo!("0x3a850 __ZN5boost6detail15sp_counted_base12weak_releaseEv")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>)")]
// 0x3aa30 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_ESH_
pub fn stub_0x3aa30() -> ! {
    todo!("0x3aa30 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_ESH_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::~map()")]
// 0x3aa90 — __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEED1Ev
pub fn stub_0x3aa90() -> ! {
    todo!("0x3aa90 __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEED1Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::pair<RBX::Name const* const,RBX::ICreator const*> const&)")]
// 0x3ad20 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
pub fn stub_0x3ad20() -> ! {
    todo!("0x3ad20 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RunService>(void)")]
// 0x3af08 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10RunServiceEEEmv
pub fn stub_0x3af08() -> ! {
    todo!("0x3af08 __ZN3RBX15ServiceProvider15doGetClassIndexINS_10RunServiceEEEmv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)")]
// 0x3b14c — __ZN5boost6detail12shared_countC2IN3RBX5Tasks8SequenceEEEPT_
pub fn stub_0x3b14c() -> ! {
    todo!("0x3b14c __ZN5boost6detail12shared_countC2IN3RBX5Tasks8SequenceEEEPT_")
}

#[doc(alias = "RBX::Tasks::Coordinator::onPreStep(RBX::TaskScheduler::Job *)")]
// 0x3b268 — __ZN3RBX5Tasks11Coordinator9onPreStepEPNS_13TaskScheduler3JobE
pub fn stub_0x3b268() -> ! {
    todo!("0x3b268 __ZN3RBX5Tasks11Coordinator9onPreStepEPNS_13TaskScheduler3JobE")
}

#[doc(alias = "RBX::Tasks::Coordinator::onPostStep(RBX::TaskScheduler::Job *)")]
// 0x3b26c — __ZN3RBX5Tasks11Coordinator10onPostStepEPNS_13TaskScheduler3JobE
pub fn stub_0x3b26c() -> ! {
    todo!("0x3b26c __ZN3RBX5Tasks11Coordinator10onPostStepEPNS_13TaskScheduler3JobE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::~sp_counted_impl_p()")]
// 0x3b270 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED1Ev
pub fn stub_0x3b270() -> ! {
    todo!("0x3b270 __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::~sp_counted_impl_p()")]
// 0x3b274 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED0Ev
pub fn stub_0x3b274() -> ! {
    todo!("0x3b274 __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::dispose(void)")]
// 0x3b278 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE7disposeEv
pub fn stub_0x3b278() -> ! {
    todo!("0x3b278 __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::get_deleter(std::type_info const&)")]
// 0x3b32c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE11get_deleterERKSt9type_info
pub fn stub_0x3b32c() -> ! {
    todo!("0x3b32c __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::get_untyped_deleter(void)")]
// 0x3b330 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE19get_untyped_deleterEv
pub fn stub_0x3b330() -> ! {
    todo!("0x3b330 __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tasks::ExclusiveSequence>(RBX::Tasks::ExclusiveSequence *)")]
// 0x3b334 — __ZN5boost6detail12shared_countC2IN3RBX5Tasks17ExclusiveSequenceEEEPT_
pub fn stub_0x3b334() -> ! {
    todo!("0x3b334 __ZN5boost6detail12shared_countC2IN3RBX5Tasks17ExclusiveSequenceEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::~sp_counted_impl_p()")]
// 0x3b450 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED1Ev
pub fn stub_0x3b450() -> ! {
    todo!("0x3b450 __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::~sp_counted_impl_p()")]
// 0x3b454 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED0Ev
pub fn stub_0x3b454() -> ! {
    todo!("0x3b454 __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::dispose(void)")]
// 0x3b458 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE7disposeEv
pub fn stub_0x3b458() -> ! {
    todo!("0x3b458 __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::get_deleter(std::type_info const&)")]
// 0x3b50c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE11get_deleterERKSt9type_info
pub fn stub_0x3b50c() -> ! {
    todo!("0x3b50c __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::get_untyped_deleter(void)")]
// 0x3b510 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE19get_untyped_deleterEv
pub fn stub_0x3b510() -> ! {
    todo!("0x3b510 __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::find<RBX::ControllerService>(void)const")]
// 0x3b518 — __ZNK3RBX15ServiceProvider4findINS_17ControllerServiceEEEPT_v
pub fn stub_0x3b518() -> ! {
    todo!("0x3b518 __ZNK3RBX15ServiceProvider4findINS_17ControllerServiceEEEPT_v")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ControllerService>(void)")]
// 0x3b910 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17ControllerServiceEEEmv
pub fn stub_0x3b910() -> ! {
    todo!("0x3b910 __ZN3RBX15ServiceProvider15doGetClassIndexINS_17ControllerServiceEEEmv")
}

#[doc(alias = "rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0>::operator delete(void *)")]
// 0x3bcb8 — __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EEdlEPv
pub fn stub_0x3bcb8() -> ! {
    todo!("0x3bcb8 __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EEdlEPv")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::insert(rbx::signals::signal<void ()(void)>::slot *)")]
// 0x3be00 — __ZN3rbx7signals6signalIFvvEE6insertEPNS3_4slotE
pub fn stub_0x3be00() -> ! {
    todo!("0x3be00 __ZN3rbx7signals6signalIFvvEE6insertEPNS3_4slotE")
}

#[doc(alias = "void rbx_core::SharedPtr_add_weak_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
// 0x3c010 — __ZN5boost26intrusive_ptr_add_weak_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// was: void boost::intrusive_ptr_add_weak_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)
pub fn stub_0x3c010() -> ! {
    todo!("0x3c010 __ZN5boost26intrusive_ptr_add_weak_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot>::operator=(rbx::signals::signal<void ()(void)>::slot*)")]
// 0x3c0c8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(void)>::slot>::operator=(rbx::signals::signal<void ()(void)>::slot*)
pub fn stub_0x3c0c8() -> ! {
    todo!("0x3c0c8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSEPS6_")
}

#[doc(alias = "boost::mutex::unlock(void)")]
// 0x3c170 — __ZN5boost5mutex6unlockEv
pub fn stub_0x3c170() -> ! {
    todo!("0x3c170 __ZN5boost5mutex6unlockEv")
}

#[doc(alias = "void boost::throw_exception<boost::lock_error>(boost::lock_error const&)")]
// 0x3c2a0 — __ZN5boost15throw_exceptionINS_10lock_errorEEEvRKT_
pub fn stub_0x3c2a0() -> ! {
    todo!("0x3c2a0 __ZN5boost15throw_exceptionINS_10lock_errorEEEvRKT_")
}

#[doc(alias = "boost::lock_error::~lock_error()")]
// 0x3c470 — __ZN5boost10lock_errorD0Ev
pub fn stub_0x3c470() -> ! {
    todo!("0x3c470 __ZN5boost10lock_errorD0Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
// 0x3c4a0 — __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED2Ev
pub fn stub_0x3c4a0() -> ! {
    todo!("0x3c4a0 __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED2Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
// 0x3c4e0 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_10lock_errorEED1Ev
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()
pub fn stub_0x3c4e0() -> ! {
    todo!("0x3c4e0 __ZThn20_N5boost16exception_detail19error_info_injectorINS_10lock_errorEED1Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
// 0x3c528 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()
pub fn stub_0x3c528() -> ! {
    todo!("0x3c528 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
// 0x3c570 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev
pub fn stub_0x3c570() -> ! {
    todo!("0x3c570 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone(void)const")]
// 0x3c5b8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE5cloneEv
pub fn stub_0x3c5b8() -> ! {
    todo!("0x3c5b8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE5cloneEv")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
// 0x3c678 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()
pub fn stub_0x3c678() -> ! {
    todo!("0x3c678 __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
// 0x3c680 — __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED0Ev
pub fn stub_0x3c680() -> ! {
    todo!("0x3c680 __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED0Ev")
}

#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::adopt(boost::exception_detail::error_info_container*)")]
// 0x3c698 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEE5adoptEPS2_
pub fn stub_0x3c698() -> ! {
    todo!("0x3c698 __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEE5adoptEPS2_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::lock_error> const&)")]
// 0x3c6c8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEEC1ERKS4_
pub fn stub_0x3c6c8() -> ! {
    todo!("0x3c6c8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEEC1ERKS4_")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::safe_static_init_mutex(void)")]
// 0x3c920 — __ZN3rbx7signals6signalIFvvEE22safe_static_init_mutexEv
pub fn stub_0x3c920() -> ! {
    todo!("0x3c920 __ZN3rbx7signals6signalIFvvEE22safe_static_init_mutexEv")
}

#[doc(alias = "boost::thread_resource_error::~thread_resource_error()")]
// 0x3c928 — __ZN5boost21thread_resource_errorD1Ev
pub fn stub_0x3c928() -> ! {
    todo!("0x3c928 __ZN5boost21thread_resource_errorD1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
// 0x3c958 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED2Ev
pub fn stub_0x3c958() -> ! {
    todo!("0x3c958 __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED2Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
// 0x3c998 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()
pub fn stub_0x3c998() -> ! {
    todo!("0x3c998 __ZThn20_N5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
// 0x3c9e0 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()
pub fn stub_0x3c9e0() -> ! {
    todo!("0x3c9e0 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
// 0x3ca28 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev
pub fn stub_0x3ca28() -> ! {
    todo!("0x3ca28 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const")]
// 0x3ca70 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv
pub fn stub_0x3ca70() -> ! {
    todo!("0x3ca70 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
// 0x3cb30 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()
pub fn stub_0x3cb30() -> ! {
    todo!("0x3cb30 __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const")]
// 0x3cb38 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const
pub fn stub_0x3cb38() -> ! {
    todo!("0x3cb38 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
// 0x3cb48 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED0Ev
pub fn stub_0x3cb48() -> ! {
    todo!("0x3cb48 __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::thread_resource_error> const&)")]
// 0x3cb60 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS4_
pub fn stub_0x3cb60() -> ! {
    todo!("0x3cb60 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS4_")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()")]
// 0x3cdb8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED1Ev
pub fn stub_0x3cdb8() -> ! {
    todo!("0x3cdb8 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()")]
// 0x3ce64 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED0Ev
pub fn stub_0x3ce64() -> ! {
    todo!("0x3ce64 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)")]
// 0x3cf18 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv
pub fn stub_0x3cf18() -> ! {
    todo!("0x3cf18 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)")]
// 0x3cf20 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)
pub fn stub_0x3cf20() -> ! {
    todo!("0x3cf20 __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>::operator()(void)")]
// 0x3cf28 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv
pub fn stub_0x3cf28() -> ! {
    todo!("0x3cf28 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::remove(rbx::signals::signal<void ()(void)>::slot *)")]
// 0x3cf40 — __ZN3rbx7signals6signalIFvvEE6removeEPNS3_4slotE
pub fn stub_0x3cf40() -> ! {
    todo!("0x3cf40 __ZN3rbx7signals6signalIFvvEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::safe_static_init_mutex(void)")]
// 0x3d030 — __ZN3rbx7signals6signalIFvvEE4slot22safe_static_init_mutexEv
pub fn stub_0x3d030() -> ! {
    todo!("0x3d030 __ZN3rbx7signals6signalIFvvEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::~slot()")]
// 0x3d038 — __ZN3rbx7signals6signalIFvvEE4slotD1Ev
pub fn stub_0x3d038() -> ! {
    todo!("0x3d038 __ZN3rbx7signals6signalIFvvEE4slotD1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()")]
// 0x3d0e4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED1Ev
pub fn stub_0x3d0e4() -> ! {
    todo!("0x3d0e4 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()")]
// 0x3d190 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED0Ev
pub fn stub_0x3d190() -> ! {
    todo!("0x3d190 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED0Ev")
}

#[doc(alias = "rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0>::counts::counts(void)")]
// 0x3d240 — __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EE6countsC2Ev
pub fn stub_0x3d240() -> ! {
    todo!("0x3d240 __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EE6countsC2Ev")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ViewBase>(RBX::ViewBase *)")]
// 0x3db4c — __ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_
pub fn stub_0x3db4c() -> ! {
    todo!("0x3db4c __ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::~sp_counted_impl_p()")]
// 0x3dc40 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED1Ev
pub fn stub_0x3dc40() -> ! {
    todo!("0x3dc40 __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::~sp_counted_impl_p()")]
// 0x3dc44 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED0Ev
pub fn stub_0x3dc44() -> ! {
    todo!("0x3dc44 __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::dispose(void)")]
// 0x3dc48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE7disposeEv
pub fn stub_0x3dc48() -> ! {
    todo!("0x3dc48 __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_untyped_deleter(void)")]
// 0x3dc5c — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE19get_untyped_deleterEv
pub fn stub_0x3dc5c() -> ! {
    todo!("0x3dc5c __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(rbx_core::SharedPtr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const")]
// 0x3dc60 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(boost::shared_ptr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const
pub fn stub_0x3dc60() -> ! {
    todo!("0x3dc60 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
// 0x3dd34 — __ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_
pub fn stub_0x3dd34() -> ! {
    todo!("0x3dd34 __ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
// 0x3de28 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED1Ev
pub fn stub_0x3de28() -> ! {
    todo!("0x3de28 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
// 0x3de2c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED0Ev
pub fn stub_0x3de2c() -> ! {
    todo!("0x3de2c __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::dispose(void)")]
// 0x3de30 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE7disposeEv
pub fn stub_0x3de30() -> ! {
    todo!("0x3de30 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_deleter(std::type_info const&)")]
// 0x3de40 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE11get_deleterERKSt9type_info
pub fn stub_0x3de40() -> ! {
    todo!("0x3de40 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_untyped_deleter(void)")]
// 0x3de44 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE19get_untyped_deleterEv
pub fn stub_0x3de44() -> ! {
    todo!("0x3de44 __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(rbx_core::SharedPtr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const")]
// 0x3de48 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(boost::shared_ptr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const
pub fn stub_0x3de48() -> ! {
    todo!("0x3de48 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
// 0x3df1c — __ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_
pub fn stub_0x3df1c() -> ! {
    todo!("0x3df1c __ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_")
}