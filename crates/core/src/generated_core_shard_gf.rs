//! core shard GF — 100 core stubs EA-sorted, 0xf4b534..0xf4c514 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf4b524).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf4b524.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<int>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<int>)")]
// 0xf4b534 — j___ZN5boost3_bi8storage2INS0_5valueISsEENS2_IiEEEC2ES3_S4_
pub fn stub_f4b534() -> ! {
    todo!("0xf4b534 j___ZN5boost3_bi8storage2INS0_5valueISsEENS2_IiEEEC2ES3_S4_")
}

#[doc(alias = "boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list_av_2<std::string,int>::type> boost::bind<RBX::worker_thread::work_result,std::string,int,std::string,int>(RBX::worker_thread::work_result (*)(std::string,int),std::string,int)")]
// 0xf4b544 — j___ZN5boost4bindIN3RBX13worker_thread11work_resultESsiSsiEENS_3_bi6bind_tIT_PFS6_T0_T1_ENS4_9list_av_2IT2_T3_E4typeEEESA_SC_SD_
pub fn stub_f4b544() -> ! {
    todo!("0xf4b544 j___ZN5boost4bindIN3RBX13worker_thread11work_resultESsiSsiEENS_3_bi6bind_tIT_PFS6_T0_T1_ENS4_9list_av_2IT2_T3_E4typeEEESA_SC_SD_")
}

#[doc(alias = "boost::function0<RBX::worker_thread::work_result>::clear(void)")]
// 0xf4b554 — j___ZN5boost9function0IN3RBX13worker_thread11work_resultEE5clearEv
pub fn stub_f4b554() -> ! {
    todo!("0xf4b554 j___ZN5boost9function0IN3RBX13worker_thread11work_resultEE5clearEv")
}

#[doc(alias = "void boost::function0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>)")]
// 0xf4b564 — j___ZN5boost9function0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS3_PFS3_SsiENS6_5list2INS6_5valueISsEENSB_IiEEEEEEEEvT_
pub fn stub_f4b564() -> ! {
    todo!("0xf4b564 j___ZN5boost9function0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS3_PFS3_SsiENS6_5list2INS6_5valueISsEENSB_IiEEEEEEEEvT_")
}

#[doc(alias = "boost::xtime::operator boost::posix_time::ptime(void)const")]
// 0xf4b584 — j___ZNK5boost5xtimecvNS_10posix_time5ptimeEEv
pub fn stub_f4b584() -> ! {
    todo!("0xf4b584 j___ZNK5boost5xtimecvNS_10posix_time5ptimeEEv")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<RBX::worker_thread::work_result>::assign_functor<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,mpl_::bool_<true>)const")]
// 0xf4b594 — j___ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE14assign_functorINS_3_bi6bind_tIS5_PFS5_SsiENS8_5list2INS8_5valueISsEENSD_IiEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb1EEE
pub fn stub_f4b594() -> ! {
    todo!("0xf4b594 j___ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE14assign_functorINS_3_bi6bind_tIS5_PFS5_SsiENS8_5list2INS8_5valueISsEENSD_IiEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb1EEE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &)const")]
// 0xf4b5a4 — j___ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS5_PFS5_SsiENS8_5list2INS8_5valueISsEENSD_IiEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_f4b5a4() -> ! {
    todo!("0xf4b5a4 j___ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS5_PFS5_SsiENS8_5list2INS8_5valueISsEENSD_IiEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<RBX::worker_thread::work_result>::assign_to<boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>>(boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(std::string,int),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf4b5b4 — j___ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS5_PFS5_SsiENS8_5list2INS8_5valueISsEENSD_IiEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_f4b5b4() -> ! {
    todo!("0xf4b5b4 j___ZNK5boost6detail8function13basic_vtable0IN3RBX13worker_thread11work_resultEE9assign_toINS_3_bi6bind_tIS5_PFS5_SsiENS8_5list2INS8_5valueISsEENSD_IiEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MouseCommand> RBX::shared_from<RBX::MouseCommand>(RBX::MouseCommand*)")]
// 0xf4b8f4 — j___ZN3RBX11shared_fromINS_12MouseCommandEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_f4b8f4() -> ! {
    todo!("0xf4b8f4 j___ZN3RBX11shared_fromINS_12MouseCommandEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Decal> RBX::shared_from<RBX::Decal>(RBX::Decal*)")]
// 0xf4b904 — j___ZN3RBX11shared_fromINS_5DecalEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_f4b904() -> ! {
    todo!("0xf4b904 j___ZN3RBX11shared_fromINS_5DecalEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Camera> RBX::shared_from<RBX::Camera>(RBX::Camera*)")]
// 0xf4b914 — j___ZN3RBX11shared_fromINS_6CameraEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_f4b914() -> ! {
    todo!("0xf4b914 j___ZN3RBX11shared_fromINS_6CameraEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::ArrowToolBase::~ArrowToolBase()")]
// 0xf4b934 — j___ZN3RBX13ArrowToolBaseD0Ev
pub fn stub_f4b934() -> ! {
    todo!("0xf4b934 j___ZN3RBX13ArrowToolBaseD0Ev")
}

#[doc(alias = "RBX::ArrowToolBase::~ArrowToolBase()")]
// 0xf4b944 — j___ZN3RBX13ArrowToolBaseD2Ev
pub fn stub_f4b944() -> ! {
    todo!("0xf4b944 j___ZN3RBX13ArrowToolBaseD2Ev")
}

#[doc(alias = "RBX::AdvArrowToolBase::~AdvArrowToolBase()")]
// 0xf4ba24 — j___ZN3RBX16AdvArrowToolBaseD0Ev
pub fn stub_f4ba24() -> ! {
    todo!("0xf4ba24 j___ZN3RBX16AdvArrowToolBaseD0Ev")
}

#[doc(alias = "RBX::AdvArrowToolBase::~AdvArrowToolBase()")]
// 0xf4ba34 — j___ZN3RBX16AdvArrowToolBaseD2Ev
pub fn stub_f4ba34() -> ! {
    todo!("0xf4ba34 j___ZN3RBX16AdvArrowToolBaseD2Ev")
}

#[doc(alias = "RBX::Stats::StatsService::StatsService(void)")]
// 0xf4bad4 — j___ZN3RBX5Stats12StatsServiceC2Ev
pub fn stub_f4bad4() -> ! {
    todo!("0xf4bad4 j___ZN3RBX5Stats12StatsServiceC2Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<double>::TypedStatsItem(boost::function0<double>)")]
// 0xf4bae4 — j___ZN3RBX5Stats14TypedStatsItemIdEC2EN5boost9function0IdEE
pub fn stub_f4bae4() -> ! {
    todo!("0xf4bae4 j___ZN3RBX5Stats14TypedStatsItemIdEC2EN5boost9function0IdEE")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<float>::TypedStatsItem(boost::function0<float>)")]
// 0xf4baf4 — j___ZN3RBX5Stats14TypedStatsItemIfEC2EN5boost9function0IfEE
pub fn stub_f4baf4() -> ! {
    todo!("0xf4baf4 j___ZN3RBX5Stats14TypedStatsItemIfEC2EN5boost9function0IfEE")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createChildItem<double>(char const*,boost::function0<double>)")]
// 0xf4bb04 — j___ZN3RBX5Stats4Item15createChildItemIdEEPS1_PKcN5boost9function0IT_EE
pub fn stub_f4bb04() -> ! {
    todo!("0xf4bb04 j___ZN3RBX5Stats4Item15createChildItemIdEEPS1_PKcN5boost9function0IT_EE")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createChildItem<float>(char const*,boost::function0<float>)")]
// 0xf4bb14 — j___ZN3RBX5Stats4Item15createChildItemIfEEPS1_PKcN5boost9function0IT_EE
pub fn stub_f4bb14() -> ! {
    todo!("0xf4bb14 j___ZN3RBX5Stats4Item15createChildItemIfEEPS1_PKcN5boost9function0IT_EE")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createChildItem<int>(char const*,boost::function0<int>)")]
// 0xf4bb24 — j___ZN3RBX5Stats4Item15createChildItemIiEEPS1_PKcN5boost9function0IT_EE
pub fn stub_f4bb24() -> ! {
    todo!("0xf4bb24 j___ZN3RBX5Stats4Item15createChildItemIiEEPS1_PKcN5boost9function0IT_EE")
}

#[doc(alias = "RBX::World::reset(void)")]
// 0xf4bb34 — j___ZN3RBX5World5resetEv
pub fn stub_f4bb34() -> ! {
    todo!("0xf4bb34 j___ZN3RBX5World5resetEv")
}

#[doc(alias = "RBX::TouchPair::operator=(RBX::TouchPair const&)")]
// 0xf4bc14 — j___ZN3RBX9TouchPairaSERKS0_
pub fn stub_f4bc14() -> ! {
    todo!("0xf4bc14 j___ZN3RBX9TouchPairaSERKS0_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::TouchPair const&)>::operator()(RBX::TouchPair const&)")]
// 0xf4bc44 — j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9TouchPairEEEclES5_
pub fn stub_f4bc44() -> ! {
    todo!("0xf4bc44 j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9TouchPairEEEclES5_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::disconnectAll(void)")]
// 0xf4bc64 — j___ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE13disconnectAllEv
pub fn stub_f4bc64() -> ! {
    todo!("0xf4bc64 j___ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::safe_static_do_get_mutex(void)")]
// 0xf4bc74 — j___ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE24safe_static_do_get_mutexEv
pub fn stub_f4bc74() -> ! {
    todo!("0xf4bc74 j___ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot> &)")]
// 0xf4bc84 — j___ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
pub fn stub_f4bc84() -> ! {
    todo!("0xf4bc84 j___ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::on_error(std::exception &)")]
// 0xf4bc94 — j___ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE8on_errorERSt9exception
pub fn stub_f4bc94() -> ! {
    todo!("0xf4bc94 j___ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NewNullTool>::shared_ptr<RBX::NewNullTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf4bcd4 — j___ZN5boost10shared_ptrIN3RBX11NewNullToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_f4bcd4() -> ! {
    todo!("0xf4bcd4 j___ZN5boost10shared_ptrIN3RBX11NewNullToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvArrowTool>::shared_ptr<RBX::AdvArrowTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf4bce4 — j___ZN5boost10shared_ptrIN3RBX12AdvArrowToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_f4bce4() -> ! {
    todo!("0xf4bce4 j___ZN5boost10shared_ptrIN3RBX12AdvArrowToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MouseCommand>::operator=(rbx_core::SharedPtr<RBX::MouseCommand> const&)")]
// 0xf4bcf4 — j___ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSERKS3_
pub fn stub_f4bcf4() -> ! {
    todo!("0xf4bcf4 j___ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MouseCommand>& rbx_core::SharedPtr<RBX::MouseCommand>::operator=<RBX::NewNullTool>(rbx_core::SharedPtr<RBX::NewNullTool> const&)")]
// 0xf4bd04 — j___ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_11NewNullToolEEERS3_RKNS0_IT_EE
pub fn stub_f4bd04() -> ! {
    todo!("0xf4bd04 j___ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_11NewNullToolEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MouseCommand>& rbx_core::SharedPtr<RBX::MouseCommand>::operator=<RBX::AdvArrowTool>(rbx_core::SharedPtr<RBX::AdvArrowTool> const&)")]
// 0xf4bd14 — j___ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_12AdvArrowToolEEERS3_RKNS0_IT_EE
pub fn stub_f4bd14() -> ! {
    todo!("0xf4bd14 j___ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_12AdvArrowToolEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::IAdornableCollector>::shared_ptr<RBX::IAdornableCollector>(RBX::IAdornableCollector *)")]
// 0xf4bd44 — j___ZN5boost10shared_ptrIN3RBX19IAdornableCollectorEEC2IS2_EEPT_
pub fn stub_f4bd44() -> ! {
    todo!("0xf4bd44 j___ZN5boost10shared_ptrIN3RBX19IAdornableCollectorEEC2IS2_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Decal>::operator=(rbx_core::SharedPtr<RBX::Decal> const&)")]
// 0xf4bd64 — j___ZN5boost10shared_ptrIN3RBX5DecalEEaSERKS3_
pub fn stub_f4bd64() -> ! {
    todo!("0xf4bd64 j___ZN5boost10shared_ptrIN3RBX5DecalEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Camera>::operator=(rbx_core::SharedPtr<RBX::Camera> const&)")]
// 0xf4bda4 — j___ZN5boost10shared_ptrIN3RBX6CameraEEaSERKS3_
pub fn stub_f4bda4() -> ! {
    todo!("0xf4bda4 j___ZN5boost10shared_ptrIN3RBX6CameraEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DecalTool>::shared_ptr<RBX::DecalTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf4bdd4 — j___ZN5boost10shared_ptrIN3RBX9DecalToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_f4bdd4() -> ! {
    todo!("0xf4bdd4 j___ZN5boost10shared_ptrIN3RBX9DecalToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::RunService>,boost::_bi::list1<boost::_bi::value<RBX::RunService const*>>>::operator()(void)")]
// 0xf4be24 — j___ZN5boost3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX10RunServiceEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv
pub fn stub_f4be24() -> ! {
    todo!("0xf4be24 j___ZN5boost3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX10RunServiceEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv")
}

#[doc(alias = "boost::_bi::bind_t<float,boost::_mfi::cmf0<float,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>::operator()(void)")]
// 0xf4be44 — j___ZN5boost3_bi6bind_tIfNS_4_mfi4cmf0IfN3RBX5WorldEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv
pub fn stub_f4be44() -> ! {
    todo!("0xf4be44 j___ZN5boost3_bi6bind_tIfNS_4_mfi4cmf0IfN3RBX5WorldEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv")
}

#[doc(alias = "boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>::operator()(void)")]
// 0xf4be54 — j___ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX5WorldEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv
pub fn stub_f4be54() -> ! {
    todo!("0xf4be54 j___ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX5WorldEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv")
}

#[doc(alias = "boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::Kernel>,boost::_bi::list1<boost::_bi::value<RBX::Kernel const*>>>::operator()(void)")]
// 0xf4be64 — j___ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv
pub fn stub_f4be64() -> ! {
    todo!("0xf4be64 j___ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv")
}

#[doc(alias = "boost::_bi::bind_t<int,boost::_mfi::cmf1<int,RBX::World,RBX::IWorldStage::MetricType>,boost::_bi::list2<boost::_bi::value<RBX::World const*>,boost::_bi::value<RBX::IWorldStage::MetricType>>>::operator()(void)")]
// 0xf4be74 — j___ZN5boost3_bi6bind_tIiNS_4_mfi4cmf1IiN3RBX5WorldENS4_11IWorldStage10MetricTypeEEENS0_5list2INS0_5valueIPKS5_EENSA_IS7_EEEEEclEv
pub fn stub_f4be74() -> ! {
    todo!("0xf4be74 j___ZN5boost3_bi6bind_tIiNS_4_mfi4cmf1IiN3RBX5WorldENS4_11IWorldStage10MetricTypeEEENS0_5list2INS0_5valueIPKS5_EENSA_IS7_EEEEEclEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NewNullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf4bea4 — j___ZN5boost6detail12shared_countC2IPN3RBX11NewNullToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_f4bea4() -> ! {
    todo!("0xf4bea4 j___ZN5boost6detail12shared_countC2IPN3RBX11NewNullToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvArrowTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf4beb4 — j___ZN5boost6detail12shared_countC2IPN3RBX12AdvArrowToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_f4beb4() -> ! {
    todo!("0xf4beb4 j___ZN5boost6detail12shared_countC2IPN3RBX12AdvArrowToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DecalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0xf4bf24 — j___ZN5boost6detail12shared_countC2IPN3RBX9DecalToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_f4bf24() -> ! {
    todo!("0xf4bf24 j___ZN5boost6detail12shared_countC2IPN3RBX9DecalToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::function0<double>::assign_to_own(boost::function0<double> const&)")]
// 0xf4bf34 — j___ZN5boost9function0IdE13assign_to_ownERKS1_
pub fn stub_f4bf34() -> ! {
    todo!("0xf4bf34 j___ZN5boost9function0IdE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::function0<double>::clear(void)")]
// 0xf4bf44 — j___ZN5boost9function0IdE5clearEv
pub fn stub_f4bf44() -> ! {
    todo!("0xf4bf44 j___ZN5boost9function0IdE5clearEv")
}

#[doc(alias = "boost::function0<float>::assign_to_own(boost::function0<float> const&)")]
// 0xf4bf54 — j___ZN5boost9function0IfE13assign_to_ownERKS1_
pub fn stub_f4bf54() -> ! {
    todo!("0xf4bf54 j___ZN5boost9function0IfE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::function0<int>::clear(void)")]
// 0xf4bf64 — j___ZN5boost9function0IiE5clearEv
pub fn stub_f4bf64() -> ! {
    todo!("0xf4bf64 j___ZN5boost9function0IiE5clearEv")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::NewNullTool,RBX::NewNullTool>(rbx_core::SharedPtr<RBX::NewNullTool> const*,RBX::NewNullTool *)const")]
// 0xf4c014 — j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11NewNullToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_f4c014() -> ! {
    todo!("0xf4c014 j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11NewNullToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvArrowTool,RBX::AdvArrowTool>(rbx_core::SharedPtr<RBX::AdvArrowTool> const*,RBX::AdvArrowTool *)const")]
// 0xf4c024 — j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12AdvArrowToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_f4c024() -> ! {
    todo!("0xf4c024 j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12AdvArrowToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::DecalTool,RBX::DecalTool>(rbx_core::SharedPtr<RBX::DecalTool> const*,RBX::DecalTool *)const")]
// 0xf4c034 — j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9DecalToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_f4c034() -> ! {
    todo!("0xf4c034 j___ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9DecalToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::function0<int>::operator()(void)const")]
// 0xf4c044 — j___ZNK5boost9function0IiEclEv
pub fn stub_f4c044() -> ! {
    todo!("0xf4c044 j___ZNK5boost9function0IiEclEv")
}

#[doc(alias = "std::_Vector_base<RBX::TouchPair,std::allocator<RBX::TouchPair>>::_M_allocate(unsigned long)")]
// 0xf4c054 — j___ZNSt12_Vector_baseIN3RBX9TouchPairESaIS1_EE11_M_allocateEm
pub fn stub_f4c054() -> ! {
    todo!("0xf4c054 j___ZNSt12_Vector_baseIN3RBX9TouchPairESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::TouchPair * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TouchPair *,RBX::TouchPair *>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair *)")]
// 0xf4c084 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9TouchPairES5_EET0_T_S7_S6_
pub fn stub_f4c084() -> ! {
    todo!("0xf4c084 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9TouchPairES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TouchPair*,std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>>,unsigned long,RBX::TouchPair const&)")]
// 0xf4c0a4 — j___ZNSt6vectorIN3RBX9TouchPairESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_f4c0a4() -> ! {
    todo!("0xf4c0a4 j___ZNSt6vectorIN3RBX9TouchPairESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::resize(unsigned long,RBX::TouchPair)")]
// 0xf4c0b4 — j___ZNSt6vectorIN3RBX9TouchPairESaIS1_EE6resizeEmS1_
pub fn stub_f4c0b4() -> ! {
    todo!("0xf4c0b4 j___ZNSt6vectorIN3RBX9TouchPairESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::~vector()")]
// 0xf4c0c4 — j___ZNSt6vectorIN3RBX9TouchPairESaIS1_EED2Ev
pub fn stub_f4c0c4() -> ! {
    todo!("0xf4c0c4 j___ZNSt6vectorIN3RBX9TouchPairESaIS1_EED2Ev")
}

#[doc(alias = "std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::push_back(bool (* const&)(void))")]
// 0xf4c104 — j___ZNSt6vectorIPFbvESaIS1_EE9push_backERKS1_
pub fn stub_f4c104() -> ! {
    todo!("0xf4c104 j___ZNSt6vectorIPFbvESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<unsigned long *,std::allocator<unsigned long *>>::push_back(unsigned long * const&)")]
// 0xf4c124 — j___ZNSt6vectorIPmSaIS0_EE9push_backERKS0_
pub fn stub_f4c124() -> ! {
    todo!("0xf4c124 j___ZNSt6vectorIPmSaIS0_EE9push_backERKS0_")
}

#[doc(alias = "RBX::TouchPair * std::__uninitialized_copy_aux<RBX::TouchPair *,RBX::TouchPair *>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair *,std::__false_type)")]
// 0xf4c134 — j___ZSt24__uninitialized_copy_auxIPN3RBX9TouchPairES2_ET0_T_S4_S3_St12__false_type
pub fn stub_f4c134() -> ! {
    todo!("0xf4c134 j___ZSt24__uninitialized_copy_auxIPN3RBX9TouchPairES2_ET0_T_S4_S3_St12__false_type")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::TouchPair *,unsigned long,RBX::TouchPair>(RBX::TouchPair *,unsigned long,RBX::TouchPair const&,std::__false_type)")]
// 0xf4c144 — j___ZSt26__uninitialized_fill_n_auxIPN3RBX9TouchPairEmS1_EvT_T0_RKT1_St12__false_type
pub fn stub_f4c144() -> ! {
    todo!("0xf4c144 j___ZSt26__uninitialized_fill_n_auxIPN3RBX9TouchPairEmS1_EvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "void std::fill<RBX::TouchPair *,RBX::TouchPair>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair const&)")]
// 0xf4c164 — j___ZSt4fillIPN3RBX9TouchPairES1_EvT_S3_RKT0_
pub fn stub_f4c164() -> ! {
    todo!("0xf4c164 j___ZSt4fillIPN3RBX9TouchPairES1_EvT_S3_RKT0_")
}

#[doc(alias = "RBX::Body::getBranchIWorld(void)")]
// 0xf4c194 — j___ZN3RBX4Body15getBranchIWorldEv
pub fn stub_f4c194() -> ! {
    todo!("0xf4c194 j___ZN3RBX4Body15getBranchIWorldEv")
}

#[doc(alias = "RBX::Body::getIWorld(void)")]
// 0xf4c1a4 — j___ZN3RBX4Body9getIWorldEv
pub fn stub_f4c1a4() -> ! {
    todo!("0xf4c1a4 j___ZN3RBX4Body9getIWorldEv")
}

#[doc(alias = "RBX::Allocator<RBX::Body>::Allocator(void)")]
// 0xf4c1b4 — j___ZN3RBX9AllocatorINS_4BodyEEC2Ev
pub fn stub_f4c1b4() -> ! {
    todo!("0xf4c1b4 j___ZN3RBX9AllocatorINS_4BodyEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::Body>::operator delete(void *)")]
// 0xf4c1c4 — j___ZN3RBX9AllocatorINS_4BodyEEdlEPv
pub fn stub_f4c1c4() -> ! {
    todo!("0xf4c1c4 j___ZN3RBX9AllocatorINS_4BodyEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::Cofm>::operator delete(void *)")]
// 0xf4c1d4 — j___ZN3RBX9AllocatorINS_4CofmEEdlEPv
pub fn stub_f4c1d4() -> ! {
    todo!("0xf4c1d4 j___ZN3RBX9AllocatorINS_4CofmEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::Cofm>::operator new(unsigned long)")]
// 0xf4c1e4 — j___ZN3RBX9AllocatorINS_4CofmEEnwEm
pub fn stub_f4c1e4() -> ! {
    todo!("0xf4c1e4 j___ZN3RBX9AllocatorINS_4CofmEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::SimBody>::operator delete(void *)")]
// 0xf4c1f4 — j___ZN3RBX9AllocatorINS_7SimBodyEEdlEPv
pub fn stub_f4c1f4() -> ! {
    todo!("0xf4c1f4 j___ZN3RBX9AllocatorINS_7SimBodyEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::SimBody>::operator new(unsigned long)")]
// 0xf4c204 — j___ZN3RBX9AllocatorINS_7SimBodyEEnwEm
pub fn stub_f4c204() -> ! {
    todo!("0xf4c204 j___ZN3RBX9AllocatorINS_7SimBodyEEnwEm")
}

#[doc(alias = "boost::singleton_pool<RBX::Body,276u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4c214 — j___ZN5boost14singleton_poolIN3RBX4BodyELj276ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4c214() -> ! {
    todo!("0xf4c214 j___ZN5boost14singleton_poolIN3RBX4BodyELj276ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "boost::singleton_pool<RBX::Cofm,60u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4c224 — j___ZN5boost14singleton_poolIN3RBX4CofmELj60ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4c224() -> ! {
    todo!("0xf4c224 j___ZN5boost14singleton_poolIN3RBX4CofmELj60ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "boost::singleton_pool<RBX::Cofm,60u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4c234 — j___ZN5boost14singleton_poolIN3RBX4CofmELj60ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4c234() -> ! {
    todo!("0xf4c234 j___ZN5boost14singleton_poolIN3RBX4CofmELj60ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "boost::singleton_pool<RBX::SimBody,308u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4c244 — j___ZN5boost14singleton_poolIN3RBX7SimBodyELj308ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4c244() -> ! {
    todo!("0xf4c244 j___ZN5boost14singleton_poolIN3RBX7SimBodyELj308ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "boost::singleton_pool<RBX::SimBody,308u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4c254 — j___ZN5boost14singleton_poolIN3RBX7SimBodyELj308ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4c254() -> ! {
    todo!("0xf4c254 j___ZN5boost14singleton_poolIN3RBX7SimBodyELj308ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "RBX::Allocator<RBX::Cofm>::Allocator(void)")]
// 0xf4c264 — j___ZN3RBX9AllocatorINS_4CofmEEC2Ev
pub fn stub_f4c264() -> ! {
    todo!("0xf4c264 j___ZN3RBX9AllocatorINS_4CofmEEC2Ev")
}

#[doc(alias = "boost::singleton_pool<RBX::Cofm,60u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4c274 — j___ZN5boost14singleton_poolIN3RBX4CofmELj60ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4c274() -> ! {
    todo!("0xf4c274 j___ZN5boost14singleton_poolIN3RBX4CofmELj60ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "RBX::Allocator<RBX::NormalBreakConnector>::operator delete(void *)")]
// 0xf4c284 — j___ZN3RBX9AllocatorINS_20NormalBreakConnectorEEdlEPv
pub fn stub_f4c284() -> ! {
    todo!("0xf4c284 j___ZN3RBX9AllocatorINS_20NormalBreakConnectorEEdlEPv")
}

#[doc(alias = "RBX::PairParams::operator==(RBX::PairParams const&)")]
// 0xf4c294 — j___ZN3RBX10PairParamseqERKS0_
pub fn stub_f4c294() -> ! {
    todo!("0xf4c294 j___ZN3RBX10PairParamseqERKS0_")
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::operator delete(void *)")]
// 0xf4c2a4 — j___ZN3RBX9AllocatorINS_17BallBallConnectorEEdlEPv
pub fn stub_f4c2a4() -> ! {
    todo!("0xf4c2a4 j___ZN3RBX9AllocatorINS_17BallBallConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::operator delete(void *)")]
// 0xf4c2b4 — j___ZN3RBX9AllocatorINS_18BallBlockConnectorEEdlEPv
pub fn stub_f4c2b4() -> ! {
    todo!("0xf4c2b4 j___ZN3RBX9AllocatorINS_18BallBlockConnectorEEdlEPv")
}

#[doc(alias = "RBX::IndexArray<RBX::Body,&RBX::Body::getLeafBodyIndex>::fastRemove(RBX::Body*)")]
// 0xf4c404 — j___ZN3RBX10IndexArrayINS_4BodyEXadL_ZNS1_16getLeafBodyIndexEvEEE10fastRemoveEPS1_
pub fn stub_f4c404() -> ! {
    todo!("0xf4c404 j___ZN3RBX10IndexArrayINS_4BodyEXadL_ZNS1_16getLeafBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Point,&RBX::Point::getKernelIndex>::fastRemove(RBX::Point*)")]
// 0xf4c414 — j___ZN3RBX10IndexArrayINS_5PointEXadL_ZNS1_14getKernelIndexEvEEE10fastRemoveEPS1_
pub fn stub_f4c414() -> ! {
    todo!("0xf4c414 j___ZN3RBX10IndexArrayINS_5PointEXadL_ZNS1_14getKernelIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getJointBodyIndex>::fastRemove(RBX::SimBody*)")]
// 0xf4c424 — j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_17getJointBodyIndexEvEEE10fastRemoveEPS1_
pub fn stub_f4c424() -> ! {
    todo!("0xf4c424 j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_17getJointBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getContactBodyIndex>::fastRemove(RBX::SimBody*)")]
// 0xf4c434 — j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_19getContactBodyIndexEvEEE10fastRemoveEPS1_
pub fn stub_f4c434() -> ! {
    todo!("0xf4c434 j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_19getContactBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getFreeFallBodyIndex>::fastRemove(RBX::SimBody*)")]
// 0xf4c444 — j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getFreeFallBodyIndexEvEEE10fastRemoveEPS1_
pub fn stub_f4c444() -> ! {
    todo!("0xf4c444 j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getFreeFallBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getRealTimeBodyIndex>::fastRemove(RBX::SimBody*)")]
// 0xf4c454 — j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getRealTimeBodyIndexEvEEE10fastRemoveEPS1_
pub fn stub_f4c454() -> ! {
    todo!("0xf4c454 j___ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getRealTimeBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getJointIndex>::fastRemove(RBX::Connector*)")]
// 0xf4c464 — j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_13getJointIndexEvEEE10fastRemoveEPS1_
pub fn stub_f4c464() -> ! {
    todo!("0xf4c464 j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_13getJointIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getContactIndex>::fastRemove(RBX::Connector*)")]
// 0xf4c474 — j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_15getContactIndexEvEEE10fastRemoveEPS1_
pub fn stub_f4c474() -> ! {
    todo!("0xf4c474 j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_15getContactIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getHumanoidIndex>::fastRemove(RBX::Connector*)")]
// 0xf4c484 — j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_16getHumanoidIndexEvEEE10fastRemoveEPS1_
pub fn stub_f4c484() -> ! {
    todo!("0xf4c484 j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_16getHumanoidIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getRealTimeIndex>::fastRemove(RBX::Connector*)")]
// 0xf4c494 — j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_16getRealTimeIndexEvEEE10fastRemoveEPS1_
pub fn stub_f4c494() -> ! {
    todo!("0xf4c494 j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_16getRealTimeIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getSecondPassIndex>::fastRemove(RBX::Connector*)")]
// 0xf4c4a4 — j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_18getSecondPassIndexEvEEE10fastRemoveEPS1_
pub fn stub_f4c4a4() -> ! {
    todo!("0xf4c4a4 j___ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_18getSecondPassIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::KernelData::insertBody(RBX::Body *)")]
// 0xf4c4b4 — j___ZN3RBX10KernelData10insertBodyEPNS_4BodyE
pub fn stub_f4c4b4() -> ! {
    todo!("0xf4c4b4 j___ZN3RBX10KernelData10insertBodyEPNS_4BodyE")
}

#[doc(alias = "RBX::KernelData::removeBody(RBX::Body *)")]
// 0xf4c4c4 — j___ZN3RBX10KernelData10removeBodyEPNS_4BodyE
pub fn stub_f4c4c4() -> ! {
    todo!("0xf4c4c4 j___ZN3RBX10KernelData10removeBodyEPNS_4BodyE")
}

#[doc(alias = "RBX::KernelData::addLeafBody(RBX::Body *)")]
// 0xf4c4d4 — j___ZN3RBX10KernelData11addLeafBodyEPNS_4BodyE
pub fn stub_f4c4d4() -> ! {
    todo!("0xf4c4d4 j___ZN3RBX10KernelData11addLeafBodyEPNS_4BodyE")
}

#[doc(alias = "RBX::KernelData::addConnector(RBX::Connector *)")]
// 0xf4c4e4 — j___ZN3RBX10KernelData12addConnectorEPNS_9ConnectorE
pub fn stub_f4c4e4() -> ! {
    todo!("0xf4c4e4 j___ZN3RBX10KernelData12addConnectorEPNS_9ConnectorE")
}

#[doc(alias = "RBX::KernelData::addLeafBodies(RBX::Body *)")]
// 0xf4c4f4 — j___ZN3RBX10KernelData13addLeafBodiesEPNS_4BodyE
pub fn stub_f4c4f4() -> ! {
    todo!("0xf4c4f4 j___ZN3RBX10KernelData13addLeafBodiesEPNS_4BodyE")
}

#[doc(alias = "RBX::KernelData::removeConnector(RBX::Connector *)")]
// 0xf4c504 — j___ZN3RBX10KernelData15removeConnectorEPNS_9ConnectorE
pub fn stub_f4c504() -> ! {
    todo!("0xf4c504 j___ZN3RBX10KernelData15removeConnectorEPNS_9ConnectorE")
}

#[doc(alias = "RBX::KernelData::addBodyToNewList(RBX::SimBody *)")]
// 0xf4c514 — j___ZN3RBX10KernelData16addBodyToNewListEPNS_7SimBodyE
pub fn stub_f4c514() -> ! {
    todo!("0xf4c514 j___ZN3RBX10KernelData16addBodyToNewListEPNS_7SimBodyE")
}
